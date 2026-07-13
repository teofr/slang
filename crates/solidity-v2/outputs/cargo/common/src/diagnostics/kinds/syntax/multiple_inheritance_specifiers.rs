use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a contract header carries more than one inheritance
/// (`is`) specifier list.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct MultipleInheritanceSpecifiers;

impl DiagnosticExtensions for MultipleInheritanceSpecifiers {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "syntax/multiple-inheritance-specifiers"
    }

    fn message(&self) -> String {
        "Only a single inheritance specifier list can be provided.".to_string()
    }
}
