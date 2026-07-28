use std::fmt::Write;

use anyhow::{Result, bail};
use infra_utils::cargo::CargoWorkspace;
use semver::Version;
use slang_solidity_v2_common::collections::SortedSet;
use slang_solidity_v2_common::evm_targets::EvmTarget;
use slang_solidity_v2_common::versions::LanguageVersion;

use super::targets::{SlangTarget, SolcTarget, TestTarget};
use crate::snapshots::render::diagnostics_summary;
use crate::snapshots::{
    CellOutcome, NamedOutput, SnapshotInput, SnapshotRunner, SnapshotStatus, TestConfig,
    TestMatrix, files_of, run_snapshot,
};

struct DiagnosticsRunner {
    slang: SlangTarget,
    solc: SolcTarget,
    matrix: TestMatrix,
    /// `group/test`, only used to label a disagreement failure.
    test_path: String,
}

impl SnapshotRunner for DiagnosticsRunner {
    const OUTPUT_DIR: &'static str = "diagnostics_output";
    const EXTENSION: &'static str = "txt";

    fn render(
        &self,
        input: &SnapshotInput<'_>,
        version: LanguageVersion,
        target: EvmTarget,
    ) -> Result<Vec<NamedOutput>> {
        let files = files_of(input.source);

        let slang_errors = self.slang.collect_diagnostics(&files, version, target)?;
        let solc_errors = self.solc.collect_diagnostics(&files, version, target)?;

        Ok(vec![
            named_output(self.slang.name(), &slang_errors),
            named_output(self.solc.name(), &solc_errors),
        ])
    }

    /// Assert slang and solc agree on the success/failure status of every cell.
    /// The golden files themselves are informational; this agreement is the
    /// actual test.
    fn finish(&self, cells: &[CellOutcome]) -> Result<()> {
        let slang = self.slang.name();
        let solc = self.solc.name();

        if cells
            .iter()
            .all(|cell| status_of(cell, slang) == status_of(cell, solc))
        {
            return Ok(());
        }

        let mut message = String::new();
        writeln!(
            message,
            "slang and solc disagree on the compilation status of `{path}`.",
            path = self.test_path,
        )?;
        writeln!(message)?;

        for cell in cells {
            let label = match self.matrix {
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
}

fn named_output(name: &str, errors: &[String]) -> NamedOutput {
    let status = if errors.is_empty() {
        SnapshotStatus::Success
    } else {
        SnapshotStatus::Failure
    };
    NamedOutput {
        name: name.to_string(),
        status,
        contents: diagnostics_summary(errors),
    }
}

fn status_of(cell: &CellOutcome, name: &str) -> Option<SnapshotStatus> {
    cell.outputs
        .iter()
        .find(|output| output.name == name)
        .map(|output| output.status)
}

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

    let runner = DiagnosticsRunner {
        slang: SlangTarget,
        solc: SolcTarget::new(solc_versions)?,
        matrix: config.matrix,
        test_path: format!("{group_name}/{test_name}"),
    };

    run_snapshot(&runner, group_name, test_name)
}
