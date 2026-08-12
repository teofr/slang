use std::ops::Range;

use rayon::iter::{IntoParallelIterator, ParallelIterator};
use slang_solidity_v2_common::collections::{SortedMap, SortedSet};
use slang_solidity_v2_common::diagnostics::DiagnosticCollection;
use slang_solidity_v2_common::diagnostics::kinds::compilation::{
    MissingImportedFile, UnresolvedImport,
};
use slang_solidity_v2_common::evm_targets::EvmTarget;
use slang_solidity_v2_common::files::FileId;
use slang_solidity_v2_common::utils::strip_string_literal_quotes;
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_cst::structured_cst::nodes as cst;
use slang_solidity_v2_ir::ir::{self, BuildOutput};
use slang_solidity_v2_parser::{ParseOutput, Parser};
use slang_solidity_v2_semantic::context::{
    SemanticContext, SemanticFile, extract_import_paths_from_source_unit,
};

use super::file::InternalFile;
use super::unit::CompilationUnit;

/// User-provided callbacks necessary for the `CompilationBuilder` to perform its job.
///
/// The builder resolves the imports of several files at once, on different
/// threads, hence the [`Sync`] bound: a single shared `&Self` has to serve every
/// [`resolve_import()`](CompilationBuilderConfig::resolve_import) call. An
/// implementation that needs interior mutability (a cache, say) is free to use
/// whatever synchronization it prefers.
pub trait CompilationBuilderConfig: Sync {
    /// Callback used by this builder to resolve an import path.
    /// For example, if a source file contains the following statement:
    ///
    /// ```solidity
    /// import {Foo} from "foo.sol";
    /// ```
    ///
    /// Then the API will invoke the callback with the value `foo.sol` (the
    /// contents of the string literal, with the surrounding quotes stripped).
    ///
    /// The user is responsible for resolving it to a file in the compilation,
    /// and returning its ID. The returned [`UnresolvedImport`] is surfaced as a
    /// compilation diagnostic on the [`CompilationUnit`].
    ///
    /// Resolving to a file that was never added to the builder yields a
    /// [`MissingImportedFile`] diagnostic instead.
    fn resolve_import(
        &self,
        source_file_id: &FileId,
        import_path: &str,
    ) -> Result<FileId, UnresolvedImport>;
}

/// A builder for creating compilation units.
///
/// Collects the source files that make up a compilation, then turns them into a
/// [`CompilationUnit`] in [`build()`](CompilationBuilder::build). Adding files
/// only records them; all of the work (parsing, IR building, semantic analysis)
/// happens in `build()`, and every problem it runs into is reported as a
/// diagnostic on the resulting unit.
///
/// Parts of `build()` run in parallel on [`rayon`]'s global thread pool. To
/// bound how many threads that is, either configure the global pool once with
/// [`rayon::ThreadPoolBuilder::build_global`], or call `build()` inside
/// [`rayon::ThreadPool::install`] to use a pool of your own. Results do not
/// depend on the number of threads, or on the order in which they finish.
pub struct CompilationBuilder<C: CompilationBuilderConfig> {
    language_version: LanguageVersion,
    evm_target: EvmTarget,
    config: C,

    sources: SortedMap<FileId, String>,
}

/// One source file, parsed, with the import paths it contains resolved.
struct ParsedFile {
    file_id: FileId,
    contents: String,
    source_unit: cst::SourceUnit,
    /// Maps every import path in the file to the file it resolves to. Import
    /// paths the config could not resolve are absent.
    resolved_imports: SortedMap<String, FileId>,
}

impl<C: CompilationBuilderConfig> CompilationBuilder<C> {
    /// Creates a new compilation builder for the specified language version,
    /// EVM target, and resolver callbacks.
    pub fn create(
        language_version: LanguageVersion,
        evm_target: EvmTarget,
        config: C,
    ) -> CompilationBuilder<C> {
        CompilationBuilder {
            language_version,
            evm_target,
            config,

            sources: SortedMap::default(),
        }
    }

    /// Adds a source file, and its contents, to the compilation unit.
    ///
    /// The user is responsible for providing every file that takes part in the
    /// compilation, including the transitive imports of the files they care
    /// about. An import that resolves to a file which was not added is reported
    /// as a [`MissingImportedFile`] diagnostic on the resulting unit.
    ///
    /// Adding a file that has already been added replaces its contents.
    ///
    /// That matches `solc`: given a Standard JSON `sources` object containing
    /// the same key twice, it silently keeps the last entry and reports the
    /// file once. Note this is the duplicate-key policy of the JSON parser it
    /// uses rather than a documented rule of the format (verified against
    /// 0.8.36), so treat it as observed behaviour we agree with, not a
    /// guarantee `solc` makes.
    pub fn add_file(&mut self, file_id: FileId, contents: String) {
        self.sources.insert(file_id, contents);
    }

    /// Adds several source files at once. Equivalent to calling
    /// [`add_file()`](CompilationBuilder::add_file) on each of them.
    pub fn add_files(&mut self, files: impl IntoIterator<Item = (FileId, String)>) {
        self.sources.extend(files);
    }

    /// Consumes the source files added so far, and returns the final
    /// compilation unit.
    ///
    /// Parse errors, unresolvable imports, and missing imported files are all
    /// collected as diagnostics on the returned [`CompilationUnit`] — see
    /// [`CompilationUnit::diagnostics`].
    pub fn build(self) -> CompilationUnit {
        let CompilationBuilder {
            language_version,
            evm_target,
            config,

            sources,
        } = self;

        let mut diagnostics = DiagnosticCollection::default();

        let parsed_files = parse_files(&config, sources, language_version, &mut diagnostics);
        let (files, id_generator) = build_ir(parsed_files, language_version, &mut diagnostics);

        let semantic = SemanticContext::build_from(
            language_version,
            evm_target,
            &files,
            Some(id_generator.histogram()),
            &mut diagnostics,
        );

        CompilationUnit::create(language_version, evm_target, files, semantic, diagnostics)
    }
}

