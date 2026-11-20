// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: the second argument of `abi.decode` is the tuple of types to
// decode into, so a value is not a component it can hold.

contract Test {
  function f(bytes memory data) internal pure {
    abi.decode(data, (uint, 5));
  }
}
