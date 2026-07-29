use anyhow::Result;

use super::report::typing_report;
use super::report_data::ReportData;
use crate::snapshots::{
    NamedOutput, SnapshotStatus, build_compilation_unit, files_of, run_snapshot,
};

pub(crate) fn run(group_name: &str, test_name: &str) -> Result<()> {
    run_snapshot(
        "typing_output",
        "txt",
        group_name,
        test_name,
        |input, version, target| {
            let files = files_of(input.source);
            let compilation = build_compilation_unit(&files, version, target);
            let report_data = ReportData::prepare(&compilation, &files);

            let status = if report_data.is_success() {
                SnapshotStatus::Success
            } else {
                SnapshotStatus::Failure
            };

            let contents = typing_report(report_data)?;
            Ok(NamedOutput::single(status, contents))
        },
    )?;
    Ok(())
}
