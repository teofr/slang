// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#![allow(non_camel_case_types)]

use std::ops::Range;

#[allow(clippy::wildcard_imports)]
use super::nodes::*;

fn merge(start: &mut usize, end: &mut usize, range: &Range<usize>) {
    if range.start < *start {
        *start = range.start;
    }
    if range.end > *end {
        *end = range.end;
    }
}

fn merge_opt(start: &mut usize, end: &mut usize, range: &Option<Range<usize>>) {
    if let Some(r) = range {
        merge(start, end, r);
    }
}

fn result(start: usize, end: usize) -> Option<Range<usize>> {
    if start <= end {
        Some(start..end)
    } else {
        None
    }
}

//
// Sequences:
//

impl AbicoderPragmaStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.abicoder_keyword.range);

        merge_opt(&mut start, &mut end, &self.version.range());
        result(start, end)
    }
}

impl AdditiveExpressionStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge_opt(&mut start, &mut end, &self.left_operand.range());

        merge_opt(
            &mut start,
            &mut end,
            &self.expression_additive_expression_operator.range(),
        );

        merge_opt(&mut start, &mut end, &self.right_operand.range());
        result(start, end)
    }
}

impl AddressTypeStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.address_keyword.range);
        if let Some(ref val) = self.payable_keyword {
            merge(&mut start, &mut end, &val.range);
        }
        result(start, end)
    }
}

impl AndExpressionStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge_opt(&mut start, &mut end, &self.left_operand.range());

        merge(&mut start, &mut end, &self.operator.range);

        merge_opt(&mut start, &mut end, &self.right_operand.range());
        result(start, end)
    }
}

impl ArrayExpressionStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.open_bracket.range);

        merge_opt(&mut start, &mut end, &self.items.range());

        merge(&mut start, &mut end, &self.close_bracket.range);
        result(start, end)
    }
}

impl ArrayTypeNameStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge_opt(&mut start, &mut end, &self.operand.range());

        merge(&mut start, &mut end, &self.open_bracket.range);
        if let Some(ref val) = self.index {
            merge_opt(&mut start, &mut end, &val.range());
        }

        merge(&mut start, &mut end, &self.close_bracket.range);
        result(start, end)
    }
}

impl AssemblyStatementStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.assembly_keyword.range);
        if let Some(ref val) = self.label {
            merge(&mut start, &mut end, &val.range);
        }
        if let Some(ref val) = self.flags {
            merge_opt(&mut start, &mut end, &val.range());
        }

        merge_opt(&mut start, &mut end, &self.body.range());
        result(start, end)
    }
}

impl AssignmentExpressionStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge_opt(&mut start, &mut end, &self.left_operand.range());

        merge_opt(
            &mut start,
            &mut end,
            &self.expression_assignment_expression_operator.range(),
        );

        merge_opt(&mut start, &mut end, &self.right_operand.range());
        result(start, end)
    }
}

impl BitwiseAndExpressionStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge_opt(&mut start, &mut end, &self.left_operand.range());

        merge(&mut start, &mut end, &self.operator.range);

        merge_opt(&mut start, &mut end, &self.right_operand.range());
        result(start, end)
    }
}

impl BitwiseOrExpressionStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge_opt(&mut start, &mut end, &self.left_operand.range());

        merge(&mut start, &mut end, &self.operator.range);

        merge_opt(&mut start, &mut end, &self.right_operand.range());
        result(start, end)
    }
}

impl BitwiseXorExpressionStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge_opt(&mut start, &mut end, &self.left_operand.range());

        merge(&mut start, &mut end, &self.operator.range);

        merge_opt(&mut start, &mut end, &self.right_operand.range());
        result(start, end)
    }
}

impl BlockStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.open_brace.range);

        merge_opt(&mut start, &mut end, &self.statements.range());

        merge(&mut start, &mut end, &self.close_brace.range);
        result(start, end)
    }
}

impl BreakStatementStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.break_keyword.range);

        merge(&mut start, &mut end, &self.semicolon.range);
        result(start, end)
    }
}

impl CallOptionsExpressionStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge_opt(&mut start, &mut end, &self.operand.range());

        merge(&mut start, &mut end, &self.open_brace.range);

        merge_opt(&mut start, &mut end, &self.options.range());

        merge(&mut start, &mut end, &self.close_brace.range);
        result(start, end)
    }
}

impl CatchClauseStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.catch_keyword.range);
        if let Some(ref val) = self.error {
            merge_opt(&mut start, &mut end, &val.range());
        }

        merge_opt(&mut start, &mut end, &self.body.range());
        result(start, end)
    }
}

impl CatchClauseErrorStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;
        if let Some(ref val) = self.name {
            merge(&mut start, &mut end, &val.range);
        }

        merge_opt(&mut start, &mut end, &self.parameters.range());
        result(start, end)
    }
}

impl ConditionalExpressionStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge_opt(&mut start, &mut end, &self.operand.range());

        merge(&mut start, &mut end, &self.question_mark.range);

        merge_opt(&mut start, &mut end, &self.true_expression.range());

        merge(&mut start, &mut end, &self.colon.range);

        merge_opt(&mut start, &mut end, &self.false_expression.range());
        result(start, end)
    }
}

impl ConstantDefinitionStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge_opt(&mut start, &mut end, &self.type_name.range());

        merge(&mut start, &mut end, &self.constant_keyword.range);

        merge(&mut start, &mut end, &self.name.range);

        merge(&mut start, &mut end, &self.equal.range);

        merge_opt(&mut start, &mut end, &self.value.range());

        merge(&mut start, &mut end, &self.semicolon.range);
        result(start, end)
    }
}

impl ConstructorDefinitionStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.constructor_keyword.range);

        merge_opt(&mut start, &mut end, &self.parameters.range());

        merge_opt(&mut start, &mut end, &self.attributes.range());

        merge_opt(&mut start, &mut end, &self.body.range());
        result(start, end)
    }
}

impl ContinueStatementStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.continue_keyword.range);

        merge(&mut start, &mut end, &self.semicolon.range);
        result(start, end)
    }
}

impl ContractDefinitionStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;
        if let Some(ref val) = self.abstract_keyword {
            merge(&mut start, &mut end, &val.range);
        }

        merge(&mut start, &mut end, &self.contract_keyword.range);

        merge(&mut start, &mut end, &self.name.range);

        merge_opt(&mut start, &mut end, &self.specifiers.range());

        merge(&mut start, &mut end, &self.open_brace.range);

        merge_opt(&mut start, &mut end, &self.members.range());

        merge(&mut start, &mut end, &self.close_brace.range);
        result(start, end)
    }
}

impl DecimalNumberExpressionStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.literal.range);
        if let Some(ref val) = self.unit {
            merge_opt(&mut start, &mut end, &val.range());
        }
        result(start, end)
    }
}

impl DoWhileStatementStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.do_keyword.range);

        merge_opt(&mut start, &mut end, &self.body.range());

        merge(&mut start, &mut end, &self.while_keyword.range);

        merge(&mut start, &mut end, &self.open_paren.range);

        merge_opt(&mut start, &mut end, &self.condition.range());

        merge(&mut start, &mut end, &self.close_paren.range);

        merge(&mut start, &mut end, &self.semicolon.range);
        result(start, end)
    }
}

impl ElseBranchStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.else_keyword.range);

        merge_opt(&mut start, &mut end, &self.body.range());
        result(start, end)
    }
}

impl EmitStatementStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.emit_keyword.range);

        merge_opt(&mut start, &mut end, &self.event.range());

        merge_opt(&mut start, &mut end, &self.arguments.range());

        merge(&mut start, &mut end, &self.semicolon.range);
        result(start, end)
    }
}

impl EnumDefinitionStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.enum_keyword.range);

        merge(&mut start, &mut end, &self.name.range);

        merge(&mut start, &mut end, &self.open_brace.range);

        merge_opt(&mut start, &mut end, &self.members.range());

        merge(&mut start, &mut end, &self.close_brace.range);
        result(start, end)
    }
}

impl EqualityExpressionStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge_opt(&mut start, &mut end, &self.left_operand.range());

        merge_opt(
            &mut start,
            &mut end,
            &self.expression_equality_expression_operator.range(),
        );

        merge_opt(&mut start, &mut end, &self.right_operand.range());
        result(start, end)
    }
}

