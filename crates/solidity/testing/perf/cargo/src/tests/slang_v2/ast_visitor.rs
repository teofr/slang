use slang_solidity_v2::ast::Identifier;
use slang_solidity_v2::ast::visitor::{Visitor, accept_source_unit};
use slang_solidity_v2::compilation::{CompilationUnit, File};

pub type Input = CompilationUnit;

#[allow(unused)]
pub struct Output {
    pub(crate) unit: CompilationUnit,
    pub(crate) identifier_count: usize,
}

pub fn setup(project: &str) -> Input {
    super::full_compilation::test(super::full_compilation::setup(project))
}

// Kept out of line: if inlined into the gungraun wrapper, Callgrind mistakes returns into
// the wrapper for fresh entries, and toggles collection off mid-run.
#[inline(never)]
pub fn run(unit: Input) -> Output {
    test(unit)
}

pub fn test(unit: Input) -> Output {
    let mut identifier_count = 0;

    for file in unit.files() {
        identifier_count += walk_file(&file);
    }

    Output {
        unit,
        identifier_count,
    }
}

/// Walks the AST of a single file, returning the number of identifiers in it.
pub fn walk_file(file: &File<'_>) -> usize {
    let mut walk = NodeWalk::default();
    accept_source_unit(&file.ast(), &mut walk);
    walk.identifier_count
}

#[derive(Default)]
struct NodeWalk {
    identifier_count: usize,
}

impl Visitor<'_> for NodeWalk {
    fn visit_identifier(&mut self, _node: &Identifier<'_>) {
        self.identifier_count += 1;
    }
}

pub fn count_identifiers(output: &Output) -> usize {
    output.identifier_count
}

/// How many threads [`run_threaded`] spreads the files over.
pub const THREADS: usize = 4;

/// Same walk as [`run`], but spread over [`THREADS`] threads sharing the unit: thread `n` walks
/// every [`THREADS`]-th file, starting at the `n`-th one. The split is static, so that the
/// instructions each thread executes do not depend on scheduling.
#[inline(never)]
pub fn run_threaded(unit: Input) -> Output {
    let files: Vec<File<'_>> = unit.files().collect();

    let identifier_count = std::thread::scope(|scope| {
        let handles: Vec<_> = (0..THREADS)
            .map(|thread| {
                let files = &files;
                scope.spawn(move || walk_thread_chunk(files, thread))
            })
            .collect();

        handles
            .into_iter()
            .map(|handle| handle.join().expect("thread panicked"))
            .sum()
    });

    drop(files);

    Output {
        unit,
        identifier_count,
    }
}

/// The work of a single thread of [`run_threaded`]. Kept out of line, and under a name of its
/// own: Callgrind collects each thread separately, toggled by this function.
// __SLANG_PERF_AST_VISITOR_THREAD_TOGGLE__ (keep in sync)
#[inline(never)]
pub fn walk_thread_chunk(files: &[File<'_>], thread: usize) -> usize {
    files
        .iter()
        .skip(thread)
        .step_by(THREADS)
        .map(walk_file)
        .sum()
}
