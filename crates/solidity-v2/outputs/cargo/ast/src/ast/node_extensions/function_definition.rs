use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_semantic::binder;
use slang_solidity_v2_semantic::context::SemanticContext;
use slang_solidity_v2_semantic::types::TypeId;

use super::super::{FunctionDefinitionStruct, Type};

impl FunctionDefinitionStruct {
    /// Returns the type this function is dispatched through — external
    /// visibility, with `calldata` locations changed to `memory` — or `None`
    /// when nothing selects on it: an internal or private function, a modifier,
    /// or a constructor, fallback or receive.
    pub fn externalized_type(&self) -> Option<Type> {
        let type_id = externalized_type_id_of_definition(&self.semantic, self.ir_node.id())?;
        Some(Type::create(type_id, &self.semantic))
    }
}

/// The externalized type of the function declared by `definition_id`, when it
/// is part of the external interface. See
/// [`FunctionDefinitionStruct::externalized_type`].
pub(crate) fn externalized_type_id_of_definition(
    semantic: &SemanticContext,
    definition_id: NodeId,
) -> Option<TypeId> {
    let binder::Definition::Function(definition) =
        semantic.binder().find_definition_by_id(definition_id)?
    else {
        return None;
    };
    if !definition.ir_node.is_part_of_external_interface() {
        return None;
    }
    let type_id = semantic.binder().node_typing(definition_id).as_type_id()?;
    semantic.types().externalized_function_type_id(type_id)
}
