// SPDX-License-Identifier: MIT
pragma solidity *;

// `jump` is an EVM opcode mnemonic that never became a Yul built-in, so it is
// reserved as a Yul identifier on every version and target (solc error 5017),
// yet has no built-in signature to resolve to.
//
// The declared name is deliberately never referenced: a reference would resolve
// to the built-in rather than the variable and report an availability error
// instead, which would mask whether the declaration itself was rejected.
contract C {
    function f() public pure {
        assembly {
            let jump := 1
        }
    }
}