/// Parses every source file, and resolves the import paths each one contains.
///
/// Files are parsed in parallel: no file's parse observes another's, and the
/// only thing they share is the read-only config and file set. This is why the
/// full source list is collected up front — with recursive discovery, a file
/// could not be parsed until its importer's imports had been resolved.
///
/// The result does not depend on how the work was scheduled. `collect()` on an
/// indexed parallel iterator fills the output in input order, so the returned
/// files stay sorted by id no matter which thread finishes first, and each
/// file's diagnostics are merged in that same order afterwards.
fn parse_files<C: CompilationBuilderConfig>(
    config: &C,
    sources: SortedMap<FileId, String>,
    language_version: LanguageVersion,
    diagnostics: &mut DiagnosticCollection,
) -> Vec<ParsedFile> {
    // Cloning `FileId`s is a reference-count bump, so keeping the key set
    // around lets us consume `sources` without copying any file contents.
    let known_files: SortedSet<FileId> = sources.keys().cloned().collect();

    let parsed: Vec<(ParsedFile, DiagnosticCollection)> = sources
        .into_iter()
        .collect::<Vec<_>>()
        .into_par_iter()
        .map(|(file_id, contents)| {
            parse_file(config, &known_files, language_version, file_id, contents)
        })
        .collect();

    let mut parsed_files = Vec::with_capacity(parsed.len());
    for (parsed_file, file_diagnostics) in parsed {
        diagnostics.extend(file_diagnostics);
        parsed_files.push(parsed_file);
    }

    parsed_files
}

/// Parses a single source file, and resolves the import paths it contains.
///
/// Returns its diagnostics separately, rather than pushing them into a shared
/// collection, so that this can run on its own thread.
fn parse_file<C: CompilationBuilderConfig>(
    config: &C,
    known_files: &SortedSet<FileId>,
    language_version: LanguageVersion,
    file_id: FileId,
    contents: String,
) -> (ParsedFile, DiagnosticCollection) {
    let ParseOutput {
        source_unit,
        mut diagnostics,
    } = Parser::parse(&file_id, &contents, language_version);

    let mut resolved_imports = SortedMap::new();
    for (import_path, path_range) in extract_import_paths_from_cst(&source_unit, &contents) {
        let imported_file_id = match config.resolve_import(&file_id, &import_path) {
            Ok(imported_file_id) => imported_file_id,
            Err(unresolved_import) => {
                diagnostics.push(file_id.clone(), path_range, unresolved_import);
                continue;
            }
        };

        if !known_files.contains(&imported_file_id) {
            diagnostics.push(
                file_id.clone(),
                path_range,
                MissingImportedFile {
                    imported_file_id: imported_file_id.clone(),
                },
            );
        }

        // Recorded even when the file is missing: the diagnostic above is what
        // reports the problem, and the later stages are able to see that the
        // target is not part of the compilation.
        resolved_imports.insert(import_path, imported_file_id);
    }

    let parsed_file = ParsedFile {
        file_id,
        contents,
        source_unit,
        resolved_imports,
    };

    (parsed_file, diagnostics)
}

/// Lowers every parsed file into its IR representation, attaching the resolved
/// imports to the IR nodes that declare them.
fn build_ir(
    parsed_files: Vec<ParsedFile>,
    language_version: LanguageVersion,
    diagnostics: &mut DiagnosticCollection,
) -> (Vec<InternalFile>, ir::NodeIdGenerator) {
    let mut id_generator = ir::NodeIdGenerator::default();

    let files = parsed_files
        .into_iter()
        .map(|parsed_file| {
            let ParsedFile {
                file_id,
                contents,
                source_unit,
                resolved_imports,
            } = parsed_file;

            let BuildOutput {
                ir_root,
                diagnostics: ir_diagnostics,
            } = ir::build(
                &file_id,
                &source_unit,
                &contents,
                language_version,
                &mut id_generator,
            );
            diagnostics.extend(ir_diagnostics);

            let mut file = InternalFile::new(file_id, ir_root);
            for (node_id, import_path) in extract_import_paths_from_source_unit(file.ir_root()) {
                if let Some(target_file_id) = resolved_imports.get(&import_path) {
                    file.add_resolved_import(node_id, target_file_id.clone());
                }
            }
            file
        })
        .collect();

    (files, id_generator)
}

fn extract_import_paths_from_cst(
    source_unit: &cst::SourceUnit,
    contents: &str,
) -> Vec<(String, Range<usize>)> {
    let mut import_paths = Vec::new();

    for member in &source_unit.members.elements {
        let cst::SourceUnitMember::ImportDirective(import_directive) = member else {
            continue;
        };
        let range = match &import_directive.clause {
            cst::ImportClause::PathImport(path_import) => &path_import.path.range,
            cst::ImportClause::NamedImport(named_import) => &named_import.path.range,
            cst::ImportClause::ImportDeconstruction(import_deconstruction) => {
                &import_deconstruction.path.range
            }
        };
        let literal = &contents[range.clone()];
        import_paths.push((
            strip_string_literal_quotes(literal).to_owned(),
            range.clone(),
        ));
    }
    import_paths
}
