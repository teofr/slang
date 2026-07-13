// SPDX-License-Identifier: MIT
pragma solidity *;

// A contract may declare at most one fallback and one receive function. Each
// duplicate past the first is reported. A contract with exactly one of each is
// valid and must not be flagged.

contract TwoFallback { fallback() external {} fallback() external {} }
contract ThreeFallback { fallback() external {} fallback() external {} fallback() external {} }
contract TwoReceive { receive() external payable {} receive() external payable {} }
contract ThreeReceive { receive() external payable {} receive() external payable {} receive() external payable {} }
contract Valid { fallback() external {} receive() external payable {} }
