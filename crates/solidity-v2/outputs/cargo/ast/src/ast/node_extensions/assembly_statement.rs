use super::super::{AssemblyStatementStruct, Definition};

impl<'a> AssemblyStatementStruct<'a> {
    /// Returns the Solidity definitions referenced from within this assembly
    /// block (eg. state variables, local variables or functions accessed by the
    /// Yul code).
    pub fn referenced_definitions(&self) -> Vec<Definition<'a>> {
        self.semantic
            .binder()
            .assembly_block_referenced_definitions(self.ir_node.id())
            .iter()
            .map(|definition_id| {
                Definition::try_create(*definition_id, self.semantic)
                    .expect("assembly referenced node is a definition")
            })
            .collect()
    }
}
