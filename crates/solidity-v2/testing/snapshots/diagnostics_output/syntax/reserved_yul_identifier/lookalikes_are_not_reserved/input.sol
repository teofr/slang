// SPDX-License-Identifier: MIT
pragma solidity *;

// Only the exact opcode names are reserved: the bare family prefixes, the
// out-of-range indices, and anything that merely starts with a reserved name
// name no opcode, so they remain ordinary Yul identifiers. `memoryguard` is on
// solc's *future* reserved list, which only warns, so it stays an identifier
// too.
contract C {
    function f() public pure {
        assembly {
            let push := 1
            let push33 := 2
            let push0x := 3
            let dup := 4
            let dup17 := 5
            let swap := 6
            let swap17 := 7
            let jumps := 8
            let pcs := 9
            let memoryguard := 10
            pop(add(add(add(push, push33), add(push0x, dup)), add(dup17, swap)))
            pop(add(add(swap17, jumps), add(pcs, memoryguard)))
        }
    }
}
