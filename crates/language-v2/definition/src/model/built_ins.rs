use language_v2_internal_macros::{ParseInputTokens, WriteOutputTokens, derive_spanned_type};
use serde::{Deserialize, Serialize};

use crate::model::{Code, EvmTargetSpecifier, Identifier, VersionSpecifier};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
pub struct BuiltInContext {
    pub name: Identifier,
    pub scopes: Vec<BuiltInScope>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
pub struct BuiltInScope {
    pub name: Identifier,
    pub definitions: Vec<BuiltInDefinition>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
pub struct BuiltInDefinition {
    pub name: Identifier,

    /// The version range this built-in is available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<VersionSpecifier>,

    /// The EVM target range this built-in is available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evm_enabled: Option<EvmTargetSpecifier>,

    /// A verbatim Rust type to use in the definition of the variant in the
    /// internal enum if required for correct resolution or typing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_parameter: Option<Code>,
}

/// A name that is reserved in Yul inline assembly but is *not* an available
/// built-in function — so it may not be used as a Yul declaration, yet has no
/// signature to resolve to. These are the opcode mnemonics that never became
/// Yul built-ins (`dup1`, `push0`, `jump`, ...), the object-access names
/// (`datasize`, `linkersymbol`, ...), and names promoted to reserved after they
/// stopped being available (`difficulty` from Paris on).
///
/// Unlike keyword reservation, this is a *semantic* fact consumed in
/// `p6_resolve_yul`: the name still lexes and parses as an ordinary
/// `YulIdentifier` (preserving the grammar's version-independence and error
/// recovery), and the reservation is reported as a resolution diagnostic rather
/// than a parse error.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
pub struct YulReservedWord {
    /// The reserved name, written as the identifier itself (e.g. `dup1`).
    pub name: Identifier,

    /// The version range the name is reserved. Defaults to always.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<VersionSpecifier>,

    /// The EVM target range the name is reserved. Defaults to always.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evm_enabled: Option<EvmTargetSpecifier>,
}
