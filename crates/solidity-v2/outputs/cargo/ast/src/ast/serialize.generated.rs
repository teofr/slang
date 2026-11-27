// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

use std::ops::Range;

use serde::Serialize;
use serde::ser::{SerializeMap, SerializeSeq, Serializer};

#[allow(clippy::wildcard_imports)]
use super::nodes::*;

struct SerializeRange<'a>(&'a Range<usize>);

impl Serialize for SerializeRange<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("start", &self.0.start)?;
        map.serialize_entry("end", &self.0.end)?;
        map.end()
    }
}

//
// Sequences
//

impl Serialize for AbicoderPragmaStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(5))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "AbicoderPragma")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("version", &self.version())?;
        map.end()
    }
}

impl Serialize for AdditiveExpressionStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(7))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "AdditiveExpression")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("left_operand", &self.left_operand())?;
        map.serialize_entry("operator", &self.operator())?;
        map.serialize_entry("right_operand", &self.right_operand())?;
        map.end()
    }
}

impl Serialize for AddressTypeStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(5))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "AddressType")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("is_payable", &self.is_payable())?;
        map.end()
    }
}

impl Serialize for AndExpressionStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(6))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "AndExpression")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("left_operand", &self.left_operand())?;
        map.serialize_entry("right_operand", &self.right_operand())?;
        map.end()
    }
}

impl Serialize for ArrayExpressionStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(5))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "ArrayExpression")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("items", &self.items())?;
        map.end()
    }
}

impl Serialize for ArrayTypeNameStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(6))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "ArrayTypeName")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("operand", &self.operand())?;
        map.serialize_entry("index", &self.index())?;
        map.end()
    }
}

impl Serialize for AssemblyStatementStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(6))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "AssemblyStatement")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("is_memory_safe", &self.is_memory_safe())?;
        map.serialize_entry("body", &self.body())?;
        map.end()
    }
}

impl Serialize for AssignmentExpressionStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(7))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "AssignmentExpression")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("left_operand", &self.left_operand())?;
        map.serialize_entry("operator", &self.operator())?;
        map.serialize_entry("right_operand", &self.right_operand())?;
        map.end()
    }
}

impl Serialize for BitwiseAndExpressionStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(6))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "BitwiseAndExpression")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("left_operand", &self.left_operand())?;
        map.serialize_entry("right_operand", &self.right_operand())?;
        map.end()
    }
}

impl Serialize for BitwiseOrExpressionStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(6))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "BitwiseOrExpression")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("left_operand", &self.left_operand())?;
        map.serialize_entry("right_operand", &self.right_operand())?;
        map.end()
    }
}

impl Serialize for BitwiseXorExpressionStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(6))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "BitwiseXorExpression")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("left_operand", &self.left_operand())?;
        map.serialize_entry("right_operand", &self.right_operand())?;
        map.end()
    }
}

impl Serialize for BlockStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(5))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "Block")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("statements", &self.statements())?;
        map.end()
    }
}

impl Serialize for BreakStatementStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "BreakStatement")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for CallOptionsExpressionStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(6))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "CallOptionsExpression")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("operand", &self.operand())?;
        map.serialize_entry("options", &self.options())?;
        map.end()
    }
}

impl Serialize for CatchClauseStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(7))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "CatchClause")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("kind", &self.kind())?;
        map.serialize_entry("parameters", &self.parameters())?;
        map.serialize_entry("body", &self.body())?;
        map.end()
    }
}

impl Serialize for ConditionalExpressionStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(7))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "ConditionalExpression")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("operand", &self.operand())?;
        map.serialize_entry("true_expression", &self.true_expression())?;
        map.serialize_entry("false_expression", &self.false_expression())?;
        map.end()
    }
}

impl Serialize for ConstantDefinitionStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(8))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "ConstantDefinition")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("type_name", &self.type_name())?;
        map.serialize_entry("name", &self.name())?;
        map.serialize_entry("visibility", &self.visibility())?;
        map.serialize_entry("value", &self.value())?;
        map.end()
    }
}

impl Serialize for ContinueStatementStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "ContinueStatement")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for ContractDefinitionStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(9))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "ContractDefinition")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("is_abstract", &self.is_abstract())?;
        map.serialize_entry("name", &self.name())?;
        map.serialize_entry("inheritance_types", &self.inheritance_types())?;
        map.serialize_entry("storage_layout", &self.storage_layout())?;
        map.serialize_entry("members", &self.members())?;
        map.end()
    }
}

impl Serialize for DecimalNumberExpressionStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(6))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "DecimalNumberExpression")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("literal", &self.literal())?;
        map.serialize_entry("unit", &self.unit())?;
        map.end()
    }
}

impl Serialize for DoWhileStatementStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(6))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "DoWhileStatement")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("body", &self.body())?;
        map.serialize_entry("condition", &self.condition())?;
        map.end()
    }
}

impl Serialize for EmitStatementStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(6))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "EmitStatement")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("event", &self.event())?;
        map.serialize_entry("arguments", &self.arguments())?;
        map.end()
    }
}

impl Serialize for EnumDefinitionStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(6))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "EnumDefinition")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("name", &self.name())?;
        map.serialize_entry("members", &self.members())?;
        map.end()
    }
}

impl Serialize for EqualityExpressionStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(7))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "EqualityExpression")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("left_operand", &self.left_operand())?;
        map.serialize_entry("operator", &self.operator())?;
        map.serialize_entry("right_operand", &self.right_operand())?;
        map.end()
    }
}

impl Serialize for ErrorDefinitionStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(6))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "ErrorDefinition")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("name", &self.name())?;
        map.serialize_entry("parameters", &self.parameters())?;
        map.end()
    }
}

impl Serialize for EventDefinitionStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(7))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "EventDefinition")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("name", &self.name())?;
        map.serialize_entry("is_anonymous", &self.is_anonymous())?;
        map.serialize_entry("parameters", &self.parameters())?;
        map.end()
    }
}

impl Serialize for ExperimentalPragmaStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(5))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "ExperimentalPragma")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("feature", &self.feature())?;
        map.end()
    }
}

impl Serialize for ExponentiationExpressionStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(6))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "ExponentiationExpression")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("left_operand", &self.left_operand())?;
        map.serialize_entry("right_operand", &self.right_operand())?;
        map.end()
    }
}

impl Serialize for ExpressionStatementStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(5))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "ExpressionStatement")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("expression", &self.expression())?;
        map.end()
    }
}

