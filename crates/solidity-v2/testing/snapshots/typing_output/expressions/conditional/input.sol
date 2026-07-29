contract C {
    function pick(bool cond, uint256 a, uint256 b) public pure returns (uint256) {
        return cond ? a : b;
    }
}
