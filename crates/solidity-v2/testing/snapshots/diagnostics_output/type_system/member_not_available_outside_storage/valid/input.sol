// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    uint256[] s;

    // Storage dynamic arrays support `push`/`pop`.
    function f() public {
        s.push(1);
        s.pop();
    }

    // Memory and calldata arrays still expose `length`.
    function g(uint256[] memory arr) public pure returns (uint256) {
        return arr.length;
    }
}
