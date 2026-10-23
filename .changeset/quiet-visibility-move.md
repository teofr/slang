---
"@nomicfoundation/slang": patch
---

Fixed `function() external public x;` to be parsed as a function type with an `external` visibility, followed by a state variable with a `public` visibility, rather than a function type with both visibilities, starting with Solidity `0.4.22`.
