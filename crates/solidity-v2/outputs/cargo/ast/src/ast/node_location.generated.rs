// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#![allow(clippy::wildcard_imports)]

use std::ops::Range;

use slang_solidity_v2_common::files::FileId;
use slang_solidity_v2_ir::ir::{NodeIdentity, TextRange};

use super::nodes::*;

/// A trait for AST nodes that can report their location in the source.
///
/// It returns `None` for nodes that can be legitimately
/// empty, ie. collections (eg. the positional arguments of `f()`),
/// and for nodes that are not represented in the source code,
/// ie. unit and external variants.
pub trait NodeLocation {
    /// Returns the ID of the file this node belongs to, or `None` if the node
    /// is empty or not represented in the source code.
    ///
    /// Each call looks it up in the files of the compilation.
    fn calculate_file_id(&self) -> Option<&FileId>;

    /// Returns the text range of this node, or `None` if the node is empty
    /// or not represented in the source code.
    fn calculate_text_range(&self) -> Option<Range<usize>>;
}

impl NodeLocation for AbicoderPragmaStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for AdditiveExpressionStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for AddressTypeStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for AndExpressionStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for ArrayExpressionStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for ArrayTypeNameStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for AssemblyStatementStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for AssignmentExpressionStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for BitwiseAndExpressionStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for BitwiseOrExpressionStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for BitwiseXorExpressionStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for BlockStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for BreakStatementStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for CallOptionsExpressionStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for CatchClauseStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for ConditionalExpressionStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for ConstantDefinitionStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for ContinueStatementStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for ContractDefinitionStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for DecimalNumberExpressionStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for DoWhileStatementStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for EmitStatementStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for EnumDefinitionStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for EqualityExpressionStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for ErrorDefinitionStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for EventDefinitionStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for ExperimentalPragmaStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for ExponentiationExpressionStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for ExpressionStatementStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for ForStatementStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for FunctionAttributesStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for FunctionCallExpressionStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for FunctionDefinitionStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for FunctionTypeStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for FunctionTypeAttributesStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for HexNumberExpressionStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for IfStatementStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for ImportDeconstructionStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for ImportDeconstructionSymbolStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for IndexAccessExpressionStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for InequalityExpressionStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for InheritanceTypeStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for InterfaceDefinitionStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for LibraryDefinitionStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for MappingTypeStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for MemberAccessExpressionStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for ModifierAttributesStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for ModifierDefinitionStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for ModifierInvocationStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for MultiTypedDeclarationStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for MultiTypedDeclarationElementStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for MultiplicativeExpressionStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for NamedArgumentStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for NewExpressionStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for OrExpressionStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for ParameterStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for PathImportStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for PostfixExpressionStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for PragmaDirectiveStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for PrefixExpressionStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for ReturnStatementStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for RevertStatementStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for ShiftExpressionStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for SingleTypedDeclarationStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for SourceUnitStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for StateVariableAttributesStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for StateVariableDefinitionStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for StructDefinitionStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for StructMemberStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for TryStatementStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for TupleExpressionStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for TupleValueStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for TypeExpressionStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for UncheckedBlockStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for UserDefinedValueTypeDefinitionStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for UsingDeconstructionStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for UsingDeconstructionSymbolStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for UsingDirectiveStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for VariableDeclarationStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for VariableDeclarationStatementStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for VersionPragmaStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for VersionPragmaComparatorStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for WhileStatementStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for YulBlockStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for YulBreakStatementStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for YulContinueStatementStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for YulDefaultCaseStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for YulForStatementStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for YulFunctionCallExpressionStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for YulFunctionDefinitionStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for YulIfStatementStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for YulLeaveStatementStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for YulSwitchStatementStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for YulValueCaseStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for YulVariableAssignmentStatementStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for YulVariableDeclarationStatementStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for YulVariableDeclarationValueStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for AbicoderVersion {
    fn calculate_file_id(&self) -> Option<&FileId> {
        match self {
            AbicoderVersion::V1 => None,

            AbicoderVersion::V2 => None,
        }
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        match self {
            AbicoderVersion::V1 => None,

            AbicoderVersion::V2 => None,
        }
    }
}

impl NodeLocation for AdditiveExpressionOperator<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        match self {
            AdditiveExpressionOperator::Minus(inner) => inner.calculate_file_id(),

            AdditiveExpressionOperator::Plus(inner) => inner.calculate_file_id(),
        }
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        match self {
            AdditiveExpressionOperator::Minus(inner) => inner.calculate_text_range(),

            AdditiveExpressionOperator::Plus(inner) => inner.calculate_text_range(),
        }
    }
}

impl NodeLocation for ArgumentsDeclaration<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        match self {
            ArgumentsDeclaration::PositionalArguments(inner) => inner.calculate_file_id(),

            ArgumentsDeclaration::NamedArguments(inner) => inner.calculate_file_id(),
        }
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        match self {
            ArgumentsDeclaration::PositionalArguments(inner) => inner.calculate_text_range(),

            ArgumentsDeclaration::NamedArguments(inner) => inner.calculate_text_range(),
        }
    }
}

impl NodeLocation for AssignmentExpressionOperator<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        match self {
            AssignmentExpressionOperator::AmpersandEqual(inner) => inner.calculate_file_id(),

            AssignmentExpressionOperator::AsteriskEqual(inner) => inner.calculate_file_id(),

            AssignmentExpressionOperator::BarEqual(inner) => inner.calculate_file_id(),

            AssignmentExpressionOperator::CaretEqual(inner) => inner.calculate_file_id(),

            AssignmentExpressionOperator::Equal(inner) => inner.calculate_file_id(),

            AssignmentExpressionOperator::GreaterThanGreaterThanEqual(inner) => {
                inner.calculate_file_id()
            }

            AssignmentExpressionOperator::GreaterThanGreaterThanGreaterThanEqual(inner) => {
                inner.calculate_file_id()
            }

            AssignmentExpressionOperator::LessThanLessThanEqual(inner) => inner.calculate_file_id(),

            AssignmentExpressionOperator::MinusEqual(inner) => inner.calculate_file_id(),

            AssignmentExpressionOperator::PercentEqual(inner) => inner.calculate_file_id(),

            AssignmentExpressionOperator::PlusEqual(inner) => inner.calculate_file_id(),

