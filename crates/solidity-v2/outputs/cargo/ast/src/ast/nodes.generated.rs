// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#![allow(unused)]
use std::ops::Range;
use std::sync::Arc;

use slang_solidity_v2_common::files::FileId;
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;
use slang_solidity_v2_semantic::context::SemanticContext;

use super::types::Type;

//
// Sequences
//

pub type AbicoderPragma<'a> = AbicoderPragmaStruct<'a>;

#[derive(Clone, Copy)]
pub struct AbicoderPragmaStruct<'a> {
    pub(crate) ir_node: &'a ir::AbicoderPragma,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_abicoder_pragma<'a>(
    ir_node: &'a ir::AbicoderPragma,
    semantic: &'a SemanticContext,
) -> AbicoderPragma<'a> {
    AbicoderPragmaStruct { ir_node, semantic }
}

impl<'a> AbicoderPragmaStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn version(&self) -> AbicoderVersion {
        create_abicoder_version(&self.ir_node.version, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type AdditiveExpression<'a> = AdditiveExpressionStruct<'a>;

#[derive(Clone, Copy)]
pub struct AdditiveExpressionStruct<'a> {
    pub(crate) ir_node: &'a ir::AdditiveExpression,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_additive_expression<'a>(
    ir_node: &'a ir::AdditiveExpression,
    semantic: &'a SemanticContext,
) -> AdditiveExpression<'a> {
    AdditiveExpressionStruct { ir_node, semantic }
}

impl<'a> AdditiveExpressionStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn left_operand(&self) -> Expression<'a> {
        create_expression(&self.ir_node.left_operand, self.semantic)
    }

    pub fn operator(&self) -> AdditiveExpressionOperator<'a> {
        create_additive_expression_operator(&self.ir_node.operator, self.semantic)
    }

    pub fn right_operand(&self) -> Expression<'a> {
        create_expression(&self.ir_node.right_operand, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type AddressType<'a> = AddressTypeStruct<'a>;

#[derive(Clone, Copy)]
pub struct AddressTypeStruct<'a> {
    pub(crate) ir_node: &'a ir::AddressType,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_address_type<'a>(
    ir_node: &'a ir::AddressType,
    semantic: &'a SemanticContext,
) -> AddressType<'a> {
    AddressTypeStruct { ir_node, semantic }
}

impl<'a> AddressTypeStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn is_payable(&self) -> bool {
        self.ir_node.is_payable
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type AndExpression<'a> = AndExpressionStruct<'a>;

#[derive(Clone, Copy)]
pub struct AndExpressionStruct<'a> {
    pub(crate) ir_node: &'a ir::AndExpression,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_and_expression<'a>(
    ir_node: &'a ir::AndExpression,
    semantic: &'a SemanticContext,
) -> AndExpression<'a> {
    AndExpressionStruct { ir_node, semantic }
}

impl<'a> AndExpressionStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn left_operand(&self) -> Expression<'a> {
        create_expression(&self.ir_node.left_operand, self.semantic)
    }

    pub fn right_operand(&self) -> Expression<'a> {
        create_expression(&self.ir_node.right_operand, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type ArrayExpression<'a> = ArrayExpressionStruct<'a>;

#[derive(Clone, Copy)]
pub struct ArrayExpressionStruct<'a> {
    pub(crate) ir_node: &'a ir::ArrayExpression,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_array_expression<'a>(
    ir_node: &'a ir::ArrayExpression,
    semantic: &'a SemanticContext,
) -> ArrayExpression<'a> {
    ArrayExpressionStruct { ir_node, semantic }
}

impl<'a> ArrayExpressionStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn items(&self) -> ArrayValues<'a> {
        create_array_values(&self.ir_node.items, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type ArrayTypeName<'a> = ArrayTypeNameStruct<'a>;

#[derive(Clone, Copy)]
pub struct ArrayTypeNameStruct<'a> {
    pub(crate) ir_node: &'a ir::ArrayTypeName,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_array_type_name<'a>(
    ir_node: &'a ir::ArrayTypeName,
    semantic: &'a SemanticContext,
) -> ArrayTypeName<'a> {
    ArrayTypeNameStruct { ir_node, semantic }
}

impl<'a> ArrayTypeNameStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn operand(&self) -> TypeName<'a> {
        create_type_name(&self.ir_node.operand, self.semantic)
    }

    pub fn index(&self) -> Option<Expression<'a>> {
        self.ir_node
            .index
            .as_ref()
            .map(|ir_node| create_expression(ir_node, self.semantic))
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type AssemblyStatement<'a> = AssemblyStatementStruct<'a>;

#[derive(Clone, Copy)]
pub struct AssemblyStatementStruct<'a> {
    pub(crate) ir_node: &'a ir::AssemblyStatement,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_assembly_statement<'a>(
    ir_node: &'a ir::AssemblyStatement,
    semantic: &'a SemanticContext,
) -> AssemblyStatement<'a> {
    AssemblyStatementStruct { ir_node, semantic }
}

impl<'a> AssemblyStatementStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn is_memory_safe(&self) -> bool {
        self.ir_node.is_memory_safe
    }

    pub fn body(&self) -> YulBlock<'a> {
        create_yul_block(&self.ir_node.body, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type AssignmentExpression<'a> = AssignmentExpressionStruct<'a>;

#[derive(Clone, Copy)]
pub struct AssignmentExpressionStruct<'a> {
    pub(crate) ir_node: &'a ir::AssignmentExpression,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_assignment_expression<'a>(
    ir_node: &'a ir::AssignmentExpression,
    semantic: &'a SemanticContext,
) -> AssignmentExpression<'a> {
    AssignmentExpressionStruct { ir_node, semantic }
}

impl<'a> AssignmentExpressionStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn left_operand(&self) -> Expression<'a> {
        create_expression(&self.ir_node.left_operand, self.semantic)
    }

    pub fn operator(&self) -> AssignmentExpressionOperator<'a> {
        create_assignment_expression_operator(&self.ir_node.operator, self.semantic)
    }

    pub fn right_operand(&self) -> Expression<'a> {
        create_expression(&self.ir_node.right_operand, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type BitwiseAndExpression<'a> = BitwiseAndExpressionStruct<'a>;

#[derive(Clone, Copy)]
pub struct BitwiseAndExpressionStruct<'a> {
    pub(crate) ir_node: &'a ir::BitwiseAndExpression,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_bitwise_and_expression<'a>(
    ir_node: &'a ir::BitwiseAndExpression,
    semantic: &'a SemanticContext,
) -> BitwiseAndExpression<'a> {
    BitwiseAndExpressionStruct { ir_node, semantic }
}

impl<'a> BitwiseAndExpressionStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn left_operand(&self) -> Expression<'a> {
        create_expression(&self.ir_node.left_operand, self.semantic)
    }

    pub fn right_operand(&self) -> Expression<'a> {
        create_expression(&self.ir_node.right_operand, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type BitwiseOrExpression<'a> = BitwiseOrExpressionStruct<'a>;

#[derive(Clone, Copy)]
pub struct BitwiseOrExpressionStruct<'a> {
    pub(crate) ir_node: &'a ir::BitwiseOrExpression,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_bitwise_or_expression<'a>(
    ir_node: &'a ir::BitwiseOrExpression,
    semantic: &'a SemanticContext,
) -> BitwiseOrExpression<'a> {
    BitwiseOrExpressionStruct { ir_node, semantic }
}

impl<'a> BitwiseOrExpressionStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn left_operand(&self) -> Expression<'a> {
        create_expression(&self.ir_node.left_operand, self.semantic)
    }

    pub fn right_operand(&self) -> Expression<'a> {
        create_expression(&self.ir_node.right_operand, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type BitwiseXorExpression<'a> = BitwiseXorExpressionStruct<'a>;

#[derive(Clone, Copy)]
pub struct BitwiseXorExpressionStruct<'a> {
    pub(crate) ir_node: &'a ir::BitwiseXorExpression,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_bitwise_xor_expression<'a>(
    ir_node: &'a ir::BitwiseXorExpression,
    semantic: &'a SemanticContext,
) -> BitwiseXorExpression<'a> {
    BitwiseXorExpressionStruct { ir_node, semantic }
}

impl<'a> BitwiseXorExpressionStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn left_operand(&self) -> Expression<'a> {
        create_expression(&self.ir_node.left_operand, self.semantic)
    }

    pub fn right_operand(&self) -> Expression<'a> {
        create_expression(&self.ir_node.right_operand, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type Block<'a> = BlockStruct<'a>;

#[derive(Clone, Copy)]
pub struct BlockStruct<'a> {
    pub(crate) ir_node: &'a ir::Block,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_block<'a>(ir_node: &'a ir::Block, semantic: &'a SemanticContext) -> Block<'a> {
    BlockStruct { ir_node, semantic }
}

impl<'a> BlockStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn statements(&self) -> Statements<'a> {
        create_statements(&self.ir_node.statements, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type BreakStatement<'a> = BreakStatementStruct<'a>;

#[derive(Clone, Copy)]
pub struct BreakStatementStruct<'a> {
    pub(crate) ir_node: &'a ir::BreakStatement,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_break_statement<'a>(
    ir_node: &'a ir::BreakStatement,
    semantic: &'a SemanticContext,
) -> BreakStatement<'a> {
    BreakStatementStruct { ir_node, semantic }
}

impl<'a> BreakStatementStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type CallOptionsExpression<'a> = CallOptionsExpressionStruct<'a>;

#[derive(Clone, Copy)]
pub struct CallOptionsExpressionStruct<'a> {
    pub(crate) ir_node: &'a ir::CallOptionsExpression,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_call_options_expression<'a>(
    ir_node: &'a ir::CallOptionsExpression,
    semantic: &'a SemanticContext,
) -> CallOptionsExpression<'a> {
    CallOptionsExpressionStruct { ir_node, semantic }
}

impl<'a> CallOptionsExpressionStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn operand(&self) -> Expression<'a> {
        create_expression(&self.ir_node.operand, self.semantic)
    }

    pub fn options(&self) -> CallOptions<'a> {
        create_call_options(&self.ir_node.options, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type CatchClause<'a> = CatchClauseStruct<'a>;

#[derive(Clone, Copy)]
pub struct CatchClauseStruct<'a> {
    pub(crate) ir_node: &'a ir::CatchClause,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_catch_clause<'a>(
    ir_node: &'a ir::CatchClause,
    semantic: &'a SemanticContext,
) -> CatchClause<'a> {
    CatchClauseStruct { ir_node, semantic }
}

impl<'a> CatchClauseStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn kind(&self) -> CatchClauseKind {
        create_catch_clause_kind(&self.ir_node.kind, self.semantic)
    }

    pub fn parameters(&self) -> Option<Parameters<'a>> {
        self.ir_node
            .parameters
            .as_ref()
            .map(|ir_node| create_parameters(ir_node, self.semantic))
    }

    pub fn body(&self) -> Block<'a> {
        create_block(&self.ir_node.body, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type ConditionalExpression<'a> = ConditionalExpressionStruct<'a>;

#[derive(Clone, Copy)]
pub struct ConditionalExpressionStruct<'a> {
    pub(crate) ir_node: &'a ir::ConditionalExpression,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_conditional_expression<'a>(
    ir_node: &'a ir::ConditionalExpression,
    semantic: &'a SemanticContext,
) -> ConditionalExpression<'a> {
    ConditionalExpressionStruct { ir_node, semantic }
}

impl<'a> ConditionalExpressionStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn operand(&self) -> Expression<'a> {
        create_expression(&self.ir_node.operand, self.semantic)
    }

    pub fn true_expression(&self) -> Expression<'a> {
        create_expression(&self.ir_node.true_expression, self.semantic)
    }

    pub fn false_expression(&self) -> Expression<'a> {
        create_expression(&self.ir_node.false_expression, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type ConstantDefinition<'a> = ConstantDefinitionStruct<'a>;

#[derive(Clone, Copy)]
pub struct ConstantDefinitionStruct<'a> {
    pub(crate) ir_node: &'a ir::ConstantDefinition,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_constant_definition<'a>(
    ir_node: &'a ir::ConstantDefinition,
    semantic: &'a SemanticContext,
) -> ConstantDefinition<'a> {
    ConstantDefinitionStruct { ir_node, semantic }
}

impl<'a> ConstantDefinitionStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn type_name(&self) -> TypeName<'a> {
        create_type_name(&self.ir_node.type_name, self.semantic)
    }

    pub fn name(&self) -> Identifier<'a> {
        create_identifier(&self.ir_node.name, self.semantic)
    }

    pub fn visibility(&self) -> Option<StateVariableVisibility> {
        self.ir_node
            .visibility
            .as_ref()
            .map(|ir_node| create_state_variable_visibility(ir_node, self.semantic))
    }

    pub fn value(&self) -> Option<Expression<'a>> {
        self.ir_node
            .value
            .as_ref()
            .map(|ir_node| create_expression(ir_node, self.semantic))
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type ContinueStatement<'a> = ContinueStatementStruct<'a>;

#[derive(Clone, Copy)]
pub struct ContinueStatementStruct<'a> {
    pub(crate) ir_node: &'a ir::ContinueStatement,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_continue_statement<'a>(
    ir_node: &'a ir::ContinueStatement,
    semantic: &'a SemanticContext,
) -> ContinueStatement<'a> {
    ContinueStatementStruct { ir_node, semantic }
}

impl<'a> ContinueStatementStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type ContractDefinition<'a> = ContractDefinitionStruct<'a>;

#[derive(Clone, Copy)]
pub struct ContractDefinitionStruct<'a> {
    pub(crate) ir_node: &'a ir::ContractDefinition,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_contract_definition<'a>(
    ir_node: &'a ir::ContractDefinition,
    semantic: &'a SemanticContext,
) -> ContractDefinition<'a> {
    ContractDefinitionStruct { ir_node, semantic }
}

impl<'a> ContractDefinitionStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn is_abstract(&self) -> bool {
        self.ir_node.is_abstract
    }

    pub fn name(&self) -> Identifier<'a> {
        create_identifier(&self.ir_node.name, self.semantic)
    }

    pub fn inheritance_types(&self) -> InheritanceTypes<'a> {
        create_inheritance_types(&self.ir_node.inheritance_types, self.semantic)
    }

    pub fn storage_layout(&self) -> Option<Expression<'a>> {
        self.ir_node
            .storage_layout
            .as_ref()
            .map(|ir_node| create_expression(ir_node, self.semantic))
    }

    pub fn members(&self) -> ContractMembers<'a> {
        create_contract_members(&self.ir_node.members, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type DecimalNumberExpression<'a> = DecimalNumberExpressionStruct<'a>;

#[derive(Clone, Copy)]
pub struct DecimalNumberExpressionStruct<'a> {
    pub(crate) ir_node: &'a ir::DecimalNumberExpression,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_decimal_number_expression<'a>(
    ir_node: &'a ir::DecimalNumberExpression,
    semantic: &'a SemanticContext,
) -> DecimalNumberExpression<'a> {
    DecimalNumberExpressionStruct { ir_node, semantic }
}

impl<'a> DecimalNumberExpressionStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn literal(&self) -> DecimalLiteral<'a> {
        create_decimal_literal(&self.ir_node.literal, self.semantic)
    }

    pub fn unit(&self) -> Option<NumberUnit<'a>> {
        self.ir_node
            .unit
            .as_ref()
            .map(|ir_node| create_number_unit(ir_node, self.semantic))
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type DoWhileStatement<'a> = DoWhileStatementStruct<'a>;

#[derive(Clone, Copy)]
pub struct DoWhileStatementStruct<'a> {
    pub(crate) ir_node: &'a ir::DoWhileStatement,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_do_while_statement<'a>(
    ir_node: &'a ir::DoWhileStatement,
    semantic: &'a SemanticContext,
) -> DoWhileStatement<'a> {
    DoWhileStatementStruct { ir_node, semantic }
}

impl<'a> DoWhileStatementStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn body(&self) -> Statement<'a> {
        create_statement(&self.ir_node.body, self.semantic)
    }

    pub fn condition(&self) -> Expression<'a> {
        create_expression(&self.ir_node.condition, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type EmitStatement<'a> = EmitStatementStruct<'a>;

#[derive(Clone, Copy)]
pub struct EmitStatementStruct<'a> {
    pub(crate) ir_node: &'a ir::EmitStatement,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_emit_statement<'a>(
    ir_node: &'a ir::EmitStatement,
    semantic: &'a SemanticContext,
) -> EmitStatement<'a> {
    EmitStatementStruct { ir_node, semantic }
}

impl<'a> EmitStatementStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn event(&self) -> IdentifierPath<'a> {
        create_identifier_path(&self.ir_node.event, self.semantic)
    }

    pub fn arguments(&self) -> ArgumentsDeclaration<'a> {
        create_arguments_declaration(&self.ir_node.arguments, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type EnumDefinition<'a> = EnumDefinitionStruct<'a>;

#[derive(Clone, Copy)]
pub struct EnumDefinitionStruct<'a> {
    pub(crate) ir_node: &'a ir::EnumDefinition,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_enum_definition<'a>(
    ir_node: &'a ir::EnumDefinition,
    semantic: &'a SemanticContext,
) -> EnumDefinition<'a> {
    EnumDefinitionStruct { ir_node, semantic }
}

impl<'a> EnumDefinitionStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn name(&self) -> Identifier<'a> {
        create_identifier(&self.ir_node.name, self.semantic)
    }

    pub fn members(&self) -> EnumMembers<'a> {
        create_enum_members(&self.ir_node.members, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type EqualityExpression<'a> = EqualityExpressionStruct<'a>;

#[derive(Clone, Copy)]
pub struct EqualityExpressionStruct<'a> {
    pub(crate) ir_node: &'a ir::EqualityExpression,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_equality_expression<'a>(
    ir_node: &'a ir::EqualityExpression,
    semantic: &'a SemanticContext,
) -> EqualityExpression<'a> {
    EqualityExpressionStruct { ir_node, semantic }
}

impl<'a> EqualityExpressionStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn left_operand(&self) -> Expression<'a> {
        create_expression(&self.ir_node.left_operand, self.semantic)
    }

    pub fn operator(&self) -> EqualityExpressionOperator<'a> {
        create_equality_expression_operator(&self.ir_node.operator, self.semantic)
    }

    pub fn right_operand(&self) -> Expression<'a> {
        create_expression(&self.ir_node.right_operand, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type ErrorDefinition<'a> = ErrorDefinitionStruct<'a>;

#[derive(Clone, Copy)]
pub struct ErrorDefinitionStruct<'a> {
    pub(crate) ir_node: &'a ir::ErrorDefinition,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_error_definition<'a>(
    ir_node: &'a ir::ErrorDefinition,
    semantic: &'a SemanticContext,
) -> ErrorDefinition<'a> {
    ErrorDefinitionStruct { ir_node, semantic }
}

impl<'a> ErrorDefinitionStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn name(&self) -> Identifier<'a> {
        create_identifier(&self.ir_node.name, self.semantic)
    }

    pub fn parameters(&self) -> Parameters<'a> {
        create_parameters(&self.ir_node.parameters, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type EventDefinition<'a> = EventDefinitionStruct<'a>;

#[derive(Clone, Copy)]
pub struct EventDefinitionStruct<'a> {
    pub(crate) ir_node: &'a ir::EventDefinition,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_event_definition<'a>(
    ir_node: &'a ir::EventDefinition,
    semantic: &'a SemanticContext,
) -> EventDefinition<'a> {
    EventDefinitionStruct { ir_node, semantic }
}

impl<'a> EventDefinitionStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn name(&self) -> Identifier<'a> {
        create_identifier(&self.ir_node.name, self.semantic)
    }

    pub fn is_anonymous(&self) -> bool {
        self.ir_node.is_anonymous
    }

    pub fn parameters(&self) -> Parameters<'a> {
        create_parameters(&self.ir_node.parameters, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type ExperimentalPragma<'a> = ExperimentalPragmaStruct<'a>;

#[derive(Clone, Copy)]
pub struct ExperimentalPragmaStruct<'a> {
    pub(crate) ir_node: &'a ir::ExperimentalPragma,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_experimental_pragma<'a>(
    ir_node: &'a ir::ExperimentalPragma,
    semantic: &'a SemanticContext,
) -> ExperimentalPragma<'a> {
    ExperimentalPragmaStruct { ir_node, semantic }
}

impl<'a> ExperimentalPragmaStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn feature(&self) -> ExperimentalFeature {
        create_experimental_feature(&self.ir_node.feature, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type ExponentiationExpression<'a> = ExponentiationExpressionStruct<'a>;

#[derive(Clone, Copy)]
pub struct ExponentiationExpressionStruct<'a> {
    pub(crate) ir_node: &'a ir::ExponentiationExpression,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_exponentiation_expression<'a>(
    ir_node: &'a ir::ExponentiationExpression,
    semantic: &'a SemanticContext,
) -> ExponentiationExpression<'a> {
    ExponentiationExpressionStruct { ir_node, semantic }
}

impl<'a> ExponentiationExpressionStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn left_operand(&self) -> Expression<'a> {
        create_expression(&self.ir_node.left_operand, self.semantic)
    }

    pub fn right_operand(&self) -> Expression<'a> {
        create_expression(&self.ir_node.right_operand, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type ExpressionStatement<'a> = ExpressionStatementStruct<'a>;

#[derive(Clone, Copy)]
pub struct ExpressionStatementStruct<'a> {
    pub(crate) ir_node: &'a ir::ExpressionStatement,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_expression_statement<'a>(
    ir_node: &'a ir::ExpressionStatement,
    semantic: &'a SemanticContext,
) -> ExpressionStatement<'a> {
    ExpressionStatementStruct { ir_node, semantic }
}

impl<'a> ExpressionStatementStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn expression(&self) -> Expression<'a> {
        create_expression(&self.ir_node.expression, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type ForStatement<'a> = ForStatementStruct<'a>;

#[derive(Clone, Copy)]
pub struct ForStatementStruct<'a> {
    pub(crate) ir_node: &'a ir::ForStatement,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_for_statement<'a>(
    ir_node: &'a ir::ForStatement,
    semantic: &'a SemanticContext,
) -> ForStatement<'a> {
    ForStatementStruct { ir_node, semantic }
}

impl<'a> ForStatementStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn initialization(&self) -> ForStatementInitialization<'a> {
        create_for_statement_initialization(&self.ir_node.initialization, self.semantic)
    }

    pub fn condition(&self) -> ForStatementCondition<'a> {
        create_for_statement_condition(&self.ir_node.condition, self.semantic)
    }

    pub fn iterator(&self) -> Option<Expression<'a>> {
        self.ir_node
            .iterator
            .as_ref()
            .map(|ir_node| create_expression(ir_node, self.semantic))
    }

    pub fn body(&self) -> Statement<'a> {
        create_statement(&self.ir_node.body, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type FunctionAttributes<'a> = FunctionAttributesStruct<'a>;

#[derive(Clone, Copy)]
pub struct FunctionAttributesStruct<'a> {
    pub(crate) ir_node: &'a ir::FunctionAttributes,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_function_attributes<'a>(
    ir_node: &'a ir::FunctionAttributes,
    semantic: &'a SemanticContext,
) -> FunctionAttributes<'a> {
    FunctionAttributesStruct { ir_node, semantic }
}

