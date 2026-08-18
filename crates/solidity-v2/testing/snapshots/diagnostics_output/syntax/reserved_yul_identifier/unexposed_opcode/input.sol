// SPDX-License-Identifier: MIT
pragma solidity *;

// `pc` is an EVM opcode that Yul deliberately doesn't expose as a built-in, yet
// solc still reserves its name on every version and EVM target, so the
// declaration below is always rejected. Slang reserves it as a keyword that is
// never enabled in the grammar, so the name can't be lexed as a `YulIdentifier`
// and the rejection is a syntax error rather than a declaration error.
contract C {
    function f() public pure {
        assembly {
            let pc := 1
        }
    }
}
