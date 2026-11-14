"""fn-324: medians over seeds per candidate and age, as ratios to the reference."""
import re, statistics as st, sys
rows = {}
for path in sys.argv[1:]:
    for line in open(path):
        m = re.search(r"COARSE (\S+) age (\d+) seed (\d+) (.*)", line)
        if not m:
            continue
        w = m.group(4).split()
        d = {w[i]: float(w[i + 1]) for i in range(0, len(w), 2)}
        rows.setdefault((m.group(1), int(m.group(2))), []).append(d)
keys = ["height", "spread", "dbh", "limbs", "shed", "leaf"]
for age in sorted({a for _, a in rows}):
    ref = {k: st.median(d[k] for d in rows[("reference", age)]) for k in keys + ["seconds", "grown"]}
    print(f"age {age} reference", {k: round(v, 3) for k, v in ref.items()})
    for (label, a), ds in rows.items():
        if a != age or label == "reference":
            continue
        ratio = {k: st.median(d[k] for d in ds) / ref[k] for k in keys}
        worst = max(abs(v - 1) for v in ratio.values())
        secs = st.median(d["seconds"] for d in ds)
        print(f"{label:9} n={len(ds)} " + " ".join(f"{k} {ratio[k]:.2f}" for k in keys) + f" worst {worst:.2f} seconds {secs:.3f} ({secs / ref['seconds']:.3f}x)")
