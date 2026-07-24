//! Prints a breakdown of the type-coverage frontier: how many nodes slang
//! leaves untyped across the whole corpus, tallied by node kind. Run via
//! `infra verify solc-semantic-suite --stats`.

// Normal dependencies used only by the library, named here so this binary target
// doesn't trip `unused_crate_dependencies` (as the `semantic_tests` harness does).
// `datatest-stable` is a dev-dependency (used by the `semantic_tests` harness);
// the bin's own test build links it, so name it under `cfg(test)` too.
use anyhow::Result;
#[cfg(test)]
use datatest_stable as _;
use flate2 as _;
use infra_utils as _;
use rayon as _;
use semver as _;
use serde as _;
use serde_json as _;
use slang_solidity_v2 as _;
use slang_solidity_v2_common as _;
use solidity_testing_solc_comparison::stats::collect_untyped_stats;
use solidity_testing_utils as _;
use solidity_v2_testing_utils as _;
use tar as _;

fn main() -> Result<()> {
    let stats = collect_untyped_stats()?;

    println!("untyped (version, test) pairs: {}", stats.untyped_pairs);
    println!(
        "total untyped node occurrences: {}\n",
        stats.total_occurrences()
    );
    println!(
        "{:<28} {:>12} {:>14}",
        "kind", "occurrences", "pairs w/ kind"
    );

    // Sort by occurrences, descending, so the biggest gaps come first.
    let mut rows: Vec<_> = stats.by_kind.iter().collect();
    rows.sort_by(|a, b| b.1.occurrences.cmp(&a.1.occurrences).then(a.0.cmp(b.0)));
    for (kind, counts) in rows {
        println!(
            "{:<28} {:>12} {:>14}",
            kind, counts.occurrences, counts.pairs
        );
    }
    Ok(())
}
