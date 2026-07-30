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

    /// The version range this built-in's *name* is reserved, i.e. can't be used
    /// as a Yul identifier. Defaults to always, and is only narrower than
    /// `enabled` for names `solc` has not promoted to reserved identifiers yet:
    /// those may still be declared before the built-in exists, where `solc`
    /// merely warns that the name will be promoted in the future.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reserved: Option<VersionSpecifier>,

    /// The EVM target range this built-in's *name* is reserved. See `reserved`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evm_reserved: Option<EvmTargetSpecifier>,

    /// A verbatim Rust type to use in the definition of the variant in the
    /// internal enum if required for correct resolution or typing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_parameter: Option<Code>,
}
