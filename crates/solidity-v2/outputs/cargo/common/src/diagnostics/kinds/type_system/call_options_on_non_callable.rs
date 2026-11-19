use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when call options (eg. `{value: 1}`) are applied to an
/// expression that is not a function: a value of another type, or a type name
/// (eg. `x{value: 1}`, `uint{value: 1}`).
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CallOptionsOnNonCallable;

impl DiagnosticExtensions for CallOptionsOnNonCallable {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/call-options-on-non-callable"
    }

    fn message(&self) -> String {
        "Expected callable expression before call options.".to_owned()
    }
}
