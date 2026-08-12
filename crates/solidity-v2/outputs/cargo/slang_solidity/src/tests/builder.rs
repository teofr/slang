use slang_solidity_v2_common::evm_targets::EvmTarget;

use crate::ast::NodeId;
use crate::compilation::{CompilationBuilder, CompilationBuilderConfig, CompilationUnit, FileId};
use crate::diagnostics::DiagnosticExtensions;
use crate::diagnostics::kinds::compilation::UnresolvedImport;
use crate::utils::LanguageVersion;

/// Resolves every import path to a file of the same name, and refuses paths
/// starting with `?` so that unresolvable imports can be exercised too.
struct TestConfig;

impl CompilationBuilderConfig for TestConfig {
    fn resolve_import(
        &self,
        _source_file_id: &FileId,
        import_path: &str,
    ) -> Result<FileId, UnresolvedImport> {
        if import_path.starts_with('?') {
            return Err(UnresolvedImport {
                reason: "Unresolved import.".to_string(),
            });
        }

        Ok(import_path.into())
    }
}

fn builder() -> CompilationBuilder<TestConfig> {
    CompilationBuilder::create(LanguageVersion::LATEST, EvmTarget::LATEST, TestConfig)
}

fn contract(name: &str, imports: &[&str]) -> String {
    use std::fmt::Write;

    let imports = imports.iter().fold(String::new(), |mut text, path| {
        writeln!(text, "import \"{path}\";").unwrap();
        text
    });

    format!("pragma solidity ^0.8.0;\n{imports}\ncontract {name} {{}}\n")
}

fn diagnostic_codes(unit: &CompilationUnit) -> Vec<&'static str> {
    unit.diagnostics()
        .iter()
        .map(DiagnosticExtensions::code)
        .collect()
}

/// The source text each diagnostic points at.
///
/// Asserting on the sliced text rather than on raw offsets keeps the tests
/// readable, and makes a wrong range show up as the wrong snippet instead of as
/// two numbers that have to be counted out by hand.
fn diagnostic_snippets<'a>(unit: &CompilationUnit, source: &'a str) -> Vec<&'a str> {
    unit.diagnostics()
        .iter()
        .map(|diagnostic| &source[diagnostic.text_range().clone()])
        .collect()
}

#[test]
fn builds_every_file_that_was_added() {
    let mut builder = builder();

    builder.add_files([
        ("main.sol".into(), contract("Main", &["lib.sol"])),
        ("lib.sol".into(), contract("Lib", &[])),
    ]);
    // Not imported by anything, but still part of the compilation.
    builder.add_file("extra.sol".into(), contract("Extra", &[]));

    let unit = builder.build();

    assert!(unit.diagnostics().is_empty(), "{:#?}", unit.diagnostics());

    let file_ids: Vec<String> = unit
        .files()
        .map(|file| file.id().as_str().to_owned())
        .collect();
    assert_eq!(file_ids, ["extra.sol", "lib.sol", "main.sol"]);
}

#[test]
fn adding_a_file_twice_replaces_its_contents() {
    let mut builder = builder();

    builder.add_file("main.sol".into(), contract("Stale", &[]));
    builder.add_file("main.sol".into(), contract("Fresh", &[]));

    let unit = builder.build();

    assert!(unit.diagnostics().is_empty(), "{:#?}", unit.diagnostics());
    assert_eq!(unit.files().count(), 1);

    let contract_names: Vec<String> = unit
        .all_contracts()
        .map(|contract| contract.name().name().to_owned())
        .collect();
    assert_eq!(contract_names, ["Fresh"]);
}

#[test]
fn imports_outside_the_added_files_are_reported_once() {
    let mut builder = builder();

    builder.add_file("main.sol".into(), contract("Main", &["absent.sol"]));

    let unit = builder.build();

    assert_eq!(
        diagnostic_codes(&unit),
        ["compilation/missing-imported-file"]
    );
}

#[test]
fn imports_the_config_cannot_resolve_are_reported() {
    let mut builder = builder();

    builder.add_file("main.sol".into(), contract("Main", &["?nowhere.sol"]));

    let unit = builder.build();

    assert_eq!(diagnostic_codes(&unit), ["compilation/unresolved-import"]);
}

// Both tests below pin the *span* of an import diagnostic, not just its code, so
// that an editor underlines the offending path. Each source puts a resolvable
// import ahead of the failing one: a range that defaulted to the first import,
// or to `0..0`, would slice out the wrong text and fail.

