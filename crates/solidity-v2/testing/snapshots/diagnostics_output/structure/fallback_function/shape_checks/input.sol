// SPDX-License-Identifier: MIT
pragma solidity *;

// Consolidated coverage for the fallback-function shape checks that are
// representable in Slang v2 (ie. that parse cleanly and so require a semantic
// check rather than being rejected by the grammar):
//
//   structure/library-fallback-function    - libraries cannot have a fallback
//   structure/fallback-function-mutability  - fallback must be payable/non-payable
//   structure/fallback-function-signature   - signature must be `fallback()` or
//                                             `fallback(bytes calldata) returns (bytes memory)`
//
// The visibility rule (`fallback` must be `external`) is enforced by the v2
// grammar itself: `internal`/`private`/`public` are not accepted attributes,
// so those forms are syntax errors and are covered by the parser tests.

// Libraries cannot have fallback functions.
library L {
    fallback() external {}
}

// Mutability must be payable or non-payable; `pure`/`view` are rejected.
contract MutPure {
    uint x;
    fallback() external pure { x = 2; }
}
contract MutView {
    uint x;
    fallback() external view { x = 2; }
}

// Signature violations. The only accepted forms are `fallback()` and
// `fallback(bytes calldata) returns (bytes memory)`.
contract SigParamWrongType {
    fallback(uint256) external {}
}
contract SigParamOnly {
    fallback(bytes calldata _input) external {}
}
contract SigReturnOnly {
    fallback() external returns (bytes memory _output) {}
}
contract SigParamWrongLocation {
    fallback(bytes memory) external returns (bytes memory) {}
}
contract SigReturnWrongLocation {
    fallback(bytes calldata) external returns (bytes calldata) {}
}
contract SigMultipleReturns {
    fallback() external returns (bytes memory, bytes memory) {}
}
contract SigReturnWrongType {
    fallback() external returns (uint256) {}
}

// Accepted signatures (no diagnostics).
contract ValidNoArgs {
    fallback() external {}
}
contract ValidWithArgs {
    fallback(bytes calldata) external returns (bytes memory) {}
}
