// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: an item of an array literal needs a value, and a call to a
// function that returns nothing has none.

contract Test {
  function nothing() internal pure {}

  function f() internal pure {
    [nothing()];
  }
}
