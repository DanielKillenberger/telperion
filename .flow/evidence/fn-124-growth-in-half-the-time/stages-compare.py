"""R4: native CPU build stages (examples/generation_stages.rs) and peak RSS
for base and candidate, alternated per round, one process per fixture per
round, first build dropped; peak RSS is each process's wait4 ru_maxrss.
Usage: stages-compare.py <base-binary> <candidate-binary> [rounds] > summary.json"""
import json, os, statistics as st, subprocess, sys
base, cand = sys.argv[1], sys.argv[2]
rounds = int(sys.argv[3]) if len(sys.argv) > 3 else 3
stages = ['growth', 'rings', 'surface', 'placement', 'cull', 'total']
out = {}
fixtures = os.environ.get('FIXTURES', 'oregon-white-oak:1,oregon-white-oak:7,norway-spruce:1,norway-spruce:7')
for preset, seed in [(f.split(':')[0], int(f.split(':')[1])) for f in fixtures.split(',')]:
    if True:
        acc = {a: {'ms': {s: [] for s in stages}, 'rss_kib': [], 'hash': set()} for a in ['base', 'candidate']}
        for r in range(rounds):
            arms = [('base', base), ('candidate', cand)]
            for arm, binary in (arms if r % 2 == 0 else arms[::-1]):
                p = subprocess.Popen([binary, preset, str(seed)], stdout=subprocess.PIPE, text=True)
                text = p.stdout.read(); _, status, usage = os.wait4(p.pid, 0)
                assert status == 0, (arm, preset, seed)
                rows = [json.loads(l) for l in text.splitlines() if '"sample"' in l]
                for row in rows[1:]:
                    for s in stages: acc[arm]['ms'][s].append(row['milliseconds'][s])
                acc[arm]['hash'] |= {row['output_fnv1a64'] for row in rows}
                acc[arm]['rss_kib'].append(usage.ru_maxrss)
        key = f'{preset}-{seed}'; out[key] = {}
        for arm, a in acc.items():
            out[key][arm] = {'median_ms': {s: round(st.median(v), 3) for s, v in a['ms'].items()},
                             'peak_rss_kib_median': st.median(a['rss_kib']), 'hash': sorted(a['hash'])}
        b, c = out[key]['base']['median_ms'], out[key]['candidate']['median_ms']
        out[key]['change_pct'] = {s: round((c[s] / b[s] - 1) * 100, 2) for s in stages}
        out[key]['identical'] = out[key]['base']['hash'] == out[key]['candidate']['hash']
        print(key, out[key]['change_pct'], 'rss', out[key]['base']['peak_rss_kib_median'], out[key]['candidate']['peak_rss_kib_median'],
              'identical' if out[key]['identical'] else 'DIFFERENT', file=sys.stderr)
print(json.dumps(out, indent=1))
