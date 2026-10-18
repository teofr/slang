"""Usage: parse_log.py <tool-result.json|raw.txt> <name> <outdir>: saves raw log and parsed metrics JSON."""
import json, re, sys
src, name, out = sys.argv[1:4]
txt = open(src).read()
try:
    txt = json.loads(txt)['logs_content']
except Exception:
    pass
open(f'{out}/{name}.log.txt', 'w').write(txt)
bench = None
res = {}
head = re.compile(r'^(slang_v2::\S+ [^:]+):')
metric = re.compile(r'^\s+([A-Za-z0-9 +\-]+?):\s+(\d+)\|')
for line in txt.split('\n'):
    line = re.sub(r'^\d{4}-\d\d-\d\dT[\d:.]+Z ', '', line)
    m = head.match(line)
    if m:
        bench = m.group(1); res[bench] = {}; continue
    m = metric.match(line)
    if m and bench:
        res[bench][m.group(1)] = int(m.group(2))
json.dump(res, open(f'{out}/{name}.json', 'w'), indent=1, sort_keys=True)
print(name, len(res), 'benches')
