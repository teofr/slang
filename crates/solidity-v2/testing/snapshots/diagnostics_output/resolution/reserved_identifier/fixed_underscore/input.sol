// SPDX-License-Identifier: MIT
pragma solidity *;

// `_` is a fixed reserved word in inline assembly (solc error 4113), reserved
// on every version and target and for every declaration kind. Unlike `this`
// and `super` (which the grammar tokenises as keywords, so they fail to parse
// as identifiers), `_` parses as an ordinary Yul identifier and so is rejected
// semantically in `p6_resolve_yul`.
//
// The declared name is deliberately never referenced: a reference would resolve
// to the built-in rather than the variable and report an availability error
// instead, which would mask whether the declaration itself was rejected.
contract C {
    function f() public pure {
        assembly {
            let _ := 1
        }
    }
}
