use std::fmt::Write;

use anyhow::Result;
use slang_solidity_v2::compilation::FileId;

use super::report_data::{
    CollectedDefinition, CollectedIdentifier, CollectedReference, CollectedResolution, ReportData,
};
use crate::snapshots::render::{SEPARATOR, SourceLabel, annotated_source, report_diagnostics};

pub(crate) fn binder_report(report_data: &'_ ReportData<'_>) -> Result<String> {
    let mut report = String::new();

    let ReportData {
        compilation,
        files,
        all_definitions,
        all_references,
        unbound_identifiers,
    } = report_data;

    if !compilation.diagnostics().is_empty() {
        report_diagnostics(
            &mut report,
            "Parse errors:",
            compilation.diagnostics(),
            files,
        )?;
        writeln!(report, "{SEPARATOR}")?;
    }

    report_all_definitions(&mut report, all_definitions)?;

    writeln!(report, "{SEPARATOR}")?;

    report_all_references(&mut report, all_references)?;

    writeln!(report, "{SEPARATOR}")?;

    report_unbound_identifiers(&mut report, unbound_identifiers)?;

    for file in compilation.files() {
        writeln!(report, "{SEPARATOR}")?;

        if let Some(contents) = files.get(file.id()) {
            render_bindings_for_file(
                &mut report,
                file.id(),
                contents,
                all_definitions,
                all_references,
                unbound_identifiers,
            )?;
        }
    }

    Ok(report)
}

fn report_all_definitions(
    report: &mut String,
    all_definitions: &[CollectedDefinition],
) -> Result<()> {
    writeln!(
        report,
        "Definitions ({definitions_count}):",
        definitions_count = all_definitions.len(),
    )?;
    for definition in all_definitions {
        writeln!(report, "- {definition}")?;
    }
    Ok(())
}

fn report_all_references(report: &mut String, all_references: &[CollectedReference]) -> Result<()> {
    writeln!(
        report,
        "References ({references_count}):",
        references_count = all_references.len()
    )?;
    for reference in all_references {
        writeln!(report, "- {reference}")?;
    }
    Ok(())
}

fn report_unbound_identifiers(
    report: &mut String,
    unbound_identifiers: &[CollectedIdentifier],
) -> Result<()> {
    writeln!(
        report,
        "Unbound identifiers ({unbound_identifiers_count}):",
        unbound_identifiers_count = unbound_identifiers.len()
    )?;
    for unbound_identifier in unbound_identifiers {
        writeln!(report, "- {unbound_identifier}")?;
    }
    Ok(())
}

fn render_bindings_for_file(
    report: &mut String,
    file_id: &FileId,
    contents: &str,
    all_definitions: &[CollectedDefinition],
    all_references: &[CollectedReference],
    unbound_identifiers: &[CollectedIdentifier],
) -> Result<()> {
    let mut labels: Vec<SourceLabel> = Vec::new();

    for definition in all_definitions {
        if definition.identifier.file_id() != file_id {
            continue;
        }

        let message = format!(
            "name: {definition_id}",
            definition_id = definition.definition_id,
        );
        labels.push((definition.identifier.range().clone(), message));
    }

    for reference in all_references {
        if reference.identifier.file_id() != file_id {
            continue;
        }

        let message = match &reference.resolution {
            CollectedResolution::Unresolved => "unresolved".to_string(),
            CollectedResolution::BuiltIn => "built-in".to_string(),
            CollectedResolution::Definition(definition_id) => {
                format!("ref: {definition_id}")
            }
        };
        labels.push((reference.identifier.range().clone(), message));
    }

    for unbound_identifier in unbound_identifiers {
        if unbound_identifier.file_id() != file_id {
            continue;
        }

        labels.push((unbound_identifier.range().clone(), "???".to_string()));
    }

    annotated_source(report, "Bindings", file_id, contents, &labels)
}