            AssignmentExpressionOperator::SlashEqual(inner) => inner.calculate_file_id(),
        }
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        match self {
            AssignmentExpressionOperator::AmpersandEqual(inner) => inner.calculate_text_range(),

            AssignmentExpressionOperator::AsteriskEqual(inner) => inner.calculate_text_range(),

            AssignmentExpressionOperator::BarEqual(inner) => inner.calculate_text_range(),

            AssignmentExpressionOperator::CaretEqual(inner) => inner.calculate_text_range(),

            AssignmentExpressionOperator::Equal(inner) => inner.calculate_text_range(),

            AssignmentExpressionOperator::GreaterThanGreaterThanEqual(inner) => {
                inner.calculate_text_range()
            }

            AssignmentExpressionOperator::GreaterThanGreaterThanGreaterThanEqual(inner) => {
                inner.calculate_text_range()
            }

            AssignmentExpressionOperator::LessThanLessThanEqual(inner) => {
                inner.calculate_text_range()
            }

            AssignmentExpressionOperator::MinusEqual(inner) => inner.calculate_text_range(),

            AssignmentExpressionOperator::PercentEqual(inner) => inner.calculate_text_range(),

            AssignmentExpressionOperator::PlusEqual(inner) => inner.calculate_text_range(),

            AssignmentExpressionOperator::SlashEqual(inner) => inner.calculate_text_range(),
        }
    }
}

impl NodeLocation for CatchClauseKind {
    fn calculate_file_id(&self) -> Option<&FileId> {
        match self {
            CatchClauseKind::Error => None,

            CatchClauseKind::Panic => None,

            CatchClauseKind::LowLevel => None,
        }
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        match self {
            CatchClauseKind::Error => None,

            CatchClauseKind::Panic => None,

            CatchClauseKind::LowLevel => None,
        }
    }
}

impl NodeLocation for ContractMember<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        match self {
            ContractMember::UsingDirective(inner) => inner.calculate_file_id(),

            ContractMember::FunctionDefinition(inner) => inner.calculate_file_id(),

            ContractMember::ModifierDefinition(inner) => inner.calculate_file_id(),

            ContractMember::StructDefinition(inner) => inner.calculate_file_id(),

            ContractMember::EnumDefinition(inner) => inner.calculate_file_id(),

            ContractMember::EventDefinition(inner) => inner.calculate_file_id(),

            ContractMember::ErrorDefinition(inner) => inner.calculate_file_id(),

            ContractMember::UserDefinedValueTypeDefinition(inner) => inner.calculate_file_id(),

            ContractMember::StateVariableDefinition(inner) => inner.calculate_file_id(),

            ContractMember::ConstantDefinition(inner) => inner.calculate_file_id(),
        }
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        match self {
            ContractMember::UsingDirective(inner) => inner.calculate_text_range(),

            ContractMember::FunctionDefinition(inner) => inner.calculate_text_range(),

            ContractMember::ModifierDefinition(inner) => inner.calculate_text_range(),

            ContractMember::StructDefinition(inner) => inner.calculate_text_range(),

            ContractMember::EnumDefinition(inner) => inner.calculate_text_range(),

            ContractMember::EventDefinition(inner) => inner.calculate_text_range(),

            ContractMember::ErrorDefinition(inner) => inner.calculate_text_range(),

            ContractMember::UserDefinedValueTypeDefinition(inner) => inner.calculate_text_range(),

            ContractMember::StateVariableDefinition(inner) => inner.calculate_text_range(),

            ContractMember::ConstantDefinition(inner) => inner.calculate_text_range(),
        }
    }
}

impl NodeLocation for ElementaryType<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        match self {
            ElementaryType::BoolKeyword(inner) => inner.calculate_file_id(),

            ElementaryType::StringKeyword(inner) => inner.calculate_file_id(),

            ElementaryType::AddressType(inner) => inner.calculate_file_id(),

            ElementaryType::BytesKeyword(inner) => inner.calculate_file_id(),

            ElementaryType::IntKeyword(inner) => inner.calculate_file_id(),

            ElementaryType::UintKeyword(inner) => inner.calculate_file_id(),

            ElementaryType::FixedKeyword(inner) => inner.calculate_file_id(),

            ElementaryType::UfixedKeyword(inner) => inner.calculate_file_id(),
        }
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        match self {
            ElementaryType::BoolKeyword(inner) => inner.calculate_text_range(),

            ElementaryType::StringKeyword(inner) => inner.calculate_text_range(),

            ElementaryType::AddressType(inner) => inner.calculate_text_range(),

            ElementaryType::BytesKeyword(inner) => inner.calculate_text_range(),

            ElementaryType::IntKeyword(inner) => inner.calculate_text_range(),

            ElementaryType::UintKeyword(inner) => inner.calculate_text_range(),

            ElementaryType::FixedKeyword(inner) => inner.calculate_text_range(),

            ElementaryType::UfixedKeyword(inner) => inner.calculate_text_range(),
        }
    }
}

impl NodeLocation for EqualityExpressionOperator<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        match self {
            EqualityExpressionOperator::BangEqual(inner) => inner.calculate_file_id(),

            EqualityExpressionOperator::EqualEqual(inner) => inner.calculate_file_id(),
        }
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        match self {
            EqualityExpressionOperator::BangEqual(inner) => inner.calculate_text_range(),

            EqualityExpressionOperator::EqualEqual(inner) => inner.calculate_text_range(),
        }
    }
}

impl NodeLocation for ExperimentalFeature {
    fn calculate_file_id(&self) -> Option<&FileId> {
        match self {
            ExperimentalFeature::ABIEncoderV2 => None,

            ExperimentalFeature::SMTChecker => None,

            ExperimentalFeature::Solidity => None,

            ExperimentalFeature::Unrecognized => None,
        }
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        match self {
            ExperimentalFeature::ABIEncoderV2 => None,

            ExperimentalFeature::SMTChecker => None,

            ExperimentalFeature::Solidity => None,

            ExperimentalFeature::Unrecognized => None,
        }
    }
}

impl NodeLocation for Expression<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        match self {
            Expression::AssignmentExpression(inner) => inner.calculate_file_id(),

            Expression::ConditionalExpression(inner) => inner.calculate_file_id(),

            Expression::OrExpression(inner) => inner.calculate_file_id(),

            Expression::AndExpression(inner) => inner.calculate_file_id(),

            Expression::EqualityExpression(inner) => inner.calculate_file_id(),

            Expression::InequalityExpression(inner) => inner.calculate_file_id(),

            Expression::BitwiseOrExpression(inner) => inner.calculate_file_id(),

            Expression::BitwiseXorExpression(inner) => inner.calculate_file_id(),

