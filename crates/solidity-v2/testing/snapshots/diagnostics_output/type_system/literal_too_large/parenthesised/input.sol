// SPDX-License-Identifier: MIT
pragma solidity *;

// Parentheses are looked through, so a too-large literal inside them is
// reported at the literal itself, where solc reports the parenthesised
// expression: both as a statement (3757) and as a tuple component (3390).

contract C {
    function f() public pure {
        (2**300);
        (1, (2**270));
    }
}
