// SPDX-License-Identifier: MIT
pragma solidity *;

// A tuple that is assigned to is written component by component, so a literal
// component is reported as not being a location, and not as too large.

contract C {
    function f() public pure {
        uint256 x;
        (2**270, x) = (1, 2);
    }
}
