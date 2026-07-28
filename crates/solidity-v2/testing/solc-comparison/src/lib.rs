//! Runs slang v2 against solc's `libsolidity` test corpora, across every
//! supported Solidity version.
//!
//! Two `datatest-stable` harnesses drive it: `tests/semantic_tests.rs` (the
//! `semanticTests` corpus) and `tests/syntax_tests.rs` (the known-valid subset
//! of the `syntaxTests` corpus). This library holds the shared logic they
//! drive: fetching each version's tests ([`dataset`], [`syntax`]), parsing the
//! `isoltest` format ([`mod@test_case`]), running slang ([`runner`]), and the
//! checked-in baselines ([`baseline`]).
//!
//! [`datatest-stable`]: https://github.com/nextest-rs/datatest-stable

#[cfg(test)]
use datatest_stable as _;

pub mod baseline;
pub mod dataset;
pub mod runner;
pub mod syntax;
pub mod test_case;
