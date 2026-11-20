use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when an assignment targets a location in storage whose
/// type is a mapping or contains one (eg. a struct with a mapping member). A
/// mapping cannot be copied, so only a local storage pointer to such a type
/// can be assigned to.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct AssignmentToTypeWithMapping;

impl DiagnosticExtensions for AssignmentToTypeWithMapping {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/assignment-to-type-with-mapping"
    }

    fn message(&self) -> String {
        "Types in storage containing (nested) mappings cannot be assigned to.".to_owned()
    }
}
