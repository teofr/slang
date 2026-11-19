// SPDX-License-Identifier: MIT
pragma solidity *;

// A literal too large for any integer type cannot be a component of a tuple
// with more than one component. solc only checks this for number literals, and
// otherwise keeps the raw component types of a tuple.

function f() pure {
    (2**256, 1);
}
