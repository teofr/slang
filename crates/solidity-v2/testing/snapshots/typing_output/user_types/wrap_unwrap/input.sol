type ShortString is bytes32;

contract Test {
    function test(bytes32 data) public pure returns (ShortString) {
        ShortString s = ShortString.wrap(data);
        return s;
    }
}
