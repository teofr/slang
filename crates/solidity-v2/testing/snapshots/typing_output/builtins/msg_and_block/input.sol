contract C {
    function info() public view returns (address, uint256) {
        return (msg.sender, block.timestamp);
    }
}
