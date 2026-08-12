// SPDX-License-Identifier: MIT
pragma solidity *;

// Companion to `yul_variable_shadows_blobhash`, pinned one version lower.
// `blobhash` is introduced in 0.8.24, so at 0.8.23 it is neither a Solidity
// global nor a Yul built-in on *any* EVM target: `let blobhash := 1` must be
// accepted everywhere. The sibling test pins 0.8.24 and shows the name becoming
// reserved from Cancun, so together they separate the two axes — this one proves
// the language-version gate alone frees the name, independently of the target.
contract C {
    function f() public pure {
        assembly {
            let blobhash := 1
        }
    }
}
