use crate::ir::nodes::{NodeId, NodeKind};

/// A per-kind tally of the nodes allocated by a [`NodeIdGenerator`].
///
/// Backed by a fixed-size array indexed by the [`NodeKind`] discriminant, so
/// recording a node is a single array increment. It is populated for free as
/// the IR is built and lets downstream consumers (e.g. the binder) pre-size
/// their per-node collections instead of growing and rehashing them.
#[derive(Clone, Debug)]
pub struct NodeKindHistogram {
    counts: [u32; NodeKind::COUNT],
}

impl NodeKindHistogram {
    fn record(&mut self, kind: NodeKind) {
        self.counts[kind as usize] += 1;
    }

    /// The number of nodes of the given `kind` allocated so far.
    pub fn count(&self, kind: NodeKind) -> u32 {
        self.counts[kind as usize]
    }

    /// The total number of nodes recorded across all kinds.
    pub fn total(&self) -> u32 {
        self.counts.iter().sum()
    }

    /// Adds the per-kind counts of `other` into this histogram.
    ///
    /// Used to fold the per-file histograms produced when each file is lowered
    /// with its own generator back into a single whole-compilation histogram.
    pub fn merge(&mut self, other: &NodeKindHistogram) {
        for (total, count) in self.counts.iter_mut().zip(other.counts.iter()) {
            *total += *count;
        }
    }
}

impl Default for NodeKindHistogram {
    fn default() -> Self {
        Self {
            counts: [0; NodeKind::COUNT],
        }
    }
}

/// Hands out the [`NodeId`]s for a single file, as an ascending run within that
/// file's own index space (see [`NodeId`]).
///
/// A generator is fixed to one `file` (the file's sorted position) and hands out
/// `(file, 1), (file, 2), …` — index `0` is left unused, matching the historical
/// choice to never hand out the all-zero id. Because a file's ids depend only on
/// its `file` half and the order it is built in, files can be lowered in any
/// order, or concurrently, without their ids changing. [`Default`] is the
/// generator for the first file, which is the right choice when lowering a
/// single file in isolation.
///
/// While allocating ids it also accumulates a [`NodeKindHistogram`] of the kinds
/// it has been asked to allocate (see [`Self::histogram`]).
pub struct NodeIdGenerator {
    file: u32,
    next_index: u32,
    histogram: NodeKindHistogram,
}

impl NodeIdGenerator {
    /// Creates the generator for the file at sorted position `file_index`.
    pub fn for_file(file_index: usize) -> Self {
        Self {
            file: u32::try_from(file_index).expect("file index fits in a u32"),
            // Index 0 is never handed out; the first node of every file is 1.
            next_index: 1,
            histogram: NodeKindHistogram::default(),
        }
    }

    /// Returns a `NodeId` greater than any previously returned by this
    /// generator and records a new `kind` in the histogram.
    /// The returned ID is unique and suitable for use as a total-order key.
    pub fn next_id_of(&mut self, kind: NodeKind) -> NodeId {
        self.histogram.record(kind);
        let index = self.next_index;
        debug_assert!(index != 0, "file exhausted its 2^32 node-id index space");
        self.next_index += 1;
        NodeId::new(self.file, index)
    }

    /// The per-kind histogram of the nodes allocated so far.
    pub fn histogram(&self) -> &NodeKindHistogram {
        &self.histogram
    }

    /// Consumes the generator, returning the histogram it accumulated.
    pub fn into_histogram(self) -> NodeKindHistogram {
        self.histogram
    }
}

impl Default for NodeIdGenerator {
    fn default() -> Self {
        Self::for_file(0)
    }
}
