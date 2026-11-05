// ---- path: main.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

import "./lib.sol" as M;

// Both branches name the same module. solc gives a module itself as its mobile
// type and never asks a conditional for a nameable type, so it accepts this.
// Slang gives no module a mobile type, and reports both branches.

function f() pure {
    true ? M : M;
}

// ---- path: lib.sol
// SPDX-License-Identifier: MIT
pragma solidity *;

uint256 constant K = 1;