            Expression::BitwiseAndExpression(inner) => inner.calculate_file_id(),

            Expression::ShiftExpression(inner) => inner.calculate_file_id(),

            Expression::AdditiveExpression(inner) => inner.calculate_file_id(),

            Expression::MultiplicativeExpression(inner) => inner.calculate_file_id(),

            Expression::ExponentiationExpression(inner) => inner.calculate_file_id(),

            Expression::PostfixExpression(inner) => inner.calculate_file_id(),

            Expression::PrefixExpression(inner) => inner.calculate_file_id(),

            Expression::FunctionCallExpression(inner) => inner.calculate_file_id(),

            Expression::CallOptionsExpression(inner) => inner.calculate_file_id(),

            Expression::MemberAccessExpression(inner) => inner.calculate_file_id(),

            Expression::IndexAccessExpression(inner) => inner.calculate_file_id(),

            Expression::NewExpression(inner) => inner.calculate_file_id(),

            Expression::TupleExpression(inner) => inner.calculate_file_id(),

            Expression::TypeExpression(inner) => inner.calculate_file_id(),

            Expression::ArrayExpression(inner) => inner.calculate_file_id(),

            Expression::HexNumberExpression(inner) => inner.calculate_file_id(),

            Expression::DecimalNumberExpression(inner) => inner.calculate_file_id(),

            Expression::StringExpression(inner) => inner.calculate_file_id(),

            Expression::ElementaryType(inner) => inner.calculate_file_id(),

            Expression::PayableKeyword(inner) => inner.calculate_file_id(),

            Expression::ThisKeyword(inner) => inner.calculate_file_id(),

            Expression::SuperKeyword(inner) => inner.calculate_file_id(),

            Expression::TrueKeyword(inner) => inner.calculate_file_id(),

            Expression::FalseKeyword(inner) => inner.calculate_file_id(),

            Expression::Identifier(inner) => inner.calculate_file_id(),
        }
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        match self {
            Expression::AssignmentExpression(inner) => inner.calculate_text_range(),

            Expression::ConditionalExpression(inner) => inner.calculate_text_range(),

            Expression::OrExpression(inner) => inner.calculate_text_range(),

            Expression::AndExpression(inner) => inner.calculate_text_range(),

            Expression::EqualityExpression(inner) => inner.calculate_text_range(),

            Expression::InequalityExpression(inner) => inner.calculate_text_range(),

            Expression::BitwiseOrExpression(inner) => inner.calculate_text_range(),

            Expression::BitwiseXorExpression(inner) => inner.calculate_text_range(),

            Expression::BitwiseAndExpression(inner) => inner.calculate_text_range(),

            Expression::ShiftExpression(inner) => inner.calculate_text_range(),

            Expression::AdditiveExpression(inner) => inner.calculate_text_range(),

            Expression::MultiplicativeExpression(inner) => inner.calculate_text_range(),

            Expression::ExponentiationExpression(inner) => inner.calculate_text_range(),

            Expression::PostfixExpression(inner) => inner.calculate_text_range(),

            Expression::PrefixExpression(inner) => inner.calculate_text_range(),

            Expression::FunctionCallExpression(inner) => inner.calculate_text_range(),

            Expression::CallOptionsExpression(inner) => inner.calculate_text_range(),

            Expression::MemberAccessExpression(inner) => inner.calculate_text_range(),

            Expression::IndexAccessExpression(inner) => inner.calculate_text_range(),

            Expression::NewExpression(inner) => inner.calculate_text_range(),

            Expression::TupleExpression(inner) => inner.calculate_text_range(),

            Expression::TypeExpression(inner) => inner.calculate_text_range(),

            Expression::ArrayExpression(inner) => inner.calculate_text_range(),

            Expression::HexNumberExpression(inner) => inner.calculate_text_range(),

            Expression::DecimalNumberExpression(inner) => inner.calculate_text_range(),

            Expression::StringExpression(inner) => inner.calculate_text_range(),

            Expression::ElementaryType(inner) => inner.calculate_text_range(),

            Expression::PayableKeyword(inner) => inner.calculate_text_range(),

            Expression::ThisKeyword(inner) => inner.calculate_text_range(),

            Expression::SuperKeyword(inner) => inner.calculate_text_range(),

            Expression::TrueKeyword(inner) => inner.calculate_text_range(),

            Expression::FalseKeyword(inner) => inner.calculate_text_range(),

            Expression::Identifier(inner) => inner.calculate_text_range(),
        }
    }
}

impl NodeLocation for ForStatementCondition<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        match self {
            ForStatementCondition::ExpressionStatement(inner) => inner.calculate_file_id(),

            ForStatementCondition::Semicolon(inner) => inner.calculate_file_id(),
        }
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        match self {
            ForStatementCondition::ExpressionStatement(inner) => inner.calculate_text_range(),

            ForStatementCondition::Semicolon(inner) => inner.calculate_text_range(),
        }
    }
}

impl NodeLocation for ForStatementInitialization<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        match self {
            ForStatementInitialization::VariableDeclarationStatement(inner) => {
                inner.calculate_file_id()
            }

            ForStatementInitialization::ExpressionStatement(inner) => inner.calculate_file_id(),

            ForStatementInitialization::Semicolon(inner) => inner.calculate_file_id(),
        }
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        match self {
            ForStatementInitialization::VariableDeclarationStatement(inner) => {
                inner.calculate_text_range()
            }

            ForStatementInitialization::ExpressionStatement(inner) => inner.calculate_text_range(),

            ForStatementInitialization::Semicolon(inner) => inner.calculate_text_range(),
        }
    }
}

impl NodeLocation for FunctionKind {
    fn calculate_file_id(&self) -> Option<&FileId> {
        match self {
            FunctionKind::Regular => None,

            FunctionKind::Constructor => None,

            FunctionKind::Fallback => None,

            FunctionKind::Receive => None,
        }
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        match self {
            FunctionKind::Regular => None,

            FunctionKind::Constructor => None,

            FunctionKind::Fallback => None,

            FunctionKind::Receive => None,
        }
    }
}

impl NodeLocation for FunctionMutability {
    fn calculate_file_id(&self) -> Option<&FileId> {
        match self {
            FunctionMutability::Pure => None,

            FunctionMutability::View => None,

            FunctionMutability::NonPayable => None,

            FunctionMutability::Payable => None,
        }
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        match self {
            FunctionMutability::Pure => None,

            FunctionMutability::View => None,

            FunctionMutability::NonPayable => None,

            FunctionMutability::Payable => None,
        }
    }
}

