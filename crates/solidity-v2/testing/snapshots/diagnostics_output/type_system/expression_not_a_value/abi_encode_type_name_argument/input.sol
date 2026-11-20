// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: `abi.encode` encodes values, so a type name is not an argument it
// can take. Only a few arguments of a built-in may name a type.

contract Test {
  function f() internal pure returns (bytes memory) {
    return abi.encode(uint);
  }
}
