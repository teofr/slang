use num_bigint::BigInt;
use num_rational::BigRational;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_semantic::binder;
use slang_solidity_v2_semantic::context::SemanticContext;
use slang_solidity_v2_semantic::types::{Number, TypeId};

/// Returns the literal number carried by a node's type, when the type is a
/// number-shaped literal kind (integer, hex-integer, or rational).
pub(crate) fn number_value_of_node(semantic: &SemanticContext, node_id: NodeId) -> Option<Number> {
    let type_id = semantic.binder().node_typing(node_id).as_type_id()?;
    semantic.types().number_value_of_type_id(type_id)
}

pub(crate) fn integer_value_of_node(semantic: &SemanticContext, node_id: NodeId) -> Option<BigInt> {
    match number_value_of_node(semantic, node_id)? {
        Number::Integer(value) => Some(value),
        Number::Rational(_) => None,
    }
}

pub(crate) fn rational_value_of_node(
    semantic: &SemanticContext,
    node_id: NodeId,
) -> Option<BigRational> {
    match number_value_of_node(semantic, node_id)? {
        Number::Integer(value) => Some(BigRational::from(value)),
        Number::Rational(value) => Some(value),
    }
}

/// The externalized type of the function declared by `definition_id`, when it
/// is part of the external interface. See
/// [`FunctionDefinitionStruct::externalized_type`](crate::ast::FunctionDefinitionStruct::externalized_type).
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
