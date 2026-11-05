// SPDX-License-Identifier: MIT
pragma solidity *;

// A component of a tuple needs a type of its own, and no integer type can hold
// `1E111`.

contract C {
    function f() public pure {
        uint256 x;
        (x, ) = (1, 1E111);
    }
}
