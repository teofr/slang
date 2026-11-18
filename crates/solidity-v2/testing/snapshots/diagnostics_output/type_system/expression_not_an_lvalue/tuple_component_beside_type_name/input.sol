// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported twice: every component of a tuple on the left hand side is judged
// on its own, so one that is not a value does not hide another that is not a
// location.

contract Test {
  enum E { A }

  function f() public pure {
    (E, 1) = (E.A, 2);
  }
}
