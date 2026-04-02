// This file is generated automatically by infrastructure scripts. Please don't edit by hand.

#![allow(non_camel_case_types)]
#![allow(unused_variables)]

#[allow(clippy::wildcard_imports)]
use super::nodes::*;

pub trait Visitor {
    fn enter_abicoder_pragma(&mut self, _node: &AbicoderPragma) -> bool {
        true
    }
    fn leave_abicoder_pragma(&mut self, _node: &AbicoderPragma) {}

    fn enter_additive_expression(&mut self, _node: &AdditiveExpression) -> bool {
        true
    }
    fn leave_additive_expression(&mut self, _node: &AdditiveExpression) {}

    fn enter_address_type(&mut self, _node: &AddressType) -> bool {
        true
    }
    fn leave_address_type(&mut self, _node: &AddressType) {}

    fn enter_and_expression(&mut self, _node: &AndExpression) -> bool {
        true
    }
    fn leave_and_expression(&mut self, _node: &AndExpression) {}

    fn enter_array_expression(&mut self, _node: &ArrayExpression) -> bool {
        true
    }
    fn leave_array_expression(&mut self, _node: &ArrayExpression) {}

    fn enter_array_type_name(&mut self, _node: &ArrayTypeName) -> bool {
        true
    }
    fn leave_array_type_name(&mut self, _node: &ArrayTypeName) {}

    fn enter_assembly_statement(&mut self, _node: &AssemblyStatement) -> bool {
        true
    }
    fn leave_assembly_statement(&mut self, _node: &AssemblyStatement) {}

    fn enter_assignment_expression(&mut self, _node: &AssignmentExpression) -> bool {
        true
    }
    fn leave_assignment_expression(&mut self, _node: &AssignmentExpression) {}

    fn enter_bitwise_and_expression(&mut self, _node: &BitwiseAndExpression) -> bool {
        true
    }
    fn leave_bitwise_and_expression(&mut self, _node: &BitwiseAndExpression) {}

    fn enter_bitwise_or_expression(&mut self, _node: &BitwiseOrExpression) -> bool {
        true
    }
    fn leave_bitwise_or_expression(&mut self, _node: &BitwiseOrExpression) {}

    fn enter_bitwise_xor_expression(&mut self, _node: &BitwiseXorExpression) -> bool {
        true
    }
    fn leave_bitwise_xor_expression(&mut self, _node: &BitwiseXorExpression) {}

    fn enter_block(&mut self, _node: &Block) -> bool {
        true
    }
    fn leave_block(&mut self, _node: &Block) {}

    fn enter_break_statement(&mut self, _node: &BreakStatement) -> bool {
        true
    }
    fn leave_break_statement(&mut self, _node: &BreakStatement) {}

    fn enter_call_options_expression(&mut self, _node: &CallOptionsExpression) -> bool {
        true
    }
    fn leave_call_options_expression(&mut self, _node: &CallOptionsExpression) {}

    fn enter_catch_clause(&mut self, _node: &CatchClause) -> bool {
        true
    }
    fn leave_catch_clause(&mut self, _node: &CatchClause) {}

    fn enter_catch_clause_error(&mut self, _node: &CatchClauseError) -> bool {
        true
    }
    fn leave_catch_clause_error(&mut self, _node: &CatchClauseError) {}

    fn enter_conditional_expression(&mut self, _node: &ConditionalExpression) -> bool {
        true
    }
    fn leave_conditional_expression(&mut self, _node: &ConditionalExpression) {}

    fn enter_constant_definition(&mut self, _node: &ConstantDefinition) -> bool {
        true
    }
    fn leave_constant_definition(&mut self, _node: &ConstantDefinition) {}

    fn enter_constructor_definition(&mut self, _node: &ConstructorDefinition) -> bool {
        true
    }
    fn leave_constructor_definition(&mut self, _node: &ConstructorDefinition) {}

    fn enter_continue_statement(&mut self, _node: &ContinueStatement) -> bool {
        true
    }
    fn leave_continue_statement(&mut self, _node: &ContinueStatement) {}

    fn enter_contract_definition(&mut self, _node: &ContractDefinition) -> bool {
        true
    }
    fn leave_contract_definition(&mut self, _node: &ContractDefinition) {}

    fn enter_decimal_number_expression(&mut self, _node: &DecimalNumberExpression) -> bool {
        true
    }
    fn leave_decimal_number_expression(&mut self, _node: &DecimalNumberExpression) {}

    fn enter_do_while_statement(&mut self, _node: &DoWhileStatement) -> bool {
        true
    }
    fn leave_do_while_statement(&mut self, _node: &DoWhileStatement) {}

    fn enter_else_branch(&mut self, _node: &ElseBranch) -> bool {
        true
    }
    fn leave_else_branch(&mut self, _node: &ElseBranch) {}

    fn enter_emit_statement(&mut self, _node: &EmitStatement) -> bool {
        true
    }
    fn leave_emit_statement(&mut self, _node: &EmitStatement) {}

    fn enter_enum_definition(&mut self, _node: &EnumDefinition) -> bool {
        true
    }
    fn leave_enum_definition(&mut self, _node: &EnumDefinition) {}

    fn enter_equality_expression(&mut self, _node: &EqualityExpression) -> bool {
        true
    }
    fn leave_equality_expression(&mut self, _node: &EqualityExpression) {}

    fn enter_error_definition(&mut self, _node: &ErrorDefinition) -> bool {
        true
    }
    fn leave_error_definition(&mut self, _node: &ErrorDefinition) {}

    fn enter_error_parameter(&mut self, _node: &ErrorParameter) -> bool {
        true
    }
    fn leave_error_parameter(&mut self, _node: &ErrorParameter) {}

    fn enter_error_parameters_declaration(&mut self, _node: &ErrorParametersDeclaration) -> bool {
        true
    }
    fn leave_error_parameters_declaration(&mut self, _node: &ErrorParametersDeclaration) {}

    fn enter_event_definition(&mut self, _node: &EventDefinition) -> bool {
        true
    }
    fn leave_event_definition(&mut self, _node: &EventDefinition) {}

    fn enter_event_parameter(&mut self, _node: &EventParameter) -> bool {
        true
    }
    fn leave_event_parameter(&mut self, _node: &EventParameter) {}

    fn enter_event_parameters_declaration(&mut self, _node: &EventParametersDeclaration) -> bool {
        true
    }
    fn leave_event_parameters_declaration(&mut self, _node: &EventParametersDeclaration) {}

    fn enter_experimental_pragma(&mut self, _node: &ExperimentalPragma) -> bool {
        true
    }
    fn leave_experimental_pragma(&mut self, _node: &ExperimentalPragma) {}

    fn enter_exponentiation_expression(&mut self, _node: &ExponentiationExpression) -> bool {
        true
    }
    fn leave_exponentiation_expression(&mut self, _node: &ExponentiationExpression) {}

    fn enter_expression_statement(&mut self, _node: &ExpressionStatement) -> bool {
        true
    }
    fn leave_expression_statement(&mut self, _node: &ExpressionStatement) {}

    fn enter_fallback_function_definition(&mut self, _node: &FallbackFunctionDefinition) -> bool {
        true
    }
    fn leave_fallback_function_definition(&mut self, _node: &FallbackFunctionDefinition) {}

    fn enter_for_statement(&mut self, _node: &ForStatement) -> bool {
        true
    }
    fn leave_for_statement(&mut self, _node: &ForStatement) {}

    fn enter_function_call_expression(&mut self, _node: &FunctionCallExpression) -> bool {
        true
    }
    fn leave_function_call_expression(&mut self, _node: &FunctionCallExpression) {}

    fn enter_function_definition(&mut self, _node: &FunctionDefinition) -> bool {
        true
    }
    fn leave_function_definition(&mut self, _node: &FunctionDefinition) {}

    fn enter_function_type(&mut self, _node: &FunctionType) -> bool {
        true
    }
    fn leave_function_type(&mut self, _node: &FunctionType) {}

    fn enter_hex_number_expression(&mut self, _node: &HexNumberExpression) -> bool {
        true
    }
    fn leave_hex_number_expression(&mut self, _node: &HexNumberExpression) {}

    fn enter_if_statement(&mut self, _node: &IfStatement) -> bool {
        true
    }
    fn leave_if_statement(&mut self, _node: &IfStatement) {}

    fn enter_import_alias(&mut self, _node: &ImportAlias) -> bool {
        true
    }
    fn leave_import_alias(&mut self, _node: &ImportAlias) {}

    fn enter_import_deconstruction(&mut self, _node: &ImportDeconstruction) -> bool {
        true
    }
    fn leave_import_deconstruction(&mut self, _node: &ImportDeconstruction) {}

    fn enter_import_deconstruction_symbol(&mut self, _node: &ImportDeconstructionSymbol) -> bool {
        true
    }
    fn leave_import_deconstruction_symbol(&mut self, _node: &ImportDeconstructionSymbol) {}

    fn enter_import_directive(&mut self, _node: &ImportDirective) -> bool {
        true
    }
    fn leave_import_directive(&mut self, _node: &ImportDirective) {}

    fn enter_index_access_end(&mut self, _node: &IndexAccessEnd) -> bool {
        true
    }
    fn leave_index_access_end(&mut self, _node: &IndexAccessEnd) {}

    fn enter_index_access_expression(&mut self, _node: &IndexAccessExpression) -> bool {
        true
    }
    fn leave_index_access_expression(&mut self, _node: &IndexAccessExpression) {}

    fn enter_inequality_expression(&mut self, _node: &InequalityExpression) -> bool {
        true
    }
    fn leave_inequality_expression(&mut self, _node: &InequalityExpression) {}

    fn enter_inheritance_specifier(&mut self, _node: &InheritanceSpecifier) -> bool {
        true
    }
    fn leave_inheritance_specifier(&mut self, _node: &InheritanceSpecifier) {}

    fn enter_inheritance_type(&mut self, _node: &InheritanceType) -> bool {
        true
    }
    fn leave_inheritance_type(&mut self, _node: &InheritanceType) {}

    fn enter_interface_definition(&mut self, _node: &InterfaceDefinition) -> bool {
        true
    }
    fn leave_interface_definition(&mut self, _node: &InterfaceDefinition) {}

    fn enter_library_definition(&mut self, _node: &LibraryDefinition) -> bool {
        true
    }
    fn leave_library_definition(&mut self, _node: &LibraryDefinition) {}

    fn enter_mapping_key(&mut self, _node: &MappingKey) -> bool {
        true
    }
    fn leave_mapping_key(&mut self, _node: &MappingKey) {}

    fn enter_mapping_type(&mut self, _node: &MappingType) -> bool {
        true
    }
    fn leave_mapping_type(&mut self, _node: &MappingType) {}

    fn enter_mapping_value(&mut self, _node: &MappingValue) -> bool {
        true
    }
    fn leave_mapping_value(&mut self, _node: &MappingValue) {}

    fn enter_member_access_expression(&mut self, _node: &MemberAccessExpression) -> bool {
        true
    }
    fn leave_member_access_expression(&mut self, _node: &MemberAccessExpression) {}

    fn enter_modifier_definition(&mut self, _node: &ModifierDefinition) -> bool {
        true
    }
    fn leave_modifier_definition(&mut self, _node: &ModifierDefinition) {}

    fn enter_modifier_invocation(&mut self, _node: &ModifierInvocation) -> bool {
        true
    }
    fn leave_modifier_invocation(&mut self, _node: &ModifierInvocation) {}

    fn enter_multi_typed_declaration(&mut self, _node: &MultiTypedDeclaration) -> bool {
        true
    }
    fn leave_multi_typed_declaration(&mut self, _node: &MultiTypedDeclaration) {}

    fn enter_multi_typed_declaration_element(
        &mut self,
        _node: &MultiTypedDeclarationElement,
    ) -> bool {
        true
    }
    fn leave_multi_typed_declaration_element(&mut self, _node: &MultiTypedDeclarationElement) {}

    fn enter_multiplicative_expression(&mut self, _node: &MultiplicativeExpression) -> bool {
        true
    }
    fn leave_multiplicative_expression(&mut self, _node: &MultiplicativeExpression) {}

    fn enter_named_argument(&mut self, _node: &NamedArgument) -> bool {
        true
    }
    fn leave_named_argument(&mut self, _node: &NamedArgument) {}

    fn enter_named_argument_group(&mut self, _node: &NamedArgumentGroup) -> bool {
        true
    }
    fn leave_named_argument_group(&mut self, _node: &NamedArgumentGroup) {}

    fn enter_named_arguments_declaration(&mut self, _node: &NamedArgumentsDeclaration) -> bool {
        true
    }
    fn leave_named_arguments_declaration(&mut self, _node: &NamedArgumentsDeclaration) {}

    fn enter_named_import(&mut self, _node: &NamedImport) -> bool {
        true
    }
    fn leave_named_import(&mut self, _node: &NamedImport) {}

    fn enter_new_expression(&mut self, _node: &NewExpression) -> bool {
        true
    }
    fn leave_new_expression(&mut self, _node: &NewExpression) {}

    fn enter_or_expression(&mut self, _node: &OrExpression) -> bool {
        true
    }
    fn leave_or_expression(&mut self, _node: &OrExpression) {}

    fn enter_override_paths_declaration(&mut self, _node: &OverridePathsDeclaration) -> bool {
        true
    }
    fn leave_override_paths_declaration(&mut self, _node: &OverridePathsDeclaration) {}

    fn enter_override_specifier(&mut self, _node: &OverrideSpecifier) -> bool {
        true
    }
    fn leave_override_specifier(&mut self, _node: &OverrideSpecifier) {}

    fn enter_parameter(&mut self, _node: &Parameter) -> bool {
        true
    }
    fn leave_parameter(&mut self, _node: &Parameter) {}

    fn enter_parameters_declaration(&mut self, _node: &ParametersDeclaration) -> bool {
        true
    }
    fn leave_parameters_declaration(&mut self, _node: &ParametersDeclaration) {}

    fn enter_path_import(&mut self, _node: &PathImport) -> bool {
        true
    }
    fn leave_path_import(&mut self, _node: &PathImport) {}

    fn enter_positional_arguments_declaration(
        &mut self,
        _node: &PositionalArgumentsDeclaration,
    ) -> bool {
        true
    }
    fn leave_positional_arguments_declaration(&mut self, _node: &PositionalArgumentsDeclaration) {}

