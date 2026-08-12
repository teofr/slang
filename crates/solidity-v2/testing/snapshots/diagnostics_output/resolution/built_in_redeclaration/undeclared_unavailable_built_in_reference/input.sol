// SPDX-License-Identifier: MIT
pragma solidity *;

// Companion to `reference_to_unavailable_built_in_name`: the same reference
// with *no* declaration in scope. Preferring a user definition over an
// unavailable built-in must not stop the built-in from resolving when nothing
// else is in scope, so this keeps reporting the precise
// `incompatible-built-in-target` diagnostic before Cancun rather than a bare
// "unresolved identifier". solc reports `Identifier "mcopy" not found` here, so
// both reject; only the wording differs.
contract C {
    function f() public pure {
        assembly {
            let x := mcopy
        }
    }
}
