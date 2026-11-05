// SPDX-License-Identifier: MIT
pragma solidity *;

// A tuple used as a statement is still built, so its components need a type.

contract C {
    function f() public pure {
        (2**270, 1);
    }
}
