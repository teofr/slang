// ---- path: main.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

import "./lib.sol" as M;

// Reported: a module cannot be an element of an array literal. solc gives a
// module itself as its mobile type, and then rejects it as not nameable.

function f() pure {
    [M];
}

// ---- path: lib.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

uint256 constant K = 1;
