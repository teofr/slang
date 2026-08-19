use gungraun::{
    Callgrind, Dhat, Direction, EntryPoint, FlamegraphConfig, LibraryBenchmarkConfig, OutputFormat,
};

/// Env var that, when set (to any value), makes
/// [`benchmark_config_with_num_callers`] skip the DHAT tool and run Callgrind
/// only.
///
/// DHAT is expensive (especially at high `num_callers`), so in certain
/// cases we want to skip it.
// __SLANG_PERF_SKIP_DHAT_ENV__ (keep in sync)
pub const SKIP_DHAT_ENV: &str = "SLANG_PERF_SKIP_DHAT";

/// Shared `LibraryBenchmarkConfig` used by every perf benchmark in this crate.
/// Centralised so the bench `main!` calls can't drift apart.
///
/// `num_callers` sets Valgrind's `--num-callers`: the maximum depth of the call
/// stack Valgrind records (and unwinds) at each allocation. Allocations are
/// attributed to a stack trace truncated to this depth, so a larger value
/// distinguishes allocation sites that share their top frames (e.g. a `malloc`
/// reached through several layers of generic/`Vec` code) and gives more
/// precise per-site attribution — at the cost of slower runs, since DHAT
/// unwinds that many frames on every allocation. Smaller values are coarser
/// but cheaper. Must be between 1 and 500 (DHAT's maximum).
pub fn benchmark_config_with_num_callers(num_callers: usize) -> LibraryBenchmarkConfig {
    build_config(num_callers, &[], false)
}

/// Like [`benchmark_config_with_num_callers`], but also attributing work done on
/// threads other than the one the benchmark function is called on.
///
/// `gungraun` already runs Callgrind with `--separate-threads=yes` and
/// `--fair-sched=try`, so every thread's metrics are collected into their own
/// file. Two things still have to be said explicitly:
///
/// - **A toggle per stage that runs off the main thread.** Collection is driven
///   by toggles, and the default one is the benchmark function — which only ever
///   runs on the calling thread, so a worker that never enters it reports
///   zeroes. Each entry in `thread_toggles` adds a `--toggle-collect` for a
///   frame that runs *inside* the workers, in Callgrind's glob syntax.
///
///   The benchmark function stays a toggle as well, so work that did *not* move
///   off the main thread is still counted. That matters as more of the pipeline
///   is parallelised: a benchmark that collected only the worker frames would
///   report the parallel stages and silently drop everything still sequential,
///   so its total would shrink as coverage grew. Adding a stage to the parallel
///   set means adding its frame here, not replacing what is already listed.
/// - **`show_intermediate`.** Without it only the total over all threads is
///   printed. The per-thread breakdown is the interesting part of a parallelism
///   benchmark — a total alone can't distinguish "spread evenly over four
///   threads" from "all of it still on one".
///
/// Note the total is what CI compares against its thresholds, and it is
/// *thread-count independent* by design: parallelising work should move
/// instructions between threads, not create more of them. A jump in the total
/// means the parallel version is doing more work, which is exactly the
/// regression a deterministic tool can catch and a wall-clock one cannot.
pub fn benchmark_config_with_thread_toggles(
    num_callers: usize,
    thread_toggles: &[&str],
) -> LibraryBenchmarkConfig {
    build_config(num_callers, thread_toggles, true)
}

fn build_config(
    num_callers: usize,
    thread_toggles: &[&str],
    show_intermediate: bool,
) -> LibraryBenchmarkConfig {
    assert!(
        0 < num_callers && num_callers <= 500,
        "num_callers must be between 1 and 500"
    );

    let mut config = LibraryBenchmarkConfig::default();

    // Only touched by the threaded variant, so the existing suites' output — and
    // therefore what the Bencher adapter parses for them — is left exactly as it
    // was.
    if show_intermediate {
        config.output_format(OutputFormat::default().show_intermediate(true));
    }

    config
        // 'valgrind' supports many tools. We run 'callgrind', which reports these metrics:
        // https://kcachegrind.github.io/html/Home.html
        //
        // Instructions:            Total CPU instructions executed.
        // LL Hits:                 Total (simulated) number of times the LL cache was hit.
        // L2 Hits:                 Total (simulated) number of times the L2 cache was hit.
        // RAM Hits:                Total (simulated) number of times the RAM was hit.
        // Total read+write:        Total memory reads/writes during the entire execution.
        // Estimated Cycles:        Number of CPU cycles (estimated) that went by during the entire execution.
        //
        // We also enable flame graphs into Cargo's 'target' directory.
        // They will be listed by 'infra perf' at the end of the run:
        .tool(
            Callgrind::with_args(
                thread_toggles
                    .iter()
                    .map(|toggle| format!("--toggle-collect={toggle}")),
            )
            .flamegraph(FlamegraphConfig::default().direction(Direction::BottomToTop))
            // Collection starts off, and each listed frame turns it on wherever
            // it runs. Keeping the default entry point instead does not work:
            // Callgrind toggles flip collection on entry *and off on exit*, so a
            // stage toggle reached from inside the (already collecting) benchmark
            // function switches collection off for exactly the stage being
            // measured. Measured on the sequential pipeline, that reported
            // 164,691,671 instructions where the stage alone is 192,763,352 —
            // the parse silently subtracted rather than added.
            .entry_point(if thread_toggles.is_empty() {
                EntryPoint::Default
            } else {
                EntryPoint::None
            }),
        )
        // 'valgrind' executes tests without any environment variables set by default.
        // Let's disable this behavior to be able to execute our infra utilities:
        .env_clear(false);

    // The 'DHAT' tool is much slower than Callgrind, so it's skipped on PR
    // benchmarks for the slower suites (see `SKIP_DHAT_ENV`). When enabled, it
    // reports these metrics: https://valgrind.org/docs/manual/dh-manual.html
    //
    // Total bytes:             How many bytes were allocated over the entire execution.
    // Total blocks:            How many heap blocks were allocated over the entire execution.
    // At t-gmax bytes:         How many bytes were alive when the heap size reached its global maximum (as measured in bytes).
    // At t-gmax blocks:        How many heap blocks were alive when the heap size reached its global maximum (as measured in bytes).
    // At t-end bytes:          How many bytes were alive at the end of execution (were not explicitly freed).
    // At t-end blocks:         How many heap blocks were alive at the end of execution (were not explicitly freed).
    // Reads bytes:             How many bytes within heap blocks were read during the entire execution.
    // Writes bytes:            How many bytes within heap blocks were written during the entire execution.
    if std::env::var_os(SKIP_DHAT_ENV).is_none() {
        // We set the DHAT arguments to whatever the user provided.
        let dhat_args = [format!("--num-callers={num_callers}")];
        config.tool(Dhat::with_args(dhat_args));
    }

    config
}
