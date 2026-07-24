use std::path::PathBuf;
use std::sync::OnceLock;

use anyhow::{Context, Result};
use infra_utils::cargo::CargoWorkspace;
use infra_utils::paths::PathExtensions;
use serde::{Deserialize, Serialize};
use slang_solidity_v2_common::collections::{SortedMap, SortedSet};
use slang_solidity_v2_common::versions::LanguageVersion;

const CRATE_NAME: &str = "solidity_testing_solc_comparison";
const EXPECTED_FAILURES_FILE: &str = "expected-failures.json";

/// The set of tests expected to currently fail, grouped by the Solidity version
/// they fail at.
#[derive(Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Baseline {
    failures: SortedMap<LanguageVersion, SortedSet<String>>,
}

impl Baseline {
    /// Loads the checked-in baseline. The file is committed to the repo, so its
    /// absence is a real error (a broken checkout or a wrong working directory)
    /// rather than an empty baseline — a silent default would turn every
    /// expected failure into a spurious regression.
    pub fn load() -> Result<Self> {
        let path = expected_failures_path()?;
        let contents = path
            .read_to_string()
            .with_context(|| format!("failed to read the checked-in baseline at {path:?}"))?;
        Ok(serde_json::from_str(&contents)?)
    }

    /// Whether `test_path` is expected to fail at `version`.
    pub fn is_expected_failure(&self, version: LanguageVersion, test_path: &str) -> bool {
        self.failures
            .get(&version)
            .is_some_and(|paths| paths.contains(test_path))
    }

    /// Records that `test_path` did or didn't fail at `version`, adding or
    /// removing it from the baseline accordingly. Returns whether this actually
    /// changed the baseline (so the caller can avoid rewriting an unchanged
    /// file).
    pub fn record(&mut self, version: LanguageVersion, test_path: &str, failed: bool) -> bool {
        if failed {
            self.failures
                .entry(version)
                .or_default()
                .insert(test_path.to_owned())
        } else if let Some(paths) = self.failures.get_mut(&version) {
            let changed = paths.remove(test_path);
            if paths.is_empty() {
                self.failures.remove(&version);
            }
            changed
        } else {
            false
        }
    }

    /// Writes the baseline back to `expected-failures.json`.
    pub fn write(&self) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        expected_failures_path()?.write_string(format!("{json}\n"))
    }
}

/// Path to the checked-in baseline, located via the shared cargo-workspace
/// helper (which resolves the crate's source directory from the workspace
/// manifest).
fn expected_failures_path() -> Result<PathBuf> {
    static CRATE_DIR: OnceLock<Result<PathBuf, String>> = OnceLock::new();
    let crate_dir = CRATE_DIR
        .get_or_init(|| CargoWorkspace::locate_source_crate(CRATE_NAME).map_err(|e| e.to_string()))
        .as_ref()
        .map_err(|e| anyhow::anyhow!("{e}"))?;
    Ok(crate_dir.join(EXPECTED_FAILURES_FILE))
}