impl Serialize for ForStatementStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(8))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "ForStatement")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("initialization", &self.initialization())?;
        map.serialize_entry("condition", &self.condition())?;
        map.serialize_entry("iterator", &self.iterator())?;
        map.serialize_entry("body", &self.body())?;
        map.end()
    }
}

impl Serialize for FunctionAttributesStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(10))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "FunctionAttributes")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("visibility", &self.visibility())?;
        map.serialize_entry("has_explicit_visibility", &self.has_explicit_visibility())?;
        map.serialize_entry("mutability", &self.mutability())?;
        map.serialize_entry("is_virtual", &self.is_virtual())?;
        map.serialize_entry("override_specifier", &self.override_specifier())?;
        map.serialize_entry("modifier_invocations", &self.modifier_invocations())?;
        map.end()
    }
}

impl Serialize for FunctionCallExpressionStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(6))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "FunctionCallExpression")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("operand", &self.operand())?;
        map.serialize_entry("arguments", &self.arguments())?;
        map.end()
    }
}

impl Serialize for FunctionDefinitionStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(10))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "FunctionDefinition")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("kind", &self.kind())?;
        map.serialize_entry("name", &self.name())?;
        map.serialize_entry("parameters", &self.parameters())?;
        map.serialize_entry("attributes", &self.attributes())?;
        map.serialize_entry("returns", &self.returns())?;
        map.serialize_entry("body", &self.body())?;
        map.end()
    }
}

impl Serialize for FunctionTypeStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(7))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "FunctionType")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("parameters", &self.parameters())?;
        map.serialize_entry("attributes", &self.attributes())?;
        map.serialize_entry("returns", &self.returns())?;
        map.end()
    }
}

impl Serialize for FunctionTypeAttributesStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(6))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "FunctionTypeAttributes")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("visibility", &self.visibility())?;
        map.serialize_entry("mutability", &self.mutability())?;
        map.end()
    }
}

impl Serialize for HexNumberExpressionStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(5))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "HexNumberExpression")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("literal", &self.literal())?;
        map.end()
    }
}

impl Serialize for IfStatementStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(7))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "IfStatement")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("condition", &self.condition())?;
        map.serialize_entry("body", &self.body())?;
        map.serialize_entry("else_branch", &self.else_branch())?;
        map.end()
    }
}

impl Serialize for ImportDeconstructionStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(6))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "ImportDeconstruction")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("symbols", &self.symbols())?;
        map.serialize_entry("path", &self.path())?;
        map.end()
    }
}

impl Serialize for ImportDeconstructionSymbolStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(6))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "ImportDeconstructionSymbol")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("name", &self.name())?;
        map.serialize_entry("alias", &self.alias())?;
        map.end()
    }
}

impl Serialize for IndexAccessExpressionStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(8))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "IndexAccessExpression")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("operand", &self.operand())?;
        map.serialize_entry("start", &self.start())?;
        map.serialize_entry("end", &self.end())?;
        map.serialize_entry("is_slice", &self.is_slice())?;
        map.end()
    }
}

impl Serialize for InequalityExpressionStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(7))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "InequalityExpression")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("left_operand", &self.left_operand())?;
        map.serialize_entry("operator", &self.operator())?;
        map.serialize_entry("right_operand", &self.right_operand())?;
        map.end()
    }
}

impl Serialize for InheritanceTypeStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(6))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "InheritanceType")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("type_name", &self.type_name())?;
        map.serialize_entry("arguments", &self.arguments())?;
        map.end()
    }
}

impl Serialize for InterfaceDefinitionStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(7))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "InterfaceDefinition")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("name", &self.name())?;
        map.serialize_entry("inheritance", &self.inheritance())?;
        map.serialize_entry("members", &self.members())?;
        map.end()
    }
}

impl Serialize for LibraryDefinitionStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(6))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "LibraryDefinition")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("name", &self.name())?;
        map.serialize_entry("members", &self.members())?;
        map.end()
    }
}

impl Serialize for MappingTypeStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(6))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "MappingType")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("key_type", &self.key_type())?;
        map.serialize_entry("value_type", &self.value_type())?;
        map.end()
    }
}

impl Serialize for MemberAccessExpressionStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(6))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "MemberAccessExpression")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("operand", &self.operand())?;
        map.serialize_entry("member", &self.member())?;
        map.end()
    }
}

impl Serialize for ModifierAttributesStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(6))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "ModifierAttributes")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("is_virtual", &self.is_virtual())?;
        map.serialize_entry("override_specifier", &self.override_specifier())?;
        map.end()
    }
}

impl Serialize for ModifierDefinitionStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(8))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "ModifierDefinition")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("name", &self.name())?;
        map.serialize_entry("parameters", &self.parameters())?;
        map.serialize_entry("attributes", &self.attributes())?;
        map.serialize_entry("body", &self.body())?;
        map.end()
    }
}

impl Serialize for ModifierInvocationStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(6))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "ModifierInvocation")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("name", &self.name())?;
        map.serialize_entry("arguments", &self.arguments())?;
        map.end()
    }
}

impl Serialize for MultiTypedDeclarationStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(6))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "MultiTypedDeclaration")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("elements", &self.elements())?;
        map.serialize_entry("value", &self.value())?;
        map.end()
    }
}

impl Serialize for MultiTypedDeclarationElementStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(5))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "MultiTypedDeclarationElement")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("member", &self.member())?;
        map.end()
    }
}

impl Serialize for MultiplicativeExpressionStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(7))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "MultiplicativeExpression")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("left_operand", &self.left_operand())?;
        map.serialize_entry("operator", &self.operator())?;
        map.serialize_entry("right_operand", &self.right_operand())?;
        map.end()
    }
}

impl Serialize for NamedArgumentStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(6))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "NamedArgument")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("name", &self.name())?;
        map.serialize_entry("value", &self.value())?;
        map.end()
    }
}

impl Serialize for NewExpressionStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(5))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "NewExpression")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("type_name", &self.type_name())?;
        map.end()
    }
}

impl Serialize for OrExpressionStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(6))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "OrExpression")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("left_operand", &self.left_operand())?;
        map.serialize_entry("right_operand", &self.right_operand())?;
        map.end()
    }
}

impl Serialize for ParameterStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(8))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "Parameter")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("type_name", &self.type_name())?;
        map.serialize_entry("storage_location", &self.storage_location())?;
        map.serialize_entry("name", &self.name())?;
        map.serialize_entry("is_indexed", &self.is_indexed())?;
        map.end()
    }
}

impl Serialize for PathImportStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(6))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "PathImport")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("path", &self.path())?;
        map.serialize_entry("alias", &self.alias())?;
        map.end()
    }
}

