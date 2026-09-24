import subprocess, math
a = subprocess.run(['dump-base/target/release/dump'], capture_output=True, text=True).stdout.splitlines()
b = subprocess.run(['dump-cand/target/release/dump'], capture_output=True, text=True).stdout.splitlines()
for la, lb in zip(a, b):
    xa, xb = la.split(), lb.split()
    key = ' '.join(xa[:2])
    if la == lb: print(key, 'identical', xa[2]); continue
    if xa[2] != xb[2]: print(key, 'NODE COUNT', xa[2], xb[2]); continue
    worst = 0; moved = 0; parents = 0
    for na, nb in zip(xa[3:], xb[3:]):
        if na == nb: continue
        moved += 1
        pa, pb = na.split(','), nb.split(',')
        if pa[4:] != pb[4:]: parents += 1
        worst = max(worst, max(abs(float(u) - float(v)) for u, v in zip(pa[:4], pb[:4])))
    print(key, f'nodes {xa[2]} moved {moved} parent-changes {parents} max-delta {worst:.3e} m')
