use std::ops::Range;

use slang_solidity_v2_semantic::types::literals;

use super::super::{StringExpression, Type};
use crate::ast::{HexStringLiteralsStruct, StringLiteralsStruct, UnicodeStringLiteralsStruct};

/// Computes the byte range spanning a sequence of literal tokens, from the
/// start of the first to the end of the last. Falls back to an empty range for
/// an empty sequence (which the grammar does not produce).
macro_rules! literals_range {
    ($literals:expr) => {{
        let start = $literals
            .iter()
            .next()
            .map_or(0, |literal| literal.get_text_range().start);
        let end = $literals
            .iter()
            .last()
            .map_or(0, |literal| literal.get_text_range().end);
        start..end
    }};
}

impl StringExpression {
    /// Returns the type assigned to this string expression by the typing pass.
    ///
    /// The typing pass binds the resolved type to the first literal in the
    /// sequence; this accessor delegates to that literal. Returns `None` when
    /// the sequence is empty or the typing pass did not record a type.
    pub fn get_type(&self) -> Option<Type> {
        match self {
            StringExpression::StringLiterals(literals) => literals.iter().next()?.get_type(),
            StringExpression::HexStringLiterals(literals) => literals.iter().next()?.get_type(),
            StringExpression::UnicodeStringLiterals(literals) => literals.iter().next()?.get_type(),
        }
    }

    /// Returns the concatenated decoded string value as bytes.
    pub fn value(&self) -> Vec<u8> {
        match self {
            StringExpression::StringLiterals(literals) => literals.value(),
            StringExpression::HexStringLiterals(literals) => literals.value(),
            StringExpression::UnicodeStringLiterals(literals) => literals.value(),
        }
    }

    /// Returns the byte range this string expression spans, covering the whole
    /// sequence of adjacent string literals.
    pub fn get_text_range(&self) -> Range<usize> {
        match self {
            StringExpression::StringLiterals(literals) => literals_range!(literals),
            StringExpression::HexStringLiterals(literals) => literals_range!(literals),
            StringExpression::UnicodeStringLiterals(literals) => literals_range!(literals),
        }
    }
}

impl StringLiteralsStruct {
    pub fn value(&self) -> Vec<u8> {
        literals::value_of_string_literals(&self.ir_nodes)
    }
}

impl HexStringLiteralsStruct {
    pub fn value(&self) -> Vec<u8> {
        literals::value_of_hex_string_literals(&self.ir_nodes)
    }
}

impl UnicodeStringLiteralsStruct {
    pub fn value(&self) -> Vec<u8> {
        literals::value_of_unicode_string_literals(&self.ir_nodes)
    }
}
