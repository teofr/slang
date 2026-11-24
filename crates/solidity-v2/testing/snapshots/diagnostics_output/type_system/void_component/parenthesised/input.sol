// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: a pair of parentheses reads the expression inside, and a call to a
// function that returns nothing has no value to read.

contract Test {
  function nothing() internal pure {}

  function f() internal pure {
    (nothing());
  }
}
