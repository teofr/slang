// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported once: a type name takes no call options, so there is no conversion
// left to judge.

contract Test {
  function f() internal pure {
    uint{value: 1}(5);
  }
}
