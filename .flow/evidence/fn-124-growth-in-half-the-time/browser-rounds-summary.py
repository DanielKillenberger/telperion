"""Pools the five warm samples of every interleaved round per arm and fixture
(rounds-<arm>-<round>.json from mature-generation.mjs) and prints skeleton and
completed-frame medians, with each round's own medians beside them.
Usage: browser-rounds-summary.py <raw-dir> [arms...] > summary.json"""
import glob, json, statistics as st, sys
raw = sys.argv[1]; arms = sys.argv[2:] or ['base', 'nosimd', 'simd']
out = {}
for arm in arms:
    for path in sorted(glob.glob(f'{raw}/rounds-{arm}-*.json')):
        r = json.load(open(path))
        for row in r['rows']:
            k = f"{row['preset']}-{row['seed']}"; warm = row['samples'][1:]
            o = out.setdefault(k, {}).setdefault(arm, {'skeleton': [], 'frame': [], 'rounds': [], 'wasm': r['wasmSha256']})
            sk = [s['submitted']['stages']['skeletonMs'] for s in warm]; fr = [s['totalToCompletedFrameMs'] for s in warm]
            o['skeleton'] += sk; o['frame'] += fr; o['rounds'].append([round(st.median(sk), 1), round(st.median(fr), 1)])
for k, arms_ in out.items():
    for arm, o in arms_.items():
        o['skeleton_median'] = round(st.median(o['skeleton']), 2); o['frame_median'] = round(st.median(o['frame']), 2)
        print(f"{k:20} {arm:7} skeleton {o['skeleton_median']:6.1f} frame {o['frame_median']:6.1f} rounds(skel,frame) {o['rounds']}", file=sys.stderr)
        del o['skeleton'], o['frame']
print(json.dumps(out, indent=1))
