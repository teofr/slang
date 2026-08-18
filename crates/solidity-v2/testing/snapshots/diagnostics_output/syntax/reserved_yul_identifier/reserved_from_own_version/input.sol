// SPDX-License-Identifier: MIT
pragma solidity *;

// Reservation follows the set of opcodes solc knows about, which grows over
// time: `push0` arrives with Shanghai support in `0.8.20`, and before that the
// name is an ordinary Yul identifier. That is a language-version boundary, so
// the keyword's `reserved` specifier expresses it directly.
contract C {
    function f() public pure {
        assembly {
            let push0 := 1
        }
    }
}
