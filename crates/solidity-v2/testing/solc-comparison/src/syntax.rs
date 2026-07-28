//! The `syntaxTests` sibling corpus.
//!
//! solc's `test/libsolidity/syntaxTests` cases are compile-time tests: their
//! `// ----` trailer lists the diagnostics solc expects. A case with **no**
//! error-level expectation is known-valid Solidity (solc accepts it), so it's a
//! natural extension of the `semanticTests` accept-check in [`crate::dataset`] —
//! roughly doubling the known-valid corpus.
//!
//! The corpus is fetched *alongside* `semanticTests` by [`crate::dataset`] — one
//! download per version extracts both trees — so this module only enumerates and
//! classifies the `syntaxTests` files. The harness in `tests/syntax_tests.rs`
//! runs slang against the trailer-free cases and compares against
//! `expected-syntax-failures.json`. Cases that *do* expect errors are out of
//! scope here (they'd exercise the reverse direction — that slang rejects what
//! solc rejects) and are skipped by the harness.

use std::path::Path;

use semver::Version;
use slang_solidity_v2_common::versions::LanguageVersion;

/// Selects real, standalone syntax-test files: `v<version>/syntaxTests/<test>.sol`,
/// excluding `_`-prefixed fixture directories. Matched against each file's path
/// relative to the shared cache dir ([`crate::dataset::dataset_root`]).
pub const HARNESS_PATTERN: &str =
    r"^v[0-9]+\.[0-9]+\.[0-9]+/syntaxTests/(?:[^/_][^/]*/)*[^/_][^/]*\.sol$";

/// Parses the `(language version, path relative to `syntaxTests`)` out of a full
/// test-file path of the form `.../v<version>/syntaxTests/<rel>`.
pub fn parse_version_and_relpath(path: &Path) -> Option<(LanguageVersion, String)> {
    let components: Vec<&str> = path.iter().filter_map(|c| c.to_str()).collect();
    let index = components.iter().position(|&c| c == "syntaxTests")?;
    let version_tag = components.get(index.checked_sub(1)?)?;
    let version = Version::parse(version_tag.strip_prefix('v')?).ok()?;
    let language_version = LanguageVersion::try_from(version).ok()?;
    let relative_path = components[index + 1..].join("/");
    Some((language_version, relative_path))
}

/// Whether a syntax-test file declares any expected diagnostic in its `// ----`
/// trailer. Such a case is one solc *rejects* (or warns on), so it's out of
/// scope for the accept-only check; only trailer-free (or empty-trailer) cases
/// are known-valid Solidity.
pub fn has_error_expectations(contents: &str) -> bool {
    let mut in_trailer = false;
    for line in contents.lines() {
        let trimmed = line.trim_end();
        if trimmed == "// ----" {
            in_trailer = true;
            continue;
        }
        // Any non-empty `//` line inside the expectation block is a declared
        // diagnostic (e.g. `// TypeError 1234: ...`, `// Warning 5678: ...`).
        if in_trailer
            && trimmed
                .strip_prefix("//")
                .is_some_and(|rest| !rest.trim().is_empty())
        {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_error_expectations() {
        // A trailer with a declared error is out of scope (solc rejects it).
        assert!(has_error_expectations(
            "contract C { uint x = true; }\n// ----\n// TypeError 9574: (0-0): oops\n"
        ));
        // An empty trailer, or no trailer at all, is a known-valid case.
        assert!(!has_error_expectations("contract C {}\n// ----\n"));
        assert!(!has_error_expectations("contract C {}\n"));
    }

    #[test]
    fn parses_version_and_relpath_from_syntax_path() {
        let (version, relative_path) = parse_version_and_relpath(Path::new(
            "target/solc-comparison/v0.8.20/syntaxTests/scoping/scoping.sol",
        ))
        .expect("a well-formed syntax path parses");
        assert_eq!(relative_path, "scoping/scoping.sol");
        assert_eq!(Version::from(version).to_string(), "0.8.20");

        // A `semanticTests` path is not matched by the syntax parser.
        assert!(
            parse_version_and_relpath(Path::new(
                "target/solc-comparison/v0.8.20/semanticTests/x.sol"
            ))
            .is_none()
        );
    }
}
