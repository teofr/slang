//! Shared rendering of semantic [`Type`]s into the human-readable strings used
//! by the snapshot reports (e.g. `binder_output`, `typing_output`).
//!
//! Keeping this in one place ensures every snapshot kind renders types
//! identically, so their golden files don't drift apart.

use slang_solidity_v2::ast::{DataLocation, Definition, LiteralKind, Type};

/// Renders an optional type, falling back to `"unresolved"` when the typing
/// pass recorded no type for the node.
pub(crate) fn type_or_unresolved(type_: Option<Type>) -> String {
    type_.map_or_else(|| "unresolved".to_string(), |type_| type_display(&type_))
}

pub(crate) fn definition_name(definition: &Definition) -> String {
    definition.identifier().name().to_string()
}

/// Returns the value type a definition declares, or `None` for definitions that
/// have no value type of their own (eg. contracts, structs, enums, imports).
///
/// This is the single source of truth for mapping a definition to its type,
/// shared by `binder_output` (in its definitions list) and `typing_output`
/// (annotating definition sites).
pub(crate) fn definition_value_type(definition: &Definition) -> Option<Type> {
    match definition {
        Definition::Constant(constant) => constant.get_type(),
        Definition::EnumMember(enum_member) => enum_member.get_type(),
        Definition::Function(function) => function.get_type(),
        Definition::Parameter(parameter) => parameter.get_type(),
        Definition::StateVariable(state_variable) => state_variable.get_type(),
        Definition::StructMember(struct_member) => struct_member.get_type(),
        Definition::Variable(variable) => variable.get_type(),
        _ => None,
    }
}

#[allow(clippy::too_many_lines)]
pub(crate) fn type_display(type_: &Type) -> String {
    match type_ {
        Type::Address(address) => {
            if address.is_payable() {
                "address payable".to_string()
            } else {
                "address".to_string()
            }
        }
        Type::Array(array) => format!(
            "{element_type}[] {location}",
            element_type = type_display(&array.element_type()),
            location = data_location_display(array.location()),
        ),
        Type::Boolean(_) => "bool".to_string(),
        Type::ByteArray(byte_array) => format!("bytes{width}", width = byte_array.width()),
        Type::Bytes(bytes) => {
            format!(
                "bytes {location}",
                location = data_location_display(bytes.location())
            )
        }
        Type::Contract(contract) => definition_name(&contract.definition()),
        Type::Enum(enum_) => definition_name(&enum_.definition()),
        Type::FixedPointNumber(fixed) => {
            format!(
                "{signed}fixed{bits}x{precision_bits}",
                signed = if fixed.is_signed() { "" } else { "u" },
                bits = fixed.bits(),
                precision_bits = fixed.decimal_places(),
            )
        }
        Type::FixedSizeArray(fixed_size_array) => format!(
            "{element_type}[{size}] {location}",
            element_type = type_display(&fixed_size_array.element_type()),
            size = fixed_size_array.size(),
            location = data_location_display(fixed_size_array.location()),
        ),
        Type::Function(function) => {
            format!(
                "function ({parameters}) returns {returns}",
                parameters = function
                    .parameter_types()
                    .iter()
                    .map(type_display)
                    .collect::<Vec<_>>()
                    .join(", "),
                returns = type_display(&function.return_type()),
            )
        }
        Type::Integer(integer) => {
            format!(
                "{signed}int{bits}",
                signed = if integer.is_signed() { "" } else { "u" },
                bits = integer.bits(),
            )
        }
        Type::Interface(interface) => definition_name(&interface.definition()),
        Type::Library(library) => definition_name(&library.definition()),
        Type::Literal(literal) => match literal.kind() {
            LiteralKind::Integer { value } => format!("lit-integer({value})"),
            LiteralKind::HexInteger { value, bytes } => {
                format!("lit-hex({value}, {bytes})")
            }
            LiteralKind::Rational { value } => format!("lit-rational({value})"),
            LiteralKind::HexString { bytes } => format!("lit-hexstring({bytes})"),
            LiteralKind::String { bytes } => format!("lit-string({bytes})"),
            LiteralKind::Address { value } => format!("lit-address({value})"),
        },
        Type::Mapping(mapping) => {
            format!(
                "{key} => {value}",
                key = type_display(&mapping.key_type()),
                value = type_display(&mapping.value_type()),
            )
        }
        // A meta-type is the typing of an expression that names a type rather
        // than a value (eg. `uint` in `uint(x)`, or a contract name used as a
        // qualifier). Definition reports never produce these, but expression
        // reports do, so render them as `type(<referenced type>)`.
        Type::MetaType(meta) => format!("type({inner})", inner = type_display(&meta.meta_type())),
        Type::UserMetaType(meta) => {
            format!("type({name})", name = definition_name(&meta.definition()))
        }
        Type::String(string) => {
            format!(
                "string {location}",
                location = data_location_display(string.location())
            )
        }
        Type::Struct(struct_) => {
            format!(
                "{name} {location}",
                name = definition_name(&struct_.definition()),
                location = data_location_display(struct_.location()),
            )
        }
        Type::Tuple(tuple) => {
            format!(
                "({types})",
                types = tuple
                    .types()
                    .iter()
                    .map(type_display)
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        }
        Type::UserDefinedValue(user_defined_value) => {
            definition_name(&user_defined_value.definition())
        }
        Type::Void(_) => "void".to_string(),
    }
}

pub(crate) fn data_location_display(location: DataLocation) -> &'static str {
    match location {
        DataLocation::Memory => "memory",
        DataLocation::Storage => "storage",
        DataLocation::Calldata => "calldata",
        DataLocation::Inherited => "(inherited)",
    }
}
