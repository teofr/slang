// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: a mapping cannot be copied, so neither a mapping in storage nor a
// struct holding one can be assigned to.

contract Test {
  struct Ledger {
    mapping(uint => uint) balances;
  }

  mapping(uint => uint) first;
  mapping(uint => uint) second;
  Ledger ledger;
  Ledger other;
  Ledger[] ledgers;

  function f() internal {
    first = second;
    ledger = other;
    ledgers[0] = other;
  }
}
