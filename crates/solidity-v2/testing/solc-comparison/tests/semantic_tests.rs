//! Data-driven harness that runs slang v2 against solc's `libsolidity` semantic
//! tests, one case per `(version, test-file)` pair.
//!
//! This suite fetches an external dataset, so it's excluded from the default
//! `infra test` run and driven by `infra verify` instead (see the crate's
//! `Cargo.toml` and [`dataset::dataset_root`]). Each case asserts that slang's
//! result matches its checked-in per-(version, test) state across two baselines:
//!
//! - [`EXPECTED_FAILURES_FILE`]: tests slang doesn't compile cleanly.
//! - [`EXPECTED_UNTYPED_FILE`]: tests that compile cleanly but still have
//!   in-scope nodes slang doesn't type.
//!
//! A test not in either baseline is expected to compile cleanly *and* be fully
//! typed. Any mismatch — a fresh failure, a fresh untyped node, or a baselined
//! case that now does better — is reported so the baseline stays honest.
//!
//! Outside CI the cases rewrite the baselines instead of asserting against them
//! (keyed off `GitHub::is_running_in_ci`), matching the repo's other snapshot
//! tests.

// Dependencies used only by the library, named here so the integration-test
// target doesn't trip `unused_crate_dependencies` (as the perf benches do).
use std::sync::{LazyLock, Mutex};

use anyhow as _;
use datatest_stable::Utf8Path;
use flate2 as _;
use infra_utils::github::GitHub;
use rayon as _;
use semver as _;
use serde as _;
use serde_json as _;
use slang_solidity_v2 as _;
use slang_solidity_v2_common as _;
use solidity_testing_solc_comparison::baseline::{
    Baseline, EXPECTED_FAILURES_FILE, EXPECTED_UNTYPED_FILE,
};
use solidity_testing_solc_comparison::dataset::{self, HARNESS_PATTERN, dataset_root};
use solidity_testing_solc_comparison::runner::{self, Outcome};
use solidity_testing_utils as _;
use solidity_v2_testing_utils as _;
use tar as _;

/// The two checked-in baselines, loaded once for the (read-only) checking path.
struct Baselines {
    failures: Baseline,
    untyped: Baseline,
}

fn baselines() -> &'static Baselines {
    static BASELINES: LazyLock<Baselines> = LazyLock::new(|| Baselines {
        failures: Baseline::load(EXPECTED_FAILURES_FILE).expect("failed to load failures baseline"),
        untyped: Baseline::load(EXPECTED_UNTYPED_FILE).expect("failed to load untyped baseline"),
    });
    &BASELINES
}

/// In update mode, the baselines being rewritten. The `Mutex` serializes the
/// in-process test threads sharing these; `Baseline::record` adds the
/// cross-process file lock when it writes.
fn baseline_updater() -> &'static Mutex<Baselines> {
    static UPDATER: LazyLock<Mutex<Baselines>> = LazyLock::new(|| {
        Mutex::new(Baselines {
            failures: Baseline::load(EXPECTED_FAILURES_FILE)
                .expect("failed to load failures baseline"),
            untyped: Baseline::load(EXPECTED_UNTYPED_FILE)
                .expect("failed to load untyped baseline"),
        })
    });
    &UPDATER
}

fn check(path: &Utf8Path) -> datatest_stable::Result<()> {
    let Some((version, relative_path)) = dataset::parse_version_and_relpath(path.as_std_path())
    else {
        return Err(format!("could not parse version/path from {path}").into());
    };

    let outcome = runner::run_test(path.as_std_path(), version);
    let failed = matches!(outcome, Outcome::Failed { .. });
    let untyped = matches!(outcome, Outcome::Untyped { .. });

    if !GitHub::is_running_in_ci() {
        // Record every case into both baselines; each `record` adds or removes
        // the entry and writes that baseline back (under a file lock) only when
        // it actually changes. The two states are mutually exclusive, so
        // recording both keeps them disjoint.
        let mut baselines = baseline_updater().lock().unwrap();
        baselines.failures.record(version, &relative_path, failed)?;
        baselines.untyped.record(version, &relative_path, untyped)?;
        return Ok(());
    }

    let baselines = baselines();
    let expected_failure = baselines.failures.contains(version, &relative_path);
    let expected_untyped = baselines.untyped.contains(version, &relative_path);

    match outcome {
        // Compiles cleanly and fully typed: fine unless a baseline still expects
        // it to do worse, in which case that baseline is stale.
        Outcome::Passed => {
            if expected_failure {
                return Err(stale(&relative_path, version, "now compiles cleanly").into());
            }
            if expected_untyped {
                return Err(stale(&relative_path, version, "is now fully typed").into());
            }
            Ok(())
        }

        // Emits errors: fine only if the failures baseline already expects it;
        // otherwise it's a regression (valid Solidity that slang now rejects).
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

        // Compiles but has untyped nodes: fine only if the untyped baseline
        // already expects it. If it was expected to fail, the failures baseline
        // is stale; otherwise this is a fresh type-coverage regression.
        Outcome::Untyped { nodes } => {
            if expected_untyped {
                Ok(())
            } else if expected_failure {
                Err(stale(
                    &relative_path,
                    version,
                    "now compiles cleanly (but is not fully typed)",
                )
                .into())
            } else {
                Err(format!(
                    "type-coverage regression at {version}: `{relative_path}` compiles but has \
                     nodes with no type.\n\
                     If this is intended, add it to the baseline by running the suite locally \
                     (outside CI).\n\n{}",
                    nodes.join("\n")
                )
                .into())
            }
        }
    }
}

/// Message for a case that now does better than its baseline expects.
fn stale(relative_path: &str, version: impl std::fmt::Display, did: &str) -> String {
    format!(
        "`{relative_path}` at {version} is in a baseline but {did}. \
         Regenerate the baselines by running the suite locally (outside CI)."
    )
}

datatest_stable::harness! {
    { test = check, root = dataset_root(), pattern = HARNESS_PATTERN },
}