impl Serialize for PostfixExpressionStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(6))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "PostfixExpression")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("operand", &self.operand())?;
        map.serialize_entry("operator", &self.operator())?;
        map.end()
    }
}

impl Serialize for PragmaDirectiveStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(5))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "PragmaDirective")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("pragma", &self.pragma())?;
        map.end()
    }
}

impl Serialize for PrefixExpressionStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(6))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "PrefixExpression")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("operator", &self.operator())?;
        map.serialize_entry("operand", &self.operand())?;
        map.end()
    }
}

impl Serialize for ReturnStatementStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(5))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "ReturnStatement")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("expression", &self.expression())?;
        map.end()
    }
}

impl Serialize for RevertStatementStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(6))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "RevertStatement")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("error", &self.error())?;
        map.serialize_entry("arguments", &self.arguments())?;
        map.end()
    }
}

impl Serialize for ShiftExpressionStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(7))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "ShiftExpression")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("left_operand", &self.left_operand())?;
        map.serialize_entry("operator", &self.operator())?;
        map.serialize_entry("right_operand", &self.right_operand())?;
        map.end()
    }
}

impl Serialize for SingleTypedDeclarationStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(6))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "SingleTypedDeclaration")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("declaration", &self.declaration())?;
        map.serialize_entry("value", &self.value())?;
        map.end()
    }
}

impl Serialize for SourceUnitStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(5))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "SourceUnit")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("members", &self.members())?;
        map.end()
    }
}

impl Serialize for StateVariableAttributesStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(7))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "StateVariableAttributes")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("visibility", &self.visibility())?;
        map.serialize_entry("mutability", &self.mutability())?;
        map.serialize_entry("override_specifier", &self.override_specifier())?;
        map.end()
    }
}

impl Serialize for StateVariableDefinitionStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(8))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "StateVariableDefinition")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("type_name", &self.type_name())?;
        map.serialize_entry("name", &self.name())?;
        map.serialize_entry("value", &self.value())?;
        map.serialize_entry("attributes", &self.attributes())?;
        map.end()
    }
}

impl Serialize for StructDefinitionStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(6))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "StructDefinition")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("name", &self.name())?;
        map.serialize_entry("members", &self.members())?;
        map.end()
    }
}

impl Serialize for StructMemberStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(6))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "StructMember")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("type_name", &self.type_name())?;
        map.serialize_entry("name", &self.name())?;
        map.end()
    }
}

impl Serialize for TryStatementStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(8))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "TryStatement")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("expression", &self.expression())?;
        map.serialize_entry("returns", &self.returns())?;
        map.serialize_entry("body", &self.body())?;
        map.serialize_entry("catch_clauses", &self.catch_clauses())?;
        map.end()
    }
}

impl Serialize for TupleExpressionStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(5))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "TupleExpression")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("items", &self.items())?;
        map.end()
    }
}

impl Serialize for TupleValueStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(5))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "TupleValue")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("expression", &self.expression())?;
        map.end()
    }
}

impl Serialize for TypeExpressionStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(5))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "TypeExpression")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("type_name", &self.type_name())?;
        map.end()
    }
}

impl Serialize for UncheckedBlockStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(5))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "UncheckedBlock")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("block", &self.block())?;
        map.end()
    }
}

impl Serialize for UserDefinedValueTypeDefinitionStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(6))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "UserDefinedValueTypeDefinition")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("name", &self.name())?;
        map.serialize_entry("value_type", &self.value_type())?;
        map.end()
    }
}

impl Serialize for UsingDeconstructionStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(5))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "UsingDeconstruction")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("symbols", &self.symbols())?;
        map.end()
    }
}

impl Serialize for UsingDeconstructionSymbolStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(6))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "UsingDeconstructionSymbol")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("name", &self.name())?;
        map.serialize_entry("alias", &self.alias())?;
        map.end()
    }
}

impl Serialize for UsingDirectiveStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(7))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "UsingDirective")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("clause", &self.clause())?;
        map.serialize_entry("target", &self.target())?;
        map.serialize_entry("is_global", &self.is_global())?;
        map.end()
    }
}

impl Serialize for VariableDeclarationStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(7))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "VariableDeclaration")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("type_name", &self.type_name())?;
        map.serialize_entry("storage_location", &self.storage_location())?;
        map.serialize_entry("name", &self.name())?;
        map.end()
    }
}

impl Serialize for VariableDeclarationStatementStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(5))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "VariableDeclarationStatement")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("target", &self.target())?;
        map.end()
    }
}

impl Serialize for VersionPragmaStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(5))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "VersionPragma")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("sets", &self.sets())?;
        map.end()
    }
}

impl Serialize for VersionPragmaComparatorStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(6))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "VersionPragmaComparator")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("operator", &self.operator())?;
        map.serialize_entry("specifier", &self.specifier())?;
        map.end()
    }
}

impl Serialize for WhileStatementStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(6))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "WhileStatement")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("condition", &self.condition())?;
        map.serialize_entry("body", &self.body())?;
        map.end()
    }
}

impl Serialize for YulBlockStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(5))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "YulBlock")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("statements", &self.statements())?;
        map.end()
    }
}

impl Serialize for YulBreakStatementStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "YulBreakStatement")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for YulContinueStatementStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "YulContinueStatement")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for YulDefaultCaseStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(5))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "YulDefaultCase")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("body", &self.body())?;
        map.end()
    }
}

impl Serialize for YulForStatementStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(8))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "YulForStatement")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("initialization", &self.initialization())?;
        map.serialize_entry("condition", &self.condition())?;
        map.serialize_entry("iterator", &self.iterator())?;
        map.serialize_entry("body", &self.body())?;
        map.end()
    }
}

impl Serialize for YulFunctionCallExpressionStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(6))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "YulFunctionCallExpression")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("operand", &self.operand())?;
        map.serialize_entry("arguments", &self.arguments())?;
        map.end()
    }
}

impl Serialize for YulFunctionDefinitionStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(8))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "YulFunctionDefinition")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("name", &self.name())?;
        map.serialize_entry("parameters", &self.parameters())?;
        map.serialize_entry("returns", &self.returns())?;
        map.serialize_entry("body", &self.body())?;
        map.end()
    }
}

impl Serialize for YulIfStatementStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(6))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "YulIfStatement")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("condition", &self.condition())?;
        map.serialize_entry("body", &self.body())?;
        map.end()
    }
}

