use ruint::aliases::{U160, U256};
use slang_solidity_v2_common::nodes::NodeId;
use slang_solidity_v2_ir::ir;

use super::Pass;
use crate::binder::{Definition, Resolution, Typing};
use crate::built_ins::InternalBuiltIn;
use crate::passes::common::constant_evaluator::{evaluate_compile_time_constant, ConstantResolver};
use crate::passes::common::{node_id_for_expression_typing, node_location};
use crate::types::{
    literals, AddressType, ArrayType, ContractType, DataLocation, EnumType, FixedSizeArrayType,
    FunctionType, IntegerType, InterfaceType, LibraryType, LiteralKind, MetaType, Number,
    StringType, StructType, Type, TypeId, UserDefinedValueType, UserMetaType,
};

impl Pass<'_> {
    /// Registers `inner` and the [`Type::MetaType`] wrapping it, returning the
    /// meta-type as a `Typing::Resolved`. This is how expressions that refer to
    /// a type rather than a value (eg. the `uint` in a cast `uint(x)`) are
    /// typed.
    pub(super) fn meta_typing_of(&mut self, inner: Type) -> Typing {
        let type_id = self.types.register_type(inner);
        Typing::Resolved(
            self.types
                .register_type(Type::MetaType(MetaType { type_id })),
        )
    }

    pub(super) fn typing_of_expression(&self, node: &ir::Expression) -> Typing {
        match node {
            // These are always typed as boolean
            ir::Expression::OrExpression(_)
            | ir::Expression::AndExpression(_)
            | ir::Expression::EqualityExpression(_)
            | ir::Expression::InequalityExpression(_)
            | ir::Expression::TrueKeyword(_)
            | ir::Expression::FalseKeyword(_) => Typing::Resolved(self.types.boolean()),

            // Other special cases
            ir::Expression::SuperKeyword(_) => Typing::Super,

            // By default, query the binder for registered typing information.
            // This includes elementary types and the `payable` keyword, whose
            // meta-typings are stored by the corresponding visitor hooks.
            _ => {
                let node_id = node_id_for_expression_typing(node).expect(
                    "typing of expression variant not handled and it doesn't have a NodeId",
                );
                self.binder.node_typing(node_id)
            }
        }
    }

    /// Returns the typing of an expression when it is a *value*, filtering out
    /// meta-types (see [`Typing::as_value_type_id`]). This is the accessor for
    /// value positions: operator operands, conditional branches, receivers,
    /// tuple/array literal elements.
    pub(super) fn value_type_id_of_expression(&self, node: &ir::Expression) -> Option<TypeId> {
        self.typing_of_expression(node).as_value_type_id(self.types)
    }

    /// Builds the meta-typing of an array type expression `T[...]` (an index
    /// access whose operand is itself a meta-type): `T[]` when there is no
    /// index, `T[n]` when the index evaluates to a compile-time constant
    /// (a literal or a constant like `uint[LEN]`, mirroring what
    /// `resolve_array_length` does for array type *names* in p3). A size that
    /// does not evaluate to a positive in-range integer yields `Unresolved`.
    // TODO(validation): report the array-length diagnostics (not constant,
    // zero, fractional, ...) that the p3 type-name path reports.
    pub(super) fn meta_typing_of_array_type_expression(
        &mut self,
        element_type: TypeId,
        node: &ir::IndexAccessExpression,
    ) -> Typing {
        let Some(size_expression) = &node.start else {
            return self.meta_typing_of(Type::Array(ArrayType {
                element_type,
                location: DataLocation::Memory,
            }));
        };
        let (file_id, range) = node_location(size_expression, self.file_node_mapper);
        let size = evaluate_compile_time_constant(
            size_expression,
            self.current_scope_id(),
            self.types,
            &ConstantResolver {
                binder: self.binder,
                use_site: Some((&file_id, range.start)),
            },
        )
        .ok()
        .and_then(|value| value.into_integer())
        .and_then(|value| U256::try_from(&value).ok())
        .filter(|size| !size.is_zero());
        match size {
            Some(size) => self.meta_typing_of(Type::FixedSizeArray(FixedSizeArrayType {
                element_type,
                size,
                location: DataLocation::Memory,
            })),
            None => Typing::Unresolved,
        }
    }

    pub(super) fn type_of_elementary_type(elementary_type: &ir::ElementaryType) -> Type {
        match elementary_type {
            ir::ElementaryType::AddressType(address_type) => Type::Address(AddressType {
                is_payable: address_type.is_payable,
            }),
            ir::ElementaryType::BytesKeyword(terminal) => {
                Type::from_bytes_keyword(terminal.unparse(), Some(DataLocation::Memory)).unwrap()
            }
            ir::ElementaryType::IntKeyword(terminal) => Type::from_int_keyword(terminal.unparse()),
            ir::ElementaryType::UintKeyword(terminal) => {
                Type::from_uint_keyword(terminal.unparse())
            }
            ir::ElementaryType::FixedKeyword(terminal) => {
                Type::from_fixed_keyword(terminal.unparse())
            }
            ir::ElementaryType::UfixedKeyword(terminal) => {
                Type::from_ufixed_keyword(terminal.unparse())
            }
            ir::ElementaryType::BoolKeyword(_) => Type::Boolean,
            ir::ElementaryType::StringKeyword(_) => Type::String(StringType {
                location: DataLocation::Memory,
            }),
        }
    }

    pub(super) fn type_of_definition(&self, definition_id: NodeId) -> Option<Type> {
        let definition = self.binder.find_definition_by_id(definition_id)?;
        match definition {
            Definition::Contract(_) => Some(Type::Contract(ContractType { definition_id })),
            Definition::Enum(_) => Some(Type::Enum(EnumType { definition_id })),
            Definition::Interface(_) => Some(Type::Interface(InterfaceType { definition_id })),
            Definition::Library(_) => Some(Type::Library(LibraryType { definition_id })),
            Definition::Struct(_) => Some(Type::Struct(StructType {
                definition_id,
                location: DataLocation::Memory,
            })),
            Definition::UserDefinedValueType(_) => {
                Some(Type::UserDefinedValue(UserDefinedValueType {
                    definition_id,
                }))
            }
            _ => None,
        }
    }

    /// Returns the type of an binary operator expression. If both operands are
    /// number literals, applies `op` to fold them into a narrowed literal type;
    /// otherwise falls back to the implicit-convertibility rule between the
    /// operand types.
    pub(super) fn type_of_binary_operator_expression<F>(
        &mut self,
        left_operand: &ir::Expression,
        right_operand: &ir::Expression,
        op: F,
    ) -> Option<TypeId>
    where
        F: FnOnce(&Number, &Number) -> Option<Number>,
    {
        let left_type_id = self.value_type_id_of_expression(left_operand)?;
        let right_type_id = self.value_type_id_of_expression(right_operand)?;

        // If both operands are number constants, fold them using the given operator.
        if let (Some(left_value), Some(right_value)) = (
            self.types.number_value_of_type_id(left_type_id),
            self.types.number_value_of_type_id(right_type_id),
        ) {
            return op(&left_value, &right_value).map(|result| {
                self.types
                    .register_type(Type::Literal(result.to_literal_kind()))
            });
        }

        // TODO(validation) SDR[44]: check that both operands are valid for the operator
        // (needs additional parameter or check at the call site)
        if self
            .types
            .implicitly_convertible_to(right_type_id, left_type_id)
        {
            Some(left_type_id)
        } else if self
            .types
            .implicitly_convertible_to(left_type_id, right_type_id)
        {
            Some(right_type_id)
        } else {
            // TODO(validation) SDR[43]: the types are not compatible, we should
            // emit an error, or signal our caller
            None
        }
    }

    pub(super) fn type_of_prefix_expression(
        &mut self,
        node: &ir::PrefixExpression,
    ) -> Option<TypeId> {
        match node.operator {
            ir::PrefixExpressionOperator::Minus(_) | ir::PrefixExpressionOperator::Tilde(_) => {
                // Fold `-<constant>` or `~<constant>` by operating on the
                // operand's known number value.
                let operand_type_id = self.value_type_id_of_expression(&node.operand)?;
                if let Some(value) = self.types.number_value_of_type_id(operand_type_id) {
                    let result = match node.operator {
                        ir::PrefixExpressionOperator::Minus(_) => value.negate(),
                        ir::PrefixExpressionOperator::Tilde(_) => value.bit_not()?,
                        _ => unreachable!(),
                    };
                    Some(
                        self.types
                            .register_type(Type::Literal(result.to_literal_kind())),
                    )
                } else {
                    // TODO(validation) SDR[1734]: check that the operand type supports the operator
                    Some(operand_type_id)
                }
            }
            ir::PrefixExpressionOperator::PlusPlus(_)
            | ir::PrefixExpressionOperator::MinusMinus(_) => {
                self.value_type_id_of_expression(&node.operand)
            }
            ir::PrefixExpressionOperator::Bang(_) => {
                // TODO(validation) SDR[49]: check that the operand is boolean
                Some(self.types.boolean())
            }
            ir::PrefixExpressionOperator::DeleteKeyword(_) => Some(self.types.void()),
        }
    }

    pub(super) fn type_of_array_expression(
        &mut self,
        array: &ir::ArrayExpression,
    ) -> Option<TypeId> {
        let mut item_type_ids: Vec<TypeId> = Vec::with_capacity(array.items.len());
        for item in &array.items {
            item_type_ids.push(self.value_type_id_of_expression(item)?);
        }
        let element_type = self.types.type_of_array_literal(&item_type_ids)?;
        Some(
            self.types
                .register_type(Type::FixedSizeArray(FixedSizeArrayType {
                    element_type,
                    size: U256::from(array.items.len()),
                    location: DataLocation::Memory,
                })),
        )
    }

    pub(super) fn type_of_left_typed_binary_operator_expression<F>(
        &mut self,
        left_operand: &ir::Expression,
        right_operand: &ir::Expression,
        op: F,
    ) -> Option<TypeId>
    where
        F: FnOnce(&Number, &Number) -> Option<Number>,
    {
        let left_type_id = self.value_type_id_of_expression(left_operand)?;
        let right_type_id = self.value_type_id_of_expression(right_operand)?;

        let left_value = self.types.number_value_of_type_id(left_type_id);
        let right_value = self.types.number_value_of_type_id(right_type_id);

        if let (Some(left_value), Some(right_value)) = (&left_value, &right_value) {
            // Both constants, so fold them
            op(left_value, right_value).map(|result| {
                self.types
                    .register_type(Type::Literal(result.to_literal_kind()))
            })
        } else if let Some(left_value) = &left_value {
            // For shifts or exponentiations, if the left operand is a literal,
            // the result is either a `uint256` or an `int256` depending on the
            // sign of `left_operand`.
            if left_value.is_negative() {
                Some(self.types.register_type(Type::Integer(IntegerType {
                    is_signed: true,
                    bits: 256,
                })))
            } else {
                Some(self.types.uint256())
            }
        } else {
            // TODO(validation) SDR[1735]: check that the operand types are valid (needs
            // additional parameter or validation at call site)
            Some(left_type_id)
        }
    }

    pub(super) fn typing_of_resolution(&self, resolution: &Resolution) -> Typing {
        match resolution {
            Resolution::Unresolved => Typing::Unresolved,
            Resolution::BuiltIn(built_in) => self.built_ins_resolver().typing_of(built_in),
            Resolution::Definition(definition_id) => self.binder.node_typing(*definition_id),
            Resolution::Ambiguous(definitions) => {
                let mut type_ids = Vec::new();
                for definition_id in definitions {
                    if let Typing::Resolved(type_id) = self.binder.node_typing(*definition_id) {
                        type_ids.push(type_id);
                    }
                }
                Typing::Undetermined(type_ids)
            }
        }
    }

    pub(super) fn typing_is_contract_reference(&self, typing: &Typing) -> bool {
        match typing {
            Typing::This(type_id) | Typing::Resolved(type_id) => matches!(
                self.types.get_type_by_id(*type_id),
                Type::Contract(_) | Type::Interface(_)
            ),
            _ => false,
        }
    }

    pub(super) fn typing_of_resolution_as_contract_member(
        &mut self,
        resolution: &Resolution,
    ) -> Typing {
        // Check if the target is a state variable, and if it has a getter
        if let Resolution::Definition(definition_id) = resolution {
            if let Definition::StateVariable(state_var_definition) =
                self.binder.find_definition_by_id(*definition_id).unwrap()
            {
                if let Some(getter_type_id) = state_var_definition.getter_type_id {
                    return Typing::Resolved(getter_type_id);
                }
            }
        }

        let mut typing = self.typing_of_resolution(resolution);
        let resolved_type = typing
            .as_type_id()
            .map(|type_id| self.types.get_type_by_id(type_id));

        if let Some(Type::Function(function_type)) = resolved_type {
            // If the resolved type is a function and the operand is either
            // `this` or something of an address type, the function is being
            // used as an external function. If the member is a *public*
            // function, change the expression typing to indicate the
            // external access.
            if function_type.is_externally_visible() {
                let external_function_type =
                    self.types.externalize_function_type(function_type.clone());
                let type_id_with_external_visibility = self
                    .types
                    .register_type(Type::Function(external_function_type));
                typing = Typing::Resolved(type_id_with_external_visibility);
            }
        }

        typing
    }

    /// Returns the `TypeId` of the *value* a member-access operand is invoked
    /// on, when there is one — eg. `a` in `a.f(...)`. This is the receiver that
    /// overload resolution binds as an implicit first argument (for `using`
    /// attached functions). Returns `None` when the operand is not a member
    /// access, has no type, or is a meta-type (a type *name* like `Lib` in
    /// `Lib.f(...)` is not a value, so it binds no receiver).
    fn type_id_of_value_receiver(&self, operand: &ir::Expression) -> Option<TypeId> {
        let ir::Expression::MemberAccessExpression(member_access_expression) = operand else {
            return None;
        };
        // The value filter excludes meta-type operands: a type name refers to
        // a type, not a value, so it isn't a receiver and doesn't bind a first
        // argument for overload resolution. Note a bare library name (eg. `L`
        // in `L.foo(...)`) is a `UserMetaType` here — the meta-type of the
        // *name* — not a `Type::Library` value (which is what `this` in a
        // library, or a `L(addr)` cast, produces).
        self.value_type_id_of_expression(&member_access_expression.operand)
    }

    fn typing_of_cast(&mut self, argument_typing: &Typing, target_type: Type) -> Typing {
        // TODO(validation) SDR[40]: this is a cast to the given type, but we
        // need to verify that the (single) argument is convertible
        //
        // The argument must be a *value*: a meta-type argument (eg. the `bool`
        // in `uint(bool)`, casting a type name) does not type.
        match argument_typing.as_value_type_id(self.types) {
            Some(argument_type_id) => {
                // the resulting cast type inherits the data location of the argument
                let argument_type = self.types.get_type_by_id(argument_type_id);
                let type_id = if let Some(data_location) = argument_type.data_location() {
                    self.types
                        .register_type_with_data_location(target_type, data_location)
                } else {
                    self.types.register_type(target_type)
                };
                Typing::Resolved(type_id)
            }
            None => Typing::Unresolved,
        }
    }

    pub(super) fn collect_positional_argument_typings(
        &self,
        arguments: &[ir::Expression],
    ) -> Vec<Typing> {
        arguments
            .iter()
            .map(|argument| self.typing_of_expression(argument))
            .collect::<Vec<_>>()
    }

    pub(super) fn typing_of_function_call_with_positional_arguments(
        &mut self,
        node: &ir::FunctionCallExpression,
        arguments: &[ir::Expression],
    ) -> Typing {
        let operand_typing = self.typing_of_expression(&node.operand);
        let argument_typings = self.collect_positional_argument_typings(arguments);

        match operand_typing {
            Typing::Unresolved | Typing::This(_) | Typing::Super => {
                // `this` and `super` are not callable
                Typing::Unresolved
            }
            Typing::Resolved(type_id) => match self.types.get_type_by_id(type_id) {
                Type::Function(FunctionType { return_type, .. }) => Typing::Resolved(*return_type),
                Type::MetaType(MetaType {
                    type_id: target_type_id,
                }) => {
                    // This is an explicit cast to the (meta-)type, eg. `uint(x)`.
                    let target_type_id = *target_type_id;
                    if argument_typings.len() == 1 {
                        let target_type = self.types.get_type_by_id(target_type_id).clone();
                        self.typing_of_cast(&argument_typings[0], target_type)
                    } else {
                        Typing::Unresolved
                    }
                }
                Type::UserMetaType(UserMetaType { definition_id }) => {
                    // Generally this is a cast to the underlying type of the
                    // given definition (eg. `MyEnum(1)`, `IFoo(addr)`), except
                    // for structs for which this is a construction of the
                    // value in memory
                    let definition_id = *definition_id;
                    match self.binder.find_definition_by_id(definition_id) {
                        Some(
                            Definition::Contract(_)
                            | Definition::Interface(_)
                            | Definition::Library(_)
                            | Definition::Enum(_)
                            | Definition::Struct(_),
                        ) => {
                            // TODO(validation) SDR[39]: for contract, interface
                            // and library targets the type of the (single)
                            // argument should be an address
                            let type_ = self
                                .type_of_definition(definition_id)
                                .expect("definition kind is handled by type_of_definition");
                            Typing::Resolved(self.types.register_type(type_))
                        }
                        // NOTE: user defined value types are deliberately not
                        // castable by name — `MyUDVT(x)` is invalid; conversion
                        // goes through `MyUDVT.wrap`/`.unwrap`.
                        _ => Typing::Unresolved,
                    }
                }
                _ => {
                    // TODO(validation) SDR[41]: the operand did not resolve to a function
                    Typing::Unresolved
                }
            },
            Typing::Undetermined(type_ids) => {
                let receiver_type_id = self.type_id_of_value_receiver(&node.operand);
                let candidate = self.lookup_function_matching_positional_arguments(
                    &type_ids,
                    &argument_typings,
                    receiver_type_id,
                );

                if let Some(candidate) = candidate {
                    let return_type = candidate.return_type;

                    let reference_node_id = reference_node_id_for_expression(&node.operand);
                    let definition_id = candidate.definition_id;
                    if let (Some(node_id), Some(definition_id)) = (reference_node_id, definition_id)
                    {
                        // TODO: maybe update the typing of the operand as well?
                        self.binder
                            .fixup_reference(node_id, Resolution::Definition(definition_id));
                    }

                    Typing::Resolved(return_type)
                } else {
                    Typing::Unresolved
                }
            }
            Typing::NewExpression(type_id) => {
                match self.types.get_type_by_id(type_id) {
                    Type::Array(_) | Type::Contract(_) => Typing::Resolved(type_id),
                    _ => {
                        // only contracts can be created with `new`
                        Typing::Unresolved
                    }
                }
            }
            // Special case: for `abi.decode` we need to register the types
            // given as the second argument and we need a mutable `TypeRegistry`
            // for that.
            Typing::BuiltIn(InternalBuiltIn::AbiDecode) => {
                self.typing_of_abi_decode(&argument_typings)
            }
            Typing::BuiltIn(built_in) => self
                .built_ins_resolver()
                .typing_of_function_call(&built_in, &argument_typings),
        }
    }

    fn typing_of_abi_decode(&mut self, argument_typings: &[Typing]) -> Typing {
        if argument_typings.len() != 2 {
            return Typing::Unresolved;
        }
        match &argument_typings[1] {
            Typing::Resolved(type_id) => match self.types.get_type_by_id(*type_id) {
                // `abi.decode(data, (T))` for an elementary type `T`: the
                // single-element tuple collapses to the meta-type of `T`, which
                // decodes to `T` itself.
                Type::MetaType(MetaType { type_id }) => Typing::Resolved(*type_id),
                // `abi.decode(data, (T))` for a user-defined type `T`.
                Type::UserMetaType(UserMetaType { definition_id }) => {
                    let definition_id = *definition_id;
                    if let Some(type_) = self.type_of_definition(definition_id) {
                        Typing::Resolved(self.types.register_type(type_))
                    } else {
                        Typing::Unresolved
                    }
                }
                // Multi-element `abi.decode(data, (T1, T2, ...))`: the second
                // argument is a tuple expression whose *elements* are each
                // meta-types. To be correct we'd produce a tuple of the decoded
                // value types by unwrapping each element (as the single-element
                // arms above do); we currently return the tuple-of-meta-types
                // verbatim.
                // TODO(validation) SDR[42]: unwrap the tuple's element
                // meta-types, and error if `type_id` is not a tuple of types.
                _ => Typing::Resolved(*type_id),
            },
            _ => Typing::Unresolved,
        }
    }

    pub(super) fn collect_named_argument_typings(
        &self,
        arguments: &[ir::NamedArgument],
    ) -> Vec<(String, Typing)> {
        arguments
            .iter()
            .map(|argument| {
                (
                    argument.name.unparse().to_string(),
                    self.typing_of_expression(&argument.value),
                )
            })
            .collect::<Vec<_>>()
    }

    pub(super) fn typing_of_function_call_with_named_arguments(
        &mut self,
        node: &ir::FunctionCallExpression,
        arguments: &[ir::NamedArgument],
    ) -> Typing {
        let operand_typing = self.typing_of_expression(&node.operand);

        let (typing, definition_id) = match operand_typing {
            Typing::Unresolved | Typing::This(_) | Typing::Super => {
                // `this` and `super` are not callable
                (Typing::Unresolved, None)
            }
            Typing::Resolved(type_id) => match self.types.get_type_by_id(type_id) {
                Type::Function(FunctionType {
                    definition_id,
                    return_type,
                    ..
                }) => (Typing::Resolved(*return_type), *definition_id),
                Type::MetaType(_) => {
                    // This is a cast to the given type and is not valid with named arguments
                    (Typing::Unresolved, None)
                }
                Type::UserMetaType(UserMetaType { definition_id }) => {
                    // Function call with named arguments are only valid in user
                    // types of the struct kind, which results in the construction
                    // of such struct in memory
                    let definition_id = *definition_id;
                    match self.binder.find_definition_by_id(definition_id) {
                        Some(Definition::Struct(_)) => {
                            // struct construction
                            let type_ = self
                                .type_of_definition(definition_id)
                                .expect("struct definitions are handled by type_of_definition");
                            let type_id = self.types.register_type(type_);
                            (Typing::Resolved(type_id), Some(definition_id))
                        }
                        Some(Definition::Event(_)) => {
                            // this is an event called as a function, which is valid in <0.5.0
                            (Typing::Resolved(self.types.void()), Some(definition_id))
                        }
                        _ => (Typing::Unresolved, None),
                    }
                }
                _ => {
                    // TODO(validation) SDR[41]: the operand did not resolve to a function
                    (Typing::Unresolved, None)
                }
            },
            Typing::Undetermined(type_ids) => {
                let receiver_type_id = self.type_id_of_value_receiver(&node.operand);
                let argument_typings = self.collect_named_argument_typings(arguments);
                let candidate = self.lookup_function_matching_named_arguments(
                    &type_ids,
                    &argument_typings,
                    receiver_type_id,
                );

                if let Some(candidate) = candidate {
                    let return_type = candidate.return_type;

                    let reference_node_id = reference_node_id_for_expression(&node.operand);
                    let definition_id = candidate.definition_id;
                    if let (Some(node_id), Some(definition_id)) = (reference_node_id, definition_id)
                    {
                        // TODO: maybe update the typing of the operand as well?
                        self.binder
                            .fixup_reference(node_id, Resolution::Definition(definition_id));
                    }

                    (Typing::Resolved(return_type), definition_id)
                } else {
                    (Typing::Unresolved, None)
                }
            }
            Typing::NewExpression(type_id) => {
                if let Type::Contract(ContractType { definition_id }) =
                    self.types.get_type_by_id(type_id)
                {
                    (Typing::Resolved(type_id), Some(*definition_id))
                } else {
                    // only contracts can be created with `new`
                    (Typing::Unresolved, None)
                }
            }
            Typing::BuiltIn(_) => {
                // built-ins cannot be called with named arguments
                (Typing::Unresolved, None)
            }
        };

        // Reference and resolve named arguments
        self.resolve_named_arguments(arguments, definition_id);

        typing
    }
}

