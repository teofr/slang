//! Shared snapshot infrastructure for v2 cargo test runners.
//!
//! Every snapshot kind implements [`SnapshotRunner`], supplying only its
//! per-cell [`render`](SnapshotRunner::render) and, if needed, a cross-cell
//! [`finish`](SnapshotRunner::finish) check. The provided
//! [`run`](SnapshotRunner::run) handles everything shared: locating the test,
//! reading `input.sol`, iterating the version/target matrix, collapsing
//! unchanged consecutive outputs, and writing through [`CodegenFileSystem`].

mod compilation;
mod config;
pub(crate) mod render;

use std::path::Path;

use anyhow::Result;
pub(crate) use compilation::{build_compilation_unit, files_of};
pub(crate) use config::{TestConfig, TestMatrix};
use infra_utils::cargo::CargoWorkspace;
use infra_utils::codegen::CodegenFileSystem;
use infra_utils::paths::PathExtensions;
use slang_solidity_v2_common::collections::Map;
use slang_solidity_v2_common::evm_targets::EvmTarget;
use slang_solidity_v2_common::versions::LanguageVersion;

/// Whether a single snapshot output represents a successful or failed analysis.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SnapshotStatus {
    Success,
    Failure,
}

/// One rendered output of a snapshot cell.
///
/// A kind usually produces a single output written into the test's `generated/`
/// directory (`name` empty). Kinds that emit several parallel streams — like
/// `diagnostics_output` (slang vs solc) — return one `NamedOutput` per stream,
/// each written into its own `generated/<name>/` subdirectory.
pub(crate) struct NamedOutput {
    pub name: String,
    pub status: SnapshotStatus,
    pub contents: String,
}

impl NamedOutput {
    /// A single unnamed output, written directly into `generated/`.
    pub(crate) fn single(status: SnapshotStatus, contents: String) -> Vec<Self> {
        vec![Self {
            name: String::new(),
            status,
            contents,
        }]
    }
}

/// The outputs a runner produced for one `(version, target)` matrix cell,
/// passed to [`SnapshotRunner::finish`] for cross-cell assertions.
pub(crate) struct CellOutcome {
    pub version: LanguageVersion,
    pub target: EvmTarget,
    pub outputs: Vec<NamedOutput>,
}

/// The `input.sol` under test, handed to each [`SnapshotRunner::render`].
pub(crate) struct SnapshotInput<'a> {
    pub path: &'a Path,
    pub source: &'a str,
}

pub(crate) trait SnapshotRunner {
    /// The snapshot suite directory under `testing/snapshots/` (eg.
    /// `"binder_output"`).
    const OUTPUT_DIR: &'static str;

    /// The golden file extension (eg. `"txt"`, `"yml"`).
    const EXTENSION: &'static str;

    /// Renders the snapshot output(s) for a single matrix cell.
    fn render(
        &self,
        input: &SnapshotInput<'_>,
        version: LanguageVersion,
        target: EvmTarget,
    ) -> Result<Vec<NamedOutput>>;

    /// Optional cross-cell assertion run after the whole matrix (eg. checking
    /// that two streams agree). Defaults to a no-op.
    fn finish(&self, _cells: &[CellOutcome]) -> Result<()> {
        Ok(())
    }
}

/// Drives the full matrix for one test against a [`SnapshotRunner`].
///
/// This is a free function rather than a provided trait method precisely so a
/// kind can't override the shared plumbing — it only supplies `render`,
/// `finish`, and the associated constants.
pub(crate) fn run_snapshot<R: SnapshotRunner + ?Sized>(
    runner: &R,
    group: &str,
    test: &str,
) -> Result<()> {
    let test_dir = CargoWorkspace::locate_source_crate("solidity_v2_testing_snapshots")?
        .join(R::OUTPUT_DIR)
        .join(group)
        .join(test);
    let mut fs = CodegenFileSystem::default();

    let input_path = test_dir.join("input.sol");
    let source = input_path.read_to_string()?;
    let input = SnapshotInput {
        path: &input_path,
        source: &source,
    };

    let config = TestConfig::resolve(&test_dir)?;

    // Whichever axis the config varies; the other is pinned.
    let cells: Vec<(LanguageVersion, EvmTarget)> = match config.matrix {
        TestMatrix::SingleTargetAllVersions { target } => {
            LanguageVersion::ALL.iter().map(|&v| (v, target)).collect()
        }
        TestMatrix::SingleVersionAllTargets { version } => {
            EvmTarget::ALL.iter().map(|&t| (version, t)).collect()
        }
    };

    // The last-written contents for each output stream, so consecutive
    // unchanged cells collapse into a single golden file (per stream).
    let mut last_contents: Map<String, String> = Map::default();
    let mut outcomes: Vec<CellOutcome> = Vec::with_capacity(cells.len());

    for (version, target) in cells {
        let outputs = runner.render(&input, version, target)?;

        for output in &outputs {
            if last_contents.get(&output.name).map(String::as_str) != Some(output.contents.as_str())
            {
                let filename =
                    snapshot_filename(config.matrix, version, target, output.status, R::EXTENSION);
                let dir = if output.name.is_empty() {
                    test_dir.join("generated")
                } else {
                    test_dir.join("generated").join(&output.name)
                };
                fs.write_file_raw(dir.join(filename), &output.contents)?;
                last_contents.insert(output.name.clone(), output.contents.clone());
            }
        }

        outcomes.push(CellOutcome {
            version,
            target,
            outputs,
        });
    }

    runner.finish(&outcomes)
}

fn snapshot_filename(
    matrix: TestMatrix,
    version: LanguageVersion,
    target: EvmTarget,
    status: SnapshotStatus,
    extension: &str,
) -> String {
    let status = match status {
        SnapshotStatus::Success => "success",
        SnapshotStatus::Failure => "failure",
    };

    match matrix {
        TestMatrix::SingleTargetAllVersions { .. } => {
            format!("{version}-{status}.{extension}")
        }
        TestMatrix::SingleVersionAllTargets { .. } => {
            let index = target as u32;
            let name = target.to_string().to_lowercase();
            format!("{index:02}-{name}-{status}.{extension}")
        }
    }
}
