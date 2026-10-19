# Review, round 2: PR 2255 + 1c / 2b / 2c / 4, and what to try next

Self-contained; corrects round 1 where the new numbers contradict it.

## TL;DR

- **Round 1 was wrong to recommend 2a.** Its win was LLVM's single-call-site inlining heuristic, and PR 2254 (`#[inline(always)] next_lexeme`) turns it into a loss. With 4 in place the PR's own shape (2c) is right: `0+1b+2c+3+4` is −3.17% vs main, `main+1c+4` is −3.20%. Parity (+0.03%) is a wash, not an identity: the PR wins on 6/9 contracts and loses up to +0.3% on `uniswap`, `one_step_leverage_f`, `multicall3` (more `///` lines per token).
- The parity is **structurally sound but not guarded.** I built both variants and read the asm: under fat LTO (the bench) `NonTriviaTokens::next` is inlined into LALRPOP's drive loop as the comments say; in the parser crate's own non-LTO codegen (what a consumer without LTO links) it is an out-of-line function reached through a hoisted GOT pointer (`callq *%r12`) from `Parser::parse`. Nothing pins either outcome; one `#[inline(never)]` is all that keeps `read_natspec_comment` out of the loop.
- **1c is safe on main**, with one thing to verify (debug-build recursion in logos' skip path) and one codegen gap (nothing rejects `skipped = true` on a NatSpec item).
- Land as two PRs: **A = main + 1c + 4** (perf, −3.2%), **B = 2255 rebased on A with 1b's gap merge + 2c + 3** (correctness/simplicity, ±0.0%). Measure next: **2d**, **6**, **8**.

## 1. Correctness and code review

### 1c (skip all regular trivia on main)

Safe. Skipping is a leaf action, not a pattern change: the DFA, priorities and `allow_greedy` are untouched, only the leaf returns `FilterResult::Skip` (`contexts.rs.jinja2`, `skip_trivia`). Longest match picks the same lexemes; the skipped ones are exactly those `TokenStream` dropped in `if lexeme.kind.is_trivia() { continue }`, so the non-trivia + NatSpec sequence is byte-identical. main's `///` regex merges a run *inside one lexeme* and never depended on the trivia around it: a blank line, `/* */`, `////` or `// x` still ends the match (the tail needs `(SpaceOrTab)*///(TextAfterSlashes)?`, and `TextAfterSlashes` can't start with `/`); the second run is a second lexeme and `TokenStream` keeps the last, as before. `////` goes to the `//` regex (skipped instead of emitted). EOF runs are still dropped. All 12 regular-trivia items across Pragma/Solidity/Yul carry the flag; morphs happen on tokens only, so no run straddles contexts. `Lexer` is crate-private (`lib.rs` exports `Parser`/`ParseOutput` only); the only visible change is `lexer/tests.rs`.

Two follow-ups:

1. **Debug-build recursion.** logos 0.16.1 defaults to goto codegen (`state_machine_codegen` off; `logos-codegen/src/generator/leaf.rs:62-64`), so `Skip` is `return goto_root(lex, …)`, a tail call that release builds flatten and debug builds don't. Previously every trivia lexeme returned to `next()`; now a trivia run chains inside the lexer. On main `//` lines are merged by the regex so chains are short; on 2255 (1b) each `//` line is its own lexeme, so a 2000-line header is a 2000-deep chain in debug tests (2 MiB thread stacks). Add a stress test (`"// x\n".repeat(50_000) + "contract C {}"`) and, if it overflows, enable `state_machine_codegen` or re-merge `//` lines in the regex.
2. **`skipped` is unvalidated.** `p3_references/mod.rs` destructures `skipped: _`. `skipped = true` on a `NatSpecComment` item silently drops all docs; skipping a kind in one context but not another makes `debug_assert!(!lexeme.kind.is_trivia())` input-dependent. Either reject `skipped && category == NatSpecComment` in p3, or drop the flag and have codegen skip "every trivia whose category isn't NatSpecComment" (no language-model change). Generate out the dead `LexemeKind` variants and the 18 `unreachable!` arms in `TerminalKind::from` (`lexemes.generated.rs:517-538`), or document why they stay.

### 2b

Faithful to 2a, no behaviour change; `#[cold]` on `read_natspec_comment` is a mislabel for comment-dense files (once per `///` line) but harmless. Superseded by 2c (§2).

### 2c with 1b (`natspec.rs` on `bench/2255-combined-no2-2254`)

