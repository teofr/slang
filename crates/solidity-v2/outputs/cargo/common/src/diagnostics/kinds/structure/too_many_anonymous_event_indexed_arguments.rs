use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when an anonymous event declares more than 4 indexed
/// arguments. An anonymous event has no signature topic, so all 4 topics are
/// available for indexed arguments.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TooManyAnonymousEventIndexedArguments;

impl DiagnosticExtensions for TooManyAnonymousEventIndexedArguments {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "structure/too-many-anonymous-event-indexed-arguments"
    }

    fn message(&self) -> String {
        "More than 4 indexed arguments for anonymous event.".to_string()
    }
}
