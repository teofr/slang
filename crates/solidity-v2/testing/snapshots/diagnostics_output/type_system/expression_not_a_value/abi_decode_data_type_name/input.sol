// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: only the second argument of `abi.decode` names types. The first is
// the data to decode, a value.

contract Test {
  function f() internal pure returns (uint) {
    return abi.decode(uint, (uint));
  }
}
