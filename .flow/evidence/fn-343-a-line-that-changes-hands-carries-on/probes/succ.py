"""fn-343 R1: on take-over joints >= 10 deg with the bearer 1 cm or more,
the bearer's own successor (relay or continuation) beside the lateral taken.
Run: python3 -I succ.py <log> [<log> ...]"""
import sys
from collections import Counter
for path in sys.argv[1:]:
    c, ratios = Counter(), {}
    for line in open(path):
        if not line.startswith("JOINT") or not ("cause=fork" in line or "cause=lateral_at_end" in line):
            continue
        d = dict(w.split("=", 1) for w in line.split()[4:] if "=" in w)
        if float(d["turn"]) < 10 or float(d["bearer_r"]) < 0.01:
            continue
        stop = "living" if d["apex_end"] == "-1" else d["stop"].split("@")[0]
        grp = "moved_on" if stop == "moved_on" else ("failed/aborted" if stop.startswith(("fail", "abort")) else stop)
        best = None
        for e in d["succ"].strip("[]").split(","):
            if e:
                kind, pa, r, n = e.split(":")
                if int(n) > 0 and (best is None or float(r) > best[1]):
                    best = (kind, float(r))
        lat = float(d["lat_r"])
        key = (grp, best[0] if best else "no successor with wood")
        c[key] += 1
        if best:
            ratios.setdefault(key, []).append(best[1] / lat)
    print(path.split("/")[-1])
    for k, v in sorted(c.items(), key=lambda kv: -kv[1]):
        r = sorted(ratios.get(k, []))
        med = f", its base radius over the lateral's, median {r[len(r) // 2]:.2f}" if r else ""
        print(f"  {k[0]:<16} {k[1]:<24} {v}{med}")
