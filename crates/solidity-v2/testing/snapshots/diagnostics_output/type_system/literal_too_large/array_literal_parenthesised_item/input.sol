// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported at the literal: the parentheses around an item are looked through.

function f() pure {
    [(2**256)];
}
