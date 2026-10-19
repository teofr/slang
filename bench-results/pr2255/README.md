# PR 2255 NatSpec lexing benchmarks

Parser-stage results of dry-run `slang_v2` gungraun benchmarks for NomicFoundation/slang PR 2255 and the follow-up options. Page: https://claude.ai/artifact/H1qmXQz8pwRcpYL44mb45S

- `metrics/*.json`: per-benchmark Callgrind/DHAT metrics, parsed from each run's log
- `logs/*.log.txt.gz`: raw job logs
- `review.md`: independent review of the changes
- `scripts/`: log parser and comparison scripts

## Results

| Variant | Compared to | Instructions | Est. cycles | Total bytes | Total blocks | vs PR merge base (instr.) |
|---|---|---|---|---|---|---|
| PR 2255 (as is) | `2255-mergebase-baseline` | +2.20% | +2.74% | +0.00% | +0.00% | +2.20% |
| 1: skip whitespace + `//` | `2255-baseline` | -2.88% | -2.70% | +0.00% | +0.00% | -0.74% |
| 1b: skip all regular trivia, merge from source | `2255-baseline` | -2.94% | -2.66% | +0.00% | +0.00% | -0.80% |
| 2: pending comment field | `2255-baseline` | -0.32% | -1.32% | +0.00% | +0.00% | +1.88% |
| 3: range without line break | `2255-baseline` | -0.00% | -0.07% | +0.00% | +0.00% | +2.20% |
| 1b+2+3 combined | `2255-baseline` | -3.84% | -3.82% | +0.00% | +0.00% | -1.72% |
| 1 on main | `main-baseline` | -1.82% | -1.68% | +0.00% | +0.00% | — |

| Contract | PR 2255 instr. | PR vs merge base | 1 | 1b | 2 | 3 | combined | main | 1 on main |
|---|---|---|---|---|---|---|---|---|---|
| cooldogs | 11,406,998 | +1.08% | -1.69% | -1.32% | -0.11% | +0.00% | -1.92% | 11,284,958 | -1.11% |
| create_x | 12,521,260 | +1.76% | -2.42% | -2.34% | -0.22% | +0.00% | -3.13% | 12,304,415 | -1.50% |
| ens_registrar_controller | 11,517,108 | +1.94% | -2.79% | -2.92% | -0.23% | -0.00% | -3.75% | 11,297,865 | -1.78% |
| merkle_proof | 1,007,075 | +1.93% | -2.76% | -2.68% | -0.19% | -0.00% | -3.43% | 987,993 | -1.71% |
| multicall3 | 1,144,474 | +2.45% | -2.78% | -2.73% | -0.37% | -0.00% | -3.64% | 1,117,093 | -1.71% |
| one_step_leverage_f | 11,099,336 | +2.63% | -2.98% | -3.20% | -0.38% | -0.00% | -4.13% | 10,814,993 | -1.83% |
| pointer_libraries | 8,691,987 | +3.28% | -4.40% | -5.06% | -0.64% | -0.00% | -6.42% | 8,416,631 | -2.91% |
| ui_pool_data_provider_v3 | 12,298,507 | +2.20% | -3.20% | -3.34% | -0.30% | -0.00% | -4.27% | 12,034,079 | -2.08% |
| uniswap | 82,966,353 | +2.58% | -2.84% | -2.82% | -0.41% | -0.02% | -3.78% | 80,886,343 | -1.72% |

- PR 2255 (as is): largest geometric-mean instructions change in a non-parser stage: 0.00%
- 1: skip whitespace + `//`: largest geometric-mean instructions change in a non-parser stage: 0.00%
- 1b: skip all regular trivia, merge from source: largest geometric-mean instructions change in a non-parser stage: 0.00%
- 2: pending comment field: largest geometric-mean instructions change in a non-parser stage: 0.00%
- 3: range without line break: largest geometric-mean instructions change in a non-parser stage: 0.00%
- 1b+2+3 combined: largest geometric-mean instructions change in a non-parser stage: 0.00%
- 1 on main: largest geometric-mean instructions change in a non-parser stage: 0.00%

Combined vs main+opt1: +0.10% parser instructions.

## Option 1c on main

`bench/main-opt1c`: skip all regular trivia on main (main already merges `///` lines in its lexer).

- 1c vs main: −2.12% parser instructions (−1.74% est. cycles); vs main+1: −0.31%.
- Combined PR vs main+1c: +0.41% instructions (+0.46% cycles), between +0.28% and +0.63% per contract. This is the like-for-like cost of PR 2255.
