//! Shared building blocks for snapshot kinds that need a compiled
//! `CompilationUnit` (eg. `binder_output`, `diagnostics_output`'s slang
//! target): splitting a multi-part `input.sol` into files, and building a
//! `CompilationUnit` from them.

use anyhow::Result;
use slang_solidity_v2::compilation::{
    CompilationBuilder, CompilationBuilderConfig, CompilationUnit, FileId,
};
use slang_solidity_v2_common::collections::SortedMap;
use slang_solidity_v2_common::diagnostics::kinds::compilation::{MissingFile, UnresolvedImport};
use slang_solidity_v2_common::evm_targets::EvmTarget;
use slang_solidity_v2_common::versions::LanguageVersion;

use crate::utils::multi_part_file::split_multi_file;
use crate::utils::path_resolver;

pub(crate) type FileSourceMap = SortedMap<FileId, String>;

/// Splits a (possibly multi-part) `input.sol` into its files. A sorted map
/// keeps file iteration order deterministic across runs.
pub(crate) fn files_of(source: &str) -> FileSourceMap {
    split_multi_file(source)
        .parts
        .iter()
        .map(|part| (part.name.into(), part.contents.to_string()))
        .collect()
}

/// Builds a `CompilationUnit` from the given files at a specific
/// version/target.
pub(crate) fn build_compilation_unit(
    files: &FileSourceMap,
    version: LanguageVersion,
    target: EvmTarget,
) -> CompilationUnit {
    let config = TestConfig {
        files: files.clone(),
    };
    let mut builder = CompilationBuilder::create(version, target, config);

    // `add_file()` recursively adds dependencies, so adding the root file would
    // be enough. To avoid depending on the ordering of the parts in
    // `input.sol`, and since re-adding a file is idempotent, we add them all.
    for file in files.keys() {
        builder.add_file(file.clone());
    }

    builder.build()
}

struct TestConfig {
    files: FileSourceMap,
}

impl CompilationBuilderConfig for TestConfig {
    fn read_file(&mut self, file_id: &FileId) -> Result<String, MissingFile> {
        self.files.get(file_id).cloned().ok_or_else(|| MissingFile {
            reason: "File not found".to_string(),
        })
    }

    fn resolve_import(
        &mut self,
        source_file_id: &FileId,
        import_path: &str,
    ) -> Result<FileId, UnresolvedImport> {
        path_resolver::resolve_import(source_file_id, import_path).ok_or_else(|| UnresolvedImport {
            reason: "Unresolved import".to_string(),
        })
    }
}
