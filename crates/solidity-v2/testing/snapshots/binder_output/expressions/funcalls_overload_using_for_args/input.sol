// Two library functions of the same name are attached to `uint` via
// `using for`. Accessing one on a value binds the receiver as the first
// argument, and overload resolution then disambiguates between the bound
// candidates using the remaining argument.
library Lib {
  function combine(uint self, uint other) internal pure returns (uint) {
    return self + other;
  }

  function combine(uint self, bool flag) internal pure returns (uint) {
    return flag ? self : 0;
  }
}

contract Test {
  using Lib for uint;

  function run(uint a) internal pure returns (uint) {
    uint x = a.combine(2);   // -> combine(uint, uint)
    return a.combine(x > 0); // -> combine(uint, bool)
  }
}