    fn enter_postfix_expression(&mut self, _node: &PostfixExpression) -> bool {
        true
    }
    fn leave_postfix_expression(&mut self, _node: &PostfixExpression) {}

    fn enter_pragma_directive(&mut self, _node: &PragmaDirective) -> bool {
        true
    }
    fn leave_pragma_directive(&mut self, _node: &PragmaDirective) {}

    fn enter_prefix_expression(&mut self, _node: &PrefixExpression) -> bool {
        true
    }
    fn leave_prefix_expression(&mut self, _node: &PrefixExpression) {}

    fn enter_receive_function_definition(&mut self, _node: &ReceiveFunctionDefinition) -> bool {
        true
    }
    fn leave_receive_function_definition(&mut self, _node: &ReceiveFunctionDefinition) {}

    fn enter_return_statement(&mut self, _node: &ReturnStatement) -> bool {
        true
    }
    fn leave_return_statement(&mut self, _node: &ReturnStatement) {}

    fn enter_returns_declaration(&mut self, _node: &ReturnsDeclaration) -> bool {
        true
    }
    fn leave_returns_declaration(&mut self, _node: &ReturnsDeclaration) {}

    fn enter_revert_statement(&mut self, _node: &RevertStatement) -> bool {
        true
    }
    fn leave_revert_statement(&mut self, _node: &RevertStatement) {}

    fn enter_shift_expression(&mut self, _node: &ShiftExpression) -> bool {
        true
    }
    fn leave_shift_expression(&mut self, _node: &ShiftExpression) {}

    fn enter_single_typed_declaration(&mut self, _node: &SingleTypedDeclaration) -> bool {
        true
    }
    fn leave_single_typed_declaration(&mut self, _node: &SingleTypedDeclaration) {}

    fn enter_source_unit(&mut self, _node: &SourceUnit) -> bool {
        true
    }
    fn leave_source_unit(&mut self, _node: &SourceUnit) {}

    fn enter_state_variable_definition(&mut self, _node: &StateVariableDefinition) -> bool {
        true
    }
    fn leave_state_variable_definition(&mut self, _node: &StateVariableDefinition) {}

    fn enter_state_variable_definition_value(
        &mut self,
        _node: &StateVariableDefinitionValue,
    ) -> bool {
        true
    }
    fn leave_state_variable_definition_value(&mut self, _node: &StateVariableDefinitionValue) {}

    fn enter_storage_layout_specifier(&mut self, _node: &StorageLayoutSpecifier) -> bool {
        true
    }
    fn leave_storage_layout_specifier(&mut self, _node: &StorageLayoutSpecifier) {}

    fn enter_struct_definition(&mut self, _node: &StructDefinition) -> bool {
        true
    }
    fn leave_struct_definition(&mut self, _node: &StructDefinition) {}

    fn enter_struct_member(&mut self, _node: &StructMember) -> bool {
        true
    }
    fn leave_struct_member(&mut self, _node: &StructMember) {}

    fn enter_try_statement(&mut self, _node: &TryStatement) -> bool {
        true
    }
    fn leave_try_statement(&mut self, _node: &TryStatement) {}

    fn enter_tuple_expression(&mut self, _node: &TupleExpression) -> bool {
        true
    }
    fn leave_tuple_expression(&mut self, _node: &TupleExpression) {}

    fn enter_tuple_value(&mut self, _node: &TupleValue) -> bool {
        true
    }
    fn leave_tuple_value(&mut self, _node: &TupleValue) {}

    fn enter_type_expression(&mut self, _node: &TypeExpression) -> bool {
        true
    }
    fn leave_type_expression(&mut self, _node: &TypeExpression) {}

    fn enter_unchecked_block(&mut self, _node: &UncheckedBlock) -> bool {
        true
    }
    fn leave_unchecked_block(&mut self, _node: &UncheckedBlock) {}

    fn enter_user_defined_value_type_definition(
        &mut self,
        _node: &UserDefinedValueTypeDefinition,
    ) -> bool {
        true
    }
    fn leave_user_defined_value_type_definition(&mut self, _node: &UserDefinedValueTypeDefinition) {
    }

    fn enter_using_alias(&mut self, _node: &UsingAlias) -> bool {
        true
    }
    fn leave_using_alias(&mut self, _node: &UsingAlias) {}

    fn enter_using_deconstruction(&mut self, _node: &UsingDeconstruction) -> bool {
        true
    }
    fn leave_using_deconstruction(&mut self, _node: &UsingDeconstruction) {}

    fn enter_using_deconstruction_symbol(&mut self, _node: &UsingDeconstructionSymbol) -> bool {
        true
    }
    fn leave_using_deconstruction_symbol(&mut self, _node: &UsingDeconstructionSymbol) {}

    fn enter_using_directive(&mut self, _node: &UsingDirective) -> bool {
        true
    }
    fn leave_using_directive(&mut self, _node: &UsingDirective) {}

    fn enter_variable_declaration(&mut self, _node: &VariableDeclaration) -> bool {
        true
    }
    fn leave_variable_declaration(&mut self, _node: &VariableDeclaration) {}

    fn enter_variable_declaration_statement(
        &mut self,
        _node: &VariableDeclarationStatement,
    ) -> bool {
        true
    }
    fn leave_variable_declaration_statement(&mut self, _node: &VariableDeclarationStatement) {}

    fn enter_variable_declaration_value(&mut self, _node: &VariableDeclarationValue) -> bool {
        true
    }
    fn leave_variable_declaration_value(&mut self, _node: &VariableDeclarationValue) {}

    fn enter_version_pragma(&mut self, _node: &VersionPragma) -> bool {
        true
    }
    fn leave_version_pragma(&mut self, _node: &VersionPragma) {}

    fn enter_version_range(&mut self, _node: &VersionRange) -> bool {
        true
    }
    fn leave_version_range(&mut self, _node: &VersionRange) {}

    fn enter_version_term(&mut self, _node: &VersionTerm) -> bool {
        true
    }
    fn leave_version_term(&mut self, _node: &VersionTerm) {}

    fn enter_while_statement(&mut self, _node: &WhileStatement) -> bool {
        true
    }
    fn leave_while_statement(&mut self, _node: &WhileStatement) {}

    fn enter_yul_block(&mut self, _node: &YulBlock) -> bool {
        true
    }
    fn leave_yul_block(&mut self, _node: &YulBlock) {}

    fn enter_yul_break_statement(&mut self, _node: &YulBreakStatement) -> bool {
        true
    }
    fn leave_yul_break_statement(&mut self, _node: &YulBreakStatement) {}

    fn enter_yul_continue_statement(&mut self, _node: &YulContinueStatement) -> bool {
        true
    }
    fn leave_yul_continue_statement(&mut self, _node: &YulContinueStatement) {}

    fn enter_yul_default_case(&mut self, _node: &YulDefaultCase) -> bool {
        true
    }
    fn leave_yul_default_case(&mut self, _node: &YulDefaultCase) {}

    fn enter_yul_flags_declaration(&mut self, _node: &YulFlagsDeclaration) -> bool {
        true
    }
    fn leave_yul_flags_declaration(&mut self, _node: &YulFlagsDeclaration) {}

    fn enter_yul_for_statement(&mut self, _node: &YulForStatement) -> bool {
        true
    }
    fn leave_yul_for_statement(&mut self, _node: &YulForStatement) {}

    fn enter_yul_function_call_expression(&mut self, _node: &YulFunctionCallExpression) -> bool {
        true
    }
    fn leave_yul_function_call_expression(&mut self, _node: &YulFunctionCallExpression) {}

    fn enter_yul_function_definition(&mut self, _node: &YulFunctionDefinition) -> bool {
        true
    }
    fn leave_yul_function_definition(&mut self, _node: &YulFunctionDefinition) {}

    fn enter_yul_if_statement(&mut self, _node: &YulIfStatement) -> bool {
        true
    }
    fn leave_yul_if_statement(&mut self, _node: &YulIfStatement) {}

    fn enter_yul_leave_statement(&mut self, _node: &YulLeaveStatement) -> bool {
        true
    }
    fn leave_yul_leave_statement(&mut self, _node: &YulLeaveStatement) {}

    fn enter_yul_parameters_declaration(&mut self, _node: &YulParametersDeclaration) -> bool {
        true
    }
    fn leave_yul_parameters_declaration(&mut self, _node: &YulParametersDeclaration) {}

    fn enter_yul_returns_declaration(&mut self, _node: &YulReturnsDeclaration) -> bool {
        true
    }
    fn leave_yul_returns_declaration(&mut self, _node: &YulReturnsDeclaration) {}

    fn enter_yul_switch_statement(&mut self, _node: &YulSwitchStatement) -> bool {
        true
    }
    fn leave_yul_switch_statement(&mut self, _node: &YulSwitchStatement) {}

    fn enter_yul_value_case(&mut self, _node: &YulValueCase) -> bool {
        true
    }
    fn leave_yul_value_case(&mut self, _node: &YulValueCase) {}

    fn enter_yul_variable_assignment_statement(
        &mut self,
        _node: &YulVariableAssignmentStatement,
    ) -> bool {
        true
    }
    fn leave_yul_variable_assignment_statement(&mut self, _node: &YulVariableAssignmentStatement) {}

    fn enter_yul_variable_declaration_statement(
        &mut self,
        _node: &YulVariableDeclarationStatement,
    ) -> bool {
        true
    }
    fn leave_yul_variable_declaration_statement(
        &mut self,
        _node: &YulVariableDeclarationStatement,
    ) {
    }

    fn enter_yul_variable_declaration_value(
        &mut self,
        _node: &YulVariableDeclarationValue,
    ) -> bool {
        true
    }
    fn leave_yul_variable_declaration_value(&mut self, _node: &YulVariableDeclarationValue) {}

    fn enter_abicoder_version(&mut self, _node: &AbicoderVersion) -> bool {
        true
    }
    fn leave_abicoder_version(&mut self, _node: &AbicoderVersion) {}

    fn enter_arguments_declaration(&mut self, _node: &ArgumentsDeclaration) -> bool {
        true
    }
    fn leave_arguments_declaration(&mut self, _node: &ArgumentsDeclaration) {}

    fn enter_constructor_attribute(&mut self, _node: &ConstructorAttribute) -> bool {
        true
    }
    fn leave_constructor_attribute(&mut self, _node: &ConstructorAttribute) {}

    fn enter_contract_member(&mut self, _node: &ContractMember) -> bool {
        true
    }
    fn leave_contract_member(&mut self, _node: &ContractMember) {}

    fn enter_contract_specifier(&mut self, _node: &ContractSpecifier) -> bool {
        true
    }
    fn leave_contract_specifier(&mut self, _node: &ContractSpecifier) {}

    fn enter_elementary_type(&mut self, _node: &ElementaryType) -> bool {
        true
    }
    fn leave_elementary_type(&mut self, _node: &ElementaryType) {}

    fn enter_experimental_feature(&mut self, _node: &ExperimentalFeature) -> bool {
        true
    }
    fn leave_experimental_feature(&mut self, _node: &ExperimentalFeature) {}

    fn enter_expression(&mut self, _node: &Expression) -> bool {
        true
    }
    fn leave_expression(&mut self, _node: &Expression) {}

    fn enter_expression_additive_expression_operator(
        &mut self,
        _node: &Expression_AdditiveExpression_Operator,
    ) -> bool {
        true
    }
    fn leave_expression_additive_expression_operator(
        &mut self,
        _node: &Expression_AdditiveExpression_Operator,
    ) {
    }

    fn enter_expression_assignment_expression_operator(
        &mut self,
        _node: &Expression_AssignmentExpression_Operator,
    ) -> bool {
        true
    }
    fn leave_expression_assignment_expression_operator(
        &mut self,
        _node: &Expression_AssignmentExpression_Operator,
    ) {
    }

    fn enter_expression_equality_expression_operator(
        &mut self,
        _node: &Expression_EqualityExpression_Operator,
    ) -> bool {
        true
    }
    fn leave_expression_equality_expression_operator(
        &mut self,
        _node: &Expression_EqualityExpression_Operator,
    ) {
    }

    fn enter_expression_inequality_expression_operator(
        &mut self,
        _node: &Expression_InequalityExpression_Operator,
    ) -> bool {
        true
    }
    fn leave_expression_inequality_expression_operator(
        &mut self,
        _node: &Expression_InequalityExpression_Operator,
    ) {
    }

    fn enter_expression_multiplicative_expression_operator(
        &mut self,
        _node: &Expression_MultiplicativeExpression_Operator,
    ) -> bool {
        true
    }
    fn leave_expression_multiplicative_expression_operator(
        &mut self,
        _node: &Expression_MultiplicativeExpression_Operator,
    ) {
    }

    fn enter_expression_postfix_expression_operator(
        &mut self,
        _node: &Expression_PostfixExpression_Operator,
    ) -> bool {
        true
    }
    fn leave_expression_postfix_expression_operator(
        &mut self,
        _node: &Expression_PostfixExpression_Operator,
    ) {
    }

    fn enter_expression_prefix_expression_operator(
        &mut self,
        _node: &Expression_PrefixExpression_Operator,
    ) -> bool {
        true
    }
    fn leave_expression_prefix_expression_operator(
        &mut self,
        _node: &Expression_PrefixExpression_Operator,
    ) {
    }

    fn enter_expression_shift_expression_operator(
        &mut self,
        _node: &Expression_ShiftExpression_Operator,
    ) -> bool {
        true
    }
    fn leave_expression_shift_expression_operator(
        &mut self,
        _node: &Expression_ShiftExpression_Operator,
    ) {
    }

    fn enter_fallback_function_attribute(&mut self, _node: &FallbackFunctionAttribute) -> bool {
        true
    }
    fn leave_fallback_function_attribute(&mut self, _node: &FallbackFunctionAttribute) {}

    fn enter_for_statement_condition(&mut self, _node: &ForStatementCondition) -> bool {
        true
    }
    fn leave_for_statement_condition(&mut self, _node: &ForStatementCondition) {}

    fn enter_for_statement_initialization(&mut self, _node: &ForStatementInitialization) -> bool {
        true
    }
    fn leave_for_statement_initialization(&mut self, _node: &ForStatementInitialization) {}

