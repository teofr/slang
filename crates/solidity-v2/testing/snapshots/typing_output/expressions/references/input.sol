contract C {
    uint256 stored;

    function setAndGet(uint256 value) public returns (uint256) {
        stored = value;
        return stored;
    }
}
