// ---- path: main.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

import "./lib.sol" as M;

// Accepted: a module as a statement expression is inert.

function f() pure {
    M;
}

// ---- path: lib.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

uint256 constant K = 1;
