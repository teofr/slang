// Like solc, constant folding is limited to 4096 bits of precision, so these
// are rejected even though each result would reduce into range.

pragma solidity *;

contract Test {
    uint256[2 ** 5000 / 2 ** 4990] power;
    uint256[1e2000 / 1e1999] scientific;
    uint256[(1 << 5000) >> 4995] shifted;
}
