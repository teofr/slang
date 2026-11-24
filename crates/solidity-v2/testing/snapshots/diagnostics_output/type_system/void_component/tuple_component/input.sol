// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: a tuple that is read is built from the values of its components,
// and a call to a function that returns nothing has none. Each such component
// is reported, however deeply it is nested.

contract Test {
  function nothing() internal pure {}

  function f() internal pure {
    (nothing(), 1);
    (1, (2, nothing()));
  }
}
