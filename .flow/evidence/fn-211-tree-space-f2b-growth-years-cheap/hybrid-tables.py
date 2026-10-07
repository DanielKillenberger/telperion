#!/usr/bin/env python3
"""fn-211.6: HYBRID.md's tables from the probe's raw lines.
usage: hybrid-tables.py <run file>...  (the warm repeat, rep 1, where present)"""
import json, statistics, sys

T = ["balance", "allot", "coarse", "items_grow", "items_sketch", "handover", "relay", "shade", "mark"]
# Bytes as the CPU holds them (size_of, printed by the probe) and lean.
LINE_EXTRA = 48  # Line: units f64, successor Vec, root u64, horizon f64
NODE_DRAW = 8    # Draws::nodes, per phytomer


def load(path):
    reps, cur, end, loads = {}, None, {}, []
    for line in open(path):
        if line.startswith("REP "):
            cur = int(line.split()[1]); reps[cur] = []
        elif line.startswith("HYB "):
            reps[cur].append(json.loads(line[4:]))
        elif line.startswith("END "):
            e = json.loads(line[4:]); end[e["rep"]] = e
        elif line.startswith("LOAD"):
            loads.append(line.split()[1])
    rep = max(reps)
    return reps[rep], end.get(rep), loads, rep


def crossing(r):
    sz = r["sz"]
    axis_held = sz["axis"] + sz["draws"] + LINE_EXTRA
    phyt_held = sz["phytomer"] + NODE_DRAW
    unit_held = sz["unit"] + 8
    down_rec = r["live"] + (r["live0"] - r["live"]) + r["handed_axes"] + r["handed_phyt"]
    down_held = (24 * r["live"] + 4 * (r["live0"] - r["live"])
                 + axis_held * r["handed_axes"] + phyt_held * r["handed_phyt"] + 16 * r["handed_live"])
    down_lean = (12 * r["live"] + 4 * (r["live0"] - r["live"])
                 + 40 * r["handed_axes"] + 28 * r["handed_phyt"] + 4 * r["handed_live"])
    new_axes = r["axes1"] - r["axes0"]
    up_rec = new_axes + r["grown"] + r["live"] + r["live1"]
    up_held = axis_held * new_axes + phyt_held * r["grown"] + unit_held * r["live"] + 4 * r["live1"]
    up_lean = 40 * new_axes + 28 * r["grown"] + 24 * r["live"] + 4 * r["live1"]
    return dict(down_rec=down_rec, down_held=down_held, down_lean=down_lean,
                up_rec=up_rec, up_held=up_held, up_lean=up_lean, new_axes=new_axes)


for path in sys.argv[1:]:
    rows, end, loads, rep = load(path)
    tot = {k: sum(r["t"][i] for r in rows) for i, k in enumerate(T)}
    cpu = [sum(r["cpu"][i] for r in rows) for i in range(4)] if "cpu" in rows[0] else None
    print(f"## {path} (rep {rep}; load {' -> '.join(loads)})")
    if end:
        print(f"build {end['build']:.3f} s, years {end['years']:.3f}, settle {end['settle']:.3f}, roots {end['roots']}, fine {end['fine']}, coarse {end['coarse']}")
    print("phase totals s: " + ", ".join(f"{k} {v:.3f}" for k, v in tot.items()))
    if cpu:
        s = sum(cpu)
        print("items' growth core-seconds: homes {:.3f} units {:.3f} merge {:.3f} wake {:.3f} (merge share {:.1%})".format(*cpu, cpu[2] / s))
    keep = sum(tot[k] for k in ["balance", "allot", "coarse", "items_sketch", "handover", "relay", "shade"])
    print(f"CPU keeps (years, no marks) {keep:.3f} s; GPU-able items' growth {tot['items_grow']:.3f} s; marks {tot['mark']:.3f} s")
    cs = [crossing(r) for r in rows]
    for k in ["down_rec", "down_held", "down_lean", "up_rec", "up_held", "up_lean"]:
        v = [c[k] for c in cs]
        late = v[-11:]
        print(f"{k}: total {sum(v):,}  median/cycle {statistics.median(v):,.0f}  max {max(v):,} (cycle {v.index(max(v)) + 1})  last-10 median {statistics.median(late):,.0f}")
    print("grown phytomers (items) total {:,}; new axes {:,}; grown apexes {:,}".format(
        sum(r["grown"] for r in rows), sum(c["new_axes"] for c in cs), sum(r["live"] for r in rows)))
    print("cycle: items_grow ms, phyt, live, up_held KB, down_held KB")
    for r, c in zip(rows, cs):
        if r["cycle"] % 10 == 0 or r["cycle"] in (5, 79):
            print(f"  {r['cycle']}: {1e3*r['t'][3]:.1f} ms, {r['grown']:,}, {r['live']:,}, {c['up_held']/1e3:,.0f}, {c['down_held']/1e3:,.0f}")
    print()
