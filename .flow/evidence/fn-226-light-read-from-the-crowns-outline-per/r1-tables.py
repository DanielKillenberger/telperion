#!/usr/bin/env python3
"""fn-226 R1: tables from raw/r1/r1.jsonl."""
import json, sys, os
rows = [json.loads(l) for l in open(os.path.join(os.path.dirname(__file__), "raw/r1/r1.jsonl"))]
key = lambda r: (r["age"], r["seed"])
lit = {key(r): r for r in rows if r["variant"] == "lit"}
unlit = {key(r): r for r in rows if r["variant"] == "unlit"}
groups = [
    ("balance (i)", ["balance"]),
    ("allot (ii, iii)", ["allot"]),
    ("rough layout (sketch)", ["coarse_sketch", "items_sketch"]),
    ("re-lay", ["relay_assemble", "relay_tree", "relay_disperse"]),
    ("gather", ["shade_gather"]),
    ("box", ["shade_box"]),
    ("deposit", ["shade_deposit"]),
    ("sweep", ["shade_sweep"]),
    ("sample", ["shade_read"]),
    ("write back", ["shade_write"]),
    ("growth, wake, handover", ["coarse_grow", "items_grow", "handover"]),
    ("settle", ["settle"]),
]
print("| Oak | Build s (unlit) | " + " | ".join(g[0] for g in groups) + " | untimed |")
print("|---|--:|" + "--:|" * (len(groups) + 1))
for k in sorted(lit):
    r = lit[k]
    cells = []
    tot = 0
    for _, ns in groups:
        t = sum(r["t_" + n] for n in ns)
        tot += t
        cells.append(f"{t:.3f} ({100*t/r['build']:.0f} %)")
    u = unlit.get(k, {}).get("build", float("nan"))
    print(f"| {k[0]} s{k[1]} | {r['build']:.3f} ({u:.3f}) | " + " | ".join(cells) + f" | {r['build']-tot:.3f} |")
print()
print("| Oak | Leaves deposited | Buds sampled | Lattice nodes swept (sum) | Re-lays | Phytomers re-laid | Cycles shaded |")
print("|---|--:|--:|--:|--:|--:|--:|")
for k in sorted(lit):
    r = lit[k]
    print(f"| {k[0]} s{k[1]} | {r['n_leaves']:,} | {r['n_tips']:,} | {r['n_nodes']:,} | {r['n_relays']} | {r['n_relay_phytomers']:,} | {r['n_cycles_shaded']} |")
print()
order = ["lit", "unlit", "flat", "norelay", "noallot", "nobalance", "nohazard", "nogirth"]
print("| Oak | Variant | Build s | Height m | Crown x / y m | Crown base live / major m | Kept | Grown | Shed % |")
print("|---|---|--:|--:|--:|--:|--:|--:|--:|")
for k in sorted(lit):
    for v in order:
        for r in rows:
            if r["variant"] == v and key(r) == k:
                print(f"| {k[0]} s{k[1]} | {v} | {r['build']:.2f} | {r['height']:.1f} | {r['crown_x']:.1f} / {r['crown_y']:.1f} | {r['base_live']:.2f} / {r['base_major']:.2f} | {r['kept']:,} | {r['grown']:,} | {r['shed']:.1f} |")
