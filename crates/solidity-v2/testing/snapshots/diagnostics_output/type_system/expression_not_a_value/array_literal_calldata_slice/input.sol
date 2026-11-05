// SPDX-License-Identifier: MIT
pragma solidity *;

// Accepted: a slice of a statically encoded calldata array decays to the array
// it slices (`uint256[] calldata`), which is a valid array element type.

contract Test {
  function f(uint256[] calldata c) external pure {
    [c[1:2]];
  }
}
