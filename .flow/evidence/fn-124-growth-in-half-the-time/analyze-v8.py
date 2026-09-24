"""Attribute the skeleton stage of a V8 profile written by browser-profile.mjs.

Usage: analyze-v8.py <profile.json> [root-substring] [depth]
Finds every frame whose name contains root-substring (default: the pipeline's
skeleton stage), sums the sampled time beneath it, and prints the inclusive
and self time of each function under it, so the table's self column adds up
to the root's total.
"""
import json, sys, collections
path = sys.argv[1]
root_key = sys.argv[2] if len(sys.argv) > 2 else 'pipeline::skeleton'
data = json.load(open(path))
p = data['profile']
nodes = {n['id']: n for n in p['nodes']}
parent = {}
for n in p['nodes']:
    for c in n.get('children', []):
        parent[c] = n['id']
# Time per sample: the delta to the next sample.
deltas = p['timeDeltas'][1:] + [0]
self_us = collections.Counter()
for sid, dt in zip(p['samples'], deltas):
    self_us[sid] += dt
def name(i):
    f = nodes[i]['callFrame']
    return f['functionName'] or f"({f['url'].split('/')[-1]}:{f['lineNumber']})"
def chain(i):
    out = []
    while i in nodes:
        out.append(name(i)); i = parent.get(i)
    return out
calls = len(data['skeletonMs'])
def stage(names):  # outermost first; names as V8 reports them (physical functions)
    has = lambda part: any(part in n for n in names)
    if has('local::Frontier>::advance'):
        if has('Planner>::run'):
            if has('Curtain>::admits') or has('planner::rejected') or has('radius_toward'):
                return 'local advance / planner run / envelope admission'
            if has('Planner>::heading') or has('limit_turn'):
                return 'local advance / planner run / heading and turn limit'
            return 'local advance / planner run / rest'
        if has('Curtain>::admits') or has('planner::rejected'):
            return 'local advance / admission outside the planner'
        if has('Planner>::heading'):
            return 'local advance / twig heading outside the planner'
        if has('validate_range'):
            return 'local advance / validation'
        return 'local advance / loop body'
    if has('scaffold::Builder'):
        return 'scaffold advance'
    if has('local::Frontier>::seed'):
        return 'local seeding'
    if has('radius::solve'):
        return 'radius solve'
    if has('identify_range') or has('Frontier>::remap'):
        return 'identity and remap'
    return 'other: ' + names[-1][:60]
stages = collections.Counter()
total = 0.0; inclusive = collections.Counter(); exclusive = collections.Counter()
for sid, us in self_us.items():
    c = chain(sid)
    hit = [k for k, n in enumerate(c) if root_key in n]
    if not hit:
        continue
    total += us
    below = c[:hit[-1]]  # frames under the outermost root frame
    exclusive[c[0] if below else c[hit[-1]]] += us
    for n in set(below):
        inclusive[n] += us
    stages[stage(below[::-1] or [c[hit[-1]]])] += us
per = lambda us: us / 1000 / calls
print(f"root '{root_key}': {per(total):.2f} ms per call over {calls} calls; stage timer median {sorted(data['skeletonMs'])[calls//2]:.2f} ms")
print(f"{'inclusive':>9} {'self':>7}  function")
for n, us in sorted(inclusive.items(), key=lambda kv: -kv[1])[:int(sys.argv[3]) if len(sys.argv) > 3 and sys.argv[3].isdigit() else 40]:
    print(f"{per(us):9.2f} {per(exclusive[n]):7.2f}  {n[:150]}")
print(f"self-time rows sum: {per(sum(exclusive.values())):.2f} ms")
print("stage split (ms per call):")
for k, us in sorted(stages.items(), key=lambda kv: -kv[1]):
    print(f"{per(us):7.2f}  {k}")
if '--json' in sys.argv:
    json.dump({'skeleton_ms_per_call': per(total), 'stage_timer_ms': data['skeletonMs'],
               'stages_ms': {k: round(per(us), 3) for k, us in stages.items()},
               'functions_inclusive_ms': {n: round(per(us), 3) for n, us in inclusive.most_common(50)}},
              open(sys.argv[sys.argv.index('--json') + 1], 'w'), indent=1)
