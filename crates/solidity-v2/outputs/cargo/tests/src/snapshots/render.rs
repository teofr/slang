//! Consolidated renderers shared across snapshot kinds: the section separator,
//! parse-error rendering, an ariadne-annotated source block, and a diagnostics
//! summary.

use std::fmt::Write;
use std::ops::Range;

use anyhow::Result;
use ariadne::{Color, Config, Label, Report, ReportBuilder, ReportKind, Source};
use slang_solidity_v2::compilation::FileId;
use slang_solidity_v2::diagnostics::DiagnosticCollection;
use slang_solidity_v2_common::collections::SortedMap;
use solidity_v2_testing_utils::reporting::diagnostic;

pub(crate) const SEPARATOR: &str =
    "\n------------------------------------------------------------------------\n";

/// A label to render on the annotated source: a byte range and its message.
pub(crate) type SourceLabel = (Range<usize>, String);

type Span<'a> = (&'a str, Range<usize>);

/// Renders the compilation's parse errors under the given heading.
pub(crate) fn report_diagnostics(
    report: &mut String,
    heading: &str,
    diagnostics: &DiagnosticCollection,
    file_contents: &SortedMap<FileId, String>,
) -> Result<()> {
    writeln!(report, "{heading}")?;
    for diagnostic in diagnostics {
        let file_id = diagnostic.file_id();
        let source = file_contents.get(file_id).cloned().unwrap_or_default();
        let rendered = diagnostic::render(diagnostic, file_id.as_str(), &source, false);
        writeln!(report, "{rendered}")?;
    }
    Ok(())
}

/// Renders a `Diagnostics: N` summary followed by each already-rendered
/// diagnostic (used by `diagnostics_output`).
pub(crate) fn diagnostics_summary(rendered: &[String]) -> String {
    let mut contents = String::new();
    writeln!(contents, "Diagnostics: {count}", count = rendered.len()).unwrap();
    for diagnostic in rendered {
        writeln!(contents).unwrap();
        writeln!(contents, "{diagnostic}").unwrap();
    }
    contents
}

/// Appends an ariadne report annotating `contents` with the given labels, under
/// a custom report kind (eg. `"Bindings"`, `"Typings"`).
pub(crate) fn annotated_source(
    report: &mut String,
    kind: &'static str,
    file_id: &FileId,
    contents: &str,
    labels: &[SourceLabel],
) -> Result<()> {
    let file_id_str = file_id.as_str();
    let mut builder: ReportBuilder<'_, Span<'_>> =
        Report::build(ReportKind::Custom(kind, Color::Unset), file_id_str, 0)
            .with_config(Config::default().with_color(false));

    for (range, message) in labels {
        // ariadne works with character offsets, not byte offsets, so we need to
        // convert ranges.
        // TODO: the next ariadne release should allow byte offsets (see
        // https://github.com/NomicFoundation/slang/issues/1536)
        let char_range = {
            let start = contents[..range.start].chars().count();
            let end = contents[..range.end].chars().count();
            start..end
        };
        builder.add_label(Label::new((file_id_str, char_range)).with_message(message));
    }

    let mut buffer = Vec::<u8>::new();
    builder
        .finish()
        .write((file_id_str, Source::from(contents)), &mut buffer)?;
    report.extend(String::from_utf8(buffer));

    Ok(())
}