impl<'a> FunctionAttributesStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn visibility(&self) -> FunctionVisibility {
        create_function_visibility(&self.ir_node.visibility, self.semantic)
    }

    pub fn has_explicit_visibility(&self) -> bool {
        self.ir_node.has_explicit_visibility
    }

    pub fn mutability(&self) -> FunctionMutability {
        create_function_mutability(&self.ir_node.mutability, self.semantic)
    }

    pub fn is_virtual(&self) -> bool {
        self.ir_node.is_virtual
    }

    pub fn override_specifier(&self) -> Option<OverridePaths<'a>> {
        self.ir_node
            .override_specifier
            .as_ref()
            .map(|ir_node| create_override_paths(ir_node, self.semantic))
    }

    pub fn modifier_invocations(&self) -> ModifierInvocations<'a> {
        create_modifier_invocations(&self.ir_node.modifier_invocations, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type FunctionCallExpression<'a> = FunctionCallExpressionStruct<'a>;

#[derive(Clone, Copy)]
pub struct FunctionCallExpressionStruct<'a> {
    pub(crate) ir_node: &'a ir::FunctionCallExpression,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_function_call_expression<'a>(
    ir_node: &'a ir::FunctionCallExpression,
    semantic: &'a SemanticContext,
) -> FunctionCallExpression<'a> {
    FunctionCallExpressionStruct { ir_node, semantic }
}

impl<'a> FunctionCallExpressionStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn operand(&self) -> Expression<'a> {
        create_expression(&self.ir_node.operand, self.semantic)
    }

    pub fn arguments(&self) -> ArgumentsDeclaration<'a> {
        create_arguments_declaration(&self.ir_node.arguments, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type FunctionDefinition<'a> = FunctionDefinitionStruct<'a>;

#[derive(Clone, Copy)]
pub struct FunctionDefinitionStruct<'a> {
    pub(crate) ir_node: &'a ir::FunctionDefinition,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_function_definition<'a>(
    ir_node: &'a ir::FunctionDefinition,
    semantic: &'a SemanticContext,
) -> FunctionDefinition<'a> {
    FunctionDefinitionStruct { ir_node, semantic }
}

impl<'a> FunctionDefinitionStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn kind(&self) -> FunctionKind {
        create_function_kind(&self.ir_node.kind, self.semantic)
    }

    pub fn name(&self) -> Option<Identifier<'a>> {
        self.ir_node
            .name
            .as_ref()
            .map(|ir_node| create_identifier(ir_node, self.semantic))
    }

    pub fn parameters(&self) -> Parameters<'a> {
        create_parameters(&self.ir_node.parameters, self.semantic)
    }

    pub fn attributes(&self) -> FunctionAttributes<'a> {
        create_function_attributes(&self.ir_node.attributes, self.semantic)
    }

    pub fn returns(&self) -> Option<Parameters<'a>> {
        self.ir_node
            .returns
            .as_ref()
            .map(|ir_node| create_parameters(ir_node, self.semantic))
    }

    pub fn body(&self) -> Option<Block<'a>> {
        self.ir_node
            .body
            .as_ref()
            .map(|ir_node| create_block(ir_node, self.semantic))
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type FunctionType<'a> = FunctionTypeStruct<'a>;

#[derive(Clone, Copy)]
pub struct FunctionTypeStruct<'a> {
    pub(crate) ir_node: &'a ir::FunctionType,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_function_type<'a>(
    ir_node: &'a ir::FunctionType,
    semantic: &'a SemanticContext,
) -> FunctionType<'a> {
    FunctionTypeStruct { ir_node, semantic }
}

impl<'a> FunctionTypeStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn parameters(&self) -> Parameters<'a> {
        create_parameters(&self.ir_node.parameters, self.semantic)
    }

    pub fn attributes(&self) -> FunctionTypeAttributes<'a> {
        create_function_type_attributes(&self.ir_node.attributes, self.semantic)
    }

    pub fn returns(&self) -> Option<Parameters<'a>> {
        self.ir_node
            .returns
            .as_ref()
            .map(|ir_node| create_parameters(ir_node, self.semantic))
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type FunctionTypeAttributes<'a> = FunctionTypeAttributesStruct<'a>;

#[derive(Clone, Copy)]
pub struct FunctionTypeAttributesStruct<'a> {
    pub(crate) ir_node: &'a ir::FunctionTypeAttributes,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_function_type_attributes<'a>(
    ir_node: &'a ir::FunctionTypeAttributes,
    semantic: &'a SemanticContext,
) -> FunctionTypeAttributes<'a> {
    FunctionTypeAttributesStruct { ir_node, semantic }
}

impl<'a> FunctionTypeAttributesStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn visibility(&self) -> FunctionVisibility {
        create_function_visibility(&self.ir_node.visibility, self.semantic)
    }

    pub fn mutability(&self) -> FunctionMutability {
        create_function_mutability(&self.ir_node.mutability, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type HexNumberExpression<'a> = HexNumberExpressionStruct<'a>;

#[derive(Clone, Copy)]
pub struct HexNumberExpressionStruct<'a> {
    pub(crate) ir_node: &'a ir::HexNumberExpression,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_hex_number_expression<'a>(
    ir_node: &'a ir::HexNumberExpression,
    semantic: &'a SemanticContext,
) -> HexNumberExpression<'a> {
    HexNumberExpressionStruct { ir_node, semantic }
}

impl<'a> HexNumberExpressionStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn literal(&self) -> HexLiteral<'a> {
        create_hex_literal(&self.ir_node.literal, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type IfStatement<'a> = IfStatementStruct<'a>;

#[derive(Clone, Copy)]
pub struct IfStatementStruct<'a> {
    pub(crate) ir_node: &'a ir::IfStatement,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_if_statement<'a>(
    ir_node: &'a ir::IfStatement,
    semantic: &'a SemanticContext,
) -> IfStatement<'a> {
    IfStatementStruct { ir_node, semantic }
}

impl<'a> IfStatementStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn condition(&self) -> Expression<'a> {
        create_expression(&self.ir_node.condition, self.semantic)
    }

    pub fn body(&self) -> Statement<'a> {
        create_statement(&self.ir_node.body, self.semantic)
    }

    pub fn else_branch(&self) -> Option<Statement<'a>> {
        self.ir_node
            .else_branch
            .as_ref()
            .map(|ir_node| create_statement(ir_node, self.semantic))
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type ImportDeconstruction<'a> = ImportDeconstructionStruct<'a>;

#[derive(Clone, Copy)]
pub struct ImportDeconstructionStruct<'a> {
    pub(crate) ir_node: &'a ir::ImportDeconstruction,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_import_deconstruction<'a>(
    ir_node: &'a ir::ImportDeconstruction,
    semantic: &'a SemanticContext,
) -> ImportDeconstruction<'a> {
    ImportDeconstructionStruct { ir_node, semantic }
}

impl<'a> ImportDeconstructionStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn symbols(&self) -> ImportDeconstructionSymbols<'a> {
        create_import_deconstruction_symbols(&self.ir_node.symbols, self.semantic)
    }

    pub fn path(&self) -> StringLiteral<'a> {
        create_string_literal(&self.ir_node.path, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type ImportDeconstructionSymbol<'a> = ImportDeconstructionSymbolStruct<'a>;

#[derive(Clone, Copy)]
pub struct ImportDeconstructionSymbolStruct<'a> {
    pub(crate) ir_node: &'a ir::ImportDeconstructionSymbol,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_import_deconstruction_symbol<'a>(
    ir_node: &'a ir::ImportDeconstructionSymbol,
    semantic: &'a SemanticContext,
) -> ImportDeconstructionSymbol<'a> {
    ImportDeconstructionSymbolStruct { ir_node, semantic }
}

impl<'a> ImportDeconstructionSymbolStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn name(&self) -> Identifier<'a> {
        create_identifier(&self.ir_node.name, self.semantic)
    }

    pub fn alias(&self) -> Option<Identifier<'a>> {
        self.ir_node
            .alias
            .as_ref()
            .map(|ir_node| create_identifier(ir_node, self.semantic))
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type IndexAccessExpression<'a> = IndexAccessExpressionStruct<'a>;

#[derive(Clone, Copy)]
pub struct IndexAccessExpressionStruct<'a> {
    pub(crate) ir_node: &'a ir::IndexAccessExpression,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_index_access_expression<'a>(
    ir_node: &'a ir::IndexAccessExpression,
    semantic: &'a SemanticContext,
) -> IndexAccessExpression<'a> {
    IndexAccessExpressionStruct { ir_node, semantic }
}

impl<'a> IndexAccessExpressionStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn operand(&self) -> Expression<'a> {
        create_expression(&self.ir_node.operand, self.semantic)
    }

    pub fn start(&self) -> Option<Expression<'a>> {
        self.ir_node
            .start
            .as_ref()
            .map(|ir_node| create_expression(ir_node, self.semantic))
    }

    pub fn end(&self) -> Option<Expression<'a>> {
        self.ir_node
            .end
            .as_ref()
            .map(|ir_node| create_expression(ir_node, self.semantic))
    }

    pub fn is_slice(&self) -> bool {
        self.ir_node.is_slice
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type InequalityExpression<'a> = InequalityExpressionStruct<'a>;

#[derive(Clone, Copy)]
pub struct InequalityExpressionStruct<'a> {
    pub(crate) ir_node: &'a ir::InequalityExpression,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_inequality_expression<'a>(
    ir_node: &'a ir::InequalityExpression,
    semantic: &'a SemanticContext,
) -> InequalityExpression<'a> {
    InequalityExpressionStruct { ir_node, semantic }
}

impl<'a> InequalityExpressionStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn left_operand(&self) -> Expression<'a> {
        create_expression(&self.ir_node.left_operand, self.semantic)
    }

    pub fn operator(&self) -> InequalityExpressionOperator<'a> {
        create_inequality_expression_operator(&self.ir_node.operator, self.semantic)
    }

    pub fn right_operand(&self) -> Expression<'a> {
        create_expression(&self.ir_node.right_operand, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type InheritanceType<'a> = InheritanceTypeStruct<'a>;

#[derive(Clone, Copy)]
pub struct InheritanceTypeStruct<'a> {
    pub(crate) ir_node: &'a ir::InheritanceType,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_inheritance_type<'a>(
    ir_node: &'a ir::InheritanceType,
    semantic: &'a SemanticContext,
) -> InheritanceType<'a> {
    InheritanceTypeStruct { ir_node, semantic }
}

impl<'a> InheritanceTypeStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn type_name(&self) -> IdentifierPath<'a> {
        create_identifier_path(&self.ir_node.type_name, self.semantic)
    }

    pub fn arguments(&self) -> Option<PositionalArguments<'a>> {
        self.ir_node
            .arguments
            .as_ref()
            .map(|ir_node| create_positional_arguments(ir_node, self.semantic))
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type InterfaceDefinition<'a> = InterfaceDefinitionStruct<'a>;

#[derive(Clone, Copy)]
pub struct InterfaceDefinitionStruct<'a> {
    pub(crate) ir_node: &'a ir::InterfaceDefinition,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_interface_definition<'a>(
    ir_node: &'a ir::InterfaceDefinition,
    semantic: &'a SemanticContext,
) -> InterfaceDefinition<'a> {
    InterfaceDefinitionStruct { ir_node, semantic }
}

impl<'a> InterfaceDefinitionStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn name(&self) -> Identifier<'a> {
        create_identifier(&self.ir_node.name, self.semantic)
    }

    pub fn inheritance(&self) -> Option<InheritanceTypes<'a>> {
        self.ir_node
            .inheritance
            .as_ref()
            .map(|ir_node| create_inheritance_types(ir_node, self.semantic))
    }

    pub fn members(&self) -> InterfaceMembers<'a> {
        create_interface_members(&self.ir_node.members, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type LibraryDefinition<'a> = LibraryDefinitionStruct<'a>;

#[derive(Clone, Copy)]
pub struct LibraryDefinitionStruct<'a> {
    pub(crate) ir_node: &'a ir::LibraryDefinition,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_library_definition<'a>(
    ir_node: &'a ir::LibraryDefinition,
    semantic: &'a SemanticContext,
) -> LibraryDefinition<'a> {
    LibraryDefinitionStruct { ir_node, semantic }
}

impl<'a> LibraryDefinitionStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn name(&self) -> Identifier<'a> {
        create_identifier(&self.ir_node.name, self.semantic)
    }

    pub fn members(&self) -> LibraryMembers<'a> {
        create_library_members(&self.ir_node.members, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type MappingType<'a> = MappingTypeStruct<'a>;

#[derive(Clone, Copy)]
pub struct MappingTypeStruct<'a> {
    pub(crate) ir_node: &'a ir::MappingType,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_mapping_type<'a>(
    ir_node: &'a ir::MappingType,
    semantic: &'a SemanticContext,
) -> MappingType<'a> {
    MappingTypeStruct { ir_node, semantic }
}

impl<'a> MappingTypeStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn key_type(&self) -> Parameter<'a> {
        create_parameter(&self.ir_node.key_type, self.semantic)
    }

    pub fn value_type(&self) -> Parameter<'a> {
        create_parameter(&self.ir_node.value_type, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type MemberAccessExpression<'a> = MemberAccessExpressionStruct<'a>;

#[derive(Clone, Copy)]
pub struct MemberAccessExpressionStruct<'a> {
    pub(crate) ir_node: &'a ir::MemberAccessExpression,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_member_access_expression<'a>(
    ir_node: &'a ir::MemberAccessExpression,
    semantic: &'a SemanticContext,
) -> MemberAccessExpression<'a> {
    MemberAccessExpressionStruct { ir_node, semantic }
}

impl<'a> MemberAccessExpressionStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn operand(&self) -> Expression<'a> {
        create_expression(&self.ir_node.operand, self.semantic)
    }

    pub fn member(&self) -> Identifier<'a> {
        create_identifier(&self.ir_node.member, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type ModifierAttributes<'a> = ModifierAttributesStruct<'a>;

#[derive(Clone, Copy)]
pub struct ModifierAttributesStruct<'a> {
    pub(crate) ir_node: &'a ir::ModifierAttributes,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_modifier_attributes<'a>(
    ir_node: &'a ir::ModifierAttributes,
    semantic: &'a SemanticContext,
) -> ModifierAttributes<'a> {
    ModifierAttributesStruct { ir_node, semantic }
}

impl<'a> ModifierAttributesStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn is_virtual(&self) -> bool {
        self.ir_node.is_virtual
    }

    pub fn override_specifier(&self) -> Option<OverridePaths<'a>> {
        self.ir_node
            .override_specifier
            .as_ref()
            .map(|ir_node| create_override_paths(ir_node, self.semantic))
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type ModifierDefinition<'a> = ModifierDefinitionStruct<'a>;

#[derive(Clone, Copy)]
pub struct ModifierDefinitionStruct<'a> {
    pub(crate) ir_node: &'a ir::ModifierDefinition,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_modifier_definition<'a>(
    ir_node: &'a ir::ModifierDefinition,
    semantic: &'a SemanticContext,
) -> ModifierDefinition<'a> {
    ModifierDefinitionStruct { ir_node, semantic }
}

impl<'a> ModifierDefinitionStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn name(&self) -> Identifier<'a> {
        create_identifier(&self.ir_node.name, self.semantic)
    }

    pub fn parameters(&self) -> Parameters<'a> {
        create_parameters(&self.ir_node.parameters, self.semantic)
    }

    pub fn attributes(&self) -> ModifierAttributes<'a> {
        create_modifier_attributes(&self.ir_node.attributes, self.semantic)
    }

    pub fn body(&self) -> Option<Block<'a>> {
        self.ir_node
            .body
            .as_ref()
            .map(|ir_node| create_block(ir_node, self.semantic))
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type ModifierInvocation<'a> = ModifierInvocationStruct<'a>;

#[derive(Clone, Copy)]
pub struct ModifierInvocationStruct<'a> {
    pub(crate) ir_node: &'a ir::ModifierInvocation,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_modifier_invocation<'a>(
    ir_node: &'a ir::ModifierInvocation,
    semantic: &'a SemanticContext,
) -> ModifierInvocation<'a> {
    ModifierInvocationStruct { ir_node, semantic }
}

impl<'a> ModifierInvocationStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn name(&self) -> IdentifierPath<'a> {
        create_identifier_path(&self.ir_node.name, self.semantic)
    }

    pub fn arguments(&self) -> Option<PositionalArguments<'a>> {
        self.ir_node
            .arguments
            .as_ref()
            .map(|ir_node| create_positional_arguments(ir_node, self.semantic))
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type MultiTypedDeclaration<'a> = MultiTypedDeclarationStruct<'a>;

#[derive(Clone, Copy)]
pub struct MultiTypedDeclarationStruct<'a> {
    pub(crate) ir_node: &'a ir::MultiTypedDeclaration,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_multi_typed_declaration<'a>(
    ir_node: &'a ir::MultiTypedDeclaration,
    semantic: &'a SemanticContext,
) -> MultiTypedDeclaration<'a> {
    MultiTypedDeclarationStruct { ir_node, semantic }
}

impl<'a> MultiTypedDeclarationStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn elements(&self) -> MultiTypedDeclarationElements<'a> {
        create_multi_typed_declaration_elements(&self.ir_node.elements, self.semantic)
    }

    pub fn value(&self) -> Expression<'a> {
        create_expression(&self.ir_node.value, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type MultiTypedDeclarationElement<'a> = MultiTypedDeclarationElementStruct<'a>;

#[derive(Clone, Copy)]
pub struct MultiTypedDeclarationElementStruct<'a> {
    pub(crate) ir_node: &'a ir::MultiTypedDeclarationElement,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_multi_typed_declaration_element<'a>(
    ir_node: &'a ir::MultiTypedDeclarationElement,
    semantic: &'a SemanticContext,
) -> MultiTypedDeclarationElement<'a> {
    MultiTypedDeclarationElementStruct { ir_node, semantic }
}

impl<'a> MultiTypedDeclarationElementStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn member(&self) -> Option<VariableDeclaration<'a>> {
        self.ir_node
            .member
            .as_ref()
            .map(|ir_node| create_variable_declaration(ir_node, self.semantic))
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type MultiplicativeExpression<'a> = MultiplicativeExpressionStruct<'a>;

#[derive(Clone, Copy)]
pub struct MultiplicativeExpressionStruct<'a> {
    pub(crate) ir_node: &'a ir::MultiplicativeExpression,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_multiplicative_expression<'a>(
    ir_node: &'a ir::MultiplicativeExpression,
    semantic: &'a SemanticContext,
) -> MultiplicativeExpression<'a> {
    MultiplicativeExpressionStruct { ir_node, semantic }
}

impl<'a> MultiplicativeExpressionStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn left_operand(&self) -> Expression<'a> {
        create_expression(&self.ir_node.left_operand, self.semantic)
    }

    pub fn operator(&self) -> MultiplicativeExpressionOperator<'a> {
        create_multiplicative_expression_operator(&self.ir_node.operator, self.semantic)
    }

    pub fn right_operand(&self) -> Expression<'a> {
        create_expression(&self.ir_node.right_operand, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type NamedArgument<'a> = NamedArgumentStruct<'a>;

#[derive(Clone, Copy)]
pub struct NamedArgumentStruct<'a> {
    pub(crate) ir_node: &'a ir::NamedArgument,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_named_argument<'a>(
    ir_node: &'a ir::NamedArgument,
    semantic: &'a SemanticContext,
) -> NamedArgument<'a> {
    NamedArgumentStruct { ir_node, semantic }
}

impl<'a> NamedArgumentStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn name(&self) -> Identifier<'a> {
        create_identifier(&self.ir_node.name, self.semantic)
    }

    pub fn value(&self) -> Expression<'a> {
        create_expression(&self.ir_node.value, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type NewExpression<'a> = NewExpressionStruct<'a>;

#[derive(Clone, Copy)]
pub struct NewExpressionStruct<'a> {
    pub(crate) ir_node: &'a ir::NewExpression,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_new_expression<'a>(
    ir_node: &'a ir::NewExpression,
    semantic: &'a SemanticContext,
) -> NewExpression<'a> {
    NewExpressionStruct { ir_node, semantic }
}

impl<'a> NewExpressionStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn type_name(&self) -> TypeName<'a> {
        create_type_name(&self.ir_node.type_name, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type OrExpression<'a> = OrExpressionStruct<'a>;

#[derive(Clone, Copy)]
pub struct OrExpressionStruct<'a> {
    pub(crate) ir_node: &'a ir::OrExpression,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_or_expression<'a>(
    ir_node: &'a ir::OrExpression,
    semantic: &'a SemanticContext,
) -> OrExpression<'a> {
    OrExpressionStruct { ir_node, semantic }
}

impl<'a> OrExpressionStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn left_operand(&self) -> Expression<'a> {
        create_expression(&self.ir_node.left_operand, self.semantic)
    }

    pub fn right_operand(&self) -> Expression<'a> {
        create_expression(&self.ir_node.right_operand, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type Parameter<'a> = ParameterStruct<'a>;

#[derive(Clone, Copy)]
pub struct ParameterStruct<'a> {
    pub(crate) ir_node: &'a ir::Parameter,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_parameter<'a>(
    ir_node: &'a ir::Parameter,
    semantic: &'a SemanticContext,
) -> Parameter<'a> {
    ParameterStruct { ir_node, semantic }
}

impl<'a> ParameterStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn type_name(&self) -> TypeName<'a> {
        create_type_name(&self.ir_node.type_name, self.semantic)
    }

    pub fn storage_location(&self) -> Option<StorageLocation<'a>> {
        self.ir_node
            .storage_location
            .as_ref()
            .map(|ir_node| create_storage_location(ir_node, self.semantic))
    }

    pub fn name(&self) -> Option<Identifier<'a>> {
        self.ir_node
            .name
            .as_ref()
            .map(|ir_node| create_identifier(ir_node, self.semantic))
    }

    pub fn is_indexed(&self) -> bool {
        self.ir_node.is_indexed
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type PathImport<'a> = PathImportStruct<'a>;

#[derive(Clone, Copy)]
pub struct PathImportStruct<'a> {
    pub(crate) ir_node: &'a ir::PathImport,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_path_import<'a>(
    ir_node: &'a ir::PathImport,
    semantic: &'a SemanticContext,
) -> PathImport<'a> {
    PathImportStruct { ir_node, semantic }
}

impl<'a> PathImportStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn path(&self) -> StringLiteral<'a> {
        create_string_literal(&self.ir_node.path, self.semantic)
    }

    pub fn alias(&self) -> Option<Identifier<'a>> {
        self.ir_node
            .alias
            .as_ref()
            .map(|ir_node| create_identifier(ir_node, self.semantic))
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type PostfixExpression<'a> = PostfixExpressionStruct<'a>;

#[derive(Clone, Copy)]
pub struct PostfixExpressionStruct<'a> {
    pub(crate) ir_node: &'a ir::PostfixExpression,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_postfix_expression<'a>(
    ir_node: &'a ir::PostfixExpression,
    semantic: &'a SemanticContext,
) -> PostfixExpression<'a> {
    PostfixExpressionStruct { ir_node, semantic }
}

impl<'a> PostfixExpressionStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn operand(&self) -> Expression<'a> {
        create_expression(&self.ir_node.operand, self.semantic)
    }

    pub fn operator(&self) -> PostfixExpressionOperator<'a> {
        create_postfix_expression_operator(&self.ir_node.operator, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type PragmaDirective<'a> = PragmaDirectiveStruct<'a>;

#[derive(Clone, Copy)]
pub struct PragmaDirectiveStruct<'a> {
    pub(crate) ir_node: &'a ir::PragmaDirective,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_pragma_directive<'a>(
    ir_node: &'a ir::PragmaDirective,
    semantic: &'a SemanticContext,
) -> PragmaDirective<'a> {
    PragmaDirectiveStruct { ir_node, semantic }
}

impl<'a> PragmaDirectiveStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn pragma(&self) -> Pragma<'a> {
        create_pragma(&self.ir_node.pragma, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type PrefixExpression<'a> = PrefixExpressionStruct<'a>;

#[derive(Clone, Copy)]
pub struct PrefixExpressionStruct<'a> {
    pub(crate) ir_node: &'a ir::PrefixExpression,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_prefix_expression<'a>(
    ir_node: &'a ir::PrefixExpression,
    semantic: &'a SemanticContext,
) -> PrefixExpression<'a> {
    PrefixExpressionStruct { ir_node, semantic }
}

impl<'a> PrefixExpressionStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn operator(&self) -> PrefixExpressionOperator<'a> {
        create_prefix_expression_operator(&self.ir_node.operator, self.semantic)
    }

