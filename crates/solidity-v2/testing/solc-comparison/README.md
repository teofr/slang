# `solidity_testing_solc_comparison`

Runs **slang (v2)** against `solc`'s own [`libsolidity` semantic test
suite](https://github.com/argotorg/solidity/tree/develop/test/libsolidity/semanticTests),
checking that all of this **valid** Solidity still compiles without slang
emitting any error diagnostics.

It does this for **every Solidity version slang v2 supports** (0.8.0 up to the
latest): for each version it downloads the semantic tests from that version's
`solc` release tag and runs slang against them pinned to that same language
version.

## Why

The team is incrementally adding semantic validations to slang — making invalid
Solidity emit errors instead of compiling silently. The risk is that a new
validation is too aggressive and starts rejecting **valid** code.

The `semanticTests` suite is a large corpus of Solidity that `solc` itself
guarantees compiles (they are runtime-behavior tests; every source in the suite
compiles successfully under `solc`). Running slang against them and asserting
"no errors" is therefore a cheap, high-signal guard against over-eager
validations.

## Usage

This runs as part of `infra verify` (and, in turn, `infra ci`), as a step after
`infra test` in the CI pipeline. It is intentionally kept out of `infra test`,
since it fetches an external dataset.

```sh
# Run the whole suite (all versions). In CI this checks against the committed
# baseline; run locally it regenerates it (see "Baseline update mode" below).
infra verify

# cargo test args are forwarded — e.g. run just the 0.8.20 cases.
infra verify solc-semantic-suite -- v0.8.20/

# Check locally without rewriting the baseline (as CI does):
CI=1 infra verify
```

Each `(version, test)` pair is a separate test case, so a run fails if any test
**regresses** (fails without being in the baseline) or if the baseline is
**stale** (a listed pair now passes). This is what makes it a CI regression
guard.

### Baseline update mode

Like the repo's other snapshot tests, the mode is chosen by the `CI` env var:

- **In CI** (`CI` set) the cases **check** against the committed baseline and
  the run fails on any drift.
- **Run locally** (`CI` unset) the cases instead **rewrite** the baseline
  (`expected-failures.json`), and the fetch step re-pins `pinned-commits.json`.

So after intentionally changing which tests pass (a new validation, a parser
fix, a version bump), just run `infra verify solc-semantic-suite` locally and commit
the regenerated files.

## How it works

The suite is a [`datatest-stable`](https://github.com/nextest-rs/datatest-stable)
harness (`tests/semantic_tests.rs`, `harness = false`), run via `cargo test`.
`datatest-stable` generates **one test case per file**; we point its `root` at
a directory holding every version's tests
(`target/solc-comparison/v<version>/…`), so the generated cases span the whole
`(version, test)` matrix.

We run it with `cargo test` (in-process, threaded) rather than `cargo nextest`:
nextest is [process-per-test by design](https://nexte.st/docs/design/why-process-per-test/)
and not configurable otherwise, so spawning ~50k processes is both slow (minutes
vs seconds) and overwhelms nextest's list phase. `datatest-stable` supports both
runners, and the whole matrix runs in-process in seconds.

1. **Fetch** — the harness's `root` expression downloads, for every supported
   version (`LanguageVersion::ALL`), the `argotorg/solidity` tarball at that
   version's release tag (e.g. `v0.8.20`) and extracts the `semanticTests/`
   tree into `target/solc-comparison/<tag>/`, reusing the shared
   `infra_utils::http` download helper. The versions are fetched in parallel
   (via `rayon`), since a cold cache means three dozen independent network
   downloads. Release tags are immutable, so a populated cache is reused without
   hitting the network (and `target/` is cached in CI). Because the `root`
   expression fetches this whole dataset, the suite is excluded from the default
   `infra test` run (see `Cargo.toml`) and driven only by `infra verify`.

    **Tags are mutable, so we pin the commit.** Each tarball's `pax_global_header`
    carries the commit SHA the tag resolved to; we record it (in
    [`pinned-commits.json`](./pinned-commits.json), a `{ "<version>": "<sha>" }`
    map) when the baseline is generated, and verify it on every fetch. If a tag
    is later re-pointed at a different commit, the fetch fails loudly rather than
    silently testing against changed content. (A git commit SHA is itself a
    content hash, so this subsumes a separate checksum — no extra download needed.)

2. **Parse** — each test file is in the `isoltest` format: Solidity source
   (optionally split into multiple named sources via `==== Source: <name> ====`
   and referencing shared fixtures via `==== ExternalSource: <path> ====`),
   followed by a `// ====` settings block and a `// ----` runtime-expectation
   block. We parse out the sources and the `EVMVersion` setting; the runtime
   expectations are ignored.
3. **Run** — each case parses its `(version, test)` out of the file path,
   compiles with the slang v2 `CompilationBuilder` pinned to that language
   version and the resolved EVM target (the `EVMVersion` setting if present,
   else that version's default), resolving imports with the shared
   `solidity_testing_utils` `ImportResolver`. The case **passes** iff slang's
   result (clean / has-errors) matches the baseline for that `(version, test)`.
4. **Baseline** — in CI (checking) each case is compared to the baseline.
   Outside CI (update mode) the cases instead rewrite `expected-failures.json`,
   and the fetch step re-pins `pinned-commits.json` (see "Baseline update mode").

## Design decisions

- **slang v2, not v1.** The semantic validations under active development live
  in the v2 diagnostics pipeline, so that's what we exercise. v2 also exposes a
  single `CompilationUnit::diagnostics()` surface that already merges parse and
  semantic errors.
- **`datatest-stable`, run in-process.** Rather than a bespoke pass/fail loop,
  each `(version, test)` is a real test case with a name and filtering, using
  `datatest-stable` (the same crate the repo could use for other data-driven
  suites). The corpus is downloaded at runtime (not vendored), so it uses
  `datatest-stable`'s runtime `root` rather than codegen'd `#[test]`s. We run it
  under `cargo test` (not nextest) because nextest's process-per-test model
  doesn't scale to ~50k cases — see "How it works".
- **Every version, each against its own tests.** We run all of
  `LanguageVersion::ALL`, and for each we use the semantic tests from that
  version's release tag, pinned to that language version. This catches
  version-specific regressions (a validation that's fine at 0.8.35 but breaks
  0.8.0). Release tags are immutable, so runs stay reproducible.
- **A per-`(version, test)` baseline.** `expected-failures.json` is a
  serde-serialized JSON object keyed by version (a `BTreeMap<Version, …>`, so
  keys are ordered `0.8.9` before `0.8.30`), mapping each version to the test
  paths expected to fail at it. Each case is thus a self-contained assertion
  (its expected outcome is known without looking at other cases), which lets the
  check live naturally in a per-test harness.
- **A checked-in baseline, not a hard zero-failures assertion.** slang v2 is
  under development and doesn't yet accept 100% of valid Solidity, so a strict
  "zero failures" gate would be red from day one. The baseline captures the
  current known gaps and turns the check into a **regression detector**: it only
  fails on _new_ breakage. When slang improves, re-running the suite locally
  shrinks the list.

## Known limitations / potential problems

This approach is deliberately a **one-directional, best-effort** check. Things
to be aware of:

1. **One-directional.** It only verifies slang _accepts_ what solc accepts. It
   does **not** verify slang _rejects_ what solc rejects (that's what the v2
   `diagnostics_output` snapshot tests, which run real `solc`, are for), nor
   that the produced CST/bindings are _correct_ — only that no error is emitted.
2. **Approximate EVM-target resolution.** When a test sets `EVMVersion` (e.g.
   `>=byzantium`, `<cancun`) it's resolved to a single concrete target: the
   language version's own default target when that satisfies the constraint,
   otherwise the nearest supported target that does. Without a setting we use
   the version's default. solc's isoltest actually runs each test across a
   _range_ of targets; we pick one representative.
3. **`ExternalSource` path resolution is best-effort.** Fixtures are loaded from
   disk and imports resolved via the shared `ImportResolver`. The dotted /
   non-normalized-source-name fixtures deliberately stress solc's exotic
   source-unit-name normalization, which we don't fully replicate; such tests
   land in the baseline. Malformed / unusual `ExternalSource` specs whose target
   file can't be found count as a **failure** (captured in the baseline) — the
   runner never silently skips a case, since that could hide a regression.
4. **Experimental Solidity lands in the baseline.** Tests containing
   `pragma experimental solidity` use a language slang doesn't implement, so
   slang emits errors on them; rather than special-casing a skip, they're
   treated like any other current gap and captured in the baseline.
   (Note: the unrelated isoltest `experimental: true` _setting_ — which marks an
   experimental _codegen backend_ — is ordinary Solidity and is checked the same
   as everything else.)
5. **External dataset.** One tarball per supported version is fetched over the
   network and isn't part of the default `infra test` run. CI must have network
   access the first time; afterwards the extracted trees are reused from the
   `target/` cache.

## What the current baseline tells us

Across all supported versions (0.8.0–0.8.36) this runs ~51k (version, test)
combinations. The baseline currently holds **342 failing (version, test)
pairs**, spanning **16 distinct tests** — every one a place where slang v2
rejects valid Solidity today. They cluster into a few themes: inline-assembly
target/version-gated builtins (and builtin-vs-identifier shadowing),
aliased-import free-function overload sets, `block` members across the Paris
fork (`difficulty`/`prevrandao`), and the experimental-Solidity tests (which use
a language slang doesn't implement).
