// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported: the types `abi.decode` decodes into are written as a tuple, even
// when there is only one.

contract Test {
  function f(bytes memory data) internal pure {
    abi.decode(data, uint);
  }
}
