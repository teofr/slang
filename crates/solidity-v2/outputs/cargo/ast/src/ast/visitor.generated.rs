// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#[allow(clippy::wildcard_imports)]
use super::nodes::*;

pub trait Visitor {
    fn enter_abicoder_pragma(&mut self, _node: &AbicoderPragma<'_>) -> bool {
        true
    }
    fn leave_abicoder_pragma(&mut self, _node: &AbicoderPragma<'_>) {}

    fn enter_additive_expression(&mut self, _node: &AdditiveExpression<'_>) -> bool {
        true
    }
    fn leave_additive_expression(&mut self, _node: &AdditiveExpression<'_>) {}

    fn enter_address_type(&mut self, _node: &AddressType<'_>) -> bool {
        true
    }
    fn leave_address_type(&mut self, _node: &AddressType<'_>) {}

    fn enter_and_expression(&mut self, _node: &AndExpression<'_>) -> bool {
        true
    }
    fn leave_and_expression(&mut self, _node: &AndExpression<'_>) {}

    fn enter_array_expression(&mut self, _node: &ArrayExpression<'_>) -> bool {
        true
    }
    fn leave_array_expression(&mut self, _node: &ArrayExpression<'_>) {}

    fn enter_array_type_name(&mut self, _node: &ArrayTypeName<'_>) -> bool {
        true
    }
    fn leave_array_type_name(&mut self, _node: &ArrayTypeName<'_>) {}

    fn enter_assembly_statement(&mut self, _node: &AssemblyStatement<'_>) -> bool {
        true
    }
    fn leave_assembly_statement(&mut self, _node: &AssemblyStatement<'_>) {}

    fn enter_assignment_expression(&mut self, _node: &AssignmentExpression<'_>) -> bool {
        true
    }
    fn leave_assignment_expression(&mut self, _node: &AssignmentExpression<'_>) {}

    fn enter_bitwise_and_expression(&mut self, _node: &BitwiseAndExpression<'_>) -> bool {
        true
    }
    fn leave_bitwise_and_expression(&mut self, _node: &BitwiseAndExpression<'_>) {}

    fn enter_bitwise_or_expression(&mut self, _node: &BitwiseOrExpression<'_>) -> bool {
        true
    }
    fn leave_bitwise_or_expression(&mut self, _node: &BitwiseOrExpression<'_>) {}

    fn enter_bitwise_xor_expression(&mut self, _node: &BitwiseXorExpression<'_>) -> bool {
        true
    }
    fn leave_bitwise_xor_expression(&mut self, _node: &BitwiseXorExpression<'_>) {}

    fn enter_block(&mut self, _node: &Block<'_>) -> bool {
        true
    }
    fn leave_block(&mut self, _node: &Block<'_>) {}

    fn enter_break_statement(&mut self, _node: &BreakStatement<'_>) -> bool {
        true
    }
    fn leave_break_statement(&mut self, _node: &BreakStatement<'_>) {}

    fn enter_call_options_expression(&mut self, _node: &CallOptionsExpression<'_>) -> bool {
        true
    }
    fn leave_call_options_expression(&mut self, _node: &CallOptionsExpression<'_>) {}

    fn enter_catch_clause(&mut self, _node: &CatchClause<'_>) -> bool {
        true
    }
    fn leave_catch_clause(&mut self, _node: &CatchClause<'_>) {}

    fn enter_conditional_expression(&mut self, _node: &ConditionalExpression<'_>) -> bool {
        true
    }
    fn leave_conditional_expression(&mut self, _node: &ConditionalExpression<'_>) {}

    fn enter_constant_definition(&mut self, _node: &ConstantDefinition<'_>) -> bool {
        true
    }
    fn leave_constant_definition(&mut self, _node: &ConstantDefinition<'_>) {}

    fn enter_continue_statement(&mut self, _node: &ContinueStatement<'_>) -> bool {
        true
    }
    fn leave_continue_statement(&mut self, _node: &ContinueStatement<'_>) {}

    fn enter_contract_definition(&mut self, _node: &ContractDefinition<'_>) -> bool {
        true
    }
    fn leave_contract_definition(&mut self, _node: &ContractDefinition<'_>) {}

    fn enter_decimal_number_expression(&mut self, _node: &DecimalNumberExpression<'_>) -> bool {
        true
    }
    fn leave_decimal_number_expression(&mut self, _node: &DecimalNumberExpression<'_>) {}

    fn enter_do_while_statement(&mut self, _node: &DoWhileStatement<'_>) -> bool {
        true
    }
    fn leave_do_while_statement(&mut self, _node: &DoWhileStatement<'_>) {}

    fn enter_emit_statement(&mut self, _node: &EmitStatement<'_>) -> bool {
        true
    }
    fn leave_emit_statement(&mut self, _node: &EmitStatement<'_>) {}

    fn enter_enum_definition(&mut self, _node: &EnumDefinition<'_>) -> bool {
        true
    }
    fn leave_enum_definition(&mut self, _node: &EnumDefinition<'_>) {}

    fn enter_equality_expression(&mut self, _node: &EqualityExpression<'_>) -> bool {
        true
    }
    fn leave_equality_expression(&mut self, _node: &EqualityExpression<'_>) {}

    fn enter_error_definition(&mut self, _node: &ErrorDefinition<'_>) -> bool {
        true
    }
    fn leave_error_definition(&mut self, _node: &ErrorDefinition<'_>) {}

    fn enter_event_definition(&mut self, _node: &EventDefinition<'_>) -> bool {
        true
    }
    fn leave_event_definition(&mut self, _node: &EventDefinition<'_>) {}

    fn enter_experimental_pragma(&mut self, _node: &ExperimentalPragma<'_>) -> bool {
        true
    }
    fn leave_experimental_pragma(&mut self, _node: &ExperimentalPragma<'_>) {}

    fn enter_exponentiation_expression(&mut self, _node: &ExponentiationExpression<'_>) -> bool {
        true
    }
    fn leave_exponentiation_expression(&mut self, _node: &ExponentiationExpression<'_>) {}

    fn enter_expression_statement(&mut self, _node: &ExpressionStatement<'_>) -> bool {
        true
    }
    fn leave_expression_statement(&mut self, _node: &ExpressionStatement<'_>) {}

    fn enter_for_statement(&mut self, _node: &ForStatement<'_>) -> bool {
        true
    }
    fn leave_for_statement(&mut self, _node: &ForStatement<'_>) {}

    fn enter_function_attributes(&mut self, _node: &FunctionAttributes<'_>) -> bool {
        true
    }
    fn leave_function_attributes(&mut self, _node: &FunctionAttributes<'_>) {}

    fn enter_function_call_expression(&mut self, _node: &FunctionCallExpression<'_>) -> bool {
        true
    }
    fn leave_function_call_expression(&mut self, _node: &FunctionCallExpression<'_>) {}

    fn enter_function_definition(&mut self, _node: &FunctionDefinition<'_>) -> bool {
        true
    }
    fn leave_function_definition(&mut self, _node: &FunctionDefinition<'_>) {}

    fn enter_function_type(&mut self, _node: &FunctionType<'_>) -> bool {
        true
    }
    fn leave_function_type(&mut self, _node: &FunctionType<'_>) {}

    fn enter_function_type_attributes(&mut self, _node: &FunctionTypeAttributes<'_>) -> bool {
        true
    }
    fn leave_function_type_attributes(&mut self, _node: &FunctionTypeAttributes<'_>) {}

    fn enter_hex_number_expression(&mut self, _node: &HexNumberExpression<'_>) -> bool {
        true
    }
    fn leave_hex_number_expression(&mut self, _node: &HexNumberExpression<'_>) {}

    fn enter_if_statement(&mut self, _node: &IfStatement<'_>) -> bool {
        true
    }
    fn leave_if_statement(&mut self, _node: &IfStatement<'_>) {}

    fn enter_import_deconstruction(&mut self, _node: &ImportDeconstruction<'_>) -> bool {
        true
    }
    fn leave_import_deconstruction(&mut self, _node: &ImportDeconstruction<'_>) {}

    fn enter_import_deconstruction_symbol(
        &mut self,
        _node: &ImportDeconstructionSymbol<'_>,
    ) -> bool {
        true
    }
    fn leave_import_deconstruction_symbol(&mut self, _node: &ImportDeconstructionSymbol<'_>) {}

    fn enter_index_access_expression(&mut self, _node: &IndexAccessExpression<'_>) -> bool {
        true
    }
    fn leave_index_access_expression(&mut self, _node: &IndexAccessExpression<'_>) {}

    fn enter_inequality_expression(&mut self, _node: &InequalityExpression<'_>) -> bool {
        true
    }
    fn leave_inequality_expression(&mut self, _node: &InequalityExpression<'_>) {}

    fn enter_inheritance_type(&mut self, _node: &InheritanceType<'_>) -> bool {
        true
    }
    fn leave_inheritance_type(&mut self, _node: &InheritanceType<'_>) {}

    fn enter_interface_definition(&mut self, _node: &InterfaceDefinition<'_>) -> bool {
        true
    }
    fn leave_interface_definition(&mut self, _node: &InterfaceDefinition<'_>) {}

    fn enter_library_definition(&mut self, _node: &LibraryDefinition<'_>) -> bool {
        true
    }
    fn leave_library_definition(&mut self, _node: &LibraryDefinition<'_>) {}

    fn enter_mapping_type(&mut self, _node: &MappingType<'_>) -> bool {
        true
    }
    fn leave_mapping_type(&mut self, _node: &MappingType<'_>) {}

    fn enter_member_access_expression(&mut self, _node: &MemberAccessExpression<'_>) -> bool {
        true
    }
    fn leave_member_access_expression(&mut self, _node: &MemberAccessExpression<'_>) {}

    fn enter_modifier_attributes(&mut self, _node: &ModifierAttributes<'_>) -> bool {
        true
    }
    fn leave_modifier_attributes(&mut self, _node: &ModifierAttributes<'_>) {}

    fn enter_modifier_definition(&mut self, _node: &ModifierDefinition<'_>) -> bool {
        true
    }
    fn leave_modifier_definition(&mut self, _node: &ModifierDefinition<'_>) {}

    fn enter_modifier_invocation(&mut self, _node: &ModifierInvocation<'_>) -> bool {
        true
    }
    fn leave_modifier_invocation(&mut self, _node: &ModifierInvocation<'_>) {}

    fn enter_multi_typed_declaration(&mut self, _node: &MultiTypedDeclaration<'_>) -> bool {
        true
    }
    fn leave_multi_typed_declaration(&mut self, _node: &MultiTypedDeclaration<'_>) {}

    fn enter_multi_typed_declaration_element(
        &mut self,
        _node: &MultiTypedDeclarationElement<'_>,
    ) -> bool {
        true
    }
    fn leave_multi_typed_declaration_element(&mut self, _node: &MultiTypedDeclarationElement<'_>) {}

    fn enter_multiplicative_expression(&mut self, _node: &MultiplicativeExpression<'_>) -> bool {
        true
    }
    fn leave_multiplicative_expression(&mut self, _node: &MultiplicativeExpression<'_>) {}

    fn enter_named_argument(&mut self, _node: &NamedArgument<'_>) -> bool {
        true
    }
    fn leave_named_argument(&mut self, _node: &NamedArgument<'_>) {}

    fn enter_new_expression(&mut self, _node: &NewExpression<'_>) -> bool {
        true
    }
    fn leave_new_expression(&mut self, _node: &NewExpression<'_>) {}

    fn enter_or_expression(&mut self, _node: &OrExpression<'_>) -> bool {
        true
    }
    fn leave_or_expression(&mut self, _node: &OrExpression<'_>) {}

    fn enter_parameter(&mut self, _node: &Parameter<'_>) -> bool {
        true
    }
    fn leave_parameter(&mut self, _node: &Parameter<'_>) {}

    fn enter_path_import(&mut self, _node: &PathImport<'_>) -> bool {
        true
    }
    fn leave_path_import(&mut self, _node: &PathImport<'_>) {}

    fn enter_postfix_expression(&mut self, _node: &PostfixExpression<'_>) -> bool {
        true
    }
    fn leave_postfix_expression(&mut self, _node: &PostfixExpression<'_>) {}

    fn enter_pragma_directive(&mut self, _node: &PragmaDirective<'_>) -> bool {
        true
    }
    fn leave_pragma_directive(&mut self, _node: &PragmaDirective<'_>) {}

    fn enter_prefix_expression(&mut self, _node: &PrefixExpression<'_>) -> bool {
        true
    }
    fn leave_prefix_expression(&mut self, _node: &PrefixExpression<'_>) {}

    fn enter_return_statement(&mut self, _node: &ReturnStatement<'_>) -> bool {
        true
    }
    fn leave_return_statement(&mut self, _node: &ReturnStatement<'_>) {}

    fn enter_revert_statement(&mut self, _node: &RevertStatement<'_>) -> bool {
        true
    }
    fn leave_revert_statement(&mut self, _node: &RevertStatement<'_>) {}

    fn enter_shift_expression(&mut self, _node: &ShiftExpression<'_>) -> bool {
        true
    }
    fn leave_shift_expression(&mut self, _node: &ShiftExpression<'_>) {}

    fn enter_single_typed_declaration(&mut self, _node: &SingleTypedDeclaration<'_>) -> bool {
        true
    }
    fn leave_single_typed_declaration(&mut self, _node: &SingleTypedDeclaration<'_>) {}

    fn enter_source_unit(&mut self, _node: &SourceUnit<'_>) -> bool {
        true
    }
    fn leave_source_unit(&mut self, _node: &SourceUnit<'_>) {}

    fn enter_state_variable_attributes(&mut self, _node: &StateVariableAttributes<'_>) -> bool {
        true
    }
    fn leave_state_variable_attributes(&mut self, _node: &StateVariableAttributes<'_>) {}

    fn enter_state_variable_definition(&mut self, _node: &StateVariableDefinition<'_>) -> bool {
        true
    }
    fn leave_state_variable_definition(&mut self, _node: &StateVariableDefinition<'_>) {}

    fn enter_struct_definition(&mut self, _node: &StructDefinition<'_>) -> bool {
        true
    }
    fn leave_struct_definition(&mut self, _node: &StructDefinition<'_>) {}

    fn enter_struct_member(&mut self, _node: &StructMember<'_>) -> bool {
        true
    }
    fn leave_struct_member(&mut self, _node: &StructMember<'_>) {}

    fn enter_try_statement(&mut self, _node: &TryStatement<'_>) -> bool {
        true
    }
    fn leave_try_statement(&mut self, _node: &TryStatement<'_>) {}

    fn enter_tuple_expression(&mut self, _node: &TupleExpression<'_>) -> bool {
        true
    }
    fn leave_tuple_expression(&mut self, _node: &TupleExpression<'_>) {}

    fn enter_tuple_value(&mut self, _node: &TupleValue<'_>) -> bool {
        true
    }
    fn leave_tuple_value(&mut self, _node: &TupleValue<'_>) {}

    fn enter_type_expression(&mut self, _node: &TypeExpression<'_>) -> bool {
        true
    }
    fn leave_type_expression(&mut self, _node: &TypeExpression<'_>) {}

    fn enter_unchecked_block(&mut self, _node: &UncheckedBlock<'_>) -> bool {
        true
    }
    fn leave_unchecked_block(&mut self, _node: &UncheckedBlock<'_>) {}

    fn enter_user_defined_value_type_definition(
        &mut self,
        _node: &UserDefinedValueTypeDefinition<'_>,
    ) -> bool {
        true
    }
    fn leave_user_defined_value_type_definition(
        &mut self,
        _node: &UserDefinedValueTypeDefinition<'_>,
    ) {
    }

    fn enter_using_deconstruction(&mut self, _node: &UsingDeconstruction<'_>) -> bool {
        true
    }
    fn leave_using_deconstruction(&mut self, _node: &UsingDeconstruction<'_>) {}

    fn enter_using_deconstruction_symbol(&mut self, _node: &UsingDeconstructionSymbol<'_>) -> bool {
        true
    }
    fn leave_using_deconstruction_symbol(&mut self, _node: &UsingDeconstructionSymbol<'_>) {}

    fn enter_using_directive(&mut self, _node: &UsingDirective<'_>) -> bool {
        true
    }
    fn leave_using_directive(&mut self, _node: &UsingDirective<'_>) {}

    fn enter_variable_declaration(&mut self, _node: &VariableDeclaration<'_>) -> bool {
        true
    }
    fn leave_variable_declaration(&mut self, _node: &VariableDeclaration<'_>) {}

    fn enter_variable_declaration_statement(
        &mut self,
        _node: &VariableDeclarationStatement<'_>,
    ) -> bool {
        true
    }
    fn leave_variable_declaration_statement(&mut self, _node: &VariableDeclarationStatement<'_>) {}

    fn enter_version_pragma(&mut self, _node: &VersionPragma<'_>) -> bool {
        true
    }
    fn leave_version_pragma(&mut self, _node: &VersionPragma<'_>) {}

    fn enter_version_pragma_comparator(&mut self, _node: &VersionPragmaComparator<'_>) -> bool {
        true
    }
    fn leave_version_pragma_comparator(&mut self, _node: &VersionPragmaComparator<'_>) {}

    fn enter_while_statement(&mut self, _node: &WhileStatement<'_>) -> bool {
        true
    }
    fn leave_while_statement(&mut self, _node: &WhileStatement<'_>) {}

    fn enter_yul_block(&mut self, _node: &YulBlock<'_>) -> bool {
        true
    }
    fn leave_yul_block(&mut self, _node: &YulBlock<'_>) {}

    fn enter_yul_break_statement(&mut self, _node: &YulBreakStatement<'_>) -> bool {
        true
    }
    fn leave_yul_break_statement(&mut self, _node: &YulBreakStatement<'_>) {}

    fn enter_yul_continue_statement(&mut self, _node: &YulContinueStatement<'_>) -> bool {
        true
    }
    fn leave_yul_continue_statement(&mut self, _node: &YulContinueStatement<'_>) {}

    fn enter_yul_default_case(&mut self, _node: &YulDefaultCase<'_>) -> bool {
        true
    }
    fn leave_yul_default_case(&mut self, _node: &YulDefaultCase<'_>) {}

    fn enter_yul_for_statement(&mut self, _node: &YulForStatement<'_>) -> bool {
        true
    }
    fn leave_yul_for_statement(&mut self, _node: &YulForStatement<'_>) {}

    fn enter_yul_function_call_expression(
        &mut self,
        _node: &YulFunctionCallExpression<'_>,
    ) -> bool {
        true
    }
    fn leave_yul_function_call_expression(&mut self, _node: &YulFunctionCallExpression<'_>) {}

    fn enter_yul_function_definition(&mut self, _node: &YulFunctionDefinition<'_>) -> bool {
        true
    }
    fn leave_yul_function_definition(&mut self, _node: &YulFunctionDefinition<'_>) {}

    fn enter_yul_if_statement(&mut self, _node: &YulIfStatement<'_>) -> bool {
        true
    }
    fn leave_yul_if_statement(&mut self, _node: &YulIfStatement<'_>) {}

    fn enter_yul_leave_statement(&mut self, _node: &YulLeaveStatement<'_>) -> bool {
        true
    }
    fn leave_yul_leave_statement(&mut self, _node: &YulLeaveStatement<'_>) {}

    fn enter_yul_switch_statement(&mut self, _node: &YulSwitchStatement<'_>) -> bool {
        true
    }
    fn leave_yul_switch_statement(&mut self, _node: &YulSwitchStatement<'_>) {}

    fn enter_yul_value_case(&mut self, _node: &YulValueCase<'_>) -> bool {
        true
    }
    fn leave_yul_value_case(&mut self, _node: &YulValueCase<'_>) {}

    fn enter_yul_variable_assignment_statement(
        &mut self,
        _node: &YulVariableAssignmentStatement<'_>,
    ) -> bool {
        true
    }
    fn leave_yul_variable_assignment_statement(
        &mut self,
        _node: &YulVariableAssignmentStatement<'_>,
    ) {
    }

    fn enter_yul_variable_declaration_statement(
        &mut self,
        _node: &YulVariableDeclarationStatement<'_>,
    ) -> bool {
        true
    }
    fn leave_yul_variable_declaration_statement(
        &mut self,
        _node: &YulVariableDeclarationStatement<'_>,
    ) {
    }

    fn enter_yul_variable_declaration_value(
        &mut self,
        _node: &YulVariableDeclarationValue<'_>,
    ) -> bool {
        true
    }
    fn leave_yul_variable_declaration_value(&mut self, _node: &YulVariableDeclarationValue<'_>) {}

    fn enter_abicoder_version(&mut self, _node: &AbicoderVersion) -> bool {
        true
    }
    fn leave_abicoder_version(&mut self, _node: &AbicoderVersion) {}

    fn enter_additive_expression_operator(
        &mut self,
        _node: &AdditiveExpressionOperator<'_>,
    ) -> bool {
        true
    }
    fn leave_additive_expression_operator(&mut self, _node: &AdditiveExpressionOperator<'_>) {}

    fn enter_arguments_declaration(&mut self, _node: &ArgumentsDeclaration<'_>) -> bool {
        true
    }
    fn leave_arguments_declaration(&mut self, _node: &ArgumentsDeclaration<'_>) {}

    fn enter_assignment_expression_operator(
        &mut self,
        _node: &AssignmentExpressionOperator<'_>,
    ) -> bool {
        true
    }
    fn leave_assignment_expression_operator(&mut self, _node: &AssignmentExpressionOperator<'_>) {}

    fn enter_catch_clause_kind(&mut self, _node: &CatchClauseKind) -> bool {
        true
    }
    fn leave_catch_clause_kind(&mut self, _node: &CatchClauseKind) {}

    fn enter_contract_member(&mut self, _node: &ContractMember<'_>) -> bool {
        true
    }
    fn leave_contract_member(&mut self, _node: &ContractMember<'_>) {}

    fn enter_elementary_type(&mut self, _node: &ElementaryType<'_>) -> bool {
        true
    }
    fn leave_elementary_type(&mut self, _node: &ElementaryType<'_>) {}

    fn enter_equality_expression_operator(
        &mut self,
        _node: &EqualityExpressionOperator<'_>,
    ) -> bool {
        true
    }
    fn leave_equality_expression_operator(&mut self, _node: &EqualityExpressionOperator<'_>) {}

    fn enter_experimental_feature(&mut self, _node: &ExperimentalFeature) -> bool {
        true
    }
    fn leave_experimental_feature(&mut self, _node: &ExperimentalFeature) {}

    fn enter_expression(&mut self, _node: &Expression<'_>) -> bool {
        true
    }
    fn leave_expression(&mut self, _node: &Expression<'_>) {}

    fn enter_for_statement_condition(&mut self, _node: &ForStatementCondition<'_>) -> bool {
        true
    }
    fn leave_for_statement_condition(&mut self, _node: &ForStatementCondition<'_>) {}

    fn enter_for_statement_initialization(
        &mut self,
        _node: &ForStatementInitialization<'_>,
    ) -> bool {
        true
    }
    fn leave_for_statement_initialization(&mut self, _node: &ForStatementInitialization<'_>) {}

    fn enter_function_kind(&mut self, _node: &FunctionKind) -> bool {
        true
    }
    fn leave_function_kind(&mut self, _node: &FunctionKind) {}

    fn enter_function_mutability(&mut self, _node: &FunctionMutability) -> bool {
        true
    }
    fn leave_function_mutability(&mut self, _node: &FunctionMutability) {}

    fn enter_function_visibility(&mut self, _node: &FunctionVisibility) -> bool {
        true
    }
    fn leave_function_visibility(&mut self, _node: &FunctionVisibility) {}

    fn enter_import_clause(&mut self, _node: &ImportClause<'_>) -> bool {
        true
    }
    fn leave_import_clause(&mut self, _node: &ImportClause<'_>) {}

    fn enter_inequality_expression_operator(
        &mut self,
        _node: &InequalityExpressionOperator<'_>,
    ) -> bool {
        true
    }
    fn leave_inequality_expression_operator(&mut self, _node: &InequalityExpressionOperator<'_>) {}

    fn enter_multiplicative_expression_operator(
        &mut self,
        _node: &MultiplicativeExpressionOperator<'_>,
    ) -> bool {
        true
    }
    fn leave_multiplicative_expression_operator(
        &mut self,
        _node: &MultiplicativeExpressionOperator<'_>,
    ) {
    }

    fn enter_number_unit(&mut self, _node: &NumberUnit<'_>) -> bool {
        true
    }
    fn leave_number_unit(&mut self, _node: &NumberUnit<'_>) {}

    fn enter_postfix_expression_operator(&mut self, _node: &PostfixExpressionOperator<'_>) -> bool {
        true
    }
    fn leave_postfix_expression_operator(&mut self, _node: &PostfixExpressionOperator<'_>) {}

    fn enter_pragma(&mut self, _node: &Pragma<'_>) -> bool {
        true
    }
    fn leave_pragma(&mut self, _node: &Pragma<'_>) {}

    fn enter_prefix_expression_operator(&mut self, _node: &PrefixExpressionOperator<'_>) -> bool {
        true
    }
    fn leave_prefix_expression_operator(&mut self, _node: &PrefixExpressionOperator<'_>) {}

    fn enter_shift_expression_operator(&mut self, _node: &ShiftExpressionOperator<'_>) -> bool {
        true
    }
    fn leave_shift_expression_operator(&mut self, _node: &ShiftExpressionOperator<'_>) {}

    fn enter_source_unit_member(&mut self, _node: &SourceUnitMember<'_>) -> bool {
        true
    }
    fn leave_source_unit_member(&mut self, _node: &SourceUnitMember<'_>) {}

    fn enter_state_variable_mutability(&mut self, _node: &StateVariableMutability) -> bool {
        true
    }
    fn leave_state_variable_mutability(&mut self, _node: &StateVariableMutability) {}

    fn enter_state_variable_visibility(&mut self, _node: &StateVariableVisibility) -> bool {
        true
    }
    fn leave_state_variable_visibility(&mut self, _node: &StateVariableVisibility) {}

    fn enter_statement(&mut self, _node: &Statement<'_>) -> bool {
        true
    }
    fn leave_statement(&mut self, _node: &Statement<'_>) {}

    fn enter_storage_location(&mut self, _node: &StorageLocation<'_>) -> bool {
        true
    }
    fn leave_storage_location(&mut self, _node: &StorageLocation<'_>) {}

    fn enter_string_expression(&mut self, _node: &StringExpression<'_>) -> bool {
        true
    }
    fn leave_string_expression(&mut self, _node: &StringExpression<'_>) {}

    fn enter_type_name(&mut self, _node: &TypeName<'_>) -> bool {
        true
    }
    fn leave_type_name(&mut self, _node: &TypeName<'_>) {}

    fn enter_using_clause(&mut self, _node: &UsingClause<'_>) -> bool {
        true
    }
    fn leave_using_clause(&mut self, _node: &UsingClause<'_>) {}

    fn enter_using_operator(&mut self, _node: &UsingOperator<'_>) -> bool {
        true
    }
    fn leave_using_operator(&mut self, _node: &UsingOperator<'_>) {}

    fn enter_using_target(&mut self, _node: &UsingTarget<'_>) -> bool {
        true
    }
    fn leave_using_target(&mut self, _node: &UsingTarget<'_>) {}

    fn enter_variable_declaration_target(&mut self, _node: &VariableDeclarationTarget<'_>) -> bool {
        true
    }
    fn leave_variable_declaration_target(&mut self, _node: &VariableDeclarationTarget<'_>) {}

    fn enter_version_pragma_component(&mut self, _node: &VersionPragmaComponent<'_>) -> bool {
        true
    }
    fn leave_version_pragma_component(&mut self, _node: &VersionPragmaComponent<'_>) {}

    fn enter_version_pragma_operator(&mut self, _node: &VersionPragmaOperator) -> bool {
        true
    }
    fn leave_version_pragma_operator(&mut self, _node: &VersionPragmaOperator) {}

    fn enter_yul_expression(&mut self, _node: &YulExpression<'_>) -> bool {
        true
    }
    fn leave_yul_expression(&mut self, _node: &YulExpression<'_>) {}

    fn enter_yul_literal(&mut self, _node: &YulLiteral<'_>) -> bool {
        true
    }
    fn leave_yul_literal(&mut self, _node: &YulLiteral<'_>) {}

    fn enter_yul_statement(&mut self, _node: &YulStatement<'_>) -> bool {
        true
    }
    fn leave_yul_statement(&mut self, _node: &YulStatement<'_>) {}

    fn enter_array_values(&mut self, _items: &ArrayValues<'_>) -> bool {
        true
    }
    fn leave_array_values(&mut self, _items: &ArrayValues<'_>) {}

    fn enter_call_options(&mut self, _items: &CallOptions<'_>) -> bool {
        true
    }
    fn leave_call_options(&mut self, _items: &CallOptions<'_>) {}

    fn enter_catch_clauses(&mut self, _items: &CatchClauses<'_>) -> bool {
        true
    }
    fn leave_catch_clauses(&mut self, _items: &CatchClauses<'_>) {}

    fn enter_contract_members(&mut self, _items: &ContractMembers<'_>) -> bool {
        true
    }
    fn leave_contract_members(&mut self, _items: &ContractMembers<'_>) {}

    fn enter_enum_members(&mut self, _items: &EnumMembers<'_>) -> bool {
        true
    }
    fn leave_enum_members(&mut self, _items: &EnumMembers<'_>) {}

    fn enter_hex_string_literals(&mut self, _items: &HexStringLiterals<'_>) -> bool {
        true
    }
    fn leave_hex_string_literals(&mut self, _items: &HexStringLiterals<'_>) {}

    fn enter_identifier_path(&mut self, _items: &IdentifierPath<'_>) -> bool {
        true
    }
    fn leave_identifier_path(&mut self, _items: &IdentifierPath<'_>) {}

    fn enter_import_deconstruction_symbols(
        &mut self,
        _items: &ImportDeconstructionSymbols<'_>,
    ) -> bool {
        true
    }
    fn leave_import_deconstruction_symbols(&mut self, _items: &ImportDeconstructionSymbols<'_>) {}

    fn enter_inheritance_types(&mut self, _items: &InheritanceTypes<'_>) -> bool {
        true
    }
    fn leave_inheritance_types(&mut self, _items: &InheritanceTypes<'_>) {}

    fn enter_interface_members(&mut self, _items: &InterfaceMembers<'_>) -> bool {
        true
    }
    fn leave_interface_members(&mut self, _items: &InterfaceMembers<'_>) {}

    fn enter_library_members(&mut self, _items: &LibraryMembers<'_>) -> bool {
        true
    }
    fn leave_library_members(&mut self, _items: &LibraryMembers<'_>) {}

    fn enter_modifier_invocations(&mut self, _items: &ModifierInvocations<'_>) -> bool {
        true
    }
    fn leave_modifier_invocations(&mut self, _items: &ModifierInvocations<'_>) {}

    fn enter_multi_typed_declaration_elements(
        &mut self,
        _items: &MultiTypedDeclarationElements<'_>,
    ) -> bool {
        true
    }
    fn leave_multi_typed_declaration_elements(
        &mut self,
        _items: &MultiTypedDeclarationElements<'_>,
    ) {
    }

    fn enter_named_arguments(&mut self, _items: &NamedArguments<'_>) -> bool {
        true
    }
    fn leave_named_arguments(&mut self, _items: &NamedArguments<'_>) {}

    fn enter_override_paths(&mut self, _items: &OverridePaths<'_>) -> bool {
        true
    }
    fn leave_override_paths(&mut self, _items: &OverridePaths<'_>) {}

    fn enter_parameters(&mut self, _items: &Parameters<'_>) -> bool {
        true
    }
    fn leave_parameters(&mut self, _items: &Parameters<'_>) {}

    fn enter_positional_arguments(&mut self, _items: &PositionalArguments<'_>) -> bool {
        true
    }
    fn leave_positional_arguments(&mut self, _items: &PositionalArguments<'_>) {}

    fn enter_source_unit_members(&mut self, _items: &SourceUnitMembers<'_>) -> bool {
        true
    }
    fn leave_source_unit_members(&mut self, _items: &SourceUnitMembers<'_>) {}

    fn enter_statements(&mut self, _items: &Statements<'_>) -> bool {
        true
    }
    fn leave_statements(&mut self, _items: &Statements<'_>) {}

    fn enter_string_literals(&mut self, _items: &StringLiterals<'_>) -> bool {
        true
    }
    fn leave_string_literals(&mut self, _items: &StringLiterals<'_>) {}

    fn enter_struct_members(&mut self, _items: &StructMembers<'_>) -> bool {
        true
    }
    fn leave_struct_members(&mut self, _items: &StructMembers<'_>) {}

    fn enter_tuple_values(&mut self, _items: &TupleValues<'_>) -> bool {
        true
    }
    fn leave_tuple_values(&mut self, _items: &TupleValues<'_>) {}

    fn enter_unicode_string_literals(&mut self, _items: &UnicodeStringLiterals<'_>) -> bool {
        true
    }
    fn leave_unicode_string_literals(&mut self, _items: &UnicodeStringLiterals<'_>) {}

    fn enter_using_deconstruction_symbols(
        &mut self,
        _items: &UsingDeconstructionSymbols<'_>,
    ) -> bool {
        true
    }
    fn leave_using_deconstruction_symbols(&mut self, _items: &UsingDeconstructionSymbols<'_>) {}

    fn enter_version_pragma_expression_set(
        &mut self,
        _items: &VersionPragmaExpressionSet<'_>,
    ) -> bool {
        true
    }
    fn leave_version_pragma_expression_set(&mut self, _items: &VersionPragmaExpressionSet<'_>) {}

    fn enter_version_pragma_expression_sets(
        &mut self,
        _items: &VersionPragmaExpressionSets<'_>,
    ) -> bool {
        true
    }
    fn leave_version_pragma_expression_sets(&mut self, _items: &VersionPragmaExpressionSets<'_>) {}

    fn enter_version_pragma_specifier(&mut self, _items: &VersionPragmaSpecifier<'_>) -> bool {
        true
    }
    fn leave_version_pragma_specifier(&mut self, _items: &VersionPragmaSpecifier<'_>) {}

    fn enter_yul_arguments(&mut self, _items: &YulArguments<'_>) -> bool {
        true
    }
    fn leave_yul_arguments(&mut self, _items: &YulArguments<'_>) {}

    fn enter_yul_parameters(&mut self, _items: &YulParameters<'_>) -> bool {
        true
    }
    fn leave_yul_parameters(&mut self, _items: &YulParameters<'_>) {}

    fn enter_yul_path(&mut self, _items: &YulPath<'_>) -> bool {
        true
    }
    fn leave_yul_path(&mut self, _items: &YulPath<'_>) {}

    fn enter_yul_paths(&mut self, _items: &YulPaths<'_>) -> bool {
        true
    }
    fn leave_yul_paths(&mut self, _items: &YulPaths<'_>) {}

    fn enter_yul_statements(&mut self, _items: &YulStatements<'_>) -> bool {
        true
    }
    fn leave_yul_statements(&mut self, _items: &YulStatements<'_>) {}

    fn enter_yul_value_cases(&mut self, _items: &YulValueCases<'_>) -> bool {
        true
    }
    fn leave_yul_value_cases(&mut self, _items: &YulValueCases<'_>) {}

    fn enter_yul_variable_names(&mut self, _items: &YulVariableNames<'_>) -> bool {
        true
    }
    fn leave_yul_variable_names(&mut self, _items: &YulVariableNames<'_>) {}

    fn visit_bytes_keyword(&mut self, _node: &BytesKeyword<'_>) {}

    fn visit_decimal_literal(&mut self, _node: &DecimalLiteral<'_>) {}

    fn visit_fixed_keyword(&mut self, _node: &FixedKeyword<'_>) {}

    fn visit_hex_literal(&mut self, _node: &HexLiteral<'_>) {}

    fn visit_hex_string_literal(&mut self, _node: &HexStringLiteral<'_>) {}

    fn visit_identifier(&mut self, _node: &Identifier<'_>) {}

    fn visit_int_keyword(&mut self, _node: &IntKeyword<'_>) {}

    fn visit_string_literal(&mut self, _node: &StringLiteral<'_>) {}

    fn visit_ufixed_keyword(&mut self, _node: &UfixedKeyword<'_>) {}

    fn visit_uint_keyword(&mut self, _node: &UintKeyword<'_>) {}

    fn visit_unicode_string_literal(&mut self, _node: &UnicodeStringLiteral<'_>) {}
}

/// Implements the `enter_*` and `leave_*` methods of [`Visitor`] for every sequence and collection
/// node by forwarding the node to the caller's `$enter` and, if given, `$leave` methods. Choices keep
/// their defaults: each one spans its variant.
#[macro_export]
macro_rules! impl_nonterminal_visitor_hooks {
    (enter = $enter:ident $(, leave = $leave:ident)?) => {
        fn enter_abicoder_pragma(&mut self, node: &$crate::ast::AbicoderPragmaStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_abicoder_pragma(&mut self, node: &$crate::ast::AbicoderPragmaStruct<'_>) { self.$leave(node); })?

        fn enter_additive_expression(&mut self, node: &$crate::ast::AdditiveExpressionStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_additive_expression(&mut self, node: &$crate::ast::AdditiveExpressionStruct<'_>) { self.$leave(node); })?

        fn enter_address_type(&mut self, node: &$crate::ast::AddressTypeStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_address_type(&mut self, node: &$crate::ast::AddressTypeStruct<'_>) { self.$leave(node); })?

        fn enter_and_expression(&mut self, node: &$crate::ast::AndExpressionStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_and_expression(&mut self, node: &$crate::ast::AndExpressionStruct<'_>) { self.$leave(node); })?

        fn enter_array_expression(&mut self, node: &$crate::ast::ArrayExpressionStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_array_expression(&mut self, node: &$crate::ast::ArrayExpressionStruct<'_>) { self.$leave(node); })?

        fn enter_array_type_name(&mut self, node: &$crate::ast::ArrayTypeNameStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_array_type_name(&mut self, node: &$crate::ast::ArrayTypeNameStruct<'_>) { self.$leave(node); })?

        fn enter_assembly_statement(&mut self, node: &$crate::ast::AssemblyStatementStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_assembly_statement(&mut self, node: &$crate::ast::AssemblyStatementStruct<'_>) { self.$leave(node); })?

        fn enter_assignment_expression(&mut self, node: &$crate::ast::AssignmentExpressionStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_assignment_expression(&mut self, node: &$crate::ast::AssignmentExpressionStruct<'_>) { self.$leave(node); })?

        fn enter_bitwise_and_expression(&mut self, node: &$crate::ast::BitwiseAndExpressionStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_bitwise_and_expression(&mut self, node: &$crate::ast::BitwiseAndExpressionStruct<'_>) { self.$leave(node); })?

        fn enter_bitwise_or_expression(&mut self, node: &$crate::ast::BitwiseOrExpressionStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_bitwise_or_expression(&mut self, node: &$crate::ast::BitwiseOrExpressionStruct<'_>) { self.$leave(node); })?

        fn enter_bitwise_xor_expression(&mut self, node: &$crate::ast::BitwiseXorExpressionStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_bitwise_xor_expression(&mut self, node: &$crate::ast::BitwiseXorExpressionStruct<'_>) { self.$leave(node); })?

        fn enter_block(&mut self, node: &$crate::ast::BlockStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_block(&mut self, node: &$crate::ast::BlockStruct<'_>) { self.$leave(node); })?

        fn enter_break_statement(&mut self, node: &$crate::ast::BreakStatementStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_break_statement(&mut self, node: &$crate::ast::BreakStatementStruct<'_>) { self.$leave(node); })?

        fn enter_call_options_expression(&mut self, node: &$crate::ast::CallOptionsExpressionStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_call_options_expression(&mut self, node: &$crate::ast::CallOptionsExpressionStruct<'_>) { self.$leave(node); })?

        fn enter_catch_clause(&mut self, node: &$crate::ast::CatchClauseStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_catch_clause(&mut self, node: &$crate::ast::CatchClauseStruct<'_>) { self.$leave(node); })?

        fn enter_conditional_expression(&mut self, node: &$crate::ast::ConditionalExpressionStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_conditional_expression(&mut self, node: &$crate::ast::ConditionalExpressionStruct<'_>) { self.$leave(node); })?

        fn enter_constant_definition(&mut self, node: &$crate::ast::ConstantDefinitionStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_constant_definition(&mut self, node: &$crate::ast::ConstantDefinitionStruct<'_>) { self.$leave(node); })?

        fn enter_continue_statement(&mut self, node: &$crate::ast::ContinueStatementStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_continue_statement(&mut self, node: &$crate::ast::ContinueStatementStruct<'_>) { self.$leave(node); })?

        fn enter_contract_definition(&mut self, node: &$crate::ast::ContractDefinitionStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_contract_definition(&mut self, node: &$crate::ast::ContractDefinitionStruct<'_>) { self.$leave(node); })?

        fn enter_decimal_number_expression(&mut self, node: &$crate::ast::DecimalNumberExpressionStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_decimal_number_expression(&mut self, node: &$crate::ast::DecimalNumberExpressionStruct<'_>) { self.$leave(node); })?

        fn enter_do_while_statement(&mut self, node: &$crate::ast::DoWhileStatementStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_do_while_statement(&mut self, node: &$crate::ast::DoWhileStatementStruct<'_>) { self.$leave(node); })?

        fn enter_emit_statement(&mut self, node: &$crate::ast::EmitStatementStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_emit_statement(&mut self, node: &$crate::ast::EmitStatementStruct<'_>) { self.$leave(node); })?

        fn enter_enum_definition(&mut self, node: &$crate::ast::EnumDefinitionStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_enum_definition(&mut self, node: &$crate::ast::EnumDefinitionStruct<'_>) { self.$leave(node); })?

        fn enter_equality_expression(&mut self, node: &$crate::ast::EqualityExpressionStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_equality_expression(&mut self, node: &$crate::ast::EqualityExpressionStruct<'_>) { self.$leave(node); })?

        fn enter_error_definition(&mut self, node: &$crate::ast::ErrorDefinitionStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_error_definition(&mut self, node: &$crate::ast::ErrorDefinitionStruct<'_>) { self.$leave(node); })?

        fn enter_event_definition(&mut self, node: &$crate::ast::EventDefinitionStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_event_definition(&mut self, node: &$crate::ast::EventDefinitionStruct<'_>) { self.$leave(node); })?

        fn enter_experimental_pragma(&mut self, node: &$crate::ast::ExperimentalPragmaStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_experimental_pragma(&mut self, node: &$crate::ast::ExperimentalPragmaStruct<'_>) { self.$leave(node); })?

        fn enter_exponentiation_expression(&mut self, node: &$crate::ast::ExponentiationExpressionStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_exponentiation_expression(&mut self, node: &$crate::ast::ExponentiationExpressionStruct<'_>) { self.$leave(node); })?

        fn enter_expression_statement(&mut self, node: &$crate::ast::ExpressionStatementStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_expression_statement(&mut self, node: &$crate::ast::ExpressionStatementStruct<'_>) { self.$leave(node); })?

        fn enter_for_statement(&mut self, node: &$crate::ast::ForStatementStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_for_statement(&mut self, node: &$crate::ast::ForStatementStruct<'_>) { self.$leave(node); })?

        fn enter_function_attributes(&mut self, node: &$crate::ast::FunctionAttributesStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_function_attributes(&mut self, node: &$crate::ast::FunctionAttributesStruct<'_>) { self.$leave(node); })?

        fn enter_function_call_expression(&mut self, node: &$crate::ast::FunctionCallExpressionStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_function_call_expression(&mut self, node: &$crate::ast::FunctionCallExpressionStruct<'_>) { self.$leave(node); })?

        fn enter_function_definition(&mut self, node: &$crate::ast::FunctionDefinitionStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_function_definition(&mut self, node: &$crate::ast::FunctionDefinitionStruct<'_>) { self.$leave(node); })?

        fn enter_function_type(&mut self, node: &$crate::ast::FunctionTypeStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_function_type(&mut self, node: &$crate::ast::FunctionTypeStruct<'_>) { self.$leave(node); })?

        fn enter_function_type_attributes(&mut self, node: &$crate::ast::FunctionTypeAttributesStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_function_type_attributes(&mut self, node: &$crate::ast::FunctionTypeAttributesStruct<'_>) { self.$leave(node); })?

        fn enter_hex_number_expression(&mut self, node: &$crate::ast::HexNumberExpressionStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_hex_number_expression(&mut self, node: &$crate::ast::HexNumberExpressionStruct<'_>) { self.$leave(node); })?

        fn enter_if_statement(&mut self, node: &$crate::ast::IfStatementStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_if_statement(&mut self, node: &$crate::ast::IfStatementStruct<'_>) { self.$leave(node); })?

        fn enter_import_deconstruction(&mut self, node: &$crate::ast::ImportDeconstructionStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_import_deconstruction(&mut self, node: &$crate::ast::ImportDeconstructionStruct<'_>) { self.$leave(node); })?

        fn enter_import_deconstruction_symbol(&mut self, node: &$crate::ast::ImportDeconstructionSymbolStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_import_deconstruction_symbol(&mut self, node: &$crate::ast::ImportDeconstructionSymbolStruct<'_>) { self.$leave(node); })?

        fn enter_index_access_expression(&mut self, node: &$crate::ast::IndexAccessExpressionStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_index_access_expression(&mut self, node: &$crate::ast::IndexAccessExpressionStruct<'_>) { self.$leave(node); })?

        fn enter_inequality_expression(&mut self, node: &$crate::ast::InequalityExpressionStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_inequality_expression(&mut self, node: &$crate::ast::InequalityExpressionStruct<'_>) { self.$leave(node); })?

        fn enter_inheritance_type(&mut self, node: &$crate::ast::InheritanceTypeStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_inheritance_type(&mut self, node: &$crate::ast::InheritanceTypeStruct<'_>) { self.$leave(node); })?

        fn enter_interface_definition(&mut self, node: &$crate::ast::InterfaceDefinitionStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_interface_definition(&mut self, node: &$crate::ast::InterfaceDefinitionStruct<'_>) { self.$leave(node); })?

        fn enter_library_definition(&mut self, node: &$crate::ast::LibraryDefinitionStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_library_definition(&mut self, node: &$crate::ast::LibraryDefinitionStruct<'_>) { self.$leave(node); })?

        fn enter_mapping_type(&mut self, node: &$crate::ast::MappingTypeStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_mapping_type(&mut self, node: &$crate::ast::MappingTypeStruct<'_>) { self.$leave(node); })?

        fn enter_member_access_expression(&mut self, node: &$crate::ast::MemberAccessExpressionStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_member_access_expression(&mut self, node: &$crate::ast::MemberAccessExpressionStruct<'_>) { self.$leave(node); })?

        fn enter_modifier_attributes(&mut self, node: &$crate::ast::ModifierAttributesStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_modifier_attributes(&mut self, node: &$crate::ast::ModifierAttributesStruct<'_>) { self.$leave(node); })?

        fn enter_modifier_definition(&mut self, node: &$crate::ast::ModifierDefinitionStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_modifier_definition(&mut self, node: &$crate::ast::ModifierDefinitionStruct<'_>) { self.$leave(node); })?

        fn enter_modifier_invocation(&mut self, node: &$crate::ast::ModifierInvocationStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_modifier_invocation(&mut self, node: &$crate::ast::ModifierInvocationStruct<'_>) { self.$leave(node); })?

        fn enter_multi_typed_declaration(&mut self, node: &$crate::ast::MultiTypedDeclarationStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_multi_typed_declaration(&mut self, node: &$crate::ast::MultiTypedDeclarationStruct<'_>) { self.$leave(node); })?

        fn enter_multi_typed_declaration_element(&mut self, node: &$crate::ast::MultiTypedDeclarationElementStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_multi_typed_declaration_element(&mut self, node: &$crate::ast::MultiTypedDeclarationElementStruct<'_>) { self.$leave(node); })?

        fn enter_multiplicative_expression(&mut self, node: &$crate::ast::MultiplicativeExpressionStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_multiplicative_expression(&mut self, node: &$crate::ast::MultiplicativeExpressionStruct<'_>) { self.$leave(node); })?

        fn enter_named_argument(&mut self, node: &$crate::ast::NamedArgumentStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_named_argument(&mut self, node: &$crate::ast::NamedArgumentStruct<'_>) { self.$leave(node); })?

        fn enter_new_expression(&mut self, node: &$crate::ast::NewExpressionStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_new_expression(&mut self, node: &$crate::ast::NewExpressionStruct<'_>) { self.$leave(node); })?

        fn enter_or_expression(&mut self, node: &$crate::ast::OrExpressionStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_or_expression(&mut self, node: &$crate::ast::OrExpressionStruct<'_>) { self.$leave(node); })?

        fn enter_parameter(&mut self, node: &$crate::ast::ParameterStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_parameter(&mut self, node: &$crate::ast::ParameterStruct<'_>) { self.$leave(node); })?

        fn enter_path_import(&mut self, node: &$crate::ast::PathImportStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_path_import(&mut self, node: &$crate::ast::PathImportStruct<'_>) { self.$leave(node); })?

        fn enter_postfix_expression(&mut self, node: &$crate::ast::PostfixExpressionStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_postfix_expression(&mut self, node: &$crate::ast::PostfixExpressionStruct<'_>) { self.$leave(node); })?

        fn enter_pragma_directive(&mut self, node: &$crate::ast::PragmaDirectiveStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_pragma_directive(&mut self, node: &$crate::ast::PragmaDirectiveStruct<'_>) { self.$leave(node); })?

        fn enter_prefix_expression(&mut self, node: &$crate::ast::PrefixExpressionStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_prefix_expression(&mut self, node: &$crate::ast::PrefixExpressionStruct<'_>) { self.$leave(node); })?

        fn enter_return_statement(&mut self, node: &$crate::ast::ReturnStatementStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_return_statement(&mut self, node: &$crate::ast::ReturnStatementStruct<'_>) { self.$leave(node); })?

        fn enter_revert_statement(&mut self, node: &$crate::ast::RevertStatementStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_revert_statement(&mut self, node: &$crate::ast::RevertStatementStruct<'_>) { self.$leave(node); })?

        fn enter_shift_expression(&mut self, node: &$crate::ast::ShiftExpressionStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_shift_expression(&mut self, node: &$crate::ast::ShiftExpressionStruct<'_>) { self.$leave(node); })?

        fn enter_single_typed_declaration(&mut self, node: &$crate::ast::SingleTypedDeclarationStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_single_typed_declaration(&mut self, node: &$crate::ast::SingleTypedDeclarationStruct<'_>) { self.$leave(node); })?

        fn enter_source_unit(&mut self, node: &$crate::ast::SourceUnitStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_source_unit(&mut self, node: &$crate::ast::SourceUnitStruct<'_>) { self.$leave(node); })?

        fn enter_state_variable_attributes(&mut self, node: &$crate::ast::StateVariableAttributesStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_state_variable_attributes(&mut self, node: &$crate::ast::StateVariableAttributesStruct<'_>) { self.$leave(node); })?

        fn enter_state_variable_definition(&mut self, node: &$crate::ast::StateVariableDefinitionStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_state_variable_definition(&mut self, node: &$crate::ast::StateVariableDefinitionStruct<'_>) { self.$leave(node); })?

        fn enter_struct_definition(&mut self, node: &$crate::ast::StructDefinitionStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_struct_definition(&mut self, node: &$crate::ast::StructDefinitionStruct<'_>) { self.$leave(node); })?

        fn enter_struct_member(&mut self, node: &$crate::ast::StructMemberStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_struct_member(&mut self, node: &$crate::ast::StructMemberStruct<'_>) { self.$leave(node); })?

        fn enter_try_statement(&mut self, node: &$crate::ast::TryStatementStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_try_statement(&mut self, node: &$crate::ast::TryStatementStruct<'_>) { self.$leave(node); })?

        fn enter_tuple_expression(&mut self, node: &$crate::ast::TupleExpressionStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_tuple_expression(&mut self, node: &$crate::ast::TupleExpressionStruct<'_>) { self.$leave(node); })?

        fn enter_tuple_value(&mut self, node: &$crate::ast::TupleValueStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_tuple_value(&mut self, node: &$crate::ast::TupleValueStruct<'_>) { self.$leave(node); })?

        fn enter_type_expression(&mut self, node: &$crate::ast::TypeExpressionStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_type_expression(&mut self, node: &$crate::ast::TypeExpressionStruct<'_>) { self.$leave(node); })?

        fn enter_unchecked_block(&mut self, node: &$crate::ast::UncheckedBlockStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_unchecked_block(&mut self, node: &$crate::ast::UncheckedBlockStruct<'_>) { self.$leave(node); })?

        fn enter_user_defined_value_type_definition(&mut self, node: &$crate::ast::UserDefinedValueTypeDefinitionStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_user_defined_value_type_definition(&mut self, node: &$crate::ast::UserDefinedValueTypeDefinitionStruct<'_>) { self.$leave(node); })?

        fn enter_using_deconstruction(&mut self, node: &$crate::ast::UsingDeconstructionStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_using_deconstruction(&mut self, node: &$crate::ast::UsingDeconstructionStruct<'_>) { self.$leave(node); })?

        fn enter_using_deconstruction_symbol(&mut self, node: &$crate::ast::UsingDeconstructionSymbolStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_using_deconstruction_symbol(&mut self, node: &$crate::ast::UsingDeconstructionSymbolStruct<'_>) { self.$leave(node); })?

        fn enter_using_directive(&mut self, node: &$crate::ast::UsingDirectiveStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_using_directive(&mut self, node: &$crate::ast::UsingDirectiveStruct<'_>) { self.$leave(node); })?

        fn enter_variable_declaration(&mut self, node: &$crate::ast::VariableDeclarationStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_variable_declaration(&mut self, node: &$crate::ast::VariableDeclarationStruct<'_>) { self.$leave(node); })?

        fn enter_variable_declaration_statement(&mut self, node: &$crate::ast::VariableDeclarationStatementStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_variable_declaration_statement(&mut self, node: &$crate::ast::VariableDeclarationStatementStruct<'_>) { self.$leave(node); })?

        fn enter_version_pragma(&mut self, node: &$crate::ast::VersionPragmaStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_version_pragma(&mut self, node: &$crate::ast::VersionPragmaStruct<'_>) { self.$leave(node); })?

        fn enter_version_pragma_comparator(&mut self, node: &$crate::ast::VersionPragmaComparatorStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_version_pragma_comparator(&mut self, node: &$crate::ast::VersionPragmaComparatorStruct<'_>) { self.$leave(node); })?

        fn enter_while_statement(&mut self, node: &$crate::ast::WhileStatementStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_while_statement(&mut self, node: &$crate::ast::WhileStatementStruct<'_>) { self.$leave(node); })?

        fn enter_yul_block(&mut self, node: &$crate::ast::YulBlockStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_yul_block(&mut self, node: &$crate::ast::YulBlockStruct<'_>) { self.$leave(node); })?

        fn enter_yul_break_statement(&mut self, node: &$crate::ast::YulBreakStatementStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_yul_break_statement(&mut self, node: &$crate::ast::YulBreakStatementStruct<'_>) { self.$leave(node); })?

        fn enter_yul_continue_statement(&mut self, node: &$crate::ast::YulContinueStatementStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_yul_continue_statement(&mut self, node: &$crate::ast::YulContinueStatementStruct<'_>) { self.$leave(node); })?

        fn enter_yul_default_case(&mut self, node: &$crate::ast::YulDefaultCaseStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_yul_default_case(&mut self, node: &$crate::ast::YulDefaultCaseStruct<'_>) { self.$leave(node); })?

        fn enter_yul_for_statement(&mut self, node: &$crate::ast::YulForStatementStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_yul_for_statement(&mut self, node: &$crate::ast::YulForStatementStruct<'_>) { self.$leave(node); })?

        fn enter_yul_function_call_expression(&mut self, node: &$crate::ast::YulFunctionCallExpressionStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_yul_function_call_expression(&mut self, node: &$crate::ast::YulFunctionCallExpressionStruct<'_>) { self.$leave(node); })?

        fn enter_yul_function_definition(&mut self, node: &$crate::ast::YulFunctionDefinitionStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_yul_function_definition(&mut self, node: &$crate::ast::YulFunctionDefinitionStruct<'_>) { self.$leave(node); })?

        fn enter_yul_if_statement(&mut self, node: &$crate::ast::YulIfStatementStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_yul_if_statement(&mut self, node: &$crate::ast::YulIfStatementStruct<'_>) { self.$leave(node); })?

        fn enter_yul_leave_statement(&mut self, node: &$crate::ast::YulLeaveStatementStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_yul_leave_statement(&mut self, node: &$crate::ast::YulLeaveStatementStruct<'_>) { self.$leave(node); })?

        fn enter_yul_switch_statement(&mut self, node: &$crate::ast::YulSwitchStatementStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_yul_switch_statement(&mut self, node: &$crate::ast::YulSwitchStatementStruct<'_>) { self.$leave(node); })?

        fn enter_yul_value_case(&mut self, node: &$crate::ast::YulValueCaseStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_yul_value_case(&mut self, node: &$crate::ast::YulValueCaseStruct<'_>) { self.$leave(node); })?

        fn enter_yul_variable_assignment_statement(&mut self, node: &$crate::ast::YulVariableAssignmentStatementStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_yul_variable_assignment_statement(&mut self, node: &$crate::ast::YulVariableAssignmentStatementStruct<'_>) { self.$leave(node); })?

        fn enter_yul_variable_declaration_statement(&mut self, node: &$crate::ast::YulVariableDeclarationStatementStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_yul_variable_declaration_statement(&mut self, node: &$crate::ast::YulVariableDeclarationStatementStruct<'_>) { self.$leave(node); })?

        fn enter_yul_variable_declaration_value(&mut self, node: &$crate::ast::YulVariableDeclarationValueStruct<'_>) -> bool { self.$enter(node) }
        $(fn leave_yul_variable_declaration_value(&mut self, node: &$crate::ast::YulVariableDeclarationValueStruct<'_>) { self.$leave(node); })?

        fn enter_array_values(&mut self, items: &$crate::ast::ArrayValuesStruct<'_>) -> bool { self.$enter(items) }
        $(fn leave_array_values(&mut self, items: &$crate::ast::ArrayValuesStruct<'_>) { self.$leave(items); })?

        fn enter_call_options(&mut self, items: &$crate::ast::CallOptionsStruct<'_>) -> bool { self.$enter(items) }
        $(fn leave_call_options(&mut self, items: &$crate::ast::CallOptionsStruct<'_>) { self.$leave(items); })?

        fn enter_catch_clauses(&mut self, items: &$crate::ast::CatchClausesStruct<'_>) -> bool { self.$enter(items) }
        $(fn leave_catch_clauses(&mut self, items: &$crate::ast::CatchClausesStruct<'_>) { self.$leave(items); })?

        fn enter_contract_members(&mut self, items: &$crate::ast::ContractMembersStruct<'_>) -> bool { self.$enter(items) }
        $(fn leave_contract_members(&mut self, items: &$crate::ast::ContractMembersStruct<'_>) { self.$leave(items); })?

        fn enter_enum_members(&mut self, items: &$crate::ast::EnumMembersStruct<'_>) -> bool { self.$enter(items) }
        $(fn leave_enum_members(&mut self, items: &$crate::ast::EnumMembersStruct<'_>) { self.$leave(items); })?

        fn enter_hex_string_literals(&mut self, items: &$crate::ast::HexStringLiteralsStruct<'_>) -> bool { self.$enter(items) }
        $(fn leave_hex_string_literals(&mut self, items: &$crate::ast::HexStringLiteralsStruct<'_>) { self.$leave(items); })?

        fn enter_identifier_path(&mut self, items: &$crate::ast::IdentifierPathStruct<'_>) -> bool { self.$enter(items) }
        $(fn leave_identifier_path(&mut self, items: &$crate::ast::IdentifierPathStruct<'_>) { self.$leave(items); })?

        fn enter_import_deconstruction_symbols(&mut self, items: &$crate::ast::ImportDeconstructionSymbolsStruct<'_>) -> bool { self.$enter(items) }
        $(fn leave_import_deconstruction_symbols(&mut self, items: &$crate::ast::ImportDeconstructionSymbolsStruct<'_>) { self.$leave(items); })?

        fn enter_inheritance_types(&mut self, items: &$crate::ast::InheritanceTypesStruct<'_>) -> bool { self.$enter(items) }
        $(fn leave_inheritance_types(&mut self, items: &$crate::ast::InheritanceTypesStruct<'_>) { self.$leave(items); })?

        fn enter_interface_members(&mut self, items: &$crate::ast::InterfaceMembersStruct<'_>) -> bool { self.$enter(items) }
        $(fn leave_interface_members(&mut self, items: &$crate::ast::InterfaceMembersStruct<'_>) { self.$leave(items); })?

        fn enter_library_members(&mut self, items: &$crate::ast::LibraryMembersStruct<'_>) -> bool { self.$enter(items) }
        $(fn leave_library_members(&mut self, items: &$crate::ast::LibraryMembersStruct<'_>) { self.$leave(items); })?

        fn enter_modifier_invocations(&mut self, items: &$crate::ast::ModifierInvocationsStruct<'_>) -> bool { self.$enter(items) }
        $(fn leave_modifier_invocations(&mut self, items: &$crate::ast::ModifierInvocationsStruct<'_>) { self.$leave(items); })?

        fn enter_multi_typed_declaration_elements(&mut self, items: &$crate::ast::MultiTypedDeclarationElementsStruct<'_>) -> bool { self.$enter(items) }
        $(fn leave_multi_typed_declaration_elements(&mut self, items: &$crate::ast::MultiTypedDeclarationElementsStruct<'_>) { self.$leave(items); })?

        fn enter_named_arguments(&mut self, items: &$crate::ast::NamedArgumentsStruct<'_>) -> bool { self.$enter(items) }
        $(fn leave_named_arguments(&mut self, items: &$crate::ast::NamedArgumentsStruct<'_>) { self.$leave(items); })?

        fn enter_override_paths(&mut self, items: &$crate::ast::OverridePathsStruct<'_>) -> bool { self.$enter(items) }
        $(fn leave_override_paths(&mut self, items: &$crate::ast::OverridePathsStruct<'_>) { self.$leave(items); })?

        fn enter_parameters(&mut self, items: &$crate::ast::ParametersStruct<'_>) -> bool { self.$enter(items) }
        $(fn leave_parameters(&mut self, items: &$crate::ast::ParametersStruct<'_>) { self.$leave(items); })?

        fn enter_positional_arguments(&mut self, items: &$crate::ast::PositionalArgumentsStruct<'_>) -> bool { self.$enter(items) }
        $(fn leave_positional_arguments(&mut self, items: &$crate::ast::PositionalArgumentsStruct<'_>) { self.$leave(items); })?

        fn enter_source_unit_members(&mut self, items: &$crate::ast::SourceUnitMembersStruct<'_>) -> bool { self.$enter(items) }
        $(fn leave_source_unit_members(&mut self, items: &$crate::ast::SourceUnitMembersStruct<'_>) { self.$leave(items); })?

        fn enter_statements(&mut self, items: &$crate::ast::StatementsStruct<'_>) -> bool { self.$enter(items) }
        $(fn leave_statements(&mut self, items: &$crate::ast::StatementsStruct<'_>) { self.$leave(items); })?

        fn enter_string_literals(&mut self, items: &$crate::ast::StringLiteralsStruct<'_>) -> bool { self.$enter(items) }
        $(fn leave_string_literals(&mut self, items: &$crate::ast::StringLiteralsStruct<'_>) { self.$leave(items); })?

        fn enter_struct_members(&mut self, items: &$crate::ast::StructMembersStruct<'_>) -> bool { self.$enter(items) }
        $(fn leave_struct_members(&mut self, items: &$crate::ast::StructMembersStruct<'_>) { self.$leave(items); })?

        fn enter_tuple_values(&mut self, items: &$crate::ast::TupleValuesStruct<'_>) -> bool { self.$enter(items) }
        $(fn leave_tuple_values(&mut self, items: &$crate::ast::TupleValuesStruct<'_>) { self.$leave(items); })?

        fn enter_unicode_string_literals(&mut self, items: &$crate::ast::UnicodeStringLiteralsStruct<'_>) -> bool { self.$enter(items) }
        $(fn leave_unicode_string_literals(&mut self, items: &$crate::ast::UnicodeStringLiteralsStruct<'_>) { self.$leave(items); })?

        fn enter_using_deconstruction_symbols(&mut self, items: &$crate::ast::UsingDeconstructionSymbolsStruct<'_>) -> bool { self.$enter(items) }
        $(fn leave_using_deconstruction_symbols(&mut self, items: &$crate::ast::UsingDeconstructionSymbolsStruct<'_>) { self.$leave(items); })?

        fn enter_version_pragma_expression_set(&mut self, items: &$crate::ast::VersionPragmaExpressionSetStruct<'_>) -> bool { self.$enter(items) }
        $(fn leave_version_pragma_expression_set(&mut self, items: &$crate::ast::VersionPragmaExpressionSetStruct<'_>) { self.$leave(items); })?

        fn enter_version_pragma_expression_sets(&mut self, items: &$crate::ast::VersionPragmaExpressionSetsStruct<'_>) -> bool { self.$enter(items) }
        $(fn leave_version_pragma_expression_sets(&mut self, items: &$crate::ast::VersionPragmaExpressionSetsStruct<'_>) { self.$leave(items); })?

        fn enter_version_pragma_specifier(&mut self, items: &$crate::ast::VersionPragmaSpecifierStruct<'_>) -> bool { self.$enter(items) }
        $(fn leave_version_pragma_specifier(&mut self, items: &$crate::ast::VersionPragmaSpecifierStruct<'_>) { self.$leave(items); })?

        fn enter_yul_arguments(&mut self, items: &$crate::ast::YulArgumentsStruct<'_>) -> bool { self.$enter(items) }
        $(fn leave_yul_arguments(&mut self, items: &$crate::ast::YulArgumentsStruct<'_>) { self.$leave(items); })?

        fn enter_yul_parameters(&mut self, items: &$crate::ast::YulParametersStruct<'_>) -> bool { self.$enter(items) }
        $(fn leave_yul_parameters(&mut self, items: &$crate::ast::YulParametersStruct<'_>) { self.$leave(items); })?

        fn enter_yul_path(&mut self, items: &$crate::ast::YulPathStruct<'_>) -> bool { self.$enter(items) }
        $(fn leave_yul_path(&mut self, items: &$crate::ast::YulPathStruct<'_>) { self.$leave(items); })?

        fn enter_yul_paths(&mut self, items: &$crate::ast::YulPathsStruct<'_>) -> bool { self.$enter(items) }
        $(fn leave_yul_paths(&mut self, items: &$crate::ast::YulPathsStruct<'_>) { self.$leave(items); })?

        fn enter_yul_statements(&mut self, items: &$crate::ast::YulStatementsStruct<'_>) -> bool { self.$enter(items) }
        $(fn leave_yul_statements(&mut self, items: &$crate::ast::YulStatementsStruct<'_>) { self.$leave(items); })?

        fn enter_yul_value_cases(&mut self, items: &$crate::ast::YulValueCasesStruct<'_>) -> bool { self.$enter(items) }
        $(fn leave_yul_value_cases(&mut self, items: &$crate::ast::YulValueCasesStruct<'_>) { self.$leave(items); })?

        fn enter_yul_variable_names(&mut self, items: &$crate::ast::YulVariableNamesStruct<'_>) -> bool { self.$enter(items) }
        $(fn leave_yul_variable_names(&mut self, items: &$crate::ast::YulVariableNamesStruct<'_>) { self.$leave(items); })?
      };
}

pub use crate::impl_nonterminal_visitor_hooks;

//
// Sequences
//

pub fn accept_abicoder_pragma(node: &AbicoderPragma<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_abicoder_pragma(node) {
        return;
    }
    accept_abicoder_version(&node.version(), visitor);
    visitor.leave_abicoder_pragma(node);
}

pub fn accept_additive_expression(node: &AdditiveExpression<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_additive_expression(node) {
        return;
    }
    accept_expression(&node.left_operand(), visitor);
    accept_additive_expression_operator(&node.operator(), visitor);
    accept_expression(&node.right_operand(), visitor);
    visitor.leave_additive_expression(node);
}

pub fn accept_address_type(node: &AddressType<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_address_type(node) {
        return;
    }
    visitor.leave_address_type(node);
}

pub fn accept_and_expression(node: &AndExpression<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_and_expression(node) {
        return;
    }
    accept_expression(&node.left_operand(), visitor);
    accept_expression(&node.right_operand(), visitor);
    visitor.leave_and_expression(node);
}

pub fn accept_array_expression(node: &ArrayExpression<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_array_expression(node) {
        return;
    }
    accept_array_values(&node.items(), visitor);
    visitor.leave_array_expression(node);
}

pub fn accept_array_type_name(node: &ArrayTypeName<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_array_type_name(node) {
        return;
    }
    accept_type_name(&node.operand(), visitor);
    if let Some(ref index) = node.index() {
        accept_expression(index, visitor);
    }
    visitor.leave_array_type_name(node);
}

pub fn accept_assembly_statement(node: &AssemblyStatement<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_assembly_statement(node) {
        return;
    }
    accept_yul_block(&node.body(), visitor);
    visitor.leave_assembly_statement(node);
}

pub fn accept_assignment_expression(node: &AssignmentExpression<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_assignment_expression(node) {
        return;
    }
    accept_expression(&node.left_operand(), visitor);
    accept_assignment_expression_operator(&node.operator(), visitor);
    accept_expression(&node.right_operand(), visitor);
    visitor.leave_assignment_expression(node);
}

pub fn accept_bitwise_and_expression(node: &BitwiseAndExpression<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_bitwise_and_expression(node) {
        return;
    }
    accept_expression(&node.left_operand(), visitor);
    accept_expression(&node.right_operand(), visitor);
    visitor.leave_bitwise_and_expression(node);
}

pub fn accept_bitwise_or_expression(node: &BitwiseOrExpression<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_bitwise_or_expression(node) {
        return;
    }
    accept_expression(&node.left_operand(), visitor);
    accept_expression(&node.right_operand(), visitor);
    visitor.leave_bitwise_or_expression(node);
}

pub fn accept_bitwise_xor_expression(node: &BitwiseXorExpression<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_bitwise_xor_expression(node) {
        return;
    }
    accept_expression(&node.left_operand(), visitor);
    accept_expression(&node.right_operand(), visitor);
    visitor.leave_bitwise_xor_expression(node);
}

pub fn accept_block(node: &Block<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_block(node) {
        return;
    }
    accept_statements(&node.statements(), visitor);
    visitor.leave_block(node);
}

pub fn accept_break_statement(node: &BreakStatement<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_break_statement(node) {
        return;
    }
    visitor.leave_break_statement(node);
}

pub fn accept_call_options_expression(
    node: &CallOptionsExpression<'_>,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_call_options_expression(node) {
        return;
    }
    accept_expression(&node.operand(), visitor);
    accept_call_options(&node.options(), visitor);
    visitor.leave_call_options_expression(node);
}

pub fn accept_catch_clause(node: &CatchClause<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_catch_clause(node) {
        return;
    }
    accept_catch_clause_kind(&node.kind(), visitor);
    if let Some(ref parameters) = node.parameters() {
        accept_parameters(parameters, visitor);
    }
    accept_block(&node.body(), visitor);
    visitor.leave_catch_clause(node);
}

pub fn accept_conditional_expression(node: &ConditionalExpression<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_conditional_expression(node) {
        return;
    }
    accept_expression(&node.operand(), visitor);
    accept_expression(&node.true_expression(), visitor);
    accept_expression(&node.false_expression(), visitor);
    visitor.leave_conditional_expression(node);
}

pub fn accept_constant_definition(node: &ConstantDefinition<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_constant_definition(node) {
        return;
    }
    accept_type_name(&node.type_name(), visitor);
    visitor.visit_identifier(&node.name());
    if let Some(ref visibility) = node.visibility() {
        accept_state_variable_visibility(visibility, visitor);
    }
    if let Some(ref value) = node.value() {
        accept_expression(value, visitor);
    }
    visitor.leave_constant_definition(node);
}

pub fn accept_continue_statement(node: &ContinueStatement<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_continue_statement(node) {
        return;
    }
    visitor.leave_continue_statement(node);
}

pub fn accept_contract_definition(node: &ContractDefinition<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_contract_definition(node) {
        return;
    }
    visitor.visit_identifier(&node.name());
    accept_inheritance_types(&node.inheritance_types(), visitor);
    if let Some(ref storage_layout) = node.storage_layout() {
        accept_expression(storage_layout, visitor);
    }
    accept_contract_members(&node.members(), visitor);
    visitor.leave_contract_definition(node);
}

pub fn accept_decimal_number_expression(
    node: &DecimalNumberExpression<'_>,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_decimal_number_expression(node) {
        return;
    }
    visitor.visit_decimal_literal(&node.literal());
    if let Some(ref unit) = node.unit() {
        accept_number_unit(unit, visitor);
    }
    visitor.leave_decimal_number_expression(node);
}

pub fn accept_do_while_statement(node: &DoWhileStatement<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_do_while_statement(node) {
        return;
    }
    accept_statement(&node.body(), visitor);
    accept_expression(&node.condition(), visitor);
    visitor.leave_do_while_statement(node);
}

pub fn accept_emit_statement(node: &EmitStatement<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_emit_statement(node) {
        return;
    }
    accept_identifier_path(&node.event(), visitor);
    accept_arguments_declaration(&node.arguments(), visitor);
    visitor.leave_emit_statement(node);
}

pub fn accept_enum_definition(node: &EnumDefinition<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_enum_definition(node) {
        return;
    }
    visitor.visit_identifier(&node.name());
    accept_enum_members(&node.members(), visitor);
    visitor.leave_enum_definition(node);
}

pub fn accept_equality_expression(node: &EqualityExpression<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_equality_expression(node) {
        return;
    }
    accept_expression(&node.left_operand(), visitor);
    accept_equality_expression_operator(&node.operator(), visitor);
    accept_expression(&node.right_operand(), visitor);
    visitor.leave_equality_expression(node);
}

pub fn accept_error_definition(node: &ErrorDefinition<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_error_definition(node) {
        return;
    }
    visitor.visit_identifier(&node.name());
    accept_parameters(&node.parameters(), visitor);
    visitor.leave_error_definition(node);
}

pub fn accept_event_definition(node: &EventDefinition<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_event_definition(node) {
        return;
    }
    visitor.visit_identifier(&node.name());
    accept_parameters(&node.parameters(), visitor);
    visitor.leave_event_definition(node);
}

pub fn accept_experimental_pragma(node: &ExperimentalPragma<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_experimental_pragma(node) {
        return;
    }
    accept_experimental_feature(&node.feature(), visitor);
    visitor.leave_experimental_pragma(node);
}

pub fn accept_exponentiation_expression(
    node: &ExponentiationExpression<'_>,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_exponentiation_expression(node) {
        return;
    }
    accept_expression(&node.left_operand(), visitor);
    accept_expression(&node.right_operand(), visitor);
    visitor.leave_exponentiation_expression(node);
}

pub fn accept_expression_statement(node: &ExpressionStatement<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_expression_statement(node) {
        return;
    }
    accept_expression(&node.expression(), visitor);
    visitor.leave_expression_statement(node);
}

pub fn accept_for_statement(node: &ForStatement<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_for_statement(node) {
        return;
    }
    accept_for_statement_initialization(&node.initialization(), visitor);
    accept_for_statement_condition(&node.condition(), visitor);
    if let Some(ref iterator) = node.iterator() {
        accept_expression(iterator, visitor);
    }
    accept_statement(&node.body(), visitor);
    visitor.leave_for_statement(node);
}

pub fn accept_function_attributes(node: &FunctionAttributes<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_function_attributes(node) {
        return;
    }
    accept_function_visibility(&node.visibility(), visitor);
    accept_function_mutability(&node.mutability(), visitor);
    if let Some(ref override_specifier) = node.override_specifier() {
        accept_override_paths(override_specifier, visitor);
    }
    accept_modifier_invocations(&node.modifier_invocations(), visitor);
    visitor.leave_function_attributes(node);
}

pub fn accept_function_call_expression(
    node: &FunctionCallExpression<'_>,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_function_call_expression(node) {
        return;
    }
    accept_expression(&node.operand(), visitor);
    accept_arguments_declaration(&node.arguments(), visitor);
    visitor.leave_function_call_expression(node);
}

pub fn accept_function_definition(node: &FunctionDefinition<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_function_definition(node) {
        return;
    }
    accept_function_kind(&node.kind(), visitor);
    if let Some(ref name) = node.name() {
        visitor.visit_identifier(name);
    }
    accept_parameters(&node.parameters(), visitor);
    accept_function_attributes(&node.attributes(), visitor);
    if let Some(ref returns) = node.returns() {
        accept_parameters(returns, visitor);
    }
    if let Some(ref body) = node.body() {
        accept_block(body, visitor);
    }
    visitor.leave_function_definition(node);
}

pub fn accept_function_type(node: &FunctionType<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_function_type(node) {
        return;
    }
    accept_parameters(&node.parameters(), visitor);
    accept_function_type_attributes(&node.attributes(), visitor);
    if let Some(ref returns) = node.returns() {
        accept_parameters(returns, visitor);
    }
    visitor.leave_function_type(node);
}

pub fn accept_function_type_attributes(
    node: &FunctionTypeAttributes<'_>,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_function_type_attributes(node) {
        return;
    }
    accept_function_visibility(&node.visibility(), visitor);
    accept_function_mutability(&node.mutability(), visitor);
    visitor.leave_function_type_attributes(node);
}

pub fn accept_hex_number_expression(node: &HexNumberExpression<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_hex_number_expression(node) {
        return;
    }
    visitor.visit_hex_literal(&node.literal());
    visitor.leave_hex_number_expression(node);
}

pub fn accept_if_statement(node: &IfStatement<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_if_statement(node) {
        return;
    }
    accept_expression(&node.condition(), visitor);
    accept_statement(&node.body(), visitor);
    if let Some(ref else_branch) = node.else_branch() {
        accept_statement(else_branch, visitor);
    }
    visitor.leave_if_statement(node);
}

pub fn accept_import_deconstruction(node: &ImportDeconstruction<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_import_deconstruction(node) {
        return;
    }
    accept_import_deconstruction_symbols(&node.symbols(), visitor);
    visitor.visit_string_literal(&node.path());
    visitor.leave_import_deconstruction(node);
}

pub fn accept_import_deconstruction_symbol(
    node: &ImportDeconstructionSymbol<'_>,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_import_deconstruction_symbol(node) {
        return;
    }
    visitor.visit_identifier(&node.name());
    if let Some(ref alias) = node.alias() {
        visitor.visit_identifier(alias);
    }
    visitor.leave_import_deconstruction_symbol(node);
}

pub fn accept_index_access_expression(
    node: &IndexAccessExpression<'_>,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_index_access_expression(node) {
        return;
    }
    accept_expression(&node.operand(), visitor);
    if let Some(ref start) = node.start() {
        accept_expression(start, visitor);
    }
    if let Some(ref end) = node.end() {
        accept_expression(end, visitor);
    }
    visitor.leave_index_access_expression(node);
}

pub fn accept_inequality_expression(node: &InequalityExpression<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_inequality_expression(node) {
        return;
    }
    accept_expression(&node.left_operand(), visitor);
    accept_inequality_expression_operator(&node.operator(), visitor);
    accept_expression(&node.right_operand(), visitor);
    visitor.leave_inequality_expression(node);
}

pub fn accept_inheritance_type(node: &InheritanceType<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_inheritance_type(node) {
        return;
    }
    accept_identifier_path(&node.type_name(), visitor);
    if let Some(ref arguments) = node.arguments() {
        accept_positional_arguments(arguments, visitor);
    }
    visitor.leave_inheritance_type(node);
}

pub fn accept_interface_definition(node: &InterfaceDefinition<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_interface_definition(node) {
        return;
    }
    visitor.visit_identifier(&node.name());
    if let Some(ref inheritance) = node.inheritance() {
        accept_inheritance_types(inheritance, visitor);
    }
    accept_interface_members(&node.members(), visitor);
    visitor.leave_interface_definition(node);
}

pub fn accept_library_definition(node: &LibraryDefinition<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_library_definition(node) {
        return;
    }
    visitor.visit_identifier(&node.name());
    accept_library_members(&node.members(), visitor);
    visitor.leave_library_definition(node);
}

pub fn accept_mapping_type(node: &MappingType<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_mapping_type(node) {
        return;
    }
    accept_parameter(&node.key_type(), visitor);
    accept_parameter(&node.value_type(), visitor);
    visitor.leave_mapping_type(node);
}

pub fn accept_member_access_expression(
    node: &MemberAccessExpression<'_>,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_member_access_expression(node) {
        return;
    }
    accept_expression(&node.operand(), visitor);
    visitor.visit_identifier(&node.member());
    visitor.leave_member_access_expression(node);
}

pub fn accept_modifier_attributes(node: &ModifierAttributes<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_modifier_attributes(node) {
        return;
    }
    if let Some(ref override_specifier) = node.override_specifier() {
        accept_override_paths(override_specifier, visitor);
    }
    visitor.leave_modifier_attributes(node);
}

pub fn accept_modifier_definition(node: &ModifierDefinition<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_modifier_definition(node) {
        return;
    }
    visitor.visit_identifier(&node.name());
    accept_parameters(&node.parameters(), visitor);
    accept_modifier_attributes(&node.attributes(), visitor);
    if let Some(ref body) = node.body() {
        accept_block(body, visitor);
    }
    visitor.leave_modifier_definition(node);
}

pub fn accept_modifier_invocation(node: &ModifierInvocation<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_modifier_invocation(node) {
        return;
    }
    accept_identifier_path(&node.name(), visitor);
    if let Some(ref arguments) = node.arguments() {
        accept_positional_arguments(arguments, visitor);
    }
    visitor.leave_modifier_invocation(node);
}

pub fn accept_multi_typed_declaration(
    node: &MultiTypedDeclaration<'_>,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_multi_typed_declaration(node) {
        return;
    }
    accept_multi_typed_declaration_elements(&node.elements(), visitor);
    accept_expression(&node.value(), visitor);
    visitor.leave_multi_typed_declaration(node);
}

pub fn accept_multi_typed_declaration_element(
    node: &MultiTypedDeclarationElement<'_>,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_multi_typed_declaration_element(node) {
        return;
    }
    if let Some(ref member) = node.member() {
        accept_variable_declaration(member, visitor);
    }
    visitor.leave_multi_typed_declaration_element(node);
}

pub fn accept_multiplicative_expression(
    node: &MultiplicativeExpression<'_>,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_multiplicative_expression(node) {
        return;
    }
    accept_expression(&node.left_operand(), visitor);
    accept_multiplicative_expression_operator(&node.operator(), visitor);
    accept_expression(&node.right_operand(), visitor);
    visitor.leave_multiplicative_expression(node);
}

pub fn accept_named_argument(node: &NamedArgument<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_named_argument(node) {
        return;
    }
    visitor.visit_identifier(&node.name());
    accept_expression(&node.value(), visitor);
    visitor.leave_named_argument(node);
}

pub fn accept_new_expression(node: &NewExpression<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_new_expression(node) {
        return;
    }
    accept_type_name(&node.type_name(), visitor);
    visitor.leave_new_expression(node);
}

pub fn accept_or_expression(node: &OrExpression<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_or_expression(node) {
        return;
    }
    accept_expression(&node.left_operand(), visitor);
    accept_expression(&node.right_operand(), visitor);
    visitor.leave_or_expression(node);
}

pub fn accept_parameter(node: &Parameter<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_parameter(node) {
        return;
    }
    accept_type_name(&node.type_name(), visitor);
    if let Some(ref storage_location) = node.storage_location() {
        accept_storage_location(storage_location, visitor);
    }
    if let Some(ref name) = node.name() {
        visitor.visit_identifier(name);
    }
    visitor.leave_parameter(node);
}

pub fn accept_path_import(node: &PathImport<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_path_import(node) {
        return;
    }
    visitor.visit_string_literal(&node.path());
    if let Some(ref alias) = node.alias() {
        visitor.visit_identifier(alias);
    }
    visitor.leave_path_import(node);
}

pub fn accept_postfix_expression(node: &PostfixExpression<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_postfix_expression(node) {
        return;
    }
    accept_expression(&node.operand(), visitor);
    accept_postfix_expression_operator(&node.operator(), visitor);
    visitor.leave_postfix_expression(node);
}

pub fn accept_pragma_directive(node: &PragmaDirective<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_pragma_directive(node) {
        return;
    }
    accept_pragma(&node.pragma(), visitor);
    visitor.leave_pragma_directive(node);
}

pub fn accept_prefix_expression(node: &PrefixExpression<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_prefix_expression(node) {
        return;
    }
    accept_prefix_expression_operator(&node.operator(), visitor);
    accept_expression(&node.operand(), visitor);
    visitor.leave_prefix_expression(node);
}

pub fn accept_return_statement(node: &ReturnStatement<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_return_statement(node) {
        return;
    }
    if let Some(ref expression) = node.expression() {
        accept_expression(expression, visitor);
    }
    visitor.leave_return_statement(node);
}

pub fn accept_revert_statement(node: &RevertStatement<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_revert_statement(node) {
        return;
    }
    accept_identifier_path(&node.error(), visitor);
    accept_arguments_declaration(&node.arguments(), visitor);
    visitor.leave_revert_statement(node);
}

pub fn accept_shift_expression(node: &ShiftExpression<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_shift_expression(node) {
        return;
    }
    accept_expression(&node.left_operand(), visitor);
    accept_shift_expression_operator(&node.operator(), visitor);
    accept_expression(&node.right_operand(), visitor);
    visitor.leave_shift_expression(node);
}

pub fn accept_single_typed_declaration(
    node: &SingleTypedDeclaration<'_>,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_single_typed_declaration(node) {
        return;
    }
    accept_variable_declaration(&node.declaration(), visitor);
    if let Some(ref value) = node.value() {
        accept_expression(value, visitor);
    }
    visitor.leave_single_typed_declaration(node);
}

pub fn accept_source_unit(node: &SourceUnit<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_source_unit(node) {
        return;
    }
    accept_source_unit_members(&node.members(), visitor);
    visitor.leave_source_unit(node);
}

pub fn accept_state_variable_attributes(
    node: &StateVariableAttributes<'_>,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_state_variable_attributes(node) {
        return;
    }
    accept_state_variable_visibility(&node.visibility(), visitor);
    accept_state_variable_mutability(&node.mutability(), visitor);
    if let Some(ref override_specifier) = node.override_specifier() {
        accept_override_paths(override_specifier, visitor);
    }
    visitor.leave_state_variable_attributes(node);
}

pub fn accept_state_variable_definition(
    node: &StateVariableDefinition<'_>,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_state_variable_definition(node) {
        return;
    }
    accept_type_name(&node.type_name(), visitor);
    visitor.visit_identifier(&node.name());
    if let Some(ref value) = node.value() {
        accept_expression(value, visitor);
    }
    accept_state_variable_attributes(&node.attributes(), visitor);
    visitor.leave_state_variable_definition(node);
}

pub fn accept_struct_definition(node: &StructDefinition<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_struct_definition(node) {
        return;
    }
    visitor.visit_identifier(&node.name());
    accept_struct_members(&node.members(), visitor);
    visitor.leave_struct_definition(node);
}

pub fn accept_struct_member(node: &StructMember<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_struct_member(node) {
        return;
    }
    accept_type_name(&node.type_name(), visitor);
    visitor.visit_identifier(&node.name());
    visitor.leave_struct_member(node);
}

pub fn accept_try_statement(node: &TryStatement<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_try_statement(node) {
        return;
    }
    accept_expression(&node.expression(), visitor);
    if let Some(ref returns) = node.returns() {
        accept_parameters(returns, visitor);
    }
    accept_block(&node.body(), visitor);
    accept_catch_clauses(&node.catch_clauses(), visitor);
    visitor.leave_try_statement(node);
}

pub fn accept_tuple_expression(node: &TupleExpression<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_tuple_expression(node) {
        return;
    }
    accept_tuple_values(&node.items(), visitor);
    visitor.leave_tuple_expression(node);
}

pub fn accept_tuple_value(node: &TupleValue<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_tuple_value(node) {
        return;
    }
    if let Some(ref expression) = node.expression() {
        accept_expression(expression, visitor);
    }
    visitor.leave_tuple_value(node);
}

pub fn accept_type_expression(node: &TypeExpression<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_type_expression(node) {
        return;
    }
    accept_type_name(&node.type_name(), visitor);
    visitor.leave_type_expression(node);
}

pub fn accept_unchecked_block(node: &UncheckedBlock<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_unchecked_block(node) {
        return;
    }
    accept_block(&node.block(), visitor);
    visitor.leave_unchecked_block(node);
}

pub fn accept_user_defined_value_type_definition(
    node: &UserDefinedValueTypeDefinition<'_>,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_user_defined_value_type_definition(node) {
        return;
    }
    visitor.visit_identifier(&node.name());
    accept_elementary_type(&node.value_type(), visitor);
    visitor.leave_user_defined_value_type_definition(node);
}

pub fn accept_using_deconstruction(node: &UsingDeconstruction<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_using_deconstruction(node) {
        return;
    }
    accept_using_deconstruction_symbols(&node.symbols(), visitor);
    visitor.leave_using_deconstruction(node);
}

pub fn accept_using_deconstruction_symbol(
    node: &UsingDeconstructionSymbol<'_>,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_using_deconstruction_symbol(node) {
        return;
    }
    accept_identifier_path(&node.name(), visitor);
    if let Some(ref alias) = node.alias() {
        accept_using_operator(alias, visitor);
    }
    visitor.leave_using_deconstruction_symbol(node);
}

pub fn accept_using_directive(node: &UsingDirective<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_using_directive(node) {
        return;
    }
    accept_using_clause(&node.clause(), visitor);
    accept_using_target(&node.target(), visitor);
    visitor.leave_using_directive(node);
}

pub fn accept_variable_declaration(node: &VariableDeclaration<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_variable_declaration(node) {
        return;
    }
    accept_type_name(&node.type_name(), visitor);
    if let Some(ref storage_location) = node.storage_location() {
        accept_storage_location(storage_location, visitor);
    }
    visitor.visit_identifier(&node.name());
    visitor.leave_variable_declaration(node);
}

pub fn accept_variable_declaration_statement(
    node: &VariableDeclarationStatement<'_>,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_variable_declaration_statement(node) {
        return;
    }
    accept_variable_declaration_target(&node.target(), visitor);
    visitor.leave_variable_declaration_statement(node);
}

pub fn accept_version_pragma(node: &VersionPragma<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_version_pragma(node) {
        return;
    }
    accept_version_pragma_expression_sets(&node.sets(), visitor);
    visitor.leave_version_pragma(node);
}

pub fn accept_version_pragma_comparator(
    node: &VersionPragmaComparator<'_>,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_version_pragma_comparator(node) {
        return;
    }
    accept_version_pragma_operator(&node.operator(), visitor);
    accept_version_pragma_specifier(&node.specifier(), visitor);
    visitor.leave_version_pragma_comparator(node);
}

pub fn accept_while_statement(node: &WhileStatement<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_while_statement(node) {
        return;
    }
    accept_expression(&node.condition(), visitor);
    accept_statement(&node.body(), visitor);
    visitor.leave_while_statement(node);
}

pub fn accept_yul_block(node: &YulBlock<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_block(node) {
        return;
    }
    accept_yul_statements(&node.statements(), visitor);
    visitor.leave_yul_block(node);
}

pub fn accept_yul_break_statement(node: &YulBreakStatement<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_break_statement(node) {
        return;
    }
    visitor.leave_yul_break_statement(node);
}

pub fn accept_yul_continue_statement(node: &YulContinueStatement<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_continue_statement(node) {
        return;
    }
    visitor.leave_yul_continue_statement(node);
}

pub fn accept_yul_default_case(node: &YulDefaultCase<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_default_case(node) {
        return;
    }
    accept_yul_block(&node.body(), visitor);
    visitor.leave_yul_default_case(node);
}

pub fn accept_yul_for_statement(node: &YulForStatement<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_for_statement(node) {
        return;
    }
    accept_yul_block(&node.initialization(), visitor);
    accept_yul_expression(&node.condition(), visitor);
    accept_yul_block(&node.iterator(), visitor);
    accept_yul_block(&node.body(), visitor);
    visitor.leave_yul_for_statement(node);
}

pub fn accept_yul_function_call_expression(
    node: &YulFunctionCallExpression<'_>,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_yul_function_call_expression(node) {
        return;
    }
    accept_yul_expression(&node.operand(), visitor);
    accept_yul_arguments(&node.arguments(), visitor);
    visitor.leave_yul_function_call_expression(node);
}

pub fn accept_yul_function_definition(
    node: &YulFunctionDefinition<'_>,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_yul_function_definition(node) {
        return;
    }
    visitor.visit_identifier(&node.name());
    accept_yul_parameters(&node.parameters(), visitor);
    if let Some(ref returns) = node.returns() {
        accept_yul_variable_names(returns, visitor);
    }
    accept_yul_block(&node.body(), visitor);
    visitor.leave_yul_function_definition(node);
}

pub fn accept_yul_if_statement(node: &YulIfStatement<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_if_statement(node) {
        return;
    }
    accept_yul_expression(&node.condition(), visitor);
    accept_yul_block(&node.body(), visitor);
    visitor.leave_yul_if_statement(node);
}

pub fn accept_yul_leave_statement(node: &YulLeaveStatement<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_leave_statement(node) {
        return;
    }
    visitor.leave_yul_leave_statement(node);
}

pub fn accept_yul_switch_statement(node: &YulSwitchStatement<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_switch_statement(node) {
        return;
    }
    accept_yul_expression(&node.expression(), visitor);
    accept_yul_value_cases(&node.value_cases(), visitor);
    if let Some(ref default_case) = node.default_case() {
        accept_yul_default_case(default_case, visitor);
    }
    visitor.leave_yul_switch_statement(node);
}

pub fn accept_yul_value_case(node: &YulValueCase<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_value_case(node) {
        return;
    }
    accept_yul_literal(&node.value(), visitor);
    accept_yul_block(&node.body(), visitor);
    visitor.leave_yul_value_case(node);
}

pub fn accept_yul_variable_assignment_statement(
    node: &YulVariableAssignmentStatement<'_>,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_yul_variable_assignment_statement(node) {
        return;
    }
    accept_yul_paths(&node.variables(), visitor);
    accept_yul_expression(&node.expression(), visitor);
    visitor.leave_yul_variable_assignment_statement(node);
}

pub fn accept_yul_variable_declaration_statement(
    node: &YulVariableDeclarationStatement<'_>,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_yul_variable_declaration_statement(node) {
        return;
    }
    accept_yul_variable_names(&node.variables(), visitor);
    if let Some(ref value) = node.value() {
        accept_yul_variable_declaration_value(value, visitor);
    }
    visitor.leave_yul_variable_declaration_statement(node);
}

pub fn accept_yul_variable_declaration_value(
    node: &YulVariableDeclarationValue<'_>,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_yul_variable_declaration_value(node) {
        return;
    }
    accept_yul_expression(&node.expression(), visitor);
    visitor.leave_yul_variable_declaration_value(node);
}

//
// Choices
//

pub fn accept_abicoder_version(node: &AbicoderVersion, visitor: &mut impl Visitor) {
    if !visitor.enter_abicoder_version(node) {
        return;
    }
    match node {
        AbicoderVersion::V1 => {}
        AbicoderVersion::V2 => {}
    }
    visitor.leave_abicoder_version(node);
}

pub fn accept_additive_expression_operator(
    node: &AdditiveExpressionOperator<'_>,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_additive_expression_operator(node) {
        return;
    }
    match node {
        AdditiveExpressionOperator::Minus(_) => {}
        AdditiveExpressionOperator::Plus(_) => {}
    }
    visitor.leave_additive_expression_operator(node);
}

pub fn accept_arguments_declaration(node: &ArgumentsDeclaration<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_arguments_declaration(node) {
        return;
    }
    match node {
        ArgumentsDeclaration::PositionalArguments(positional_arguments) => {
            accept_positional_arguments(positional_arguments, visitor);
        }
        ArgumentsDeclaration::NamedArguments(named_arguments) => {
            accept_named_arguments(named_arguments, visitor);
        }
    }
    visitor.leave_arguments_declaration(node);
}

pub fn accept_assignment_expression_operator(
    node: &AssignmentExpressionOperator<'_>,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_assignment_expression_operator(node) {
        return;
    }
    match node {
        AssignmentExpressionOperator::AmpersandEqual(_) => {}
        AssignmentExpressionOperator::AsteriskEqual(_) => {}
        AssignmentExpressionOperator::BarEqual(_) => {}
        AssignmentExpressionOperator::CaretEqual(_) => {}
        AssignmentExpressionOperator::Equal(_) => {}
        AssignmentExpressionOperator::GreaterThanGreaterThanEqual(_) => {}
        AssignmentExpressionOperator::GreaterThanGreaterThanGreaterThanEqual(_) => {}
        AssignmentExpressionOperator::LessThanLessThanEqual(_) => {}
        AssignmentExpressionOperator::MinusEqual(_) => {}
        AssignmentExpressionOperator::PercentEqual(_) => {}
        AssignmentExpressionOperator::PlusEqual(_) => {}
        AssignmentExpressionOperator::SlashEqual(_) => {}
    }
    visitor.leave_assignment_expression_operator(node);
}

pub fn accept_catch_clause_kind(node: &CatchClauseKind, visitor: &mut impl Visitor) {
    if !visitor.enter_catch_clause_kind(node) {
        return;
    }
    match node {
        CatchClauseKind::Error => {}
        CatchClauseKind::Panic => {}
        CatchClauseKind::LowLevel => {}
    }
    visitor.leave_catch_clause_kind(node);
}

pub fn accept_contract_member(node: &ContractMember<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_contract_member(node) {
        return;
    }
    match node {
        ContractMember::UsingDirective(using_directive) => {
            accept_using_directive(using_directive, visitor);
        }
        ContractMember::FunctionDefinition(function_definition) => {
            accept_function_definition(function_definition, visitor);
        }
        ContractMember::ModifierDefinition(modifier_definition) => {
            accept_modifier_definition(modifier_definition, visitor);
        }
        ContractMember::StructDefinition(struct_definition) => {
            accept_struct_definition(struct_definition, visitor);
        }
        ContractMember::EnumDefinition(enum_definition) => {
            accept_enum_definition(enum_definition, visitor);
        }
        ContractMember::EventDefinition(event_definition) => {
            accept_event_definition(event_definition, visitor);
        }
        ContractMember::ErrorDefinition(error_definition) => {
            accept_error_definition(error_definition, visitor);
        }
        ContractMember::UserDefinedValueTypeDefinition(user_defined_value_type_definition) => {
            accept_user_defined_value_type_definition(user_defined_value_type_definition, visitor);
        }
        ContractMember::StateVariableDefinition(state_variable_definition) => {
            accept_state_variable_definition(state_variable_definition, visitor);
        }
        ContractMember::ConstantDefinition(constant_definition) => {
            accept_constant_definition(constant_definition, visitor);
        }
    }
    visitor.leave_contract_member(node);
}

pub fn accept_elementary_type(node: &ElementaryType<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_elementary_type(node) {
        return;
    }
    match node {
        ElementaryType::BoolKeyword(_) => {}
        ElementaryType::StringKeyword(_) => {}
        ElementaryType::AddressType(address_type) => {
            accept_address_type(address_type, visitor);
        }
        ElementaryType::BytesKeyword(bytes_keyword) => {
            visitor.visit_bytes_keyword(bytes_keyword);
        }
        ElementaryType::IntKeyword(int_keyword) => {
            visitor.visit_int_keyword(int_keyword);
        }
        ElementaryType::UintKeyword(uint_keyword) => {
            visitor.visit_uint_keyword(uint_keyword);
        }
        ElementaryType::FixedKeyword(fixed_keyword) => {
            visitor.visit_fixed_keyword(fixed_keyword);
        }
        ElementaryType::UfixedKeyword(ufixed_keyword) => {
            visitor.visit_ufixed_keyword(ufixed_keyword);
        }
    }
    visitor.leave_elementary_type(node);
}

pub fn accept_equality_expression_operator(
    node: &EqualityExpressionOperator<'_>,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_equality_expression_operator(node) {
        return;
    }
    match node {
        EqualityExpressionOperator::BangEqual(_) => {}
        EqualityExpressionOperator::EqualEqual(_) => {}
    }
    visitor.leave_equality_expression_operator(node);
}

pub fn accept_experimental_feature(node: &ExperimentalFeature, visitor: &mut impl Visitor) {
    if !visitor.enter_experimental_feature(node) {
        return;
    }
    match node {
        ExperimentalFeature::ABIEncoderV2 => {}
        ExperimentalFeature::SMTChecker => {}
        ExperimentalFeature::Solidity => {}
        ExperimentalFeature::Unrecognized => {}
    }
    visitor.leave_experimental_feature(node);
}

pub fn accept_expression(node: &Expression<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_expression(node) {
        return;
    }
    match node {
        Expression::AssignmentExpression(assignment_expression) => {
            accept_assignment_expression(assignment_expression, visitor);
        }
        Expression::ConditionalExpression(conditional_expression) => {
            accept_conditional_expression(conditional_expression, visitor);
        }
        Expression::OrExpression(or_expression) => {
            accept_or_expression(or_expression, visitor);
        }
        Expression::AndExpression(and_expression) => {
            accept_and_expression(and_expression, visitor);
        }
        Expression::EqualityExpression(equality_expression) => {
            accept_equality_expression(equality_expression, visitor);
        }
        Expression::InequalityExpression(inequality_expression) => {
            accept_inequality_expression(inequality_expression, visitor);
        }
        Expression::BitwiseOrExpression(bitwise_or_expression) => {
            accept_bitwise_or_expression(bitwise_or_expression, visitor);
        }
        Expression::BitwiseXorExpression(bitwise_xor_expression) => {
            accept_bitwise_xor_expression(bitwise_xor_expression, visitor);
        }
        Expression::BitwiseAndExpression(bitwise_and_expression) => {
            accept_bitwise_and_expression(bitwise_and_expression, visitor);
        }
        Expression::ShiftExpression(shift_expression) => {
            accept_shift_expression(shift_expression, visitor);
        }
        Expression::AdditiveExpression(additive_expression) => {
            accept_additive_expression(additive_expression, visitor);
        }
        Expression::MultiplicativeExpression(multiplicative_expression) => {
            accept_multiplicative_expression(multiplicative_expression, visitor);
        }
        Expression::ExponentiationExpression(exponentiation_expression) => {
            accept_exponentiation_expression(exponentiation_expression, visitor);
        }
        Expression::PostfixExpression(postfix_expression) => {
            accept_postfix_expression(postfix_expression, visitor);
        }
        Expression::PrefixExpression(prefix_expression) => {
            accept_prefix_expression(prefix_expression, visitor);
        }
        Expression::FunctionCallExpression(function_call_expression) => {
            accept_function_call_expression(function_call_expression, visitor);
        }
        Expression::CallOptionsExpression(call_options_expression) => {
            accept_call_options_expression(call_options_expression, visitor);
        }
        Expression::MemberAccessExpression(member_access_expression) => {
            accept_member_access_expression(member_access_expression, visitor);
        }
        Expression::IndexAccessExpression(index_access_expression) => {
            accept_index_access_expression(index_access_expression, visitor);
        }
        Expression::NewExpression(new_expression) => {
            accept_new_expression(new_expression, visitor);
        }
        Expression::TupleExpression(tuple_expression) => {
            accept_tuple_expression(tuple_expression, visitor);
        }
        Expression::TypeExpression(type_expression) => {
            accept_type_expression(type_expression, visitor);
        }
        Expression::ArrayExpression(array_expression) => {
            accept_array_expression(array_expression, visitor);
        }
        Expression::HexNumberExpression(hex_number_expression) => {
            accept_hex_number_expression(hex_number_expression, visitor);
        }
        Expression::DecimalNumberExpression(decimal_number_expression) => {
            accept_decimal_number_expression(decimal_number_expression, visitor);
        }
        Expression::StringExpression(string_expression) => {
            accept_string_expression(string_expression, visitor);
        }
        Expression::ElementaryType(elementary_type) => {
            accept_elementary_type(elementary_type, visitor);
        }
        Expression::PayableKeyword(_) => {}
        Expression::ThisKeyword(_) => {}
        Expression::SuperKeyword(_) => {}
        Expression::TrueKeyword(_) => {}
        Expression::FalseKeyword(_) => {}
        Expression::Identifier(identifier) => {
            visitor.visit_identifier(identifier);
        }
    }
    visitor.leave_expression(node);
}

pub fn accept_for_statement_condition(
    node: &ForStatementCondition<'_>,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_for_statement_condition(node) {
        return;
    }
    match node {
        ForStatementCondition::ExpressionStatement(expression_statement) => {
            accept_expression_statement(expression_statement, visitor);
        }
        ForStatementCondition::Semicolon(_) => {}
    }
    visitor.leave_for_statement_condition(node);
}

pub fn accept_for_statement_initialization(
    node: &ForStatementInitialization<'_>,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_for_statement_initialization(node) {
        return;
    }
    match node {
        ForStatementInitialization::VariableDeclarationStatement(
            variable_declaration_statement,
        ) => {
            accept_variable_declaration_statement(variable_declaration_statement, visitor);
        }
        ForStatementInitialization::ExpressionStatement(expression_statement) => {
            accept_expression_statement(expression_statement, visitor);
        }
        ForStatementInitialization::Semicolon(_) => {}
    }
    visitor.leave_for_statement_initialization(node);
}

pub fn accept_function_kind(node: &FunctionKind, visitor: &mut impl Visitor) {
    if !visitor.enter_function_kind(node) {
        return;
    }
    match node {
        FunctionKind::Regular => {}
        FunctionKind::Constructor => {}
        FunctionKind::Fallback => {}
        FunctionKind::Receive => {}
    }
    visitor.leave_function_kind(node);
}

pub fn accept_function_mutability(node: &FunctionMutability, visitor: &mut impl Visitor) {
    if !visitor.enter_function_mutability(node) {
        return;
    }
    match node {
        FunctionMutability::Pure => {}
        FunctionMutability::View => {}
        FunctionMutability::NonPayable => {}
        FunctionMutability::Payable => {}
    }
    visitor.leave_function_mutability(node);
}

pub fn accept_function_visibility(node: &FunctionVisibility, visitor: &mut impl Visitor) {
    if !visitor.enter_function_visibility(node) {
        return;
    }
    match node {
        FunctionVisibility::Public => {}
        FunctionVisibility::Private => {}
        FunctionVisibility::Internal => {}
        FunctionVisibility::External => {}
    }
    visitor.leave_function_visibility(node);
}

pub fn accept_import_clause(node: &ImportClause<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_import_clause(node) {
        return;
    }
    match node {
        ImportClause::PathImport(path_import) => {
            accept_path_import(path_import, visitor);
        }
        ImportClause::ImportDeconstruction(import_deconstruction) => {
            accept_import_deconstruction(import_deconstruction, visitor);
        }
    }
    visitor.leave_import_clause(node);
}

pub fn accept_inequality_expression_operator(
    node: &InequalityExpressionOperator<'_>,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_inequality_expression_operator(node) {
        return;
    }
    match node {
        InequalityExpressionOperator::GreaterThan(_) => {}
        InequalityExpressionOperator::GreaterThanEqual(_) => {}
        InequalityExpressionOperator::LessThan(_) => {}
        InequalityExpressionOperator::LessThanEqual(_) => {}
    }
    visitor.leave_inequality_expression_operator(node);
}

pub fn accept_multiplicative_expression_operator(
    node: &MultiplicativeExpressionOperator<'_>,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_multiplicative_expression_operator(node) {
        return;
    }
    match node {
        MultiplicativeExpressionOperator::Asterisk(_) => {}
        MultiplicativeExpressionOperator::Percent(_) => {}
        MultiplicativeExpressionOperator::Slash(_) => {}
    }
    visitor.leave_multiplicative_expression_operator(node);
}

pub fn accept_number_unit(node: &NumberUnit<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_number_unit(node) {
        return;
    }
    match node {
        NumberUnit::WeiKeyword(_) => {}
        NumberUnit::GweiKeyword(_) => {}
        NumberUnit::EtherKeyword(_) => {}
        NumberUnit::SecondsKeyword(_) => {}
        NumberUnit::MinutesKeyword(_) => {}
        NumberUnit::HoursKeyword(_) => {}
        NumberUnit::DaysKeyword(_) => {}
        NumberUnit::WeeksKeyword(_) => {}
    }
    visitor.leave_number_unit(node);
}

pub fn accept_postfix_expression_operator(
    node: &PostfixExpressionOperator<'_>,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_postfix_expression_operator(node) {
        return;
    }
    match node {
        PostfixExpressionOperator::MinusMinus(_) => {}
        PostfixExpressionOperator::PlusPlus(_) => {}
    }
    visitor.leave_postfix_expression_operator(node);
}

pub fn accept_pragma(node: &Pragma<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_pragma(node) {
        return;
    }
    match node {
        Pragma::VersionPragma(version_pragma) => {
            accept_version_pragma(version_pragma, visitor);
        }
        Pragma::AbicoderPragma(abicoder_pragma) => {
            accept_abicoder_pragma(abicoder_pragma, visitor);
        }
        Pragma::ExperimentalPragma(experimental_pragma) => {
            accept_experimental_pragma(experimental_pragma, visitor);
        }
    }
    visitor.leave_pragma(node);
}

pub fn accept_prefix_expression_operator(
    node: &PrefixExpressionOperator<'_>,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_prefix_expression_operator(node) {
        return;
    }
    match node {
        PrefixExpressionOperator::Bang(_) => {}
        PrefixExpressionOperator::DeleteKeyword(_) => {}
        PrefixExpressionOperator::Minus(_) => {}
        PrefixExpressionOperator::MinusMinus(_) => {}
        PrefixExpressionOperator::PlusPlus(_) => {}
        PrefixExpressionOperator::Tilde(_) => {}
    }
    visitor.leave_prefix_expression_operator(node);
}

pub fn accept_shift_expression_operator(
    node: &ShiftExpressionOperator<'_>,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_shift_expression_operator(node) {
        return;
    }
    match node {
        ShiftExpressionOperator::GreaterThanGreaterThan(_) => {}
        ShiftExpressionOperator::GreaterThanGreaterThanGreaterThan(_) => {}
        ShiftExpressionOperator::LessThanLessThan(_) => {}
    }
    visitor.leave_shift_expression_operator(node);
}

pub fn accept_source_unit_member(node: &SourceUnitMember<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_source_unit_member(node) {
        return;
    }
    match node {
        SourceUnitMember::PragmaDirective(pragma_directive) => {
            accept_pragma_directive(pragma_directive, visitor);
        }
        SourceUnitMember::ImportClause(import_clause) => {
            accept_import_clause(import_clause, visitor);
        }
        SourceUnitMember::ContractDefinition(contract_definition) => {
            accept_contract_definition(contract_definition, visitor);
        }
        SourceUnitMember::InterfaceDefinition(interface_definition) => {
            accept_interface_definition(interface_definition, visitor);
        }
        SourceUnitMember::LibraryDefinition(library_definition) => {
            accept_library_definition(library_definition, visitor);
        }
        SourceUnitMember::StructDefinition(struct_definition) => {
            accept_struct_definition(struct_definition, visitor);
        }
        SourceUnitMember::EnumDefinition(enum_definition) => {
            accept_enum_definition(enum_definition, visitor);
        }
        SourceUnitMember::FunctionDefinition(function_definition) => {
            accept_function_definition(function_definition, visitor);
        }
        SourceUnitMember::ErrorDefinition(error_definition) => {
            accept_error_definition(error_definition, visitor);
        }
        SourceUnitMember::UserDefinedValueTypeDefinition(user_defined_value_type_definition) => {
            accept_user_defined_value_type_definition(user_defined_value_type_definition, visitor);
        }
        SourceUnitMember::UsingDirective(using_directive) => {
            accept_using_directive(using_directive, visitor);
        }
        SourceUnitMember::EventDefinition(event_definition) => {
            accept_event_definition(event_definition, visitor);
        }
        SourceUnitMember::ConstantDefinition(constant_definition) => {
            accept_constant_definition(constant_definition, visitor);
        }
    }
    visitor.leave_source_unit_member(node);
}

pub fn accept_state_variable_mutability(
    node: &StateVariableMutability,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_state_variable_mutability(node) {
        return;
    }
    match node {
        StateVariableMutability::Mutable => {}
        StateVariableMutability::Constant => {}
        StateVariableMutability::Immutable => {}
        StateVariableMutability::Transient => {}
    }
    visitor.leave_state_variable_mutability(node);
}

pub fn accept_state_variable_visibility(
    node: &StateVariableVisibility,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_state_variable_visibility(node) {
        return;
    }
    match node {
        StateVariableVisibility::Public => {}
        StateVariableVisibility::Private => {}
        StateVariableVisibility::Internal => {}
    }
    visitor.leave_state_variable_visibility(node);
}

pub fn accept_statement(node: &Statement<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_statement(node) {
        return;
    }
    match node {
        Statement::IfStatement(if_statement) => {
            accept_if_statement(if_statement, visitor);
        }
        Statement::ForStatement(for_statement) => {
            accept_for_statement(for_statement, visitor);
        }
        Statement::WhileStatement(while_statement) => {
            accept_while_statement(while_statement, visitor);
        }
        Statement::DoWhileStatement(do_while_statement) => {
            accept_do_while_statement(do_while_statement, visitor);
        }
        Statement::ContinueStatement(continue_statement) => {
            accept_continue_statement(continue_statement, visitor);
        }
        Statement::BreakStatement(break_statement) => {
            accept_break_statement(break_statement, visitor);
        }
        Statement::ReturnStatement(return_statement) => {
            accept_return_statement(return_statement, visitor);
        }
        Statement::EmitStatement(emit_statement) => {
            accept_emit_statement(emit_statement, visitor);
        }
        Statement::TryStatement(try_statement) => {
            accept_try_statement(try_statement, visitor);
        }
        Statement::RevertStatement(revert_statement) => {
            accept_revert_statement(revert_statement, visitor);
        }
        Statement::AssemblyStatement(assembly_statement) => {
            accept_assembly_statement(assembly_statement, visitor);
        }
        Statement::Block(block) => {
            accept_block(block, visitor);
        }
        Statement::UncheckedBlock(unchecked_block) => {
            accept_unchecked_block(unchecked_block, visitor);
        }
        Statement::VariableDeclarationStatement(variable_declaration_statement) => {
            accept_variable_declaration_statement(variable_declaration_statement, visitor);
        }
        Statement::ExpressionStatement(expression_statement) => {
            accept_expression_statement(expression_statement, visitor);
        }
    }
    visitor.leave_statement(node);
}

pub fn accept_storage_location(node: &StorageLocation<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_storage_location(node) {
        return;
    }
    match node {
        StorageLocation::MemoryKeyword(_) => {}
        StorageLocation::StorageKeyword(_) => {}
        StorageLocation::CallDataKeyword(_) => {}
    }
    visitor.leave_storage_location(node);
}

pub fn accept_string_expression(node: &StringExpression<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_string_expression(node) {
        return;
    }
    match node {
        StringExpression::StringLiterals(string_literals) => {
            accept_string_literals(string_literals, visitor);
        }
        StringExpression::HexStringLiterals(hex_string_literals) => {
            accept_hex_string_literals(hex_string_literals, visitor);
        }
        StringExpression::UnicodeStringLiterals(unicode_string_literals) => {
            accept_unicode_string_literals(unicode_string_literals, visitor);
        }
    }
    visitor.leave_string_expression(node);
}

pub fn accept_type_name(node: &TypeName<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_type_name(node) {
        return;
    }
    match node {
        TypeName::ArrayTypeName(array_type_name) => {
            accept_array_type_name(array_type_name, visitor);
        }
        TypeName::FunctionType(function_type) => {
            accept_function_type(function_type, visitor);
        }
        TypeName::MappingType(mapping_type) => {
            accept_mapping_type(mapping_type, visitor);
        }
        TypeName::ElementaryType(elementary_type) => {
            accept_elementary_type(elementary_type, visitor);
        }
        TypeName::IdentifierPath(identifier_path) => {
            accept_identifier_path(identifier_path, visitor);
        }
    }
    visitor.leave_type_name(node);
}

pub fn accept_using_clause(node: &UsingClause<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_using_clause(node) {
        return;
    }
    match node {
        UsingClause::IdentifierPath(identifier_path) => {
            accept_identifier_path(identifier_path, visitor);
        }
        UsingClause::UsingDeconstruction(using_deconstruction) => {
            accept_using_deconstruction(using_deconstruction, visitor);
        }
    }
    visitor.leave_using_clause(node);
}

pub fn accept_using_operator(node: &UsingOperator<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_using_operator(node) {
        return;
    }
    match node {
        UsingOperator::Ampersand(_) => {}
        UsingOperator::Asterisk(_) => {}
        UsingOperator::BangEqual(_) => {}
        UsingOperator::Bar(_) => {}
        UsingOperator::Caret(_) => {}
        UsingOperator::EqualEqual(_) => {}
        UsingOperator::GreaterThan(_) => {}
        UsingOperator::GreaterThanEqual(_) => {}
        UsingOperator::LessThan(_) => {}
        UsingOperator::LessThanEqual(_) => {}
        UsingOperator::Minus(_) => {}
        UsingOperator::Percent(_) => {}
        UsingOperator::Plus(_) => {}
        UsingOperator::Slash(_) => {}
        UsingOperator::Tilde(_) => {}
    }
    visitor.leave_using_operator(node);
}

pub fn accept_using_target(node: &UsingTarget<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_using_target(node) {
        return;
    }
    match node {
        UsingTarget::TypeName(type_name) => {
            accept_type_name(type_name, visitor);
        }
        UsingTarget::Asterisk(_) => {}
    }
    visitor.leave_using_target(node);
}

pub fn accept_variable_declaration_target(
    node: &VariableDeclarationTarget<'_>,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_variable_declaration_target(node) {
        return;
    }
    match node {
        VariableDeclarationTarget::SingleTypedDeclaration(single_typed_declaration) => {
            accept_single_typed_declaration(single_typed_declaration, visitor);
        }
        VariableDeclarationTarget::MultiTypedDeclaration(multi_typed_declaration) => {
            accept_multi_typed_declaration(multi_typed_declaration, visitor);
        }
    }
    visitor.leave_variable_declaration_target(node);
}

pub fn accept_version_pragma_component(
    node: &VersionPragmaComponent<'_>,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_version_pragma_component(node) {
        return;
    }
    match node {
        VersionPragmaComponent::Wildcard => {}
        VersionPragmaComponent::Unrecognized => {}
        VersionPragmaComponent::Number(_) => {}
    }
    visitor.leave_version_pragma_component(node);
}

pub fn accept_version_pragma_operator(node: &VersionPragmaOperator, visitor: &mut impl Visitor) {
    if !visitor.enter_version_pragma_operator(node) {
        return;
    }
    match node {
        VersionPragmaOperator::Caret => {}
        VersionPragmaOperator::Tilde => {}
        VersionPragmaOperator::Equal => {}
        VersionPragmaOperator::LessThan => {}
        VersionPragmaOperator::LessThanEqual => {}
        VersionPragmaOperator::GreaterThan => {}
        VersionPragmaOperator::GreaterThanEqual => {}
    }
    visitor.leave_version_pragma_operator(node);
}

pub fn accept_yul_expression(node: &YulExpression<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_expression(node) {
        return;
    }
    match node {
        YulExpression::YulFunctionCallExpression(yul_function_call_expression) => {
            accept_yul_function_call_expression(yul_function_call_expression, visitor);
        }
        YulExpression::YulLiteral(yul_literal) => {
            accept_yul_literal(yul_literal, visitor);
        }
        YulExpression::YulPath(yul_path) => {
            accept_yul_path(yul_path, visitor);
        }
    }
    visitor.leave_yul_expression(node);
}

pub fn accept_yul_literal(node: &YulLiteral<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_literal(node) {
        return;
    }
    match node {
        YulLiteral::TrueKeyword(_) => {}
        YulLiteral::FalseKeyword(_) => {}
        YulLiteral::DecimalLiteral(decimal_literal) => {
            visitor.visit_decimal_literal(decimal_literal);
        }
        YulLiteral::HexLiteral(hex_literal) => {
            visitor.visit_hex_literal(hex_literal);
        }
        YulLiteral::HexStringLiteral(hex_string_literal) => {
            visitor.visit_hex_string_literal(hex_string_literal);
        }
        YulLiteral::StringLiteral(string_literal) => {
            visitor.visit_string_literal(string_literal);
        }
    }
    visitor.leave_yul_literal(node);
}

pub fn accept_yul_statement(node: &YulStatement<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_statement(node) {
        return;
    }
    match node {
        YulStatement::YulBlock(yul_block) => {
            accept_yul_block(yul_block, visitor);
        }
        YulStatement::YulFunctionDefinition(yul_function_definition) => {
            accept_yul_function_definition(yul_function_definition, visitor);
        }
        YulStatement::YulIfStatement(yul_if_statement) => {
            accept_yul_if_statement(yul_if_statement, visitor);
        }
        YulStatement::YulForStatement(yul_for_statement) => {
            accept_yul_for_statement(yul_for_statement, visitor);
        }
        YulStatement::YulSwitchStatement(yul_switch_statement) => {
            accept_yul_switch_statement(yul_switch_statement, visitor);
        }
        YulStatement::YulLeaveStatement(yul_leave_statement) => {
            accept_yul_leave_statement(yul_leave_statement, visitor);
        }
        YulStatement::YulBreakStatement(yul_break_statement) => {
            accept_yul_break_statement(yul_break_statement, visitor);
        }
        YulStatement::YulContinueStatement(yul_continue_statement) => {
            accept_yul_continue_statement(yul_continue_statement, visitor);
        }
        YulStatement::YulVariableAssignmentStatement(yul_variable_assignment_statement) => {
            accept_yul_variable_assignment_statement(yul_variable_assignment_statement, visitor);
        }
        YulStatement::YulVariableDeclarationStatement(yul_variable_declaration_statement) => {
            accept_yul_variable_declaration_statement(yul_variable_declaration_statement, visitor);
        }
        YulStatement::YulExpression(yul_expression) => {
            accept_yul_expression(yul_expression, visitor);
        }
    }
    visitor.leave_yul_statement(node);
}

//
// Repeated & Separated
//

#[inline]
fn accept_array_values(items: &ArrayValues<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_array_values(items) {
        return;
    }
    for item in items.iter() {
        accept_expression(&item, visitor);
    }
    visitor.leave_array_values(items);
}

#[inline]
fn accept_call_options(items: &CallOptions<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_call_options(items) {
        return;
    }
    for item in items.iter() {
        accept_named_argument(&item, visitor);
    }
    visitor.leave_call_options(items);
}

#[inline]
fn accept_catch_clauses(items: &CatchClauses<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_catch_clauses(items) {
        return;
    }
    for item in items.iter() {
        accept_catch_clause(&item, visitor);
    }
    visitor.leave_catch_clauses(items);
}

#[inline]
fn accept_contract_members(items: &ContractMembers<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_contract_members(items) {
        return;
    }
    for item in items.iter() {
        accept_contract_member(&item, visitor);
    }
    visitor.leave_contract_members(items);
}

#[inline]
fn accept_enum_members(items: &EnumMembers<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_enum_members(items) {
        return;
    }
    for item in items.iter() {
        visitor.visit_identifier(&item);
    }
    visitor.leave_enum_members(items);
}

#[inline]
fn accept_hex_string_literals(items: &HexStringLiterals<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_hex_string_literals(items) {
        return;
    }
    for item in items.iter() {
        visitor.visit_hex_string_literal(&item);
    }
    visitor.leave_hex_string_literals(items);
}

#[inline]
fn accept_identifier_path(items: &IdentifierPath<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_identifier_path(items) {
        return;
    }
    for item in items.iter() {
        visitor.visit_identifier(&item);
    }
    visitor.leave_identifier_path(items);
}

#[inline]
fn accept_import_deconstruction_symbols(
    items: &ImportDeconstructionSymbols<'_>,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_import_deconstruction_symbols(items) {
        return;
    }
    for item in items.iter() {
        accept_import_deconstruction_symbol(&item, visitor);
    }
    visitor.leave_import_deconstruction_symbols(items);
}

#[inline]
fn accept_inheritance_types(items: &InheritanceTypes<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_inheritance_types(items) {
        return;
    }
    for item in items.iter() {
        accept_inheritance_type(&item, visitor);
    }
    visitor.leave_inheritance_types(items);
}

#[inline]
fn accept_interface_members(items: &InterfaceMembers<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_interface_members(items) {
        return;
    }
    for item in items.iter() {
        accept_contract_member(&item, visitor);
    }
    visitor.leave_interface_members(items);
}

#[inline]
fn accept_library_members(items: &LibraryMembers<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_library_members(items) {
        return;
    }
    for item in items.iter() {
        accept_contract_member(&item, visitor);
    }
    visitor.leave_library_members(items);
}

#[inline]
fn accept_modifier_invocations(items: &ModifierInvocations<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_modifier_invocations(items) {
        return;
    }
    for item in items.iter() {
        accept_modifier_invocation(&item, visitor);
    }
    visitor.leave_modifier_invocations(items);
}

#[inline]
fn accept_multi_typed_declaration_elements(
    items: &MultiTypedDeclarationElements<'_>,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_multi_typed_declaration_elements(items) {
        return;
    }
    for item in items.iter() {
        accept_multi_typed_declaration_element(&item, visitor);
    }
    visitor.leave_multi_typed_declaration_elements(items);
}

#[inline]
fn accept_named_arguments(items: &NamedArguments<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_named_arguments(items) {
        return;
    }
    for item in items.iter() {
        accept_named_argument(&item, visitor);
    }
    visitor.leave_named_arguments(items);
}

#[inline]
fn accept_override_paths(items: &OverridePaths<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_override_paths(items) {
        return;
    }
    for item in items.iter() {
        accept_identifier_path(&item, visitor);
    }
    visitor.leave_override_paths(items);
}

#[inline]
fn accept_parameters(items: &Parameters<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_parameters(items) {
        return;
    }
    for item in items.iter() {
        accept_parameter(&item, visitor);
    }
    visitor.leave_parameters(items);
}

#[inline]
fn accept_positional_arguments(items: &PositionalArguments<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_positional_arguments(items) {
        return;
    }
    for item in items.iter() {
        accept_expression(&item, visitor);
    }
    visitor.leave_positional_arguments(items);
}

#[inline]
fn accept_source_unit_members(items: &SourceUnitMembers<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_source_unit_members(items) {
        return;
    }
    for item in items.iter() {
        accept_source_unit_member(&item, visitor);
    }
    visitor.leave_source_unit_members(items);
}

#[inline]
fn accept_statements(items: &Statements<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_statements(items) {
        return;
    }
    for item in items.iter() {
        accept_statement(&item, visitor);
    }
    visitor.leave_statements(items);
}

#[inline]
fn accept_string_literals(items: &StringLiterals<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_string_literals(items) {
        return;
    }
    for item in items.iter() {
        visitor.visit_string_literal(&item);
    }
    visitor.leave_string_literals(items);
}

#[inline]
fn accept_struct_members(items: &StructMembers<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_struct_members(items) {
        return;
    }
    for item in items.iter() {
        accept_struct_member(&item, visitor);
    }
    visitor.leave_struct_members(items);
}

#[inline]
fn accept_tuple_values(items: &TupleValues<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_tuple_values(items) {
        return;
    }
    for item in items.iter() {
        accept_tuple_value(&item, visitor);
    }
    visitor.leave_tuple_values(items);
}

#[inline]
fn accept_unicode_string_literals(items: &UnicodeStringLiterals<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_unicode_string_literals(items) {
        return;
    }
    for item in items.iter() {
        visitor.visit_unicode_string_literal(&item);
    }
    visitor.leave_unicode_string_literals(items);
}

#[inline]
fn accept_using_deconstruction_symbols(
    items: &UsingDeconstructionSymbols<'_>,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_using_deconstruction_symbols(items) {
        return;
    }
    for item in items.iter() {
        accept_using_deconstruction_symbol(&item, visitor);
    }
    visitor.leave_using_deconstruction_symbols(items);
}

#[inline]
fn accept_version_pragma_expression_set(
    items: &VersionPragmaExpressionSet<'_>,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_version_pragma_expression_set(items) {
        return;
    }
    for item in items.iter() {
        accept_version_pragma_comparator(&item, visitor);
    }
    visitor.leave_version_pragma_expression_set(items);
}

#[inline]
fn accept_version_pragma_expression_sets(
    items: &VersionPragmaExpressionSets<'_>,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_version_pragma_expression_sets(items) {
        return;
    }
    for item in items.iter() {
        accept_version_pragma_expression_set(&item, visitor);
    }
    visitor.leave_version_pragma_expression_sets(items);
}

#[inline]
fn accept_version_pragma_specifier(items: &VersionPragmaSpecifier<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_version_pragma_specifier(items) {
        return;
    }
    for item in items.iter() {
        accept_version_pragma_component(&item, visitor);
    }
    visitor.leave_version_pragma_specifier(items);
}

#[inline]
fn accept_yul_arguments(items: &YulArguments<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_arguments(items) {
        return;
    }
    for item in items.iter() {
        accept_yul_expression(&item, visitor);
    }
    visitor.leave_yul_arguments(items);
}

#[inline]
fn accept_yul_parameters(items: &YulParameters<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_parameters(items) {
        return;
    }
    for item in items.iter() {
        visitor.visit_identifier(&item);
    }
    visitor.leave_yul_parameters(items);
}

#[inline]
fn accept_yul_path(items: &YulPath<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_path(items) {
        return;
    }
    for item in items.iter() {
        visitor.visit_identifier(&item);
    }
    visitor.leave_yul_path(items);
}

#[inline]
fn accept_yul_paths(items: &YulPaths<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_paths(items) {
        return;
    }
    for item in items.iter() {
        accept_yul_path(&item, visitor);
    }
    visitor.leave_yul_paths(items);
}

#[inline]
fn accept_yul_statements(items: &YulStatements<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_statements(items) {
        return;
    }
    for item in items.iter() {
        accept_yul_statement(&item, visitor);
    }
    visitor.leave_yul_statements(items);
}

#[inline]
fn accept_yul_value_cases(items: &YulValueCases<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_value_cases(items) {
        return;
    }
    for item in items.iter() {
        accept_yul_value_case(&item, visitor);
    }
    visitor.leave_yul_value_cases(items);
}

#[inline]
fn accept_yul_variable_names(items: &YulVariableNames<'_>, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_variable_names(items) {
        return;
    }
    for item in items.iter() {
        visitor.visit_identifier(&item);
    }
    visitor.leave_yul_variable_names(items);
}
