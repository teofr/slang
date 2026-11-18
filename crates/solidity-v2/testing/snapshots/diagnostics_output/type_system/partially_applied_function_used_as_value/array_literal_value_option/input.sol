// SPDX-License-Identifier: MIT
pragma solidity *;

// The first item has a `value` call option applied, so it has no mobile type.

contract D {
    function f(uint256 a) external payable {}
    function g(uint256 a) external {}
}

contract C {
    function f() public {
        D d;
        [d.f{value: 1}, d.g][0](8);
    }
}
