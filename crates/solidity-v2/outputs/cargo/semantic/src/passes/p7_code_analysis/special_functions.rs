//! Shape checks for the special `fallback` function.
//!
//! A fallback's accepted state mutabilities and its accepted signatures are
//! properties of its type, and whether a library is allowed to declare one is a
//! structural rule. Either way the check needs only the function definition and
//! the kind of its enclosing container, so it runs here, over the collected
//! definitions.
//!
//! Note that the v2 grammar already rejects most malformed special functions
//! as syntax errors (eg. `internal`/`public` visibility, or `view`/`pure`
//! mutability on a `receive`). Only the forms that parse cleanly need a
//! semantic check here.

use slang_solidity_v2_common::diagnostics::kinds::structure::LibraryFallbackFunction;
use slang_solidity_v2_common::diagnostics::kinds::type_system::{
    FallbackFunctionMutability, FallbackFunctionSignature,
};
use slang_solidity_v2_common::diagnostics::DiagnosticCollection;
use slang_solidity_v2_ir::ir;

use crate::binder::{Binder, Definition};
use crate::context::FileNodeMapper;

/// Validates the shape of every `fallback` function in the program.
pub(crate) fn check_fallback_functions(
    binder: &Binder,
    file_node_mapper: &FileNodeMapper,
    diagnostics: &mut DiagnosticCollection,
) {
    for definition in binder.definitions().values() {
        let (members, enclosing_is_library) = match definition {
            Definition::Contract(contract) => (&contract.ir_node.members, false),
            Definition::Interface(interface) => (&interface.ir_node.members, false),
            Definition::Library(library) => (&library.ir_node.members, true),
            _ => continue,
        };

        for member in members {
            let ir::ContractMember::FunctionDefinition(function) = member else {
                continue;
            };

            if matches!(function.kind, ir::FunctionKind::Fallback) {
                let file_id = file_node_mapper.file_id_from_node_id(function.id());
                check_fallback_function(function, enclosing_is_library, file_id, diagnostics);
            }
        }
    }
}

/// Emits the shape diagnostics for a single `fallback` function:
///
/// * libraries cannot declare a fallback function;
/// * a fallback must be `payable` or non-payable (not `pure`/`view`); and
/// * if it declares any parameters or returns, the signature must be exactly
///   `fallback(bytes calldata) returns (bytes memory)`.
///
/// The checks are independent, mirroring solc, so a single fallback can emit
/// more than one of them.
fn check_fallback_function(
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
