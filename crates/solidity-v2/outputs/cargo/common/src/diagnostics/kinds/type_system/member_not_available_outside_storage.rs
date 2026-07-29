use serde::Serialize;

use crate::diagnostics::extensions::DiagnosticExtensions;
use crate::diagnostics::severity::DiagnosticSeverity;

/// Diagnostic emitted when a storage-only member of a dynamic array or `bytes`
/// value (ie. `.push` / `.pop`) is accessed on a value whose data location is
/// `memory` or `calldata`. Those members mutate the backing storage, so they
/// are only available on storage arrays; memory and calldata arrays expose just
/// `.length`. Mirrors solc's `TypeError 4994`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct MemberNotAvailableOutsideStorage {
    /// The offending member name, as written (eg. `"push"` or `"pop"`).
    pub member: String,
}

impl DiagnosticExtensions for MemberNotAvailableOutsideStorage {
    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn code(&self) -> &'static str {
        "type-system/member-not-available-outside-storage"
    }

    fn message(&self) -> String {
        format!(
            "Member \"{member}\" is not available outside of storage.",
            member = self.member
        )
    }
}
