// SPDX-License-Identifier: MIT
pragma solidity *;

// A literal too large for any integer type cannot be used as a statement
// expression. solc only checks this for number literals.

function f() pure {
    2**256;
}
