use std::fmt::Write;
use std::ops::Range;

use anyhow::{Result, bail};

use super::report_data::ReportData;
use crate::snapshots::render::{SEPARATOR, SourceLabel, annotated_source, report_diagnostics};

/// Maximum number of characters shown for an expression's source snippet in its
/// label before it is truncated.
const MAX_SNIPPET_LENGTH: usize = 40;

pub(crate) fn typing_report(report_data: ReportData<'_>) -> Result<String> {
    let mut report = String::new();

    let ReportData {
        compilation,
        files,
        mut typings_by_file,
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

    for (index, file) in compilation.files().enumerate() {
        if index > 0 {
            writeln!(report, "{SEPARATOR}")?;
        }

        let Some(contents) = files.get(file.id()) else {
            bail!(
                "missing source contents for file {file_id}",
                file_id = file.id()
            );
        };

        // Consume the collected typings so their ranges move into the labels
        // without cloning.
        let typings = typings_by_file.remove(file.id()).unwrap_or_default();

        // Each label carries the node's own source snippet, so its type stays
        // unambiguous even where the ariadne underlines of nested expressions
        // overlap.
        let labels: Vec<SourceLabel> = typings
            .into_iter()
            .map(|typing| {
                let snippet = render_snippet(contents, &typing.range);
                let message = format!("`{snippet}`: {type_}", type_ = typing.type_label());
                (typing.range, message)
            })
            .collect();

        annotated_source(&mut report, "Typings", file.id(), contents, &labels)?;
    }

    Ok(report)
}

fn render_snippet(contents: &str, range: &Range<usize>) -> String {
    let text = contents.get(range.clone()).unwrap_or_default();
    let char_count = text.chars().count();
    let mut snippet: String = text.chars().take(MAX_SNIPPET_LENGTH).collect();
    if char_count > MAX_SNIPPET_LENGTH {
        snippet.push_str("...");
    }
    snippet
        .replace('\t', "\\t")
        .replace('\r', "\\r")
        .replace('\n', "\\n")
}
