// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: call options apply to a function, not to a value of another type.

contract Test {
  function f() internal pure {
    uint count;
    count{value: 1};
  }
}