impl Serialize for YulLeaveStatementStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "YulLeaveStatement")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for YulSwitchStatementStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(7))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "YulSwitchStatement")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("expression", &self.expression())?;
        map.serialize_entry("value_cases", &self.value_cases())?;
        map.serialize_entry("default_case", &self.default_case())?;
        map.end()
    }
}

impl Serialize for YulValueCaseStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(6))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "YulValueCase")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("value", &self.value())?;
        map.serialize_entry("body", &self.body())?;
        map.end()
    }
}

impl Serialize for YulVariableAssignmentStatementStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(6))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "YulVariableAssignmentStatement")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("variables", &self.variables())?;
        map.serialize_entry("expression", &self.expression())?;
        map.end()
    }
}

impl Serialize for YulVariableDeclarationStatementStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(6))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "YulVariableDeclarationStatement")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("variables", &self.variables())?;
        map.serialize_entry("value", &self.value())?;
        map.end()
    }
}

impl Serialize for YulVariableDeclarationValueStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(5))?;
        map.serialize_entry("id", &self.node_id())?;
        map.serialize_entry("type", "YulVariableDeclarationValue")?;
        map.serialize_entry("range", &SerializeRange(self.get_text_range()))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("expression", &self.expression())?;
        map.end()
    }
}

//
// Choices
//

impl Serialize for AbicoderVersion {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            AbicoderVersion::V1 => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "AbicoderVersion::V1")?;
                map.end()
            }
            AbicoderVersion::V2 => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "AbicoderVersion::V2")?;
                map.end()
            }
        }
    }
}

impl Serialize for AdditiveExpressionOperator<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            AdditiveExpressionOperator::Minus(inner) => inner.serialize(serializer),
            AdditiveExpressionOperator::Plus(inner) => inner.serialize(serializer),
        }
    }
}

impl Serialize for ArgumentsDeclaration<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            ArgumentsDeclaration::PositionalArguments(inner) => inner.serialize(serializer),
            ArgumentsDeclaration::NamedArguments(inner) => inner.serialize(serializer),
        }
    }
}

impl Serialize for AssignmentExpressionOperator<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            AssignmentExpressionOperator::AmpersandEqual(inner) => inner.serialize(serializer),
            AssignmentExpressionOperator::AsteriskEqual(inner) => inner.serialize(serializer),
            AssignmentExpressionOperator::BarEqual(inner) => inner.serialize(serializer),
            AssignmentExpressionOperator::CaretEqual(inner) => inner.serialize(serializer),
            AssignmentExpressionOperator::Equal(inner) => inner.serialize(serializer),
            AssignmentExpressionOperator::GreaterThanGreaterThanEqual(inner) => {
                inner.serialize(serializer)
            }
            AssignmentExpressionOperator::GreaterThanGreaterThanGreaterThanEqual(inner) => {
                inner.serialize(serializer)
            }
            AssignmentExpressionOperator::LessThanLessThanEqual(inner) => {
                inner.serialize(serializer)
            }
            AssignmentExpressionOperator::MinusEqual(inner) => inner.serialize(serializer),
            AssignmentExpressionOperator::PercentEqual(inner) => inner.serialize(serializer),
            AssignmentExpressionOperator::PlusEqual(inner) => inner.serialize(serializer),
            AssignmentExpressionOperator::SlashEqual(inner) => inner.serialize(serializer),
        }
    }
}

impl Serialize for CatchClauseKind {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            CatchClauseKind::Error => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "CatchClauseKind::Error")?;
                map.end()
            }
            CatchClauseKind::Panic => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "CatchClauseKind::Panic")?;
                map.end()
            }
            CatchClauseKind::LowLevel => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "CatchClauseKind::LowLevel")?;
                map.end()
            }
        }
    }
}

impl Serialize for ContractMember<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            ContractMember::UsingDirective(inner) => inner.serialize(serializer),
            ContractMember::FunctionDefinition(inner) => inner.serialize(serializer),
            ContractMember::ModifierDefinition(inner) => inner.serialize(serializer),
            ContractMember::StructDefinition(inner) => inner.serialize(serializer),
            ContractMember::EnumDefinition(inner) => inner.serialize(serializer),
            ContractMember::EventDefinition(inner) => inner.serialize(serializer),
            ContractMember::ErrorDefinition(inner) => inner.serialize(serializer),
            ContractMember::UserDefinedValueTypeDefinition(inner) => inner.serialize(serializer),
            ContractMember::StateVariableDefinition(inner) => inner.serialize(serializer),
            ContractMember::ConstantDefinition(inner) => inner.serialize(serializer),
        }
    }
}

impl Serialize for ElementaryType<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            ElementaryType::BoolKeyword(inner) => inner.serialize(serializer),
            ElementaryType::StringKeyword(inner) => inner.serialize(serializer),
            ElementaryType::AddressType(inner) => inner.serialize(serializer),
            ElementaryType::BytesKeyword(inner) => inner.serialize(serializer),
            ElementaryType::IntKeyword(inner) => inner.serialize(serializer),
            ElementaryType::UintKeyword(inner) => inner.serialize(serializer),
            ElementaryType::FixedKeyword(inner) => inner.serialize(serializer),
            ElementaryType::UfixedKeyword(inner) => inner.serialize(serializer),
        }
    }
}

impl Serialize for EqualityExpressionOperator<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            EqualityExpressionOperator::BangEqual(inner) => inner.serialize(serializer),
            EqualityExpressionOperator::EqualEqual(inner) => inner.serialize(serializer),
        }
    }
}

impl Serialize for ExperimentalFeature {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            ExperimentalFeature::ABIEncoderV2 => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "ExperimentalFeature::ABIEncoderV2")?;
                map.end()
            }
            ExperimentalFeature::SMTChecker => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "ExperimentalFeature::SMTChecker")?;
                map.end()
            }
            ExperimentalFeature::Solidity => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "ExperimentalFeature::Solidity")?;
                map.end()
            }
            ExperimentalFeature::Unrecognized => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "ExperimentalFeature::Unrecognized")?;
                map.end()
            }
        }
    }
}

