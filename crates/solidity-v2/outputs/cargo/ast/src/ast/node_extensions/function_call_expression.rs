use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;
use slang_solidity_v2_semantic::binder::Typing;

use super::super::FunctionCallExpressionStruct;

impl FunctionCallExpressionStruct {
    /// Returns `true` if this call is a type conversion (e.g. `uint256(x)`,
    /// `address(y)`) rather than a function call.
    pub fn is_type_conversion(&self) -> bool {
        match &self.ir_node.operand {
            ir::Expression::ElementaryType(_) | ir::Expression::PayableKeyword(_) => true,
            ir::Expression::Identifier(terminal) => self.node_is_meta_type(terminal.id()),
            ir::Expression::MemberAccessExpression(mae) => self.node_is_meta_type(mae.id()),
            _ => false,
        }
    }

    /// Returns `true` if the typing recorded for `node_id` is a meta-type (ie.
    /// it refers to a type rather than a value). The predicate itself lives on
    /// `Type::is_meta_type`; this just maps the node to its resolved type.
    fn node_is_meta_type(&self, node_id: NodeId) -> bool {
        let Typing::Resolved(type_id) = self.semantic.binder().node_typing(node_id) else {
            return false;
        };
        self.semantic.types().get_type_by_id(type_id).is_meta_type()
    }
}
