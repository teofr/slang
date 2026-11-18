// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: call options keep the typing of an operand that is not a function,
// so a tuple of type names gets past them. It has no mobile type, as an item of
// an array literal needs.

function f() pure {
    [(uint, bool){value: 1}];
}
