import json, sys, statistics as st

T, L = {}, {}
for l in open(sys.argv[1]):
    d = json.loads(l)
    if 'error' in d:
        print('ERR', d)
        continue
    if 'points' in d:
        continue
    if d.get('tree'):
        T[d['seed']] = d
        L[d['seed']] = []
        continue
    L[d['seed']].append(d)
rows = []
for s, t in sorted(T.items()):
    low = [d for d in L[s] if d['pa'][0] == 8]
    rows.append(dict(
        seed=s, limbs=len(low), long=sum(d['len'] >= 3 for d in low),
        beyond=sum(d['tipreach'] > t['crown95'] for d in low),
        reach=max([d['tipreach'] for d in low], default=0) / t['crown95'],
        wood=sum(d['len'] for d in low),
        relay40=sum(1 for d in low for j in d['joins'] if j['kind'] == 'relay' and j['turn'] >= 40),
        kinks=sum(len(d['kinks']) for d in low), c95=t['crown95'], h=t['height'], phyt=t['phytomers']))
for k in ['limbs', 'long', 'beyond', 'reach', 'wood', 'relay40', 'kinks', 'c95', 'h', 'phyt']:
    v = [r[k] for r in rows]
    s7 = [r[k] for r in rows if r['seed'] == 7]
    print(f"{k}: median {st.median(v):.2f} p90 {sorted(v)[int(.9 * len(v))]:.2f} "
          f"max {max(v):.2f} seed7 {s7[0] if s7 else float('nan'):.2f}")
