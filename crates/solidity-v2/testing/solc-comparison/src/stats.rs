//! On-demand analysis of the type-coverage frontier: runs the whole corpus and
//! tallies the untyped nodes by kind. Surfaced via `infra verify
//! solc-semantic-suite --stats`; not part of the checked baseline.

use anyhow::Result;
use slang_solidity_v2_common::collections::{SortedMap, SortedSet};

use crate::dataset::fetch_all_versions;
use crate::runner::{Outcome, run_test};

/// How many untyped nodes of a given kind were seen, and in how many distinct
/// `(version, test)` pairs at least one such node appears.
#[derive(Default)]
pub struct KindStats {
    /// Total untyped nodes of this kind across the corpus.
    pub occurrences: u64,
    /// Distinct `(version, test)` pairs containing at least one.
    pub pairs: u64,
}

/// The untyped-node tally across every supported version's test corpus.
pub struct UntypedStats {
    /// Distinct `(version, test)` pairs with at least one untyped node.
    pub untyped_pairs: u64,
    /// Per node-kind counts, keyed by the kind name (e.g. `"Identifier"`).
    pub by_kind: SortedMap<String, KindStats>,
}

impl UntypedStats {
    /// Total untyped-node occurrences across all kinds.
    pub fn total_occurrences(&self) -> u64 {
        self.by_kind.values().map(|k| k.occurrences).sum()
    }
}

/// Compiles every `(version, test)` pair and tallies the untyped nodes by kind.
/// Reuses the cached dataset (only a cold cache hits the network).
pub fn collect_untyped_stats() -> Result<UntypedStats> {
    let datasets = fetch_all_versions()?;

    let mut by_kind: SortedMap<String, KindStats> = SortedMap::new();
    let mut untyped_pairs = 0;

    for dataset in &datasets {
        let version = dataset.version();
        for relative_path in dataset.test_files()? {
            let path = dataset.root().join(&relative_path);
            let Outcome::Untyped { nodes } = run_test(&path, version) else {
                continue;
            };

            untyped_pairs += 1;

            // A node renders as `"<kind> <file>:<start>..<end>"`; the kind is the
            // leading token. Count occurrences, and the pair once per kind.
            let mut kinds_here = SortedSet::new();
            for node in nodes {
                let kind = node.split(' ').next().unwrap_or("<unknown>");
                by_kind.entry(kind.to_owned()).or_default().occurrences += 1;
                kinds_here.insert(kind.to_owned());
            }
            for kind in kinds_here {
                by_kind.entry(kind).or_default().pairs += 1;
            }
        }
    }

    Ok(UntypedStats {
        untyped_pairs,
        by_kind,
    })
}