impl ErrorDefinitionStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.error_keyword.range);

        merge(&mut start, &mut end, &self.name.range);

        merge_opt(&mut start, &mut end, &self.members.range());

        merge(&mut start, &mut end, &self.semicolon.range);
        result(start, end)
    }
}

impl ErrorParameterStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge_opt(&mut start, &mut end, &self.type_name.range());
        if let Some(ref val) = self.name {
            merge(&mut start, &mut end, &val.range);
        }
        result(start, end)
    }
}

impl ErrorParametersDeclarationStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.open_paren.range);

        merge_opt(&mut start, &mut end, &self.parameters.range());

        merge(&mut start, &mut end, &self.close_paren.range);
        result(start, end)
    }
}

impl EventDefinitionStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.event_keyword.range);

        merge(&mut start, &mut end, &self.name.range);

        merge_opt(&mut start, &mut end, &self.parameters.range());
        if let Some(ref val) = self.anonymous_keyword {
            merge(&mut start, &mut end, &val.range);
        }

        merge(&mut start, &mut end, &self.semicolon.range);
        result(start, end)
    }
}

impl EventParameterStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge_opt(&mut start, &mut end, &self.type_name.range());
        if let Some(ref val) = self.indexed_keyword {
            merge(&mut start, &mut end, &val.range);
        }
        if let Some(ref val) = self.name {
            merge(&mut start, &mut end, &val.range);
        }
        result(start, end)
    }
}

impl EventParametersDeclarationStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.open_paren.range);

        merge_opt(&mut start, &mut end, &self.parameters.range());

        merge(&mut start, &mut end, &self.close_paren.range);
        result(start, end)
    }
}

impl ExperimentalPragmaStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.experimental_keyword.range);

        merge_opt(&mut start, &mut end, &self.feature.range());
        result(start, end)
    }
}

impl ExponentiationExpressionStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge_opt(&mut start, &mut end, &self.left_operand.range());

        merge(&mut start, &mut end, &self.operator.range);

        merge_opt(&mut start, &mut end, &self.right_operand.range());
        result(start, end)
    }
}

impl ExpressionStatementStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge_opt(&mut start, &mut end, &self.expression.range());

        merge(&mut start, &mut end, &self.semicolon.range);
        result(start, end)
    }
}

impl FallbackFunctionDefinitionStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.fallback_keyword.range);

        merge_opt(&mut start, &mut end, &self.parameters.range());

        merge_opt(&mut start, &mut end, &self.attributes.range());
        if let Some(ref val) = self.returns {
            merge_opt(&mut start, &mut end, &val.range());
        }

        merge_opt(&mut start, &mut end, &self.body.range());
        result(start, end)
    }
}

impl ForStatementStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.for_keyword.range);

        merge(&mut start, &mut end, &self.open_paren.range);

        merge_opt(&mut start, &mut end, &self.initialization.range());

        merge_opt(&mut start, &mut end, &self.condition.range());
        if let Some(ref val) = self.iterator {
            merge_opt(&mut start, &mut end, &val.range());
        }

        merge(&mut start, &mut end, &self.close_paren.range);

        merge_opt(&mut start, &mut end, &self.body.range());
        result(start, end)
    }
}

impl FunctionCallExpressionStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge_opt(&mut start, &mut end, &self.operand.range());

        merge_opt(&mut start, &mut end, &self.arguments.range());
        result(start, end)
    }
}

impl FunctionDefinitionStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.function_keyword.range);

        merge_opt(&mut start, &mut end, &self.name.range());

        merge_opt(&mut start, &mut end, &self.parameters.range());

        merge_opt(&mut start, &mut end, &self.attributes.range());
        if let Some(ref val) = self.returns {
            merge_opt(&mut start, &mut end, &val.range());
        }

        merge_opt(&mut start, &mut end, &self.body.range());
        result(start, end)
    }
}

impl FunctionTypeStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.function_keyword.range);

        merge_opt(&mut start, &mut end, &self.parameters.range());

        merge_opt(&mut start, &mut end, &self.attributes.range());
        if let Some(ref val) = self.returns {
            merge_opt(&mut start, &mut end, &val.range());
        }
        result(start, end)
    }
}

impl HexNumberExpressionStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.literal.range);
        result(start, end)
    }
}

impl IfStatementStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.if_keyword.range);

        merge(&mut start, &mut end, &self.open_paren.range);

        merge_opt(&mut start, &mut end, &self.condition.range());

        merge(&mut start, &mut end, &self.close_paren.range);

        merge_opt(&mut start, &mut end, &self.body.range());
        if let Some(ref val) = self.else_branch {
            merge_opt(&mut start, &mut end, &val.range());
        }
        result(start, end)
    }
}

impl ImportAliasStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.as_keyword.range);

        merge(&mut start, &mut end, &self.identifier.range);
        result(start, end)
    }
}

impl ImportDeconstructionStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.open_brace.range);

        merge_opt(&mut start, &mut end, &self.symbols.range());

        merge(&mut start, &mut end, &self.close_brace.range);

        merge(&mut start, &mut end, &self.from_keyword.range);

        merge(&mut start, &mut end, &self.path.range);
        result(start, end)
    }
}

impl ImportDeconstructionSymbolStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.name.range);
        if let Some(ref val) = self.alias {
            merge_opt(&mut start, &mut end, &val.range());
        }
        result(start, end)
    }
}

impl ImportDirectiveStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.import_keyword.range);

        merge_opt(&mut start, &mut end, &self.clause.range());

        merge(&mut start, &mut end, &self.semicolon.range);
        result(start, end)
    }
}

impl IndexAccessEndStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.colon.range);
        if let Some(ref val) = self.end {
            merge_opt(&mut start, &mut end, &val.range());
        }
        result(start, end)
    }
}

impl IndexAccessExpressionStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge_opt(&mut start, &mut end, &self.operand.range());

        merge(&mut start, &mut end, &self.open_bracket.range);
        if let Some(ref val) = self.start {
            merge_opt(&mut start, &mut end, &val.range());
        }
        if let Some(ref val) = self.end {
            merge_opt(&mut start, &mut end, &val.range());
        }

        merge(&mut start, &mut end, &self.close_bracket.range);
        result(start, end)
    }
}

impl InequalityExpressionStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge_opt(&mut start, &mut end, &self.left_operand.range());

        merge_opt(
            &mut start,
            &mut end,
            &self.expression_inequality_expression_operator.range(),
        );

        merge_opt(&mut start, &mut end, &self.right_operand.range());
        result(start, end)
    }
}

impl InheritanceSpecifierStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.is_keyword.range);

        merge_opt(&mut start, &mut end, &self.types.range());
        result(start, end)
    }
}

impl InheritanceTypeStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge_opt(&mut start, &mut end, &self.type_name.range());
        if let Some(ref val) = self.arguments {
            merge_opt(&mut start, &mut end, &val.range());
        }
        result(start, end)
    }
}

impl InterfaceDefinitionStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.interface_keyword.range);

        merge(&mut start, &mut end, &self.name.range);
        if let Some(ref val) = self.inheritance {
            merge_opt(&mut start, &mut end, &val.range());
        }

        merge(&mut start, &mut end, &self.open_brace.range);

        merge_opt(&mut start, &mut end, &self.members.range());

        merge(&mut start, &mut end, &self.close_brace.range);
        result(start, end)
    }
}

impl LibraryDefinitionStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.library_keyword.range);

        merge(&mut start, &mut end, &self.name.range);

        merge(&mut start, &mut end, &self.open_brace.range);

        merge_opt(&mut start, &mut end, &self.members.range());

        merge(&mut start, &mut end, &self.close_brace.range);
        result(start, end)
    }
}

impl MappingKeyStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge_opt(&mut start, &mut end, &self.key_type.range());
        if let Some(ref val) = self.name {
            merge(&mut start, &mut end, &val.range);
        }
        result(start, end)
    }
}

impl MappingTypeStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.mapping_keyword.range);

        merge(&mut start, &mut end, &self.open_paren.range);

        merge_opt(&mut start, &mut end, &self.key_type.range());

        merge(&mut start, &mut end, &self.equal_greater_than.range);

        merge_opt(&mut start, &mut end, &self.value_type.range());

        merge(&mut start, &mut end, &self.close_paren.range);
        result(start, end)
    }
}

