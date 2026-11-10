"""fn-342 R3: the trunk path at 300 (thickest wood from the root, TRUNK lines of
probes/turns.patch): which axis it was born as at each height, and its turns.

Run: python3 -I trunk.py <log>
"""
import sys
from collections import defaultdict

NAMES = {0: "seedling", 1: "sapling", 2: "trunk", 3: "low fork", 4: "leader", 5: "top fork", 6: "limb", 7: "bough", 9: "branch"}

runs = defaultdict(list)
for line in open(sys.argv[1]):
    w = line.split()
    if w and w[0] == "TRUNK":
        d = dict(x.split("=", 1) for x in w[3:])
        runs[w[1]].append(d)
for seed, rows in runs.items():
    print(f"## seed {seed}: trunk path {len(rows)} phytomers, top {float(rows[-1]['z']):.1f} m")
    print("| from m | to m | born as | origin | axes | born years | turns >= 10 deg | largest turn, deg | summed turn, deg |")
    print("|---|---|---|---|---|---|---|---|---|")
    seg = []
    def flush():
        if not seg:
            return
        t = [float(d["turn_next"]) for d in seg if d["turn_next"] != "NaN"]
        axes = {d["axis"] for d in seg}
        births = sorted({int(d["birth"]) for d in seg})
        print(f"| {float(seg[0]['z']):.1f} | {float(seg[-1]['z']):.1f} | {NAMES.get(int(seg[0]['pa']), seg[0]['pa'])} | "
              f"{seg[0]['origin']} and on | {len(axes)} | {births[0]}-{births[-1]} | {sum(x >= 10 for x in t)} | "
              f"{max(t, default=0):.1f} | {sum(t):.0f} |")
    for d in rows:
        if seg and d["pa"] != seg[-1]["pa"]:
            flush()
            seg = []
        seg.append(d)
    flush()
    print()
