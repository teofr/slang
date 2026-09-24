use super::super::{FunctionDefinitionStruct, Type};
use super::common::externalized_type_id_of_definition;

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