impl MappingValueStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge_opt(&mut start, &mut end, &self.type_name.range());
        if let Some(ref val) = self.name {
            merge(&mut start, &mut end, &val.range);
        }
        result(start, end)
    }
}

impl MemberAccessExpressionStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge_opt(&mut start, &mut end, &self.operand.range());

        merge(&mut start, &mut end, &self.period.range);

        merge_opt(&mut start, &mut end, &self.member.range());
        result(start, end)
    }
}

impl ModifierDefinitionStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.modifier_keyword.range);

        merge(&mut start, &mut end, &self.name.range);
        if let Some(ref val) = self.parameters {
            merge_opt(&mut start, &mut end, &val.range());
        }

        merge_opt(&mut start, &mut end, &self.attributes.range());

        merge_opt(&mut start, &mut end, &self.body.range());
        result(start, end)
    }
}

impl ModifierInvocationStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge_opt(&mut start, &mut end, &self.name.range());
        if let Some(ref val) = self.arguments {
            merge_opt(&mut start, &mut end, &val.range());
        }
        result(start, end)
    }
}

impl MultiTypedDeclarationStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.open_paren.range);

        merge_opt(&mut start, &mut end, &self.elements.range());

        merge(&mut start, &mut end, &self.close_paren.range);

        merge_opt(&mut start, &mut end, &self.value.range());
        result(start, end)
    }
}

impl MultiTypedDeclarationElementStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;
        if let Some(ref val) = self.member {
            merge_opt(&mut start, &mut end, &val.range());
        }
        result(start, end)
    }
}

impl MultiplicativeExpressionStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge_opt(&mut start, &mut end, &self.left_operand.range());

        merge_opt(
            &mut start,
            &mut end,
            &self.expression_multiplicative_expression_operator.range(),
        );

        merge_opt(&mut start, &mut end, &self.right_operand.range());
        result(start, end)
    }
}

impl NamedArgumentStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.name.range);

        merge(&mut start, &mut end, &self.colon.range);

        merge_opt(&mut start, &mut end, &self.value.range());
        result(start, end)
    }
}

impl NamedArgumentGroupStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.open_brace.range);

        merge_opt(&mut start, &mut end, &self.arguments.range());

        merge(&mut start, &mut end, &self.close_brace.range);
        result(start, end)
    }
}

impl NamedArgumentsDeclarationStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.open_paren.range);

        merge_opt(&mut start, &mut end, &self.arguments.range());

        merge(&mut start, &mut end, &self.close_paren.range);
        result(start, end)
    }
}

impl NamedImportStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.asterisk.range);

        merge_opt(&mut start, &mut end, &self.alias.range());

        merge(&mut start, &mut end, &self.from_keyword.range);

        merge(&mut start, &mut end, &self.path.range);
        result(start, end)
    }
}

impl NewExpressionStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.new_keyword.range);

        merge_opt(&mut start, &mut end, &self.type_name.range());
        result(start, end)
    }
}

impl OrExpressionStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge_opt(&mut start, &mut end, &self.left_operand.range());

        merge(&mut start, &mut end, &self.operator.range);

        merge_opt(&mut start, &mut end, &self.right_operand.range());
        result(start, end)
    }
}

impl OverridePathsDeclarationStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.open_paren.range);

        merge_opt(&mut start, &mut end, &self.paths.range());

        merge(&mut start, &mut end, &self.close_paren.range);
        result(start, end)
    }
}

impl OverrideSpecifierStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.override_keyword.range);
        if let Some(ref val) = self.overridden {
            merge_opt(&mut start, &mut end, &val.range());
        }
        result(start, end)
    }
}

impl ParameterStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge_opt(&mut start, &mut end, &self.type_name.range());
        if let Some(ref val) = self.storage_location {
            merge_opt(&mut start, &mut end, &val.range());
        }
        if let Some(ref val) = self.name {
            merge(&mut start, &mut end, &val.range);
        }
        result(start, end)
    }
}

impl ParametersDeclarationStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.open_paren.range);

        merge_opt(&mut start, &mut end, &self.parameters.range());

        merge(&mut start, &mut end, &self.close_paren.range);
        result(start, end)
    }
}

impl PathImportStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.path.range);
        if let Some(ref val) = self.alias {
            merge_opt(&mut start, &mut end, &val.range());
        }
        result(start, end)
    }
}

impl PositionalArgumentsDeclarationStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.open_paren.range);

        merge_opt(&mut start, &mut end, &self.arguments.range());

        merge(&mut start, &mut end, &self.close_paren.range);
        result(start, end)
    }
}

impl PostfixExpressionStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge_opt(&mut start, &mut end, &self.operand.range());

        merge_opt(
            &mut start,
            &mut end,
            &self.expression_postfix_expression_operator.range(),
        );
        result(start, end)
    }
}

impl PragmaDirectiveStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.pragma_keyword.range);

        merge_opt(&mut start, &mut end, &self.pragma.range());

        merge(&mut start, &mut end, &self.semicolon.range);
        result(start, end)
    }
}

impl PrefixExpressionStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge_opt(
            &mut start,
            &mut end,
            &self.expression_prefix_expression_operator.range(),
        );

        merge_opt(&mut start, &mut end, &self.operand.range());
        result(start, end)
    }
}

impl ReceiveFunctionDefinitionStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.receive_keyword.range);

        merge_opt(&mut start, &mut end, &self.parameters.range());

        merge_opt(&mut start, &mut end, &self.attributes.range());

        merge_opt(&mut start, &mut end, &self.body.range());
        result(start, end)
    }
}

impl ReturnStatementStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.return_keyword.range);
        if let Some(ref val) = self.expression {
            merge_opt(&mut start, &mut end, &val.range());
        }

        merge(&mut start, &mut end, &self.semicolon.range);
        result(start, end)
    }
}

impl ReturnsDeclarationStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.returns_keyword.range);

        merge_opt(&mut start, &mut end, &self.variables.range());
        result(start, end)
    }
}

impl RevertStatementStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.revert_keyword.range);

        merge_opt(&mut start, &mut end, &self.error.range());

        merge_opt(&mut start, &mut end, &self.arguments.range());

        merge(&mut start, &mut end, &self.semicolon.range);
        result(start, end)
    }
}

impl ShiftExpressionStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge_opt(&mut start, &mut end, &self.left_operand.range());

        merge_opt(
            &mut start,
            &mut end,
            &self.expression_shift_expression_operator.range(),
        );

        merge_opt(&mut start, &mut end, &self.right_operand.range());
        result(start, end)
    }
}

impl SingleTypedDeclarationStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge_opt(&mut start, &mut end, &self.declaration.range());
        if let Some(ref val) = self.value {
            merge_opt(&mut start, &mut end, &val.range());
        }
        result(start, end)
    }
}

impl SourceUnitStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge_opt(&mut start, &mut end, &self.members.range());
        result(start, end)
    }
}

impl StateVariableDefinitionStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge_opt(&mut start, &mut end, &self.type_name.range());

        merge_opt(&mut start, &mut end, &self.attributes.range());

        merge(&mut start, &mut end, &self.name.range);
        if let Some(ref val) = self.value {
            merge_opt(&mut start, &mut end, &val.range());
        }

        merge(&mut start, &mut end, &self.semicolon.range);
        result(start, end)
    }
}

impl StateVariableDefinitionValueStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.equal.range);

        merge_opt(&mut start, &mut end, &self.value.range());
        result(start, end)
    }
}

impl StorageLayoutSpecifierStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.layout_keyword.range);

        merge(&mut start, &mut end, &self.at_keyword.range);

        merge_opt(&mut start, &mut end, &self.expression.range());
        result(start, end)
    }
}

impl StructDefinitionStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.struct_keyword.range);

        merge(&mut start, &mut end, &self.name.range);

        merge(&mut start, &mut end, &self.open_brace.range);

        merge_opt(&mut start, &mut end, &self.members.range());

        merge(&mut start, &mut end, &self.close_brace.range);
        result(start, end)
    }
}

impl StructMemberStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge_opt(&mut start, &mut end, &self.type_name.range());

        merge(&mut start, &mut end, &self.name.range);

        merge(&mut start, &mut end, &self.semicolon.range);
        result(start, end)
    }
}

