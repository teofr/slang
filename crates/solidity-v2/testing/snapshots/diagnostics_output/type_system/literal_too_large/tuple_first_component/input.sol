// SPDX-License-Identifier: MIT
pragma solidity *;

// The first component of the tuple is too large for any integer type.

contract C {
    function f() public pure {
        uint256 x;
        (x, ) = (1E111, 1);
    }
}
