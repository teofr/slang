use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when the second argument of `abi.decode` is not written
/// as a tuple of the types to decode into (eg. `abi.decode(data, uint)` instead
/// of `abi.decode(data, (uint))`).
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct AbiDecodeTypesNotATuple;

impl DiagnosticExtensions for AbiDecodeTypesNotATuple {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/abi-decode-types-not-a-tuple"
    }

    fn message(&self) -> String {
        "The second argument to \"abi.decode\" has to be a tuple of types.".to_owned()
    }
}
