use slang_solidity_v2_common::files::FileId;
use slang_solidity_v2_common::nodes::NodeId;

use super::SemanticFile;

/// Maps a [`NodeId`] back to the file it belongs to.
///
/// Every node id carries its file's position in its high half (see [`NodeId`]),
/// so this is a direct index into a `Vec<FileId>` keyed by that position — no
/// search. The files a `SemanticContext` is built from occupy contiguous
/// positions `0..n`, by construction of the IR trees.
pub(crate) struct FileNodeMapper {
    /// File IDs indexed by the `file` half of a node id.
    files_by_index: Vec<FileId>,
}

impl FileNodeMapper {
    pub(crate) fn build_from(files: &[impl SemanticFile]) -> Self {
        // Place each file at the slot its own nodes name (its root's `file`
        // half), so the result is correct regardless of the order `files` are
        // given in.
        let mut files_by_index: Vec<Option<FileId>> = Vec::with_capacity(files.len());
        for file in files {
            let index = file.ir_root().id().file() as usize;
            if index >= files_by_index.len() {
                files_by_index.resize(index + 1, None);
            }
            files_by_index[index] = Some(file.id().clone());
        }

        let files_by_index = files_by_index
            .into_iter()
            .map(|file_id| file_id.expect("file positions are contiguous from 0"))
            .collect();

        Self { files_by_index }
    }

    pub(crate) fn file_id_from_node_id(&self, node_id: NodeId) -> &FileId {
        &self.files_by_index[node_id.file() as usize]
    }
}
