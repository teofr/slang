// SPDX-License-Identifier: MIT
pragma solidity *;

// Reported once: the tuple of types `abi.decode` decodes into is read, so a
// too-large literal in it is reported as such, and not again as a component
// that is not a type name.

contract Test {
  function f(bytes memory data) internal pure {
    abi.decode(data, (uint, 2**300));
  }
}
