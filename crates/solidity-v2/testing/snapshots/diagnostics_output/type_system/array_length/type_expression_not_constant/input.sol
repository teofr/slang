// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // The array length in the `abi.decode` type expression is a non-constant
    // parameter, so it is not a compile-time constant.
    function decode(bytes memory data, uint256 n) internal pure {
        abi.decode(data, (uint256[n]));
    }
}
