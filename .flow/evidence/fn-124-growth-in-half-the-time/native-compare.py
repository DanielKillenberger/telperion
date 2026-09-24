"""Paired native skeleton timing: base and candidate growth_profile binaries,
alternated per round, one process per fixture per round, first build dropped.
Usage: native-compare.py <base-binary> <candidate-binary> [rounds] > summary.json
Fails when any candidate tree hash differs from base."""
import json, statistics as st, subprocess, sys
base, cand = sys.argv[1], sys.argv[2]
rounds = int(sys.argv[3]) if len(sys.argv) > 3 else 5
fixtures = [(p, s) for p in ['oregon-white-oak', 'norway-spruce'] for s in [1, 7]]
out = {}
for preset, seed in fixtures:
    runs = {'base': [], 'candidate': []}; hashes = {'base': set(), 'candidate': set()}
    for r in range(rounds):
        order = [('base', base), ('candidate', cand)]
        for label, binary in (order if r % 2 == 0 else order[::-1]):
            lines = subprocess.run([binary, preset, str(seed)], capture_output=True, text=True, check=True).stdout.splitlines()
            rows = [json.loads(l) for l in lines]
            runs[label] += [x['ms'] for x in rows[1:]]; hashes[label] |= {x['tree_fnv1a64'] for x in rows}
    m = {k: st.median(v) for k, v in runs.items()}
    out[f'{preset}-{seed}'] = {'median_ms': m, 'change_pct': (m['candidate'] / m['base'] - 1) * 100,
                               'identical': hashes['base'] == hashes['candidate'] and len(hashes['base']) == 1,
                               'hash': sorted(hashes['candidate']), 'samples': len(runs['base'])}
    print(preset, seed, {k: round(v, 2) for k, v in m.items()}, f"{out[f'{preset}-{seed}']['change_pct']:+.1f}%", 'identical' if out[f'{preset}-{seed}']['identical'] else 'DIFFERENT', file=sys.stderr)
print(json.dumps(out, indent=1))