    pub fn operand(&self) -> Expression<'a> {
        create_expression(&self.ir_node.operand, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type ReturnStatement<'a> = ReturnStatementStruct<'a>;

#[derive(Clone, Copy)]
pub struct ReturnStatementStruct<'a> {
    pub(crate) ir_node: &'a ir::ReturnStatement,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_return_statement<'a>(
    ir_node: &'a ir::ReturnStatement,
    semantic: &'a SemanticContext,
) -> ReturnStatement<'a> {
    ReturnStatementStruct { ir_node, semantic }
}

impl<'a> ReturnStatementStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn expression(&self) -> Option<Expression<'a>> {
        self.ir_node
            .expression
            .as_ref()
            .map(|ir_node| create_expression(ir_node, self.semantic))
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type RevertStatement<'a> = RevertStatementStruct<'a>;

#[derive(Clone, Copy)]
pub struct RevertStatementStruct<'a> {
    pub(crate) ir_node: &'a ir::RevertStatement,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_revert_statement<'a>(
    ir_node: &'a ir::RevertStatement,
    semantic: &'a SemanticContext,
) -> RevertStatement<'a> {
    RevertStatementStruct { ir_node, semantic }
}

impl<'a> RevertStatementStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn error(&self) -> IdentifierPath<'a> {
        create_identifier_path(&self.ir_node.error, self.semantic)
    }

    pub fn arguments(&self) -> ArgumentsDeclaration<'a> {
        create_arguments_declaration(&self.ir_node.arguments, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type ShiftExpression<'a> = ShiftExpressionStruct<'a>;

#[derive(Clone, Copy)]
pub struct ShiftExpressionStruct<'a> {
    pub(crate) ir_node: &'a ir::ShiftExpression,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_shift_expression<'a>(
    ir_node: &'a ir::ShiftExpression,
    semantic: &'a SemanticContext,
) -> ShiftExpression<'a> {
    ShiftExpressionStruct { ir_node, semantic }
}

impl<'a> ShiftExpressionStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn left_operand(&self) -> Expression<'a> {
        create_expression(&self.ir_node.left_operand, self.semantic)
    }

    pub fn operator(&self) -> ShiftExpressionOperator<'a> {
        create_shift_expression_operator(&self.ir_node.operator, self.semantic)
    }

    pub fn right_operand(&self) -> Expression<'a> {
        create_expression(&self.ir_node.right_operand, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type SingleTypedDeclaration<'a> = SingleTypedDeclarationStruct<'a>;

#[derive(Clone, Copy)]
pub struct SingleTypedDeclarationStruct<'a> {
    pub(crate) ir_node: &'a ir::SingleTypedDeclaration,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_single_typed_declaration<'a>(
    ir_node: &'a ir::SingleTypedDeclaration,
    semantic: &'a SemanticContext,
) -> SingleTypedDeclaration<'a> {
    SingleTypedDeclarationStruct { ir_node, semantic }
}

impl<'a> SingleTypedDeclarationStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn declaration(&self) -> VariableDeclaration<'a> {
        create_variable_declaration(&self.ir_node.declaration, self.semantic)
    }

    pub fn value(&self) -> Option<Expression<'a>> {
        self.ir_node
            .value
            .as_ref()
            .map(|ir_node| create_expression(ir_node, self.semantic))
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type SourceUnit<'a> = SourceUnitStruct<'a>;

#[derive(Clone, Copy)]
pub struct SourceUnitStruct<'a> {
    pub(crate) ir_node: &'a ir::SourceUnit,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_source_unit<'a>(
    ir_node: &'a ir::SourceUnit,
    semantic: &'a SemanticContext,
) -> SourceUnit<'a> {
    SourceUnitStruct { ir_node, semantic }
}

impl<'a> SourceUnitStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn members(&self) -> SourceUnitMembers<'a> {
        create_source_unit_members(&self.ir_node.members, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type StateVariableAttributes<'a> = StateVariableAttributesStruct<'a>;

#[derive(Clone, Copy)]
pub struct StateVariableAttributesStruct<'a> {
    pub(crate) ir_node: &'a ir::StateVariableAttributes,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_state_variable_attributes<'a>(
    ir_node: &'a ir::StateVariableAttributes,
    semantic: &'a SemanticContext,
) -> StateVariableAttributes<'a> {
    StateVariableAttributesStruct { ir_node, semantic }
}

impl<'a> StateVariableAttributesStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn visibility(&self) -> StateVariableVisibility {
        create_state_variable_visibility(&self.ir_node.visibility, self.semantic)
    }

    pub fn mutability(&self) -> StateVariableMutability {
        create_state_variable_mutability(&self.ir_node.mutability, self.semantic)
    }

    pub fn override_specifier(&self) -> Option<OverridePaths<'a>> {
        self.ir_node
            .override_specifier
            .as_ref()
            .map(|ir_node| create_override_paths(ir_node, self.semantic))
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type StateVariableDefinition<'a> = StateVariableDefinitionStruct<'a>;

#[derive(Clone, Copy)]
pub struct StateVariableDefinitionStruct<'a> {
    pub(crate) ir_node: &'a ir::StateVariableDefinition,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_state_variable_definition<'a>(
    ir_node: &'a ir::StateVariableDefinition,
    semantic: &'a SemanticContext,
) -> StateVariableDefinition<'a> {
    StateVariableDefinitionStruct { ir_node, semantic }
}

impl<'a> StateVariableDefinitionStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn type_name(&self) -> TypeName<'a> {
        create_type_name(&self.ir_node.type_name, self.semantic)
    }

    pub fn name(&self) -> Identifier<'a> {
        create_identifier(&self.ir_node.name, self.semantic)
    }

    pub fn value(&self) -> Option<Expression<'a>> {
        self.ir_node
            .value
            .as_ref()
            .map(|ir_node| create_expression(ir_node, self.semantic))
    }

    pub fn attributes(&self) -> StateVariableAttributes<'a> {
        create_state_variable_attributes(&self.ir_node.attributes, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type StructDefinition<'a> = StructDefinitionStruct<'a>;

#[derive(Clone, Copy)]
pub struct StructDefinitionStruct<'a> {
    pub(crate) ir_node: &'a ir::StructDefinition,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_struct_definition<'a>(
    ir_node: &'a ir::StructDefinition,
    semantic: &'a SemanticContext,
) -> StructDefinition<'a> {
    StructDefinitionStruct { ir_node, semantic }
}

impl<'a> StructDefinitionStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn name(&self) -> Identifier<'a> {
        create_identifier(&self.ir_node.name, self.semantic)
    }

    pub fn members(&self) -> StructMembers<'a> {
        create_struct_members(&self.ir_node.members, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type StructMember<'a> = StructMemberStruct<'a>;

#[derive(Clone, Copy)]
pub struct StructMemberStruct<'a> {
    pub(crate) ir_node: &'a ir::StructMember,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_struct_member<'a>(
    ir_node: &'a ir::StructMember,
    semantic: &'a SemanticContext,
) -> StructMember<'a> {
    StructMemberStruct { ir_node, semantic }
}

impl<'a> StructMemberStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn type_name(&self) -> TypeName<'a> {
        create_type_name(&self.ir_node.type_name, self.semantic)
    }

    pub fn name(&self) -> Identifier<'a> {
        create_identifier(&self.ir_node.name, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type TryStatement<'a> = TryStatementStruct<'a>;

#[derive(Clone, Copy)]
pub struct TryStatementStruct<'a> {
    pub(crate) ir_node: &'a ir::TryStatement,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_try_statement<'a>(
    ir_node: &'a ir::TryStatement,
    semantic: &'a SemanticContext,
) -> TryStatement<'a> {
    TryStatementStruct { ir_node, semantic }
}

impl<'a> TryStatementStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn expression(&self) -> Expression<'a> {
        create_expression(&self.ir_node.expression, self.semantic)
    }

    pub fn returns(&self) -> Option<Parameters<'a>> {
        self.ir_node
            .returns
            .as_ref()
            .map(|ir_node| create_parameters(ir_node, self.semantic))
    }

    pub fn body(&self) -> Block<'a> {
        create_block(&self.ir_node.body, self.semantic)
    }

    pub fn catch_clauses(&self) -> CatchClauses<'a> {
        create_catch_clauses(&self.ir_node.catch_clauses, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type TupleExpression<'a> = TupleExpressionStruct<'a>;

#[derive(Clone, Copy)]
pub struct TupleExpressionStruct<'a> {
    pub(crate) ir_node: &'a ir::TupleExpression,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_tuple_expression<'a>(
    ir_node: &'a ir::TupleExpression,
    semantic: &'a SemanticContext,
) -> TupleExpression<'a> {
    TupleExpressionStruct { ir_node, semantic }
}

impl<'a> TupleExpressionStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn items(&self) -> TupleValues<'a> {
        create_tuple_values(&self.ir_node.items, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type TupleValue<'a> = TupleValueStruct<'a>;

#[derive(Clone, Copy)]
pub struct TupleValueStruct<'a> {
    pub(crate) ir_node: &'a ir::TupleValue,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_tuple_value<'a>(
    ir_node: &'a ir::TupleValue,
    semantic: &'a SemanticContext,
) -> TupleValue<'a> {
    TupleValueStruct { ir_node, semantic }
}

impl<'a> TupleValueStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn expression(&self) -> Option<Expression<'a>> {
        self.ir_node
            .expression
            .as_ref()
            .map(|ir_node| create_expression(ir_node, self.semantic))
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type TypeExpression<'a> = TypeExpressionStruct<'a>;

#[derive(Clone, Copy)]
pub struct TypeExpressionStruct<'a> {
    pub(crate) ir_node: &'a ir::TypeExpression,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_type_expression<'a>(
    ir_node: &'a ir::TypeExpression,
    semantic: &'a SemanticContext,
) -> TypeExpression<'a> {
    TypeExpressionStruct { ir_node, semantic }
}

impl<'a> TypeExpressionStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn type_name(&self) -> TypeName<'a> {
        create_type_name(&self.ir_node.type_name, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type UncheckedBlock<'a> = UncheckedBlockStruct<'a>;

#[derive(Clone, Copy)]
pub struct UncheckedBlockStruct<'a> {
    pub(crate) ir_node: &'a ir::UncheckedBlock,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_unchecked_block<'a>(
    ir_node: &'a ir::UncheckedBlock,
    semantic: &'a SemanticContext,
) -> UncheckedBlock<'a> {
    UncheckedBlockStruct { ir_node, semantic }
}

impl<'a> UncheckedBlockStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn block(&self) -> Block<'a> {
        create_block(&self.ir_node.block, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type UserDefinedValueTypeDefinition<'a> = UserDefinedValueTypeDefinitionStruct<'a>;

#[derive(Clone, Copy)]
pub struct UserDefinedValueTypeDefinitionStruct<'a> {
    pub(crate) ir_node: &'a ir::UserDefinedValueTypeDefinition,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_user_defined_value_type_definition<'a>(
    ir_node: &'a ir::UserDefinedValueTypeDefinition,
    semantic: &'a SemanticContext,
) -> UserDefinedValueTypeDefinition<'a> {
    UserDefinedValueTypeDefinitionStruct { ir_node, semantic }
}

impl<'a> UserDefinedValueTypeDefinitionStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn name(&self) -> Identifier<'a> {
        create_identifier(&self.ir_node.name, self.semantic)
    }

    pub fn value_type(&self) -> ElementaryType<'a> {
        create_elementary_type(&self.ir_node.value_type, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type UsingDeconstruction<'a> = UsingDeconstructionStruct<'a>;

#[derive(Clone, Copy)]
pub struct UsingDeconstructionStruct<'a> {
    pub(crate) ir_node: &'a ir::UsingDeconstruction,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_using_deconstruction<'a>(
    ir_node: &'a ir::UsingDeconstruction,
    semantic: &'a SemanticContext,
) -> UsingDeconstruction<'a> {
    UsingDeconstructionStruct { ir_node, semantic }
}

impl<'a> UsingDeconstructionStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn symbols(&self) -> UsingDeconstructionSymbols<'a> {
        create_using_deconstruction_symbols(&self.ir_node.symbols, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type UsingDeconstructionSymbol<'a> = UsingDeconstructionSymbolStruct<'a>;

#[derive(Clone, Copy)]
pub struct UsingDeconstructionSymbolStruct<'a> {
    pub(crate) ir_node: &'a ir::UsingDeconstructionSymbol,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_using_deconstruction_symbol<'a>(
    ir_node: &'a ir::UsingDeconstructionSymbol,
    semantic: &'a SemanticContext,
) -> UsingDeconstructionSymbol<'a> {
    UsingDeconstructionSymbolStruct { ir_node, semantic }
}

impl<'a> UsingDeconstructionSymbolStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn name(&self) -> IdentifierPath<'a> {
        create_identifier_path(&self.ir_node.name, self.semantic)
    }

    pub fn alias(&self) -> Option<UsingOperator<'a>> {
        self.ir_node
            .alias
            .as_ref()
            .map(|ir_node| create_using_operator(ir_node, self.semantic))
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type UsingDirective<'a> = UsingDirectiveStruct<'a>;

#[derive(Clone, Copy)]
pub struct UsingDirectiveStruct<'a> {
    pub(crate) ir_node: &'a ir::UsingDirective,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_using_directive<'a>(
    ir_node: &'a ir::UsingDirective,
    semantic: &'a SemanticContext,
) -> UsingDirective<'a> {
    UsingDirectiveStruct { ir_node, semantic }
}

impl<'a> UsingDirectiveStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn clause(&self) -> UsingClause<'a> {
        create_using_clause(&self.ir_node.clause, self.semantic)
    }

    pub fn target(&self) -> UsingTarget<'a> {
        create_using_target(&self.ir_node.target, self.semantic)
    }

    pub fn is_global(&self) -> bool {
        self.ir_node.is_global
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type VariableDeclaration<'a> = VariableDeclarationStruct<'a>;

#[derive(Clone, Copy)]
pub struct VariableDeclarationStruct<'a> {
    pub(crate) ir_node: &'a ir::VariableDeclaration,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_variable_declaration<'a>(
    ir_node: &'a ir::VariableDeclaration,
    semantic: &'a SemanticContext,
) -> VariableDeclaration<'a> {
    VariableDeclarationStruct { ir_node, semantic }
}

impl<'a> VariableDeclarationStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn type_name(&self) -> TypeName<'a> {
        create_type_name(&self.ir_node.type_name, self.semantic)
    }

    pub fn storage_location(&self) -> Option<StorageLocation<'a>> {
        self.ir_node
            .storage_location
            .as_ref()
            .map(|ir_node| create_storage_location(ir_node, self.semantic))
    }

    pub fn name(&self) -> Identifier<'a> {
        create_identifier(&self.ir_node.name, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type VariableDeclarationStatement<'a> = VariableDeclarationStatementStruct<'a>;

#[derive(Clone, Copy)]
pub struct VariableDeclarationStatementStruct<'a> {
    pub(crate) ir_node: &'a ir::VariableDeclarationStatement,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_variable_declaration_statement<'a>(
    ir_node: &'a ir::VariableDeclarationStatement,
    semantic: &'a SemanticContext,
) -> VariableDeclarationStatement<'a> {
    VariableDeclarationStatementStruct { ir_node, semantic }
}

impl<'a> VariableDeclarationStatementStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn target(&self) -> VariableDeclarationTarget<'a> {
        create_variable_declaration_target(&self.ir_node.target, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type VersionPragma<'a> = VersionPragmaStruct<'a>;

#[derive(Clone, Copy)]
pub struct VersionPragmaStruct<'a> {
    pub(crate) ir_node: &'a ir::VersionPragma,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_version_pragma<'a>(
    ir_node: &'a ir::VersionPragma,
    semantic: &'a SemanticContext,
) -> VersionPragma<'a> {
    VersionPragmaStruct { ir_node, semantic }
}

impl<'a> VersionPragmaStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn sets(&self) -> VersionPragmaExpressionSets<'a> {
        create_version_pragma_expression_sets(&self.ir_node.sets, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type VersionPragmaComparator<'a> = VersionPragmaComparatorStruct<'a>;

#[derive(Clone, Copy)]
pub struct VersionPragmaComparatorStruct<'a> {
    pub(crate) ir_node: &'a ir::VersionPragmaComparator,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_version_pragma_comparator<'a>(
    ir_node: &'a ir::VersionPragmaComparator,
    semantic: &'a SemanticContext,
) -> VersionPragmaComparator<'a> {
    VersionPragmaComparatorStruct { ir_node, semantic }
}

impl<'a> VersionPragmaComparatorStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn operator(&self) -> VersionPragmaOperator {
        create_version_pragma_operator(&self.ir_node.operator, self.semantic)
    }

    pub fn specifier(&self) -> VersionPragmaSpecifier<'a> {
        create_version_pragma_specifier(&self.ir_node.specifier, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type WhileStatement<'a> = WhileStatementStruct<'a>;

#[derive(Clone, Copy)]
pub struct WhileStatementStruct<'a> {
    pub(crate) ir_node: &'a ir::WhileStatement,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_while_statement<'a>(
    ir_node: &'a ir::WhileStatement,
    semantic: &'a SemanticContext,
) -> WhileStatement<'a> {
    WhileStatementStruct { ir_node, semantic }
}

impl<'a> WhileStatementStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn condition(&self) -> Expression<'a> {
        create_expression(&self.ir_node.condition, self.semantic)
    }

    pub fn body(&self) -> Statement<'a> {
        create_statement(&self.ir_node.body, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type YulBlock<'a> = YulBlockStruct<'a>;

#[derive(Clone, Copy)]
pub struct YulBlockStruct<'a> {
    pub(crate) ir_node: &'a ir::YulBlock,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_yul_block<'a>(
    ir_node: &'a ir::YulBlock,
    semantic: &'a SemanticContext,
) -> YulBlock<'a> {
    YulBlockStruct { ir_node, semantic }
}

impl<'a> YulBlockStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn statements(&self) -> YulStatements<'a> {
        create_yul_statements(&self.ir_node.statements, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type YulBreakStatement<'a> = YulBreakStatementStruct<'a>;

#[derive(Clone, Copy)]
pub struct YulBreakStatementStruct<'a> {
    pub(crate) ir_node: &'a ir::YulBreakStatement,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_yul_break_statement<'a>(
    ir_node: &'a ir::YulBreakStatement,
    semantic: &'a SemanticContext,
) -> YulBreakStatement<'a> {
    YulBreakStatementStruct { ir_node, semantic }
}

impl<'a> YulBreakStatementStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type YulContinueStatement<'a> = YulContinueStatementStruct<'a>;

#[derive(Clone, Copy)]
pub struct YulContinueStatementStruct<'a> {
    pub(crate) ir_node: &'a ir::YulContinueStatement,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_yul_continue_statement<'a>(
    ir_node: &'a ir::YulContinueStatement,
    semantic: &'a SemanticContext,
) -> YulContinueStatement<'a> {
    YulContinueStatementStruct { ir_node, semantic }
}

impl<'a> YulContinueStatementStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type YulDefaultCase<'a> = YulDefaultCaseStruct<'a>;

#[derive(Clone, Copy)]
pub struct YulDefaultCaseStruct<'a> {
    pub(crate) ir_node: &'a ir::YulDefaultCase,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_yul_default_case<'a>(
    ir_node: &'a ir::YulDefaultCase,
    semantic: &'a SemanticContext,
) -> YulDefaultCase<'a> {
    YulDefaultCaseStruct { ir_node, semantic }
}

impl<'a> YulDefaultCaseStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn body(&self) -> YulBlock<'a> {
        create_yul_block(&self.ir_node.body, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type YulForStatement<'a> = YulForStatementStruct<'a>;

#[derive(Clone, Copy)]
pub struct YulForStatementStruct<'a> {
    pub(crate) ir_node: &'a ir::YulForStatement,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_yul_for_statement<'a>(
    ir_node: &'a ir::YulForStatement,
    semantic: &'a SemanticContext,
) -> YulForStatement<'a> {
    YulForStatementStruct { ir_node, semantic }
}

impl<'a> YulForStatementStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn initialization(&self) -> YulBlock<'a> {
        create_yul_block(&self.ir_node.initialization, self.semantic)
    }

    pub fn condition(&self) -> YulExpression<'a> {
        create_yul_expression(&self.ir_node.condition, self.semantic)
    }

    pub fn iterator(&self) -> YulBlock<'a> {
        create_yul_block(&self.ir_node.iterator, self.semantic)
    }

    pub fn body(&self) -> YulBlock<'a> {
        create_yul_block(&self.ir_node.body, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type YulFunctionCallExpression<'a> = YulFunctionCallExpressionStruct<'a>;

#[derive(Clone, Copy)]
pub struct YulFunctionCallExpressionStruct<'a> {
    pub(crate) ir_node: &'a ir::YulFunctionCallExpression,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_yul_function_call_expression<'a>(
    ir_node: &'a ir::YulFunctionCallExpression,
    semantic: &'a SemanticContext,
) -> YulFunctionCallExpression<'a> {
    YulFunctionCallExpressionStruct { ir_node, semantic }
}

impl<'a> YulFunctionCallExpressionStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn operand(&self) -> YulExpression<'a> {
        create_yul_expression(&self.ir_node.operand, self.semantic)
    }

    pub fn arguments(&self) -> YulArguments<'a> {
        create_yul_arguments(&self.ir_node.arguments, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type YulFunctionDefinition<'a> = YulFunctionDefinitionStruct<'a>;

#[derive(Clone, Copy)]
pub struct YulFunctionDefinitionStruct<'a> {
    pub(crate) ir_node: &'a ir::YulFunctionDefinition,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_yul_function_definition<'a>(
    ir_node: &'a ir::YulFunctionDefinition,
    semantic: &'a SemanticContext,
) -> YulFunctionDefinition<'a> {
    YulFunctionDefinitionStruct { ir_node, semantic }
}

impl<'a> YulFunctionDefinitionStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn name(&self) -> Identifier<'a> {
        create_identifier(&self.ir_node.name, self.semantic)
    }

    pub fn parameters(&self) -> YulParameters<'a> {
        create_yul_parameters(&self.ir_node.parameters, self.semantic)
    }

    pub fn returns(&self) -> Option<YulVariableNames<'a>> {
        self.ir_node
            .returns
            .as_ref()
            .map(|ir_node| create_yul_variable_names(ir_node, self.semantic))
    }

    pub fn body(&self) -> YulBlock<'a> {
        create_yul_block(&self.ir_node.body, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type YulIfStatement<'a> = YulIfStatementStruct<'a>;

#[derive(Clone, Copy)]
pub struct YulIfStatementStruct<'a> {
    pub(crate) ir_node: &'a ir::YulIfStatement,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_yul_if_statement<'a>(
    ir_node: &'a ir::YulIfStatement,
    semantic: &'a SemanticContext,
) -> YulIfStatement<'a> {
    YulIfStatementStruct { ir_node, semantic }
}

impl<'a> YulIfStatementStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn condition(&self) -> YulExpression<'a> {
        create_yul_expression(&self.ir_node.condition, self.semantic)
    }

    pub fn body(&self) -> YulBlock<'a> {
        create_yul_block(&self.ir_node.body, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type YulLeaveStatement<'a> = YulLeaveStatementStruct<'a>;

#[derive(Clone, Copy)]
pub struct YulLeaveStatementStruct<'a> {
    pub(crate) ir_node: &'a ir::YulLeaveStatement,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_yul_leave_statement<'a>(
    ir_node: &'a ir::YulLeaveStatement,
    semantic: &'a SemanticContext,
) -> YulLeaveStatement<'a> {
    YulLeaveStatementStruct { ir_node, semantic }
}

