//! Shape checks for the special `fallback` (and, in a sibling change, `receive`)
//! functions. These are structural rules that depend only on the function
//! definition and the kind of its enclosing container, so they are emitted
//! while collecting definitions rather than in a later, resolution-dependent
//! pass.
//!
//! Note that the v2 grammar already rejects most malformed special functions
//! as syntax errors (eg. `internal`/`public` visibility, or `view`/`pure`
//! mutability on a `receive`). Only the forms that parse cleanly need a
//! semantic check here.

use slang_solidity_v2_common::diagnostics::kinds::structure::{
    FallbackFunctionMutability, FallbackFunctionSignature, LibraryFallbackFunction,
};
use slang_solidity_v2_common::diagnostics::DiagnosticCollection;
use slang_solidity_v2_ir::ir;

/// Emits the structural diagnostics for a `fallback` function:
///
/// * libraries cannot declare a fallback function;
/// * a fallback must be `payable` or non-payable (not `pure`/`view`); and
/// * if it declares any parameters or returns, the signature must be exactly
///   `fallback(bytes calldata) returns (bytes memory)`.
///
/// The checks are independent, mirroring solc, so a single fallback can emit
/// more than one of them.
pub(super) fn check_fallback_function(
    node: &ir::FunctionDefinition,
    enclosing_is_library: bool,
    file_id: &str,
    diagnostics: &mut DiagnosticCollection,
) {
    if enclosing_is_library {
        diagnostics.push(
            file_id.to_owned(),
            node.range.clone(),
            LibraryFallbackFunction,
        );
    }

    match node.mutability {
        ir::FunctionMutability::Pure => diagnostics.push(
            file_id.to_owned(),
            node.range.clone(),
            FallbackFunctionMutability {
                mutability: "pure".to_owned(),
            },
        ),
        ir::FunctionMutability::View => diagnostics.push(
            file_id.to_owned(),
            node.range.clone(),
            FallbackFunctionMutability {
                mutability: "view".to_owned(),
            },
        ),
        ir::FunctionMutability::NonPayable | ir::FunctionMutability::Payable => {}
    }

    let has_parameters = !node.parameters.is_empty();
    let has_returns = node
        .returns
        .as_ref()
        .is_some_and(|returns| !returns.is_empty());

    // The signature rule only applies once the fallback declares parameters
    // and/or returns; a bare `fallback()` is always accepted.
    if (has_parameters || has_returns) && !is_accepted_fallback_with_args(node) {
        diagnostics.push(
            file_id.to_owned(),
            node.range.clone(),
            FallbackFunctionSignature,
        );
    }
}

/// Whether `node` matches the only accepted signature that carries arguments:
/// `fallback(bytes calldata) returns (bytes memory)`.
fn is_accepted_fallback_with_args(node: &ir::FunctionDefinition) -> bool {
    let [parameter] = node.parameters.as_slice() else {
        return false;
    };
    let Some([return_parameter]) = node.returns.as_deref() else {
        return false;
    };

    is_bytes_at(parameter, Location::CallData) && is_bytes_at(return_parameter, Location::Memory)
}

enum Location {
    CallData,
    Memory,
}

/// Whether `parameter` is the dynamic `bytes` type at the given data location.
fn is_bytes_at(parameter: &ir::Parameter, location: Location) -> bool {
    let is_dynamic_bytes = matches!(
        &parameter.type_name,
        ir::TypeName::ElementaryType(ir::ElementaryType::BytesKeyword(keyword))
            if keyword.unparse() == "bytes"
    );

    let location_matches = matches!(
        (&parameter.storage_location, location),
        (
            Some(ir::StorageLocation::CallDataKeyword(_)),
            Location::CallData
        ) | (
            Some(ir::StorageLocation::MemoryKeyword(_)),
            Location::Memory
        )
    );

    is_dynamic_bytes && location_matches
}
