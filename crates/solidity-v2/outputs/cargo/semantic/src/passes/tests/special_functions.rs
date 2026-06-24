use slang_solidity_v2_common::diagnostics::{DiagnosticCollection, DiagnosticExtensions};
use slang_solidity_v2_common::evm_targets::EvmTarget;
use slang_solidity_v2_common::versions::LanguageVersion;
use slang_solidity_v2_ir::ir::NodeIdGenerator;

use super::build_file;
use crate::context::SemanticContext;

/// Runs every semantic pass over `contents` and returns the codes of every
/// diagnostic that was emitted, sorted for stable comparison. The fallback
/// shape checks run in the code-analysis pass (`p6`), so the whole pipeline
/// has to run for them to fire.
fn collect_diagnostic_codes(contents: &str) -> Vec<String> {
    let mut id_generator = NodeIdGenerator::default();
    let file = build_file(
        "test.sol",
        contents,
        &mut id_generator,
        LanguageVersion::LATEST,
    );

    let files = [file];
    let mut diagnostics = DiagnosticCollection::default();
    let _context = SemanticContext::build_from(
        LanguageVersion::LATEST,
        EvmTarget::LATEST,
        &files,
        &mut diagnostics,
    );

    let mut codes: Vec<String> = diagnostics
        .iter()
        .map(|diagnostic| diagnostic.code().to_owned())
        .collect();
    codes.sort();
    codes
}

#[test]
fn fallback_library_is_rejected() {
    let codes = collect_diagnostic_codes("library L { fallback() external {} }");
    assert_eq!(codes, ["structure/library-fallback-function"]);
}

#[test]
fn fallback_pure_mutability_is_rejected() {
    let codes = collect_diagnostic_codes("contract C { fallback() external pure {} }");
    assert_eq!(codes, ["type-system/fallback-function-mutability"]);
}

#[test]
fn fallback_view_mutability_is_rejected() {
    let codes = collect_diagnostic_codes("contract C { fallback() external view {} }");
    assert_eq!(codes, ["type-system/fallback-function-mutability"]);
}

#[test]
fn fallback_with_wrong_param_type_is_rejected() {
    let codes = collect_diagnostic_codes("contract C { fallback(uint256) external {} }");
    assert_eq!(codes, ["type-system/fallback-function-signature"]);
}

#[test]
fn fallback_with_param_only_is_rejected() {
    let codes =
        collect_diagnostic_codes("contract C { fallback(bytes calldata _input) external {} }");
    assert_eq!(codes, ["type-system/fallback-function-signature"]);
}

#[test]
fn fallback_with_return_only_is_rejected() {
    let codes = collect_diagnostic_codes(
        "contract C { fallback() external returns (bytes memory _output) {} }",
    );
    assert_eq!(codes, ["type-system/fallback-function-signature"]);
}

#[test]
fn fallback_with_param_wrong_location_is_rejected() {
    let codes = collect_diagnostic_codes(
        "contract C { fallback(bytes memory) external returns (bytes memory) {} }",
    );
    assert_eq!(codes, ["type-system/fallback-function-signature"]);
}

#[test]
fn fallback_with_return_wrong_location_is_rejected() {
    let codes = collect_diagnostic_codes(
        "contract C { fallback(bytes calldata) external returns (bytes calldata) {} }",
    );
    assert_eq!(codes, ["type-system/fallback-function-signature"]);
}

#[test]
fn fallback_with_multiple_returns_is_rejected() {
    let codes = collect_diagnostic_codes(
        "contract C { fallback() external returns (bytes memory, bytes memory) {} }",
    );
    assert_eq!(codes, ["type-system/fallback-function-signature"]);
}

#[test]
fn fallback_with_wrong_return_type_is_rejected() {
    let codes = collect_diagnostic_codes("contract C { fallback() external returns (uint256) {} }");
    assert_eq!(codes, ["type-system/fallback-function-signature"]);
}

#[test]
fn fallback_no_args_is_accepted() {
    assert!(collect_diagnostic_codes("contract C { fallback() external {} }").is_empty());
}

#[test]
fn fallback_with_args_is_accepted() {
    assert!(collect_diagnostic_codes(
        "contract C { fallback(bytes calldata) external returns (bytes memory) {} }"
    )
    .is_empty());
}

#[test]
fn fallback_payable_is_accepted() {
    assert!(collect_diagnostic_codes("contract C { fallback() external payable {} }").is_empty());
}
