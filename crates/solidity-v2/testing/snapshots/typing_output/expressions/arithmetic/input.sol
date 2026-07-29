contract C {
    function f(uint256 x) public pure returns (uint256) {
        uint256 y = x + 1;
        return y * 2;
    }
}
