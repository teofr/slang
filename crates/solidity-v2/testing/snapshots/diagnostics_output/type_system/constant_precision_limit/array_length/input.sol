// SPDX-License-Identifier: MIT
pragma solidity *;

// Folding stops at the first operator whose result needs more than 4096 bits,
// rather than computing `2 ** 4294967295` in full.
contract C {
    uint256[2 ** 4294967295 - 2 ** 4294967295 + 1] data;
}