    fn enter_function_attribute(&mut self, _node: &FunctionAttribute) -> bool {
        true
    }
    fn leave_function_attribute(&mut self, _node: &FunctionAttribute) {}

    fn enter_function_body(&mut self, _node: &FunctionBody) -> bool {
        true
    }
    fn leave_function_body(&mut self, _node: &FunctionBody) {}

    fn enter_function_name(&mut self, _node: &FunctionName) -> bool {
        true
    }
    fn leave_function_name(&mut self, _node: &FunctionName) {}

    fn enter_function_type_attribute(&mut self, _node: &FunctionTypeAttribute) -> bool {
        true
    }
    fn leave_function_type_attribute(&mut self, _node: &FunctionTypeAttribute) {}

    fn enter_identifier_path_element(&mut self, _node: &IdentifierPathElement) -> bool {
        true
    }
    fn leave_identifier_path_element(&mut self, _node: &IdentifierPathElement) {}

    fn enter_import_clause(&mut self, _node: &ImportClause) -> bool {
        true
    }
    fn leave_import_clause(&mut self, _node: &ImportClause) {}

    fn enter_mapping_key_type(&mut self, _node: &MappingKeyType) -> bool {
        true
    }
    fn leave_mapping_key_type(&mut self, _node: &MappingKeyType) {}

    fn enter_modifier_attribute(&mut self, _node: &ModifierAttribute) -> bool {
        true
    }
    fn leave_modifier_attribute(&mut self, _node: &ModifierAttribute) {}

    fn enter_number_unit(&mut self, _node: &NumberUnit) -> bool {
        true
    }
    fn leave_number_unit(&mut self, _node: &NumberUnit) {}

    fn enter_pragma(&mut self, _node: &Pragma) -> bool {
        true
    }
    fn leave_pragma(&mut self, _node: &Pragma) {}

    fn enter_receive_function_attribute(&mut self, _node: &ReceiveFunctionAttribute) -> bool {
        true
    }
    fn leave_receive_function_attribute(&mut self, _node: &ReceiveFunctionAttribute) {}

    fn enter_source_unit_member(&mut self, _node: &SourceUnitMember) -> bool {
        true
    }
    fn leave_source_unit_member(&mut self, _node: &SourceUnitMember) {}

    fn enter_state_variable_attribute(&mut self, _node: &StateVariableAttribute) -> bool {
        true
    }
    fn leave_state_variable_attribute(&mut self, _node: &StateVariableAttribute) {}

    fn enter_statement(&mut self, _node: &Statement) -> bool {
        true
    }
    fn leave_statement(&mut self, _node: &Statement) {}

    fn enter_storage_location(&mut self, _node: &StorageLocation) -> bool {
        true
    }
    fn leave_storage_location(&mut self, _node: &StorageLocation) {}

    fn enter_string_expression(&mut self, _node: &StringExpression) -> bool {
        true
    }
    fn leave_string_expression(&mut self, _node: &StringExpression) {}

    fn enter_type_name(&mut self, _node: &TypeName) -> bool {
        true
    }
    fn leave_type_name(&mut self, _node: &TypeName) {}

    fn enter_using_clause(&mut self, _node: &UsingClause) -> bool {
        true
    }
    fn leave_using_clause(&mut self, _node: &UsingClause) {}

    fn enter_using_operator(&mut self, _node: &UsingOperator) -> bool {
        true
    }
    fn leave_using_operator(&mut self, _node: &UsingOperator) {}

    fn enter_using_target(&mut self, _node: &UsingTarget) -> bool {
        true
    }
    fn leave_using_target(&mut self, _node: &UsingTarget) {}

    fn enter_variable_declaration_target(&mut self, _node: &VariableDeclarationTarget) -> bool {
        true
    }
    fn leave_variable_declaration_target(&mut self, _node: &VariableDeclarationTarget) {}

    fn enter_version_expression(&mut self, _node: &VersionExpression) -> bool {
        true
    }
    fn leave_version_expression(&mut self, _node: &VersionExpression) {}

    fn enter_version_literal(&mut self, _node: &VersionLiteral) -> bool {
        true
    }
    fn leave_version_literal(&mut self, _node: &VersionLiteral) {}

    fn enter_version_operator(&mut self, _node: &VersionOperator) -> bool {
        true
    }
    fn leave_version_operator(&mut self, _node: &VersionOperator) {}

    fn enter_yul_expression(&mut self, _node: &YulExpression) -> bool {
        true
    }
    fn leave_yul_expression(&mut self, _node: &YulExpression) {}

    fn enter_yul_literal(&mut self, _node: &YulLiteral) -> bool {
        true
    }
    fn leave_yul_literal(&mut self, _node: &YulLiteral) {}

    fn enter_yul_statement(&mut self, _node: &YulStatement) -> bool {
        true
    }
    fn leave_yul_statement(&mut self, _node: &YulStatement) {}

    fn enter_yul_switch_case(&mut self, _node: &YulSwitchCase) -> bool {
        true
    }
    fn leave_yul_switch_case(&mut self, _node: &YulSwitchCase) {}

    fn enter_array_values(&mut self, _node: &ArrayValues) -> bool {
        true
    }
    fn leave_array_values(&mut self, _node: &ArrayValues) {}

    fn enter_call_options(&mut self, _node: &CallOptions) -> bool {
        true
    }
    fn leave_call_options(&mut self, _node: &CallOptions) {}

    fn enter_catch_clauses(&mut self, _node: &CatchClauses) -> bool {
        true
    }
    fn leave_catch_clauses(&mut self, _node: &CatchClauses) {}

    fn enter_constructor_attributes(&mut self, _node: &ConstructorAttributes) -> bool {
        true
    }
    fn leave_constructor_attributes(&mut self, _node: &ConstructorAttributes) {}

    fn enter_contract_members(&mut self, _node: &ContractMembers) -> bool {
        true
    }
    fn leave_contract_members(&mut self, _node: &ContractMembers) {}

    fn enter_contract_specifiers(&mut self, _node: &ContractSpecifiers) -> bool {
        true
    }
    fn leave_contract_specifiers(&mut self, _node: &ContractSpecifiers) {}

    fn enter_enum_members(&mut self, _node: &EnumMembers) -> bool {
        true
    }
    fn leave_enum_members(&mut self, _node: &EnumMembers) {}

    fn enter_error_parameters(&mut self, _node: &ErrorParameters) -> bool {
        true
    }
    fn leave_error_parameters(&mut self, _node: &ErrorParameters) {}

    fn enter_event_parameters(&mut self, _node: &EventParameters) -> bool {
        true
    }
    fn leave_event_parameters(&mut self, _node: &EventParameters) {}

    fn enter_fallback_function_attributes(&mut self, _node: &FallbackFunctionAttributes) -> bool {
        true
    }
    fn leave_fallback_function_attributes(&mut self, _node: &FallbackFunctionAttributes) {}

    fn enter_function_attributes(&mut self, _node: &FunctionAttributes) -> bool {
        true
    }
    fn leave_function_attributes(&mut self, _node: &FunctionAttributes) {}

    fn enter_function_type_attributes(&mut self, _node: &FunctionTypeAttributes) -> bool {
        true
    }
    fn leave_function_type_attributes(&mut self, _node: &FunctionTypeAttributes) {}

    fn enter_hex_string_literals(&mut self, _node: &HexStringLiterals) -> bool {
        true
    }
    fn leave_hex_string_literals(&mut self, _node: &HexStringLiterals) {}

    fn enter_identifier_path(&mut self, _node: &IdentifierPath) -> bool {
        true
    }
    fn leave_identifier_path(&mut self, _node: &IdentifierPath) {}

    fn enter_import_deconstruction_symbols(&mut self, _node: &ImportDeconstructionSymbols) -> bool {
        true
    }
    fn leave_import_deconstruction_symbols(&mut self, _node: &ImportDeconstructionSymbols) {}

    fn enter_inheritance_types(&mut self, _node: &InheritanceTypes) -> bool {
        true
    }
    fn leave_inheritance_types(&mut self, _node: &InheritanceTypes) {}

    fn enter_interface_members(&mut self, _node: &InterfaceMembers) -> bool {
        true
    }
    fn leave_interface_members(&mut self, _node: &InterfaceMembers) {}

    fn enter_library_members(&mut self, _node: &LibraryMembers) -> bool {
        true
    }
    fn leave_library_members(&mut self, _node: &LibraryMembers) {}

    fn enter_modifier_attributes(&mut self, _node: &ModifierAttributes) -> bool {
        true
    }
    fn leave_modifier_attributes(&mut self, _node: &ModifierAttributes) {}

    fn enter_multi_typed_declaration_elements(
        &mut self,
        _node: &MultiTypedDeclarationElements,
    ) -> bool {
        true
    }
    fn leave_multi_typed_declaration_elements(&mut self, _node: &MultiTypedDeclarationElements) {}

    fn enter_named_arguments(&mut self, _node: &NamedArguments) -> bool {
        true
    }
    fn leave_named_arguments(&mut self, _node: &NamedArguments) {}

    fn enter_override_paths(&mut self, _node: &OverridePaths) -> bool {
        true
    }
    fn leave_override_paths(&mut self, _node: &OverridePaths) {}

    fn enter_parameters(&mut self, _node: &Parameters) -> bool {
        true
    }
    fn leave_parameters(&mut self, _node: &Parameters) {}

    fn enter_positional_arguments(&mut self, _node: &PositionalArguments) -> bool {
        true
    }
    fn leave_positional_arguments(&mut self, _node: &PositionalArguments) {}

    fn enter_receive_function_attributes(&mut self, _node: &ReceiveFunctionAttributes) -> bool {
        true
    }
    fn leave_receive_function_attributes(&mut self, _node: &ReceiveFunctionAttributes) {}

    fn enter_simple_version_literal(&mut self, _node: &SimpleVersionLiteral) -> bool {
        true
    }
    fn leave_simple_version_literal(&mut self, _node: &SimpleVersionLiteral) {}

    fn enter_source_unit_members(&mut self, _node: &SourceUnitMembers) -> bool {
        true
    }
    fn leave_source_unit_members(&mut self, _node: &SourceUnitMembers) {}

    fn enter_state_variable_attributes(&mut self, _node: &StateVariableAttributes) -> bool {
        true
    }
    fn leave_state_variable_attributes(&mut self, _node: &StateVariableAttributes) {}

    fn enter_statements(&mut self, _node: &Statements) -> bool {
        true
    }
    fn leave_statements(&mut self, _node: &Statements) {}

    fn enter_string_literals(&mut self, _node: &StringLiterals) -> bool {
        true
    }
    fn leave_string_literals(&mut self, _node: &StringLiterals) {}

    fn enter_struct_members(&mut self, _node: &StructMembers) -> bool {
        true
    }
    fn leave_struct_members(&mut self, _node: &StructMembers) {}

    fn enter_tuple_values(&mut self, _node: &TupleValues) -> bool {
        true
    }
    fn leave_tuple_values(&mut self, _node: &TupleValues) {}

    fn enter_unicode_string_literals(&mut self, _node: &UnicodeStringLiterals) -> bool {
        true
    }
    fn leave_unicode_string_literals(&mut self, _node: &UnicodeStringLiterals) {}

    fn enter_using_deconstruction_symbols(&mut self, _node: &UsingDeconstructionSymbols) -> bool {
        true
    }
    fn leave_using_deconstruction_symbols(&mut self, _node: &UsingDeconstructionSymbols) {}

    fn enter_version_expression_set(&mut self, _node: &VersionExpressionSet) -> bool {
        true
    }
    fn leave_version_expression_set(&mut self, _node: &VersionExpressionSet) {}

    fn enter_version_expression_sets(&mut self, _node: &VersionExpressionSets) -> bool {
        true
    }
    fn leave_version_expression_sets(&mut self, _node: &VersionExpressionSets) {}

    fn enter_yul_arguments(&mut self, _node: &YulArguments) -> bool {
        true
    }
    fn leave_yul_arguments(&mut self, _node: &YulArguments) {}

    fn enter_yul_flags(&mut self, _node: &YulFlags) -> bool {
        true
    }
    fn leave_yul_flags(&mut self, _node: &YulFlags) {}

    fn enter_yul_parameters(&mut self, _node: &YulParameters) -> bool {
        true
    }
    fn leave_yul_parameters(&mut self, _node: &YulParameters) {}

    fn enter_yul_path(&mut self, _node: &YulPath) -> bool {
        true
    }
    fn leave_yul_path(&mut self, _node: &YulPath) {}

    fn enter_yul_paths(&mut self, _node: &YulPaths) -> bool {
        true
    }
    fn leave_yul_paths(&mut self, _node: &YulPaths) {}

    fn enter_yul_statements(&mut self, _node: &YulStatements) -> bool {
        true
    }
    fn leave_yul_statements(&mut self, _node: &YulStatements) {}

    fn enter_yul_switch_cases(&mut self, _node: &YulSwitchCases) -> bool {
        true
    }
    fn leave_yul_switch_cases(&mut self, _node: &YulSwitchCases) {}

    fn enter_yul_variable_names(&mut self, _node: &YulVariableNames) -> bool {
        true
    }
    fn leave_yul_variable_names(&mut self, _node: &YulVariableNames) {}
}

//
// Sequences
//

pub fn accept_abicoder_pragma(node: &AbicoderPragma, visitor: &mut impl Visitor) {
    if !visitor.enter_abicoder_pragma(node) {
        return;
    }
    accept_abicoder_version(&node.version, visitor);
    visitor.leave_abicoder_pragma(node);
}

pub fn accept_additive_expression(node: &AdditiveExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_additive_expression(node) {
        return;
    }
    accept_expression(&node.left_operand, visitor);
    accept_expression_additive_expression_operator(
        &node.expression_additive_expression_operator,
        visitor,
    );
    accept_expression(&node.right_operand, visitor);
    visitor.leave_additive_expression(node);
}

pub fn accept_address_type(node: &AddressType, visitor: &mut impl Visitor) {
    if !visitor.enter_address_type(node) {
        return;
    }

    visitor.leave_address_type(node);
}

pub fn accept_and_expression(node: &AndExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_and_expression(node) {
        return;
    }
    accept_expression(&node.left_operand, visitor);
    accept_expression(&node.right_operand, visitor);
    visitor.leave_and_expression(node);
}

