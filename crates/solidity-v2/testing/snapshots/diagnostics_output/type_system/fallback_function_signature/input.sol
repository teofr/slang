// SPDX-License-Identifier: MIT
pragma solidity *;

// Declaring parameters without the accepted signature is invalid.
contract A {
    fallback(uint256) external {}
}

// Declaring returns without the accepted signature is invalid.
contract B {
    fallback() external returns (uint256) {}
}

// The two accepted forms are valid and must not be flagged:
// `fallback()` and `fallback(bytes calldata) returns (bytes memory)`.
contract C {
    fallback() external {}
}

contract D {
    fallback(bytes calldata) external returns (bytes memory) {}
}