- `read_natspec_comment` + `is_line_continuation` are equivalent to the PR's `CommentReader` (round 1's argument holds: the gap between two NatSpec lexemes is only skipped trivia; it is "one break + `[ \t]*`" iff the machine would be in `AfterLineBreak`). CRLF, lone `\r`, `\n\r` (two breaks → split, consistent with `LineBreak = \n | \r\n?`), `\x0c` (UNRECOGNIZED either way), `/**` after `///` (replaces), EOF (dropped) checked.
- `3` is implicit now (`range.end` is the last `///` lexeme's end, which never includes the break), and the best branch also removed main's second trim in `ir/.../memory_safety.rs`, so round 1's rebase hazard is handled.
- Still missing from round 1: unit tests for `is_line_continuation` (`"\n"`, `"\r\n  "`, `"\r"`, `"\n\n"`, `"\n\t\n"`, `"\n/* x */ "`, `""`). It is the one piece that outlives the trivia skipping.
- The comment on `read_natspec_comment` ("`next` … is inlined into LALRPOP's parsing loop") is true under LTO and verified by 2a+4, but it is LLVM's choice, not the code's (§2). Say so or pin it (2d). Nit: `NonTriviaTokens.source` duplicates `logos::Lexer::source()`.

### 4

Correct and cheap. The comment "Always inlined into the parser's hot loop" should say *why*: with two call sites (hot `next`, cold `read_natspec_comment`) LLVM stopped inlining a body that is a 3-way `ContextWrapper` match plus the morph match, costing a call and an `Option<Lexeme>` round trip through memory per token. Without the reason it reads as cargo cult.

solc behaviour is unchanged by all of these; round 1's comparison stands.

## 2. Interpreting the inlining results

Calibration: from pairs differing by a known amount of code, 1 instruction/token ≈ 0.08% of the parser stage (≈1200 instructions/token including LALRPOP and CST construction).

Verified on release builds of `bench/2255-combined-no2-2254` and `bench/2255-combined-2254` (scratch worktrees, removed): the parser crate with `--emit=asm`, and its `cargo test --release --no-run` binary, which inherits `lto = true`.

- Crate codegen: `next` is out-of-line in both; `Parser::parse` (LALRPOP's `drive`/`next_token` inlined) calls it via GOT at the two `next_token` sites. The three logos `lex` entries are called directly from `next`, so 4 did its job: both `next_lexeme`s are gone.
- Fat LTO (2a+4): no `next` symbol survives; the function hosting the drive loop calls the three `lex` entries, `attach_natspec_to`, `__reduce` and three `core::str::slice_error_fail` directly — `next` *and* the gap check were inlined into the parsing loop. This is what Callgrind measured.
- **2c+4 `next`**: 2 callee-saved pushes. Hot path: context dispatch → `call lex` → `None`/UNRECOGNIZED checks → morph chain (4–5 `cmp`) → `is_natspec_comment` as `sub/cmp/ja` + `movabs/bt` + 2 fallback compares → 3 stores for the `Option<Result<(usize, LexemeKind, usize), ()>>`.
- **2a+4 `next`**: 6 callee-saved pushes, a jump table for kind classification, all of `is_line_continuation` (UTF-8 decode of the gap) plus a `slice_error_fail` path inlined, and on **every** token a load, a store and a compare of `self.pending`'s discriminant (`movb $2, 72(%r14)` / `cmpb $2, 72(%r14)`).

That explains the table. Without 4: 2a/2b had one `next_lexeme` call site, so LLVM inlined it; 2c had two and got neither, paying a call + `Option<Lexeme>` round trip per token (−0.83 vs −1.72/−1.81) — round 1 read that as design. With 4 the layer is gone everywhere: 2c is minimal (−3.17); 2b keeps the `pending` load/store/compare and the loop back-edge (+0.32 ≈ 4 instr/token); 2a also drags the gap check and jump table into the drive loop, raising register pressure across LALRPOP's whole state machine (+1.4 more). 2a+4 is worse than 2a alone because 2a alone already had `next_lexeme` inlined; 4 only added code. main+1c+4 vs the PR: main keeps a dead `is_trivia()` test and `natspec_comment.take()` per token; the PR does one `next()` round trip and a gap check per `///` line instead of one regex match per block. Net zero; sign follows `///`-density.

Robustness: the *shape* is robust (one out-of-line lexer call per token, a two-compare filter). Fragile: (a) `read_natspec_comment` staying out rests on one `#[inline(never)]`; (b) `next` being inlined is an LTO-time heuristic, and the crate's own codegen already decides otherwise, so non-LTO users pay ≈1% the bench never sees; (c) the hot loop is intra-crate (`Parser::parse` and the lexer both live in `slang_solidity_v2_parser`), so attributes carry over to users, heuristics don't. To make it stick: fix the two comments and state the shape contract on `next` (one lexer call, one kind compare, no state); add the gap tests; land A and B as separate PRs with `ci:perf` so each gets its own bencher point (the 1% `instructions` threshold already exists, `crates/infra/cli/src/commands/perf/cargo/mod.rs:169-171`); measure 2d.

## 3. Alternatives not yet tried

| ID | Idea | Base | Expected gain (parser stage) | Risk / cost |
|---|---|---|---|---|
| **2d** | `#[inline(always)]` on `NonTriviaTokens::next`; `Item = (usize, LexemeKind, usize)` (drop the dead `Result<_, ()>`; `__ToTriple` exists for the bare triple) | both | ~0 on the LTO bench; ~1% for non-LTO consumers (call, pushes, 3 sret stores + reloads ≈ 12–15 instr/token); turns a heuristic into a contract | none |
| **6** | Codegen orders `LexemeKind` with trivia (NatSpec last) as a contiguous tail; `is_natspec_comment`/`is_trivia` become `(kind as u8) >= FIRST` — one compare instead of range + `bt` + 2 compares | both | 0.2–0.4% (3–5 instr/token) | low; `strum` and LALRPOP map by name |
| **8** | Morph detection via `static MORPH: [u8; 256]` indexed by kind (or logos callbacks on the 5 context-switching tokens setting `extras.pending_morph`), replacing the 4–5 `cmp` chain in `Lexer::next_lexeme` | both | 0.3–0.5% | low; keep `brace_depth` as is |
| **5a** | Merge `///` lines in a logos callback peeking `lex.remainder()` for `\r?\n[ \t]*///[^/]` and `bump`ing | 2255 | 0–0.3%, only on comment-dense files (removes a `next()` round trip + gap check per `///` line; erases the uniswap residual) | medium: hand-written sub-lexer in generated code (`////`, CRLF, char boundaries); puts line structure back in the lexer, which the PR wanted out |
| **5b** | `#[logos(skip …)]` instead of the `skip_trivia` callback | both | 0 (same leaf) | pointless; keep the callback |
| **5c** | No `NatSpecStack`: scan the source backwards from each documentable node's start | both | tiny (reduce path only) | high: `/* /// */`, strings, `// x /// y` are ambiguous without lexer state |
| **5d** | Keep the lexer trivia-free for good; **re-lex trivia from token gaps on demand** (a ~50-line gap lexer generated from the same `Trivia` items) when the full-fidelity CST needs it | both | preserves the −3% when the "include trivia" TODO lands; otherwise that TODO returns it all | medium: two descriptions of the trivia grammar unless generated; design work, not speed work |
| 7 | Collapse the `ContextWrapper` 3-way dispatch (fn pointer to the active `lex`) | both | ≤0.25% | medium for ~3 instr; skip |
| 9 | `Cell` fast path instead of `RefCell::borrow_mut` in `pop_for_token_at_range` | both | ≤0.1% | low; not worth a PR |
| 10 | `codegen-units = 1` (+ `panic = "abort"` where allowed) in `[profile.release]` | bench | 1–3% typical, and it damps the inlining lottery behind this table | build time; affects our binaries only, so no substitute for attributes |
| 11 | PGO for the bench | bench | 5–10% | doesn't ship to users; only shows the ceiling |
| — | Lex NatSpec only when a documentable node can follow | — | — | infeasible with LALRPOP's pull iterator; the stack already does the lazy equivalent |
| — | `LexemeKind: Copy` / smaller `Lexeme` | — | 0 | already 1 byte; `Range` copies are moves |
| — | Runtime "return trivia" switch | — | — | logos skipping is compile-time; a flag needs a second DFA → use 5d |

Top 3 to measure: **2d** (one attribute; also the robustness fix), **6**, **8**. 5d deserves a design note before anyone picks up the trivia TODO.

## 4. Verdict and path

**Adopt for PR 2255: `0 + 1b-merge + 2c + 3`, rebased on `main + 1c + 4`** — the `bench/2255-combined-no2-2254` tree, split in two:

- **PR A (perf): lexer skips non-NatSpec trivia + `#[inline(always)] next_lexeme`** (`main-1c.diff` + `4-pr2254.diff`), plus p3 validation or category-based codegen for `skipped`, the two comment fixes, the debug-recursion stress test, and a `TODO(v2)` saying the skipping goes away (or becomes 5d) when trivia enters the CST. −3.2% on its own; run with `ci:perf`.
- **PR B (2255): one lexeme per `///` line, merge by source gap, range without trailing break.** `natspec.rs` as on the best branch, `is_line_continuation` unit tests, and a PR body stating the motivation (the regex encoded indentation/blank-line rules in four DFAs per context and still couldn't express "last comment wins"). Expected ±0.0% on A; say that, not "−5.3% vs main".
- Drop 2a/2b; fold 2254 into A.

**For Omar:** (1) 2255 as filed is +2.2% in the parser stage, but the cost is the call-layer heuristic plus trivia being emitted, not the design — A removes both and B is neutral; (2) the inlining comments describe LLVM's current LTO choices, not guarantees; add the reasons and consider 2d so non-LTO users get the same loop; (3) merge-by-gap is deliberately independent of which trivia lexemes exist, so it survives both the skipping and its removal; (4) the real decision is 5d vs "give the 3% back later", not 2a vs 2c.


---

**Correction (verified while publishing):** the best branch did not remove main's second line-break trim. It is based on PR 2255's base, which predates that trim (`memory_safety.rs:70` on main), so the trim must still be removed when rebasing option 3 onto main.
