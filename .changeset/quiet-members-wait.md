---
"@nomicfoundation/slang": patch
---

Fixed `function() x;` in a contract to be parsed as a `StateVariableDefinition` named `x`, rather than an `UnnamedFunctionDefinition` with an `x` modifier, before Solidity `0.6.0`.
