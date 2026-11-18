use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a single byte of a fixed-size byte array (eg.
/// `bytes32`) is written to. Indexing one copies the byte out of the word
/// holding it, so there is nothing to modify in place.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct WriteToFixedBytesElement;

impl DiagnosticExtensions for WriteToFixedBytesElement {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/write-to-fixed-bytes-element"
    }

    fn message(&self) -> String {
        "Single bytes in fixed bytes arrays cannot be modified.".to_owned()
    }
}
