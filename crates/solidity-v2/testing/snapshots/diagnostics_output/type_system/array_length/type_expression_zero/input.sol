// SPDX-License-Identifier: MIT
pragma solidity *;

contract C {
    // A zero-length array in an `abi.decode` type expression.
    function decode(bytes memory data) internal pure {
        abi.decode(data, (uint256[0]));
    }
}
