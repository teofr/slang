// SPDX-License-Identifier: MIT
pragma solidity *;

// `blobhash` is both a Solidity global (from 0.8.24) and a Yul built-in (from
// Cancun). At 0.8.23 it is neither yet, so solc accepts the declaration below
// on every target it supports. slang, however, treats `blobhash` as a Solidity
// global unconditionally — its global lookup is not gated by version or target
// — and so rejects the declaration as shadowing on every target, diverging from
// solc wherever solc supports the target (see `expected_solc_divergence`). This
// pins that ungated-lookup behaviour one release before the built-in exists.
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