impl TryStatementStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.try_keyword.range);

        merge_opt(&mut start, &mut end, &self.expression.range());
        if let Some(ref val) = self.returns {
            merge_opt(&mut start, &mut end, &val.range());
        }

        merge_opt(&mut start, &mut end, &self.body.range());

        merge_opt(&mut start, &mut end, &self.catch_clauses.range());
        result(start, end)
    }
}

impl TupleExpressionStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.open_paren.range);

        merge_opt(&mut start, &mut end, &self.items.range());

        merge(&mut start, &mut end, &self.close_paren.range);
        result(start, end)
    }
}

impl TupleValueStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;
        if let Some(ref val) = self.expression {
            merge_opt(&mut start, &mut end, &val.range());
        }
        result(start, end)
    }
}

impl TypeExpressionStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.type_keyword.range);

        merge(&mut start, &mut end, &self.open_paren.range);

        merge_opt(&mut start, &mut end, &self.type_name.range());

        merge(&mut start, &mut end, &self.close_paren.range);
        result(start, end)
    }
}

impl UncheckedBlockStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.unchecked_keyword.range);

        merge_opt(&mut start, &mut end, &self.block.range());
        result(start, end)
    }
}

impl UserDefinedValueTypeDefinitionStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.type_keyword.range);

        merge(&mut start, &mut end, &self.name.range);

        merge(&mut start, &mut end, &self.is_keyword.range);

        merge_opt(&mut start, &mut end, &self.value_type.range());

        merge(&mut start, &mut end, &self.semicolon.range);
        result(start, end)
    }
}

impl UsingAliasStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.as_keyword.range);

        merge_opt(&mut start, &mut end, &self.operator.range());
        result(start, end)
    }
}

impl UsingDeconstructionStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.open_brace.range);

        merge_opt(&mut start, &mut end, &self.symbols.range());

        merge(&mut start, &mut end, &self.close_brace.range);
        result(start, end)
    }
}

impl UsingDeconstructionSymbolStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge_opt(&mut start, &mut end, &self.name.range());
        if let Some(ref val) = self.alias {
            merge_opt(&mut start, &mut end, &val.range());
        }
        result(start, end)
    }
}

impl UsingDirectiveStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.using_keyword.range);

        merge_opt(&mut start, &mut end, &self.clause.range());

        merge(&mut start, &mut end, &self.for_keyword.range);

        merge_opt(&mut start, &mut end, &self.target.range());
        if let Some(ref val) = self.global_keyword {
            merge(&mut start, &mut end, &val.range);
        }

        merge(&mut start, &mut end, &self.semicolon.range);
        result(start, end)
    }
}

impl VariableDeclarationStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge_opt(&mut start, &mut end, &self.type_name.range());
        if let Some(ref val) = self.storage_location {
            merge_opt(&mut start, &mut end, &val.range());
        }

        merge(&mut start, &mut end, &self.name.range);
        result(start, end)
    }
}

impl VariableDeclarationStatementStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge_opt(&mut start, &mut end, &self.target.range());

        merge(&mut start, &mut end, &self.semicolon.range);
        result(start, end)
    }
}

impl VariableDeclarationValueStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.equal.range);

        merge_opt(&mut start, &mut end, &self.expression.range());
        result(start, end)
    }
}

impl VersionPragmaStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.solidity_keyword.range);

        merge_opt(&mut start, &mut end, &self.sets.range());
        result(start, end)
    }
}

impl VersionRangeStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge_opt(&mut start, &mut end, &self.start.range());

        merge(&mut start, &mut end, &self.minus.range);

        merge_opt(&mut start, &mut end, &self.end.range());
        result(start, end)
    }
}

impl VersionTermStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;
        if let Some(ref val) = self.operator {
            merge_opt(&mut start, &mut end, &val.range());
        }

        merge_opt(&mut start, &mut end, &self.literal.range());
        result(start, end)
    }
}

impl WhileStatementStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.while_keyword.range);

        merge(&mut start, &mut end, &self.open_paren.range);

        merge_opt(&mut start, &mut end, &self.condition.range());

        merge(&mut start, &mut end, &self.close_paren.range);

        merge_opt(&mut start, &mut end, &self.body.range());
        result(start, end)
    }
}

impl YulBlockStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.open_brace.range);

        merge_opt(&mut start, &mut end, &self.statements.range());

        merge(&mut start, &mut end, &self.close_brace.range);
        result(start, end)
    }
}

impl YulBreakStatementStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.break_keyword.range);
        result(start, end)
    }
}

impl YulContinueStatementStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.continue_keyword.range);
        result(start, end)
    }
}

impl YulDefaultCaseStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.default_keyword.range);

        merge_opt(&mut start, &mut end, &self.body.range());
        result(start, end)
    }
}

impl YulFlagsDeclarationStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.open_paren.range);

        merge_opt(&mut start, &mut end, &self.flags.range());

        merge(&mut start, &mut end, &self.close_paren.range);
        result(start, end)
    }
}

impl YulForStatementStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.for_keyword.range);

        merge_opt(&mut start, &mut end, &self.initialization.range());

        merge_opt(&mut start, &mut end, &self.condition.range());

        merge_opt(&mut start, &mut end, &self.iterator.range());

        merge_opt(&mut start, &mut end, &self.body.range());
        result(start, end)
    }
}

impl YulFunctionCallExpressionStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge_opt(&mut start, &mut end, &self.operand.range());

        merge(&mut start, &mut end, &self.open_paren.range);

        merge_opt(&mut start, &mut end, &self.arguments.range());

        merge(&mut start, &mut end, &self.close_paren.range);
        result(start, end)
    }
}

impl YulFunctionDefinitionStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.function_keyword.range);

        merge(&mut start, &mut end, &self.name.range);

        merge_opt(&mut start, &mut end, &self.parameters.range());
        if let Some(ref val) = self.returns {
            merge_opt(&mut start, &mut end, &val.range());
        }

        merge_opt(&mut start, &mut end, &self.body.range());
        result(start, end)
    }
}

impl YulIfStatementStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.if_keyword.range);

        merge_opt(&mut start, &mut end, &self.condition.range());

        merge_opt(&mut start, &mut end, &self.body.range());
        result(start, end)
    }
}

impl YulLeaveStatementStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.leave_keyword.range);
        result(start, end)
    }
}

impl YulParametersDeclarationStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.open_paren.range);

        merge_opt(&mut start, &mut end, &self.parameters.range());

        merge(&mut start, &mut end, &self.close_paren.range);
        result(start, end)
    }
}

impl YulReturnsDeclarationStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.minus_greater_than.range);

        merge_opt(&mut start, &mut end, &self.variables.range());
        result(start, end)
    }
}

impl YulSwitchStatementStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.switch_keyword.range);

        merge_opt(&mut start, &mut end, &self.expression.range());

        merge_opt(&mut start, &mut end, &self.cases.range());
        result(start, end)
    }
}

impl YulValueCaseStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.case_keyword.range);

        merge_opt(&mut start, &mut end, &self.value.range());

        merge_opt(&mut start, &mut end, &self.body.range());
        result(start, end)
    }
}

impl YulVariableAssignmentStatementStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge_opt(&mut start, &mut end, &self.variables.range());

        merge(&mut start, &mut end, &self.assignment.range);

        merge_opt(&mut start, &mut end, &self.expression.range());
        result(start, end)
    }
}

impl YulVariableDeclarationStatementStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.let_keyword.range);

        merge_opt(&mut start, &mut end, &self.variables.range());
        if let Some(ref val) = self.value {
            merge_opt(&mut start, &mut end, &val.range());
        }
        result(start, end)
    }
}

impl YulVariableDeclarationValueStruct {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;

        merge(&mut start, &mut end, &self.assignment.range);

        merge_opt(&mut start, &mut end, &self.expression.range());
        result(start, end)
    }
}

//
// Choices:
//

impl AbicoderVersion {
    pub fn range(&self) -> Option<Range<usize>> {
        match self {
            Self::AbicoderV1Keyword(inner) => Some(inner.range.clone()),
            Self::AbicoderV2Keyword(inner) => Some(inner.range.clone()),
        }
    }
}

impl ArgumentsDeclaration {
    pub fn range(&self) -> Option<Range<usize>> {
        match self {
            Self::PositionalArgumentsDeclaration(inner) => inner.range(),
            Self::NamedArgumentsDeclaration(inner) => inner.range(),
        }
    }
}