#[test]
fn a_missing_imported_file_is_reported_at_its_import_path() {
    let mut builder = builder();

    let main = contract("Main", &["present.sol", "absent.sol"]);
    builder.add_file("main.sol".into(), main.clone());
    builder.add_file("present.sol".into(), contract("Present", &[]));

    let unit = builder.build();

    assert_eq!(
        diagnostic_codes(&unit),
        ["compilation/missing-imported-file"]
    );
    assert_eq!(diagnostic_snippets(&unit, &main), ["\"absent.sol\""]);
}

#[test]
fn an_unresolvable_import_is_reported_at_its_import_path() {
    let mut builder = builder();

    let main = contract("Main", &["present.sol", "?nowhere.sol"]);
    builder.add_file("main.sol".into(), main.clone());
    builder.add_file("present.sol".into(), contract("Present", &[]));

    let unit = builder.build();

    assert_eq!(diagnostic_codes(&unit), ["compilation/unresolved-import"]);
    assert_eq!(diagnostic_snippets(&unit, &main), ["\"?nowhere.sol\""]);
}

/// Files are parsed in parallel, so the amount of parallelism available must not
/// change what comes out: not the file order, not the node ids, and not the
/// diagnostics. Builds the same sources on pools of several sizes and compares
/// everything observable against the single-threaded result.
#[test]
fn output_is_independent_of_the_thread_count() {
    // Enough files to be spread over several threads, a few of them broken, so
    // that diagnostics take part in the comparison too.
    let sources: Vec<(FileId, String)> = (0..32)
        .map(|index| {
            let file_id: FileId = format!("file{index}.sol").into();
            let contents = match index % 4 {
                // Imports the next file, which exists.
                0 => contract(&format!("C{index}"), &[&format!("file{}.sol", index + 1)]),
                // Imports a file that was never added.
                1 => contract(&format!("C{index}"), &["absent.sol"]),
                // Fails to parse.
                2 => format!("pragma solidity ^0.8.0;\ncontract C{index} {{"),
                _ => contract(&format!("C{index}"), &[]),
            };
            (file_id, contents)
        })
        .collect();

    let build = || {
        let mut builder = builder();
        builder.add_files(sources.clone());
        builder.build()
    };

    let baseline = single_threaded_pool().install(build);
    let baseline_files: Vec<String> = baseline
        .files()
        .map(|file| file.id().as_str().to_owned())
        .collect();
    let baseline_diagnostics = format!("{:?}", baseline.diagnostics());
    let baseline_definitions: Vec<String> = baseline
        .all_definitions()
        .map(|definition| format!("{:?}", definition.node_id()))
        .collect();

    // Guard against the comparisons below being vacuous.
    assert!(!baseline.diagnostics().is_empty());
    assert!(!baseline_definitions.is_empty());
    assert_eq!(baseline_files.len(), sources.len());

    // The comparisons below only show that the thread count doesn't matter; they
    // would also hold if every run agreed on a *wrong* order. So pin the order
    // down in absolute terms as well: node ids are handed out file by file, so
    // walking files in id order must yield increasing root node ids.
    assert_root_node_ids_follow_file_order(&baseline);

    for threads in [2, 4, 8, 16] {
        let pool = rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build()
            .expect("thread pool builds");
        let unit = pool.install(build);

        let files: Vec<String> = unit
            .files()
            .map(|file| file.id().as_str().to_owned())
            .collect();
        assert_eq!(files, baseline_files, "file order diverged on {threads}");

        assert_eq!(
            format!("{:?}", unit.diagnostics()),
            baseline_diagnostics,
            "diagnostics diverged on {threads} threads"
        );

        let definitions: Vec<String> = unit
            .all_definitions()
            .map(|definition| format!("{:?}", definition.node_id()))
            .collect();
        assert_eq!(
            definitions, baseline_definitions,
            "node ids diverged on {threads} threads"
        );
    }
}

/// Asserts that the unit's files were lowered to IR in file-id order, which is
/// what makes node ids stable for a given source list.
fn assert_root_node_ids_follow_file_order(unit: &CompilationUnit) {
    let mut previous: Option<(String, NodeId)> = None;

    for file in unit.files() {
        let file_id = file.id().as_str().to_owned();
        let node_id = file.ast().node_id();

        if let Some((previous_file_id, previous_node_id)) = previous {
            assert!(
                previous_node_id < node_id,
                "'{previous_file_id}' has node id {previous_node_id:?}, \
                 but the later '{file_id}' has {node_id:?}"
            );
        }

        previous = Some((file_id, node_id));
    }
}

fn single_threaded_pool() -> rayon::ThreadPool {
    rayon::ThreadPoolBuilder::new()
        .num_threads(1)
        .build()
        .expect("thread pool builds")
}
