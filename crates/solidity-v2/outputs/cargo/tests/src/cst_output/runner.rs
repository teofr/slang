use anyhow::Result;
use infra_utils::paths::PathExtensions;
use slang_solidity_v2_common::evm_targets::EvmTarget;
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_parser::Parser as V2Parser;
use solidity_v2_testing_utils::cst_renderer::render;

use crate::snapshots::{NamedOutput, SnapshotInput, SnapshotRunner, SnapshotStatus, run_snapshot};

struct CstRunner;

impl SnapshotRunner for CstRunner {
    const OUTPUT_DIR: &'static str = "cst_output";
    const EXTENSION: &'static str = "yml";

    fn render(
        &self,
        input: &SnapshotInput<'_>,
        version: LanguageVersion,
        _target: EvmTarget,
    ) -> Result<Vec<NamedOutput>> {
        let source_id = input.path.strip_repo_root()?.unwrap_str();
        let file_id = source_id.into();

        let output = V2Parser::parse(&file_id, input.source, version);
        let (ok, contents) = render(input.source, source_id, &output);

        let status = if ok {
            SnapshotStatus::Success
        } else {
            SnapshotStatus::Failure
        };
        Ok(NamedOutput::single(status, contents))
    }
}

pub(crate) fn run(parser_name: &str, test_name: &str) -> Result<()> {
    run_snapshot(&CstRunner, parser_name, test_name)?;
    Ok(())
}