impl ConstructorAttribute {
    pub fn range(&self) -> Option<Range<usize>> {
        match self {
            Self::ModifierInvocation(inner) => inner.range(),
            Self::InternalKeyword(inner) => Some(inner.range.clone()),
            Self::PayableKeyword(inner) => Some(inner.range.clone()),
            Self::PublicKeyword(inner) => Some(inner.range.clone()),
        }
    }
}

impl ContractMember {
    pub fn range(&self) -> Option<Range<usize>> {
        match self {
            Self::UsingDirective(inner) => inner.range(),
            Self::FunctionDefinition(inner) => inner.range(),
            Self::ConstructorDefinition(inner) => inner.range(),
            Self::ReceiveFunctionDefinition(inner) => inner.range(),
            Self::FallbackFunctionDefinition(inner) => inner.range(),
            Self::ModifierDefinition(inner) => inner.range(),
            Self::StructDefinition(inner) => inner.range(),
            Self::EnumDefinition(inner) => inner.range(),
            Self::EventDefinition(inner) => inner.range(),
            Self::ErrorDefinition(inner) => inner.range(),
            Self::UserDefinedValueTypeDefinition(inner) => inner.range(),
            Self::StateVariableDefinition(inner) => inner.range(),
        }
    }
}

impl ContractSpecifier {
    pub fn range(&self) -> Option<Range<usize>> {
        match self {
            Self::InheritanceSpecifier(inner) => inner.range(),
            Self::StorageLayoutSpecifier(inner) => inner.range(),
        }
    }
}

impl ElementaryType {
    pub fn range(&self) -> Option<Range<usize>> {
        match self {
            Self::BoolKeyword(inner) => Some(inner.range.clone()),
            Self::StringKeyword(inner) => Some(inner.range.clone()),
            Self::AddressType(inner) => inner.range(),
            Self::BytesKeyword(inner) => Some(inner.range.clone()),
            Self::IntKeyword(inner) => Some(inner.range.clone()),
            Self::UintKeyword(inner) => Some(inner.range.clone()),
            Self::FixedKeyword(inner) => Some(inner.range.clone()),
            Self::UfixedKeyword(inner) => Some(inner.range.clone()),
        }
    }
}

impl ExperimentalFeature {
    pub fn range(&self) -> Option<Range<usize>> {
        match self {
            Self::ABIEncoderV2Keyword(inner) => Some(inner.range.clone()),
            Self::SMTCheckerKeyword(inner) => Some(inner.range.clone()),
            Self::PragmaStringLiteral(inner) => Some(inner.range.clone()),
        }
    }
}

impl Expression {
    pub fn range(&self) -> Option<Range<usize>> {
        match self {
            Self::AssignmentExpression(inner) => inner.range(),
            Self::ConditionalExpression(inner) => inner.range(),
            Self::OrExpression(inner) => inner.range(),
            Self::AndExpression(inner) => inner.range(),
            Self::EqualityExpression(inner) => inner.range(),
            Self::InequalityExpression(inner) => inner.range(),
            Self::BitwiseOrExpression(inner) => inner.range(),
            Self::BitwiseXorExpression(inner) => inner.range(),
            Self::BitwiseAndExpression(inner) => inner.range(),
            Self::ShiftExpression(inner) => inner.range(),
            Self::AdditiveExpression(inner) => inner.range(),
            Self::MultiplicativeExpression(inner) => inner.range(),
            Self::ExponentiationExpression(inner) => inner.range(),
            Self::PostfixExpression(inner) => inner.range(),
            Self::PrefixExpression(inner) => inner.range(),
            Self::FunctionCallExpression(inner) => inner.range(),
            Self::CallOptionsExpression(inner) => inner.range(),
            Self::MemberAccessExpression(inner) => inner.range(),
            Self::IndexAccessExpression(inner) => inner.range(),
            Self::NewExpression(inner) => inner.range(),
            Self::TupleExpression(inner) => inner.range(),
            Self::TypeExpression(inner) => inner.range(),
            Self::ArrayExpression(inner) => inner.range(),
            Self::HexNumberExpression(inner) => inner.range(),
            Self::DecimalNumberExpression(inner) => inner.range(),
            Self::StringExpression(inner) => inner.range(),
            Self::ElementaryType(inner) => inner.range(),
            Self::PayableKeyword(inner) => Some(inner.range.clone()),
            Self::ThisKeyword(inner) => Some(inner.range.clone()),
            Self::SuperKeyword(inner) => Some(inner.range.clone()),
            Self::TrueKeyword(inner) => Some(inner.range.clone()),
            Self::FalseKeyword(inner) => Some(inner.range.clone()),
            Self::Identifier(inner) => Some(inner.range.clone()),
        }
    }
}

impl Expression_AdditiveExpression_Operator {
    pub fn range(&self) -> Option<Range<usize>> {
        match self {
            Self::Minus(inner) => Some(inner.range.clone()),
            Self::Plus(inner) => Some(inner.range.clone()),
        }
    }
}

impl Expression_AssignmentExpression_Operator {
    pub fn range(&self) -> Option<Range<usize>> {
        match self {
            Self::AmpersandEqual(inner) => Some(inner.range.clone()),
            Self::AsteriskEqual(inner) => Some(inner.range.clone()),
            Self::BarEqual(inner) => Some(inner.range.clone()),
            Self::CaretEqual(inner) => Some(inner.range.clone()),
            Self::Equal(inner) => Some(inner.range.clone()),
            Self::GreaterThanGreaterThanEqual(inner) => Some(inner.range.clone()),
            Self::GreaterThanGreaterThanGreaterThanEqual(inner) => Some(inner.range.clone()),
            Self::LessThanLessThanEqual(inner) => Some(inner.range.clone()),
            Self::MinusEqual(inner) => Some(inner.range.clone()),
            Self::PercentEqual(inner) => Some(inner.range.clone()),
            Self::PlusEqual(inner) => Some(inner.range.clone()),
            Self::SlashEqual(inner) => Some(inner.range.clone()),
        }
    }
}

impl Expression_EqualityExpression_Operator {
    pub fn range(&self) -> Option<Range<usize>> {
        match self {
            Self::BangEqual(inner) => Some(inner.range.clone()),
            Self::EqualEqual(inner) => Some(inner.range.clone()),
        }
    }
}

impl Expression_InequalityExpression_Operator {
    pub fn range(&self) -> Option<Range<usize>> {
        match self {
            Self::GreaterThan(inner) => Some(inner.range.clone()),
            Self::GreaterThanEqual(inner) => Some(inner.range.clone()),
            Self::LessThan(inner) => Some(inner.range.clone()),
            Self::LessThanEqual(inner) => Some(inner.range.clone()),
        }
    }
}

impl Expression_MultiplicativeExpression_Operator {
    pub fn range(&self) -> Option<Range<usize>> {
        match self {
            Self::Asterisk(inner) => Some(inner.range.clone()),
            Self::Percent(inner) => Some(inner.range.clone()),
            Self::Slash(inner) => Some(inner.range.clone()),
        }
    }
}

impl Expression_PostfixExpression_Operator {
    pub fn range(&self) -> Option<Range<usize>> {
        match self {
            Self::MinusMinus(inner) => Some(inner.range.clone()),
            Self::PlusPlus(inner) => Some(inner.range.clone()),
        }
    }
}

impl Expression_PrefixExpression_Operator {
    pub fn range(&self) -> Option<Range<usize>> {
        match self {
            Self::Bang(inner) => Some(inner.range.clone()),
            Self::DeleteKeyword(inner) => Some(inner.range.clone()),
            Self::Minus(inner) => Some(inner.range.clone()),
            Self::MinusMinus(inner) => Some(inner.range.clone()),
            Self::PlusPlus(inner) => Some(inner.range.clone()),
            Self::Tilde(inner) => Some(inner.range.clone()),
        }
    }
}

impl Expression_ShiftExpression_Operator {
    pub fn range(&self) -> Option<Range<usize>> {
        match self {
            Self::GreaterThanGreaterThan(inner) => Some(inner.range.clone()),
            Self::GreaterThanGreaterThanGreaterThan(inner) => Some(inner.range.clone()),
            Self::LessThanLessThan(inner) => Some(inner.range.clone()),
        }
    }
}

