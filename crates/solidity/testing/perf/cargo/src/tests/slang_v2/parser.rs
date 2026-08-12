use rayon::iter::{IntoParallelIterator, ParallelIterator};
use slang_solidity_v2_cst::structured_cst::nodes::{SourceUnit, SourceUnitMember};
use slang_solidity_v2_parser::{ParseOutput, Parser};

use crate::dataset::SolidityProject;
use crate::tests::slang_v2::common::parse_version;

pub type Input = &'static SolidityProject;
pub type Output = Vec<(String, SourceUnit)>;

pub fn setup(project: &str) -> Input {
    crate::tests::setup::setup(project)
}

pub fn run(project: Input) -> Output {
    let lang_version = parse_version(project);
    let mut source_units = Vec::new();
    for (key, source) in &project.sources {
        let ParseOutput {
            source_unit,
            diagnostics,
        } = Parser::parse(&key.as_str().into(), source, lang_version);
        assert!(diagnostics.is_empty());

        source_units.push((key.clone(), source_unit));
    }
    assert!(!source_units.is_empty());

    source_units
}

pub fn test(project: Input) -> Output {
    run(project)
}

/// Parses the same files as [`run`], but across `rayon`'s current thread pool.
///
/// This mirrors the parse phase of `CompilationBuilder::build()`, down to using
/// an indexed `collect()` so results come back in source order. It exists so the
/// parse stage's own scaling can be measured, rather than only its diluted
/// contribution to the end-to-end build.
pub fn run_in_parallel(project: Input) -> Output {
    let lang_version = parse_version(project);

    let source_units: Output = project
        .sources
        .iter()
        .collect::<Vec<_>>()
        .into_par_iter()
        .map(|(key, source)| {
            let ParseOutput {
                source_unit,
                diagnostics,
            } = Parser::parse(&key.as_str().into(), source, lang_version);
            assert!(diagnostics.is_empty());

            (key.clone(), source_unit)
        })
        .collect();
    assert!(!source_units.is_empty());

    source_units
}

pub fn count_contracts(output: &Output) -> usize {
    let mut contract_count = 0;
    for (_, source_unit) in output {
        for member in &source_unit.members.elements {
            match member {
                SourceUnitMember::ContractDefinition(_)
                | SourceUnitMember::InterfaceDefinition(_)
                | SourceUnitMember::LibraryDefinition(_) => contract_count += 1,
                _ => {}
            }
        }
    }
    contract_count
}
