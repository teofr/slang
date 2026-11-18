// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported once: a write to a constant is rejected, but the assignment is
// still a value of the constant's type, so the conditional it is a branch of
// has a type too.

contract Test {
  uint constant LIMIT = 1;

  function f(bool condition) internal pure returns (uint) {
    return condition ? (LIMIT = 2) : 3;
  }
}
