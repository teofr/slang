# Review: PR 2255 and the opt1/1b/2/3 follow-ups

## TL;DR

- The PR's own merging logic is correct and matches solc (last comment wins, blank line or any other comment splits `///` runs, `////` is regular, regular comments don't drop a pending doc comment). I found no input where the 1b/combined source-gap check and the PR's `CommentReader` disagree (argument below).
- The perf story should not be used to justify the PR: `main + opt1` (−1.82% vs main) is equal or better than `PR + combined` (−1.72% vs merge base, and merge base ≈ main for the parser stage within ±0.005%). Skipping trivia is an orthogonal win; the PR itself is still a ~0.1–0.5% parser-stage regression on an equal footing, i.e. ~0.05–0.2% end to end (parser is 42–50% of pipeline instructions). That is noise; the PR should be judged on correctness and simplicity, where it wins.
- Recommend: merge PR 2255 with opt2 + opt3 + 1b's merge-by-source-gap. Land trivia skipping (1b) as a separate, clearly temporary PR, unless the team is sure the "include trivia" TODO is far off.

## 1. Correctness

**PR as is** (`parser/natspec.rs` `CommentReader::read`): the state machine is sound. Edge cases checked: `\r`-only and CRLF (`EndOfLine` is one lexeme for `\r\n`, so `AfterLine → AfterLineBreak` once; a stray `\r \n` becomes two line breaks and correctly splits), `///` at EOF (dropped, same as solc: no documented token), `////` (longest match → `SingleLineComment`, splits a run but keeps the earlier comment pending, as solc does), `/** */` after `///` (replaces it), lexical-context switches (morphs only happen on real tokens, so two natspec lexemes are never in different contexts without a token between them; `is_single_line_natspec` covers all three kinds anyway), UNRECOGNIZED (non-trivia → the comment attaches to it; parse fails anyway). Unicode whitespace (`\u{a0}`, `\u{2028}`) is UNRECOGNIZED in both slang and solc-ish (solc errors too), so no divergence that matters. Form feed / vertical tab: solc's `isUnicodeLinebreak` (0x0a–0x0d) ends a `///` comment at `\x0b`/`\x0c`; slang's `[^\r\n]*` swallows them into the line. Pre-existing, not introduced here; worth a test and a comment at most.

The `debug_assert`s in `attach_natspec_to` / `pop_documenting` are unchanged and still hold because the token stream remains monotonic.

