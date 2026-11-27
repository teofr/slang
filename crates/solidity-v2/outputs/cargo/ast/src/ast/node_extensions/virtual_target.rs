use slang_solidity_v2_common::nodes::NodeId;

use super::super::{FunctionDefinition, StateVariableDefinition};

/// What a bare-name reference to a function runs in a contract: a function, or
/// the getter of a public state variable that overrides it.
pub enum VirtualTarget<'a> {
    Function(FunctionDefinition<'a>),
    Getter(StateVariableDefinition<'a>),
}

impl<'a> VirtualTarget<'a> {
    pub fn node_id(&self) -> NodeId {
        match self {
            Self::Function(function) => function.node_id(),
            Self::Getter(state_variable) => state_variable.node_id(),
        }
    }
}
