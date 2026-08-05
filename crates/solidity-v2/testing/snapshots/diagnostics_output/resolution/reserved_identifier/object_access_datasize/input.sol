// SPDX-License-Identifier: MIT
pragma solidity *;

// `datasize` is an object-access name: reserved in inline assembly (solc error
// 5017) but not available as a built-in there, on every version and target.
//
// The declared name is deliberately never referenced: a reference would resolve
// to the built-in rather than the variable and report an availability error
// instead, which would mask whether the declaration itself was rejected.
contract C {
    function f() public pure {
        assembly {
            let datasize := 1
        }
    }
}