**1b / combined gap check** (`is_line_continuation`): equivalent to the PR's machine. Proof sketch: between two natspec lexemes with no token between them, the gap consists only of skipped trivia: `( |\t)+`, `\n|\r\n?`, `//[^\r\n]*`, `/\*…\*/`. The PR merges iff the trivia sequence is exactly `EndOfLine, Whitespace*`. Every `//`/`/*` lexeme contains `/`, a second line break contains `\n`/`\r`, so "gap = one line break + `[ \t]*`" is exactly that condition. CRLF is handled by stripping `"\r\n"` before `['\n','\r']`. Byte slicing is safe since logos spans are char boundaries. Note 1b is actually *more* robust than the PR's machine: it doesn't care which trivia lexemes exist, so it keeps working unchanged when trivia lexemes come back. The `debug_assert!(!lexeme.kind.is_trivia())` in `next()` can't fire (UNRECOGNIZED isn't trivia).

**opt1 (skip ws + `//` only)**: safe for the reason given (a `//` comment always ends at a line break, so a run is still split by two `EndOfLine`s). Fine, but it leaves the state machine in place, so it's the worst of both.

**opt2**: a faithful refactor; the `pending` field reproduces `read_natspec_comment`'s loop. No behaviour change.

**opt3**: fine, but it's built on the PR base; upstream main (9e2a8a50) added a *second* trim in `memory_safety.rs:67-70` (`text.trim_end_matches(['\r','\n'])` for the reported range). On rebase, delete that too, otherwise the comment lies.

## 2. Code improvements

- **Codegen mechanism**: a per-item `skipped = true` in the language definition is an implementation detail leaking into the language model (`language-v2/.../trivia.rs`), and nothing validates it (`p3_references` ignores it; `skipped = true` on a `NatSpecComment` would silently break NatSpec). Prefer either (a) a codegen policy "skip every trivia item whose category isn't `NatSpecComment`" with no model change, or (b) keep the flag but add a p3 check rejecting `skipped` on NatSpec items. The `skip_trivia` callback returning `FilterResult::Skip` is fine; `#[logos(skip …)]` is the same leaf in the DFA, and the callback keeps your explicit `priority`/`allow_greedy` handling, so there's no reason to switch.
- The `LexemeKind` enum still carries `Whitespace`/`EndOfLine`/`SingleLineComment`/`MultiLineComment` variants that are never produced, plus `unreachable!` arms in the generated terminal-kind mapping. Acceptable if the skipping is temporary; otherwise generate them out.
- Combined `natspec.rs`: `PendingComment.is_line` could be dropped by comparing `is_single_line_natspec` of the previous lexeme's kind, but the field is clearer. Rename `NonTriviaTokens` → `Tokens` or `TokenStream` once the lexer skips trivia itself (the doc comment already says it only filters NatSpec). The remaining `#[inline(never)]` on `attach_natspec_to`/`pop_documenting` are justified by comments; keep them, they're about the LALRPOP loop, not the lexer loop.
- Add unit tests for `is_line_continuation` (`"\n"`, `"\r\n  "`, `"\r"`, `"\n\n"`, `"\n\t\n"`, `"\n/* x */ "`, `""`) and a parser test for `\x0c`.
- PR commit has no body; the motivation for reverting the regex (DFA size? `////` handling? correctness of the greedy multi-line regex?) should be written down, since the benchmark doesn't argue for it.

## 3. Benchmark validity

- Callgrind instruction counts are deterministic enough: merge base vs main differ by ≤ ±0.005% in the parser stage (identical code), which is the noise floor. Single runs are OK for instructions.
- Geometric mean over 9 contracts is fine; the per-contract spread (−1.9% to −6.4% for combined) is comment density, and `pointer_libraries` is the outlier. Report the range, not only the mean.
- "Non-parser stages 0.00%" is accurate (≤0.006%), confirming the setup isolates parsing.
- The 11-commit gap doesn't bias anything: none of those commits touch the parser, and the numbers confirm it. But it does mean the honest comparison is `main-opt1` vs `2255-combined`, which the tables don't draw: they're within 0.1% of each other, with main-opt1 slightly ahead.
- Estimated cycles are a synthetic L1/LL/RAM-penalty model and are code-layout sensitive; opt2's cycles (−1.32%) vs instructions (−0.32%) and "combined > sum of parts" (−3.84% vs −3.26%) are inlining/layout artefacts that may flip with a toolchain bump. Quote instructions; treat cycles as corroboration only.
- The stage measured includes CST construction, so the lexer share is already diluted; the real-world effect of any option is ≤ 0.1 ms per file. Nobody will notice; the decision is about code shape.

## 4. Architecture

- **Merging in the token stream vs the lexer regex**: the token stream is the right layer. The regex version needed four fragments per context, encoded indentation/blank-line rules into a DFA, and still couldn't express "only the last comment counts". A small merge step next to `NatSpecStack` is where that rule belongs and is testable in isolation. The 1b formulation (merge by source gap) is the cleanest of the three.
- **Lexer stops emitting trivia (1/1b)**: for a tool whose stated goal is a full-fidelity CST, this is a step backwards that will be undone, and the ~3% will be given back then. It is cheap to undo (flip the flags) and 1b's merge logic survives the undo, so it's not harmful; but it's a temporary optimisation, and it should be labelled as such (`TODO(v2)` on the flags), separate from the PR.
- **opt3**: changing `NatSpec.range` to exclude the trailing line break is the right semantics (symmetric with `/** */`, no EOF special case, matches what the IR already trims). v2 is unreleased; do it now.

## 5. Verdict

| Option | Verdict |
|---|---|
| PR 2255 | Recommend, with changes below. The +2.2% parser-stage cost is ~1% end to end and buys a simpler lexer and correct solc-like semantics. |
| opt1 | Don't recommend on its own; superseded by 1b. |
| opt1b | Recommend the merge-by-gap logic unconditionally; the lexer skipping is optional and temporary. If adopted, use category-based codegen or validate the flag. |
| opt2 | Recommend: simpler, one `next_lexeme` call site, no measurable cost. |
| opt3 | Recommend; also remove the redundant trim on main's `memory_safety.rs:67-70`. |
| combined | Recommend with the changes above. Its gain over main is ≈0, so present it as "no regression" rather than "3.8% faster". |

**Path forward for PR 2255**: rebase on main; fold in opt2 + opt3 + 1b's `read_natspec_comment`/`is_line_continuation` (which only need `source` in the token stream, not trivia skipping); write the motivation in the PR body; add the gap unit tests. Then open a second small PR "lexer: skip non-NatSpec trivia (temporary, until trivia is in the CST)" with the codegen change; it is independently reviewable and independently revertible.
