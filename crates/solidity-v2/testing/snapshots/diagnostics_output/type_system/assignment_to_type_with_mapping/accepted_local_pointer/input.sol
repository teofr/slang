// SPDX-License-Identifier: MIT
pragma solidity *;

// Accepted: assigning to a local storage pointer copies nothing, it only
// points the variable elsewhere. Writing through the mapping is fine too.

contract Test {
  mapping(uint => uint) first;
  mapping(uint => uint) second;

  function f() internal returns (mapping(uint => uint) storage result) {
    mapping(uint => uint) storage local = first;
    local = second;
    local[0] = 1;
    first[1] = 2;
    result = local;
  }
}