impl NodeLocation for FunctionVisibility {
    fn calculate_file_id(&self) -> Option<&FileId> {
        match self {
            FunctionVisibility::Public => None,

            FunctionVisibility::Private => None,

            FunctionVisibility::Internal => None,

            FunctionVisibility::External => None,
        }
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        match self {
            FunctionVisibility::Public => None,

            FunctionVisibility::Private => None,

            FunctionVisibility::Internal => None,

            FunctionVisibility::External => None,
        }
    }
}

impl NodeLocation for ImportClause<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        match self {
            ImportClause::PathImport(inner) => inner.calculate_file_id(),

            ImportClause::ImportDeconstruction(inner) => inner.calculate_file_id(),
        }
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        match self {
            ImportClause::PathImport(inner) => inner.calculate_text_range(),

            ImportClause::ImportDeconstruction(inner) => inner.calculate_text_range(),
        }
    }
}

impl NodeLocation for InequalityExpressionOperator<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        match self {
            InequalityExpressionOperator::GreaterThan(inner) => inner.calculate_file_id(),

            InequalityExpressionOperator::GreaterThanEqual(inner) => inner.calculate_file_id(),

            InequalityExpressionOperator::LessThan(inner) => inner.calculate_file_id(),

            InequalityExpressionOperator::LessThanEqual(inner) => inner.calculate_file_id(),
        }
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        match self {
            InequalityExpressionOperator::GreaterThan(inner) => inner.calculate_text_range(),

            InequalityExpressionOperator::GreaterThanEqual(inner) => inner.calculate_text_range(),

            InequalityExpressionOperator::LessThan(inner) => inner.calculate_text_range(),

            InequalityExpressionOperator::LessThanEqual(inner) => inner.calculate_text_range(),
        }
    }
}

impl NodeLocation for MultiplicativeExpressionOperator<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        match self {
            MultiplicativeExpressionOperator::Asterisk(inner) => inner.calculate_file_id(),

            MultiplicativeExpressionOperator::Percent(inner) => inner.calculate_file_id(),

            MultiplicativeExpressionOperator::Slash(inner) => inner.calculate_file_id(),
        }
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        match self {
            MultiplicativeExpressionOperator::Asterisk(inner) => inner.calculate_text_range(),

            MultiplicativeExpressionOperator::Percent(inner) => inner.calculate_text_range(),

            MultiplicativeExpressionOperator::Slash(inner) => inner.calculate_text_range(),
        }
    }
}

impl NodeLocation for NumberUnit<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        match self {
            NumberUnit::WeiKeyword(inner) => inner.calculate_file_id(),

            NumberUnit::GweiKeyword(inner) => inner.calculate_file_id(),

            NumberUnit::EtherKeyword(inner) => inner.calculate_file_id(),

            NumberUnit::SecondsKeyword(inner) => inner.calculate_file_id(),

            NumberUnit::MinutesKeyword(inner) => inner.calculate_file_id(),

            NumberUnit::HoursKeyword(inner) => inner.calculate_file_id(),

            NumberUnit::DaysKeyword(inner) => inner.calculate_file_id(),

            NumberUnit::WeeksKeyword(inner) => inner.calculate_file_id(),
        }
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        match self {
            NumberUnit::WeiKeyword(inner) => inner.calculate_text_range(),

            NumberUnit::GweiKeyword(inner) => inner.calculate_text_range(),

            NumberUnit::EtherKeyword(inner) => inner.calculate_text_range(),

            NumberUnit::SecondsKeyword(inner) => inner.calculate_text_range(),

            NumberUnit::MinutesKeyword(inner) => inner.calculate_text_range(),

            NumberUnit::HoursKeyword(inner) => inner.calculate_text_range(),

            NumberUnit::DaysKeyword(inner) => inner.calculate_text_range(),

            NumberUnit::WeeksKeyword(inner) => inner.calculate_text_range(),
        }
    }
}

impl NodeLocation for PostfixExpressionOperator<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        match self {
            PostfixExpressionOperator::MinusMinus(inner) => inner.calculate_file_id(),

            PostfixExpressionOperator::PlusPlus(inner) => inner.calculate_file_id(),
        }
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        match self {
            PostfixExpressionOperator::MinusMinus(inner) => inner.calculate_text_range(),

            PostfixExpressionOperator::PlusPlus(inner) => inner.calculate_text_range(),
        }
    }
}

impl NodeLocation for Pragma<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        match self {
            Pragma::VersionPragma(inner) => inner.calculate_file_id(),

            Pragma::AbicoderPragma(inner) => inner.calculate_file_id(),

            Pragma::ExperimentalPragma(inner) => inner.calculate_file_id(),
        }
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        match self {
            Pragma::VersionPragma(inner) => inner.calculate_text_range(),

            Pragma::AbicoderPragma(inner) => inner.calculate_text_range(),

            Pragma::ExperimentalPragma(inner) => inner.calculate_text_range(),
        }
    }
}

impl NodeLocation for PrefixExpressionOperator<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        match self {
            PrefixExpressionOperator::Bang(inner) => inner.calculate_file_id(),

            PrefixExpressionOperator::DeleteKeyword(inner) => inner.calculate_file_id(),

            PrefixExpressionOperator::Minus(inner) => inner.calculate_file_id(),

            PrefixExpressionOperator::MinusMinus(inner) => inner.calculate_file_id(),

            PrefixExpressionOperator::PlusPlus(inner) => inner.calculate_file_id(),

            PrefixExpressionOperator::Tilde(inner) => inner.calculate_file_id(),
        }
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        match self {
            PrefixExpressionOperator::Bang(inner) => inner.calculate_text_range(),

            PrefixExpressionOperator::DeleteKeyword(inner) => inner.calculate_text_range(),

            PrefixExpressionOperator::Minus(inner) => inner.calculate_text_range(),

            PrefixExpressionOperator::MinusMinus(inner) => inner.calculate_text_range(),

            PrefixExpressionOperator::PlusPlus(inner) => inner.calculate_text_range(),

            PrefixExpressionOperator::Tilde(inner) => inner.calculate_text_range(),
        }
    }
}

impl NodeLocation for ShiftExpressionOperator<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        match self {
            ShiftExpressionOperator::GreaterThanGreaterThan(inner) => inner.calculate_file_id(),

            ShiftExpressionOperator::GreaterThanGreaterThanGreaterThan(inner) => {
                inner.calculate_file_id()
            }

            ShiftExpressionOperator::LessThanLessThan(inner) => inner.calculate_file_id(),
        }
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        match self {
            ShiftExpressionOperator::GreaterThanGreaterThan(inner) => inner.calculate_text_range(),

            ShiftExpressionOperator::GreaterThanGreaterThanGreaterThan(inner) => {
                inner.calculate_text_range()
            }

            ShiftExpressionOperator::LessThanLessThan(inner) => inner.calculate_text_range(),
        }
    }
}

