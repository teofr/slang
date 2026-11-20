// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported on the component, where the tuple is built, rather than on the
// whole tuple as an array item.

contract C {
    function f() public pure {
        [(2**256, 1)];
    }
}