impl Serialize for Expression<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Expression::AssignmentExpression(inner) => inner.serialize(serializer),
            Expression::ConditionalExpression(inner) => inner.serialize(serializer),
            Expression::OrExpression(inner) => inner.serialize(serializer),
            Expression::AndExpression(inner) => inner.serialize(serializer),
            Expression::EqualityExpression(inner) => inner.serialize(serializer),
            Expression::InequalityExpression(inner) => inner.serialize(serializer),
            Expression::BitwiseOrExpression(inner) => inner.serialize(serializer),
            Expression::BitwiseXorExpression(inner) => inner.serialize(serializer),
            Expression::BitwiseAndExpression(inner) => inner.serialize(serializer),
            Expression::ShiftExpression(inner) => inner.serialize(serializer),
            Expression::AdditiveExpression(inner) => inner.serialize(serializer),
            Expression::MultiplicativeExpression(inner) => inner.serialize(serializer),
            Expression::ExponentiationExpression(inner) => inner.serialize(serializer),
            Expression::PostfixExpression(inner) => inner.serialize(serializer),
            Expression::PrefixExpression(inner) => inner.serialize(serializer),
            Expression::FunctionCallExpression(inner) => inner.serialize(serializer),
            Expression::CallOptionsExpression(inner) => inner.serialize(serializer),
            Expression::MemberAccessExpression(inner) => inner.serialize(serializer),
            Expression::IndexAccessExpression(inner) => inner.serialize(serializer),
            Expression::NewExpression(inner) => inner.serialize(serializer),
            Expression::TupleExpression(inner) => inner.serialize(serializer),
            Expression::TypeExpression(inner) => inner.serialize(serializer),
            Expression::ArrayExpression(inner) => inner.serialize(serializer),
            Expression::HexNumberExpression(inner) => inner.serialize(serializer),
            Expression::DecimalNumberExpression(inner) => inner.serialize(serializer),
            Expression::StringExpression(inner) => inner.serialize(serializer),
            Expression::ElementaryType(inner) => inner.serialize(serializer),
            Expression::PayableKeyword(inner) => inner.serialize(serializer),
            Expression::ThisKeyword(inner) => inner.serialize(serializer),
            Expression::SuperKeyword(inner) => inner.serialize(serializer),
            Expression::TrueKeyword(inner) => inner.serialize(serializer),
            Expression::FalseKeyword(inner) => inner.serialize(serializer),
            Expression::Identifier(inner) => inner.serialize(serializer),
        }
    }
}

impl Serialize for ForStatementCondition<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            ForStatementCondition::ExpressionStatement(inner) => inner.serialize(serializer),
            ForStatementCondition::Semicolon(inner) => inner.serialize(serializer),
        }
    }
}

impl Serialize for ForStatementInitialization<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            ForStatementInitialization::VariableDeclarationStatement(inner) => {
                inner.serialize(serializer)
            }
            ForStatementInitialization::ExpressionStatement(inner) => inner.serialize(serializer),
            ForStatementInitialization::Semicolon(inner) => inner.serialize(serializer),
        }
    }
}

impl Serialize for FunctionKind {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            FunctionKind::Regular => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "FunctionKind::Regular")?;
                map.end()
            }
            FunctionKind::Constructor => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "FunctionKind::Constructor")?;
                map.end()
            }
            FunctionKind::Fallback => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "FunctionKind::Fallback")?;
                map.end()
            }
            FunctionKind::Receive => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "FunctionKind::Receive")?;
                map.end()
            }
        }
    }
}

impl Serialize for FunctionMutability {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            FunctionMutability::Pure => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "FunctionMutability::Pure")?;
                map.end()
            }
            FunctionMutability::View => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "FunctionMutability::View")?;
                map.end()
            }
            FunctionMutability::NonPayable => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "FunctionMutability::NonPayable")?;
                map.end()
            }
            FunctionMutability::Payable => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "FunctionMutability::Payable")?;
                map.end()
            }
        }
    }
}

impl Serialize for FunctionVisibility {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            FunctionVisibility::Public => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "FunctionVisibility::Public")?;
                map.end()
            }
            FunctionVisibility::Private => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "FunctionVisibility::Private")?;
                map.end()
            }
            FunctionVisibility::Internal => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "FunctionVisibility::Internal")?;
                map.end()
            }
            FunctionVisibility::External => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "FunctionVisibility::External")?;
                map.end()
            }
        }
    }
}

impl Serialize for ImportClause<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            ImportClause::PathImport(inner) => inner.serialize(serializer),
            ImportClause::ImportDeconstruction(inner) => inner.serialize(serializer),
        }
    }
}

impl Serialize for InequalityExpressionOperator<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            InequalityExpressionOperator::GreaterThan(inner) => inner.serialize(serializer),
            InequalityExpressionOperator::GreaterThanEqual(inner) => inner.serialize(serializer),
            InequalityExpressionOperator::LessThan(inner) => inner.serialize(serializer),
            InequalityExpressionOperator::LessThanEqual(inner) => inner.serialize(serializer),
        }
    }
}

impl Serialize for MultiplicativeExpressionOperator<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            MultiplicativeExpressionOperator::Asterisk(inner) => inner.serialize(serializer),
            MultiplicativeExpressionOperator::Percent(inner) => inner.serialize(serializer),
            MultiplicativeExpressionOperator::Slash(inner) => inner.serialize(serializer),
        }
    }
}

impl Serialize for NumberUnit<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            NumberUnit::WeiKeyword(inner) => inner.serialize(serializer),
            NumberUnit::GweiKeyword(inner) => inner.serialize(serializer),
            NumberUnit::EtherKeyword(inner) => inner.serialize(serializer),
            NumberUnit::SecondsKeyword(inner) => inner.serialize(serializer),
            NumberUnit::MinutesKeyword(inner) => inner.serialize(serializer),
            NumberUnit::HoursKeyword(inner) => inner.serialize(serializer),
            NumberUnit::DaysKeyword(inner) => inner.serialize(serializer),
            NumberUnit::WeeksKeyword(inner) => inner.serialize(serializer),
        }
    }
}

impl Serialize for PostfixExpressionOperator<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            PostfixExpressionOperator::MinusMinus(inner) => inner.serialize(serializer),
            PostfixExpressionOperator::PlusPlus(inner) => inner.serialize(serializer),
        }
    }
}

impl Serialize for Pragma<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Pragma::VersionPragma(inner) => inner.serialize(serializer),
            Pragma::AbicoderPragma(inner) => inner.serialize(serializer),
            Pragma::ExperimentalPragma(inner) => inner.serialize(serializer),
        }
    }
}

impl Serialize for PrefixExpressionOperator<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            PrefixExpressionOperator::Bang(inner) => inner.serialize(serializer),
            PrefixExpressionOperator::DeleteKeyword(inner) => inner.serialize(serializer),
            PrefixExpressionOperator::Minus(inner) => inner.serialize(serializer),
            PrefixExpressionOperator::MinusMinus(inner) => inner.serialize(serializer),
            PrefixExpressionOperator::PlusPlus(inner) => inner.serialize(serializer),
            PrefixExpressionOperator::Tilde(inner) => inner.serialize(serializer),
        }
    }
}

