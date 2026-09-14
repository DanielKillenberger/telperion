"""Summarize measurements before changing any identity/audit/look pin."""
import json
from pathlib import Path
p = Path(__file__).resolve().parent
names = ['oregon-white-oak','norway-spruce','ordinary','telperion','laurelin']
def mature(path):
    return max((json.loads(x) for x in path.read_text().splitlines() if json.loads(x)['kind']=='growth'),key=lambda x:x['age'])
def coords(a):
    return '('+', '.join(f'{v:.6f}' for v in a)+')'
s = ['# Convergence before the single re-pin\n',
     "The wire audit's HELD inventory is also recorded before its first move: 28\npaths become 32. Add growth.juvenileBranching, shootStep, thickeningDelay,\nthickeningShape and vigourFloor, whose numeric values are shared by every row;\nremove skeleton.habit.sheddingThreshold, now varied (0 versus the restored\n0.45). The exact set equality and the assertions proving all varying paths\nblend are unchanged. The first full gate exposed this stale schema inventory\nafter the geometry pins moved; no geometry, identity or look pin moves again.\n",
     'Pre-change measurements are from base ccb44eaf609c15e4df7170385f81b372b0d5f532, seed 7. Both columns use production growth at derived maturity. Bounds are node bounds in metres, excluding bark and leaves.\n',
     '| Preset / state | Age | Nodes | Crossover | Min bounds | Max bounds |',
     '|---|---:|---:|---:|---|---|']
deltas=[]
for name in names:
    old=mature(p/'prechange'/f'{name}.jsonl'); new=mature(p/'measurements'/f'{name}.jsonl')
    for label,row in [('before',old),('after',new)]:
        s.append(f'| {name} / {label} | {row["age"]:g} | {row["nodes"]:,} | {row["crossover"]:,} | {coords(row["bounds"]["min"])} | {coords(row["bounds"]["max"])} |')
    d=new['nodes']-old['nodes']; c=new['crossover']-old['crossover']
    deltas.append(f'\n{name}: nodes {d:+,} ({d/old["nodes"]*100:+.2f}%), crossover {c:+,} ({c/old["crossover"]*100:+.2f}%).\n')
s.extend(deltas)
s.append('''
Permanent structural buds no longer flush prematurely as local terminals. Variable juvenile steps retain actual axis length; seedling shoots fill current space before planning adult extensions. The age-dependent pipe scale changes when local stations become eligible. Those mechanisms change the number and position of wood segments and their leaf contacts. Restoration of the 0.45 threshold also removes shaded shoots on Ordinary and the Two Trees. The three mature crowns remain far above tens of nodes; their exact populations are reported rather than assumed equal to the zero-threshold build.

The legacy envelope rows remain in each JSONL file as a separate comparison. Ordinary’s seed-42 legacy audit returns to hash 9848876633805652422 from 4584312898131064280 because its authored 0.45 threshold returns. The species legacy audit hashes and both element hashes must remain unchanged. Production skeleton/placement pins and mesh bounds/counts will move once to these measured populations. The Ordinary clay look pin will move once for the restored threshold and new grown form; its shaders, camera and drift limits remain unchanged.
''')
(p/'CONVERGENCE.md').write_text('\n'.join(s))
