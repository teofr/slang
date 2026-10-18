use language_v2_internal_macros::{ParseInputTokens, WriteOutputTokens, derive_spanned_type};
use serde::{Deserialize, Serialize};

use crate::model::{Identifier, TokenScanner};

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
pub struct TriviaItem {
    pub name: Identifier,

    /// What the trivia is, whitespace if unset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<TriviaCategory>,

    /// Whether the lexer skips the trivia, never producing a lexeme for it, `false` if unset
    ///
    /// `NatSpec` comments can't be skipped, the parser reads them.
    ///
    /// TODO(v2): Skipping is temporary: once trivia is part of the CST, the lexer has to produce it
    /// again, or the CST has to re-lex it from the gaps between tokens.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skipped: Option<bool>,

    pub scanner: TokenScanner,
}

impl TriviaItem {
    /// What the trivia is, defaulting to whitespace.
    pub fn category(&self) -> TriviaCategory {
        self.category.unwrap_or_default()
    }

    /// Whether the lexer skips the trivia, never producing a lexeme for it.
    pub fn is_skipped(&self) -> bool {
        self.skipped.unwrap_or(false)
    }
}

/// What a trivia item is.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[derive_spanned_type(Clone, Debug, ParseInputTokens, WriteOutputTokens)]
pub enum TriviaCategory {
    /// Whitespace, including line breaks.
    #[default]
    Whitespace,
    /// A regular comment, like `// ...` or `/* ... */`.
    RegularComment,
    /// A `NatSpec` comment, documenting the code that follows it, like `/// ...` or `/** ... */`.
    NatSpecComment,
}