pub fn accept_array_expression(node: &ArrayExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_array_expression(node) {
        return;
    }
    accept_array_values(&node.items, visitor);
    visitor.leave_array_expression(node);
}

pub fn accept_array_type_name(node: &ArrayTypeName, visitor: &mut impl Visitor) {
    if !visitor.enter_array_type_name(node) {
        return;
    }
    accept_type_name(&node.operand, visitor);
    if let Some(ref index) = node.index {
        accept_expression(index, visitor);
    }

    visitor.leave_array_type_name(node);
}

pub fn accept_assembly_statement(node: &AssemblyStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_assembly_statement(node) {
        return;
    }
    if let Some(ref flags) = node.flags {
        accept_yul_flags_declaration(flags, visitor);
    }
    accept_yul_block(&node.body, visitor);
    visitor.leave_assembly_statement(node);
}

pub fn accept_assignment_expression(node: &AssignmentExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_assignment_expression(node) {
        return;
    }
    accept_expression(&node.left_operand, visitor);
    accept_expression_assignment_expression_operator(
        &node.expression_assignment_expression_operator,
        visitor,
    );
    accept_expression(&node.right_operand, visitor);
    visitor.leave_assignment_expression(node);
}

pub fn accept_bitwise_and_expression(node: &BitwiseAndExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_bitwise_and_expression(node) {
        return;
    }
    accept_expression(&node.left_operand, visitor);
    accept_expression(&node.right_operand, visitor);
    visitor.leave_bitwise_and_expression(node);
}

pub fn accept_bitwise_or_expression(node: &BitwiseOrExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_bitwise_or_expression(node) {
        return;
    }
    accept_expression(&node.left_operand, visitor);
    accept_expression(&node.right_operand, visitor);
    visitor.leave_bitwise_or_expression(node);
}

pub fn accept_bitwise_xor_expression(node: &BitwiseXorExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_bitwise_xor_expression(node) {
        return;
    }
    accept_expression(&node.left_operand, visitor);
    accept_expression(&node.right_operand, visitor);
    visitor.leave_bitwise_xor_expression(node);
}

pub fn accept_block(node: &Block, visitor: &mut impl Visitor) {
    if !visitor.enter_block(node) {
        return;
    }
    accept_statements(&node.statements, visitor);
    visitor.leave_block(node);
}

pub fn accept_break_statement(node: &BreakStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_break_statement(node) {
        return;
    }

    visitor.leave_break_statement(node);
}

pub fn accept_call_options_expression(node: &CallOptionsExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_call_options_expression(node) {
        return;
    }
    accept_expression(&node.operand, visitor);
    accept_call_options(&node.options, visitor);
    visitor.leave_call_options_expression(node);
}

pub fn accept_catch_clause(node: &CatchClause, visitor: &mut impl Visitor) {
    if !visitor.enter_catch_clause(node) {
        return;
    }
    if let Some(ref error) = node.error {
        accept_catch_clause_error(error, visitor);
    }
    accept_block(&node.body, visitor);
    visitor.leave_catch_clause(node);
}

pub fn accept_catch_clause_error(node: &CatchClauseError, visitor: &mut impl Visitor) {
    if !visitor.enter_catch_clause_error(node) {
        return;
    }
    accept_parameters_declaration(&node.parameters, visitor);
    visitor.leave_catch_clause_error(node);
}

pub fn accept_conditional_expression(node: &ConditionalExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_conditional_expression(node) {
        return;
    }
    accept_expression(&node.operand, visitor);
    accept_expression(&node.true_expression, visitor);
    accept_expression(&node.false_expression, visitor);
    visitor.leave_conditional_expression(node);
}

pub fn accept_constant_definition(node: &ConstantDefinition, visitor: &mut impl Visitor) {
    if !visitor.enter_constant_definition(node) {
        return;
    }
    accept_type_name(&node.type_name, visitor);
    accept_expression(&node.value, visitor);
    visitor.leave_constant_definition(node);
}

pub fn accept_constructor_definition(node: &ConstructorDefinition, visitor: &mut impl Visitor) {
    if !visitor.enter_constructor_definition(node) {
        return;
    }
    accept_parameters_declaration(&node.parameters, visitor);
    accept_constructor_attributes(&node.attributes, visitor);
    accept_block(&node.body, visitor);
    visitor.leave_constructor_definition(node);
}

pub fn accept_continue_statement(node: &ContinueStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_continue_statement(node) {
        return;
    }

    visitor.leave_continue_statement(node);
}

pub fn accept_contract_definition(node: &ContractDefinition, visitor: &mut impl Visitor) {
    if !visitor.enter_contract_definition(node) {
        return;
    }
    accept_contract_specifiers(&node.specifiers, visitor);
    accept_contract_members(&node.members, visitor);
    visitor.leave_contract_definition(node);
}

pub fn accept_decimal_number_expression(
    node: &DecimalNumberExpression,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_decimal_number_expression(node) {
        return;
    }
    if let Some(ref unit) = node.unit {
        accept_number_unit(unit, visitor);
    }

    visitor.leave_decimal_number_expression(node);
}

pub fn accept_do_while_statement(node: &DoWhileStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_do_while_statement(node) {
        return;
    }
    accept_statement(&node.body, visitor);
    accept_expression(&node.condition, visitor);
    visitor.leave_do_while_statement(node);
}

pub fn accept_else_branch(node: &ElseBranch, visitor: &mut impl Visitor) {
    if !visitor.enter_else_branch(node) {
        return;
    }
    accept_statement(&node.body, visitor);
    visitor.leave_else_branch(node);
}

pub fn accept_emit_statement(node: &EmitStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_emit_statement(node) {
        return;
    }
    accept_identifier_path(&node.event, visitor);
    accept_arguments_declaration(&node.arguments, visitor);
    visitor.leave_emit_statement(node);
}

pub fn accept_enum_definition(node: &EnumDefinition, visitor: &mut impl Visitor) {
    if !visitor.enter_enum_definition(node) {
        return;
    }
    accept_enum_members(&node.members, visitor);
    visitor.leave_enum_definition(node);
}

pub fn accept_equality_expression(node: &EqualityExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_equality_expression(node) {
        return;
    }
    accept_expression(&node.left_operand, visitor);
    accept_expression_equality_expression_operator(
        &node.expression_equality_expression_operator,
        visitor,
    );
    accept_expression(&node.right_operand, visitor);
    visitor.leave_equality_expression(node);
}

pub fn accept_error_definition(node: &ErrorDefinition, visitor: &mut impl Visitor) {
    if !visitor.enter_error_definition(node) {
        return;
    }
    accept_error_parameters_declaration(&node.members, visitor);
    visitor.leave_error_definition(node);
}

pub fn accept_error_parameter(node: &ErrorParameter, visitor: &mut impl Visitor) {
    if !visitor.enter_error_parameter(node) {
        return;
    }
    accept_type_name(&node.type_name, visitor);
    visitor.leave_error_parameter(node);
}

pub fn accept_error_parameters_declaration(
    node: &ErrorParametersDeclaration,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_error_parameters_declaration(node) {
        return;
    }
    accept_error_parameters(&node.parameters, visitor);
    visitor.leave_error_parameters_declaration(node);
}

pub fn accept_event_definition(node: &EventDefinition, visitor: &mut impl Visitor) {
    if !visitor.enter_event_definition(node) {
        return;
    }
    accept_event_parameters_declaration(&node.parameters, visitor);
    visitor.leave_event_definition(node);
}

pub fn accept_event_parameter(node: &EventParameter, visitor: &mut impl Visitor) {
    if !visitor.enter_event_parameter(node) {
        return;
    }
    accept_type_name(&node.type_name, visitor);
    visitor.leave_event_parameter(node);
}

pub fn accept_event_parameters_declaration(
    node: &EventParametersDeclaration,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_event_parameters_declaration(node) {
        return;
    }
    accept_event_parameters(&node.parameters, visitor);
    visitor.leave_event_parameters_declaration(node);
}

pub fn accept_experimental_pragma(node: &ExperimentalPragma, visitor: &mut impl Visitor) {
    if !visitor.enter_experimental_pragma(node) {
        return;
    }
    accept_experimental_feature(&node.feature, visitor);
    visitor.leave_experimental_pragma(node);
}

pub fn accept_exponentiation_expression(
    node: &ExponentiationExpression,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_exponentiation_expression(node) {
        return;
    }
    accept_expression(&node.left_operand, visitor);
    accept_expression(&node.right_operand, visitor);
    visitor.leave_exponentiation_expression(node);
}

pub fn accept_expression_statement(node: &ExpressionStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_expression_statement(node) {
        return;
    }
    accept_expression(&node.expression, visitor);
    visitor.leave_expression_statement(node);
}

pub fn accept_fallback_function_definition(
    node: &FallbackFunctionDefinition,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_fallback_function_definition(node) {
        return;
    }
    accept_parameters_declaration(&node.parameters, visitor);
    accept_fallback_function_attributes(&node.attributes, visitor);
    if let Some(ref returns) = node.returns {
        accept_returns_declaration(returns, visitor);
    }
    accept_function_body(&node.body, visitor);
    visitor.leave_fallback_function_definition(node);
}

pub fn accept_for_statement(node: &ForStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_for_statement(node) {
        return;
    }
    accept_for_statement_initialization(&node.initialization, visitor);
    accept_for_statement_condition(&node.condition, visitor);
    if let Some(ref iterator) = node.iterator {
        accept_expression(iterator, visitor);
    }
    accept_statement(&node.body, visitor);
    visitor.leave_for_statement(node);
}

pub fn accept_function_call_expression(node: &FunctionCallExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_function_call_expression(node) {
        return;
    }
    accept_expression(&node.operand, visitor);
    accept_arguments_declaration(&node.arguments, visitor);
    visitor.leave_function_call_expression(node);
}

pub fn accept_function_definition(node: &FunctionDefinition, visitor: &mut impl Visitor) {
    if !visitor.enter_function_definition(node) {
        return;
    }
    accept_function_name(&node.name, visitor);
    accept_parameters_declaration(&node.parameters, visitor);
    accept_function_attributes(&node.attributes, visitor);
    if let Some(ref returns) = node.returns {
        accept_returns_declaration(returns, visitor);
    }
    accept_function_body(&node.body, visitor);
    visitor.leave_function_definition(node);
}

pub fn accept_function_type(node: &FunctionType, visitor: &mut impl Visitor) {
    if !visitor.enter_function_type(node) {
        return;
    }
    accept_parameters_declaration(&node.parameters, visitor);
    accept_function_type_attributes(&node.attributes, visitor);
    if let Some(ref returns) = node.returns {
        accept_returns_declaration(returns, visitor);
    }

    visitor.leave_function_type(node);
}

pub fn accept_hex_number_expression(node: &HexNumberExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_hex_number_expression(node) {
        return;
    }

    visitor.leave_hex_number_expression(node);
}

pub fn accept_if_statement(node: &IfStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_if_statement(node) {
        return;
    }
    accept_expression(&node.condition, visitor);
    accept_statement(&node.body, visitor);
    if let Some(ref else_branch) = node.else_branch {
        accept_else_branch(else_branch, visitor);
    }

    visitor.leave_if_statement(node);
}

pub fn accept_import_alias(node: &ImportAlias, visitor: &mut impl Visitor) {
    if !visitor.enter_import_alias(node) {
        return;
    }

    visitor.leave_import_alias(node);
}

pub fn accept_import_deconstruction(node: &ImportDeconstruction, visitor: &mut impl Visitor) {
    if !visitor.enter_import_deconstruction(node) {
        return;
    }
    accept_import_deconstruction_symbols(&node.symbols, visitor);
    visitor.leave_import_deconstruction(node);
}

pub fn accept_import_deconstruction_symbol(
    node: &ImportDeconstructionSymbol,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_import_deconstruction_symbol(node) {
        return;
    }
    if let Some(ref alias) = node.alias {
        accept_import_alias(alias, visitor);
    }

    visitor.leave_import_deconstruction_symbol(node);
}

pub fn accept_import_directive(node: &ImportDirective, visitor: &mut impl Visitor) {
    if !visitor.enter_import_directive(node) {
        return;
    }
    accept_import_clause(&node.clause, visitor);
    visitor.leave_import_directive(node);
}

pub fn accept_index_access_end(node: &IndexAccessEnd, visitor: &mut impl Visitor) {
    if !visitor.enter_index_access_end(node) {
        return;
    }
    if let Some(ref end) = node.end {
        accept_expression(end, visitor);
    }

    visitor.leave_index_access_end(node);
}

pub fn accept_index_access_expression(node: &IndexAccessExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_index_access_expression(node) {
        return;
    }
    accept_expression(&node.operand, visitor);
    if let Some(ref start) = node.start {
        accept_expression(start, visitor);
    }
    if let Some(ref end) = node.end {
        accept_index_access_end(end, visitor);
    }

    visitor.leave_index_access_expression(node);
}

pub fn accept_inequality_expression(node: &InequalityExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_inequality_expression(node) {
        return;
    }
    accept_expression(&node.left_operand, visitor);
    accept_expression_inequality_expression_operator(
        &node.expression_inequality_expression_operator,
        visitor,
    );
    accept_expression(&node.right_operand, visitor);
    visitor.leave_inequality_expression(node);
}

pub fn accept_inheritance_specifier(node: &InheritanceSpecifier, visitor: &mut impl Visitor) {
    if !visitor.enter_inheritance_specifier(node) {
        return;
    }
    accept_inheritance_types(&node.types, visitor);
    visitor.leave_inheritance_specifier(node);
}

pub fn accept_inheritance_type(node: &InheritanceType, visitor: &mut impl Visitor) {
    if !visitor.enter_inheritance_type(node) {
        return;
    }
    accept_identifier_path(&node.type_name, visitor);
    if let Some(ref arguments) = node.arguments {
        accept_arguments_declaration(arguments, visitor);
    }

    visitor.leave_inheritance_type(node);
}

pub fn accept_interface_definition(node: &InterfaceDefinition, visitor: &mut impl Visitor) {
    if !visitor.enter_interface_definition(node) {
        return;
    }
    if let Some(ref inheritance) = node.inheritance {
        accept_inheritance_specifier(inheritance, visitor);
    }
    accept_interface_members(&node.members, visitor);
    visitor.leave_interface_definition(node);
}

