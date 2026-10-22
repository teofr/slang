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

/// Ends a repetition of an enum early, leaving the rest of the input to the enclosing item.
///
/// `after` and `before` are terminal variants of that enum. Once an `after` item has been
/// consumed, the repetition stops before (without consuming) the next `before` item:
///
/// ```ebnf
/// (Item - after)* (after (Item - before)*)?
/// ```
///
/// It only changes where the repetition ends: the CST is still a flat list of items.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
pub struct RepeatedStopBefore {
    pub after: Vec<Identifier>,
    pub before: Vec<Identifier>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<VersionSpecifier>,
}
