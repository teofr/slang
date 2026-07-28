//! Data-driven harness that runs slang v2 against the **known-valid** subset of
//! solc's `syntaxTests` corpus, one case per `(version, test-file)` pair.
//!
//! `syntaxTests` cases are compile-time tests whose `// ----` trailer lists the
//! diagnostics solc expects. A case with **no** error/warning expectation is
//! Solidity solc accepts, so — like the `semanticTests` suite — slang should
//! accept it too. Cases that *do* declare expected diagnostics are out of scope
//! here (they'd exercise the reverse "slang rejects what solc rejects"
//! direction) and are skipped.
//!
//! Like the semantic suite, this fetches an external dataset, so it's excluded
//! from the default `infra test` run and driven by `infra verify` instead; and
//! outside CI the cases rewrite the baseline instead of asserting against it
//! (keyed off `GitHub::is_running_in_ci`).

// Dependencies used only by the library, named here so the integration-test
// target doesn't trip `unused_crate_dependencies`.
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
use solidity_testing_solc_comparison::baseline::{Baseline, EXPECTED_SYNTAX_FAILURES_FILE};
use solidity_testing_solc_comparison::runner::{self, Outcome};
use solidity_testing_solc_comparison::{dataset, syntax};
use solidity_testing_utils as _;
use solidity_v2_testing_utils as _;
use tar as _;

/// The checked-in syntax baseline, loaded once for the (read-only) checking path.
fn baseline() -> &'static Baseline {
    static BASELINE: LazyLock<Baseline> = LazyLock::new(|| {
        Baseline::load(EXPECTED_SYNTAX_FAILURES_FILE).expect("failed to load syntax baseline")
    });
    &BASELINE
}

/// In update mode, the baseline being updated. The `Mutex` serializes the
/// in-process test threads sharing this one instance; `Baseline::record` adds
/// the cross-process file lock when it writes.
fn baseline_updater() -> &'static Mutex<Baseline> {
    static UPDATER: LazyLock<Mutex<Baseline>> = LazyLock::new(|| {
        Mutex::new(
            Baseline::load(EXPECTED_SYNTAX_FAILURES_FILE).expect("failed to load syntax baseline"),
        )
    });
    &UPDATER
}

fn check(path: &Utf8Path) -> datatest_stable::Result<()> {
    let Some((version, relative_path)) = syntax::parse_version_and_relpath(path.as_std_path())
    else {
        return Err(format!("could not parse version/path from {path}").into());
    };

    // Only the known-valid (trailer-free) cases are in scope: a case that
    // declares expected diagnostics is one solc rejects/warns on, which is the
    // reverse direction and not checked here.
    let contents = std::fs::read_to_string(path.as_std_path())
        .map_err(|error| format!("failed to read {path}: {error}"))?;
    if syntax::has_error_expectations(&contents) {
        return Ok(());
    }

    let outcome = runner::run_test(path.as_std_path(), version);
    let failed = matches!(outcome, Outcome::Failed { .. });

    if !GitHub::is_running_in_ci() {
        // `record` writes the baseline back under a file lock whenever it
        // actually changes; seeded from the committed baseline so a filtered
        // run only touches the cases that actually ran.
        let mut baseline = baseline_updater().lock().unwrap();
        baseline.record(version, &relative_path, failed)?;
        return Ok(());
    }

    let expected_failure = baseline().is_expected_failure(version, &relative_path);

    match outcome {
        // Compiles cleanly: fine unless the baseline still expects it to fail.
        Outcome::Passed => {
            if expected_failure {
                return Err(format!(
                    "`{relative_path}` at {version} is in the syntax baseline but now passes. \
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

// Reuses the shared [`dataset::dataset_root`], which downloads each version's
// tarball once and extracts both the `semanticTests` and `syntaxTests` trees;
// this harness just filters to the latter via its own pattern.
datatest_stable::harness! {
    { test = check, root = dataset::dataset_root(), pattern = syntax::HARNESS_PATTERN },
}
