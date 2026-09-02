use std::fmt;

use serde::{Serialize, Serializer};

/// Identifies a single IR node within a compilation.
///
/// A node id is a pair: the `file` it belongs to (its position in the
/// compilation's sorted file list) and the node's `index` within that file.
/// Giving every file its own index space is what lets files be lowered to IR
/// independently — the id a node gets depends only on its file's position and
/// the order it is built in, never on which thread built it.
///
/// `file` is the more significant half, so the derived ordering is file-major
/// then index; [`Self::as_u64`] packs the pair into that same single number.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NodeId {
    // Field order is load-bearing: `file` first makes the derived `Ord`/`PartialOrd`
    // compare file-major, then index.
    file: u32,
    index: u32,
}

impl NodeId {
    /// Builds a `NodeId` from its `file` half (the file's sorted position) and
    /// its `index` half (the node's index within that file).
    pub const fn new(file: u32, index: u32) -> Self {
        Self { file, index }
    }

    /// The file this node belongs to (its position in the sorted file list).
    pub const fn file(self) -> u32 {
        self.file
    }

    /// This node's index within its file.
    pub const fn index(self) -> u32 {
        self.index
    }

    /// The id as a single number, with `file` in the high 32 bits — the same
    /// ordering as the derived comparison.
    pub const fn as_u64(self) -> u64 {
        ((self.file as u64) << u32::BITS) | self.index as u64
    }
}

impl From<NodeId> for u64 {
    fn from(value: NodeId) -> Self {
        value.as_u64()
    }
}

// Serialized as the single packed number (file in the high 32 bits), so the
// two-field representation stays an implementation detail — the AST JSON keeps
// emitting node ids as plain integers.
impl Serialize for NodeId {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u64(self.as_u64())
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.file, self.index)
    }
}
