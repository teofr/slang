use std::fmt::Write;

use anyhow::{Result, bail};
use infra_utils::cargo::CargoWorkspace;
use semver::Version;
use slang_solidity_v2_common::collections::SortedSet;
use slang_solidity_v2_common::versions::LanguageVersion;

use super::targets::{SlangTarget, SolcTarget, TestTarget};
use crate::snapshots::render::diagnostics_summary;
use crate::snapshots::{
    CellOutcome, NamedOutput, SnapshotStatus, TestConfig, TestMatrix, files_of, run_snapshot,
};

fn named_output(name: &str, errors: &[String]) -> NamedOutput {
    let status = if errors.is_empty() {
        SnapshotStatus::Success
    } else {
        SnapshotStatus::Failure
    };
    NamedOutput {
        name: Some(name.to_string()),
        status,
        contents: diagnostics_summary(errors),
    }
}

fn status_of(cell: &CellOutcome, name: &str) -> Option<SnapshotStatus> {
    cell.outputs
        .iter()
        .find(|output| output.name.as_deref() == Some(name))
        .map(|output| output.status)
}

/// Assert slang and solc agree on the success/failure status of every cell. The
/// golden files themselves are informational; this agreement is the actual
/// test.
fn check_agreement(
    cells: &[CellOutcome],
    matrix: TestMatrix,
    test_path: &str,
    slang: &str,
    solc: &str,
) -> Result<()> {
    if cells
        .iter()
        .all(|cell| status_of(cell, slang) == status_of(cell, solc))
    {
        return Ok(());
    }

    let mut message = String::new();
    writeln!(
        message,
        "slang and solc disagree on the compilation status of `{test_path}`."
    )?;
    writeln!(message)?;

    for cell in cells {
        let label = match matrix {
            TestMatrix::SingleTargetAllVersions { .. } => cell.version.to_string(),
            TestMatrix::SingleVersionAllTargets { .. } => cell.target.to_string(),
        };
        let slang_status = status_of(cell, slang);
        let solc_status = status_of(cell, solc);
        let outcome = if slang_status == solc_status {
            "match"
        } else {
            "differ"
        };
        writeln!(
            message,
            "  {label}: slang={slang_status:?}, solc={solc_status:?} ({outcome})"
        )?;
    }

    bail!(message)
}

/// Does one-time setup before delegating to `run_snapshot`: resolves the config
/// to learn which solc versions the matrix will cover, then fetches those solc
/// binaries once via `SolcTarget::new` (a network call that would be wasteful to
/// repeat per cell). The `render` closure captures both targets, and afterwards
/// we check the captured cells for slang/solc agreement.
pub(crate) fn run(group_name: &str, test_name: &str) -> Result<()> {
    let test_dir = CargoWorkspace::locate_source_crate("solidity_v2_testing_snapshots")?
        .join("diagnostics_output")
        .join(group_name)
        .join(test_name);

    let config = TestConfig::resolve(&test_dir)?;
    let solc_versions: SortedSet<Version> = match config.matrix {
        TestMatrix::SingleTargetAllVersions { .. } => {
            LanguageVersion::ALL.iter().map(|v| (*v).into()).collect()
        }
        TestMatrix::SingleVersionAllTargets { version } => SortedSet::from_iter([version.into()]),
    };

    let slang = SlangTarget;
    let solc = SolcTarget::new(solc_versions)?;

    let cells = run_snapshot(
        "diagnostics_output",
        "txt",
        group_name,
        test_name,
        |input, version, target| {
            let files = files_of(input.source);

            let slang_errors = slang.collect_diagnostics(&files, version, target)?;
            let solc_errors = solc.collect_diagnostics(&files, version, target)?;

            Ok(vec![
                named_output(slang.name(), &slang_errors),
                named_output(solc.name(), &solc_errors),
            ])
        },
    )?;

    check_agreement(
        &cells,
        config.matrix,
        &format!("{group_name}/{test_name}"),
        slang.name(),
        solc.name(),
    )
}
