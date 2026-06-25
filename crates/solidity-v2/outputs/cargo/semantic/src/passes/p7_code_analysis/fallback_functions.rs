//! Shape checks for the special `fallback` function.
//!
//! A fallback's accepted state mutabilities and its accepted signatures are
//! properties of its resolved type, and whether a library may declare one is a
//! structural rule. The checks run here, in the code-analysis pass, over the
//! fully resolved program.
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
use crate::types::{
    BytesType, DataLocation, FunctionType, FunctionTypeMutability, Type, TypeRegistry,
};

/// Validates the shape of every `fallback` function in the program.
pub(crate) fn check_fallback_functions(
    binder: &Binder,
    types: &TypeRegistry,
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
                check_fallback_function(
                    function,
                    enclosing_is_library,
                    binder,
                    types,
                    file_id,
                    diagnostics,
                );
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
    binder: &Binder,
    types: &TypeRegistry,
    file_id: &str,
    diagnostics: &mut DiagnosticCollection,
) {
    // Whether a library is allowed to declare a fallback is a property of the
    // enclosing container, not of the function itself.
    if enclosing_is_library {
        diagnostics.push(
            file_id.to_owned(),
            node.range.clone(),
            LibraryFallbackFunction,
        );
    }

    // The mutability and signature rules are properties of the function's type.
    let Some(function_type) = fallback_function_type(node, binder, types) else {
        return;
    };

    match function_type.mutability {
        FunctionTypeMutability::Pure => diagnostics.push(
            file_id.to_owned(),
            node.range.clone(),
            FallbackFunctionMutability {
                mutability: "pure".to_owned(),
            },
        ),
        FunctionTypeMutability::View => diagnostics.push(
            file_id.to_owned(),
            node.range.clone(),
            FallbackFunctionMutability {
                mutability: "view".to_owned(),
            },
        ),
        FunctionTypeMutability::NonPayable | FunctionTypeMutability::Payable => {}
    }

    let has_parameters = !function_type.parameter_types.is_empty();
    let has_returns = !matches!(types.get_type_by_id(function_type.return_type), Type::Void);

    // The signature rule only applies once the fallback declares parameters
    // and/or returns; a bare `fallback()` is always accepted.
    if (has_parameters || has_returns) && !is_accepted_fallback_with_args(function_type, types) {
        diagnostics.push(
            file_id.to_owned(),
            node.range.clone(),
            FallbackFunctionSignature,
        );
    }
}

/// Recovers the [`FunctionType`] computed for `node` during type definition.
fn fallback_function_type<'a>(
    node: &ir::FunctionDefinition,
    binder: &Binder,
    types: &'a TypeRegistry,
) -> Option<&'a FunctionType> {
    let type_id = binder.node_typing(node.id()).as_type_id()?;
    match types.get_type_by_id(type_id) {
        Type::Function(function_type) => Some(function_type),
        _ => None,
    }
}

/// Whether `function_type` matches the only accepted signature that carries
/// arguments: `fallback(bytes calldata) returns (bytes memory)`.
fn is_accepted_fallback_with_args(function_type: &FunctionType, types: &TypeRegistry) -> bool {
    let [parameter_type_id] = function_type.parameter_types.as_slice() else {
        return false;
    };

    is_bytes_at(types.get_type_by_id(*parameter_type_id), DataLocation::Calldata)
        && is_bytes_at(
            types.get_type_by_id(function_type.return_type),
            DataLocation::Memory,
        )
}

/// Whether `ty` is the dynamic `bytes` type at the given data location.
fn is_bytes_at(ty: &Type, location: DataLocation) -> bool {
    matches!(ty, Type::Bytes(BytesType { location: actual }) if *actual == location)
}
