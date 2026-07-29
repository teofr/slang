// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    function f(uint256[] calldata arr) external pure {
        arr.push();
    }
}
