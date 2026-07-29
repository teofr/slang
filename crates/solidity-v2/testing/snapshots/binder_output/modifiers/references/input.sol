// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

// Migrated from the `definition_references` AST unit test: a modifier applied
// to several functions. Each application is a reference that resolves back to
// the single `onlyOwner` modifier definition.
contract Ownable {
    address _owner;

    modifier onlyOwner() {
        require(msg.sender == _owner);
        _;
    }

    function first() public onlyOwner {}
    function second() public onlyOwner {}
    function third() public onlyOwner {}
}