impl<'a> YulLeaveStatementStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type YulSwitchStatement<'a> = YulSwitchStatementStruct<'a>;

#[derive(Clone, Copy)]
pub struct YulSwitchStatementStruct<'a> {
    pub(crate) ir_node: &'a ir::YulSwitchStatement,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_yul_switch_statement<'a>(
    ir_node: &'a ir::YulSwitchStatement,
    semantic: &'a SemanticContext,
) -> YulSwitchStatement<'a> {
    YulSwitchStatementStruct { ir_node, semantic }
}

impl<'a> YulSwitchStatementStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn expression(&self) -> YulExpression<'a> {
        create_yul_expression(&self.ir_node.expression, self.semantic)
    }

    pub fn value_cases(&self) -> YulValueCases<'a> {
        create_yul_value_cases(&self.ir_node.value_cases, self.semantic)
    }

    pub fn default_case(&self) -> Option<YulDefaultCase<'a>> {
        self.ir_node
            .default_case
            .as_ref()
            .map(|ir_node| create_yul_default_case(ir_node, self.semantic))
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type YulValueCase<'a> = YulValueCaseStruct<'a>;

#[derive(Clone, Copy)]
pub struct YulValueCaseStruct<'a> {
    pub(crate) ir_node: &'a ir::YulValueCase,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_yul_value_case<'a>(
    ir_node: &'a ir::YulValueCase,
    semantic: &'a SemanticContext,
) -> YulValueCase<'a> {
    YulValueCaseStruct { ir_node, semantic }
}

impl<'a> YulValueCaseStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn value(&self) -> YulLiteral<'a> {
        create_yul_literal(&self.ir_node.value, self.semantic)
    }

    pub fn body(&self) -> YulBlock<'a> {
        create_yul_block(&self.ir_node.body, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type YulVariableAssignmentStatement<'a> = YulVariableAssignmentStatementStruct<'a>;

#[derive(Clone, Copy)]
pub struct YulVariableAssignmentStatementStruct<'a> {
    pub(crate) ir_node: &'a ir::YulVariableAssignmentStatement,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_yul_variable_assignment_statement<'a>(
    ir_node: &'a ir::YulVariableAssignmentStatement,
    semantic: &'a SemanticContext,
) -> YulVariableAssignmentStatement<'a> {
    YulVariableAssignmentStatementStruct { ir_node, semantic }
}

impl<'a> YulVariableAssignmentStatementStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn variables(&self) -> YulPaths<'a> {
        create_yul_paths(&self.ir_node.variables, self.semantic)
    }

    pub fn expression(&self) -> YulExpression<'a> {
        create_yul_expression(&self.ir_node.expression, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type YulVariableDeclarationStatement<'a> = YulVariableDeclarationStatementStruct<'a>;

#[derive(Clone, Copy)]
pub struct YulVariableDeclarationStatementStruct<'a> {
    pub(crate) ir_node: &'a ir::YulVariableDeclarationStatement,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_yul_variable_declaration_statement<'a>(
    ir_node: &'a ir::YulVariableDeclarationStatement,
    semantic: &'a SemanticContext,
) -> YulVariableDeclarationStatement<'a> {
    YulVariableDeclarationStatementStruct { ir_node, semantic }
}

impl<'a> YulVariableDeclarationStatementStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn variables(&self) -> YulVariableNames<'a> {
        create_yul_variable_names(&self.ir_node.variables, self.semantic)
    }

    pub fn value(&self) -> Option<YulVariableDeclarationValue<'a>> {
        self.ir_node
            .value
            .as_ref()
            .map(|ir_node| create_yul_variable_declaration_value(ir_node, self.semantic))
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type YulVariableDeclarationValue<'a> = YulVariableDeclarationValueStruct<'a>;

#[derive(Clone, Copy)]
pub struct YulVariableDeclarationValueStruct<'a> {
    pub(crate) ir_node: &'a ir::YulVariableDeclarationValue,
    pub(crate) semantic: &'a SemanticContext,
}

pub fn create_yul_variable_declaration_value<'a>(
    ir_node: &'a ir::YulVariableDeclarationValue,
    semantic: &'a SemanticContext,
) -> YulVariableDeclarationValue<'a> {
    YulVariableDeclarationValueStruct { ir_node, semantic }
}

impl<'a> YulVariableDeclarationValueStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn expression(&self) -> YulExpression<'a> {
        create_yul_expression(&self.ir_node.expression, self.semantic)
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

//
// Choices
//

#[derive(Clone, Copy)]
pub enum AbicoderVersion {
    V1,
    V2,
}

#[allow(clippy::clone_on_copy)]
#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_abicoder_version<'a>(
    ir_node: &'a ir::AbicoderVersion,
    semantic: &'a SemanticContext,
) -> AbicoderVersion {
    match ir_node {
        ir::AbicoderVersion::V1 => AbicoderVersion::V1,
        ir::AbicoderVersion::V2 => AbicoderVersion::V2,
    }
}

#[derive(Clone, Copy)]
pub enum AdditiveExpressionOperator<'a> {
    Minus(Minus<'a>),
    Plus(Plus<'a>),
}

#[allow(clippy::clone_on_copy)]
#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_additive_expression_operator<'a>(
    ir_node: &'a ir::AdditiveExpressionOperator,
    semantic: &'a SemanticContext,
) -> AdditiveExpressionOperator<'a> {
    match ir_node {
        ir::AdditiveExpressionOperator::Minus(variant) => {
            AdditiveExpressionOperator::Minus(create_minus(variant, semantic))
        }
        ir::AdditiveExpressionOperator::Plus(variant) => {
            AdditiveExpressionOperator::Plus(create_plus(variant, semantic))
        }
    }
}

#[derive(Clone, Copy)]
pub enum ArgumentsDeclaration<'a> {
    PositionalArguments(PositionalArguments<'a>),
    NamedArguments(NamedArguments<'a>),
}

#[allow(clippy::clone_on_copy)]
#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_arguments_declaration<'a>(
    ir_node: &'a ir::ArgumentsDeclaration,
    semantic: &'a SemanticContext,
) -> ArgumentsDeclaration<'a> {
    match ir_node {
        ir::ArgumentsDeclaration::PositionalArguments(nodes) => {
            ArgumentsDeclaration::PositionalArguments(create_positional_arguments(nodes, semantic))
        }
        ir::ArgumentsDeclaration::NamedArguments(nodes) => {
            ArgumentsDeclaration::NamedArguments(create_named_arguments(nodes, semantic))
        }
    }
}

