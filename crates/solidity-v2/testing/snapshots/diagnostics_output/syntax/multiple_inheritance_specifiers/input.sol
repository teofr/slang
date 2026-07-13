// SPDX-License-Identifier: MIT
pragma solidity *;

// A contract header may carry at most one inheritance (`is`) specifier list.
// Each list past the first is reported; a single list is valid.

contract A {}
contract B {}
contract TwoIs is A is B {}
contract ThreeIs is A is B is A {}
contract ValidInherit is A {}
