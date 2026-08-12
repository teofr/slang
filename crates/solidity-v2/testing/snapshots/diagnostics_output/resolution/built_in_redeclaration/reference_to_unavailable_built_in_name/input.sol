// SPDX-License-Identifier: MIT
pragma solidity *;

// A built-in that is not yet available does not reserve its name, so declaring
// it is legal and a later reference must resolve to that declaration. Before
// Cancun `mcopy` is an ordinary identifier, so both lines below are accepted;
// from Cancun the declaration is rejected as a built-in redeclaration.
//
// Resolving the reference to the built-in instead would report
// `incompatible-built-in-target` ("introduced in EVM target 'Cancun'") on the
// reference, even though the name refers to a perfectly good local variable.
contract C {
    function f() public pure {
        assembly {
            let mcopy := 1
            let x := mcopy
        }
    }
}
