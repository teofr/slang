// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported once, where the options are applied: a tuple of type names is not a
// function. The array literal it is an item of is left untyped, with nothing
// more to report.

function f() pure {
    [(uint, bool){value: 1}];
}
