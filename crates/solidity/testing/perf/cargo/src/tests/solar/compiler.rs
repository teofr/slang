use std::hint::black_box;
use std::path::PathBuf;

use solar::interface::{ColorChoice, Session};
use solar::sema::Compiler;

use crate::dataset::SolidityProject;

pub type Input = &'static SolidityProject;
pub type Output = usize;

pub fn setup(project: &str) -> Input {
    crate::tests::setup::setup(project)
}

/// Drives `solar` all the way through semantic analysis, so that it is
/// comparable to the v2 `compilation_unit` benchmark rather than to the parser
/// on its own: parsing, lowering the ASTs into HIR, and then the analysis
/// passes.
///
/// Returns the number of source files that were compiled, purely so that
/// callers have something cheap to assert on.
pub fn run(project: Input) -> Output {
    let sess = Session::builder()
        .with_buffer_emitter(ColorChoice::Never)
        .build();

    let mut compiler = Compiler::new(sess);

    compiler.enter_mut(|compiler| {
        let mut parsing_context = compiler.parse();

        // Import resolution is deliberately left on (solar's default), even
        // though every source is supplied up front and the dataset's paths are
        // project-relative rather than real files: solar resolves them against
        // the in-memory source map, and leaving it on keeps the work comparable
        // to `compilation_unit`, which resolves imports through its config
        // callback. Turning it off measures roughly a third less work.

        // Collected rather than passed lazily: the loader takes a parallel
        // iterator, which a plain `Map` does not implement.
        let files: Vec<(PathBuf, String)> = project
            .sources
            .iter()
            .map(|(name, contents)| (PathBuf::from(name), contents.clone()))
            .collect();

        parsing_context
            .par_load_files_with_contents(files)
            .expect("solar loads the project's sources");

        parsing_context.parse();

        let _ = compiler.lower_asts();
        let _ = compiler.analysis();

        black_box(compiler.gcx());
    });

    compiler
        .sess()
        .emitted_errors()
        .expect("solar reports its diagnostics")
        .expect("solar compiles the project without errors");

    project.sources.len()
}

pub fn test(project: Input) -> Output {
    run(project)
}
