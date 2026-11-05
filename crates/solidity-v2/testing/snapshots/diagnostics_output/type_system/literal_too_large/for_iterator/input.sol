// SPDX-License-Identifier: MIT
pragma solidity *;

// The iterator of a `for` loop is a statement of its own.

contract C {
    function f() public pure {
        for (;; 2**300) {}
    }
}