impl NodeLocation for SourceUnitMember<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        match self {
            SourceUnitMember::PragmaDirective(inner) => inner.calculate_file_id(),

            SourceUnitMember::ImportClause(inner) => inner.calculate_file_id(),

            SourceUnitMember::ContractDefinition(inner) => inner.calculate_file_id(),

            SourceUnitMember::InterfaceDefinition(inner) => inner.calculate_file_id(),

            SourceUnitMember::LibraryDefinition(inner) => inner.calculate_file_id(),

            SourceUnitMember::StructDefinition(inner) => inner.calculate_file_id(),

            SourceUnitMember::EnumDefinition(inner) => inner.calculate_file_id(),

            SourceUnitMember::FunctionDefinition(inner) => inner.calculate_file_id(),

            SourceUnitMember::ErrorDefinition(inner) => inner.calculate_file_id(),

            SourceUnitMember::UserDefinedValueTypeDefinition(inner) => inner.calculate_file_id(),

            SourceUnitMember::UsingDirective(inner) => inner.calculate_file_id(),

            SourceUnitMember::EventDefinition(inner) => inner.calculate_file_id(),

            SourceUnitMember::ConstantDefinition(inner) => inner.calculate_file_id(),
        }
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        match self {
            SourceUnitMember::PragmaDirective(inner) => inner.calculate_text_range(),

            SourceUnitMember::ImportClause(inner) => inner.calculate_text_range(),

            SourceUnitMember::ContractDefinition(inner) => inner.calculate_text_range(),

            SourceUnitMember::InterfaceDefinition(inner) => inner.calculate_text_range(),

            SourceUnitMember::LibraryDefinition(inner) => inner.calculate_text_range(),

            SourceUnitMember::StructDefinition(inner) => inner.calculate_text_range(),

            SourceUnitMember::EnumDefinition(inner) => inner.calculate_text_range(),

            SourceUnitMember::FunctionDefinition(inner) => inner.calculate_text_range(),

            SourceUnitMember::ErrorDefinition(inner) => inner.calculate_text_range(),

            SourceUnitMember::UserDefinedValueTypeDefinition(inner) => inner.calculate_text_range(),

            SourceUnitMember::UsingDirective(inner) => inner.calculate_text_range(),

            SourceUnitMember::EventDefinition(inner) => inner.calculate_text_range(),

            SourceUnitMember::ConstantDefinition(inner) => inner.calculate_text_range(),
        }
    }
}

impl NodeLocation for StateVariableMutability {
    fn calculate_file_id(&self) -> Option<&FileId> {
        match self {
            StateVariableMutability::Mutable => None,

            StateVariableMutability::Constant => None,

            StateVariableMutability::Immutable => None,

            StateVariableMutability::Transient => None,
        }
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        match self {
            StateVariableMutability::Mutable => None,

            StateVariableMutability::Constant => None,

            StateVariableMutability::Immutable => None,

            StateVariableMutability::Transient => None,
        }
    }
}

impl NodeLocation for StateVariableVisibility {
    fn calculate_file_id(&self) -> Option<&FileId> {
        match self {
            StateVariableVisibility::Public => None,

            StateVariableVisibility::Private => None,

            StateVariableVisibility::Internal => None,
        }
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        match self {
            StateVariableVisibility::Public => None,

            StateVariableVisibility::Private => None,

            StateVariableVisibility::Internal => None,
        }
    }
}

impl NodeLocation for Statement<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        match self {
            Statement::IfStatement(inner) => inner.calculate_file_id(),

            Statement::ForStatement(inner) => inner.calculate_file_id(),

            Statement::WhileStatement(inner) => inner.calculate_file_id(),

            Statement::DoWhileStatement(inner) => inner.calculate_file_id(),

            Statement::ContinueStatement(inner) => inner.calculate_file_id(),

            Statement::BreakStatement(inner) => inner.calculate_file_id(),

            Statement::ReturnStatement(inner) => inner.calculate_file_id(),

            Statement::EmitStatement(inner) => inner.calculate_file_id(),

            Statement::TryStatement(inner) => inner.calculate_file_id(),

            Statement::RevertStatement(inner) => inner.calculate_file_id(),

            Statement::AssemblyStatement(inner) => inner.calculate_file_id(),

            Statement::Block(inner) => inner.calculate_file_id(),

            Statement::UncheckedBlock(inner) => inner.calculate_file_id(),

            Statement::VariableDeclarationStatement(inner) => inner.calculate_file_id(),

            Statement::ExpressionStatement(inner) => inner.calculate_file_id(),
        }
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        match self {
            Statement::IfStatement(inner) => inner.calculate_text_range(),

            Statement::ForStatement(inner) => inner.calculate_text_range(),

            Statement::WhileStatement(inner) => inner.calculate_text_range(),

            Statement::DoWhileStatement(inner) => inner.calculate_text_range(),

            Statement::ContinueStatement(inner) => inner.calculate_text_range(),

            Statement::BreakStatement(inner) => inner.calculate_text_range(),

            Statement::ReturnStatement(inner) => inner.calculate_text_range(),

            Statement::EmitStatement(inner) => inner.calculate_text_range(),

            Statement::TryStatement(inner) => inner.calculate_text_range(),

            Statement::RevertStatement(inner) => inner.calculate_text_range(),

            Statement::AssemblyStatement(inner) => inner.calculate_text_range(),

            Statement::Block(inner) => inner.calculate_text_range(),

            Statement::UncheckedBlock(inner) => inner.calculate_text_range(),

            Statement::VariableDeclarationStatement(inner) => inner.calculate_text_range(),

            Statement::ExpressionStatement(inner) => inner.calculate_text_range(),
        }
    }
}

impl NodeLocation for StorageLocation<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        match self {
            StorageLocation::MemoryKeyword(inner) => inner.calculate_file_id(),

            StorageLocation::StorageKeyword(inner) => inner.calculate_file_id(),

            StorageLocation::CallDataKeyword(inner) => inner.calculate_file_id(),
        }
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        match self {
            StorageLocation::MemoryKeyword(inner) => inner.calculate_text_range(),

            StorageLocation::StorageKeyword(inner) => inner.calculate_text_range(),

            StorageLocation::CallDataKeyword(inner) => inner.calculate_text_range(),
        }
    }
}

impl NodeLocation for StringExpression<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        match self {
            StringExpression::StringLiterals(inner) => inner.calculate_file_id(),

