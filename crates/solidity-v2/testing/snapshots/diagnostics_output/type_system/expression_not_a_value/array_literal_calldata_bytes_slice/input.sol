// SPDX-License-Identifier: MIT
pragma solidity *;

// Accepted: a slice of `bytes calldata` decays to `bytes calldata`, which is a
// valid array element type.

contract Test {
  function f(bytes calldata b) external pure {
    [b[0:1]];
  }
}