impl FallbackFunctionAttribute {
    pub fn range(&self) -> Option<Range<usize>> {
        match self {
            Self::ModifierInvocation(inner) => inner.range(),
            Self::OverrideSpecifier(inner) => inner.range(),
            Self::ExternalKeyword(inner) => Some(inner.range.clone()),
            Self::PayableKeyword(inner) => Some(inner.range.clone()),
            Self::PureKeyword(inner) => Some(inner.range.clone()),
            Self::ViewKeyword(inner) => Some(inner.range.clone()),
            Self::VirtualKeyword(inner) => Some(inner.range.clone()),
        }
    }
}

impl ForStatementCondition {
    pub fn range(&self) -> Option<Range<usize>> {
        match self {
            Self::ExpressionStatement(inner) => inner.range(),
            Self::Semicolon(inner) => Some(inner.range.clone()),
        }
    }
}

impl ForStatementInitialization {
    pub fn range(&self) -> Option<Range<usize>> {
        match self {
            Self::VariableDeclarationStatement(inner) => inner.range(),
            Self::ExpressionStatement(inner) => inner.range(),
            Self::Semicolon(inner) => Some(inner.range.clone()),
        }
    }
}

impl FunctionAttribute {
    pub fn range(&self) -> Option<Range<usize>> {
        match self {
            Self::ModifierInvocation(inner) => inner.range(),
            Self::OverrideSpecifier(inner) => inner.range(),
            Self::ExternalKeyword(inner) => Some(inner.range.clone()),
            Self::InternalKeyword(inner) => Some(inner.range.clone()),
            Self::PayableKeyword(inner) => Some(inner.range.clone()),
            Self::PrivateKeyword(inner) => Some(inner.range.clone()),
            Self::PublicKeyword(inner) => Some(inner.range.clone()),
            Self::PureKeyword(inner) => Some(inner.range.clone()),
            Self::ViewKeyword(inner) => Some(inner.range.clone()),
            Self::VirtualKeyword(inner) => Some(inner.range.clone()),
        }
    }
}

impl FunctionBody {
    pub fn range(&self) -> Option<Range<usize>> {
        match self {
            Self::Block(inner) => inner.range(),
            Self::Semicolon(inner) => Some(inner.range.clone()),
        }
    }
}

impl FunctionName {
    pub fn range(&self) -> Option<Range<usize>> {
        match self {
            Self::Identifier(inner) => Some(inner.range.clone()),
            Self::FallbackKeyword(inner) => Some(inner.range.clone()),
            Self::ReceiveKeyword(inner) => Some(inner.range.clone()),
        }
    }
}

impl FunctionTypeAttribute {
    pub fn range(&self) -> Option<Range<usize>> {
        match self {
            Self::InternalKeyword(inner) => Some(inner.range.clone()),
            Self::ExternalKeyword(inner) => Some(inner.range.clone()),
            Self::PrivateKeyword(inner) => Some(inner.range.clone()),
            Self::PublicKeyword(inner) => Some(inner.range.clone()),
            Self::PureKeyword(inner) => Some(inner.range.clone()),
            Self::ViewKeyword(inner) => Some(inner.range.clone()),
            Self::PayableKeyword(inner) => Some(inner.range.clone()),
        }
    }
}

impl IdentifierPathElement {
    pub fn range(&self) -> Option<Range<usize>> {
        match self {
            Self::Identifier(inner) => Some(inner.range.clone()),
            Self::AddressKeyword(inner) => Some(inner.range.clone()),
        }
    }
}

impl ImportClause {
    pub fn range(&self) -> Option<Range<usize>> {
        match self {
            Self::PathImport(inner) => inner.range(),
            Self::NamedImport(inner) => inner.range(),
            Self::ImportDeconstruction(inner) => inner.range(),
        }
    }
}

impl MappingKeyType {
    pub fn range(&self) -> Option<Range<usize>> {
        match self {
            Self::ElementaryType(inner) => inner.range(),
            Self::IdentifierPath(inner) => inner.range(),
        }
    }
}

impl ModifierAttribute {
    pub fn range(&self) -> Option<Range<usize>> {
        match self {
            Self::OverrideSpecifier(inner) => inner.range(),
            Self::VirtualKeyword(inner) => Some(inner.range.clone()),
        }
    }
}

impl NumberUnit {
    pub fn range(&self) -> Option<Range<usize>> {
        match self {
            Self::WeiKeyword(inner) => Some(inner.range.clone()),
            Self::GweiKeyword(inner) => Some(inner.range.clone()),
            Self::EtherKeyword(inner) => Some(inner.range.clone()),
            Self::SecondsKeyword(inner) => Some(inner.range.clone()),
            Self::MinutesKeyword(inner) => Some(inner.range.clone()),
            Self::HoursKeyword(inner) => Some(inner.range.clone()),
            Self::DaysKeyword(inner) => Some(inner.range.clone()),
            Self::WeeksKeyword(inner) => Some(inner.range.clone()),
        }
    }
}

impl Pragma {
    pub fn range(&self) -> Option<Range<usize>> {
        match self {
            Self::VersionPragma(inner) => inner.range(),
            Self::AbicoderPragma(inner) => inner.range(),
            Self::ExperimentalPragma(inner) => inner.range(),
        }
    }
}

impl ReceiveFunctionAttribute {
    pub fn range(&self) -> Option<Range<usize>> {
        match self {
            Self::ModifierInvocation(inner) => inner.range(),
            Self::OverrideSpecifier(inner) => inner.range(),
            Self::ExternalKeyword(inner) => Some(inner.range.clone()),
            Self::PayableKeyword(inner) => Some(inner.range.clone()),
            Self::VirtualKeyword(inner) => Some(inner.range.clone()),
        }
    }
}

impl SourceUnitMember {
    pub fn range(&self) -> Option<Range<usize>> {
        match self {
            Self::PragmaDirective(inner) => inner.range(),
            Self::ImportDirective(inner) => inner.range(),
            Self::ContractDefinition(inner) => inner.range(),
            Self::InterfaceDefinition(inner) => inner.range(),
            Self::LibraryDefinition(inner) => inner.range(),
            Self::StructDefinition(inner) => inner.range(),
            Self::EnumDefinition(inner) => inner.range(),
            Self::FunctionDefinition(inner) => inner.range(),
            Self::ErrorDefinition(inner) => inner.range(),
            Self::UserDefinedValueTypeDefinition(inner) => inner.range(),
            Self::UsingDirective(inner) => inner.range(),
            Self::EventDefinition(inner) => inner.range(),
            Self::ConstantDefinition(inner) => inner.range(),
        }
    }
}

impl StateVariableAttribute {
    pub fn range(&self) -> Option<Range<usize>> {
        match self {
            Self::OverrideSpecifier(inner) => inner.range(),
            Self::ConstantKeyword(inner) => Some(inner.range.clone()),
            Self::InternalKeyword(inner) => Some(inner.range.clone()),
            Self::PrivateKeyword(inner) => Some(inner.range.clone()),
            Self::PublicKeyword(inner) => Some(inner.range.clone()),
            Self::ImmutableKeyword(inner) => Some(inner.range.clone()),
            Self::TransientKeyword(inner) => Some(inner.range.clone()),
        }
    }
}

impl Statement {
    pub fn range(&self) -> Option<Range<usize>> {
        match self {
            Self::IfStatement(inner) => inner.range(),
            Self::ForStatement(inner) => inner.range(),
            Self::WhileStatement(inner) => inner.range(),
            Self::DoWhileStatement(inner) => inner.range(),
            Self::ContinueStatement(inner) => inner.range(),
            Self::BreakStatement(inner) => inner.range(),
            Self::ReturnStatement(inner) => inner.range(),
            Self::EmitStatement(inner) => inner.range(),
            Self::TryStatement(inner) => inner.range(),
            Self::RevertStatement(inner) => inner.range(),
            Self::AssemblyStatement(inner) => inner.range(),
            Self::Block(inner) => inner.range(),
            Self::UncheckedBlock(inner) => inner.range(),
            Self::VariableDeclarationStatement(inner) => inner.range(),
            Self::ExpressionStatement(inner) => inner.range(),
        }
    }
}

impl StorageLocation {
    pub fn range(&self) -> Option<Range<usize>> {
        match self {
            Self::MemoryKeyword(inner) => Some(inner.range.clone()),
            Self::StorageKeyword(inner) => Some(inner.range.clone()),
            Self::CallDataKeyword(inner) => Some(inner.range.clone()),
        }
    }
}

impl StringExpression {
    pub fn range(&self) -> Option<Range<usize>> {
        match self {
            Self::StringLiterals(inner) => inner.range(),
            Self::HexStringLiterals(inner) => inner.range(),
            Self::UnicodeStringLiterals(inner) => inner.range(),
        }
    }
}

