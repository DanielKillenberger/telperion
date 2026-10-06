#!/usr/bin/env python3
"""fn-211 task 2: before/after tables from `measures passes` lines.
   passes-tables.py before.txt after.txt
Each configuration ran seeds 1,1,7,7 in one process; the second build of
each seed is kept (the first is cold)."""
import re, sys
from collections import OrderedDict

def read(path):
    runs, seen = OrderedDict(), {}
    for line in open(path):
        if not line.startswith("passes "):
            continue
        f = line.split()
        kv = dict(x.split("=", 1) for x in f[2:])
        key = (f[1], int(kv.pop("age")), int(kv.pop("seed")), int(kv.pop("cores")))
        kv.pop("extinction")
        seen[key] = seen.get(key, 0) + 1
        if seen[key] == 2:
            runs[key] = {k: float(v) for k, v in kv.items()}
    return runs

before, after = read(sys.argv[1]), read(sys.argv[2])
order = {"palm": 0, "beech": 1, "oak": 2, "spruce": 3}
keys = sorted(before, key=lambda k: (k[1], order[k[0]], k[2], -k[3]))
fmt = lambda v: f"{v:,.0f}" if v >= 100 else f"{v:.1f}" if v >= 1 else f"{v:.2f}"
print("| Tree | Cores | Load before / after | Total before | Total after | Change |")
print("|---|--:|--:|--:|--:|--:|")
for k in keys:
    if k not in after:
        continue
    b, a = before[k]["total"], after[k]["total"]
    cores = "all" if k[3] > 1 else "1"
    print(f"| {k[0]} {k[1]} s{k[2]} | {cores} | {before[k]['load']:.2f} / {after[k]['load']:.2f} | {fmt(b)} | {fmt(a)} | {100*(a-b)/b:+.0f}% |")
passes = ["Windows", "Balance", "Allot", "Homes", "Units", "Merge", "Wake", "Sketch",
          "RelaySettle", "RelayPlace", "RelayLevers", "RelaySag", "RelayWrite",
          "ShadeField", "ShadeRead", "Mark", "Assign", "Attachments", "Shed", "Disused",
          "Scale", "Thicken", "LeafMean", "Place", "Levers", "SagPlace", "gaps", "tail", "total"]
for tree in [("oak", 80), ("spruce", 80), ("beech", 80), ("oak", 40), ("spruce", 40), ("beech", 40)]:
    cols = [k for k in keys if k[:2] == tree and k in after]
    print(f"\n**{tree[0]} {tree[1]}** (ms; before / after)\n")
    print("| Pass | " + " | ".join(f"s{k[2]} {'all' if k[3] > 1 else '1'}" for k in cols) + " |")
    print("|---|" + "--:|" * len(cols))
    for p in passes:
        cells = [(before[k].get(p, 0.0), after[k].get(p, 0.0)) for k in cols]
        if all(b < 0.5 and a < 0.5 for b, a in cells):
            continue
        print(f"| {p} | " + " | ".join(f"{fmt(b)} / {fmt(a)}" for b, a in cells) + " |")