            StringExpression::HexStringLiterals(inner) => inner.calculate_file_id(),

            StringExpression::UnicodeStringLiterals(inner) => inner.calculate_file_id(),
        }
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        match self {
            StringExpression::StringLiterals(inner) => inner.calculate_text_range(),

            StringExpression::HexStringLiterals(inner) => inner.calculate_text_range(),

            StringExpression::UnicodeStringLiterals(inner) => inner.calculate_text_range(),
        }
    }
}

impl NodeLocation for TypeName<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        match self {
            TypeName::ArrayTypeName(inner) => inner.calculate_file_id(),

            TypeName::FunctionType(inner) => inner.calculate_file_id(),

            TypeName::MappingType(inner) => inner.calculate_file_id(),

            TypeName::ElementaryType(inner) => inner.calculate_file_id(),

            TypeName::IdentifierPath(inner) => inner.calculate_file_id(),
        }
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        match self {
            TypeName::ArrayTypeName(inner) => inner.calculate_text_range(),

            TypeName::FunctionType(inner) => inner.calculate_text_range(),

            TypeName::MappingType(inner) => inner.calculate_text_range(),

            TypeName::ElementaryType(inner) => inner.calculate_text_range(),

            TypeName::IdentifierPath(inner) => inner.calculate_text_range(),
        }
    }
}

impl NodeLocation for UsingClause<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        match self {
            UsingClause::IdentifierPath(inner) => inner.calculate_file_id(),

            UsingClause::UsingDeconstruction(inner) => inner.calculate_file_id(),
        }
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        match self {
            UsingClause::IdentifierPath(inner) => inner.calculate_text_range(),

            UsingClause::UsingDeconstruction(inner) => inner.calculate_text_range(),
        }
    }
}

impl NodeLocation for UsingOperator<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        match self {
            UsingOperator::Ampersand(inner) => inner.calculate_file_id(),

            UsingOperator::Asterisk(inner) => inner.calculate_file_id(),

            UsingOperator::BangEqual(inner) => inner.calculate_file_id(),

            UsingOperator::Bar(inner) => inner.calculate_file_id(),

            UsingOperator::Caret(inner) => inner.calculate_file_id(),

            UsingOperator::EqualEqual(inner) => inner.calculate_file_id(),

            UsingOperator::GreaterThan(inner) => inner.calculate_file_id(),

            UsingOperator::GreaterThanEqual(inner) => inner.calculate_file_id(),

            UsingOperator::LessThan(inner) => inner.calculate_file_id(),

            UsingOperator::LessThanEqual(inner) => inner.calculate_file_id(),

            UsingOperator::Minus(inner) => inner.calculate_file_id(),

            UsingOperator::Percent(inner) => inner.calculate_file_id(),

            UsingOperator::Plus(inner) => inner.calculate_file_id(),

            UsingOperator::Slash(inner) => inner.calculate_file_id(),

            UsingOperator::Tilde(inner) => inner.calculate_file_id(),
        }
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        match self {
            UsingOperator::Ampersand(inner) => inner.calculate_text_range(),

            UsingOperator::Asterisk(inner) => inner.calculate_text_range(),

            UsingOperator::BangEqual(inner) => inner.calculate_text_range(),

            UsingOperator::Bar(inner) => inner.calculate_text_range(),

            UsingOperator::Caret(inner) => inner.calculate_text_range(),

            UsingOperator::EqualEqual(inner) => inner.calculate_text_range(),

            UsingOperator::GreaterThan(inner) => inner.calculate_text_range(),

            UsingOperator::GreaterThanEqual(inner) => inner.calculate_text_range(),

            UsingOperator::LessThan(inner) => inner.calculate_text_range(),

            UsingOperator::LessThanEqual(inner) => inner.calculate_text_range(),

            UsingOperator::Minus(inner) => inner.calculate_text_range(),

            UsingOperator::Percent(inner) => inner.calculate_text_range(),

            UsingOperator::Plus(inner) => inner.calculate_text_range(),

            UsingOperator::Slash(inner) => inner.calculate_text_range(),

            UsingOperator::Tilde(inner) => inner.calculate_text_range(),
        }
    }
}

impl NodeLocation for UsingTarget<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        match self {
            UsingTarget::TypeName(inner) => inner.calculate_file_id(),

            UsingTarget::Asterisk(inner) => inner.calculate_file_id(),
        }
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        match self {
            UsingTarget::TypeName(inner) => inner.calculate_text_range(),

            UsingTarget::Asterisk(inner) => inner.calculate_text_range(),
        }
    }
}

impl NodeLocation for VariableDeclarationTarget<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        match self {
            VariableDeclarationTarget::SingleTypedDeclaration(inner) => inner.calculate_file_id(),

            VariableDeclarationTarget::MultiTypedDeclaration(inner) => inner.calculate_file_id(),
        }
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        match self {
            VariableDeclarationTarget::SingleTypedDeclaration(inner) => {
                inner.calculate_text_range()
            }

            VariableDeclarationTarget::MultiTypedDeclaration(inner) => inner.calculate_text_range(),
        }
    }
}

impl NodeLocation for VersionPragmaComponent<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        match self {
            VersionPragmaComponent::Wildcard => None,

            VersionPragmaComponent::Unrecognized => None,

            VersionPragmaComponent::Number(_) => None,
        }
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        match self {
            VersionPragmaComponent::Wildcard => None,

            VersionPragmaComponent::Unrecognized => None,

            VersionPragmaComponent::Number(_) => None,
        }
    }
}

impl NodeLocation for VersionPragmaOperator {
    fn calculate_file_id(&self) -> Option<&FileId> {
        match self {
            VersionPragmaOperator::Caret => None,

            VersionPragmaOperator::Tilde => None,

            VersionPragmaOperator::Equal => None,

            VersionPragmaOperator::LessThan => None,

            VersionPragmaOperator::LessThanEqual => None,

            VersionPragmaOperator::GreaterThan => None,

            VersionPragmaOperator::GreaterThanEqual => None,
        }
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        match self {
            VersionPragmaOperator::Caret => None,

            VersionPragmaOperator::Tilde => None,

            VersionPragmaOperator::Equal => None,

            VersionPragmaOperator::LessThan => None,

            VersionPragmaOperator::LessThanEqual => None,

            VersionPragmaOperator::GreaterThan => None,

            VersionPragmaOperator::GreaterThanEqual => None,
        }
    }
}

impl NodeLocation for YulExpression<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        match self {
            YulExpression::YulFunctionCallExpression(inner) => inner.calculate_file_id(),

            YulExpression::YulLiteral(inner) => inner.calculate_file_id(),