// Given an expression node that resolves to a reference, return the node ID of
// the identifier of the reference. If the expression cannot be traced back to a
// single reference, return `None`.
fn reference_node_id_for_expression(node: &ir::Expression) -> Option<NodeId> {
    match &node {
        ir::Expression::MemberAccessExpression(f) => Some(f.member.id()),
        ir::Expression::Identifier(f) => Some(f.id()),
        ir::Expression::CallOptionsExpression(f) => reference_node_id_for_expression(&f.operand),
        _ => None,
    }
}

/// Typing functions for literals
impl Pass<'_> {
    pub(super) fn type_of_string_expression(node: &ir::StringExpression) -> Type {
        // Hex string literals carry distinct provenance (mirroring `HexInteger`
        // vs `Integer`); regular and unicode strings share `String` since they
        // are indistinguishable once decoded.
        let kind = match node {
            ir::StringExpression::StringLiterals(literals) => {
                let value = literals::value_of_string_literals(literals);
                LiteralKind::String { bytes: value.len() }
            }
            ir::StringExpression::HexStringLiterals(literals) => {
                let value = literals::value_of_hex_string_literals(literals);
                LiteralKind::HexString { bytes: value.len() }
            }
            ir::StringExpression::UnicodeStringLiterals(literals) => {
                let value = literals::value_of_unicode_string_literals(literals);
                LiteralKind::String { bytes: value.len() }
            }
        };
        Type::Literal(kind)
    }

    pub(super) fn hex_number_literal_kind(
        hex_number_expression: &ir::HexNumberExpression,
    ) -> Option<LiteralKind> {
        let mut hex_number = hex_number_expression.literal.unparse().to_owned();
        hex_number.retain(|character| character != '_');
        // Source-text byte width: `0x` prefix is stripped
        let digits = u32::try_from(hex_number.len().saturating_sub(2)).ok()?;
        if digits == 40 {
            // TODO(validation) SDR[38]: verify the address is valid (ie. has a valid checksum)
            // We need at least an implementation of SHA3 to compute the checksum

            // Skip `0x` prefix and parse the hexadecimal number.
            // `U160::from_str_radix` ignores `_` separators.
            let value = U160::from_str_radix(&hex_number[2..], 16).ok()?;
            return Some(LiteralKind::Address { value });
        }
        let value = Number::from_hex_number_expression(hex_number_expression)?
            .into_integer()
            .expect("hex literal must parse to an integer")
            .to_biguint()
            .expect("hex literal must be non-negative");
        // Each pair of hex digits is one byte (with odd digit counts rounded up).
        let bytes = digits.div_ceil(2).max(1);
        Some(LiteralKind::HexInteger { value, bytes })
    }
}
