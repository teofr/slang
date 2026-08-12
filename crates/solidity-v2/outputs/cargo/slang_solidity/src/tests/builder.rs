use slang_solidity_v2_common::evm_targets::EvmTarget;

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