impl TypeName {
    pub fn range(&self) -> Option<Range<usize>> {
        match self {
            Self::ArrayTypeName(inner) => inner.range(),
            Self::FunctionType(inner) => inner.range(),
            Self::MappingType(inner) => inner.range(),
            Self::ElementaryType(inner) => inner.range(),
            Self::IdentifierPath(inner) => inner.range(),
        }
    }
}

impl UsingClause {
    pub fn range(&self) -> Option<Range<usize>> {
        match self {
            Self::IdentifierPath(inner) => inner.range(),
            Self::UsingDeconstruction(inner) => inner.range(),
        }
    }
}

impl UsingOperator {
    pub fn range(&self) -> Option<Range<usize>> {
        match self {
            Self::Ampersand(inner) => Some(inner.range.clone()),
            Self::Asterisk(inner) => Some(inner.range.clone()),
            Self::BangEqual(inner) => Some(inner.range.clone()),
            Self::Bar(inner) => Some(inner.range.clone()),
            Self::Caret(inner) => Some(inner.range.clone()),
            Self::EqualEqual(inner) => Some(inner.range.clone()),
            Self::GreaterThan(inner) => Some(inner.range.clone()),
            Self::GreaterThanEqual(inner) => Some(inner.range.clone()),
            Self::LessThan(inner) => Some(inner.range.clone()),
            Self::LessThanEqual(inner) => Some(inner.range.clone()),
            Self::Minus(inner) => Some(inner.range.clone()),
            Self::Percent(inner) => Some(inner.range.clone()),
            Self::Plus(inner) => Some(inner.range.clone()),
            Self::Slash(inner) => Some(inner.range.clone()),
            Self::Tilde(inner) => Some(inner.range.clone()),
        }
    }
}

impl UsingTarget {
    pub fn range(&self) -> Option<Range<usize>> {
        match self {
            Self::TypeName(inner) => inner.range(),
            Self::Asterisk(inner) => Some(inner.range.clone()),
        }
    }
}

impl VariableDeclarationTarget {
    pub fn range(&self) -> Option<Range<usize>> {
        match self {
            Self::SingleTypedDeclaration(inner) => inner.range(),
            Self::MultiTypedDeclaration(inner) => inner.range(),
        }
    }
}

impl VersionExpression {
    pub fn range(&self) -> Option<Range<usize>> {
        match self {
            Self::VersionRange(inner) => inner.range(),
            Self::VersionTerm(inner) => inner.range(),
        }
    }
}

impl VersionLiteral {
    pub fn range(&self) -> Option<Range<usize>> {
        match self {
            Self::SimpleVersionLiteral(inner) => inner.range(),
            Self::PragmaStringLiteral(inner) => Some(inner.range.clone()),
        }
    }
}

impl VersionOperator {
    pub fn range(&self) -> Option<Range<usize>> {
        match self {
            Self::PragmaCaret(inner) => Some(inner.range.clone()),
            Self::PragmaTilde(inner) => Some(inner.range.clone()),
            Self::PragmaEqual(inner) => Some(inner.range.clone()),
            Self::PragmaLessThan(inner) => Some(inner.range.clone()),
            Self::PragmaGreaterThan(inner) => Some(inner.range.clone()),
            Self::PragmaLessThanEqual(inner) => Some(inner.range.clone()),
            Self::PragmaGreaterThanEqual(inner) => Some(inner.range.clone()),
        }
    }
}

impl YulExpression {
    pub fn range(&self) -> Option<Range<usize>> {
        match self {
            Self::YulFunctionCallExpression(inner) => inner.range(),
            Self::YulLiteral(inner) => inner.range(),
            Self::YulPath(inner) => inner.range(),
        }
    }
}

impl YulLiteral {
    pub fn range(&self) -> Option<Range<usize>> {
        match self {
            Self::YulTrueKeyword(inner) => Some(inner.range.clone()),
            Self::YulFalseKeyword(inner) => Some(inner.range.clone()),
            Self::YulDecimalLiteral(inner) => Some(inner.range.clone()),
            Self::YulHexLiteral(inner) => Some(inner.range.clone()),
            Self::YulHexStringLiteral(inner) => Some(inner.range.clone()),
            Self::YulStringLiteral(inner) => Some(inner.range.clone()),
        }
    }
}

impl YulStatement {
    pub fn range(&self) -> Option<Range<usize>> {
        match self {
            Self::YulBlock(inner) => inner.range(),
            Self::YulFunctionDefinition(inner) => inner.range(),
            Self::YulIfStatement(inner) => inner.range(),
            Self::YulForStatement(inner) => inner.range(),
            Self::YulSwitchStatement(inner) => inner.range(),
            Self::YulLeaveStatement(inner) => inner.range(),
            Self::YulBreakStatement(inner) => inner.range(),
            Self::YulContinueStatement(inner) => inner.range(),
            Self::YulVariableAssignmentStatement(inner) => inner.range(),
            Self::YulVariableDeclarationStatement(inner) => inner.range(),
            Self::YulExpression(inner) => inner.range(),
        }
    }
}

impl YulSwitchCase {
    pub fn range(&self) -> Option<Range<usize>> {
        match self {
            Self::YulDefaultCase(inner) => inner.range(),
            Self::YulValueCase(inner) => inner.range(),
        }
    }
}

//
// Collections:
//

impl ArrayValues {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;
        for element in &self.elements {
            merge_opt(&mut start, &mut end, &element.range());
        }
        result(start, end)
    }
}

impl CallOptions {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;
        for element in &self.elements {
            merge_opt(&mut start, &mut end, &element.range());
        }
        result(start, end)
    }
}

impl CatchClauses {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;
        for element in &self.elements {
            merge_opt(&mut start, &mut end, &element.range());
        }
        result(start, end)
    }
}

impl ConstructorAttributes {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;
        for element in &self.elements {
            merge_opt(&mut start, &mut end, &element.range());
        }
        result(start, end)
    }
}

impl ContractMembers {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;
        for element in &self.elements {
            merge_opt(&mut start, &mut end, &element.range());
        }
        result(start, end)
    }
}

impl ContractSpecifiers {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;
        for element in &self.elements {
            merge_opt(&mut start, &mut end, &element.range());
        }
        result(start, end)
    }
}

impl EnumMembers {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;
        for element in &self.elements {
            merge(&mut start, &mut end, &element.range);
        }
        result(start, end)
    }
}

impl ErrorParameters {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;
        for element in &self.elements {
            merge_opt(&mut start, &mut end, &element.range());
        }
        result(start, end)
    }
}

impl EventParameters {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;
        for element in &self.elements {
            merge_opt(&mut start, &mut end, &element.range());
        }
        result(start, end)
    }
}

impl FallbackFunctionAttributes {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;
        for element in &self.elements {
            merge_opt(&mut start, &mut end, &element.range());
        }
        result(start, end)
    }
}

impl FunctionAttributes {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;
        for element in &self.elements {
            merge_opt(&mut start, &mut end, &element.range());
        }
        result(start, end)
    }
}

impl FunctionTypeAttributes {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;
        for element in &self.elements {
            merge_opt(&mut start, &mut end, &element.range());
        }
        result(start, end)
    }
}

impl HexStringLiterals {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;
        for element in &self.elements {
            merge(&mut start, &mut end, &element.range);
        }
        result(start, end)
    }
}

impl IdentifierPath {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;
        for element in &self.elements {
            merge_opt(&mut start, &mut end, &element.range());
        }
        result(start, end)
    }
}

impl ImportDeconstructionSymbols {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;
        for element in &self.elements {
            merge_opt(&mut start, &mut end, &element.range());
        }
        result(start, end)
    }
}

impl InheritanceTypes {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;
        for element in &self.elements {
            merge_opt(&mut start, &mut end, &element.range());
        }
        result(start, end)
    }
}

impl InterfaceMembers {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;
        for element in &self.elements {
            merge_opt(&mut start, &mut end, &element.range());
        }
        result(start, end)
    }
}

impl LibraryMembers {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;
        for element in &self.elements {
            merge_opt(&mut start, &mut end, &element.range());
        }
        result(start, end)
    }
}

impl ModifierAttributes {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;
        for element in &self.elements {
            merge_opt(&mut start, &mut end, &element.range());
        }
        result(start, end)
    }
}

impl MultiTypedDeclarationElements {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;
        for element in &self.elements {
            merge_opt(&mut start, &mut end, &element.range());
        }
        result(start, end)
    }
}

