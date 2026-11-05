// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: a magic variable has no mobile type, so it cannot be an element of
// an array literal, even one that is only used as a statement.

contract Test {
  function f() public pure {
    [msg];
  }
}
