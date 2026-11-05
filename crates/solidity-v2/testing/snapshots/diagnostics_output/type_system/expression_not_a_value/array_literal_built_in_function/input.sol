// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: a built-in function has no mobile type, so it cannot be an element
// of an array literal, even one that is only used as a statement.

contract Test {
  function f() public pure {
    [keccak256];
  }
}
