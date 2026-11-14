"""fn-325: the best candidates at each cost level, from CAND lines."""
import re, sys
rows = []
for path in sys.argv[1:]:
    for line in open(path):
        i = line.find("CAND ")
        if i < 0:
            continue
        w = line[i:].split()[1:]
        rows.append({w[k]: float(w[k + 1]) for k in range(0, len(w) - 1, 2)})
keys = "first s leaf pipe bud hazard s2 x height spread trunk limbs shed area worst seconds".split()
print(" ".join(f"{k:>7}" for k in keys))
for cap in (0.05, 0.1, 0.25, 0.5, 1.0, 5.0):
    fit = sorted((r for r in rows if r["seconds"] <= cap), key=lambda r: r["worst"])[:4]
    print(f"-- at or under {cap} s: {sum(r['seconds'] <= cap for r in rows)} candidates")
    for r in fit:
        print(" ".join(f"{r.get(k, 1.0):7.3f}" for k in keys))