pub fn accept_library_definition(node: &LibraryDefinition, visitor: &mut impl Visitor) {
    if !visitor.enter_library_definition(node) {
        return;
    }
    accept_library_members(&node.members, visitor);
    visitor.leave_library_definition(node);
}

pub fn accept_mapping_key(node: &MappingKey, visitor: &mut impl Visitor) {
    if !visitor.enter_mapping_key(node) {
        return;
    }
    accept_mapping_key_type(&node.key_type, visitor);
    visitor.leave_mapping_key(node);
}

pub fn accept_mapping_type(node: &MappingType, visitor: &mut impl Visitor) {
    if !visitor.enter_mapping_type(node) {
        return;
    }
    accept_mapping_key(&node.key_type, visitor);
    accept_mapping_value(&node.value_type, visitor);
    visitor.leave_mapping_type(node);
}

pub fn accept_mapping_value(node: &MappingValue, visitor: &mut impl Visitor) {
    if !visitor.enter_mapping_value(node) {
        return;
    }
    accept_type_name(&node.type_name, visitor);
    visitor.leave_mapping_value(node);
}

pub fn accept_member_access_expression(node: &MemberAccessExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_member_access_expression(node) {
        return;
    }
    accept_expression(&node.operand, visitor);
    accept_identifier_path_element(&node.member, visitor);
    visitor.leave_member_access_expression(node);
}

pub fn accept_modifier_definition(node: &ModifierDefinition, visitor: &mut impl Visitor) {
    if !visitor.enter_modifier_definition(node) {
        return;
    }
    if let Some(ref parameters) = node.parameters {
        accept_parameters_declaration(parameters, visitor);
    }
    accept_modifier_attributes(&node.attributes, visitor);
    accept_function_body(&node.body, visitor);
    visitor.leave_modifier_definition(node);
}

pub fn accept_modifier_invocation(node: &ModifierInvocation, visitor: &mut impl Visitor) {
    if !visitor.enter_modifier_invocation(node) {
        return;
    }
    accept_identifier_path(&node.name, visitor);
    if let Some(ref arguments) = node.arguments {
        accept_arguments_declaration(arguments, visitor);
    }

    visitor.leave_modifier_invocation(node);
}

pub fn accept_multi_typed_declaration(node: &MultiTypedDeclaration, visitor: &mut impl Visitor) {
    if !visitor.enter_multi_typed_declaration(node) {
        return;
    }
    accept_multi_typed_declaration_elements(&node.elements, visitor);
    accept_variable_declaration_value(&node.value, visitor);
    visitor.leave_multi_typed_declaration(node);
}

pub fn accept_multi_typed_declaration_element(
    node: &MultiTypedDeclarationElement,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_multi_typed_declaration_element(node) {
        return;
    }
    if let Some(ref member) = node.member {
        accept_variable_declaration(member, visitor);
    }

    visitor.leave_multi_typed_declaration_element(node);
}

pub fn accept_multiplicative_expression(
    node: &MultiplicativeExpression,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_multiplicative_expression(node) {
        return;
    }
    accept_expression(&node.left_operand, visitor);
    accept_expression_multiplicative_expression_operator(
        &node.expression_multiplicative_expression_operator,
        visitor,
    );
    accept_expression(&node.right_operand, visitor);
    visitor.leave_multiplicative_expression(node);
}

pub fn accept_named_argument(node: &NamedArgument, visitor: &mut impl Visitor) {
    if !visitor.enter_named_argument(node) {
        return;
    }
    accept_expression(&node.value, visitor);
    visitor.leave_named_argument(node);
}

pub fn accept_named_argument_group(node: &NamedArgumentGroup, visitor: &mut impl Visitor) {
    if !visitor.enter_named_argument_group(node) {
        return;
    }
    accept_named_arguments(&node.arguments, visitor);
    visitor.leave_named_argument_group(node);
}

pub fn accept_named_arguments_declaration(
    node: &NamedArgumentsDeclaration,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_named_arguments_declaration(node) {
        return;
    }
    accept_named_argument_group(&node.arguments, visitor);
    visitor.leave_named_arguments_declaration(node);
}

pub fn accept_named_import(node: &NamedImport, visitor: &mut impl Visitor) {
    if !visitor.enter_named_import(node) {
        return;
    }
    accept_import_alias(&node.alias, visitor);
    visitor.leave_named_import(node);
}

pub fn accept_new_expression(node: &NewExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_new_expression(node) {
        return;
    }
    accept_type_name(&node.type_name, visitor);
    visitor.leave_new_expression(node);
}

pub fn accept_or_expression(node: &OrExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_or_expression(node) {
        return;
    }
    accept_expression(&node.left_operand, visitor);
    accept_expression(&node.right_operand, visitor);
    visitor.leave_or_expression(node);
}

pub fn accept_override_paths_declaration(
    node: &OverridePathsDeclaration,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_override_paths_declaration(node) {
        return;
    }
    accept_override_paths(&node.paths, visitor);
    visitor.leave_override_paths_declaration(node);
}

pub fn accept_override_specifier(node: &OverrideSpecifier, visitor: &mut impl Visitor) {
    if !visitor.enter_override_specifier(node) {
        return;
    }
    if let Some(ref overridden) = node.overridden {
        accept_override_paths_declaration(overridden, visitor);
    }

    visitor.leave_override_specifier(node);
}

pub fn accept_parameter(node: &Parameter, visitor: &mut impl Visitor) {
    if !visitor.enter_parameter(node) {
        return;
    }
    accept_type_name(&node.type_name, visitor);
    if let Some(ref storage_location) = node.storage_location {
        accept_storage_location(storage_location, visitor);
    }

    visitor.leave_parameter(node);
}

pub fn accept_parameters_declaration(node: &ParametersDeclaration, visitor: &mut impl Visitor) {
    if !visitor.enter_parameters_declaration(node) {
        return;
    }
    accept_parameters(&node.parameters, visitor);
    visitor.leave_parameters_declaration(node);
}

pub fn accept_path_import(node: &PathImport, visitor: &mut impl Visitor) {
    if !visitor.enter_path_import(node) {
        return;
    }
    if let Some(ref alias) = node.alias {
        accept_import_alias(alias, visitor);
    }

    visitor.leave_path_import(node);
}

pub fn accept_positional_arguments_declaration(
    node: &PositionalArgumentsDeclaration,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_positional_arguments_declaration(node) {
        return;
    }
    accept_positional_arguments(&node.arguments, visitor);
    visitor.leave_positional_arguments_declaration(node);
}

pub fn accept_postfix_expression(node: &PostfixExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_postfix_expression(node) {
        return;
    }
    accept_expression(&node.operand, visitor);
    accept_expression_postfix_expression_operator(
        &node.expression_postfix_expression_operator,
        visitor,
    );
    visitor.leave_postfix_expression(node);
}

pub fn accept_pragma_directive(node: &PragmaDirective, visitor: &mut impl Visitor) {
    if !visitor.enter_pragma_directive(node) {
        return;
    }
    accept_pragma(&node.pragma, visitor);
    visitor.leave_pragma_directive(node);
}

pub fn accept_prefix_expression(node: &PrefixExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_prefix_expression(node) {
        return;
    }
    accept_expression_prefix_expression_operator(
        &node.expression_prefix_expression_operator,
        visitor,
    );
    accept_expression(&node.operand, visitor);
    visitor.leave_prefix_expression(node);
}

pub fn accept_receive_function_definition(
    node: &ReceiveFunctionDefinition,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_receive_function_definition(node) {
        return;
    }
    accept_parameters_declaration(&node.parameters, visitor);
    accept_receive_function_attributes(&node.attributes, visitor);
    accept_function_body(&node.body, visitor);
    visitor.leave_receive_function_definition(node);
}

pub fn accept_return_statement(node: &ReturnStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_return_statement(node) {
        return;
    }
    if let Some(ref expression) = node.expression {
        accept_expression(expression, visitor);
    }

    visitor.leave_return_statement(node);
}

pub fn accept_returns_declaration(node: &ReturnsDeclaration, visitor: &mut impl Visitor) {
    if !visitor.enter_returns_declaration(node) {
        return;
    }
    accept_parameters_declaration(&node.variables, visitor);
    visitor.leave_returns_declaration(node);
}

pub fn accept_revert_statement(node: &RevertStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_revert_statement(node) {
        return;
    }
    accept_identifier_path(&node.error, visitor);
    accept_arguments_declaration(&node.arguments, visitor);
    visitor.leave_revert_statement(node);
}

pub fn accept_shift_expression(node: &ShiftExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_shift_expression(node) {
        return;
    }
    accept_expression(&node.left_operand, visitor);
    accept_expression_shift_expression_operator(
        &node.expression_shift_expression_operator,
        visitor,
    );
    accept_expression(&node.right_operand, visitor);
    visitor.leave_shift_expression(node);
}

pub fn accept_single_typed_declaration(node: &SingleTypedDeclaration, visitor: &mut impl Visitor) {
    if !visitor.enter_single_typed_declaration(node) {
        return;
    }
    accept_variable_declaration(&node.declaration, visitor);
    if let Some(ref value) = node.value {
        accept_variable_declaration_value(value, visitor);
    }

    visitor.leave_single_typed_declaration(node);
}

pub fn accept_source_unit(node: &SourceUnit, visitor: &mut impl Visitor) {
    if !visitor.enter_source_unit(node) {
        return;
    }
    accept_source_unit_members(&node.members, visitor);
    visitor.leave_source_unit(node);
}

pub fn accept_state_variable_definition(
    node: &StateVariableDefinition,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_state_variable_definition(node) {
        return;
    }
    accept_type_name(&node.type_name, visitor);
    accept_state_variable_attributes(&node.attributes, visitor);
    if let Some(ref value) = node.value {
        accept_state_variable_definition_value(value, visitor);
    }

    visitor.leave_state_variable_definition(node);
}

pub fn accept_state_variable_definition_value(
    node: &StateVariableDefinitionValue,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_state_variable_definition_value(node) {
        return;
    }
    accept_expression(&node.value, visitor);
    visitor.leave_state_variable_definition_value(node);
}

pub fn accept_storage_layout_specifier(node: &StorageLayoutSpecifier, visitor: &mut impl Visitor) {
    if !visitor.enter_storage_layout_specifier(node) {
        return;
    }
    accept_expression(&node.expression, visitor);
    visitor.leave_storage_layout_specifier(node);
}

pub fn accept_struct_definition(node: &StructDefinition, visitor: &mut impl Visitor) {
    if !visitor.enter_struct_definition(node) {
        return;
    }
    accept_struct_members(&node.members, visitor);
    visitor.leave_struct_definition(node);
}

pub fn accept_struct_member(node: &StructMember, visitor: &mut impl Visitor) {
    if !visitor.enter_struct_member(node) {
        return;
    }
    accept_type_name(&node.type_name, visitor);
    visitor.leave_struct_member(node);
}

pub fn accept_try_statement(node: &TryStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_try_statement(node) {
        return;
    }
    accept_expression(&node.expression, visitor);
    if let Some(ref returns) = node.returns {
        accept_returns_declaration(returns, visitor);
    }
    accept_block(&node.body, visitor);
    accept_catch_clauses(&node.catch_clauses, visitor);
    visitor.leave_try_statement(node);
}

pub fn accept_tuple_expression(node: &TupleExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_tuple_expression(node) {
        return;
    }
    accept_tuple_values(&node.items, visitor);
    visitor.leave_tuple_expression(node);
}

pub fn accept_tuple_value(node: &TupleValue, visitor: &mut impl Visitor) {
    if !visitor.enter_tuple_value(node) {
        return;
    }
    if let Some(ref expression) = node.expression {
        accept_expression(expression, visitor);
    }

    visitor.leave_tuple_value(node);
}

pub fn accept_type_expression(node: &TypeExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_type_expression(node) {
        return;
    }
    accept_type_name(&node.type_name, visitor);
    visitor.leave_type_expression(node);
}

pub fn accept_unchecked_block(node: &UncheckedBlock, visitor: &mut impl Visitor) {
    if !visitor.enter_unchecked_block(node) {
        return;
    }
    accept_block(&node.block, visitor);
    visitor.leave_unchecked_block(node);
}

pub fn accept_user_defined_value_type_definition(
    node: &UserDefinedValueTypeDefinition,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_user_defined_value_type_definition(node) {
        return;
    }
    accept_elementary_type(&node.value_type, visitor);
    visitor.leave_user_defined_value_type_definition(node);
}

pub fn accept_using_alias(node: &UsingAlias, visitor: &mut impl Visitor) {
    if !visitor.enter_using_alias(node) {
        return;
    }
    accept_using_operator(&node.operator, visitor);
    visitor.leave_using_alias(node);
}

pub fn accept_using_deconstruction(node: &UsingDeconstruction, visitor: &mut impl Visitor) {
    if !visitor.enter_using_deconstruction(node) {
        return;
    }
    accept_using_deconstruction_symbols(&node.symbols, visitor);
    visitor.leave_using_deconstruction(node);
}

pub fn accept_using_deconstruction_symbol(
    node: &UsingDeconstructionSymbol,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_using_deconstruction_symbol(node) {
        return;
    }
    accept_identifier_path(&node.name, visitor);
    if let Some(ref alias) = node.alias {
        accept_using_alias(alias, visitor);
    }

    visitor.leave_using_deconstruction_symbol(node);
}

pub fn accept_using_directive(node: &UsingDirective, visitor: &mut impl Visitor) {
    if !visitor.enter_using_directive(node) {
        return;
    }
    accept_using_clause(&node.clause, visitor);
    accept_using_target(&node.target, visitor);
    visitor.leave_using_directive(node);
}

pub fn accept_variable_declaration(node: &VariableDeclaration, visitor: &mut impl Visitor) {
    if !visitor.enter_variable_declaration(node) {
        return;
    }
    accept_type_name(&node.type_name, visitor);
    if let Some(ref storage_location) = node.storage_location {
        accept_storage_location(storage_location, visitor);
    }

    visitor.leave_variable_declaration(node);
}

