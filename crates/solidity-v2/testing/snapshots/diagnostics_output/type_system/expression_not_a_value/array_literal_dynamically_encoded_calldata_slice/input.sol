// SPDX-License-Identifier: MIT
pragma solidity *;

// A slice of a calldata array with a dynamically encoded base type (here
// `string[] calldata`) does not decay: solc keeps the slice itself as its mobile
// type, which is not nameable, so the array literal has no element type. solc
// also rejects taking such a slice at all. Slang always decays a slice to the
// array it slices, and reports neither.

contract Test {
  function f(string[] calldata s) external pure {
    [s[0:1]];
  }
}
