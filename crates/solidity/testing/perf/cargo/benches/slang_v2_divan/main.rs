//! Wall-clock + heap-allocation benchmark of the Slang v2 (LALRPOP) parser,
//! using [`divan`]. Complements the instruction-counting `slang_v2`
//! iai-callgrind bench with real time and heap-allocation measurements, which is
//! what you want when comparing LALRPOP code-generation backends (e.g.
//! table-driven vs. recursive-ascent vs. tail-call) or optimization changes that
//! barely move the instruction count but do move cache/branch behaviour.
//!
//! Run with:
//!     cargo bench -p solidity_testing_perf_cargo --bench slang_v2_divan
//!
//! The [`divan::AllocProfiler`] global allocator makes each row also report
//! allocation counts and bytes (including peak live "max alloc"), so parser
//! memory use is directly comparable across backends.

use std::hint::black_box;

use divan::{AllocProfiler, Bencher};
use solidity_testing_perf_cargo::dataset::load_projects;
use solidity_testing_perf_cargo::tests;

// Crates that are dependencies of the surrounding library but not used directly
// by this bench binary; referenced here to satisfy `unused_crate_dependencies`.
mod __dependencies_used_in_lib__ {
    use anyhow as _;
    use iai_callgrind as _;
    use inflector as _;
    use infra_utils as _;
    use paste as _;
    use semver as _;
    use serde as _;
    use serde_json as _;
    use slang_solidity as _;
    use slang_solidity_v2_ast as _;
    use slang_solidity_v2_common as _;
    use slang_solidity_v2_cst as _;
    use slang_solidity_v2_ir as _;
    use slang_solidity_v2_parser as _;
    use slang_solidity_v2_semantic as _;
    use solar as _;
    use solidity_testing_utils as _;
    use streaming_iterator as _;
    use tree_sitter as _;
    use tree_sitter_solidity as _;
}

// Only 0.8.x-compatible projects can be parsed by v2.
// __SLANG_INFRA_PROJECT_LIST__ (keep in sync)
const PROJECTS: &[&str] = &[
    "uniswap",
    "multicall3",
    "create_x",
    "ui_pool_data_provider_v3",
    "cooldogs",
    "one_step_leverage_f",
    "pointer_libraries",
    "merkle_proof",
];

/// Parse every source file in the project with the v2 parser (matches
/// `tests::slang_v2::parser::run`).
#[divan::bench(args = PROJECTS.iter().copied())]
fn parse(bencher: Bencher<'_, '_>, name: &str) {
    let project = &load_projects()[name];
    let total_bytes: usize = project.sources.values().map(String::len).sum();
    bencher
        .counter(divan::counter::BytesCount::new(total_bytes))
        .bench(|| black_box(tests::slang_v2::parser::run(black_box(project))));
}

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

fn main() {
    divan::main();
}
