use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a non-anonymous event declares more than 3 indexed
/// arguments. A non-anonymous event reserves topic 0 for its signature,
/// leaving 3 topics for indexed arguments.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TooManyEventIndexedArguments;

impl DiagnosticExtensions for TooManyEventIndexedArguments {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/too-many-event-indexed-arguments"
    }

    fn message(&self) -> String {
        "More than 3 indexed arguments for event.".to_string()
    }
}
