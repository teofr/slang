// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

// Migrated (partially) from the `number_literals` AST unit test: the various
// number-literal forms and the literal types the typing pass assigns them.
contract NumberLiterals {
    uint256 constant HEX = 0xff;
    uint256 constant DEC_WITH_SEP = 1_000_000;
    uint256 constant EXPONENT = 1e3;
    uint256 constant ONE_ETHER = 1 ether;
    uint256 constant FOLDED_RATIONAL = 0.5 * 4;
}
