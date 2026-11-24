use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// What an expression with no value is a component of.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub enum VoidComponentKind {
    /// A tuple that is read, or a pair of parentheses.
    Tuple,
    /// An array literal.
    Array,
}

/// Diagnostic emitted when a component of a tuple that is read, or an item of
/// an array literal, has no value: a call to a function that returns nothing,
/// or an empty tuple (eg. `(f(), 1)` or `[f()]`, where `f` returns nothing).
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct VoidComponent {
    /// What the expression is a component of.
    pub kind: VoidComponentKind,
}

impl DiagnosticExtensions for VoidComponent {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/void-component"
    }

    fn message(&self) -> String {
        match self.kind {
            VoidComponentKind::Tuple => "Tuple component cannot be empty.",
            VoidComponentKind::Array => "Array component cannot be empty.",
        }
        .to_owned()
    }
}
