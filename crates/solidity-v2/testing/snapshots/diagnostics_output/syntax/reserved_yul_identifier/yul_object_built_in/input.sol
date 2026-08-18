// SPDX-License-Identifier: MIT
pragma solidity *;

// `datasize` is only a built-in in standalone Yul objects, never in inline
// assembly, but solc reserves it here too, so declaring it is rejected.
contract C {
    function f() public pure {
        assembly {
            let datasize := 1
        }
    }
}
