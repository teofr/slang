use anyhow::Result;
use infra_utils::paths::PathExtensions;
use slang_solidity_v2_parser::Parser as V2Parser;
use solidity_v2_testing_utils::cst_renderer::render;

use crate::snapshots::{NamedOutput, SnapshotStatus, run_snapshot};

pub(crate) fn run(parser_name: &str, test_name: &str) -> Result<()> {
    run_snapshot(
        "cst_output",
        "yml",
        parser_name,
        test_name,
        |input, version, _target| {
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
        },
    )?;
    Ok(())
}
