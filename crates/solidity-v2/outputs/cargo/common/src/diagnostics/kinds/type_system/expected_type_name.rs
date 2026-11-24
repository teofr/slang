use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when an expression is used where a type name is
/// required but it is a value instead: one of the types `abi.decode` decodes
/// into (eg. the `5` in `abi.decode(data, (uint, 5))`).
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ExpectedTypeName;

impl DiagnosticExtensions for ExpectedTypeName {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/expected-type-name"
    }

    fn message(&self) -> String {
        "Argument has to be a type name.".to_owned()
    }
}
