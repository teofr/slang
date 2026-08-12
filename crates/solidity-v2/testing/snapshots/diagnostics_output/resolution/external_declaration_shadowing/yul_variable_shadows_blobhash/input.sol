// SPDX-License-Identifier: MIT
pragma solidity *;

// `blobhash` becomes a Solidity global and a Yul built-in only from
// 0.8.24/Cancun. Before Cancun it is neither, so `let blobhash := 1` is a valid
// Yul identifier and must be accepted; from Cancun on it is rejected as a
// built-in redeclaration. Gating the Solidity-global shadow check by
// version/target is what stops slang from wrongly rejecting the pre-Cancun
// declaration (the gap the closed #45 surfaced). Pinned to 0.8.24 to stay below
// the 0.8.35 future-reserved warning, which slang does not yet model.
contract C {
    function f() public pure {
        assembly {
            let blobhash := 1
        }
    }
}
