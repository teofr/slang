import json, math, sys
d = sys.argv[1]
L = lambda n: json.load(open(f'{d}/{n}.json'))
runs = {n: L(n) for n in ['2255-mergebase-baseline','2255-baseline','2255-opt1','2255-opt1b','2255-opt2','2255-opt3','2255-combined','main-baseline','main-opt1']}
M = ['Instructions','Estimated Cycles','Total bytes','Total blocks','At t-gmax bytes']
def gm(base, var, stage, m):
    B, V = runs[base], runs[var]
    xs = [math.log(V[k][m]/B[k][m]) for k in B if f'::{stage} ' in k and B[k][m] and V[k][m]]
    return (math.exp(sum(xs)/len(xs))-1)*100 if xs else None
f = lambda x: 'n/a' if x is None else f'{x:+.2f}%'
out = {}
rows = [('PR 2255 (as is)','2255-mergebase-baseline','2255-baseline'),
        ('1: skip whitespace + `//`','2255-baseline','2255-opt1'),
        ('1b: skip all regular trivia, merge from source','2255-baseline','2255-opt1b'),
        ('2: pending comment field','2255-baseline','2255-opt2'),
        ('3: range without line break','2255-baseline','2255-opt3'),
        ('1b+2+3 combined','2255-baseline','2255-combined'),
        ('1 on main','main-baseline','main-opt1')]
md = ['| Variant | Compared to | Instructions | Est. cycles | Total bytes | Total blocks | vs PR merge base (instr.) |','|---|---|---|---|---|---|---|']
for name, b, v in rows:
    vals = [gm(b, v, 'parser', m) for m in M[:4]]
    mb = gm('2255-mergebase-baseline', v, 'parser', 'Instructions') if v.startswith('2255') else None
    out[v] = {'base': b, **{m: gm(b, v, 'parser', m) for m in M}, 'vs_mergebase_instructions': mb}
    md.append(f'| {name} | `{b}` | ' + ' | '.join(f(x) for x in vals) + f' | {f(mb) if mb is not None else "—"} |')
# per contract parser instructions
contracts = sorted({k.split(' ')[1] for k in runs['2255-baseline'] if '::parser ' in k})
md2 = ['| Contract | PR 2255 instr. | PR vs merge base | 1 | 1b | 2 | 3 | combined | main | 1 on main |','|---|---|---|---|---|---|---|---|---|---|']
pc = lambda b, v, c: (runs[v][f'slang_v2::pipeline::parser {c}']['Instructions']/runs[b][f'slang_v2::pipeline::parser {c}']['Instructions']-1)*100
for c in contracts:
    k = f'slang_v2::pipeline::parser {c}'
    md2.append(f'| {c} | {runs["2255-baseline"][k]["Instructions"]:,} | {f(pc("2255-mergebase-baseline","2255-baseline",c))} | ' +
               ' | '.join(f(pc('2255-baseline', v, c)) for v in ['2255-opt1','2255-opt1b','2255-opt2','2255-opt3','2255-combined']) +
               f' | {runs["main-baseline"][k]["Instructions"]:,} | {f(pc("main-baseline","main-opt1",c))} |')
# other stages max abs instructions change
md3 = []
for name, b, v in rows:
    mx = max(abs(gm(b, v, s, 'Instructions') or 0) for s in ['ast_analysis','ast_visitor','compute_contracts_abi','ir_builder','semantic'])
    md3.append(f'- {name}: largest geometric-mean instructions change in a non-parser stage: {mx:.2f}%')
json.dump(out, open(f'{d}/summary.json','w'), indent=1)
open(f'{d}/tables.md','w').write('\n'.join(md) + '\n\n' + '\n'.join(md2) + '\n\n' + '\n'.join(md3) + '\n')
print(open(f'{d}/tables.md').read())
