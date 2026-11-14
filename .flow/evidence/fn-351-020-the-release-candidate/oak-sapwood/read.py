"""fn-351, OAK-SAPWOOD-BY-SIZE.md: the probes' logs as one table.

    python3 -I read.py <raw/oakwood dir>

A log is `<side>-<preset>-<age>-<seed>.log`, side `cand` (before) or
`oakwood` (after), holding the probe's DIGEST, TREE and PEAK lines.
"""
import glob
import json
import os
import re
import sys

rows = {}
for path in sorted(glob.glob(os.path.join(sys.argv[1], "*.log"))):
    m = re.match(r"(cand|oakwood)-(.+)-(\w+)-(\d+)\.log$", os.path.basename(path))
    if not m:
        continue
    side, preset, age, seed = m.groups()
    text = open(path).read()
    tree = re.search(r"^TREE (.*)$", text, re.M)
    digest = re.search(r"^DIGEST (\w+)$", text, re.M)
    peak = re.search(r"^PEAK seconds ([\d.]+) rss_gib ([\d.]+) exit (\d+)", text, re.M)
    if not (tree and peak):
        continue
    t = json.loads(tree.group(1))
    rows[(preset, t["age"], int(seed), side)] = (t, digest.group(1) if digest else "-", peak.groups())

print("| preset | age | seed | side | tree digest | height m | trunk at 1.3 m, cm | drawn leaf m² | spread m | nodes | s | GiB |")
print("|---|---|---|---|---|---|---|---|---|---|---|---|")
for (preset, age, seed, side), (t, digest, peak) in sorted(rows.items(), key=lambda r: (r[0][0], r[0][1], r[0][2], r[0][3])):
    dbh = "-" if t.get("dbh") is None else f"{t['dbh'] * 100:.1f}"
    spread = (t["spread_x"] + t["spread_z"]) / 2
    name = "before" if side == "cand" else "after"
    print(f"| {preset} | {age:g} | {seed} | {name} | `{digest}` | {t['height']:.1f} | {dbh} | {t['leaf_area']:.0f} | {spread:.1f} | {t['nodes']} | {peak[0]} | {peak[1]} |")

same = [(k[:3]) for k in rows if k[3] == "cand" and (*k[:3], "oakwood") in rows
        and rows[k][1] == rows[(*k[:3], "oakwood")][1] and {**rows[k][0], "seconds": 0} == {**rows[(*k[:3], "oakwood")][0], "seconds": 0}]
print()
print("Equal before and after, digest and every measure:", sorted(same))
