//! Shared snapshot infrastructure for v2 cargo test runners.
//!
//! Each snapshot kind supplies a `render` closure and its output directory and
//! file extension to [`run_snapshot`], which handles everything shared:
//! locating the test, reading `input.sol`, iterating the version/target matrix,
//! collapsing unchanged consecutive outputs, and writing through
//! [`CodegenFileSystem`].

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
/// A kind usually produces a single unnamed output (`name` is `None`) written
/// into the test's `generated/` directory. Kinds that emit several parallel
/// streams — like `diagnostics_output` (slang vs solc) — return one
/// `NamedOutput` per stream, each written into its own `generated/<name>/`
/// subdirectory.
pub(crate) struct NamedOutput {
    pub name: Option<String>,
    pub status: SnapshotStatus,
    pub contents: String,
}

impl NamedOutput {
    /// A single unnamed output, written directly into `generated/`.
    pub(crate) fn single(status: SnapshotStatus, contents: String) -> Vec<Self> {
        vec![Self {
            name: None,
            status,
            contents,
        }]
    }
}

/// The outputs a `render` closure produced for one `(version, target)` matrix
/// cell, returned by [`run_snapshot`] for any cross-cell assertions the caller
/// needs.
pub(crate) struct CellOutcome {
    pub version: LanguageVersion,
    pub target: EvmTarget,
    pub outputs: Vec<NamedOutput>,
}

/// The `input.sol` under test, handed to each `render` closure.
pub(crate) struct SnapshotInput<'a> {
    pub path: &'a Path,
    pub source: &'a str,
}

/// Drives the full matrix for one test, invoking `render` once per
/// `(version, target)` cell, writing the golden files under `output_dir` with
/// the given file `extension`, and returning every cell's outcome so the caller
/// can run any cross-cell checks it needs (eg. `diagnostics_output` comparing
/// streams).
///
/// `output_dir` is the snapshot suite directory under `testing/snapshots/` (eg.
/// `"binder_output"`); `extension` is the golden file extension (eg. `"txt"`,
/// `"yml"`).
pub(crate) fn run_snapshot(
    output_dir: &str,
    extension: &str,
    group: &str,
    test: &str,
    render: impl Fn(&SnapshotInput<'_>, LanguageVersion, EvmTarget) -> Result<Vec<NamedOutput>>,
) -> Result<Vec<CellOutcome>> {
    let test_dir = CargoWorkspace::locate_source_crate("solidity_v2_testing_snapshots")?
        .join(output_dir)
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
    let mut last_contents: Map<Option<String>, String> = Map::default();
    let mut outcomes: Vec<CellOutcome> = Vec::with_capacity(cells.len());

    for (version, target) in cells {
        let outputs = render(&input, version, target)?;

        for output in &outputs {
            if last_contents.get(&output.name).map(String::as_str) != Some(output.contents.as_str())
            {
                let filename =
                    snapshot_filename(config.matrix, version, target, output.status, extension);
                let dir = match &output.name {
                    Some(name) => test_dir.join("generated").join(name),
                    None => test_dir.join("generated"),
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

    Ok(outcomes)
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
