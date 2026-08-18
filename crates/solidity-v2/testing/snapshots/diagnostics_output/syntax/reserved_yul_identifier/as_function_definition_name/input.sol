// SPDX-License-Identifier: MIT
pragma solidity *;

// A reserved name is rejected in every declaration position, not just in a
// `let`: solc checks Yul function names and their parameters and return
// variables as well.
contract C {
    function f() public pure {
        assembly {
            function jump(jumpi) -> jumpdest {
                jumpdest := jumpi
            }
        }
    }
}
