use language_internal_macros::{ParseInputTokens, WriteOutputTokens, derive_spanned_type};
use serde::{Deserialize, Serialize};

use crate::model::{Identifier, VersionSpecifier};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
pub struct RepeatedItem {
    pub name: Identifier,
    pub reference: Identifier,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<VersionSpecifier>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_empty: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_before: Option<RepeatedStopBefore>,
}

/// Ends the repetition early, leaving the rest of the input to the enclosing item.
///
/// Once an item that starts with one of the `after` terminals has been consumed,
/// the repetition stops before (without consuming) the next item that starts with
/// one of the `before` terminals.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
pub struct RepeatedStopBefore {
    pub after: Vec<Identifier>,
    pub before: Vec<Identifier>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<VersionSpecifier>,
}
