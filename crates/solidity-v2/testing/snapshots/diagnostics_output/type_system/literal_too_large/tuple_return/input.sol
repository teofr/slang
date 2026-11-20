// SPDX-License-Identifier: MIT
pragma solidity *;

// A returned tuple is built before it is returned.

contract C {
    function f() public pure returns (uint256, uint256) {
        return (1, 2**270);
    }
}
