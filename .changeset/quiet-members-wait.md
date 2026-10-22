---
"@nomicfoundation/slang": patch
---

Parse `function() x;` in a contract as a state variable named `x` (not an unnamed function with an `x` modifier) before Solidity `0.6.0`, matching `solc`.