#[derive(Clone, Copy)]
pub enum AssignmentExpressionOperator<'a> {
    AmpersandEqual(AmpersandEqual<'a>),
    AsteriskEqual(AsteriskEqual<'a>),
    BarEqual(BarEqual<'a>),
    CaretEqual(CaretEqual<'a>),
    Equal(Equal<'a>),
    GreaterThanGreaterThanEqual(GreaterThanGreaterThanEqual<'a>),
    GreaterThanGreaterThanGreaterThanEqual(GreaterThanGreaterThanGreaterThanEqual<'a>),
    LessThanLessThanEqual(LessThanLessThanEqual<'a>),
    MinusEqual(MinusEqual<'a>),
    PercentEqual(PercentEqual<'a>),
    PlusEqual(PlusEqual<'a>),
    SlashEqual(SlashEqual<'a>),
}

#[allow(clippy::clone_on_copy)]
#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_assignment_expression_operator<'a>(
    ir_node: &'a ir::AssignmentExpressionOperator,
    semantic: &'a SemanticContext,
) -> AssignmentExpressionOperator<'a> {
    match ir_node {
        ir::AssignmentExpressionOperator::AmpersandEqual(variant) => {
            AssignmentExpressionOperator::AmpersandEqual(create_ampersand_equal(variant, semantic))
        }
        ir::AssignmentExpressionOperator::AsteriskEqual(variant) => {
            AssignmentExpressionOperator::AsteriskEqual(create_asterisk_equal(variant, semantic))
        }
        ir::AssignmentExpressionOperator::BarEqual(variant) => {
            AssignmentExpressionOperator::BarEqual(create_bar_equal(variant, semantic))
        }
        ir::AssignmentExpressionOperator::CaretEqual(variant) => {
            AssignmentExpressionOperator::CaretEqual(create_caret_equal(variant, semantic))
        }
        ir::AssignmentExpressionOperator::Equal(variant) => {
            AssignmentExpressionOperator::Equal(create_equal(variant, semantic))
        }
        ir::AssignmentExpressionOperator::GreaterThanGreaterThanEqual(variant) => {
            AssignmentExpressionOperator::GreaterThanGreaterThanEqual(
                create_greater_than_greater_than_equal(variant, semantic),
            )
        }
        ir::AssignmentExpressionOperator::GreaterThanGreaterThanGreaterThanEqual(variant) => {
            AssignmentExpressionOperator::GreaterThanGreaterThanGreaterThanEqual(
                create_greater_than_greater_than_greater_than_equal(variant, semantic),
            )
        }
        ir::AssignmentExpressionOperator::LessThanLessThanEqual(variant) => {
            AssignmentExpressionOperator::LessThanLessThanEqual(create_less_than_less_than_equal(
                variant, semantic,
            ))
        }
        ir::AssignmentExpressionOperator::MinusEqual(variant) => {
            AssignmentExpressionOperator::MinusEqual(create_minus_equal(variant, semantic))
        }
        ir::AssignmentExpressionOperator::PercentEqual(variant) => {
            AssignmentExpressionOperator::PercentEqual(create_percent_equal(variant, semantic))
        }
        ir::AssignmentExpressionOperator::PlusEqual(variant) => {
            AssignmentExpressionOperator::PlusEqual(create_plus_equal(variant, semantic))
        }
        ir::AssignmentExpressionOperator::SlashEqual(variant) => {
            AssignmentExpressionOperator::SlashEqual(create_slash_equal(variant, semantic))
        }
    }
}

#[derive(Clone, Copy)]
pub enum CatchClauseKind {
    Error,
    Panic,
    LowLevel,
}

#[allow(clippy::clone_on_copy)]
#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_catch_clause_kind<'a>(
    ir_node: &'a ir::CatchClauseKind,
    semantic: &'a SemanticContext,
) -> CatchClauseKind {
    match ir_node {
        ir::CatchClauseKind::Error => CatchClauseKind::Error,
        ir::CatchClauseKind::Panic => CatchClauseKind::Panic,
        ir::CatchClauseKind::LowLevel => CatchClauseKind::LowLevel,
    }
}

#[derive(Clone, Copy)]
pub enum ContractMember<'a> {
    UsingDirective(UsingDirective<'a>),
    FunctionDefinition(FunctionDefinition<'a>),
    ModifierDefinition(ModifierDefinition<'a>),
    StructDefinition(StructDefinition<'a>),
    EnumDefinition(EnumDefinition<'a>),
    EventDefinition(EventDefinition<'a>),
    ErrorDefinition(ErrorDefinition<'a>),
    UserDefinedValueTypeDefinition(UserDefinedValueTypeDefinition<'a>),
    StateVariableDefinition(StateVariableDefinition<'a>),
    ConstantDefinition(ConstantDefinition<'a>),
}

#[allow(clippy::clone_on_copy)]
#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_contract_member<'a>(
    ir_node: &'a ir::ContractMember,
    semantic: &'a SemanticContext,
) -> ContractMember<'a> {
    match ir_node {
        ir::ContractMember::UsingDirective(variant) => {
            ContractMember::UsingDirective(create_using_directive(variant, semantic))
        }
        ir::ContractMember::FunctionDefinition(variant) => {
            ContractMember::FunctionDefinition(create_function_definition(variant, semantic))
        }
        ir::ContractMember::ModifierDefinition(variant) => {
            ContractMember::ModifierDefinition(create_modifier_definition(variant, semantic))
        }
        ir::ContractMember::StructDefinition(variant) => {
            ContractMember::StructDefinition(create_struct_definition(variant, semantic))
        }
        ir::ContractMember::EnumDefinition(variant) => {
            ContractMember::EnumDefinition(create_enum_definition(variant, semantic))
        }
        ir::ContractMember::EventDefinition(variant) => {
            ContractMember::EventDefinition(create_event_definition(variant, semantic))
        }
        ir::ContractMember::ErrorDefinition(variant) => {
            ContractMember::ErrorDefinition(create_error_definition(variant, semantic))
        }
        ir::ContractMember::UserDefinedValueTypeDefinition(variant) => {
            ContractMember::UserDefinedValueTypeDefinition(
                create_user_defined_value_type_definition(variant, semantic),
            )
        }
        ir::ContractMember::StateVariableDefinition(variant) => {
            ContractMember::StateVariableDefinition(create_state_variable_definition(
                variant, semantic,
            ))
        }
        ir::ContractMember::ConstantDefinition(variant) => {
            ContractMember::ConstantDefinition(create_constant_definition(variant, semantic))
        }
    }
}

#[derive(Clone, Copy)]
pub enum ElementaryType<'a> {
    BoolKeyword(BoolKeyword<'a>),
    StringKeyword(StringKeyword<'a>),
    AddressType(AddressType<'a>),
    BytesKeyword(BytesKeyword<'a>),
    IntKeyword(IntKeyword<'a>),
    UintKeyword(UintKeyword<'a>),
    FixedKeyword(FixedKeyword<'a>),
    UfixedKeyword(UfixedKeyword<'a>),
}

#[allow(clippy::clone_on_copy)]
#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_elementary_type<'a>(
    ir_node: &'a ir::ElementaryType,
    semantic: &'a SemanticContext,
) -> ElementaryType<'a> {
    match ir_node {
        ir::ElementaryType::BoolKeyword(variant) => {
            ElementaryType::BoolKeyword(create_bool_keyword(variant, semantic))
        }
        ir::ElementaryType::StringKeyword(variant) => {
            ElementaryType::StringKeyword(create_string_keyword(variant, semantic))
        }
        ir::ElementaryType::AddressType(variant) => {
            ElementaryType::AddressType(create_address_type(variant, semantic))
        }
        ir::ElementaryType::BytesKeyword(variant) => {
            ElementaryType::BytesKeyword(create_bytes_keyword(variant, semantic))
        }
        ir::ElementaryType::IntKeyword(variant) => {
            ElementaryType::IntKeyword(create_int_keyword(variant, semantic))
        }
        ir::ElementaryType::UintKeyword(variant) => {
            ElementaryType::UintKeyword(create_uint_keyword(variant, semantic))
        }
        ir::ElementaryType::FixedKeyword(variant) => {
            ElementaryType::FixedKeyword(create_fixed_keyword(variant, semantic))
        }
        ir::ElementaryType::UfixedKeyword(variant) => {
            ElementaryType::UfixedKeyword(create_ufixed_keyword(variant, semantic))
        }
    }
}

#[derive(Clone, Copy)]
pub enum EqualityExpressionOperator<'a> {
    BangEqual(BangEqual<'a>),
    EqualEqual(EqualEqual<'a>),
}

#[allow(clippy::clone_on_copy)]
#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_equality_expression_operator<'a>(
    ir_node: &'a ir::EqualityExpressionOperator,
    semantic: &'a SemanticContext,
) -> EqualityExpressionOperator<'a> {
    match ir_node {
        ir::EqualityExpressionOperator::BangEqual(variant) => {
            EqualityExpressionOperator::BangEqual(create_bang_equal(variant, semantic))
        }
        ir::EqualityExpressionOperator::EqualEqual(variant) => {
            EqualityExpressionOperator::EqualEqual(create_equal_equal(variant, semantic))
        }
    }
}

#[derive(Clone, Copy)]
pub enum ExperimentalFeature {
    ABIEncoderV2,
    SMTChecker,
    Solidity,
    Unrecognized,
}

#[allow(clippy::clone_on_copy)]
#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_experimental_feature<'a>(
    ir_node: &'a ir::ExperimentalFeature,
    semantic: &'a SemanticContext,
) -> ExperimentalFeature {
    match ir_node {
        ir::ExperimentalFeature::ABIEncoderV2 => ExperimentalFeature::ABIEncoderV2,
        ir::ExperimentalFeature::SMTChecker => ExperimentalFeature::SMTChecker,
        ir::ExperimentalFeature::Solidity => ExperimentalFeature::Solidity,
        ir::ExperimentalFeature::Unrecognized => ExperimentalFeature::Unrecognized,
    }
}

#[derive(Clone, Copy)]
pub enum Expression<'a> {
    AssignmentExpression(AssignmentExpression<'a>),
    ConditionalExpression(ConditionalExpression<'a>),
    OrExpression(OrExpression<'a>),
    AndExpression(AndExpression<'a>),
    EqualityExpression(EqualityExpression<'a>),
    InequalityExpression(InequalityExpression<'a>),
    BitwiseOrExpression(BitwiseOrExpression<'a>),
    BitwiseXorExpression(BitwiseXorExpression<'a>),
    BitwiseAndExpression(BitwiseAndExpression<'a>),
    ShiftExpression(ShiftExpression<'a>),
    AdditiveExpression(AdditiveExpression<'a>),
    MultiplicativeExpression(MultiplicativeExpression<'a>),
    ExponentiationExpression(ExponentiationExpression<'a>),
    PostfixExpression(PostfixExpression<'a>),
    PrefixExpression(PrefixExpression<'a>),
    FunctionCallExpression(FunctionCallExpression<'a>),
    CallOptionsExpression(CallOptionsExpression<'a>),
    MemberAccessExpression(MemberAccessExpression<'a>),
    IndexAccessExpression(IndexAccessExpression<'a>),
    NewExpression(NewExpression<'a>),
    TupleExpression(TupleExpression<'a>),
    TypeExpression(TypeExpression<'a>),
    ArrayExpression(ArrayExpression<'a>),
    HexNumberExpression(HexNumberExpression<'a>),
    DecimalNumberExpression(DecimalNumberExpression<'a>),
    StringExpression(StringExpression<'a>),
    ElementaryType(ElementaryType<'a>),
    PayableKeyword(PayableKeyword<'a>),
    ThisKeyword(ThisKeyword<'a>),
    SuperKeyword(SuperKeyword<'a>),
    TrueKeyword(TrueKeyword<'a>),
    FalseKeyword(FalseKeyword<'a>),
    Identifier(Identifier<'a>),
}

#[allow(clippy::clone_on_copy)]
#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_expression<'a>(
    ir_node: &'a ir::Expression,
    semantic: &'a SemanticContext,
) -> Expression<'a> {
    match ir_node {
        ir::Expression::AssignmentExpression(variant) => {
            Expression::AssignmentExpression(create_assignment_expression(variant, semantic))
        }
        ir::Expression::ConditionalExpression(variant) => {
            Expression::ConditionalExpression(create_conditional_expression(variant, semantic))
        }
        ir::Expression::OrExpression(variant) => {
            Expression::OrExpression(create_or_expression(variant, semantic))
        }
        ir::Expression::AndExpression(variant) => {
            Expression::AndExpression(create_and_expression(variant, semantic))
        }
        ir::Expression::EqualityExpression(variant) => {
            Expression::EqualityExpression(create_equality_expression(variant, semantic))
        }
        ir::Expression::InequalityExpression(variant) => {
            Expression::InequalityExpression(create_inequality_expression(variant, semantic))
        }
        ir::Expression::BitwiseOrExpression(variant) => {
            Expression::BitwiseOrExpression(create_bitwise_or_expression(variant, semantic))
        }
        ir::Expression::BitwiseXorExpression(variant) => {
            Expression::BitwiseXorExpression(create_bitwise_xor_expression(variant, semantic))
        }
        ir::Expression::BitwiseAndExpression(variant) => {
            Expression::BitwiseAndExpression(create_bitwise_and_expression(variant, semantic))
        }
        ir::Expression::ShiftExpression(variant) => {
            Expression::ShiftExpression(create_shift_expression(variant, semantic))
        }
        ir::Expression::AdditiveExpression(variant) => {
            Expression::AdditiveExpression(create_additive_expression(variant, semantic))
        }
        ir::Expression::MultiplicativeExpression(variant) => Expression::MultiplicativeExpression(
            create_multiplicative_expression(variant, semantic),
        ),
        ir::Expression::ExponentiationExpression(variant) => Expression::ExponentiationExpression(
            create_exponentiation_expression(variant, semantic),
        ),
        ir::Expression::PostfixExpression(variant) => {
            Expression::PostfixExpression(create_postfix_expression(variant, semantic))
        }
        ir::Expression::PrefixExpression(variant) => {
            Expression::PrefixExpression(create_prefix_expression(variant, semantic))
        }
        ir::Expression::FunctionCallExpression(variant) => {
            Expression::FunctionCallExpression(create_function_call_expression(variant, semantic))
        }
        ir::Expression::CallOptionsExpression(variant) => {
            Expression::CallOptionsExpression(create_call_options_expression(variant, semantic))
        }
        ir::Expression::MemberAccessExpression(variant) => {
            Expression::MemberAccessExpression(create_member_access_expression(variant, semantic))
        }
        ir::Expression::IndexAccessExpression(variant) => {
            Expression::IndexAccessExpression(create_index_access_expression(variant, semantic))
        }
        ir::Expression::NewExpression(variant) => {
            Expression::NewExpression(create_new_expression(variant, semantic))
        }
        ir::Expression::TupleExpression(variant) => {
            Expression::TupleExpression(create_tuple_expression(variant, semantic))
        }
        ir::Expression::TypeExpression(variant) => {
            Expression::TypeExpression(create_type_expression(variant, semantic))
        }
        ir::Expression::ArrayExpression(variant) => {
            Expression::ArrayExpression(create_array_expression(variant, semantic))
        }
        ir::Expression::HexNumberExpression(variant) => {
            Expression::HexNumberExpression(create_hex_number_expression(variant, semantic))
        }
        ir::Expression::DecimalNumberExpression(variant) => {
            Expression::DecimalNumberExpression(create_decimal_number_expression(variant, semantic))
        }
        ir::Expression::StringExpression(variant) => {
            Expression::StringExpression(create_string_expression(variant, semantic))
        }
        ir::Expression::ElementaryType(variant) => {
            Expression::ElementaryType(create_elementary_type(variant, semantic))
        }
        ir::Expression::PayableKeyword(variant) => {
            Expression::PayableKeyword(create_payable_keyword(variant, semantic))
        }
        ir::Expression::ThisKeyword(variant) => {
            Expression::ThisKeyword(create_this_keyword(variant, semantic))
        }
        ir::Expression::SuperKeyword(variant) => {
            Expression::SuperKeyword(create_super_keyword(variant, semantic))
        }
        ir::Expression::TrueKeyword(variant) => {
            Expression::TrueKeyword(create_true_keyword(variant, semantic))
        }
        ir::Expression::FalseKeyword(variant) => {
            Expression::FalseKeyword(create_false_keyword(variant, semantic))
        }
        ir::Expression::Identifier(variant) => {
            Expression::Identifier(create_identifier(variant, semantic))
        }
    }
}

#[derive(Clone, Copy)]
pub enum ForStatementCondition<'a> {
    ExpressionStatement(ExpressionStatement<'a>),
    Semicolon(Semicolon<'a>),
}

#[allow(clippy::clone_on_copy)]
#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_for_statement_condition<'a>(
    ir_node: &'a ir::ForStatementCondition,
    semantic: &'a SemanticContext,
) -> ForStatementCondition<'a> {
    match ir_node {
        ir::ForStatementCondition::ExpressionStatement(variant) => {
            ForStatementCondition::ExpressionStatement(create_expression_statement(
                variant, semantic,
            ))
        }
        ir::ForStatementCondition::Semicolon(variant) => {
            ForStatementCondition::Semicolon(create_semicolon(variant, semantic))
        }
    }
}

#[derive(Clone, Copy)]
pub enum ForStatementInitialization<'a> {
    VariableDeclarationStatement(VariableDeclarationStatement<'a>),
    ExpressionStatement(ExpressionStatement<'a>),
    Semicolon(Semicolon<'a>),
}

#[allow(clippy::clone_on_copy)]
#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_for_statement_initialization<'a>(
    ir_node: &'a ir::ForStatementInitialization,
    semantic: &'a SemanticContext,
) -> ForStatementInitialization<'a> {
    match ir_node {
        ir::ForStatementInitialization::VariableDeclarationStatement(variant) => {
            ForStatementInitialization::VariableDeclarationStatement(
                create_variable_declaration_statement(variant, semantic),
            )
        }
        ir::ForStatementInitialization::ExpressionStatement(variant) => {
            ForStatementInitialization::ExpressionStatement(create_expression_statement(
                variant, semantic,
            ))
        }
        ir::ForStatementInitialization::Semicolon(variant) => {
            ForStatementInitialization::Semicolon(create_semicolon(variant, semantic))
        }
    }
}

#[derive(Clone, Copy)]
pub enum FunctionKind {
    Regular,
    Constructor,
    Fallback,
    Receive,
}

#[allow(clippy::clone_on_copy)]
#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_function_kind<'a>(
    ir_node: &'a ir::FunctionKind,
    semantic: &'a SemanticContext,
) -> FunctionKind {
    match ir_node {
        ir::FunctionKind::Regular => FunctionKind::Regular,
        ir::FunctionKind::Constructor => FunctionKind::Constructor,
        ir::FunctionKind::Fallback => FunctionKind::Fallback,
        ir::FunctionKind::Receive => FunctionKind::Receive,
    }
}

#[derive(Clone, Copy)]
pub enum FunctionMutability {
    Pure,
    View,
    NonPayable,
    Payable,
}

#[allow(clippy::clone_on_copy)]
#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_function_mutability<'a>(
    ir_node: &'a ir::FunctionMutability,
    semantic: &'a SemanticContext,
) -> FunctionMutability {
    match ir_node {
        ir::FunctionMutability::Pure => FunctionMutability::Pure,
        ir::FunctionMutability::View => FunctionMutability::View,
        ir::FunctionMutability::NonPayable => FunctionMutability::NonPayable,
        ir::FunctionMutability::Payable => FunctionMutability::Payable,
    }
}

#[derive(Clone, Copy)]
pub enum FunctionVisibility {
    Public,
    Private,
    Internal,
    External,
}

#[allow(clippy::clone_on_copy)]
#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_function_visibility<'a>(
    ir_node: &'a ir::FunctionVisibility,
    semantic: &'a SemanticContext,
) -> FunctionVisibility {
    match ir_node {
        ir::FunctionVisibility::Public => FunctionVisibility::Public,
        ir::FunctionVisibility::Private => FunctionVisibility::Private,
        ir::FunctionVisibility::Internal => FunctionVisibility::Internal,
        ir::FunctionVisibility::External => FunctionVisibility::External,
    }
}

#[derive(Clone, Copy)]
pub enum ImportClause<'a> {
    PathImport(PathImport<'a>),
    ImportDeconstruction(ImportDeconstruction<'a>),
}

#[allow(clippy::clone_on_copy)]
#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_import_clause<'a>(
    ir_node: &'a ir::ImportClause,
    semantic: &'a SemanticContext,
) -> ImportClause<'a> {
    match ir_node {
        ir::ImportClause::PathImport(variant) => {
            ImportClause::PathImport(create_path_import(variant, semantic))
        }
        ir::ImportClause::ImportDeconstruction(variant) => {
            ImportClause::ImportDeconstruction(create_import_deconstruction(variant, semantic))
        }
    }
}

#[derive(Clone, Copy)]
pub enum InequalityExpressionOperator<'a> {
    GreaterThan(GreaterThan<'a>),
    GreaterThanEqual(GreaterThanEqual<'a>),
    LessThan(LessThan<'a>),
    LessThanEqual(LessThanEqual<'a>),
}

#[allow(clippy::clone_on_copy)]
#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_inequality_expression_operator<'a>(
    ir_node: &'a ir::InequalityExpressionOperator,
    semantic: &'a SemanticContext,
) -> InequalityExpressionOperator<'a> {
    match ir_node {
        ir::InequalityExpressionOperator::GreaterThan(variant) => {
            InequalityExpressionOperator::GreaterThan(create_greater_than(variant, semantic))
        }
        ir::InequalityExpressionOperator::GreaterThanEqual(variant) => {
            InequalityExpressionOperator::GreaterThanEqual(create_greater_than_equal(
                variant, semantic,
            ))
        }
        ir::InequalityExpressionOperator::LessThan(variant) => {
            InequalityExpressionOperator::LessThan(create_less_than(variant, semantic))
        }
        ir::InequalityExpressionOperator::LessThanEqual(variant) => {
            InequalityExpressionOperator::LessThanEqual(create_less_than_equal(variant, semantic))
        }
    }
}

#[derive(Clone, Copy)]
pub enum MultiplicativeExpressionOperator<'a> {
    Asterisk(Asterisk<'a>),
    Percent(Percent<'a>),
    Slash(Slash<'a>),
}

#[allow(clippy::clone_on_copy)]
#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_multiplicative_expression_operator<'a>(
    ir_node: &'a ir::MultiplicativeExpressionOperator,
    semantic: &'a SemanticContext,
) -> MultiplicativeExpressionOperator<'a> {
    match ir_node {
        ir::MultiplicativeExpressionOperator::Asterisk(variant) => {
            MultiplicativeExpressionOperator::Asterisk(create_asterisk(variant, semantic))
        }
        ir::MultiplicativeExpressionOperator::Percent(variant) => {
            MultiplicativeExpressionOperator::Percent(create_percent(variant, semantic))
        }
        ir::MultiplicativeExpressionOperator::Slash(variant) => {
            MultiplicativeExpressionOperator::Slash(create_slash(variant, semantic))
        }
    }
}

#[derive(Clone, Copy)]
pub enum NumberUnit<'a> {
    WeiKeyword(WeiKeyword<'a>),
    GweiKeyword(GweiKeyword<'a>),
    EtherKeyword(EtherKeyword<'a>),
    SecondsKeyword(SecondsKeyword<'a>),
    MinutesKeyword(MinutesKeyword<'a>),
    HoursKeyword(HoursKeyword<'a>),
    DaysKeyword(DaysKeyword<'a>),
    WeeksKeyword(WeeksKeyword<'a>),
}

#[allow(clippy::clone_on_copy)]
#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_number_unit<'a>(
    ir_node: &'a ir::NumberUnit,
    semantic: &'a SemanticContext,
) -> NumberUnit<'a> {
    match ir_node {
        ir::NumberUnit::WeiKeyword(variant) => {
            NumberUnit::WeiKeyword(create_wei_keyword(variant, semantic))
        }
        ir::NumberUnit::GweiKeyword(variant) => {
            NumberUnit::GweiKeyword(create_gwei_keyword(variant, semantic))
        }
        ir::NumberUnit::EtherKeyword(variant) => {
            NumberUnit::EtherKeyword(create_ether_keyword(variant, semantic))
        }
        ir::NumberUnit::SecondsKeyword(variant) => {
            NumberUnit::SecondsKeyword(create_seconds_keyword(variant, semantic))
        }
        ir::NumberUnit::MinutesKeyword(variant) => {
            NumberUnit::MinutesKeyword(create_minutes_keyword(variant, semantic))
        }
        ir::NumberUnit::HoursKeyword(variant) => {
            NumberUnit::HoursKeyword(create_hours_keyword(variant, semantic))
        }
        ir::NumberUnit::DaysKeyword(variant) => {
            NumberUnit::DaysKeyword(create_days_keyword(variant, semantic))
        }
        ir::NumberUnit::WeeksKeyword(variant) => {
            NumberUnit::WeeksKeyword(create_weeks_keyword(variant, semantic))
        }
    }
}

#[derive(Clone, Copy)]
pub enum PostfixExpressionOperator<'a> {
    MinusMinus(MinusMinus<'a>),
    PlusPlus(PlusPlus<'a>),
}

#[allow(clippy::clone_on_copy)]
#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_postfix_expression_operator<'a>(
    ir_node: &'a ir::PostfixExpressionOperator,
    semantic: &'a SemanticContext,
) -> PostfixExpressionOperator<'a> {
    match ir_node {
        ir::PostfixExpressionOperator::MinusMinus(variant) => {
            PostfixExpressionOperator::MinusMinus(create_minus_minus(variant, semantic))
        }
        ir::PostfixExpressionOperator::PlusPlus(variant) => {
            PostfixExpressionOperator::PlusPlus(create_plus_plus(variant, semantic))
        }
    }
}

#[derive(Clone, Copy)]
pub enum Pragma<'a> {
    VersionPragma(VersionPragma<'a>),
    AbicoderPragma(AbicoderPragma<'a>),
    ExperimentalPragma(ExperimentalPragma<'a>),
}

#[allow(clippy::clone_on_copy)]
#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_pragma<'a>(
    ir_node: &'a ir::Pragma,
    semantic: &'a SemanticContext,
) -> Pragma<'a> {
    match ir_node {
        ir::Pragma::VersionPragma(variant) => {
            Pragma::VersionPragma(create_version_pragma(variant, semantic))
        }
        ir::Pragma::AbicoderPragma(variant) => {
            Pragma::AbicoderPragma(create_abicoder_pragma(variant, semantic))
        }
        ir::Pragma::ExperimentalPragma(variant) => {
            Pragma::ExperimentalPragma(create_experimental_pragma(variant, semantic))
        }
    }
}

#[derive(Clone, Copy)]
pub enum PrefixExpressionOperator<'a> {
    Bang(Bang<'a>),
    DeleteKeyword(DeleteKeyword<'a>),
    Minus(Minus<'a>),
    MinusMinus(MinusMinus<'a>),
    PlusPlus(PlusPlus<'a>),
    Tilde(Tilde<'a>),
}

#[allow(clippy::clone_on_copy)]
#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_prefix_expression_operator<'a>(
    ir_node: &'a ir::PrefixExpressionOperator,
    semantic: &'a SemanticContext,
) -> PrefixExpressionOperator<'a> {
    match ir_node {
        ir::PrefixExpressionOperator::Bang(variant) => {
            PrefixExpressionOperator::Bang(create_bang(variant, semantic))
        }
        ir::PrefixExpressionOperator::DeleteKeyword(variant) => {
            PrefixExpressionOperator::DeleteKeyword(create_delete_keyword(variant, semantic))
        }
        ir::PrefixExpressionOperator::Minus(variant) => {
            PrefixExpressionOperator::Minus(create_minus(variant, semantic))
        }
        ir::PrefixExpressionOperator::MinusMinus(variant) => {
            PrefixExpressionOperator::MinusMinus(create_minus_minus(variant, semantic))
        }
        ir::PrefixExpressionOperator::PlusPlus(variant) => {
            PrefixExpressionOperator::PlusPlus(create_plus_plus(variant, semantic))
        }
        ir::PrefixExpressionOperator::Tilde(variant) => {
            PrefixExpressionOperator::Tilde(create_tilde(variant, semantic))
        }
    }
}

#[derive(Clone, Copy)]
pub enum ShiftExpressionOperator<'a> {
    GreaterThanGreaterThan(GreaterThanGreaterThan<'a>),
    GreaterThanGreaterThanGreaterThan(GreaterThanGreaterThanGreaterThan<'a>),
    LessThanLessThan(LessThanLessThan<'a>),
}

#[allow(clippy::clone_on_copy)]
#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_shift_expression_operator<'a>(
    ir_node: &'a ir::ShiftExpressionOperator,
    semantic: &'a SemanticContext,
) -> ShiftExpressionOperator<'a> {
    match ir_node {
        ir::ShiftExpressionOperator::GreaterThanGreaterThan(variant) => {
            ShiftExpressionOperator::GreaterThanGreaterThan(create_greater_than_greater_than(
                variant, semantic,
            ))
        }
        ir::ShiftExpressionOperator::GreaterThanGreaterThanGreaterThan(variant) => {
            ShiftExpressionOperator::GreaterThanGreaterThanGreaterThan(
                create_greater_than_greater_than_greater_than(variant, semantic),
            )
        }
        ir::ShiftExpressionOperator::LessThanLessThan(variant) => {
            ShiftExpressionOperator::LessThanLessThan(create_less_than_less_than(variant, semantic))
        }
    }
}

#[derive(Clone, Copy)]
pub enum SourceUnitMember<'a> {
    PragmaDirective(PragmaDirective<'a>),
    ImportClause(ImportClause<'a>),
    ContractDefinition(ContractDefinition<'a>),
    InterfaceDefinition(InterfaceDefinition<'a>),
    LibraryDefinition(LibraryDefinition<'a>),
    StructDefinition(StructDefinition<'a>),
    EnumDefinition(EnumDefinition<'a>),
    FunctionDefinition(FunctionDefinition<'a>),
    ErrorDefinition(ErrorDefinition<'a>),
    UserDefinedValueTypeDefinition(UserDefinedValueTypeDefinition<'a>),
    UsingDirective(UsingDirective<'a>),
    EventDefinition(EventDefinition<'a>),
    ConstantDefinition(ConstantDefinition<'a>),
}

#[allow(clippy::clone_on_copy)]
#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_source_unit_member<'a>(
    ir_node: &'a ir::SourceUnitMember,
    semantic: &'a SemanticContext,
) -> SourceUnitMember<'a> {
    match ir_node {
        ir::SourceUnitMember::PragmaDirective(variant) => {
            SourceUnitMember::PragmaDirective(create_pragma_directive(variant, semantic))
        }
        ir::SourceUnitMember::ImportClause(variant) => {
            SourceUnitMember::ImportClause(create_import_clause(variant, semantic))
        }
        ir::SourceUnitMember::ContractDefinition(variant) => {
            SourceUnitMember::ContractDefinition(create_contract_definition(variant, semantic))
        }
        ir::SourceUnitMember::InterfaceDefinition(variant) => {
            SourceUnitMember::InterfaceDefinition(create_interface_definition(variant, semantic))
        }
        ir::SourceUnitMember::LibraryDefinition(variant) => {
            SourceUnitMember::LibraryDefinition(create_library_definition(variant, semantic))
        }
        ir::SourceUnitMember::StructDefinition(variant) => {
            SourceUnitMember::StructDefinition(create_struct_definition(variant, semantic))
        }
        ir::SourceUnitMember::EnumDefinition(variant) => {
            SourceUnitMember::EnumDefinition(create_enum_definition(variant, semantic))
        }
        ir::SourceUnitMember::FunctionDefinition(variant) => {
            SourceUnitMember::FunctionDefinition(create_function_definition(variant, semantic))
        }
        ir::SourceUnitMember::ErrorDefinition(variant) => {
            SourceUnitMember::ErrorDefinition(create_error_definition(variant, semantic))
        }
        ir::SourceUnitMember::UserDefinedValueTypeDefinition(variant) => {
            SourceUnitMember::UserDefinedValueTypeDefinition(
                create_user_defined_value_type_definition(variant, semantic),
            )
        }
        ir::SourceUnitMember::UsingDirective(variant) => {
            SourceUnitMember::UsingDirective(create_using_directive(variant, semantic))
        }
        ir::SourceUnitMember::EventDefinition(variant) => {
            SourceUnitMember::EventDefinition(create_event_definition(variant, semantic))
        }
        ir::SourceUnitMember::ConstantDefinition(variant) => {
            SourceUnitMember::ConstantDefinition(create_constant_definition(variant, semantic))
        }
    }
}

#[derive(Clone, Copy)]
pub enum StateVariableMutability {
    Mutable,
    Constant,
    Immutable,
    Transient,
}

#[allow(clippy::clone_on_copy)]
#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_state_variable_mutability<'a>(
    ir_node: &'a ir::StateVariableMutability,
    semantic: &'a SemanticContext,
) -> StateVariableMutability {
    match ir_node {
        ir::StateVariableMutability::Mutable => StateVariableMutability::Mutable,
        ir::StateVariableMutability::Constant => StateVariableMutability::Constant,
        ir::StateVariableMutability::Immutable => StateVariableMutability::Immutable,
        ir::StateVariableMutability::Transient => StateVariableMutability::Transient,
    }
}

#[derive(Clone, Copy)]
pub enum StateVariableVisibility {
    Public,
    Private,
    Internal,
}

#[allow(clippy::clone_on_copy)]
#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_state_variable_visibility<'a>(
    ir_node: &'a ir::StateVariableVisibility,
    semantic: &'a SemanticContext,
) -> StateVariableVisibility {
    match ir_node {
        ir::StateVariableVisibility::Public => StateVariableVisibility::Public,
        ir::StateVariableVisibility::Private => StateVariableVisibility::Private,
        ir::StateVariableVisibility::Internal => StateVariableVisibility::Internal,
    }
}

#[derive(Clone, Copy)]
pub enum Statement<'a> {
    IfStatement(IfStatement<'a>),
    ForStatement(ForStatement<'a>),
    WhileStatement(WhileStatement<'a>),
    DoWhileStatement(DoWhileStatement<'a>),
    ContinueStatement(ContinueStatement<'a>),
    BreakStatement(BreakStatement<'a>),
    ReturnStatement(ReturnStatement<'a>),
    EmitStatement(EmitStatement<'a>),
    TryStatement(TryStatement<'a>),
    RevertStatement(RevertStatement<'a>),
    AssemblyStatement(AssemblyStatement<'a>),
    Block(Block<'a>),
    UncheckedBlock(UncheckedBlock<'a>),
    VariableDeclarationStatement(VariableDeclarationStatement<'a>),
    ExpressionStatement(ExpressionStatement<'a>),
}

#[allow(clippy::clone_on_copy)]
#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_statement<'a>(
    ir_node: &'a ir::Statement,
    semantic: &'a SemanticContext,
) -> Statement<'a> {
    match ir_node {
        ir::Statement::IfStatement(variant) => {
            Statement::IfStatement(create_if_statement(variant, semantic))
        }
        ir::Statement::ForStatement(variant) => {
            Statement::ForStatement(create_for_statement(variant, semantic))
        }
        ir::Statement::WhileStatement(variant) => {
            Statement::WhileStatement(create_while_statement(variant, semantic))
        }
        ir::Statement::DoWhileStatement(variant) => {
            Statement::DoWhileStatement(create_do_while_statement(variant, semantic))
        }
        ir::Statement::ContinueStatement(variant) => {
            Statement::ContinueStatement(create_continue_statement(variant, semantic))
        }
        ir::Statement::BreakStatement(variant) => {
            Statement::BreakStatement(create_break_statement(variant, semantic))
        }
        ir::Statement::ReturnStatement(variant) => {
            Statement::ReturnStatement(create_return_statement(variant, semantic))
        }
        ir::Statement::EmitStatement(variant) => {
            Statement::EmitStatement(create_emit_statement(variant, semantic))
        }
        ir::Statement::TryStatement(variant) => {
            Statement::TryStatement(create_try_statement(variant, semantic))
        }
        ir::Statement::RevertStatement(variant) => {
            Statement::RevertStatement(create_revert_statement(variant, semantic))
        }
        ir::Statement::AssemblyStatement(variant) => {
            Statement::AssemblyStatement(create_assembly_statement(variant, semantic))
        }
        ir::Statement::Block(variant) => Statement::Block(create_block(variant, semantic)),
        ir::Statement::UncheckedBlock(variant) => {
            Statement::UncheckedBlock(create_unchecked_block(variant, semantic))
        }
        ir::Statement::VariableDeclarationStatement(variant) => {
            Statement::VariableDeclarationStatement(create_variable_declaration_statement(
                variant, semantic,
            ))
        }
        ir::Statement::ExpressionStatement(variant) => {
            Statement::ExpressionStatement(create_expression_statement(variant, semantic))
        }
    }
}

#[derive(Clone, Copy)]
pub enum StorageLocation<'a> {
    MemoryKeyword(MemoryKeyword<'a>),
    StorageKeyword(StorageKeyword<'a>),
    CallDataKeyword(CallDataKeyword<'a>),
}

#[allow(clippy::clone_on_copy)]
#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_storage_location<'a>(
    ir_node: &'a ir::StorageLocation,
    semantic: &'a SemanticContext,
) -> StorageLocation<'a> {
    match ir_node {
        ir::StorageLocation::MemoryKeyword(variant) => {
            StorageLocation::MemoryKeyword(create_memory_keyword(variant, semantic))
        }
        ir::StorageLocation::StorageKeyword(variant) => {
            StorageLocation::StorageKeyword(create_storage_keyword(variant, semantic))
        }
        ir::StorageLocation::CallDataKeyword(variant) => {
            StorageLocation::CallDataKeyword(create_call_data_keyword(variant, semantic))
        }
    }
}

#[derive(Clone, Copy)]
pub enum StringExpression<'a> {
    StringLiterals(StringLiterals<'a>),
    HexStringLiterals(HexStringLiterals<'a>),
    UnicodeStringLiterals(UnicodeStringLiterals<'a>),
}

#[allow(clippy::clone_on_copy)]
#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_string_expression<'a>(
    ir_node: &'a ir::StringExpression,
    semantic: &'a SemanticContext,
) -> StringExpression<'a> {
    match ir_node {
        ir::StringExpression::StringLiterals(nodes) => {
            StringExpression::StringLiterals(create_string_literals(nodes, semantic))
        }
        ir::StringExpression::HexStringLiterals(nodes) => {
            StringExpression::HexStringLiterals(create_hex_string_literals(nodes, semantic))
        }
        ir::StringExpression::UnicodeStringLiterals(nodes) => {
            StringExpression::UnicodeStringLiterals(create_unicode_string_literals(nodes, semantic))
        }
    }
}

#[derive(Clone, Copy)]
pub enum TypeName<'a> {
    ArrayTypeName(ArrayTypeName<'a>),
    FunctionType(FunctionType<'a>),
    MappingType(MappingType<'a>),
    ElementaryType(ElementaryType<'a>),
    IdentifierPath(IdentifierPath<'a>),
}

#[allow(clippy::clone_on_copy)]
#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_type_name<'a>(
    ir_node: &'a ir::TypeName,
    semantic: &'a SemanticContext,
) -> TypeName<'a> {
    match ir_node {
        ir::TypeName::ArrayTypeName(variant) => {
            TypeName::ArrayTypeName(create_array_type_name(variant, semantic))
        }
        ir::TypeName::FunctionType(variant) => {
            TypeName::FunctionType(create_function_type(variant, semantic))
        }
        ir::TypeName::MappingType(variant) => {
            TypeName::MappingType(create_mapping_type(variant, semantic))
        }
        ir::TypeName::ElementaryType(variant) => {
            TypeName::ElementaryType(create_elementary_type(variant, semantic))
        }
        ir::TypeName::IdentifierPath(nodes) => {
            TypeName::IdentifierPath(create_identifier_path(nodes, semantic))
        }
    }
}

#[derive(Clone, Copy)]
pub enum UsingClause<'a> {
    IdentifierPath(IdentifierPath<'a>),
    UsingDeconstruction(UsingDeconstruction<'a>),
}

#[allow(clippy::clone_on_copy)]
#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_using_clause<'a>(
    ir_node: &'a ir::UsingClause,
    semantic: &'a SemanticContext,
) -> UsingClause<'a> {
    match ir_node {
        ir::UsingClause::IdentifierPath(nodes) => {
            UsingClause::IdentifierPath(create_identifier_path(nodes, semantic))
        }
        ir::UsingClause::UsingDeconstruction(variant) => {
            UsingClause::UsingDeconstruction(create_using_deconstruction(variant, semantic))
        }
    }
}

#[derive(Clone, Copy)]
pub enum UsingOperator<'a> {
    Ampersand(Ampersand<'a>),
    Asterisk(Asterisk<'a>),
    BangEqual(BangEqual<'a>),
    Bar(Bar<'a>),
    Caret(Caret<'a>),
    EqualEqual(EqualEqual<'a>),
    GreaterThan(GreaterThan<'a>),
    GreaterThanEqual(GreaterThanEqual<'a>),
    LessThan(LessThan<'a>),
    LessThanEqual(LessThanEqual<'a>),
    Minus(Minus<'a>),
    Percent(Percent<'a>),
    Plus(Plus<'a>),
    Slash(Slash<'a>),
    Tilde(Tilde<'a>),
}

#[allow(clippy::clone_on_copy)]
#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_using_operator<'a>(
    ir_node: &'a ir::UsingOperator,
    semantic: &'a SemanticContext,
) -> UsingOperator<'a> {
    match ir_node {
        ir::UsingOperator::Ampersand(variant) => {
            UsingOperator::Ampersand(create_ampersand(variant, semantic))
        }
        ir::UsingOperator::Asterisk(variant) => {
            UsingOperator::Asterisk(create_asterisk(variant, semantic))
        }
        ir::UsingOperator::BangEqual(variant) => {
            UsingOperator::BangEqual(create_bang_equal(variant, semantic))
        }
        ir::UsingOperator::Bar(variant) => UsingOperator::Bar(create_bar(variant, semantic)),
        ir::UsingOperator::Caret(variant) => UsingOperator::Caret(create_caret(variant, semantic)),
        ir::UsingOperator::EqualEqual(variant) => {
            UsingOperator::EqualEqual(create_equal_equal(variant, semantic))
        }
        ir::UsingOperator::GreaterThan(variant) => {
            UsingOperator::GreaterThan(create_greater_than(variant, semantic))
        }
        ir::UsingOperator::GreaterThanEqual(variant) => {
            UsingOperator::GreaterThanEqual(create_greater_than_equal(variant, semantic))
        }
        ir::UsingOperator::LessThan(variant) => {
            UsingOperator::LessThan(create_less_than(variant, semantic))
        }
        ir::UsingOperator::LessThanEqual(variant) => {
            UsingOperator::LessThanEqual(create_less_than_equal(variant, semantic))
        }
        ir::UsingOperator::Minus(variant) => UsingOperator::Minus(create_minus(variant, semantic)),
        ir::UsingOperator::Percent(variant) => {
            UsingOperator::Percent(create_percent(variant, semantic))
        }
        ir::UsingOperator::Plus(variant) => UsingOperator::Plus(create_plus(variant, semantic)),
        ir::UsingOperator::Slash(variant) => UsingOperator::Slash(create_slash(variant, semantic)),
        ir::UsingOperator::Tilde(variant) => UsingOperator::Tilde(create_tilde(variant, semantic)),
    }
}

#[derive(Clone, Copy)]
pub enum UsingTarget<'a> {
    TypeName(TypeName<'a>),
    Asterisk(Asterisk<'a>),
}

#[allow(clippy::clone_on_copy)]
#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_using_target<'a>(
    ir_node: &'a ir::UsingTarget,
    semantic: &'a SemanticContext,
) -> UsingTarget<'a> {
    match ir_node {
        ir::UsingTarget::TypeName(variant) => {
            UsingTarget::TypeName(create_type_name(variant, semantic))
        }
        ir::UsingTarget::Asterisk(variant) => {
            UsingTarget::Asterisk(create_asterisk(variant, semantic))
        }
    }
}

#[derive(Clone, Copy)]
pub enum VariableDeclarationTarget<'a> {
    SingleTypedDeclaration(SingleTypedDeclaration<'a>),
    MultiTypedDeclaration(MultiTypedDeclaration<'a>),
}

#[allow(clippy::clone_on_copy)]
#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_variable_declaration_target<'a>(
    ir_node: &'a ir::VariableDeclarationTarget,
    semantic: &'a SemanticContext,
) -> VariableDeclarationTarget<'a> {
    match ir_node {
        ir::VariableDeclarationTarget::SingleTypedDeclaration(variant) => {
            VariableDeclarationTarget::SingleTypedDeclaration(create_single_typed_declaration(
                variant, semantic,
            ))
        }
        ir::VariableDeclarationTarget::MultiTypedDeclaration(variant) => {
            VariableDeclarationTarget::MultiTypedDeclaration(create_multi_typed_declaration(
                variant, semantic,
            ))
        }
    }
}

#[derive(Clone, Copy)]
pub enum VersionPragmaComponent<'a> {
    Wildcard,
    Unrecognized,
    Number(&'a usize),
}

#[allow(clippy::clone_on_copy)]
#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_version_pragma_component<'a>(
    ir_node: &'a ir::VersionPragmaComponent,
    semantic: &'a SemanticContext,
) -> VersionPragmaComponent<'a> {
    match ir_node {
        ir::VersionPragmaComponent::Wildcard => VersionPragmaComponent::Wildcard,
        ir::VersionPragmaComponent::Unrecognized => VersionPragmaComponent::Unrecognized,
        ir::VersionPragmaComponent::Number(variant) => VersionPragmaComponent::Number(variant),
    }
}

#[derive(Clone, Copy)]
pub enum VersionPragmaOperator {
    Caret,
    Tilde,
    Equal,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
}

#[allow(clippy::clone_on_copy)]
#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_version_pragma_operator<'a>(
    ir_node: &'a ir::VersionPragmaOperator,
    semantic: &'a SemanticContext,
) -> VersionPragmaOperator {
    match ir_node {
        ir::VersionPragmaOperator::Caret => VersionPragmaOperator::Caret,
        ir::VersionPragmaOperator::Tilde => VersionPragmaOperator::Tilde,
        ir::VersionPragmaOperator::Equal => VersionPragmaOperator::Equal,
        ir::VersionPragmaOperator::LessThan => VersionPragmaOperator::LessThan,
        ir::VersionPragmaOperator::LessThanEqual => VersionPragmaOperator::LessThanEqual,
        ir::VersionPragmaOperator::GreaterThan => VersionPragmaOperator::GreaterThan,
        ir::VersionPragmaOperator::GreaterThanEqual => VersionPragmaOperator::GreaterThanEqual,
    }
}

#[derive(Clone, Copy)]
pub enum YulExpression<'a> {
    YulFunctionCallExpression(YulFunctionCallExpression<'a>),
    YulLiteral(YulLiteral<'a>),
    YulPath(YulPath<'a>),
}

#[allow(clippy::clone_on_copy)]
#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_yul_expression<'a>(
    ir_node: &'a ir::YulExpression,
    semantic: &'a SemanticContext,
) -> YulExpression<'a> {
    match ir_node {
        ir::YulExpression::YulFunctionCallExpression(variant) => {
            YulExpression::YulFunctionCallExpression(create_yul_function_call_expression(
                variant, semantic,
            ))
        }
        ir::YulExpression::YulLiteral(variant) => {
            YulExpression::YulLiteral(create_yul_literal(variant, semantic))
        }
        ir::YulExpression::YulPath(nodes) => {
            YulExpression::YulPath(create_yul_path(nodes, semantic))
        }
    }
}

#[derive(Clone, Copy)]
pub enum YulLiteral<'a> {
    TrueKeyword(TrueKeyword<'a>),
    FalseKeyword(FalseKeyword<'a>),
    DecimalLiteral(DecimalLiteral<'a>),
    HexLiteral(HexLiteral<'a>),
    HexStringLiteral(HexStringLiteral<'a>),
    StringLiteral(StringLiteral<'a>),
}

#[allow(clippy::clone_on_copy)]
#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_yul_literal<'a>(
    ir_node: &'a ir::YulLiteral,
    semantic: &'a SemanticContext,
) -> YulLiteral<'a> {
    match ir_node {
        ir::YulLiteral::TrueKeyword(variant) => {
            YulLiteral::TrueKeyword(create_true_keyword(variant, semantic))
        }
        ir::YulLiteral::FalseKeyword(variant) => {
            YulLiteral::FalseKeyword(create_false_keyword(variant, semantic))
        }
        ir::YulLiteral::DecimalLiteral(variant) => {
            YulLiteral::DecimalLiteral(create_decimal_literal(variant, semantic))
        }
        ir::YulLiteral::HexLiteral(variant) => {
            YulLiteral::HexLiteral(create_hex_literal(variant, semantic))
        }
        ir::YulLiteral::HexStringLiteral(variant) => {
            YulLiteral::HexStringLiteral(create_hex_string_literal(variant, semantic))
        }
        ir::YulLiteral::StringLiteral(variant) => {
            YulLiteral::StringLiteral(create_string_literal(variant, semantic))
        }
    }
}

#[derive(Clone, Copy)]
pub enum YulStatement<'a> {
    YulBlock(YulBlock<'a>),
    YulFunctionDefinition(YulFunctionDefinition<'a>),
    YulIfStatement(YulIfStatement<'a>),
    YulForStatement(YulForStatement<'a>),
    YulSwitchStatement(YulSwitchStatement<'a>),
    YulLeaveStatement(YulLeaveStatement<'a>),
    YulBreakStatement(YulBreakStatement<'a>),
    YulContinueStatement(YulContinueStatement<'a>),
    YulVariableAssignmentStatement(YulVariableAssignmentStatement<'a>),
    YulVariableDeclarationStatement(YulVariableDeclarationStatement<'a>),
    YulExpression(YulExpression<'a>),
}

#[allow(clippy::clone_on_copy)]
#[allow(clippy::too_many_lines)]
#[allow(clippy::trivially_copy_pass_by_ref)]
pub(crate) fn create_yul_statement<'a>(
    ir_node: &'a ir::YulStatement,
    semantic: &'a SemanticContext,
) -> YulStatement<'a> {
    match ir_node {
        ir::YulStatement::YulBlock(variant) => {
            YulStatement::YulBlock(create_yul_block(variant, semantic))
        }
        ir::YulStatement::YulFunctionDefinition(variant) => {
            YulStatement::YulFunctionDefinition(create_yul_function_definition(variant, semantic))
        }
        ir::YulStatement::YulIfStatement(variant) => {
            YulStatement::YulIfStatement(create_yul_if_statement(variant, semantic))
        }
        ir::YulStatement::YulForStatement(variant) => {
            YulStatement::YulForStatement(create_yul_for_statement(variant, semantic))
        }
        ir::YulStatement::YulSwitchStatement(variant) => {
            YulStatement::YulSwitchStatement(create_yul_switch_statement(variant, semantic))
        }
        ir::YulStatement::YulLeaveStatement(variant) => {
            YulStatement::YulLeaveStatement(create_yul_leave_statement(variant, semantic))
        }
        ir::YulStatement::YulBreakStatement(variant) => {
            YulStatement::YulBreakStatement(create_yul_break_statement(variant, semantic))
        }
        ir::YulStatement::YulContinueStatement(variant) => {
            YulStatement::YulContinueStatement(create_yul_continue_statement(variant, semantic))
        }
        ir::YulStatement::YulVariableAssignmentStatement(variant) => {
            YulStatement::YulVariableAssignmentStatement(create_yul_variable_assignment_statement(
                variant, semantic,
            ))
        }
        ir::YulStatement::YulVariableDeclarationStatement(variant) => {
            YulStatement::YulVariableDeclarationStatement(
                create_yul_variable_declaration_statement(variant, semantic),
            )
        }
        ir::YulStatement::YulExpression(variant) => {
            YulStatement::YulExpression(create_yul_expression(variant, semantic))
        }
    }
}

//
// Repeated & Separated
//

pub type ArrayValues<'a> = ArrayValuesStruct<'a>;

pub(crate) fn create_array_values<'a>(
    nodes: &'a ir::ArrayValues,
    semantic: &'a SemanticContext,
) -> ArrayValues<'a> {
    ArrayValuesStruct {
        ir_nodes: nodes,
        semantic,
    }
}

#[derive(Clone, Copy)]
pub struct ArrayValuesStruct<'a> {
    pub(crate) ir_nodes: &'a ir::ArrayValues,
    pub(crate) semantic: &'a SemanticContext,
}

impl<'a> ArrayValuesStruct<'a> {
    pub fn iter(&self) -> impl Iterator<Item = Expression<'a>> + use<'a> {
        let semantic = self.semantic;
        self.ir_nodes
            .iter()
            .map(move |ir_node| create_expression(ir_node, semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type CallOptions<'a> = CallOptionsStruct<'a>;

pub(crate) fn create_call_options<'a>(
    nodes: &'a ir::CallOptions,
    semantic: &'a SemanticContext,
) -> CallOptions<'a> {
    CallOptionsStruct {
        ir_nodes: nodes,
        semantic,
    }
}

#[derive(Clone, Copy)]
pub struct CallOptionsStruct<'a> {
    pub(crate) ir_nodes: &'a ir::CallOptions,
    pub(crate) semantic: &'a SemanticContext,
}

impl<'a> CallOptionsStruct<'a> {
    pub fn iter(&self) -> impl Iterator<Item = NamedArgument<'a>> + use<'a> {
        let semantic = self.semantic;
        self.ir_nodes
            .iter()
            .map(move |ir_node| create_named_argument(ir_node, semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type CatchClauses<'a> = CatchClausesStruct<'a>;

pub(crate) fn create_catch_clauses<'a>(
    nodes: &'a ir::CatchClauses,
    semantic: &'a SemanticContext,
) -> CatchClauses<'a> {
    CatchClausesStruct {
        ir_nodes: nodes,
        semantic,
    }
}

#[derive(Clone, Copy)]
pub struct CatchClausesStruct<'a> {
    pub(crate) ir_nodes: &'a ir::CatchClauses,
    pub(crate) semantic: &'a SemanticContext,
}

impl<'a> CatchClausesStruct<'a> {
    pub fn iter(&self) -> impl Iterator<Item = CatchClause<'a>> + use<'a> {
        let semantic = self.semantic;
        self.ir_nodes
            .iter()
            .map(move |ir_node| create_catch_clause(ir_node, semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type ContractMembers<'a> = ContractMembersStruct<'a>;

pub(crate) fn create_contract_members<'a>(
    nodes: &'a ir::ContractMembers,
    semantic: &'a SemanticContext,
) -> ContractMembers<'a> {
    ContractMembersStruct {
        ir_nodes: nodes,
        semantic,
    }
}

#[derive(Clone, Copy)]
pub struct ContractMembersStruct<'a> {
    pub(crate) ir_nodes: &'a ir::ContractMembers,
    pub(crate) semantic: &'a SemanticContext,
}

impl<'a> ContractMembersStruct<'a> {
    pub fn iter(&self) -> impl Iterator<Item = ContractMember<'a>> + use<'a> {
        let semantic = self.semantic;
        self.ir_nodes
            .iter()
            .map(move |ir_node| create_contract_member(ir_node, semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type EnumMembers<'a> = EnumMembersStruct<'a>;

pub(crate) fn create_enum_members<'a>(
    nodes: &'a ir::EnumMembers,
    semantic: &'a SemanticContext,
) -> EnumMembers<'a> {
    EnumMembersStruct {
        ir_nodes: nodes,
        semantic,
    }
}

#[derive(Clone, Copy)]
pub struct EnumMembersStruct<'a> {
    pub(crate) ir_nodes: &'a ir::EnumMembers,
    pub(crate) semantic: &'a SemanticContext,
}

impl<'a> EnumMembersStruct<'a> {
    pub fn iter(&self) -> impl Iterator<Item = Identifier<'a>> + use<'a> {
        let semantic = self.semantic;
        self.ir_nodes
            .iter()
            .map(move |ir_node| create_identifier(ir_node, semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type HexStringLiterals<'a> = HexStringLiteralsStruct<'a>;

pub(crate) fn create_hex_string_literals<'a>(
    nodes: &'a ir::HexStringLiterals,
    semantic: &'a SemanticContext,
) -> HexStringLiterals<'a> {
    HexStringLiteralsStruct {
        ir_nodes: nodes,
        semantic,
    }
}

#[derive(Clone, Copy)]
pub struct HexStringLiteralsStruct<'a> {
    pub(crate) ir_nodes: &'a ir::HexStringLiterals,
    pub(crate) semantic: &'a SemanticContext,
}

impl<'a> HexStringLiteralsStruct<'a> {
    pub fn iter(&self) -> impl Iterator<Item = HexStringLiteral<'a>> + use<'a> {
        let semantic = self.semantic;
        self.ir_nodes
            .iter()
            .map(move |ir_node| create_hex_string_literal(ir_node, semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type IdentifierPath<'a> = IdentifierPathStruct<'a>;

pub(crate) fn create_identifier_path<'a>(
    nodes: &'a ir::IdentifierPath,
    semantic: &'a SemanticContext,
) -> IdentifierPath<'a> {
    IdentifierPathStruct {
        ir_nodes: nodes,
        semantic,
    }
}

#[derive(Clone, Copy)]
pub struct IdentifierPathStruct<'a> {
    pub(crate) ir_nodes: &'a ir::IdentifierPath,
    pub(crate) semantic: &'a SemanticContext,
}

impl<'a> IdentifierPathStruct<'a> {
    pub fn iter(&self) -> impl Iterator<Item = Identifier<'a>> + use<'a> {
        let semantic = self.semantic;
        self.ir_nodes
            .iter()
            .map(move |ir_node| create_identifier(ir_node, semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type ImportDeconstructionSymbols<'a> = ImportDeconstructionSymbolsStruct<'a>;

pub(crate) fn create_import_deconstruction_symbols<'a>(
    nodes: &'a ir::ImportDeconstructionSymbols,
    semantic: &'a SemanticContext,
) -> ImportDeconstructionSymbols<'a> {
    ImportDeconstructionSymbolsStruct {
        ir_nodes: nodes,
        semantic,
    }
}

#[derive(Clone, Copy)]
pub struct ImportDeconstructionSymbolsStruct<'a> {
    pub(crate) ir_nodes: &'a ir::ImportDeconstructionSymbols,
    pub(crate) semantic: &'a SemanticContext,
}

impl<'a> ImportDeconstructionSymbolsStruct<'a> {
    pub fn iter(&self) -> impl Iterator<Item = ImportDeconstructionSymbol<'a>> + use<'a> {
        let semantic = self.semantic;
        self.ir_nodes
            .iter()
            .map(move |ir_node| create_import_deconstruction_symbol(ir_node, semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type InheritanceTypes<'a> = InheritanceTypesStruct<'a>;

pub(crate) fn create_inheritance_types<'a>(
    nodes: &'a ir::InheritanceTypes,
    semantic: &'a SemanticContext,
) -> InheritanceTypes<'a> {
    InheritanceTypesStruct {
        ir_nodes: nodes,
        semantic,
    }
}

#[derive(Clone, Copy)]
pub struct InheritanceTypesStruct<'a> {
    pub(crate) ir_nodes: &'a ir::InheritanceTypes,
    pub(crate) semantic: &'a SemanticContext,
}

impl<'a> InheritanceTypesStruct<'a> {
    pub fn iter(&self) -> impl Iterator<Item = InheritanceType<'a>> + use<'a> {
        let semantic = self.semantic;
        self.ir_nodes
            .iter()
            .map(move |ir_node| create_inheritance_type(ir_node, semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type InterfaceMembers<'a> = InterfaceMembersStruct<'a>;

pub(crate) fn create_interface_members<'a>(
    nodes: &'a ir::InterfaceMembers,
    semantic: &'a SemanticContext,
) -> InterfaceMembers<'a> {
    InterfaceMembersStruct {
        ir_nodes: nodes,
        semantic,
    }
}

#[derive(Clone, Copy)]
pub struct InterfaceMembersStruct<'a> {
    pub(crate) ir_nodes: &'a ir::InterfaceMembers,
    pub(crate) semantic: &'a SemanticContext,
}

impl<'a> InterfaceMembersStruct<'a> {
    pub fn iter(&self) -> impl Iterator<Item = ContractMember<'a>> + use<'a> {
        let semantic = self.semantic;
        self.ir_nodes
            .iter()
            .map(move |ir_node| create_contract_member(ir_node, semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type LibraryMembers<'a> = LibraryMembersStruct<'a>;

pub(crate) fn create_library_members<'a>(
    nodes: &'a ir::LibraryMembers,
    semantic: &'a SemanticContext,
) -> LibraryMembers<'a> {
    LibraryMembersStruct {
        ir_nodes: nodes,
        semantic,
    }
}

#[derive(Clone, Copy)]
pub struct LibraryMembersStruct<'a> {
    pub(crate) ir_nodes: &'a ir::LibraryMembers,
    pub(crate) semantic: &'a SemanticContext,
}

impl<'a> LibraryMembersStruct<'a> {
    pub fn iter(&self) -> impl Iterator<Item = ContractMember<'a>> + use<'a> {
        let semantic = self.semantic;
        self.ir_nodes
            .iter()
            .map(move |ir_node| create_contract_member(ir_node, semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type ModifierInvocations<'a> = ModifierInvocationsStruct<'a>;

pub(crate) fn create_modifier_invocations<'a>(
    nodes: &'a ir::ModifierInvocations,
    semantic: &'a SemanticContext,
) -> ModifierInvocations<'a> {
    ModifierInvocationsStruct {
        ir_nodes: nodes,
        semantic,
    }
}

#[derive(Clone, Copy)]
pub struct ModifierInvocationsStruct<'a> {
    pub(crate) ir_nodes: &'a ir::ModifierInvocations,
    pub(crate) semantic: &'a SemanticContext,
}

impl<'a> ModifierInvocationsStruct<'a> {
    pub fn iter(&self) -> impl Iterator<Item = ModifierInvocation<'a>> + use<'a> {
        let semantic = self.semantic;
        self.ir_nodes
            .iter()
            .map(move |ir_node| create_modifier_invocation(ir_node, semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type MultiTypedDeclarationElements<'a> = MultiTypedDeclarationElementsStruct<'a>;

pub(crate) fn create_multi_typed_declaration_elements<'a>(
    nodes: &'a ir::MultiTypedDeclarationElements,
    semantic: &'a SemanticContext,
) -> MultiTypedDeclarationElements<'a> {
    MultiTypedDeclarationElementsStruct {
        ir_nodes: nodes,
        semantic,
    }
}

#[derive(Clone, Copy)]
pub struct MultiTypedDeclarationElementsStruct<'a> {
    pub(crate) ir_nodes: &'a ir::MultiTypedDeclarationElements,
    pub(crate) semantic: &'a SemanticContext,
}

impl<'a> MultiTypedDeclarationElementsStruct<'a> {
    pub fn iter(&self) -> impl Iterator<Item = MultiTypedDeclarationElement<'a>> + use<'a> {
        let semantic = self.semantic;
        self.ir_nodes
            .iter()
            .map(move |ir_node| create_multi_typed_declaration_element(ir_node, semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type NamedArguments<'a> = NamedArgumentsStruct<'a>;

pub(crate) fn create_named_arguments<'a>(
    nodes: &'a ir::NamedArguments,
    semantic: &'a SemanticContext,
) -> NamedArguments<'a> {
    NamedArgumentsStruct {
        ir_nodes: nodes,
        semantic,
    }
}

#[derive(Clone, Copy)]
pub struct NamedArgumentsStruct<'a> {
    pub(crate) ir_nodes: &'a ir::NamedArguments,
    pub(crate) semantic: &'a SemanticContext,
}

impl<'a> NamedArgumentsStruct<'a> {
    pub fn iter(&self) -> impl Iterator<Item = NamedArgument<'a>> + use<'a> {
        let semantic = self.semantic;
        self.ir_nodes
            .iter()
            .map(move |ir_node| create_named_argument(ir_node, semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type OverridePaths<'a> = OverridePathsStruct<'a>;

pub(crate) fn create_override_paths<'a>(
    nodes: &'a ir::OverridePaths,
    semantic: &'a SemanticContext,
) -> OverridePaths<'a> {
    OverridePathsStruct {
        ir_nodes: nodes,
        semantic,
    }
}

#[derive(Clone, Copy)]
pub struct OverridePathsStruct<'a> {
    pub(crate) ir_nodes: &'a ir::OverridePaths,
    pub(crate) semantic: &'a SemanticContext,
}

impl<'a> OverridePathsStruct<'a> {
    pub fn iter(&self) -> impl Iterator<Item = IdentifierPath<'a>> + use<'a> {
        let semantic = self.semantic;
        self.ir_nodes
            .iter()
            .map(move |ir_node| create_identifier_path(ir_node, semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type Parameters<'a> = ParametersStruct<'a>;

pub(crate) fn create_parameters<'a>(
    nodes: &'a ir::Parameters,
    semantic: &'a SemanticContext,
) -> Parameters<'a> {
    ParametersStruct {
        ir_nodes: nodes,
        semantic,
    }
}

#[derive(Clone, Copy)]
pub struct ParametersStruct<'a> {
    pub(crate) ir_nodes: &'a ir::Parameters,
    pub(crate) semantic: &'a SemanticContext,
}

impl<'a> ParametersStruct<'a> {
    pub fn iter(&self) -> impl Iterator<Item = Parameter<'a>> + use<'a> {
        let semantic = self.semantic;
        self.ir_nodes
            .iter()
            .map(move |ir_node| create_parameter(ir_node, semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type PositionalArguments<'a> = PositionalArgumentsStruct<'a>;

pub(crate) fn create_positional_arguments<'a>(
    nodes: &'a ir::PositionalArguments,
    semantic: &'a SemanticContext,
) -> PositionalArguments<'a> {
    PositionalArgumentsStruct {
        ir_nodes: nodes,
        semantic,
    }
}

#[derive(Clone, Copy)]
pub struct PositionalArgumentsStruct<'a> {
    pub(crate) ir_nodes: &'a ir::PositionalArguments,
    pub(crate) semantic: &'a SemanticContext,
}

impl<'a> PositionalArgumentsStruct<'a> {
    pub fn iter(&self) -> impl Iterator<Item = Expression<'a>> + use<'a> {
        let semantic = self.semantic;
        self.ir_nodes
            .iter()
            .map(move |ir_node| create_expression(ir_node, semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type SourceUnitMembers<'a> = SourceUnitMembersStruct<'a>;

pub(crate) fn create_source_unit_members<'a>(
    nodes: &'a ir::SourceUnitMembers,
    semantic: &'a SemanticContext,
) -> SourceUnitMembers<'a> {
    SourceUnitMembersStruct {
        ir_nodes: nodes,
        semantic,
    }
}

#[derive(Clone, Copy)]
pub struct SourceUnitMembersStruct<'a> {
    pub(crate) ir_nodes: &'a ir::SourceUnitMembers,
    pub(crate) semantic: &'a SemanticContext,
}

impl<'a> SourceUnitMembersStruct<'a> {
    pub fn iter(&self) -> impl Iterator<Item = SourceUnitMember<'a>> + use<'a> {
        let semantic = self.semantic;
        self.ir_nodes
            .iter()
            .map(move |ir_node| create_source_unit_member(ir_node, semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type Statements<'a> = StatementsStruct<'a>;

pub(crate) fn create_statements<'a>(
    nodes: &'a ir::Statements,
    semantic: &'a SemanticContext,
) -> Statements<'a> {
    StatementsStruct {
        ir_nodes: nodes,
        semantic,
    }
}

#[derive(Clone, Copy)]
pub struct StatementsStruct<'a> {
    pub(crate) ir_nodes: &'a ir::Statements,
    pub(crate) semantic: &'a SemanticContext,
}

impl<'a> StatementsStruct<'a> {
    pub fn iter(&self) -> impl Iterator<Item = Statement<'a>> + use<'a> {
        let semantic = self.semantic;
        self.ir_nodes
            .iter()
            .map(move |ir_node| create_statement(ir_node, semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type StringLiterals<'a> = StringLiteralsStruct<'a>;

pub(crate) fn create_string_literals<'a>(
    nodes: &'a ir::StringLiterals,
    semantic: &'a SemanticContext,
) -> StringLiterals<'a> {
    StringLiteralsStruct {
        ir_nodes: nodes,
        semantic,
    }
}

#[derive(Clone, Copy)]
pub struct StringLiteralsStruct<'a> {
    pub(crate) ir_nodes: &'a ir::StringLiterals,
    pub(crate) semantic: &'a SemanticContext,
}

impl<'a> StringLiteralsStruct<'a> {
    pub fn iter(&self) -> impl Iterator<Item = StringLiteral<'a>> + use<'a> {
        let semantic = self.semantic;
        self.ir_nodes
            .iter()
            .map(move |ir_node| create_string_literal(ir_node, semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type StructMembers<'a> = StructMembersStruct<'a>;

pub(crate) fn create_struct_members<'a>(
    nodes: &'a ir::StructMembers,
    semantic: &'a SemanticContext,
) -> StructMembers<'a> {
    StructMembersStruct {
        ir_nodes: nodes,
        semantic,
    }
}

#[derive(Clone, Copy)]
pub struct StructMembersStruct<'a> {
    pub(crate) ir_nodes: &'a ir::StructMembers,
    pub(crate) semantic: &'a SemanticContext,
}

impl<'a> StructMembersStruct<'a> {
    pub fn iter(&self) -> impl Iterator<Item = StructMember<'a>> + use<'a> {
        let semantic = self.semantic;
        self.ir_nodes
            .iter()
            .map(move |ir_node| create_struct_member(ir_node, semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type TupleValues<'a> = TupleValuesStruct<'a>;

pub(crate) fn create_tuple_values<'a>(
    nodes: &'a ir::TupleValues,
    semantic: &'a SemanticContext,
) -> TupleValues<'a> {
    TupleValuesStruct {
        ir_nodes: nodes,
        semantic,
    }
}

#[derive(Clone, Copy)]
pub struct TupleValuesStruct<'a> {
    pub(crate) ir_nodes: &'a ir::TupleValues,
    pub(crate) semantic: &'a SemanticContext,
}

impl<'a> TupleValuesStruct<'a> {
    pub fn iter(&self) -> impl Iterator<Item = TupleValue<'a>> + use<'a> {
        let semantic = self.semantic;
        self.ir_nodes
            .iter()
            .map(move |ir_node| create_tuple_value(ir_node, semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type UnicodeStringLiterals<'a> = UnicodeStringLiteralsStruct<'a>;

pub(crate) fn create_unicode_string_literals<'a>(
    nodes: &'a ir::UnicodeStringLiterals,
    semantic: &'a SemanticContext,
) -> UnicodeStringLiterals<'a> {
    UnicodeStringLiteralsStruct {
        ir_nodes: nodes,
        semantic,
    }
}

#[derive(Clone, Copy)]
pub struct UnicodeStringLiteralsStruct<'a> {
    pub(crate) ir_nodes: &'a ir::UnicodeStringLiterals,
    pub(crate) semantic: &'a SemanticContext,
}

impl<'a> UnicodeStringLiteralsStruct<'a> {
    pub fn iter(&self) -> impl Iterator<Item = UnicodeStringLiteral<'a>> + use<'a> {
        let semantic = self.semantic;
        self.ir_nodes
            .iter()
            .map(move |ir_node| create_unicode_string_literal(ir_node, semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type UsingDeconstructionSymbols<'a> = UsingDeconstructionSymbolsStruct<'a>;

pub(crate) fn create_using_deconstruction_symbols<'a>(
    nodes: &'a ir::UsingDeconstructionSymbols,
    semantic: &'a SemanticContext,
) -> UsingDeconstructionSymbols<'a> {
    UsingDeconstructionSymbolsStruct {
        ir_nodes: nodes,
        semantic,
    }
}

#[derive(Clone, Copy)]
pub struct UsingDeconstructionSymbolsStruct<'a> {
    pub(crate) ir_nodes: &'a ir::UsingDeconstructionSymbols,
    pub(crate) semantic: &'a SemanticContext,
}

impl<'a> UsingDeconstructionSymbolsStruct<'a> {
    pub fn iter(&self) -> impl Iterator<Item = UsingDeconstructionSymbol<'a>> + use<'a> {
        let semantic = self.semantic;
        self.ir_nodes
            .iter()
            .map(move |ir_node| create_using_deconstruction_symbol(ir_node, semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type VersionPragmaExpressionSet<'a> = VersionPragmaExpressionSetStruct<'a>;

pub(crate) fn create_version_pragma_expression_set<'a>(
    nodes: &'a ir::VersionPragmaExpressionSet,
    semantic: &'a SemanticContext,
) -> VersionPragmaExpressionSet<'a> {
    VersionPragmaExpressionSetStruct {
        ir_nodes: nodes,
        semantic,
    }
}

#[derive(Clone, Copy)]
pub struct VersionPragmaExpressionSetStruct<'a> {
    pub(crate) ir_nodes: &'a ir::VersionPragmaExpressionSet,
    pub(crate) semantic: &'a SemanticContext,
}

impl<'a> VersionPragmaExpressionSetStruct<'a> {
    pub fn iter(&self) -> impl Iterator<Item = VersionPragmaComparator<'a>> + use<'a> {
        let semantic = self.semantic;
        self.ir_nodes
            .iter()
            .map(move |ir_node| create_version_pragma_comparator(ir_node, semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type VersionPragmaExpressionSets<'a> = VersionPragmaExpressionSetsStruct<'a>;

pub(crate) fn create_version_pragma_expression_sets<'a>(
    nodes: &'a ir::VersionPragmaExpressionSets,
    semantic: &'a SemanticContext,
) -> VersionPragmaExpressionSets<'a> {
    VersionPragmaExpressionSetsStruct {
        ir_nodes: nodes,
        semantic,
    }
}

#[derive(Clone, Copy)]
pub struct VersionPragmaExpressionSetsStruct<'a> {
    pub(crate) ir_nodes: &'a ir::VersionPragmaExpressionSets,
    pub(crate) semantic: &'a SemanticContext,
}

impl<'a> VersionPragmaExpressionSetsStruct<'a> {
    pub fn iter(&self) -> impl Iterator<Item = VersionPragmaExpressionSet<'a>> + use<'a> {
        let semantic = self.semantic;
        self.ir_nodes
            .iter()
            .map(move |ir_node| create_version_pragma_expression_set(ir_node, semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type VersionPragmaSpecifier<'a> = VersionPragmaSpecifierStruct<'a>;

pub(crate) fn create_version_pragma_specifier<'a>(
    nodes: &'a ir::VersionPragmaSpecifier,
    semantic: &'a SemanticContext,
) -> VersionPragmaSpecifier<'a> {
    VersionPragmaSpecifierStruct {
        ir_nodes: nodes,
        semantic,
    }
}

#[derive(Clone, Copy)]
pub struct VersionPragmaSpecifierStruct<'a> {
    pub(crate) ir_nodes: &'a ir::VersionPragmaSpecifier,
    pub(crate) semantic: &'a SemanticContext,
}

impl<'a> VersionPragmaSpecifierStruct<'a> {
    pub fn iter(&self) -> impl Iterator<Item = VersionPragmaComponent<'a>> + use<'a> {
        let semantic = self.semantic;
        self.ir_nodes
            .iter()
            .map(move |ir_node| create_version_pragma_component(ir_node, semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type YulArguments<'a> = YulArgumentsStruct<'a>;

pub(crate) fn create_yul_arguments<'a>(
    nodes: &'a ir::YulArguments,
    semantic: &'a SemanticContext,
) -> YulArguments<'a> {
    YulArgumentsStruct {
        ir_nodes: nodes,
        semantic,
    }
}

#[derive(Clone, Copy)]
pub struct YulArgumentsStruct<'a> {
    pub(crate) ir_nodes: &'a ir::YulArguments,
    pub(crate) semantic: &'a SemanticContext,
}

impl<'a> YulArgumentsStruct<'a> {
    pub fn iter(&self) -> impl Iterator<Item = YulExpression<'a>> + use<'a> {
        let semantic = self.semantic;
        self.ir_nodes
            .iter()
            .map(move |ir_node| create_yul_expression(ir_node, semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type YulParameters<'a> = YulParametersStruct<'a>;

pub(crate) fn create_yul_parameters<'a>(
    nodes: &'a ir::YulParameters,
    semantic: &'a SemanticContext,
) -> YulParameters<'a> {
    YulParametersStruct {
        ir_nodes: nodes,
        semantic,
    }
}

#[derive(Clone, Copy)]
pub struct YulParametersStruct<'a> {
    pub(crate) ir_nodes: &'a ir::YulParameters,
    pub(crate) semantic: &'a SemanticContext,
}

impl<'a> YulParametersStruct<'a> {
    pub fn iter(&self) -> impl Iterator<Item = Identifier<'a>> + use<'a> {
        let semantic = self.semantic;
        self.ir_nodes
            .iter()
            .map(move |ir_node| create_identifier(ir_node, semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type YulPath<'a> = YulPathStruct<'a>;

pub(crate) fn create_yul_path<'a>(
    nodes: &'a ir::YulPath,
    semantic: &'a SemanticContext,
) -> YulPath<'a> {
    YulPathStruct {
        ir_nodes: nodes,
        semantic,
    }
}

#[derive(Clone, Copy)]
pub struct YulPathStruct<'a> {
    pub(crate) ir_nodes: &'a ir::YulPath,
    pub(crate) semantic: &'a SemanticContext,
}

impl<'a> YulPathStruct<'a> {
    pub fn iter(&self) -> impl Iterator<Item = Identifier<'a>> + use<'a> {
        let semantic = self.semantic;
        self.ir_nodes
            .iter()
            .map(move |ir_node| create_identifier(ir_node, semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type YulPaths<'a> = YulPathsStruct<'a>;

pub(crate) fn create_yul_paths<'a>(
    nodes: &'a ir::YulPaths,
    semantic: &'a SemanticContext,
) -> YulPaths<'a> {
    YulPathsStruct {
        ir_nodes: nodes,
        semantic,
    }
}

#[derive(Clone, Copy)]
pub struct YulPathsStruct<'a> {
    pub(crate) ir_nodes: &'a ir::YulPaths,
    pub(crate) semantic: &'a SemanticContext,
}

impl<'a> YulPathsStruct<'a> {
    pub fn iter(&self) -> impl Iterator<Item = YulPath<'a>> + use<'a> {
        let semantic = self.semantic;
        self.ir_nodes
            .iter()
            .map(move |ir_node| create_yul_path(ir_node, semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type YulStatements<'a> = YulStatementsStruct<'a>;

pub(crate) fn create_yul_statements<'a>(
    nodes: &'a ir::YulStatements,
    semantic: &'a SemanticContext,
) -> YulStatements<'a> {
    YulStatementsStruct {
        ir_nodes: nodes,
        semantic,
    }
}

#[derive(Clone, Copy)]
pub struct YulStatementsStruct<'a> {
    pub(crate) ir_nodes: &'a ir::YulStatements,
    pub(crate) semantic: &'a SemanticContext,
}

impl<'a> YulStatementsStruct<'a> {
    pub fn iter(&self) -> impl Iterator<Item = YulStatement<'a>> + use<'a> {
        let semantic = self.semantic;
        self.ir_nodes
            .iter()
            .map(move |ir_node| create_yul_statement(ir_node, semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type YulValueCases<'a> = YulValueCasesStruct<'a>;

pub(crate) fn create_yul_value_cases<'a>(
    nodes: &'a ir::YulValueCases,
    semantic: &'a SemanticContext,
) -> YulValueCases<'a> {
    YulValueCasesStruct {
        ir_nodes: nodes,
        semantic,
    }
}

#[derive(Clone, Copy)]
pub struct YulValueCasesStruct<'a> {
    pub(crate) ir_nodes: &'a ir::YulValueCases,
    pub(crate) semantic: &'a SemanticContext,
}

impl<'a> YulValueCasesStruct<'a> {
    pub fn iter(&self) -> impl Iterator<Item = YulValueCase<'a>> + use<'a> {
        let semantic = self.semantic;
        self.ir_nodes
            .iter()
            .map(move |ir_node| create_yul_value_case(ir_node, semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}
pub type YulVariableNames<'a> = YulVariableNamesStruct<'a>;

pub(crate) fn create_yul_variable_names<'a>(
    nodes: &'a ir::YulVariableNames,
    semantic: &'a SemanticContext,
) -> YulVariableNames<'a> {
    YulVariableNamesStruct {
        ir_nodes: nodes,
        semantic,
    }
}

#[derive(Clone, Copy)]
pub struct YulVariableNamesStruct<'a> {
    pub(crate) ir_nodes: &'a ir::YulVariableNames,
    pub(crate) semantic: &'a SemanticContext,
}

impl<'a> YulVariableNamesStruct<'a> {
    pub fn iter(&self) -> impl Iterator<Item = Identifier<'a>> + use<'a> {
        let semantic = self.semantic;
        self.ir_nodes
            .iter()
            .map(move |ir_node| create_identifier(ir_node, semantic))
    }

    pub fn len(&self) -> usize {
        self.ir_nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ir_nodes.is_empty()
    }
}

//
// Terminals
//

pub type Ampersand<'a> = AmpersandStruct<'a>;

#[derive(Clone, Copy)]
pub struct AmpersandStruct<'a> {
    pub(crate) ir_node: &'a ir::Ampersand,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_ampersand<'a>(
    ir_node: &'a ir::Ampersand,
    semantic: &'a SemanticContext,
) -> Ampersand<'a> {
    AmpersandStruct { ir_node, semantic }
}

impl<'a> AmpersandStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type AmpersandEqual<'a> = AmpersandEqualStruct<'a>;

#[derive(Clone, Copy)]
pub struct AmpersandEqualStruct<'a> {
    pub(crate) ir_node: &'a ir::AmpersandEqual,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_ampersand_equal<'a>(
    ir_node: &'a ir::AmpersandEqual,
    semantic: &'a SemanticContext,
) -> AmpersandEqual<'a> {
    AmpersandEqualStruct { ir_node, semantic }
}

impl<'a> AmpersandEqualStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type Asterisk<'a> = AsteriskStruct<'a>;

#[derive(Clone, Copy)]
pub struct AsteriskStruct<'a> {
    pub(crate) ir_node: &'a ir::Asterisk,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_asterisk<'a>(
    ir_node: &'a ir::Asterisk,
    semantic: &'a SemanticContext,
) -> Asterisk<'a> {
    AsteriskStruct { ir_node, semantic }
}

impl<'a> AsteriskStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type AsteriskEqual<'a> = AsteriskEqualStruct<'a>;

#[derive(Clone, Copy)]
pub struct AsteriskEqualStruct<'a> {
    pub(crate) ir_node: &'a ir::AsteriskEqual,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_asterisk_equal<'a>(
    ir_node: &'a ir::AsteriskEqual,
    semantic: &'a SemanticContext,
) -> AsteriskEqual<'a> {
    AsteriskEqualStruct { ir_node, semantic }
}

impl<'a> AsteriskEqualStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type Bang<'a> = BangStruct<'a>;

#[derive(Clone, Copy)]
pub struct BangStruct<'a> {
    pub(crate) ir_node: &'a ir::Bang,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_bang<'a>(ir_node: &'a ir::Bang, semantic: &'a SemanticContext) -> Bang<'a> {
    BangStruct { ir_node, semantic }
}

impl<'a> BangStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type BangEqual<'a> = BangEqualStruct<'a>;

#[derive(Clone, Copy)]
pub struct BangEqualStruct<'a> {
    pub(crate) ir_node: &'a ir::BangEqual,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_bang_equal<'a>(
    ir_node: &'a ir::BangEqual,
    semantic: &'a SemanticContext,
) -> BangEqual<'a> {
    BangEqualStruct { ir_node, semantic }
}

impl<'a> BangEqualStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type Bar<'a> = BarStruct<'a>;

#[derive(Clone, Copy)]
pub struct BarStruct<'a> {
    pub(crate) ir_node: &'a ir::Bar,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_bar<'a>(ir_node: &'a ir::Bar, semantic: &'a SemanticContext) -> Bar<'a> {
    BarStruct { ir_node, semantic }
}

impl<'a> BarStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type BarEqual<'a> = BarEqualStruct<'a>;

#[derive(Clone, Copy)]
pub struct BarEqualStruct<'a> {
    pub(crate) ir_node: &'a ir::BarEqual,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_bar_equal<'a>(
    ir_node: &'a ir::BarEqual,
    semantic: &'a SemanticContext,
) -> BarEqual<'a> {
    BarEqualStruct { ir_node, semantic }
}

impl<'a> BarEqualStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type BoolKeyword<'a> = BoolKeywordStruct<'a>;

#[derive(Clone, Copy)]
pub struct BoolKeywordStruct<'a> {
    pub(crate) ir_node: &'a ir::BoolKeyword,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_bool_keyword<'a>(
    ir_node: &'a ir::BoolKeyword,
    semantic: &'a SemanticContext,
) -> BoolKeyword<'a> {
    BoolKeywordStruct { ir_node, semantic }
}

impl<'a> BoolKeywordStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type BytesKeyword<'a> = BytesKeywordStruct<'a>;

#[derive(Clone, Copy)]
pub struct BytesKeywordStruct<'a> {
    pub(crate) ir_node: &'a ir::BytesKeyword,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_bytes_keyword<'a>(
    ir_node: &'a ir::BytesKeyword,
    semantic: &'a SemanticContext,
) -> BytesKeyword<'a> {
    BytesKeywordStruct { ir_node, semantic }
}

impl<'a> BytesKeywordStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type CallDataKeyword<'a> = CallDataKeywordStruct<'a>;

#[derive(Clone, Copy)]
pub struct CallDataKeywordStruct<'a> {
    pub(crate) ir_node: &'a ir::CallDataKeyword,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_call_data_keyword<'a>(
    ir_node: &'a ir::CallDataKeyword,
    semantic: &'a SemanticContext,
) -> CallDataKeyword<'a> {
    CallDataKeywordStruct { ir_node, semantic }
}

impl<'a> CallDataKeywordStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type Caret<'a> = CaretStruct<'a>;

#[derive(Clone, Copy)]
pub struct CaretStruct<'a> {
    pub(crate) ir_node: &'a ir::Caret,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_caret<'a>(ir_node: &'a ir::Caret, semantic: &'a SemanticContext) -> Caret<'a> {
    CaretStruct { ir_node, semantic }
}

impl<'a> CaretStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type CaretEqual<'a> = CaretEqualStruct<'a>;

#[derive(Clone, Copy)]
pub struct CaretEqualStruct<'a> {
    pub(crate) ir_node: &'a ir::CaretEqual,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_caret_equal<'a>(
    ir_node: &'a ir::CaretEqual,
    semantic: &'a SemanticContext,
) -> CaretEqual<'a> {
    CaretEqualStruct { ir_node, semantic }
}

impl<'a> CaretEqualStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type DaysKeyword<'a> = DaysKeywordStruct<'a>;

#[derive(Clone, Copy)]
pub struct DaysKeywordStruct<'a> {
    pub(crate) ir_node: &'a ir::DaysKeyword,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_days_keyword<'a>(
    ir_node: &'a ir::DaysKeyword,
    semantic: &'a SemanticContext,
) -> DaysKeyword<'a> {
    DaysKeywordStruct { ir_node, semantic }
}

impl<'a> DaysKeywordStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type DecimalLiteral<'a> = DecimalLiteralStruct<'a>;

#[derive(Clone, Copy)]
pub struct DecimalLiteralStruct<'a> {
    pub(crate) ir_node: &'a ir::DecimalLiteral,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_decimal_literal<'a>(
    ir_node: &'a ir::DecimalLiteral,
    semantic: &'a SemanticContext,
) -> DecimalLiteral<'a> {
    DecimalLiteralStruct { ir_node, semantic }
}

impl<'a> DecimalLiteralStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type DeleteKeyword<'a> = DeleteKeywordStruct<'a>;

#[derive(Clone, Copy)]
pub struct DeleteKeywordStruct<'a> {
    pub(crate) ir_node: &'a ir::DeleteKeyword,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_delete_keyword<'a>(
    ir_node: &'a ir::DeleteKeyword,
    semantic: &'a SemanticContext,
) -> DeleteKeyword<'a> {
    DeleteKeywordStruct { ir_node, semantic }
}

impl<'a> DeleteKeywordStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type Equal<'a> = EqualStruct<'a>;

#[derive(Clone, Copy)]
pub struct EqualStruct<'a> {
    pub(crate) ir_node: &'a ir::Equal,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_equal<'a>(ir_node: &'a ir::Equal, semantic: &'a SemanticContext) -> Equal<'a> {
    EqualStruct { ir_node, semantic }
}

impl<'a> EqualStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type EqualEqual<'a> = EqualEqualStruct<'a>;

#[derive(Clone, Copy)]
pub struct EqualEqualStruct<'a> {
    pub(crate) ir_node: &'a ir::EqualEqual,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_equal_equal<'a>(
    ir_node: &'a ir::EqualEqual,
    semantic: &'a SemanticContext,
) -> EqualEqual<'a> {
    EqualEqualStruct { ir_node, semantic }
}

impl<'a> EqualEqualStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type EtherKeyword<'a> = EtherKeywordStruct<'a>;

#[derive(Clone, Copy)]
pub struct EtherKeywordStruct<'a> {
    pub(crate) ir_node: &'a ir::EtherKeyword,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_ether_keyword<'a>(
    ir_node: &'a ir::EtherKeyword,
    semantic: &'a SemanticContext,
) -> EtherKeyword<'a> {
    EtherKeywordStruct { ir_node, semantic }
}

impl<'a> EtherKeywordStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type FalseKeyword<'a> = FalseKeywordStruct<'a>;

#[derive(Clone, Copy)]
pub struct FalseKeywordStruct<'a> {
    pub(crate) ir_node: &'a ir::FalseKeyword,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_false_keyword<'a>(
    ir_node: &'a ir::FalseKeyword,
    semantic: &'a SemanticContext,
) -> FalseKeyword<'a> {
    FalseKeywordStruct { ir_node, semantic }
}

impl<'a> FalseKeywordStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type FixedKeyword<'a> = FixedKeywordStruct<'a>;

#[derive(Clone, Copy)]
pub struct FixedKeywordStruct<'a> {
    pub(crate) ir_node: &'a ir::FixedKeyword,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_fixed_keyword<'a>(
    ir_node: &'a ir::FixedKeyword,
    semantic: &'a SemanticContext,
) -> FixedKeyword<'a> {
    FixedKeywordStruct { ir_node, semantic }
}

impl<'a> FixedKeywordStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type GreaterThan<'a> = GreaterThanStruct<'a>;

#[derive(Clone, Copy)]
pub struct GreaterThanStruct<'a> {
    pub(crate) ir_node: &'a ir::GreaterThan,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_greater_than<'a>(
    ir_node: &'a ir::GreaterThan,
    semantic: &'a SemanticContext,
) -> GreaterThan<'a> {
    GreaterThanStruct { ir_node, semantic }
}

impl<'a> GreaterThanStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type GreaterThanEqual<'a> = GreaterThanEqualStruct<'a>;

#[derive(Clone, Copy)]
pub struct GreaterThanEqualStruct<'a> {
    pub(crate) ir_node: &'a ir::GreaterThanEqual,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_greater_than_equal<'a>(
    ir_node: &'a ir::GreaterThanEqual,
    semantic: &'a SemanticContext,
) -> GreaterThanEqual<'a> {
    GreaterThanEqualStruct { ir_node, semantic }
}

impl<'a> GreaterThanEqualStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type GreaterThanGreaterThan<'a> = GreaterThanGreaterThanStruct<'a>;

#[derive(Clone, Copy)]
pub struct GreaterThanGreaterThanStruct<'a> {
    pub(crate) ir_node: &'a ir::GreaterThanGreaterThan,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_greater_than_greater_than<'a>(
    ir_node: &'a ir::GreaterThanGreaterThan,
    semantic: &'a SemanticContext,
) -> GreaterThanGreaterThan<'a> {
    GreaterThanGreaterThanStruct { ir_node, semantic }
}

impl<'a> GreaterThanGreaterThanStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type GreaterThanGreaterThanEqual<'a> = GreaterThanGreaterThanEqualStruct<'a>;

#[derive(Clone, Copy)]
pub struct GreaterThanGreaterThanEqualStruct<'a> {
    pub(crate) ir_node: &'a ir::GreaterThanGreaterThanEqual,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_greater_than_greater_than_equal<'a>(
    ir_node: &'a ir::GreaterThanGreaterThanEqual,
    semantic: &'a SemanticContext,
) -> GreaterThanGreaterThanEqual<'a> {
    GreaterThanGreaterThanEqualStruct { ir_node, semantic }
}

impl<'a> GreaterThanGreaterThanEqualStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type GreaterThanGreaterThanGreaterThan<'a> = GreaterThanGreaterThanGreaterThanStruct<'a>;

#[derive(Clone, Copy)]
pub struct GreaterThanGreaterThanGreaterThanStruct<'a> {
    pub(crate) ir_node: &'a ir::GreaterThanGreaterThanGreaterThan,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_greater_than_greater_than_greater_than<'a>(
    ir_node: &'a ir::GreaterThanGreaterThanGreaterThan,
    semantic: &'a SemanticContext,
) -> GreaterThanGreaterThanGreaterThan<'a> {
    GreaterThanGreaterThanGreaterThanStruct { ir_node, semantic }
}

impl<'a> GreaterThanGreaterThanGreaterThanStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type GreaterThanGreaterThanGreaterThanEqual<'a> =
    GreaterThanGreaterThanGreaterThanEqualStruct<'a>;

#[derive(Clone, Copy)]
pub struct GreaterThanGreaterThanGreaterThanEqualStruct<'a> {
    pub(crate) ir_node: &'a ir::GreaterThanGreaterThanGreaterThanEqual,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_greater_than_greater_than_greater_than_equal<'a>(
    ir_node: &'a ir::GreaterThanGreaterThanGreaterThanEqual,
    semantic: &'a SemanticContext,
) -> GreaterThanGreaterThanGreaterThanEqual<'a> {
    GreaterThanGreaterThanGreaterThanEqualStruct { ir_node, semantic }
}

impl<'a> GreaterThanGreaterThanGreaterThanEqualStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type GweiKeyword<'a> = GweiKeywordStruct<'a>;

#[derive(Clone, Copy)]
pub struct GweiKeywordStruct<'a> {
    pub(crate) ir_node: &'a ir::GweiKeyword,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_gwei_keyword<'a>(
    ir_node: &'a ir::GweiKeyword,
    semantic: &'a SemanticContext,
) -> GweiKeyword<'a> {
    GweiKeywordStruct { ir_node, semantic }
}

impl<'a> GweiKeywordStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type HexLiteral<'a> = HexLiteralStruct<'a>;

#[derive(Clone, Copy)]
pub struct HexLiteralStruct<'a> {
    pub(crate) ir_node: &'a ir::HexLiteral,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_hex_literal<'a>(
    ir_node: &'a ir::HexLiteral,
    semantic: &'a SemanticContext,
) -> HexLiteral<'a> {
    HexLiteralStruct { ir_node, semantic }
}

impl<'a> HexLiteralStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type HexStringLiteral<'a> = HexStringLiteralStruct<'a>;

#[derive(Clone, Copy)]
pub struct HexStringLiteralStruct<'a> {
    pub(crate) ir_node: &'a ir::HexStringLiteral,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_hex_string_literal<'a>(
    ir_node: &'a ir::HexStringLiteral,
    semantic: &'a SemanticContext,
) -> HexStringLiteral<'a> {
    HexStringLiteralStruct { ir_node, semantic }
}

impl<'a> HexStringLiteralStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type HoursKeyword<'a> = HoursKeywordStruct<'a>;

#[derive(Clone, Copy)]
pub struct HoursKeywordStruct<'a> {
    pub(crate) ir_node: &'a ir::HoursKeyword,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_hours_keyword<'a>(
    ir_node: &'a ir::HoursKeyword,
    semantic: &'a SemanticContext,
) -> HoursKeyword<'a> {
    HoursKeywordStruct { ir_node, semantic }
}

impl<'a> HoursKeywordStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type Identifier<'a> = IdentifierStruct<'a>;

#[derive(Clone, Copy)]
pub struct IdentifierStruct<'a> {
    pub(crate) ir_node: &'a ir::Identifier,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_identifier<'a>(
    ir_node: &'a ir::Identifier,
    semantic: &'a SemanticContext,
) -> Identifier<'a> {
    IdentifierStruct { ir_node, semantic }
}

impl<'a> IdentifierStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type IntKeyword<'a> = IntKeywordStruct<'a>;

#[derive(Clone, Copy)]
pub struct IntKeywordStruct<'a> {
    pub(crate) ir_node: &'a ir::IntKeyword,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_int_keyword<'a>(
    ir_node: &'a ir::IntKeyword,
    semantic: &'a SemanticContext,
) -> IntKeyword<'a> {
    IntKeywordStruct { ir_node, semantic }
}

impl<'a> IntKeywordStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type LessThan<'a> = LessThanStruct<'a>;

#[derive(Clone, Copy)]
pub struct LessThanStruct<'a> {
    pub(crate) ir_node: &'a ir::LessThan,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_less_than<'a>(
    ir_node: &'a ir::LessThan,
    semantic: &'a SemanticContext,
) -> LessThan<'a> {
    LessThanStruct { ir_node, semantic }
}

impl<'a> LessThanStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type LessThanEqual<'a> = LessThanEqualStruct<'a>;

#[derive(Clone, Copy)]
pub struct LessThanEqualStruct<'a> {
    pub(crate) ir_node: &'a ir::LessThanEqual,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_less_than_equal<'a>(
    ir_node: &'a ir::LessThanEqual,
    semantic: &'a SemanticContext,
) -> LessThanEqual<'a> {
    LessThanEqualStruct { ir_node, semantic }
}

impl<'a> LessThanEqualStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type LessThanLessThan<'a> = LessThanLessThanStruct<'a>;

#[derive(Clone, Copy)]
pub struct LessThanLessThanStruct<'a> {
    pub(crate) ir_node: &'a ir::LessThanLessThan,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_less_than_less_than<'a>(
    ir_node: &'a ir::LessThanLessThan,
    semantic: &'a SemanticContext,
) -> LessThanLessThan<'a> {
    LessThanLessThanStruct { ir_node, semantic }
}

impl<'a> LessThanLessThanStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type LessThanLessThanEqual<'a> = LessThanLessThanEqualStruct<'a>;

#[derive(Clone, Copy)]
pub struct LessThanLessThanEqualStruct<'a> {
    pub(crate) ir_node: &'a ir::LessThanLessThanEqual,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_less_than_less_than_equal<'a>(
    ir_node: &'a ir::LessThanLessThanEqual,
    semantic: &'a SemanticContext,
) -> LessThanLessThanEqual<'a> {
    LessThanLessThanEqualStruct { ir_node, semantic }
}

impl<'a> LessThanLessThanEqualStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type MemoryKeyword<'a> = MemoryKeywordStruct<'a>;

#[derive(Clone, Copy)]
pub struct MemoryKeywordStruct<'a> {
    pub(crate) ir_node: &'a ir::MemoryKeyword,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_memory_keyword<'a>(
    ir_node: &'a ir::MemoryKeyword,
    semantic: &'a SemanticContext,
) -> MemoryKeyword<'a> {
    MemoryKeywordStruct { ir_node, semantic }
}

impl<'a> MemoryKeywordStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type Minus<'a> = MinusStruct<'a>;

#[derive(Clone, Copy)]
pub struct MinusStruct<'a> {
    pub(crate) ir_node: &'a ir::Minus,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_minus<'a>(ir_node: &'a ir::Minus, semantic: &'a SemanticContext) -> Minus<'a> {
    MinusStruct { ir_node, semantic }
}

impl<'a> MinusStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type MinusEqual<'a> = MinusEqualStruct<'a>;

#[derive(Clone, Copy)]
pub struct MinusEqualStruct<'a> {
    pub(crate) ir_node: &'a ir::MinusEqual,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_minus_equal<'a>(
    ir_node: &'a ir::MinusEqual,
    semantic: &'a SemanticContext,
) -> MinusEqual<'a> {
    MinusEqualStruct { ir_node, semantic }
}

impl<'a> MinusEqualStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type MinusMinus<'a> = MinusMinusStruct<'a>;

#[derive(Clone, Copy)]
pub struct MinusMinusStruct<'a> {
    pub(crate) ir_node: &'a ir::MinusMinus,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_minus_minus<'a>(
    ir_node: &'a ir::MinusMinus,
    semantic: &'a SemanticContext,
) -> MinusMinus<'a> {
    MinusMinusStruct { ir_node, semantic }
}

impl<'a> MinusMinusStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type MinutesKeyword<'a> = MinutesKeywordStruct<'a>;

#[derive(Clone, Copy)]
pub struct MinutesKeywordStruct<'a> {
    pub(crate) ir_node: &'a ir::MinutesKeyword,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_minutes_keyword<'a>(
    ir_node: &'a ir::MinutesKeyword,
    semantic: &'a SemanticContext,
) -> MinutesKeyword<'a> {
    MinutesKeywordStruct { ir_node, semantic }
}

impl<'a> MinutesKeywordStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type PayableKeyword<'a> = PayableKeywordStruct<'a>;

#[derive(Clone, Copy)]
pub struct PayableKeywordStruct<'a> {
    pub(crate) ir_node: &'a ir::PayableKeyword,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_payable_keyword<'a>(
    ir_node: &'a ir::PayableKeyword,
    semantic: &'a SemanticContext,
) -> PayableKeyword<'a> {
    PayableKeywordStruct { ir_node, semantic }
}

impl<'a> PayableKeywordStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type Percent<'a> = PercentStruct<'a>;

#[derive(Clone, Copy)]
pub struct PercentStruct<'a> {
    pub(crate) ir_node: &'a ir::Percent,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_percent<'a>(
    ir_node: &'a ir::Percent,
    semantic: &'a SemanticContext,
) -> Percent<'a> {
    PercentStruct { ir_node, semantic }
}

impl<'a> PercentStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type PercentEqual<'a> = PercentEqualStruct<'a>;

#[derive(Clone, Copy)]
pub struct PercentEqualStruct<'a> {
    pub(crate) ir_node: &'a ir::PercentEqual,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_percent_equal<'a>(
    ir_node: &'a ir::PercentEqual,
    semantic: &'a SemanticContext,
) -> PercentEqual<'a> {
    PercentEqualStruct { ir_node, semantic }
}

impl<'a> PercentEqualStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type Plus<'a> = PlusStruct<'a>;

#[derive(Clone, Copy)]
pub struct PlusStruct<'a> {
    pub(crate) ir_node: &'a ir::Plus,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_plus<'a>(ir_node: &'a ir::Plus, semantic: &'a SemanticContext) -> Plus<'a> {
    PlusStruct { ir_node, semantic }
}

impl<'a> PlusStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type PlusEqual<'a> = PlusEqualStruct<'a>;

#[derive(Clone, Copy)]
pub struct PlusEqualStruct<'a> {
    pub(crate) ir_node: &'a ir::PlusEqual,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_plus_equal<'a>(
    ir_node: &'a ir::PlusEqual,
    semantic: &'a SemanticContext,
) -> PlusEqual<'a> {
    PlusEqualStruct { ir_node, semantic }
}

impl<'a> PlusEqualStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type PlusPlus<'a> = PlusPlusStruct<'a>;

#[derive(Clone, Copy)]
pub struct PlusPlusStruct<'a> {
    pub(crate) ir_node: &'a ir::PlusPlus,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_plus_plus<'a>(
    ir_node: &'a ir::PlusPlus,
    semantic: &'a SemanticContext,
) -> PlusPlus<'a> {
    PlusPlusStruct { ir_node, semantic }
}

impl<'a> PlusPlusStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type SecondsKeyword<'a> = SecondsKeywordStruct<'a>;

#[derive(Clone, Copy)]
pub struct SecondsKeywordStruct<'a> {
    pub(crate) ir_node: &'a ir::SecondsKeyword,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_seconds_keyword<'a>(
    ir_node: &'a ir::SecondsKeyword,
    semantic: &'a SemanticContext,
) -> SecondsKeyword<'a> {
    SecondsKeywordStruct { ir_node, semantic }
}

impl<'a> SecondsKeywordStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type Semicolon<'a> = SemicolonStruct<'a>;

#[derive(Clone, Copy)]
pub struct SemicolonStruct<'a> {
    pub(crate) ir_node: &'a ir::Semicolon,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_semicolon<'a>(
    ir_node: &'a ir::Semicolon,
    semantic: &'a SemanticContext,
) -> Semicolon<'a> {
    SemicolonStruct { ir_node, semantic }
}

impl<'a> SemicolonStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type Slash<'a> = SlashStruct<'a>;

#[derive(Clone, Copy)]
pub struct SlashStruct<'a> {
    pub(crate) ir_node: &'a ir::Slash,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_slash<'a>(ir_node: &'a ir::Slash, semantic: &'a SemanticContext) -> Slash<'a> {
    SlashStruct { ir_node, semantic }
}

impl<'a> SlashStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type SlashEqual<'a> = SlashEqualStruct<'a>;

#[derive(Clone, Copy)]
pub struct SlashEqualStruct<'a> {
    pub(crate) ir_node: &'a ir::SlashEqual,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_slash_equal<'a>(
    ir_node: &'a ir::SlashEqual,
    semantic: &'a SemanticContext,
) -> SlashEqual<'a> {
    SlashEqualStruct { ir_node, semantic }
}

impl<'a> SlashEqualStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type StorageKeyword<'a> = StorageKeywordStruct<'a>;

#[derive(Clone, Copy)]
pub struct StorageKeywordStruct<'a> {
    pub(crate) ir_node: &'a ir::StorageKeyword,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_storage_keyword<'a>(
    ir_node: &'a ir::StorageKeyword,
    semantic: &'a SemanticContext,
) -> StorageKeyword<'a> {
    StorageKeywordStruct { ir_node, semantic }
}

impl<'a> StorageKeywordStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type StringKeyword<'a> = StringKeywordStruct<'a>;

#[derive(Clone, Copy)]
pub struct StringKeywordStruct<'a> {
    pub(crate) ir_node: &'a ir::StringKeyword,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_string_keyword<'a>(
    ir_node: &'a ir::StringKeyword,
    semantic: &'a SemanticContext,
) -> StringKeyword<'a> {
    StringKeywordStruct { ir_node, semantic }
}

impl<'a> StringKeywordStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type StringLiteral<'a> = StringLiteralStruct<'a>;

#[derive(Clone, Copy)]
pub struct StringLiteralStruct<'a> {
    pub(crate) ir_node: &'a ir::StringLiteral,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_string_literal<'a>(
    ir_node: &'a ir::StringLiteral,
    semantic: &'a SemanticContext,
) -> StringLiteral<'a> {
    StringLiteralStruct { ir_node, semantic }
}

impl<'a> StringLiteralStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type SuperKeyword<'a> = SuperKeywordStruct<'a>;

#[derive(Clone, Copy)]
pub struct SuperKeywordStruct<'a> {
    pub(crate) ir_node: &'a ir::SuperKeyword,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_super_keyword<'a>(
    ir_node: &'a ir::SuperKeyword,
    semantic: &'a SemanticContext,
) -> SuperKeyword<'a> {
    SuperKeywordStruct { ir_node, semantic }
}

impl<'a> SuperKeywordStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type ThisKeyword<'a> = ThisKeywordStruct<'a>;

#[derive(Clone, Copy)]
pub struct ThisKeywordStruct<'a> {
    pub(crate) ir_node: &'a ir::ThisKeyword,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_this_keyword<'a>(
    ir_node: &'a ir::ThisKeyword,
    semantic: &'a SemanticContext,
) -> ThisKeyword<'a> {
    ThisKeywordStruct { ir_node, semantic }
}

impl<'a> ThisKeywordStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type Tilde<'a> = TildeStruct<'a>;

#[derive(Clone, Copy)]
pub struct TildeStruct<'a> {
    pub(crate) ir_node: &'a ir::Tilde,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_tilde<'a>(ir_node: &'a ir::Tilde, semantic: &'a SemanticContext) -> Tilde<'a> {
    TildeStruct { ir_node, semantic }
}

impl<'a> TildeStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type TrueKeyword<'a> = TrueKeywordStruct<'a>;

#[derive(Clone, Copy)]
pub struct TrueKeywordStruct<'a> {
    pub(crate) ir_node: &'a ir::TrueKeyword,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_true_keyword<'a>(
    ir_node: &'a ir::TrueKeyword,
    semantic: &'a SemanticContext,
) -> TrueKeyword<'a> {
    TrueKeywordStruct { ir_node, semantic }
}

impl<'a> TrueKeywordStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type UfixedKeyword<'a> = UfixedKeywordStruct<'a>;

#[derive(Clone, Copy)]
pub struct UfixedKeywordStruct<'a> {
    pub(crate) ir_node: &'a ir::UfixedKeyword,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_ufixed_keyword<'a>(
    ir_node: &'a ir::UfixedKeyword,
    semantic: &'a SemanticContext,
) -> UfixedKeyword<'a> {
    UfixedKeywordStruct { ir_node, semantic }
}

impl<'a> UfixedKeywordStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type UintKeyword<'a> = UintKeywordStruct<'a>;

#[derive(Clone, Copy)]
pub struct UintKeywordStruct<'a> {
    pub(crate) ir_node: &'a ir::UintKeyword,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_uint_keyword<'a>(
    ir_node: &'a ir::UintKeyword,
    semantic: &'a SemanticContext,
) -> UintKeyword<'a> {
    UintKeywordStruct { ir_node, semantic }
}

impl<'a> UintKeywordStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type UnicodeStringLiteral<'a> = UnicodeStringLiteralStruct<'a>;

#[derive(Clone, Copy)]
pub struct UnicodeStringLiteralStruct<'a> {
    pub(crate) ir_node: &'a ir::UnicodeStringLiteral,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_unicode_string_literal<'a>(
    ir_node: &'a ir::UnicodeStringLiteral,
    semantic: &'a SemanticContext,
) -> UnicodeStringLiteral<'a> {
    UnicodeStringLiteralStruct { ir_node, semantic }
}

impl<'a> UnicodeStringLiteralStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type WeeksKeyword<'a> = WeeksKeywordStruct<'a>;

#[derive(Clone, Copy)]
pub struct WeeksKeywordStruct<'a> {
    pub(crate) ir_node: &'a ir::WeeksKeyword,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_weeks_keyword<'a>(
    ir_node: &'a ir::WeeksKeyword,
    semantic: &'a SemanticContext,
) -> WeeksKeyword<'a> {
    WeeksKeywordStruct { ir_node, semantic }
}

impl<'a> WeeksKeywordStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}

pub type WeiKeyword<'a> = WeiKeywordStruct<'a>;

#[derive(Clone, Copy)]
pub struct WeiKeywordStruct<'a> {
    pub(crate) ir_node: &'a ir::WeiKeyword,
    pub(crate) semantic: &'a SemanticContext,
}

pub(crate) fn create_wei_keyword<'a>(
    ir_node: &'a ir::WeiKeyword,
    semantic: &'a SemanticContext,
) -> WeiKeyword<'a> {
    WeiKeywordStruct { ir_node, semantic }
}

impl<'a> WeiKeywordStruct<'a> {
    pub fn node_id(&self) -> NodeId {
        self.ir_node.id()
    }

    pub fn unparse(&self) -> &'a str {
        self.ir_node.unparse()
    }

    pub fn get_type(&self) -> Option<Type<'a>> {
        Type::try_create_for_node_id(self.ir_node.id(), self.semantic)
    }

    pub fn get_file_id(&self) -> &'a FileId {
        self.semantic.file_id_from_node_id(self.ir_node.id())
    }

    pub fn get_text_range(&self) -> &'a Range<usize> {
        &self.ir_node.range
    }
}
