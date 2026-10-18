"""compare.py <dir> <base> <variant> [metric...]: per-stage geometric-mean % change, and per-bench parser detail."""
import json, math, sys
d, base, var = sys.argv[1:4]
metrics = sys.argv[4:] or ['Instructions', 'Estimated Cycles', 'Total bytes', 'Total blocks']
B = json.load(open(f'{d}/{base}.json')); V = json.load(open(f'{d}/{var}.json'))
stages = sorted({k.split(' ')[0].split('::')[-1] for k in B})
print(f'{var} vs {base}')
for m in metrics:
    row = []
    for s in stages:
        logs = [math.log(V[k][m] / B[k][m]) for k in B if k.split(' ')[0].endswith('::' + s) and k in V and B[k][m] and V[k][m]]
        row.append(f'{s}={(math.exp(sum(logs)/len(logs))-1)*100:+.2f}%' if logs else f'{s}=n/a')
    print(f'  {m}: ' + '  '.join(row))
