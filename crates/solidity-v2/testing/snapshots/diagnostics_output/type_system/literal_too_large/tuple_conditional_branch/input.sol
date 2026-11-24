// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported on the component, where the tuple is built, rather than on the
// whole tuple as a branch.

contract C {
    function f(bool c) public pure {
        c ? (1, 2**270) : (1, 2);
    }
}
