"""Skeleton and completed-frame medians of the five warm samples in each
mature-generation.mjs result given. Usage: browser-summary.py <result.json>..."""
import json, statistics as st, sys
for path in sys.argv[1:]:
    r = json.load(open(path))
    for row in r['rows']:
        warm = row['samples'][1:]
        sk = st.median(s['submitted']['stages']['skeletonMs'] for s in warm)
        fr = st.median(s['totalToCompletedFrameMs'] for s in warm)
        print(f"{path.split('/')[-1]:32} {row['preset']:17} {row['seed']} skeleton {sk:6.1f} frame {fr:6.1f} wasm {r['wasmSha256'][:12]}")
