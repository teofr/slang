use anyhow::Result;
use clap::{Parser, Subcommand};
use infra_utils::commands::Command;
use infra_utils::terminal::Terminal;

const SOLC_COMPARISON_CRATE: &str = "solidity_testing_solc_comparison";

#[derive(Clone, Debug, Default, Parser)]
pub struct VerifyController {
    #[clap(subcommand)]
    command: Option<VerifyCommand>,
}

impl VerifyController {
    // Returns `Result` for symmetry with the other command controllers, which
    // are all invoked with `?`.
    #[allow(clippy::unnecessary_wraps)]
    pub fn execute(&self) -> Result<()> {
        match &self.command {
            Some(VerifyCommand::SolcSemanticSuite { stats: true, .. }) => {
                solc_semantic_suite_stats();
            }
            Some(VerifyCommand::SolcSemanticSuite { passthrough, .. }) => {
                verify_solc_semantic_suite(passthrough);
            }
            None => verify_solc_semantic_suite(std::iter::empty::<String>()),
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Subcommand)]
enum VerifyCommand {
    /// Run slang against solc's 'libsolidity' semantic test suite (every
    /// supported version), checking that all of this (valid) Solidity still
    /// compiles without slang emitting errors.
    ///
    /// Downloads an external dataset (solc's own semantic tests) and guards
    /// against new validations in slang accidentally rejecting valid code.
    SolcSemanticSuite {
        /// Instead of running the suite, print a breakdown of the nodes slang
        /// leaves untyped across the whole corpus, tallied by node kind.
        #[arg(long)]
        stats: bool,
        #[arg(
            trailing_var_arg = true,
            help = "Passthrough arguments forwarded to `cargo test`."
        )]
        passthrough: Vec<String>,
    },
}

fn verify_solc_semantic_suite(passthrough: impl IntoIterator<Item = impl Into<String>>) {
    Terminal::step("verify solc-semantic-suite");

    // The suite is the `datatest-stable` harness (one case per (version, test)),
    // run via `cargo test` — i.e. in-process with threads. We deliberately don't
    // use nextest here: nextest is process-per-test by design (see
    // <https://nexte.st/docs/design/why-process-per-test/>), and spawning ~50k
    // processes is both slow and overwhelms nextest's list phase. In-process
    // execution runs the whole matrix in seconds. The debug build reuses the
    // cached `target/` artifacts.
    //
    // Whether this checks the baseline or rewrites it is decided by the `CI` env
    // var (like the repo's other snapshot tests): in CI it asserts against the
    // committed baseline; outside CI it regenerates it. See the solc-comparison
    // crate's `dataset::is_update_mode`.
    Command::new("cargo")
        .arg("test")
        .property("--package", SOLC_COMPARISON_CRATE)
        // `--` forwards to the test harness: `--quiet` keeps ~50k passing cases
        // to one char each instead of a line each; failures are still detailed.
        .arg("--")
        .flag("--quiet")
        .args(passthrough)
        .run();
}

fn solc_semantic_suite_stats() {
    Terminal::step("verify solc-semantic-suite --stats");

    // Reuses the same cached `target/` artifacts as the suite (debug profile),
    // so this only rebuilds the small `stats` binary. It walks the whole corpus
    // and tallies untyped nodes by kind — an on-demand view of the type-coverage
    // frontier, not part of the checked baseline.
    Command::new("cargo")
        .arg("run")
        .property("--package", SOLC_COMPARISON_CRATE)
        .property("--bin", "stats")
        .run();
}