impl NamedArguments {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;
        for element in &self.elements {
            merge_opt(&mut start, &mut end, &element.range());
        }
        result(start, end)
    }
}

impl OverridePaths {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;
        for element in &self.elements {
            merge_opt(&mut start, &mut end, &element.range());
        }
        result(start, end)
    }
}

impl Parameters {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;
        for element in &self.elements {
            merge_opt(&mut start, &mut end, &element.range());
        }
        result(start, end)
    }
}

impl PositionalArguments {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;
        for element in &self.elements {
            merge_opt(&mut start, &mut end, &element.range());
        }
        result(start, end)
    }
}

impl ReceiveFunctionAttributes {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;
        for element in &self.elements {
            merge_opt(&mut start, &mut end, &element.range());
        }
        result(start, end)
    }
}

impl SimpleVersionLiteral {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;
        for element in &self.elements {
            merge(&mut start, &mut end, &element.range);
        }
        result(start, end)
    }
}

impl SourceUnitMembers {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;
        for element in &self.elements {
            merge_opt(&mut start, &mut end, &element.range());
        }
        result(start, end)
    }
}

impl StateVariableAttributes {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;
        for element in &self.elements {
            merge_opt(&mut start, &mut end, &element.range());
        }
        result(start, end)
    }
}

impl Statements {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;
        for element in &self.elements {
            merge_opt(&mut start, &mut end, &element.range());
        }
        result(start, end)
    }
}

impl StringLiterals {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;
        for element in &self.elements {
            merge(&mut start, &mut end, &element.range);
        }
        result(start, end)
    }
}

impl StructMembers {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;
        for element in &self.elements {
            merge_opt(&mut start, &mut end, &element.range());
        }
        result(start, end)
    }
}

impl TupleValues {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;
        for element in &self.elements {
            merge_opt(&mut start, &mut end, &element.range());
        }
        result(start, end)
    }
}

impl UnicodeStringLiterals {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;
        for element in &self.elements {
            merge(&mut start, &mut end, &element.range);
        }
        result(start, end)
    }
}

impl UsingDeconstructionSymbols {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;
        for element in &self.elements {
            merge_opt(&mut start, &mut end, &element.range());
        }
        result(start, end)
    }
}

impl VersionExpressionSet {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;
        for element in &self.elements {
            merge_opt(&mut start, &mut end, &element.range());
        }
        result(start, end)
    }
}

impl VersionExpressionSets {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;
        for element in &self.elements {
            merge_opt(&mut start, &mut end, &element.range());
        }
        result(start, end)
    }
}

impl YulArguments {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;
        for element in &self.elements {
            merge_opt(&mut start, &mut end, &element.range());
        }
        result(start, end)
    }
}

impl YulFlags {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;
        for element in &self.elements {
            merge(&mut start, &mut end, &element.range);
        }
        result(start, end)
    }
}

impl YulParameters {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;
        for element in &self.elements {
            merge(&mut start, &mut end, &element.range);
        }
        result(start, end)
    }
}

impl YulPath {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;
        for element in &self.elements {
            merge(&mut start, &mut end, &element.range);
        }
        result(start, end)
    }
}

impl YulPaths {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;
        for element in &self.elements {
            merge_opt(&mut start, &mut end, &element.range());
        }
        result(start, end)
    }
}

impl YulStatements {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;
        for element in &self.elements {
            merge_opt(&mut start, &mut end, &element.range());
        }
        result(start, end)
    }
}

impl YulSwitchCases {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;
        for element in &self.elements {
            merge_opt(&mut start, &mut end, &element.range());
        }
        result(start, end)
    }
}

impl YulVariableNames {
    pub fn range(&self) -> Option<Range<usize>> {
        let mut start = usize::MAX;
        let mut end: usize = 0;
        for element in &self.elements {
            merge(&mut start, &mut end, &element.range);
        }
        result(start, end)
    }
}

//
// Terminals:
//

impl ABIEncoderV2Keyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl AbicoderKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl AbicoderV1Keyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl AbicoderV2Keyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl AbstractKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl AddressKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl AfterKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl AliasKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl Ampersand {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl AmpersandAmpersand {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl AmpersandEqual {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl AnonymousKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl ApplyKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl AsKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl AssemblyKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl Asterisk {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl AsteriskAsterisk {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl AsteriskEqual {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl AtKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl AutoKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl Bang {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl BangEqual {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl Bar {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl BarBar {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl BarEqual {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl BoolKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl BreakKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl ByteKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl BytesKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl CallDataKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl Caret {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl CaretEqual {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl CaseKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl CatchKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl CloseBrace {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl CloseBracket {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl CloseParen {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl Colon {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl Comma {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl ConstantKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl ConstructorKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl ContinueKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl ContractKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl CopyOfKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl DaysKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl DecimalLiteral {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl DefaultKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl DefineKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl DeleteKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl DoKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl ElseKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl EmitKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl EndOfLine {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl EnumKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl Equal {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl EqualEqual {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl EqualGreaterThan {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl ErrorKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl EtherKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl EventKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl ExperimentalKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl ExternalKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl FallbackKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl FalseKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl FinalKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl FixedKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl ForKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl FromKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl FunctionKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl GlobalKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl GreaterThan {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl GreaterThanEqual {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl GreaterThanGreaterThan {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl GreaterThanGreaterThanEqual {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl GreaterThanGreaterThanGreaterThan {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl GreaterThanGreaterThanGreaterThanEqual {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl GweiKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl HexKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl HexLiteral {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl HexStringLiteral {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl HoursKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl Identifier {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl IfKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl ImmutableKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl ImplementsKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl ImportKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl InKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl IndexedKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl InlineKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl IntKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl InterfaceKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl InternalKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl IsKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl LayoutKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl LessThan {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl LessThanEqual {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl LessThanLessThan {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl LessThanLessThanEqual {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl LetKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl LibraryKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl MacroKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl MappingKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl MatchKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl MemoryKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl Minus {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl MinusEqual {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl MinusMinus {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl MinutesKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl ModifierKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl MultiLineComment {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl MultiLineNatSpecComment {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl MutableKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl NewKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl NullKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl OfKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl OpenBrace {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl OpenBracket {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl OpenParen {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl OverrideKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl PartialKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl PayableKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl Percent {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl PercentEqual {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl Period {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl Plus {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl PlusEqual {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl PlusPlus {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl PragmaBarBar {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl PragmaCaret {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl PragmaEqual {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl PragmaGreaterThan {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl PragmaGreaterThanEqual {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl PragmaKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl PragmaLessThan {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl PragmaLessThanEqual {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl PragmaMinus {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl PragmaPeriod {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl PragmaSemicolon {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl PragmaStringLiteral {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl PragmaTilde {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl PrivateKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl PromiseKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl PublicKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl PureKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl QuestionMark {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl ReceiveKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl ReferenceKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl RelocatableKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl ReturnKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl ReturnsKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl RevertKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl SMTCheckerKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl SealedKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl SecondsKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl Semicolon {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl SingleLineComment {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl SingleLineNatSpecComment {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl SizeOfKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl Slash {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl SlashEqual {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl SolidityKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl StaticKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl StorageKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl StringKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl StringLiteral {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl StructKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl SuperKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl SupportsKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl SwitchKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl ThisKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl ThrowKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl Tilde {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TransientKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TrueKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TryKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TypeDefKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TypeKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl TypeOfKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl UfixedKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl UintKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl UncheckedKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl UnicodeStringLiteral {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl UsingKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl VarKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl VersionSpecifier {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl ViewKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl VirtualKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl WeeksKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl WeiKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl WhileKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl Whitespace {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl YearsKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl YulBreakKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl YulCaseKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl YulCloseBrace {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl YulCloseParen {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl YulColonEqual {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl YulComma {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl YulContinueKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl YulDecimalLiteral {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl YulDefaultKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl YulFalseKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl YulForKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl YulFunctionKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl YulHexKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl YulHexLiteral {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl YulHexStringLiteral {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl YulIdentifier {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl YulIfKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl YulLeaveKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl YulLetKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl YulMinusGreaterThan {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl YulOpenBrace {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl YulOpenParen {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl YulPeriod {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl YulStringLiteral {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl YulSuperKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl YulSwitchKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl YulThisKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}

impl YulTrueKeyword {
    pub fn range(&self) -> Option<Range<usize>> {
        Some(self.range.clone())
    }
}
