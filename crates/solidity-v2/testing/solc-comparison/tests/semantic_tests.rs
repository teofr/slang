//! Data-driven harness that runs slang v2 against solc's `libsolidity` semantic
//! tests, one case per `(version, test-file)` pair.
//!
//! This suite fetches an external dataset, so it's excluded from the default
//! `infra test` run and driven by `infra verify` instead (see the crate's
//! `Cargo.toml` and [`dataset::dataset_root`]). Each case asserts that slang's
//! result matches the checked-in per-(version, test) baseline: a fresh failure
//! is a regression, and a baselined case that now passes means the baseline is
//! stale.
//!
//! Outside CI the cases rewrite the baseline instead of asserting against it,
//! matching the repo's other snapshot tests — see [`dataset::is_update_mode`].

// Dependencies used only by the library, named here so the integration-test
// target doesn't trip `unused_crate_dependencies` (as the perf benches do).
use std::sync::{Mutex, OnceLock};

use anyhow as _;
use datatest_stable::Utf8Path;
use flate2 as _;
use infra_utils as _;
use rayon as _;
use semver as _;
use serde as _;
use serde_json as _;
use slang_solidity_v2 as _;
use slang_solidity_v2_common as _;
use solidity_testing_solc_comparison::baseline::Baseline;
use solidity_testing_solc_comparison::dataset::{self, HARNESS_PATTERN, dataset_root};
use solidity_testing_solc_comparison::runner::{self, Outcome};
use solidity_testing_utils as _;
use solidity_v2_testing_utils as _;
use tar as _;

/// The checked-in baseline, loaded once for the (read-only) checking path.
fn baseline() -> &'static Baseline {
    static BASELINE: OnceLock<Baseline> = OnceLock::new();
    BASELINE.get_or_init(|| Baseline::load().expect("failed to load baseline"))
}

/// In update mode, the baseline being updated.
fn baseline_updater() -> &'static Mutex<Baseline> {
    static UPDATER: OnceLock<Mutex<Baseline>> = OnceLock::new();
    UPDATER.get_or_init(|| Mutex::new(Baseline::load().expect("failed to load baseline")))
}

fn check(path: &Utf8Path) -> datatest_stable::Result<()> {
    let Some((version, relative_path)) = dataset::parse_version_and_relpath(path.as_std_path())
    else {
        return Err(format!("could not parse version/path from {path}").into());
    };

    let outcome = runner::run_test(path.as_std_path(), version);
    let failed = matches!(outcome, Outcome::Failed { .. });

    if dataset::is_update_mode() {
        // Record every case (a pass removes any stale entry, a failure adds
        // one) and rewrite only when the baseline actually changed.
        let mut baseline = baseline_updater().lock().unwrap();
        if baseline.record(version, &relative_path, failed) {
            baseline.write()?;
        }
        return Ok(());
    }

    let expected_failure = baseline().is_expected_failure(version, &relative_path);

    match outcome {
        // Compiles cleanly: fine unless the baseline still expects it to fail,
        // in which case the baseline is stale.
        Outcome::Passed => {
            if expected_failure {
                return Err(format!(
                    "`{relative_path}` at {version} is in the baseline but now passes. \
                     Regenerate the baseline by running the suite locally (outside CI)."
                )
                .into());
            }
            Ok(())
        }

        // Emits errors: fine only if the baseline already expects it; otherwise
        // it's a regression (valid Solidity that slang now rejects).
        Outcome::Failed { diagnostics } => {
            if expected_failure {
                Ok(())
            } else {
                Err(format!(
                    "regression at {version}: slang rejected valid Solidity `{relative_path}`.\n\
                     If this is intended, add it to the baseline by running the suite locally \
                     (outside CI).\n\n{}",
                    diagnostics.join("\n")
                )
                .into())
            }
        }
    }
}

datatest_stable::harness! {
    { test = check, root = dataset_root(), pattern = HARNESS_PATTERN },
}
