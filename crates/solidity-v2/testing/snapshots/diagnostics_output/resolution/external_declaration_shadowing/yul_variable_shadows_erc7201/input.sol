// SPDX-License-Identifier: MIT
pragma solidity *;

// `erc7201` becomes a Solidity global only in 0.8.35. Before that the name is
// not declared anywhere, so `let erc7201 := 1` is a valid Yul identifier; from
// 0.8.35 on it shadows the global and is rejected. Iterating language versions
// shows the shadow appearing exactly at its introducing version.
contract C {
    function f() public pure {
        assembly {
            let erc7201 := 1
        }
    }
}
