"""fn-344 follow-up 3: every 20 years (years 19, 39, ...), A, B and H side
by side from their grow logs: held phytomers, living tips, height (rough
layout), spread on both axes, ground area (1 m cells holding a phytomer
tip), lowest living limb and living limbs on the trunk line, resident GiB.

Usage: python3 -I probes/htable.py <seed> <label>=<log>...
"""
import re, sys


def read(path):
    rows = {}
    for line in open(path):
        m = re.match(r"CENSUS (\d+) (\d+) rss_gib=([\d.]+)", line)
        if m:
            rows.setdefault(int(m[1]), {})["rss"] = float(m[3])
        m = re.match(r"YEAR (\d+) held=(\d+) height=(\S+) stem_top=\S+ apexes=(\d+) limbs_live=(\d+) lowest_limb=(\S+)", line)
        if m:
            rows.setdefault(int(m[1]), {}).update(
                held=int(m[2]), h=float(m[3]), tips=int(m[4]), limbs=int(m[5]), low=float(m[6]))
        m = re.match(r"BAL (\d+) .*spread_x=(\S+) spread_y=(\S+) cells_m2=(\d+)", line)
        if m:
            rows.setdefault(int(m[1]), {}).update(sx=float(m[2]), sy=float(m[3]), cells=int(m[4]))
    stop = None
    for line in open(path):
        if line.startswith("STOP"):
            stop = line.split()[1]
    return rows, stop


runs = [a.split("=", 1) for a in sys.argv[2:]]
data = {k: read(v) for k, v in runs}
print("| year | v | held | living tips | height m | spread m | ground m² | lowest limb m | limbs | GiB |")
print("|---|---|---|---|---|---|---|---|---|---|")
for y in range(19, 300, 20):
    for k, _ in runs:
        rows, stop = data[k]
        r = rows.get(y)
        if not r or "held" not in r:
            continue
        low = "-" if r["low"] != r["low"] else f"{r['low']:.1f}"
        spread = f"{r['sx']:.1f} × {r['sy']:.1f}" if "sx" in r else "-"
        print(f"| {y} | {k} | {r['held']:,} | {r['tips']:,} | {r['h']:.1f} | {spread} | {r.get('cells', '-')} | {low} | {r['limbs']} | {r['rss']:.1f} |")
for k, _ in runs:
    rows, stop = data[k]
    if stop:
        print(f"\n{k} stopped: {stop}")
