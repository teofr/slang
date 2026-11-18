use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when the `length` member of an array is written to. It
/// is read-only: an array is resized through `push` and `pop` instead.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct WriteToArrayLength;

impl DiagnosticExtensions for WriteToArrayLength {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/write-to-array-length"
    }

    fn message(&self) -> String {
        "Member \"length\" is read-only and cannot be used to resize arrays.".to_owned()
    }
}
