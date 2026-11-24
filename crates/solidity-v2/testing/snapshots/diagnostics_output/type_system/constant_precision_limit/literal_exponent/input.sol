// SPDX-License-Identifier: MIT
pragma solidity *;

// A literal's exponent is checked before computing `10 ** 400000000`.
contract C {
    uint256[1e400000000] data;
}
