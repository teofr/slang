use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a contract header carries more than one storage
/// layout (`layout at`) specifier.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct MultipleStorageLayoutSpecifiers;

impl DiagnosticExtensions for MultipleStorageLayoutSpecifiers {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "syntax/multiple-storage-layout-specifiers"
    }

    fn message(&self) -> String {
        "Only a single storage layout specifier can be provided.".to_string()
    }
}
