use anyhow::Result;
use slang_solidity_v2_common::evm_targets::EvmTarget;
use slang_solidity_v2_common::versions::LanguageVersion;

use super::report::binder_report;
use super::report_data::ReportData;
use crate::snapshots::{
    NamedOutput, SnapshotInput, SnapshotRunner, SnapshotStatus, build_compilation_unit, files_of,
};

struct BinderRunner;

impl SnapshotRunner for BinderRunner {
    const OUTPUT_DIR: &'static str = "binder_output";
    const EXTENSION: &'static str = "txt";

    fn render(
        &self,
        input: &SnapshotInput<'_>,
        version: LanguageVersion,
        target: EvmTarget,
    ) -> Result<Vec<NamedOutput>> {
        let files = files_of(input.source);
        let compilation = build_compilation_unit(&files, version, target);
        let report_data = ReportData::prepare(&compilation, &files);

        let status = if report_data.all_resolved() {
            SnapshotStatus::Success
        } else {
            SnapshotStatus::Failure
        };

        let contents = binder_report(&report_data)?;
        Ok(NamedOutput::single(status, contents))
    }
}

pub(crate) fn run(group_name: &str, test_name: &str) -> Result<()> {
    BinderRunner.run(group_name, test_name)
}
