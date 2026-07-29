// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

// Migrated (partially) from the `meta_type` AST unit test: expressions that
// name a type rather than a value carry a meta-type, rendered here as
// `type(<referenced type>)`.
contract C {
    struct S {
        uint256 a;
    }

    function f(uint256 x, bytes memory b) internal pure {
        S(x);
        abi.decode(b, (uint256[]));
        uint256(x);
    }
}
