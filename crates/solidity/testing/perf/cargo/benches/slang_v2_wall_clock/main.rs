//! Wall-clock benchmarks for the Slang v2 pipeline, driven by [`divan`].
//!
//! These complement the `slang_v2` suite, which measures the same pipeline with
//! `gungraun`/Valgrind. Valgrind's counters are deterministic, which makes them
//! ideal for catching regressions in CI, but they are blind to the thing
//! parallelism is meant to improve: moving a work item onto another thread
//! costs the same number of instructions, and Valgrind serializes threads
//! anyway. So the pipeline's move to multiple threads is tracked here, in
//! wall-clock time, instead.
//!
//! Wall time on shared CI runners is too noisy to alert on, so this suite is
//! not uploaded to the Bencher dashboard. It is meant for local before/after
//! comparisons:
//!
//! ```sh
//! ./scripts/bin/infra perf cargo-wall-clock
//! ```
//!
//! Anything after `--` is forwarded to `divan`, which accepts a substring
//! filter plus flags such as `--sample-count`:
//!
//! ```sh
//! ./scripts/bin/infra perf cargo-wall-clock -- uniswap --sample-count 50
//! ```

use divan::counter::{BytesCount, ItemsCount};
use divan::{Bencher, black_box};
use solidity_testing_perf_cargo::dataset::SolidityProject;
use solidity_testing_perf_cargo::tests;

mod __dependencies_used_in_lib__ {
    use anyhow as _;
    use gungraun as _;
    use inflector as _;
    use infra_utils as _;
    use paste as _;
    use semver as _;
    use serde as _;
    use serde_json as _;
    use slang_solidity as _;
    use slang_solidity_v2 as _;
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

fn main() {
    divan::main();
}

// Single source for the project list used by every benchmark below.
// Only 0.8.x-compatible projects belong here.
// Edit this constant (and only this constant) to add or remove a project.
// __SLANG_INFRA_PROJECT_LIST__ (keep in sync)
const PROJECTS: [&str; 9] = [
    "uniswap",
    "multicall3",
    "create_x",
    "ui_pool_data_provider_v3",
    "cooldogs",
    "one_step_leverage_f",
    "pointer_libraries",
    "merkle_proof",
    "ens_registrar_controller",
];

/// Time budget per (benchmark, project) pair, in seconds.
///
/// `divan` collects up to 100 samples by default, and this bounds how long it
/// may spend doing so. At 10s nothing in the suite is actually capped: the
/// slowest pair by a wide margin is `compilation_unit`/`uniswap`, which needs
/// roughly 7s for its full 100 samples, so every pair reports 100 and a whole
/// run takes ~13s. The budget is headroom for slower machines and larger
/// projects, not a limit being hit today — at 5s `uniswap` was the one pair it
/// truncated. Override with `--max-time` when a noisy result needs more samples.
const MAX_TIME_SECS: u64 = 10;

/// Builds a whole [`slang_solidity_v2::compilation::CompilationUnit`]: parsing,
/// IR building, and semantic analysis. This is the headline number, and the one
/// that consumers of the crate observe.
#[divan::bench(args = PROJECTS, max_time = MAX_TIME_SECS)]
fn compilation_unit(bencher: Bencher<'_, '_>, project_name: &str) {
    let project = tests::slang_v2::compilation_unit::setup(project_name);

    with_throughput_counters(bencher, project)
        .bench(|| black_box(tests::slang_v2::compilation_unit::run(black_box(project))));
}

/// Parses every source of the project into a CST, without any of the later
/// stages. Tracks the parser on its own, since it is the first stage to be
/// parallelized.
#[divan::bench(args = PROJECTS, max_time = MAX_TIME_SECS)]
fn parser(bencher: Bencher<'_, '_>, project_name: &str) {
    let project = tests::slang_v2::parser::setup(project_name);

    with_throughput_counters(bencher, project)
        .bench(|| black_box(tests::slang_v2::parser::run(black_box(project))));
}

/// How the pipeline scales with the number of threads it is given.
///
/// The benchmarks above run on `rayon`'s global pool, which uses every core.
/// These instead pin one project to pools of increasing size, so the speedup a
/// parallel stage actually delivers is visible — and so a stage that fails to
/// scale (or regresses at high thread counts through contention) shows up.
mod thread_scaling {
    use divan::{Bencher, black_box};

    use super::{MAX_TIME_SECS, with_throughput_counters};
    use crate::tests;

    /// The project to sweep. Deliberately one of the larger multi-file projects:
    /// single-file projects have no file-level parallelism to exploit.
    // __SLANG_INFRA_PROJECT_LIST__ (keep in sync)
    const PROJECT: &str = "uniswap";

    /// Thread counts to measure. `1` is the serial baseline every other number
    /// should be compared against.
    const THREAD_COUNTS: [usize; 5] = [1, 2, 4, 8, 16];

    /// The whole pipeline. Bounded by the share of it that is parallel, so read
    /// this together with `parser` below rather than on its own: a stage that
    /// scales perfectly still only speeds up the total by its own share.
    #[divan::bench(args = THREAD_COUNTS, max_time = MAX_TIME_SECS)]
    fn compilation_unit(bencher: Bencher<'_, '_>, threads: usize) {
        let project = tests::slang_v2::compilation_unit::setup(PROJECT);
        let pool = pool_with(threads);

        with_throughput_counters(bencher, project).bench(|| {
            pool.install(|| black_box(tests::slang_v2::compilation_unit::run(black_box(project))))
        });
    }

    /// The parse stage on its own, which is what isolates its scaling from the
    /// still-sequential stages that follow it.
    ///
    /// Note that the top-level `parser` benchmark measures the *sequential* loop
    /// instead, so it stays a fixed reference point; this one mirrors the
    /// builder's parallel parse phase.
    #[divan::bench(args = THREAD_COUNTS, max_time = MAX_TIME_SECS)]
    fn parser(bencher: Bencher<'_, '_>, threads: usize) {
        let project = tests::slang_v2::parser::setup(PROJECT);
        let pool = pool_with(threads);

        with_throughput_counters(bencher, project).bench(|| {
            pool.install(|| black_box(tests::slang_v2::parser::run_in_parallel(black_box(project))))
        });
    }

    fn pool_with(threads: usize) -> rayon::ThreadPool {
        rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build()
            .expect("thread pool builds")
    }
}

/// Reports bytes and files processed per second, so that results remain
/// comparable across projects of very different sizes.
fn with_throughput_counters<'a, 'b>(
    bencher: Bencher<'a, 'b>,
    project: &SolidityProject,
) -> Bencher<'a, 'b> {
    let total_bytes: usize = project.sources.values().map(String::len).sum();

    bencher
        .counter(BytesCount::new(total_bytes))
        .counter(ItemsCount::new(project.sources.len()))
}
