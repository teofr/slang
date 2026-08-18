// SPDX-License-Identifier: MIT
pragma solidity *;

// The `push`/`dup`/`swap` opcode families are reserved the same way, but only
// for the indices that name an actual opcode; see
// `lookalikes_are_not_reserved` for the neighbouring names that stay valid
// identifiers.
contract C {
    function f() public pure {
        assembly {
            let dup16 := 1
        }
    }
}
