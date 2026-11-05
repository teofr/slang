// SPDX-License-Identifier: MIT
pragma solidity *;

// Unlike an array literal, a conditional and a variable declaration never ask
// for a nameable type, and a slice of a dynamically encoded calldata array
// converts to the array it slices. solc still rejects taking such a slice at
// all, which Slang does not report yet.

contract Test {
  function f(string[] calldata s) external pure returns (string[] calldata) {
    true ? s[0:1] : s[1:2];
    string[] calldata x = s[0:1];
    return x;
  }
}