pub fn accept_variable_declaration_statement(
    node: &VariableDeclarationStatement,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_variable_declaration_statement(node) {
        return;
    }
    accept_variable_declaration_target(&node.target, visitor);
    visitor.leave_variable_declaration_statement(node);
}

pub fn accept_variable_declaration_value(
    node: &VariableDeclarationValue,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_variable_declaration_value(node) {
        return;
    }
    accept_expression(&node.expression, visitor);
    visitor.leave_variable_declaration_value(node);
}

pub fn accept_version_pragma(node: &VersionPragma, visitor: &mut impl Visitor) {
    if !visitor.enter_version_pragma(node) {
        return;
    }
    accept_version_expression_sets(&node.sets, visitor);
    visitor.leave_version_pragma(node);
}

pub fn accept_version_range(node: &VersionRange, visitor: &mut impl Visitor) {
    if !visitor.enter_version_range(node) {
        return;
    }
    accept_version_literal(&node.start, visitor);
    accept_version_literal(&node.end, visitor);
    visitor.leave_version_range(node);
}

pub fn accept_version_term(node: &VersionTerm, visitor: &mut impl Visitor) {
    if !visitor.enter_version_term(node) {
        return;
    }
    if let Some(ref operator) = node.operator {
        accept_version_operator(operator, visitor);
    }
    accept_version_literal(&node.literal, visitor);
    visitor.leave_version_term(node);
}

pub fn accept_while_statement(node: &WhileStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_while_statement(node) {
        return;
    }
    accept_expression(&node.condition, visitor);
    accept_statement(&node.body, visitor);
    visitor.leave_while_statement(node);
}

pub fn accept_yul_block(node: &YulBlock, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_block(node) {
        return;
    }
    accept_yul_statements(&node.statements, visitor);
    visitor.leave_yul_block(node);
}

pub fn accept_yul_break_statement(node: &YulBreakStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_break_statement(node) {
        return;
    }

    visitor.leave_yul_break_statement(node);
}

pub fn accept_yul_continue_statement(node: &YulContinueStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_continue_statement(node) {
        return;
    }

    visitor.leave_yul_continue_statement(node);
}

pub fn accept_yul_default_case(node: &YulDefaultCase, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_default_case(node) {
        return;
    }
    accept_yul_block(&node.body, visitor);
    visitor.leave_yul_default_case(node);
}

pub fn accept_yul_flags_declaration(node: &YulFlagsDeclaration, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_flags_declaration(node) {
        return;
    }
    accept_yul_flags(&node.flags, visitor);
    visitor.leave_yul_flags_declaration(node);
}

pub fn accept_yul_for_statement(node: &YulForStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_for_statement(node) {
        return;
    }
    accept_yul_block(&node.initialization, visitor);
    accept_yul_expression(&node.condition, visitor);
    accept_yul_block(&node.iterator, visitor);
    accept_yul_block(&node.body, visitor);
    visitor.leave_yul_for_statement(node);
}

pub fn accept_yul_function_call_expression(
    node: &YulFunctionCallExpression,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_yul_function_call_expression(node) {
        return;
    }
    accept_yul_expression(&node.operand, visitor);
    accept_yul_arguments(&node.arguments, visitor);
    visitor.leave_yul_function_call_expression(node);
}

pub fn accept_yul_function_definition(node: &YulFunctionDefinition, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_function_definition(node) {
        return;
    }
    accept_yul_parameters_declaration(&node.parameters, visitor);
    if let Some(ref returns) = node.returns {
        accept_yul_returns_declaration(returns, visitor);
    }
    accept_yul_block(&node.body, visitor);
    visitor.leave_yul_function_definition(node);
}

pub fn accept_yul_if_statement(node: &YulIfStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_if_statement(node) {
        return;
    }
    accept_yul_expression(&node.condition, visitor);
    accept_yul_block(&node.body, visitor);
    visitor.leave_yul_if_statement(node);
}

pub fn accept_yul_leave_statement(node: &YulLeaveStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_leave_statement(node) {
        return;
    }

    visitor.leave_yul_leave_statement(node);
}

pub fn accept_yul_parameters_declaration(
    node: &YulParametersDeclaration,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_yul_parameters_declaration(node) {
        return;
    }
    accept_yul_parameters(&node.parameters, visitor);
    visitor.leave_yul_parameters_declaration(node);
}

pub fn accept_yul_returns_declaration(node: &YulReturnsDeclaration, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_returns_declaration(node) {
        return;
    }
    accept_yul_variable_names(&node.variables, visitor);
    visitor.leave_yul_returns_declaration(node);
}

pub fn accept_yul_switch_statement(node: &YulSwitchStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_switch_statement(node) {
        return;
    }
    accept_yul_expression(&node.expression, visitor);
    accept_yul_switch_cases(&node.cases, visitor);
    visitor.leave_yul_switch_statement(node);
}

pub fn accept_yul_value_case(node: &YulValueCase, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_value_case(node) {
        return;
    }
    accept_yul_literal(&node.value, visitor);
    accept_yul_block(&node.body, visitor);
    visitor.leave_yul_value_case(node);
}

pub fn accept_yul_variable_assignment_statement(
    node: &YulVariableAssignmentStatement,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_yul_variable_assignment_statement(node) {
        return;
    }
    accept_yul_paths(&node.variables, visitor);
    accept_yul_expression(&node.expression, visitor);
    visitor.leave_yul_variable_assignment_statement(node);
}

pub fn accept_yul_variable_declaration_statement(
    node: &YulVariableDeclarationStatement,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_yul_variable_declaration_statement(node) {
        return;
    }
    accept_yul_variable_names(&node.variables, visitor);
    if let Some(ref value) = node.value {
        accept_yul_variable_declaration_value(value, visitor);
    }

    visitor.leave_yul_variable_declaration_statement(node);
}

pub fn accept_yul_variable_declaration_value(
    node: &YulVariableDeclarationValue,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_yul_variable_declaration_value(node) {
        return;
    }
    accept_yul_expression(&node.expression, visitor);
    visitor.leave_yul_variable_declaration_value(node);
}

//
// Choices
//

pub fn accept_abicoder_version(_node: &AbicoderVersion, _visitor: &mut impl Visitor) {}

pub fn accept_arguments_declaration(node: &ArgumentsDeclaration, visitor: &mut impl Visitor) {
    if !visitor.enter_arguments_declaration(node) {
        return;
    }
    match node {
        ArgumentsDeclaration::PositionalArgumentsDeclaration(ref inner) => {
            accept_positional_arguments_declaration(inner, visitor);
        }
        ArgumentsDeclaration::NamedArgumentsDeclaration(ref inner) => {
            accept_named_arguments_declaration(inner, visitor);
        }
    }
    visitor.leave_arguments_declaration(node);
}

pub fn accept_constructor_attribute(node: &ConstructorAttribute, visitor: &mut impl Visitor) {
    if !visitor.enter_constructor_attribute(node) {
        return;
    }
    match node {
        ConstructorAttribute::ModifierInvocation(ref inner) => {
            accept_modifier_invocation(inner, visitor);
        }

        ConstructorAttribute::InternalKeyword(_)
        | ConstructorAttribute::PayableKeyword(_)
        | ConstructorAttribute::PublicKeyword(_) => {}
    }
    visitor.leave_constructor_attribute(node);
}

pub fn accept_contract_member(node: &ContractMember, visitor: &mut impl Visitor) {
    if !visitor.enter_contract_member(node) {
        return;
    }
    match node {
        ContractMember::UsingDirective(ref inner) => {
            accept_using_directive(inner, visitor);
        }
        ContractMember::FunctionDefinition(ref inner) => {
            accept_function_definition(inner, visitor);
        }
        ContractMember::ConstructorDefinition(ref inner) => {
            accept_constructor_definition(inner, visitor);
        }
        ContractMember::ReceiveFunctionDefinition(ref inner) => {
            accept_receive_function_definition(inner, visitor);
        }
        ContractMember::FallbackFunctionDefinition(ref inner) => {
            accept_fallback_function_definition(inner, visitor);
        }
        ContractMember::ModifierDefinition(ref inner) => {
            accept_modifier_definition(inner, visitor);
        }
        ContractMember::StructDefinition(ref inner) => {
            accept_struct_definition(inner, visitor);
        }
        ContractMember::EnumDefinition(ref inner) => {
            accept_enum_definition(inner, visitor);
        }
        ContractMember::EventDefinition(ref inner) => {
            accept_event_definition(inner, visitor);
        }
        ContractMember::ErrorDefinition(ref inner) => {
            accept_error_definition(inner, visitor);
        }
        ContractMember::UserDefinedValueTypeDefinition(ref inner) => {
            accept_user_defined_value_type_definition(inner, visitor);
        }
        ContractMember::StateVariableDefinition(ref inner) => {
            accept_state_variable_definition(inner, visitor);
        }
    }
    visitor.leave_contract_member(node);
}

pub fn accept_contract_specifier(node: &ContractSpecifier, visitor: &mut impl Visitor) {
    if !visitor.enter_contract_specifier(node) {
        return;
    }
    match node {
        ContractSpecifier::InheritanceSpecifier(ref inner) => {
            accept_inheritance_specifier(inner, visitor);
        }
        ContractSpecifier::StorageLayoutSpecifier(ref inner) => {
            accept_storage_layout_specifier(inner, visitor);
        }
    }
    visitor.leave_contract_specifier(node);
}

pub fn accept_elementary_type(node: &ElementaryType, visitor: &mut impl Visitor) {
    if !visitor.enter_elementary_type(node) {
        return;
    }
    match node {
        ElementaryType::AddressType(ref inner) => {
            accept_address_type(inner, visitor);
        }

        ElementaryType::BoolKeyword(_)
        | ElementaryType::StringKeyword(_)
        | ElementaryType::BytesKeyword(_)
        | ElementaryType::IntKeyword(_)
        | ElementaryType::UintKeyword(_)
        | ElementaryType::FixedKeyword(_)
        | ElementaryType::UfixedKeyword(_) => {}
    }
    visitor.leave_elementary_type(node);
}

pub fn accept_experimental_feature(_node: &ExperimentalFeature, _visitor: &mut impl Visitor) {}

pub fn accept_expression(node: &Expression, visitor: &mut impl Visitor) {
    if !visitor.enter_expression(node) {
        return;
    }
    match node {
        Expression::AssignmentExpression(ref inner) => {
            accept_assignment_expression(inner, visitor);
        }
        Expression::ConditionalExpression(ref inner) => {
            accept_conditional_expression(inner, visitor);
        }
        Expression::OrExpression(ref inner) => {
            accept_or_expression(inner, visitor);
        }
        Expression::AndExpression(ref inner) => {
            accept_and_expression(inner, visitor);
        }
        Expression::EqualityExpression(ref inner) => {
            accept_equality_expression(inner, visitor);
        }
        Expression::InequalityExpression(ref inner) => {
            accept_inequality_expression(inner, visitor);
        }
        Expression::BitwiseOrExpression(ref inner) => {
            accept_bitwise_or_expression(inner, visitor);
        }
        Expression::BitwiseXorExpression(ref inner) => {
            accept_bitwise_xor_expression(inner, visitor);
        }
        Expression::BitwiseAndExpression(ref inner) => {
            accept_bitwise_and_expression(inner, visitor);
        }
        Expression::ShiftExpression(ref inner) => {
            accept_shift_expression(inner, visitor);
        }
        Expression::AdditiveExpression(ref inner) => {
            accept_additive_expression(inner, visitor);
        }
        Expression::MultiplicativeExpression(ref inner) => {
            accept_multiplicative_expression(inner, visitor);
        }
        Expression::ExponentiationExpression(ref inner) => {
            accept_exponentiation_expression(inner, visitor);
        }
        Expression::PostfixExpression(ref inner) => {
            accept_postfix_expression(inner, visitor);
        }
        Expression::PrefixExpression(ref inner) => {
            accept_prefix_expression(inner, visitor);
        }
        Expression::FunctionCallExpression(ref inner) => {
            accept_function_call_expression(inner, visitor);
        }
        Expression::CallOptionsExpression(ref inner) => {
            accept_call_options_expression(inner, visitor);
        }
        Expression::MemberAccessExpression(ref inner) => {
            accept_member_access_expression(inner, visitor);
        }
        Expression::IndexAccessExpression(ref inner) => {
            accept_index_access_expression(inner, visitor);
        }
        Expression::NewExpression(ref inner) => {
            accept_new_expression(inner, visitor);
        }
        Expression::TupleExpression(ref inner) => {
            accept_tuple_expression(inner, visitor);
        }
        Expression::TypeExpression(ref inner) => {
            accept_type_expression(inner, visitor);
        }
        Expression::ArrayExpression(ref inner) => {
            accept_array_expression(inner, visitor);
        }
        Expression::HexNumberExpression(ref inner) => {
            accept_hex_number_expression(inner, visitor);
        }
        Expression::DecimalNumberExpression(ref inner) => {
            accept_decimal_number_expression(inner, visitor);
        }
        Expression::StringExpression(ref inner) => {
            accept_string_expression(inner, visitor);
        }
        Expression::ElementaryType(ref inner) => {
            accept_elementary_type(inner, visitor);
        }

        Expression::PayableKeyword(_)
        | Expression::ThisKeyword(_)
        | Expression::SuperKeyword(_)
        | Expression::TrueKeyword(_)
        | Expression::FalseKeyword(_)
        | Expression::Identifier(_) => {}
    }
    visitor.leave_expression(node);
}

pub fn accept_expression_additive_expression_operator(
    _node: &Expression_AdditiveExpression_Operator,
    _visitor: &mut impl Visitor,
) {
}

pub fn accept_expression_assignment_expression_operator(
    _node: &Expression_AssignmentExpression_Operator,
    _visitor: &mut impl Visitor,
) {
}

pub fn accept_expression_equality_expression_operator(
    _node: &Expression_EqualityExpression_Operator,
    _visitor: &mut impl Visitor,
) {
}

pub fn accept_expression_inequality_expression_operator(
    _node: &Expression_InequalityExpression_Operator,
    _visitor: &mut impl Visitor,
) {
}

pub fn accept_expression_multiplicative_expression_operator(
    _node: &Expression_MultiplicativeExpression_Operator,
    _visitor: &mut impl Visitor,
) {
}