impl Serialize for ShiftExpressionOperator<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            ShiftExpressionOperator::GreaterThanGreaterThan(inner) => inner.serialize(serializer),
            ShiftExpressionOperator::GreaterThanGreaterThanGreaterThan(inner) => {
                inner.serialize(serializer)
            }
            ShiftExpressionOperator::LessThanLessThan(inner) => inner.serialize(serializer),
        }
    }
}

impl Serialize for SourceUnitMember<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            SourceUnitMember::PragmaDirective(inner) => inner.serialize(serializer),
            SourceUnitMember::ImportClause(inner) => inner.serialize(serializer),
            SourceUnitMember::ContractDefinition(inner) => inner.serialize(serializer),
            SourceUnitMember::InterfaceDefinition(inner) => inner.serialize(serializer),
            SourceUnitMember::LibraryDefinition(inner) => inner.serialize(serializer),
            SourceUnitMember::StructDefinition(inner) => inner.serialize(serializer),
            SourceUnitMember::EnumDefinition(inner) => inner.serialize(serializer),
            SourceUnitMember::FunctionDefinition(inner) => inner.serialize(serializer),
            SourceUnitMember::ErrorDefinition(inner) => inner.serialize(serializer),
            SourceUnitMember::UserDefinedValueTypeDefinition(inner) => inner.serialize(serializer),
            SourceUnitMember::UsingDirective(inner) => inner.serialize(serializer),
            SourceUnitMember::EventDefinition(inner) => inner.serialize(serializer),
            SourceUnitMember::ConstantDefinition(inner) => inner.serialize(serializer),
        }
    }
}

impl Serialize for StateVariableMutability {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            StateVariableMutability::Mutable => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "StateVariableMutability::Mutable")?;
                map.end()
            }
            StateVariableMutability::Constant => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "StateVariableMutability::Constant")?;
                map.end()
            }
            StateVariableMutability::Immutable => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "StateVariableMutability::Immutable")?;
                map.end()
            }
            StateVariableMutability::Transient => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "StateVariableMutability::Transient")?;
                map.end()
            }
        }
    }
}

impl Serialize for StateVariableVisibility {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            StateVariableVisibility::Public => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "StateVariableVisibility::Public")?;
                map.end()
            }
            StateVariableVisibility::Private => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "StateVariableVisibility::Private")?;
                map.end()
            }
            StateVariableVisibility::Internal => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "StateVariableVisibility::Internal")?;
                map.end()
            }
        }
    }
}

impl Serialize for Statement<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Statement::IfStatement(inner) => inner.serialize(serializer),
            Statement::ForStatement(inner) => inner.serialize(serializer),
            Statement::WhileStatement(inner) => inner.serialize(serializer),
            Statement::DoWhileStatement(inner) => inner.serialize(serializer),
            Statement::ContinueStatement(inner) => inner.serialize(serializer),
            Statement::BreakStatement(inner) => inner.serialize(serializer),
            Statement::ReturnStatement(inner) => inner.serialize(serializer),
            Statement::EmitStatement(inner) => inner.serialize(serializer),
            Statement::TryStatement(inner) => inner.serialize(serializer),
            Statement::RevertStatement(inner) => inner.serialize(serializer),
            Statement::AssemblyStatement(inner) => inner.serialize(serializer),
            Statement::Block(inner) => inner.serialize(serializer),
            Statement::UncheckedBlock(inner) => inner.serialize(serializer),
            Statement::VariableDeclarationStatement(inner) => inner.serialize(serializer),
            Statement::ExpressionStatement(inner) => inner.serialize(serializer),
        }
    }
}

impl Serialize for StorageLocation<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            StorageLocation::MemoryKeyword(inner) => inner.serialize(serializer),
            StorageLocation::StorageKeyword(inner) => inner.serialize(serializer),
            StorageLocation::CallDataKeyword(inner) => inner.serialize(serializer),
        }
    }
}

impl Serialize for StringExpression<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            StringExpression::StringLiterals(inner) => inner.serialize(serializer),
            StringExpression::HexStringLiterals(inner) => inner.serialize(serializer),
            StringExpression::UnicodeStringLiterals(inner) => inner.serialize(serializer),
        }
    }
}

impl Serialize for TypeName<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            TypeName::ArrayTypeName(inner) => inner.serialize(serializer),
            TypeName::FunctionType(inner) => inner.serialize(serializer),
            TypeName::MappingType(inner) => inner.serialize(serializer),
            TypeName::ElementaryType(inner) => inner.serialize(serializer),
            TypeName::IdentifierPath(inner) => inner.serialize(serializer),
        }
    }
}

impl Serialize for UsingClause<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            UsingClause::IdentifierPath(inner) => inner.serialize(serializer),
            UsingClause::UsingDeconstruction(inner) => inner.serialize(serializer),
        }
    }
}

impl Serialize for UsingOperator<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            UsingOperator::Ampersand(inner) => inner.serialize(serializer),
            UsingOperator::Asterisk(inner) => inner.serialize(serializer),
            UsingOperator::BangEqual(inner) => inner.serialize(serializer),
            UsingOperator::Bar(inner) => inner.serialize(serializer),
            UsingOperator::Caret(inner) => inner.serialize(serializer),
            UsingOperator::EqualEqual(inner) => inner.serialize(serializer),
            UsingOperator::GreaterThan(inner) => inner.serialize(serializer),
            UsingOperator::GreaterThanEqual(inner) => inner.serialize(serializer),
            UsingOperator::LessThan(inner) => inner.serialize(serializer),
            UsingOperator::LessThanEqual(inner) => inner.serialize(serializer),
            UsingOperator::Minus(inner) => inner.serialize(serializer),
            UsingOperator::Percent(inner) => inner.serialize(serializer),
            UsingOperator::Plus(inner) => inner.serialize(serializer),
            UsingOperator::Slash(inner) => inner.serialize(serializer),
            UsingOperator::Tilde(inner) => inner.serialize(serializer),
        }
    }
}

impl Serialize for UsingTarget<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            UsingTarget::TypeName(inner) => inner.serialize(serializer),
            UsingTarget::Asterisk(inner) => inner.serialize(serializer),
        }
    }
}

impl Serialize for VariableDeclarationTarget<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            VariableDeclarationTarget::SingleTypedDeclaration(inner) => inner.serialize(serializer),
            VariableDeclarationTarget::MultiTypedDeclaration(inner) => inner.serialize(serializer),
        }
    }
}

