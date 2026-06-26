// SPDX-License-Identifier: MIT
pragma solidity *;

// A contract header may carry at most one storage layout (`layout at`)
// specifier. Each specifier past the first is reported; a single one is valid.
// Storage layout specifiers require Solidity 0.8.29 or newer.

contract TwoLayout layout at 0x1000 layout at 0x2000 {}
contract ThreeLayout layout at 0x1000 layout at 0x2000 layout at 0x3000 {}
contract ValidLayout layout at 0x1234 {}
