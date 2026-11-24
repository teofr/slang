// SPDX-License-Identifier: MIT
pragma solidity *;

// The literal is reported once, in the innermost tuple that holds it.

contract C {
    function f() public pure {
        uint256 a;
        uint256 b;
        uint256 c;
        (a, (b, c)) = (1, (2**270, 1));
    }
}
