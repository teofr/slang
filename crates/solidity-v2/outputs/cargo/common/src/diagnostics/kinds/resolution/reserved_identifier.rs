use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a Yul declaration (variable, function, or function
/// parameter/return) uses a name that is reserved in inline assembly but is not
/// an available built-in — e.g. an opcode mnemonic like `dup1`, `push0`, or
/// `jump`, an object-access name like `datasize`, or a name that has been
/// promoted to reserved (like `difficulty` from Paris on).
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ReservedIdentifier {
    /// The reserved name that was used as a declaration.
    pub name: String,
}

impl DiagnosticExtensions for ReservedIdentifier {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "resolution/reserved-identifier"
    }

    fn message(&self) -> String {
        format!(
            "The identifier '{}' is reserved and can not be used.",
            self.name
        )
    }
}
