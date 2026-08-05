// SPDX-License-Identifier: MIT
pragma solidity *;

// `push0` entered solc's instruction table as a reserved-only name (it never
// became a Yul built-in). Iterating language versions pins the version from
// which the name becomes reserved.
//
// The declared name is deliberately never referenced: a reference would resolve
// to the built-in rather than the variable and report an availability error
// instead, which would mask whether the declaration itself was rejected.
contract C {
    function f() public pure {
        assembly {
            let push0 := 1
        }
    }
}