pub fn accept_expression_postfix_expression_operator(
    _node: &Expression_PostfixExpression_Operator,
    _visitor: &mut impl Visitor,
) {
}

pub fn accept_expression_prefix_expression_operator(
    _node: &Expression_PrefixExpression_Operator,
    _visitor: &mut impl Visitor,
) {
}

pub fn accept_expression_shift_expression_operator(
    _node: &Expression_ShiftExpression_Operator,
    _visitor: &mut impl Visitor,
) {
}

pub fn accept_fallback_function_attribute(
    node: &FallbackFunctionAttribute,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_fallback_function_attribute(node) {
        return;
    }
    match node {
        FallbackFunctionAttribute::ModifierInvocation(ref inner) => {
            accept_modifier_invocation(inner, visitor);
        }
        FallbackFunctionAttribute::OverrideSpecifier(ref inner) => {
            accept_override_specifier(inner, visitor);
        }

        FallbackFunctionAttribute::ExternalKeyword(_)
        | FallbackFunctionAttribute::PayableKeyword(_)
        | FallbackFunctionAttribute::PureKeyword(_)
        | FallbackFunctionAttribute::ViewKeyword(_)
        | FallbackFunctionAttribute::VirtualKeyword(_) => {}
    }
    visitor.leave_fallback_function_attribute(node);
}

pub fn accept_for_statement_condition(node: &ForStatementCondition, visitor: &mut impl Visitor) {
    if !visitor.enter_for_statement_condition(node) {
        return;
    }
    match node {
        ForStatementCondition::ExpressionStatement(ref inner) => {
            accept_expression_statement(inner, visitor);
        }

        ForStatementCondition::Semicolon(_) => {}
    }
    visitor.leave_for_statement_condition(node);
}

pub fn accept_for_statement_initialization(
    node: &ForStatementInitialization,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_for_statement_initialization(node) {
        return;
    }
    match node {
        ForStatementInitialization::VariableDeclarationStatement(ref inner) => {
            accept_variable_declaration_statement(inner, visitor);
        }
        ForStatementInitialization::ExpressionStatement(ref inner) => {
            accept_expression_statement(inner, visitor);
        }

        ForStatementInitialization::Semicolon(_) => {}
    }
    visitor.leave_for_statement_initialization(node);
}

pub fn accept_function_attribute(node: &FunctionAttribute, visitor: &mut impl Visitor) {
    if !visitor.enter_function_attribute(node) {
        return;
    }
    match node {
        FunctionAttribute::ModifierInvocation(ref inner) => {
            accept_modifier_invocation(inner, visitor);
        }
        FunctionAttribute::OverrideSpecifier(ref inner) => {
            accept_override_specifier(inner, visitor);
        }

        FunctionAttribute::ExternalKeyword(_)
        | FunctionAttribute::InternalKeyword(_)
        | FunctionAttribute::PayableKeyword(_)
        | FunctionAttribute::PrivateKeyword(_)
        | FunctionAttribute::PublicKeyword(_)
        | FunctionAttribute::PureKeyword(_)
        | FunctionAttribute::ViewKeyword(_)
        | FunctionAttribute::VirtualKeyword(_) => {}
    }
    visitor.leave_function_attribute(node);
}

pub fn accept_function_body(node: &FunctionBody, visitor: &mut impl Visitor) {
    if !visitor.enter_function_body(node) {
        return;
    }
    match node {
        FunctionBody::Block(ref inner) => {
            accept_block(inner, visitor);
        }

        FunctionBody::Semicolon(_) => {}
    }
    visitor.leave_function_body(node);
}

pub fn accept_function_name(_node: &FunctionName, _visitor: &mut impl Visitor) {}

pub fn accept_function_type_attribute(_node: &FunctionTypeAttribute, _visitor: &mut impl Visitor) {}

pub fn accept_identifier_path_element(_node: &IdentifierPathElement, _visitor: &mut impl Visitor) {}

pub fn accept_import_clause(node: &ImportClause, visitor: &mut impl Visitor) {
    if !visitor.enter_import_clause(node) {
        return;
    }
    match node {
        ImportClause::PathImport(ref inner) => {
            accept_path_import(inner, visitor);
        }
        ImportClause::NamedImport(ref inner) => {
            accept_named_import(inner, visitor);
        }
        ImportClause::ImportDeconstruction(ref inner) => {
            accept_import_deconstruction(inner, visitor);
        }
    }
    visitor.leave_import_clause(node);
}

pub fn accept_mapping_key_type(node: &MappingKeyType, visitor: &mut impl Visitor) {
    if !visitor.enter_mapping_key_type(node) {
        return;
    }
    match node {
        MappingKeyType::ElementaryType(ref inner) => {
            accept_elementary_type(inner, visitor);
        }
        MappingKeyType::IdentifierPath(ref inner) => {
            accept_identifier_path(inner, visitor);
        }
    }
    visitor.leave_mapping_key_type(node);
}

pub fn accept_modifier_attribute(node: &ModifierAttribute, visitor: &mut impl Visitor) {
    if !visitor.enter_modifier_attribute(node) {
        return;
    }
    match node {
        ModifierAttribute::OverrideSpecifier(ref inner) => {
            accept_override_specifier(inner, visitor);
        }

        ModifierAttribute::VirtualKeyword(_) => {}
    }
    visitor.leave_modifier_attribute(node);
}

pub fn accept_number_unit(_node: &NumberUnit, _visitor: &mut impl Visitor) {}

pub fn accept_pragma(node: &Pragma, visitor: &mut impl Visitor) {
    if !visitor.enter_pragma(node) {
        return;
    }
    match node {
        Pragma::VersionPragma(ref inner) => {
            accept_version_pragma(inner, visitor);
        }
        Pragma::AbicoderPragma(ref inner) => {
            accept_abicoder_pragma(inner, visitor);
        }
        Pragma::ExperimentalPragma(ref inner) => {
            accept_experimental_pragma(inner, visitor);
        }
    }
    visitor.leave_pragma(node);
}

pub fn accept_receive_function_attribute(
    node: &ReceiveFunctionAttribute,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_receive_function_attribute(node) {
        return;
    }
    match node {
        ReceiveFunctionAttribute::ModifierInvocation(ref inner) => {
            accept_modifier_invocation(inner, visitor);
        }
        ReceiveFunctionAttribute::OverrideSpecifier(ref inner) => {
            accept_override_specifier(inner, visitor);
        }

        ReceiveFunctionAttribute::ExternalKeyword(_)
        | ReceiveFunctionAttribute::PayableKeyword(_)
        | ReceiveFunctionAttribute::VirtualKeyword(_) => {}
    }
    visitor.leave_receive_function_attribute(node);
}

pub fn accept_source_unit_member(node: &SourceUnitMember, visitor: &mut impl Visitor) {
    if !visitor.enter_source_unit_member(node) {
        return;
    }
    match node {
        SourceUnitMember::PragmaDirective(ref inner) => {
            accept_pragma_directive(inner, visitor);
        }
        SourceUnitMember::ImportDirective(ref inner) => {
            accept_import_directive(inner, visitor);
        }
        SourceUnitMember::ContractDefinition(ref inner) => {
            accept_contract_definition(inner, visitor);
        }
        SourceUnitMember::InterfaceDefinition(ref inner) => {
            accept_interface_definition(inner, visitor);
        }
        SourceUnitMember::LibraryDefinition(ref inner) => {
            accept_library_definition(inner, visitor);
        }
        SourceUnitMember::StructDefinition(ref inner) => {
            accept_struct_definition(inner, visitor);
        }
        SourceUnitMember::EnumDefinition(ref inner) => {
            accept_enum_definition(inner, visitor);
        }
        SourceUnitMember::FunctionDefinition(ref inner) => {
            accept_function_definition(inner, visitor);
        }
        SourceUnitMember::ErrorDefinition(ref inner) => {
            accept_error_definition(inner, visitor);
        }
        SourceUnitMember::UserDefinedValueTypeDefinition(ref inner) => {
            accept_user_defined_value_type_definition(inner, visitor);
        }
        SourceUnitMember::UsingDirective(ref inner) => {
            accept_using_directive(inner, visitor);
        }
        SourceUnitMember::EventDefinition(ref inner) => {
            accept_event_definition(inner, visitor);
        }
        SourceUnitMember::ConstantDefinition(ref inner) => {
            accept_constant_definition(inner, visitor);
        }
    }
    visitor.leave_source_unit_member(node);
}

pub fn accept_state_variable_attribute(node: &StateVariableAttribute, visitor: &mut impl Visitor) {
    if !visitor.enter_state_variable_attribute(node) {
        return;
    }
    match node {
        StateVariableAttribute::OverrideSpecifier(ref inner) => {
            accept_override_specifier(inner, visitor);
        }

        StateVariableAttribute::ConstantKeyword(_)
        | StateVariableAttribute::InternalKeyword(_)
        | StateVariableAttribute::PrivateKeyword(_)
        | StateVariableAttribute::PublicKeyword(_)
        | StateVariableAttribute::ImmutableKeyword(_)
        | StateVariableAttribute::TransientKeyword(_) => {}
    }
    visitor.leave_state_variable_attribute(node);
}

pub fn accept_statement(node: &Statement, visitor: &mut impl Visitor) {
    if !visitor.enter_statement(node) {
        return;
    }
    match node {
        Statement::IfStatement(ref inner) => {
            accept_if_statement(inner, visitor);
        }
        Statement::ForStatement(ref inner) => {
            accept_for_statement(inner, visitor);
        }
        Statement::WhileStatement(ref inner) => {
            accept_while_statement(inner, visitor);
        }
        Statement::DoWhileStatement(ref inner) => {
            accept_do_while_statement(inner, visitor);
        }
        Statement::ContinueStatement(ref inner) => {
            accept_continue_statement(inner, visitor);
        }
        Statement::BreakStatement(ref inner) => {
            accept_break_statement(inner, visitor);
        }
        Statement::ReturnStatement(ref inner) => {
            accept_return_statement(inner, visitor);
        }
        Statement::EmitStatement(ref inner) => {
            accept_emit_statement(inner, visitor);
        }
        Statement::TryStatement(ref inner) => {
            accept_try_statement(inner, visitor);
        }
        Statement::RevertStatement(ref inner) => {
            accept_revert_statement(inner, visitor);
        }
        Statement::AssemblyStatement(ref inner) => {
            accept_assembly_statement(inner, visitor);
        }
        Statement::Block(ref inner) => {
            accept_block(inner, visitor);
        }
        Statement::UncheckedBlock(ref inner) => {
            accept_unchecked_block(inner, visitor);
        }
        Statement::VariableDeclarationStatement(ref inner) => {
            accept_variable_declaration_statement(inner, visitor);
        }
        Statement::ExpressionStatement(ref inner) => {
            accept_expression_statement(inner, visitor);
        }
    }
    visitor.leave_statement(node);
}

pub fn accept_storage_location(_node: &StorageLocation, _visitor: &mut impl Visitor) {}

pub fn accept_string_expression(node: &StringExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_string_expression(node) {
        return;
    }
    match node {
        StringExpression::StringLiterals(ref inner) => {
            accept_string_literals(inner, visitor);
        }
        StringExpression::HexStringLiterals(ref inner) => {
            accept_hex_string_literals(inner, visitor);
        }
        StringExpression::UnicodeStringLiterals(ref inner) => {
            accept_unicode_string_literals(inner, visitor);
        }
    }
    visitor.leave_string_expression(node);
}

pub fn accept_type_name(node: &TypeName, visitor: &mut impl Visitor) {
    if !visitor.enter_type_name(node) {
        return;
    }
    match node {
        TypeName::ArrayTypeName(ref inner) => {
            accept_array_type_name(inner, visitor);
        }
        TypeName::FunctionType(ref inner) => {
            accept_function_type(inner, visitor);
        }
        TypeName::MappingType(ref inner) => {
            accept_mapping_type(inner, visitor);
        }
        TypeName::ElementaryType(ref inner) => {
            accept_elementary_type(inner, visitor);
        }
        TypeName::IdentifierPath(ref inner) => {
            accept_identifier_path(inner, visitor);
        }
    }
    visitor.leave_type_name(node);
}

pub fn accept_using_clause(node: &UsingClause, visitor: &mut impl Visitor) {
    if !visitor.enter_using_clause(node) {
        return;
    }
    match node {
        UsingClause::IdentifierPath(ref inner) => {
            accept_identifier_path(inner, visitor);
        }
        UsingClause::UsingDeconstruction(ref inner) => {
            accept_using_deconstruction(inner, visitor);
        }
    }
    visitor.leave_using_clause(node);
}

pub fn accept_using_operator(_node: &UsingOperator, _visitor: &mut impl Visitor) {}

pub fn accept_using_target(node: &UsingTarget, visitor: &mut impl Visitor) {
    if !visitor.enter_using_target(node) {
        return;
    }
    match node {
        UsingTarget::TypeName(ref inner) => {
            accept_type_name(inner, visitor);
        }

        UsingTarget::Asterisk(_) => {}
    }
    visitor.leave_using_target(node);
}

pub fn accept_variable_declaration_target(
    node: &VariableDeclarationTarget,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_variable_declaration_target(node) {
        return;
    }
    match node {
        VariableDeclarationTarget::SingleTypedDeclaration(ref inner) => {
            accept_single_typed_declaration(inner, visitor);
        }
        VariableDeclarationTarget::MultiTypedDeclaration(ref inner) => {
            accept_multi_typed_declaration(inner, visitor);
        }
    }
    visitor.leave_variable_declaration_target(node);
}

pub fn accept_version_expression(node: &VersionExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_version_expression(node) {
        return;
    }
    match node {
        VersionExpression::VersionRange(ref inner) => {
            accept_version_range(inner, visitor);
        }
        VersionExpression::VersionTerm(ref inner) => {
            accept_version_term(inner, visitor);
        }
    }
    visitor.leave_version_expression(node);
}

pub fn accept_version_literal(node: &VersionLiteral, visitor: &mut impl Visitor) {
    if !visitor.enter_version_literal(node) {
        return;
    }
    match node {
        VersionLiteral::SimpleVersionLiteral(ref inner) => {
            accept_simple_version_literal(inner, visitor);
        }

        VersionLiteral::PragmaStringLiteral(_) => {}
    }
    visitor.leave_version_literal(node);
}

