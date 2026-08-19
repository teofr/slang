#![allow(clippy::exit)]

use std::hint::black_box;

use gungraun::{library_benchmark, library_benchmark_group, main};
use solidity_testing_perf_cargo::config::{
    benchmark_config_with_num_callers, benchmark_config_with_thread_toggles,
};
use solidity_testing_perf_cargo::tests;
// Local aliases for the setup functions, so the generated benchmark ID reads
// `parser_setup("uniswap")` instead of `tests :: slang_v2 :: parser :: setup("uniswap")`.
use tests::slang_v2::ast_analysis::setup as ast_analysis_setup;
use tests::slang_v2::ast_visitor::setup as ast_visitor_setup;
use tests::slang_v2::compilation_unit::setup as compilation_unit_setup;
use tests::slang_v2::compute_contracts_abi::setup as compute_contracts_abi_setup;
use tests::slang_v2::ir_builder::setup as ir_builder_setup;
use tests::slang_v2::parser::setup as parser_setup;
use tests::slang_v2::semantic::setup as semantic_setup;

mod __dependencies_used_in_lib__ {
    use anyhow as _;
    use divan as _;
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

/*
 * WARNING:
 * The reported `gungraun` benchmark ID is constructed from
 * `{file_name}::{group_name}::{function_name} <project>:(<arguments>)`.
 * Changing any of the above would change the resulting benchmark ID, and
 * disconnect it from previous results.
 *
 * __SLANG_V2_INFRA_BENCHMARKS_LIST__ (keep in sync)
 */

// Single source for the project list used by every stage benchmark below.
// Only 0.8.x-compatible projects belong here.
// Edit this macro (and only this macro) to add or remove a project.
// __SLANG_INFRA_PROJECT_LIST__ (keep in sync)
macro_rules! bench_projects {
    (
        #[$lb:meta]
        $($rest:tt)*
    ) => {
        #[$lb]
        #[bench::uniswap("uniswap")]
        #[bench::multicall3("multicall3")]
        #[bench::create_x("create_x")]
        #[bench::ui_pool_data_provider_v3("ui_pool_data_provider_v3")]
        #[bench::cooldogs("cooldogs")]
        #[bench::one_step_leverage_f("one_step_leverage_f")]
        #[bench::pointer_libraries("pointer_libraries")]
        #[bench::merkle_proof("merkle_proof")]
        #[bench::ens_registrar_controller("ens_registrar_controller")]
        $($rest)*
    };
}

bench_projects! {
    #[library_benchmark(setup = parser_setup)]
    fn parser(input: tests::slang_v2::parser::Input) -> tests::slang_v2::parser::Output {
        black_box(tests::slang_v2::parser::run(black_box(input)))
    }
}

// Note: the input CST source units are consumed (dropped) during IR building.
// This is the intended use case: the CST is replaced by the IR representation.
bench_projects! {
    #[library_benchmark(setup = ir_builder_setup)]
    fn ir_builder(
        input: tests::slang_v2::ir_builder::Input,
    ) -> tests::slang_v2::ir_builder::Output {
        black_box(tests::slang_v2::ir_builder::run(
            black_box(input),
        ))
    }
}

bench_projects! {
    #[library_benchmark(setup = semantic_setup)]
    fn semantic(
        input: tests::slang_v2::semantic::Input,
    ) -> tests::slang_v2::semantic::Output {
        black_box(tests::slang_v2::semantic::run(
            black_box(input),
        ))
    }
}

bench_projects! {
    #[library_benchmark(setup = compute_contracts_abi_setup)]
    fn compute_contracts_abi(
        input: tests::slang_v2::compute_contracts_abi::Input,
    ) -> tests::slang_v2::compute_contracts_abi::Output {
        black_box(tests::slang_v2::compute_contracts_abi::run(
            black_box(input),
        ))
    }
}

bench_projects! {
    #[library_benchmark(setup = ast_visitor_setup)]
    fn ast_visitor(
        input: tests::slang_v2::ast_visitor::Input,
    ) -> tests::slang_v2::ast_visitor::Output {
        black_box(tests::slang_v2::ast_visitor::run(
            black_box(input),
        ))
    }
}

bench_projects! {
    #[library_benchmark(setup = ast_analysis_setup)]
    fn ast_analysis(
        input: tests::slang_v2::ast_analysis::Input,
    ) -> tests::slang_v2::ast_analysis::Output {
        black_box(tests::slang_v2::ast_analysis::run(
            black_box(input),
        ))
    }
}

library_benchmark_group!(
    name = pipeline;
    // __SLANG_V2_INFRA_BENCHMARKS_LIST__ (keep in sync)
    benchmarks = parser, ir_builder, semantic, compute_contracts_abi, ast_visitor, ast_analysis,
);

/// Frames that the pipeline runs on threads of its own, in Callgrind's
/// `--toggle-collect` glob syntax.
///
/// `Parser::parse` is the unit of work the pipeline hands to a thread, so this is
/// the frame that has to be collected for a worker's metrics to be non-zero. It
/// is also stable across the change that makes parsing parallel: the same frame
/// runs on the main thread while the parse is sequential, and on each worker once
/// it is not, so the benchmark ID keeps its history across that change.
///
/// The shape of the name matters and is easy to get wrong. Callgrind matches
/// against its own demangled symbols, where an inherent method on a type reads
/// `<Type>::method` with the type's full path inside the brackets:
///
/// ```text
/// <slang_solidity_v2_parser::parser::Parser>::parse
/// ```
///
/// so the leading `*` is what absorbs the `<`, and the `parser::parser` module
/// segment is required. A pattern that matches nothing is not an error — the
/// benchmark simply reports zeros, and the flamegraph step fails with
/// `No stack counts found`, which is the symptom to look for if this ever stops
/// matching (e.g. if the type moves module).
/// Add an entry when another stage moves off the calling thread — IR building,
/// say — rather than replacing what is here: the benchmark function remains a
/// toggle too, so whatever is still sequential keeps being counted, and the
/// total stays comparable as coverage grows.
const THREAD_TOGGLES: &[&str] = &["*slang_solidity_v2_parser::parser::Parser>::parse"];

// Parses the whole project through the public compilation API, measuring only
// the parse stage but attributing it per thread.
//
// The sibling `parser` benchmark above measures the same stage by looping over
// the sources directly. This one goes through `CompilationBuilder`, which is
// where the pipeline decides *how* to schedule that work — so it is the one
// that will show the parse spread across threads once it is parallelised,
// while `parser` keeps measuring the serial cost of parsing one file after
// another.
//
// Both matter: `parser` answers "did parsing get slower", this one answers "did
// distributing the parse change the total amount of work, and is that work
// actually spread out".
bench_projects! {
    #[library_benchmark(
        config = benchmark_config_with_thread_toggles(500, THREAD_TOGGLES),
        setup = compilation_unit_setup
    )]
    fn threaded_parse(
        input: tests::slang_v2::compilation_unit::Input,
    ) -> tests::slang_v2::compilation_unit::Output {
        black_box(tests::slang_v2::compilation_unit::run(black_box(input)))
    }
}

library_benchmark_group!(
    name = threads;
    benchmarks = threaded_parse,
);

main!(
    // We use the maximum possible value of `num-callers` to ensure DHAT values
    // are sensible
    config = benchmark_config_with_num_callers(500);
    library_benchmark_groups = pipeline, threads,
);