impl Serialize for VersionPragmaComponent<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            VersionPragmaComponent::Wildcard => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "VersionPragmaComponent::Wildcard")?;
                map.end()
            }
            VersionPragmaComponent::Unrecognized => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "VersionPragmaComponent::Unrecognized")?;
                map.end()
            }
            VersionPragmaComponent::Number(inner) => {
                let mut map = serializer.serialize_map(Some(2))?;
                map.serialize_entry("type", "VersionPragmaComponent::Number")?;
                map.serialize_entry("value", inner)?;
                map.end()
            }
        }
    }
}

impl Serialize for VersionPragmaOperator {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            VersionPragmaOperator::Caret => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "VersionPragmaOperator::Caret")?;
                map.end()
            }
            VersionPragmaOperator::Tilde => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "VersionPragmaOperator::Tilde")?;
                map.end()
            }
            VersionPragmaOperator::Equal => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "VersionPragmaOperator::Equal")?;
                map.end()
            }
            VersionPragmaOperator::LessThan => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "VersionPragmaOperator::LessThan")?;
                map.end()
            }
            VersionPragmaOperator::LessThanEqual => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "VersionPragmaOperator::LessThanEqual")?;
                map.end()
            }
            VersionPragmaOperator::GreaterThan => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "VersionPragmaOperator::GreaterThan")?;
                map.end()
            }
            VersionPragmaOperator::GreaterThanEqual => {
                let mut map = serializer.serialize_map(Some(1))?;
                map.serialize_entry("type", "VersionPragmaOperator::GreaterThanEqual")?;
                map.end()
            }
        }
    }
}

impl Serialize for YulExpression<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            YulExpression::YulFunctionCallExpression(inner) => inner.serialize(serializer),
            YulExpression::YulLiteral(inner) => inner.serialize(serializer),
            YulExpression::YulPath(inner) => inner.serialize(serializer),
        }
    }
}

impl Serialize for YulLiteral<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            YulLiteral::TrueKeyword(inner) => inner.serialize(serializer),
            YulLiteral::FalseKeyword(inner) => inner.serialize(serializer),
            YulLiteral::DecimalLiteral(inner) => inner.serialize(serializer),
            YulLiteral::HexLiteral(inner) => inner.serialize(serializer),
            YulLiteral::HexStringLiteral(inner) => inner.serialize(serializer),
            YulLiteral::StringLiteral(inner) => inner.serialize(serializer),
        }
    }
}

impl Serialize for YulStatement<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            YulStatement::YulBlock(inner) => inner.serialize(serializer),
            YulStatement::YulFunctionDefinition(inner) => inner.serialize(serializer),
            YulStatement::YulIfStatement(inner) => inner.serialize(serializer),
            YulStatement::YulForStatement(inner) => inner.serialize(serializer),
            YulStatement::YulSwitchStatement(inner) => inner.serialize(serializer),
            YulStatement::YulLeaveStatement(inner) => inner.serialize(serializer),
            YulStatement::YulBreakStatement(inner) => inner.serialize(serializer),
            YulStatement::YulContinueStatement(inner) => inner.serialize(serializer),
            YulStatement::YulVariableAssignmentStatement(inner) => inner.serialize(serializer),
            YulStatement::YulVariableDeclarationStatement(inner) => inner.serialize(serializer),
            YulStatement::YulExpression(inner) => inner.serialize(serializer),
        }
    }
}

//
// Repeated & Separated
//

impl Serialize for ArrayValuesStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for item in self.iter() {
            seq.serialize_element(&item)?;
        }
        seq.end()
    }
}

impl Serialize for CallOptionsStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for item in self.iter() {
            seq.serialize_element(&item)?;
        }
        seq.end()
    }
}

impl Serialize for CatchClausesStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for item in self.iter() {
            seq.serialize_element(&item)?;
        }
        seq.end()
    }
}

impl Serialize for ContractMembersStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for item in self.iter() {
            seq.serialize_element(&item)?;
        }
        seq.end()
    }
}

impl Serialize for EnumMembersStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for item in self.iter() {
            seq.serialize_element(&item)?;
        }
        seq.end()
    }
}

impl Serialize for HexStringLiteralsStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for item in self.iter() {
            seq.serialize_element(&item)?;
        }
        seq.end()
    }
}

impl Serialize for IdentifierPathStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for item in self.iter() {
            seq.serialize_element(&item)?;
        }
        seq.end()
    }
}

impl Serialize for ImportDeconstructionSymbolsStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for item in self.iter() {
            seq.serialize_element(&item)?;
        }
        seq.end()
    }
}

impl Serialize for InheritanceTypesStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for item in self.iter() {
            seq.serialize_element(&item)?;
        }
        seq.end()
    }
}

impl Serialize for InterfaceMembersStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for item in self.iter() {
            seq.serialize_element(&item)?;
        }
        seq.end()
    }
}

impl Serialize for LibraryMembersStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for item in self.iter() {
            seq.serialize_element(&item)?;
        }
        seq.end()
    }
}

impl Serialize for ModifierInvocationsStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for item in self.iter() {
            seq.serialize_element(&item)?;
        }
        seq.end()
    }
}

impl Serialize for MultiTypedDeclarationElementsStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for item in self.iter() {
            seq.serialize_element(&item)?;
        }
        seq.end()
    }
}

impl Serialize for NamedArgumentsStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for item in self.iter() {
            seq.serialize_element(&item)?;
        }
        seq.end()
    }
}

impl Serialize for OverridePathsStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for item in self.iter() {
            seq.serialize_element(&item)?;
        }
        seq.end()
    }
}

impl Serialize for ParametersStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for item in self.iter() {
            seq.serialize_element(&item)?;
        }
        seq.end()
    }
}

impl Serialize for PositionalArgumentsStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for item in self.iter() {
            seq.serialize_element(&item)?;
        }
        seq.end()
    }
}

impl Serialize for SourceUnitMembersStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for item in self.iter() {
            seq.serialize_element(&item)?;
        }
        seq.end()
    }
}

impl Serialize for StatementsStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for item in self.iter() {
            seq.serialize_element(&item)?;
        }
        seq.end()
    }
}

impl Serialize for StringLiteralsStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for item in self.iter() {
            seq.serialize_element(&item)?;
        }
        seq.end()
    }
}

impl Serialize for StructMembersStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for item in self.iter() {
            seq.serialize_element(&item)?;
        }
        seq.end()
    }
}

impl Serialize for TupleValuesStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for item in self.iter() {
            seq.serialize_element(&item)?;
        }
        seq.end()
    }
}

impl Serialize for UnicodeStringLiteralsStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for item in self.iter() {
            seq.serialize_element(&item)?;
        }
        seq.end()
    }
}

impl Serialize for UsingDeconstructionSymbolsStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for item in self.iter() {
            seq.serialize_element(&item)?;
        }
        seq.end()
    }
}

impl Serialize for VersionPragmaExpressionSetStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for item in self.iter() {
            seq.serialize_element(&item)?;
        }
        seq.end()
    }
}

impl Serialize for VersionPragmaExpressionSetsStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for item in self.iter() {
            seq.serialize_element(&item)?;
        }
        seq.end()
    }
}

impl Serialize for VersionPragmaSpecifierStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for item in self.iter() {
            seq.serialize_element(&item)?;
        }
        seq.end()
    }
}

impl Serialize for YulArgumentsStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for item in self.iter() {
            seq.serialize_element(&item)?;
        }
        seq.end()
    }
}

impl Serialize for YulParametersStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for item in self.iter() {
            seq.serialize_element(&item)?;
        }
        seq.end()
    }
}

impl Serialize for YulPathStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for item in self.iter() {
            seq.serialize_element(&item)?;
        }
        seq.end()
    }
}

impl Serialize for YulPathsStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for item in self.iter() {
            seq.serialize_element(&item)?;
        }
        seq.end()
    }
}

impl Serialize for YulStatementsStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for item in self.iter() {
            seq.serialize_element(&item)?;
        }
        seq.end()
    }
}

impl Serialize for YulValueCasesStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for item in self.iter() {
            seq.serialize_element(&item)?;
        }
        seq.end()
    }
}

impl Serialize for YulVariableNamesStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for item in self.iter() {
            seq.serialize_element(&item)?;
        }
        seq.end()
    }
}

//
// Terminals
//

impl Serialize for AmpersandStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "Ampersand")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for AmpersandEqualStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "AmpersandEqual")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for AsteriskStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "Asterisk")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for AsteriskEqualStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "AsteriskEqual")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for BangStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "Bang")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for BangEqualStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "BangEqual")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for BarStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "Bar")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for BarEqualStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "BarEqual")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for BoolKeywordStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "BoolKeyword")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for BytesKeywordStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(5))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "BytesKeyword")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("text", self.ir_node.unparse())?;
        map.end()
    }
}

impl Serialize for CallDataKeywordStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "CallDataKeyword")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for CaretStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "Caret")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for CaretEqualStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "CaretEqual")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for DaysKeywordStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "DaysKeyword")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for DecimalLiteralStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(5))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "DecimalLiteral")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("text", self.ir_node.unparse())?;
        map.end()
    }
}

impl Serialize for DeleteKeywordStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "DeleteKeyword")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for EqualStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "Equal")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for EqualEqualStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "EqualEqual")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for EtherKeywordStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "EtherKeyword")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for FalseKeywordStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "FalseKeyword")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for FixedKeywordStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(5))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "FixedKeyword")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("text", self.ir_node.unparse())?;
        map.end()
    }
}

impl Serialize for GreaterThanStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "GreaterThan")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for GreaterThanEqualStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "GreaterThanEqual")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for GreaterThanGreaterThanStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "GreaterThanGreaterThan")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for GreaterThanGreaterThanEqualStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "GreaterThanGreaterThanEqual")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for GreaterThanGreaterThanGreaterThanStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "GreaterThanGreaterThanGreaterThan")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for GreaterThanGreaterThanGreaterThanEqualStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "GreaterThanGreaterThanGreaterThanEqual")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for GweiKeywordStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "GweiKeyword")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for HexLiteralStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(5))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "HexLiteral")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("text", self.ir_node.unparse())?;
        map.end()
    }
}

impl Serialize for HexStringLiteralStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(5))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "HexStringLiteral")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("text", self.ir_node.unparse())?;
        map.end()
    }
}

impl Serialize for HoursKeywordStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "HoursKeyword")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for IdentifierStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(5))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "Identifier")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("text", self.ir_node.unparse())?;
        map.end()
    }
}

impl Serialize for IntKeywordStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(5))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "IntKeyword")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("text", self.ir_node.unparse())?;
        map.end()
    }
}

impl Serialize for LessThanStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "LessThan")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for LessThanEqualStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "LessThanEqual")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for LessThanLessThanStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "LessThanLessThan")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for LessThanLessThanEqualStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "LessThanLessThanEqual")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for MemoryKeywordStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "MemoryKeyword")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for MinusStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "Minus")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for MinusEqualStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "MinusEqual")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for MinusMinusStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "MinusMinus")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for MinutesKeywordStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "MinutesKeyword")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for PayableKeywordStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "PayableKeyword")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for PercentStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "Percent")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for PercentEqualStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "PercentEqual")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for PlusStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "Plus")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for PlusEqualStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "PlusEqual")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for PlusPlusStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "PlusPlus")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for SecondsKeywordStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "SecondsKeyword")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for SemicolonStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "Semicolon")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for SlashStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "Slash")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for SlashEqualStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "SlashEqual")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for StorageKeywordStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "StorageKeyword")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for StringKeywordStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "StringKeyword")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for StringLiteralStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(5))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "StringLiteral")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("text", self.ir_node.unparse())?;
        map.end()
    }
}

impl Serialize for SuperKeywordStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "SuperKeyword")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for ThisKeywordStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "ThisKeyword")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for TildeStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "Tilde")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for TrueKeywordStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "TrueKeyword")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for UfixedKeywordStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(5))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "UfixedKeyword")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("text", self.ir_node.unparse())?;
        map.end()
    }
}

impl Serialize for UintKeywordStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(5))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "UintKeyword")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("text", self.ir_node.unparse())?;
        map.end()
    }
}

impl Serialize for UnicodeStringLiteralStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(5))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "UnicodeStringLiteral")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.serialize_entry("text", self.ir_node.unparse())?;
        map.end()
    }
}

impl Serialize for WeeksKeywordStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "WeeksKeyword")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}

impl Serialize for WeiKeywordStruct<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let mut map = serializer.serialize_map(Some(4))?;
        map.serialize_entry("id", &self.ir_node.id())?;
        map.serialize_entry("type", "WeiKeyword")?;
        map.serialize_entry("range", &SerializeRange(&self.ir_node.range))?;
        map.serialize_entry("file", self.get_file_id())?;
        map.end()
    }
}