            YulExpression::YulPath(inner) => inner.calculate_file_id(),
        }
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        match self {
            YulExpression::YulFunctionCallExpression(inner) => inner.calculate_text_range(),

            YulExpression::YulLiteral(inner) => inner.calculate_text_range(),

            YulExpression::YulPath(inner) => inner.calculate_text_range(),
        }
    }
}

impl NodeLocation for YulLiteral<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        match self {
            YulLiteral::TrueKeyword(inner) => inner.calculate_file_id(),

            YulLiteral::FalseKeyword(inner) => inner.calculate_file_id(),

            YulLiteral::DecimalLiteral(inner) => inner.calculate_file_id(),

            YulLiteral::HexLiteral(inner) => inner.calculate_file_id(),

            YulLiteral::HexStringLiteral(inner) => inner.calculate_file_id(),

            YulLiteral::StringLiteral(inner) => inner.calculate_file_id(),
        }
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        match self {
            YulLiteral::TrueKeyword(inner) => inner.calculate_text_range(),

            YulLiteral::FalseKeyword(inner) => inner.calculate_text_range(),

            YulLiteral::DecimalLiteral(inner) => inner.calculate_text_range(),

            YulLiteral::HexLiteral(inner) => inner.calculate_text_range(),

            YulLiteral::HexStringLiteral(inner) => inner.calculate_text_range(),

            YulLiteral::StringLiteral(inner) => inner.calculate_text_range(),
        }
    }
}

impl NodeLocation for YulStatement<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        match self {
            YulStatement::YulBlock(inner) => inner.calculate_file_id(),

            YulStatement::YulFunctionDefinition(inner) => inner.calculate_file_id(),

            YulStatement::YulIfStatement(inner) => inner.calculate_file_id(),

            YulStatement::YulForStatement(inner) => inner.calculate_file_id(),

            YulStatement::YulSwitchStatement(inner) => inner.calculate_file_id(),

            YulStatement::YulLeaveStatement(inner) => inner.calculate_file_id(),

            YulStatement::YulBreakStatement(inner) => inner.calculate_file_id(),

            YulStatement::YulContinueStatement(inner) => inner.calculate_file_id(),

            YulStatement::YulVariableAssignmentStatement(inner) => inner.calculate_file_id(),

            YulStatement::YulVariableDeclarationStatement(inner) => inner.calculate_file_id(),

            YulStatement::YulExpression(inner) => inner.calculate_file_id(),
        }
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        match self {
            YulStatement::YulBlock(inner) => inner.calculate_text_range(),

            YulStatement::YulFunctionDefinition(inner) => inner.calculate_text_range(),

            YulStatement::YulIfStatement(inner) => inner.calculate_text_range(),

            YulStatement::YulForStatement(inner) => inner.calculate_text_range(),

            YulStatement::YulSwitchStatement(inner) => inner.calculate_text_range(),

            YulStatement::YulLeaveStatement(inner) => inner.calculate_text_range(),

            YulStatement::YulBreakStatement(inner) => inner.calculate_text_range(),

            YulStatement::YulContinueStatement(inner) => inner.calculate_text_range(),

            YulStatement::YulVariableAssignmentStatement(inner) => inner.calculate_text_range(),

            YulStatement::YulVariableDeclarationStatement(inner) => inner.calculate_text_range(),

            YulStatement::YulExpression(inner) => inner.calculate_text_range(),
        }
    }
}

impl NodeLocation for ArrayValuesStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        self.ir_nodes
            .node_id()
            .map(|node_id| self.semantic.file_id_from_node_id(node_id))
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        self.ir_nodes.calculate_text_range()
    }
}

impl NodeLocation for CallOptionsStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        self.ir_nodes
            .node_id()
            .map(|node_id| self.semantic.file_id_from_node_id(node_id))
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        self.ir_nodes.calculate_text_range()
    }
}

impl NodeLocation for CatchClausesStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        self.ir_nodes
            .node_id()
            .map(|node_id| self.semantic.file_id_from_node_id(node_id))
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        self.ir_nodes.calculate_text_range()
    }
}

impl NodeLocation for ContractMembersStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        self.ir_nodes
            .node_id()
            .map(|node_id| self.semantic.file_id_from_node_id(node_id))
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        self.ir_nodes.calculate_text_range()
    }
}

impl NodeLocation for EnumMembersStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        self.ir_nodes
            .node_id()
            .map(|node_id| self.semantic.file_id_from_node_id(node_id))
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        self.ir_nodes.calculate_text_range()
    }
}

impl NodeLocation for HexStringLiteralsStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        self.ir_nodes
            .node_id()
            .map(|node_id| self.semantic.file_id_from_node_id(node_id))
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        self.ir_nodes.calculate_text_range()
    }
}

impl NodeLocation for IdentifierPathStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        self.ir_nodes
            .node_id()
            .map(|node_id| self.semantic.file_id_from_node_id(node_id))
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        self.ir_nodes.calculate_text_range()
    }
}

impl NodeLocation for ImportDeconstructionSymbolsStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        self.ir_nodes
            .node_id()
            .map(|node_id| self.semantic.file_id_from_node_id(node_id))
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        self.ir_nodes.calculate_text_range()
    }
}

impl NodeLocation for InheritanceTypesStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        self.ir_nodes
            .node_id()
            .map(|node_id| self.semantic.file_id_from_node_id(node_id))
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        self.ir_nodes.calculate_text_range()
    }
}

impl NodeLocation for InterfaceMembersStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        self.ir_nodes
            .node_id()
            .map(|node_id| self.semantic.file_id_from_node_id(node_id))
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        self.ir_nodes.calculate_text_range()
    }
}

impl NodeLocation for LibraryMembersStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        self.ir_nodes
            .node_id()
            .map(|node_id| self.semantic.file_id_from_node_id(node_id))
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        self.ir_nodes.calculate_text_range()
    }
}

impl NodeLocation for ModifierInvocationsStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        self.ir_nodes
            .node_id()
            .map(|node_id| self.semantic.file_id_from_node_id(node_id))
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        self.ir_nodes.calculate_text_range()
    }
}

impl NodeLocation for MultiTypedDeclarationElementsStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        self.ir_nodes
            .node_id()
            .map(|node_id| self.semantic.file_id_from_node_id(node_id))
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        self.ir_nodes.calculate_text_range()
    }
}

impl NodeLocation for NamedArgumentsStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        self.ir_nodes
            .node_id()
            .map(|node_id| self.semantic.file_id_from_node_id(node_id))
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        self.ir_nodes.calculate_text_range()
    }
}

