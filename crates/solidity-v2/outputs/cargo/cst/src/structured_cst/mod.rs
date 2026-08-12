pub mod text_range;

#[path = "nodes.generated.rs"]
pub mod nodes;

pub use text_range::*;

/// CST nodes are boxed with [`Box`], not reference-counted: the tree is built by
/// one thread, has a single owner throughout, and is dropped once IR lowering is
/// done, so there is nothing for a refcount to track. `Box` is what makes that
/// ownership explicit while still keeping node sizes down and breaking the
/// recursive types.
///
/// A parsed source unit does have to be *moved* to another thread, since the
/// compilation pipeline parses files in parallel — which is why `Rc` will not do
/// here, however single-threaded the construction is. Assert the bound that
/// requires, so reintroducing `Rc` (or adding a non-`Sync` field to a node) fails
/// to compile at the definition rather than at some use site.
const _: () = {
    const fn assert_send_and_sync<T: Send + Sync>() {}

    assert_send_and_sync::<nodes::SourceUnit>();
};
