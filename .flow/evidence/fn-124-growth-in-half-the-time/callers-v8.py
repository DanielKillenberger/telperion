"""Immediate callers of named functions in a V8 profile (ms per profiled call).
Usage: callers-v8.py <profile.json> <substring>..."""
import json, collections, re, sys
d = json.load(open(sys.argv[1])); p = d['profile']; calls = len(d['skeletonMs'])
nodes = {n['id']: n for n in p['nodes']}; parent = {}
for n in p['nodes']:
    for c in n.get('children', []): parent[c] = n['id']
self_us = collections.Counter()
for s, dt in zip(p['samples'], p['timeDeltas'][1:] + [0]): self_us[s] += dt
nm = lambda i: re.sub(r'\[[0-9a-f]{16}\]|telperion_core::', '', nodes[i]['callFrame']['functionName'])
for target in sys.argv[2:]:
    callers = collections.Counter()
    for s, us in self_us.items():
        i = s; chain = []
        while i in nodes: chain.append(nm(i)); i = parent.get(i)
        for k, n in enumerate(chain):
            if target in n:
                callers[chain[k + 1] if k + 1 < len(chain) else '-'] += us; break
    print(target, {k[:60]: round(v / 1000 / calls, 2) for k, v in callers.most_common(6)})