impl NodeLocation for OverridePathsStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        self.ir_nodes
            .node_id()
            .map(|node_id| self.semantic.file_id_from_node_id(node_id))
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        self.ir_nodes.calculate_text_range()
    }
}

impl NodeLocation for ParametersStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        self.ir_nodes
            .node_id()
            .map(|node_id| self.semantic.file_id_from_node_id(node_id))
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        self.ir_nodes.calculate_text_range()
    }
}

impl NodeLocation for PositionalArgumentsStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        self.ir_nodes
            .node_id()
            .map(|node_id| self.semantic.file_id_from_node_id(node_id))
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        self.ir_nodes.calculate_text_range()
    }
}

impl NodeLocation for SourceUnitMembersStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        self.ir_nodes
            .node_id()
            .map(|node_id| self.semantic.file_id_from_node_id(node_id))
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        self.ir_nodes.calculate_text_range()
    }
}

impl NodeLocation for StatementsStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        self.ir_nodes
            .node_id()
            .map(|node_id| self.semantic.file_id_from_node_id(node_id))
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        self.ir_nodes.calculate_text_range()
    }
}

impl NodeLocation for StringLiteralsStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        self.ir_nodes
            .node_id()
            .map(|node_id| self.semantic.file_id_from_node_id(node_id))
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        self.ir_nodes.calculate_text_range()
    }
}

impl NodeLocation for StructMembersStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        self.ir_nodes
            .node_id()
            .map(|node_id| self.semantic.file_id_from_node_id(node_id))
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        self.ir_nodes.calculate_text_range()
    }
}

impl NodeLocation for TupleValuesStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        self.ir_nodes
            .node_id()
            .map(|node_id| self.semantic.file_id_from_node_id(node_id))
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        self.ir_nodes.calculate_text_range()
    }
}

impl NodeLocation for UnicodeStringLiteralsStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        self.ir_nodes
            .node_id()
            .map(|node_id| self.semantic.file_id_from_node_id(node_id))
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        self.ir_nodes.calculate_text_range()
    }
}

impl NodeLocation for UsingDeconstructionSymbolsStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        self.ir_nodes
            .node_id()
            .map(|node_id| self.semantic.file_id_from_node_id(node_id))
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        self.ir_nodes.calculate_text_range()
    }
}

impl NodeLocation for VersionPragmaExpressionSetStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        self.ir_nodes
            .node_id()
            .map(|node_id| self.semantic.file_id_from_node_id(node_id))
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        self.ir_nodes.calculate_text_range()
    }
}

impl NodeLocation for VersionPragmaExpressionSetsStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        self.ir_nodes
            .node_id()
            .map(|node_id| self.semantic.file_id_from_node_id(node_id))
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        self.ir_nodes.calculate_text_range()
    }
}

impl NodeLocation for VersionPragmaSpecifierStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        self.ir_nodes
            .node_id()
            .map(|node_id| self.semantic.file_id_from_node_id(node_id))
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        self.ir_nodes.calculate_text_range()
    }
}

impl NodeLocation for YulArgumentsStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        self.ir_nodes
            .node_id()
            .map(|node_id| self.semantic.file_id_from_node_id(node_id))
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        self.ir_nodes.calculate_text_range()
    }
}

impl NodeLocation for YulParametersStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        self.ir_nodes
            .node_id()
            .map(|node_id| self.semantic.file_id_from_node_id(node_id))
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        self.ir_nodes.calculate_text_range()
    }
}

impl NodeLocation for YulPathStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        self.ir_nodes
            .node_id()
            .map(|node_id| self.semantic.file_id_from_node_id(node_id))
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        self.ir_nodes.calculate_text_range()
    }
}

impl NodeLocation for YulPathsStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        self.ir_nodes
            .node_id()
            .map(|node_id| self.semantic.file_id_from_node_id(node_id))
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        self.ir_nodes.calculate_text_range()
    }
}

impl NodeLocation for YulStatementsStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        self.ir_nodes
            .node_id()
            .map(|node_id| self.semantic.file_id_from_node_id(node_id))
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        self.ir_nodes.calculate_text_range()
    }
}

impl NodeLocation for YulValueCasesStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        self.ir_nodes
            .node_id()
            .map(|node_id| self.semantic.file_id_from_node_id(node_id))
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        self.ir_nodes.calculate_text_range()
    }
}

impl NodeLocation for YulVariableNamesStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        self.ir_nodes
            .node_id()
            .map(|node_id| self.semantic.file_id_from_node_id(node_id))
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        self.ir_nodes.calculate_text_range()
    }
}

impl NodeLocation for AmpersandStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for AmpersandEqualStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for AsteriskStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for AsteriskEqualStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for BangStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for BangEqualStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for BarStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for BarEqualStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for BoolKeywordStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for BytesKeywordStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for CallDataKeywordStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for CaretStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for CaretEqualStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for DaysKeywordStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for DecimalLiteralStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for DeleteKeywordStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for EqualStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for EqualEqualStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for EtherKeywordStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for FalseKeywordStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for FixedKeywordStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for GreaterThanStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for GreaterThanEqualStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for GreaterThanGreaterThanStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for GreaterThanGreaterThanEqualStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for GreaterThanGreaterThanGreaterThanStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for GreaterThanGreaterThanGreaterThanEqualStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for GweiKeywordStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for HexLiteralStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for HexStringLiteralStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for HoursKeywordStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for IdentifierStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for IntKeywordStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for LessThanStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for LessThanEqualStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for LessThanLessThanStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for LessThanLessThanEqualStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for MemoryKeywordStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for MinusStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for MinusEqualStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for MinusMinusStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for MinutesKeywordStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for PayableKeywordStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for PercentStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for PercentEqualStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for PlusStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for PlusEqualStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for PlusPlusStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for SecondsKeywordStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for SemicolonStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for SlashStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for SlashEqualStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for StorageKeywordStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for StringKeywordStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for StringLiteralStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for SuperKeywordStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for ThisKeywordStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for TildeStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for TrueKeywordStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for UfixedKeywordStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for UintKeywordStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for UnicodeStringLiteralStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for WeeksKeywordStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}

impl NodeLocation for WeiKeywordStruct<'_> {
    fn calculate_file_id(&self) -> Option<&FileId> {
        Some(self.get_file_id())
    }

    fn calculate_text_range(&self) -> Option<Range<usize>> {
        Some(self.get_text_range().clone())
    }
}
