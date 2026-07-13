// SPDX-License-Identifier: MIT
pragma solidity *;

// Indexed event arguments map to log topics. Non-anonymous events reserve
// topic 0 for the signature, allowing at most 3 indexed arguments; anonymous
// events may use all 4. Only `indexed` parameters count. Covers both valid
// boundaries, both first-invalid counts, and a mixed indexed/non-indexed case.

contract C {
    event Ok3(uint indexed a, uint indexed b, uint indexed c);
    event Bad4(uint indexed a, uint indexed b, uint indexed c, uint indexed d);
    event Mixed4(uint indexed a, uint b, uint indexed c, uint d, uint indexed e, uint indexed f);
    event OkAnon4(uint indexed a, uint indexed b, uint indexed c, uint indexed d) anonymous;
    event BadAnon5(uint indexed a, uint indexed b, uint indexed c, uint indexed d, uint indexed e) anonymous;
}
