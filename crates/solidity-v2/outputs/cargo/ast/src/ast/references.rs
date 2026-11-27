use slang_solidity_v2_common::built_ins::BuiltIn;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;
use slang_solidity_v2_semantic::context::SemanticContext;

use super::{Definition, Identifier, create_identifier};

#[derive(Clone)]
pub struct Reference<'a> {
    identifier: Identifier<'a>,
}

impl<'a> Reference<'a> {
    pub fn try_create(ir_node: &'a ir::Identifier, semantic: &'a SemanticContext) -> Option<Self> {
        // ensure the terminal node is actually functioning as a reference
        semantic
            .binder()
            .find_reference_by_identifier_node_id(ir_node.id())?;

        Some(Self {
            identifier: create_identifier(ir_node, semantic),
        })
    }

    pub fn name(&self) -> &str {
        self.identifier.name()
    }

    pub fn resolve_to_definition(&self) -> Option<Definition<'a>> {
        self.identifier.resolve_to_definition()
    }

    pub fn resolve_to_immediate_definition(&self) -> Option<Definition<'a>> {
        self.identifier.resolve_to_immediate_definition()
    }

    pub fn resolve_to_built_in(&self) -> Option<BuiltIn> {
        self.identifier.resolve_to_built_in()
    }
}

pub(crate) fn references_binding_to_definition<'a>(
    node_id: NodeId,
    semantic: &'a SemanticContext,
) -> Vec<Reference<'a>> {
    semantic
        .binder()
        .get_references_by_definition_id(node_id)
        .iter()
        .filter_map(|node_id| {
            semantic
                .binder()
                .find_reference_by_identifier_node_id(*node_id)
                .and_then(|reference| Reference::try_create(&reference.identifier, semantic))
        })
        .collect()
}