pub fn accept_version_operator(_node: &VersionOperator, _visitor: &mut impl Visitor) {}

pub fn accept_yul_expression(node: &YulExpression, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_expression(node) {
        return;
    }
    match node {
        YulExpression::YulFunctionCallExpression(ref inner) => {
            accept_yul_function_call_expression(inner, visitor);
        }
        YulExpression::YulLiteral(ref inner) => {
            accept_yul_literal(inner, visitor);
        }
        YulExpression::YulPath(ref inner) => {
            accept_yul_path(inner, visitor);
        }
    }
    visitor.leave_yul_expression(node);
}

pub fn accept_yul_literal(_node: &YulLiteral, _visitor: &mut impl Visitor) {}

pub fn accept_yul_statement(node: &YulStatement, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_statement(node) {
        return;
    }
    match node {
        YulStatement::YulBlock(ref inner) => {
            accept_yul_block(inner, visitor);
        }
        YulStatement::YulFunctionDefinition(ref inner) => {
            accept_yul_function_definition(inner, visitor);
        }
        YulStatement::YulIfStatement(ref inner) => {
            accept_yul_if_statement(inner, visitor);
        }
        YulStatement::YulForStatement(ref inner) => {
            accept_yul_for_statement(inner, visitor);
        }
        YulStatement::YulSwitchStatement(ref inner) => {
            accept_yul_switch_statement(inner, visitor);
        }
        YulStatement::YulLeaveStatement(ref inner) => {
            accept_yul_leave_statement(inner, visitor);
        }
        YulStatement::YulBreakStatement(ref inner) => {
            accept_yul_break_statement(inner, visitor);
        }
        YulStatement::YulContinueStatement(ref inner) => {
            accept_yul_continue_statement(inner, visitor);
        }
        YulStatement::YulVariableAssignmentStatement(ref inner) => {
            accept_yul_variable_assignment_statement(inner, visitor);
        }
        YulStatement::YulVariableDeclarationStatement(ref inner) => {
            accept_yul_variable_declaration_statement(inner, visitor);
        }
        YulStatement::YulExpression(ref inner) => {
            accept_yul_expression(inner, visitor);
        }
    }
    visitor.leave_yul_statement(node);
}

pub fn accept_yul_switch_case(node: &YulSwitchCase, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_switch_case(node) {
        return;
    }
    match node {
        YulSwitchCase::YulDefaultCase(ref inner) => {
            accept_yul_default_case(inner, visitor);
        }
        YulSwitchCase::YulValueCase(ref inner) => {
            accept_yul_value_case(inner, visitor);
        }
    }
    visitor.leave_yul_switch_case(node);
}

//
// Collections
//

pub fn accept_array_values(node: &ArrayValues, visitor: &mut impl Visitor) {
    if !visitor.enter_array_values(node) {
        return;
    }
    for item in &node.elements {
        accept_expression(item, visitor);
    }
    visitor.leave_array_values(node);
}

pub fn accept_call_options(node: &CallOptions, visitor: &mut impl Visitor) {
    if !visitor.enter_call_options(node) {
        return;
    }
    for item in &node.elements {
        accept_named_argument(item, visitor);
    }
    visitor.leave_call_options(node);
}

pub fn accept_catch_clauses(node: &CatchClauses, visitor: &mut impl Visitor) {
    if !visitor.enter_catch_clauses(node) {
        return;
    }
    for item in &node.elements {
        accept_catch_clause(item, visitor);
    }
    visitor.leave_catch_clauses(node);
}

pub fn accept_constructor_attributes(node: &ConstructorAttributes, visitor: &mut impl Visitor) {
    if !visitor.enter_constructor_attributes(node) {
        return;
    }
    for item in &node.elements {
        accept_constructor_attribute(item, visitor);
    }
    visitor.leave_constructor_attributes(node);
}

pub fn accept_contract_members(node: &ContractMembers, visitor: &mut impl Visitor) {
    if !visitor.enter_contract_members(node) {
        return;
    }
    for item in &node.elements {
        accept_contract_member(item, visitor);
    }
    visitor.leave_contract_members(node);
}

pub fn accept_contract_specifiers(node: &ContractSpecifiers, visitor: &mut impl Visitor) {
    if !visitor.enter_contract_specifiers(node) {
        return;
    }
    for item in &node.elements {
        accept_contract_specifier(item, visitor);
    }
    visitor.leave_contract_specifiers(node);
}

pub fn accept_enum_members(node: &EnumMembers, visitor: &mut impl Visitor) {
    if !visitor.enter_enum_members(node) {
        return;
    }
    visitor.leave_enum_members(node);
}

pub fn accept_error_parameters(node: &ErrorParameters, visitor: &mut impl Visitor) {
    if !visitor.enter_error_parameters(node) {
        return;
    }
    for item in &node.elements {
        accept_error_parameter(item, visitor);
    }
    visitor.leave_error_parameters(node);
}

pub fn accept_event_parameters(node: &EventParameters, visitor: &mut impl Visitor) {
    if !visitor.enter_event_parameters(node) {
        return;
    }
    for item in &node.elements {
        accept_event_parameter(item, visitor);
    }
    visitor.leave_event_parameters(node);
}

pub fn accept_fallback_function_attributes(
    node: &FallbackFunctionAttributes,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_fallback_function_attributes(node) {
        return;
    }
    for item in &node.elements {
        accept_fallback_function_attribute(item, visitor);
    }
    visitor.leave_fallback_function_attributes(node);
}

pub fn accept_function_attributes(node: &FunctionAttributes, visitor: &mut impl Visitor) {
    if !visitor.enter_function_attributes(node) {
        return;
    }
    for item in &node.elements {
        accept_function_attribute(item, visitor);
    }
    visitor.leave_function_attributes(node);
}

pub fn accept_function_type_attributes(node: &FunctionTypeAttributes, visitor: &mut impl Visitor) {
    if !visitor.enter_function_type_attributes(node) {
        return;
    }
    for item in &node.elements {
        accept_function_type_attribute(item, visitor);
    }
    visitor.leave_function_type_attributes(node);
}

pub fn accept_hex_string_literals(node: &HexStringLiterals, visitor: &mut impl Visitor) {
    if !visitor.enter_hex_string_literals(node) {
        return;
    }
    visitor.leave_hex_string_literals(node);
}

pub fn accept_identifier_path(node: &IdentifierPath, visitor: &mut impl Visitor) {
    if !visitor.enter_identifier_path(node) {
        return;
    }
    for item in &node.elements {
        accept_identifier_path_element(item, visitor);
    }
    visitor.leave_identifier_path(node);
}

pub fn accept_import_deconstruction_symbols(
    node: &ImportDeconstructionSymbols,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_import_deconstruction_symbols(node) {
        return;
    }
    for item in &node.elements {
        accept_import_deconstruction_symbol(item, visitor);
    }
    visitor.leave_import_deconstruction_symbols(node);
}

pub fn accept_inheritance_types(node: &InheritanceTypes, visitor: &mut impl Visitor) {
    if !visitor.enter_inheritance_types(node) {
        return;
    }
    for item in &node.elements {
        accept_inheritance_type(item, visitor);
    }
    visitor.leave_inheritance_types(node);
}

pub fn accept_interface_members(node: &InterfaceMembers, visitor: &mut impl Visitor) {
    if !visitor.enter_interface_members(node) {
        return;
    }
    for item in &node.elements {
        accept_contract_member(item, visitor);
    }
    visitor.leave_interface_members(node);
}

pub fn accept_library_members(node: &LibraryMembers, visitor: &mut impl Visitor) {
    if !visitor.enter_library_members(node) {
        return;
    }
    for item in &node.elements {
        accept_contract_member(item, visitor);
    }
    visitor.leave_library_members(node);
}

pub fn accept_modifier_attributes(node: &ModifierAttributes, visitor: &mut impl Visitor) {
    if !visitor.enter_modifier_attributes(node) {
        return;
    }
    for item in &node.elements {
        accept_modifier_attribute(item, visitor);
    }
    visitor.leave_modifier_attributes(node);
}

pub fn accept_multi_typed_declaration_elements(
    node: &MultiTypedDeclarationElements,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_multi_typed_declaration_elements(node) {
        return;
    }
    for item in &node.elements {
        accept_multi_typed_declaration_element(item, visitor);
    }
    visitor.leave_multi_typed_declaration_elements(node);
}

pub fn accept_named_arguments(node: &NamedArguments, visitor: &mut impl Visitor) {
    if !visitor.enter_named_arguments(node) {
        return;
    }
    for item in &node.elements {
        accept_named_argument(item, visitor);
    }
    visitor.leave_named_arguments(node);
}

pub fn accept_override_paths(node: &OverridePaths, visitor: &mut impl Visitor) {
    if !visitor.enter_override_paths(node) {
        return;
    }
    for item in &node.elements {
        accept_identifier_path(item, visitor);
    }
    visitor.leave_override_paths(node);
}

pub fn accept_parameters(node: &Parameters, visitor: &mut impl Visitor) {
    if !visitor.enter_parameters(node) {
        return;
    }
    for item in &node.elements {
        accept_parameter(item, visitor);
    }
    visitor.leave_parameters(node);
}

pub fn accept_positional_arguments(node: &PositionalArguments, visitor: &mut impl Visitor) {
    if !visitor.enter_positional_arguments(node) {
        return;
    }
    for item in &node.elements {
        accept_expression(item, visitor);
    }
    visitor.leave_positional_arguments(node);
}

pub fn accept_receive_function_attributes(
    node: &ReceiveFunctionAttributes,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_receive_function_attributes(node) {
        return;
    }
    for item in &node.elements {
        accept_receive_function_attribute(item, visitor);
    }
    visitor.leave_receive_function_attributes(node);
}

pub fn accept_simple_version_literal(node: &SimpleVersionLiteral, visitor: &mut impl Visitor) {
    if !visitor.enter_simple_version_literal(node) {
        return;
    }
    visitor.leave_simple_version_literal(node);
}

pub fn accept_source_unit_members(node: &SourceUnitMembers, visitor: &mut impl Visitor) {
    if !visitor.enter_source_unit_members(node) {
        return;
    }
    for item in &node.elements {
        accept_source_unit_member(item, visitor);
    }
    visitor.leave_source_unit_members(node);
}

pub fn accept_state_variable_attributes(
    node: &StateVariableAttributes,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_state_variable_attributes(node) {
        return;
    }
    for item in &node.elements {
        accept_state_variable_attribute(item, visitor);
    }
    visitor.leave_state_variable_attributes(node);
}

pub fn accept_statements(node: &Statements, visitor: &mut impl Visitor) {
    if !visitor.enter_statements(node) {
        return;
    }
    for item in &node.elements {
        accept_statement(item, visitor);
    }
    visitor.leave_statements(node);
}

pub fn accept_string_literals(node: &StringLiterals, visitor: &mut impl Visitor) {
    if !visitor.enter_string_literals(node) {
        return;
    }
    visitor.leave_string_literals(node);
}

pub fn accept_struct_members(node: &StructMembers, visitor: &mut impl Visitor) {
    if !visitor.enter_struct_members(node) {
        return;
    }
    for item in &node.elements {
        accept_struct_member(item, visitor);
    }
    visitor.leave_struct_members(node);
}

pub fn accept_tuple_values(node: &TupleValues, visitor: &mut impl Visitor) {
    if !visitor.enter_tuple_values(node) {
        return;
    }
    for item in &node.elements {
        accept_tuple_value(item, visitor);
    }
    visitor.leave_tuple_values(node);
}

pub fn accept_unicode_string_literals(node: &UnicodeStringLiterals, visitor: &mut impl Visitor) {
    if !visitor.enter_unicode_string_literals(node) {
        return;
    }
    visitor.leave_unicode_string_literals(node);
}

pub fn accept_using_deconstruction_symbols(
    node: &UsingDeconstructionSymbols,
    visitor: &mut impl Visitor,
) {
    if !visitor.enter_using_deconstruction_symbols(node) {
        return;
    }
    for item in &node.elements {
        accept_using_deconstruction_symbol(item, visitor);
    }
    visitor.leave_using_deconstruction_symbols(node);
}

pub fn accept_version_expression_set(node: &VersionExpressionSet, visitor: &mut impl Visitor) {
    if !visitor.enter_version_expression_set(node) {
        return;
    }
    for item in &node.elements {
        accept_version_expression(item, visitor);
    }
    visitor.leave_version_expression_set(node);
}

pub fn accept_version_expression_sets(node: &VersionExpressionSets, visitor: &mut impl Visitor) {
    if !visitor.enter_version_expression_sets(node) {
        return;
    }
    for item in &node.elements {
        accept_version_expression_set(item, visitor);
    }
    visitor.leave_version_expression_sets(node);
}

pub fn accept_yul_arguments(node: &YulArguments, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_arguments(node) {
        return;
    }
    for item in &node.elements {
        accept_yul_expression(item, visitor);
    }
    visitor.leave_yul_arguments(node);
}

pub fn accept_yul_flags(node: &YulFlags, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_flags(node) {
        return;
    }
    visitor.leave_yul_flags(node);
}

pub fn accept_yul_parameters(node: &YulParameters, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_parameters(node) {
        return;
    }
    visitor.leave_yul_parameters(node);
}

pub fn accept_yul_path(node: &YulPath, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_path(node) {
        return;
    }
    visitor.leave_yul_path(node);
}

pub fn accept_yul_paths(node: &YulPaths, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_paths(node) {
        return;
    }
    for item in &node.elements {
        accept_yul_path(item, visitor);
    }
    visitor.leave_yul_paths(node);
}

pub fn accept_yul_statements(node: &YulStatements, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_statements(node) {
        return;
    }
    for item in &node.elements {
        accept_yul_statement(item, visitor);
    }
    visitor.leave_yul_statements(node);
}

pub fn accept_yul_switch_cases(node: &YulSwitchCases, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_switch_cases(node) {
        return;
    }
    for item in &node.elements {
        accept_yul_switch_case(item, visitor);
    }
    visitor.leave_yul_switch_cases(node);
}

pub fn accept_yul_variable_names(node: &YulVariableNames, visitor: &mut impl Visitor) {
    if !visitor.enter_yul_variable_names(node) {
        return;
    }
    visitor.leave_yul_variable_names(node);
}
