// SPDX-License-Identifier: MIT
pragma solidity *;

// `blobhash` is both a Solidity global (from 0.8.24) and a Yul built-in (from
// Cancun). At 0.8.24 slang rejects the declaration below on every target, but
// via two different mechanisms: pre-Cancun it shadows the Solidity global
// (`external-declaration-shadowing`), and from Cancun on it redeclares the Yul
// built-in (`built-in-redeclaration`). solc only agrees on the Cancun-onward
// targets, where `blobhash` is a Yul built-in; on the earlier targets it
// accepts the declaration, so slang diverges there (see
// `expected_solc_divergence`). Because slang's Solidity-global lookup is not
// gated by target, the reservation this test would otherwise pin is masked by
// the shadowing rejection until Cancun.
//
// The declared name is deliberately never referenced: a reference would resolve
// to the built-in rather than the variable and report an availability error
// instead, which would mask whether the declaration itself was rejected.
contract C {
    function f() public pure {
        assembly {
            let blobhash := 1
        }
    }
}
