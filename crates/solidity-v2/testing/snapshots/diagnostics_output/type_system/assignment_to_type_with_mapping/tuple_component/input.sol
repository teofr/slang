// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: a tuple is assigned to component by component, so a mapping in
// storage is rejected where it stands.

contract Test {
  mapping(uint => uint) first;
  mapping(uint => uint) second;

  function f() internal {
    uint count;
    (count, first) = (1, second);
  }
}
