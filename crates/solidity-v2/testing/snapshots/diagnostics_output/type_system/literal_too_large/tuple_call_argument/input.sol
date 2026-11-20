// SPDX-License-Identifier: MIT
pragma solidity *;

// An argument of a built-in call can be a tuple too.

contract C {
    function f() public pure {
        abi.encode((1, 2**270));
    }
}
