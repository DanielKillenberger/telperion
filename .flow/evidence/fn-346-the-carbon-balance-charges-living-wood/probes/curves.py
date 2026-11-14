"""fn-346 run: the census curve of a grow every 10 years, from its log's
YEAR and BAL lines (rough layout, before the settle).

Usage: curves.py <log> [<log>...]
"""
import re
import sys

for path in sys.argv[1:]:
    year, bal = {}, {}
    for line in open(path):
        kv = dict(re.findall(r"(\w+)=([-\w.]+)", line))
        if line.startswith("YEAR "):
            year[int(line.split()[1])] = kv
        elif line.startswith("BAL "):
            bal[int(line.split()[1])] = kv
    print(f"\n{path.split('/')[-1]}\n")
    print("| year | height m | spread m | held | living tips | limbs | lowest limb m | income | charge | charge / income |")
    print("|---|---|---|---|---|---|---|---|---|---|")
    for y in sorted(bal):
        a, b = year.get(y, {}), bal[y]
        inc, ch = float(b["income"]), float(b["charged"])
        low = a.get("lowest_limb", "nan")
        low = "-" if low.lower() == "nan" else f"{float(low):.1f}"
        print(f"| {y} | {float(a['height']):.1f} | {float(b['spread_x']):.1f} × {float(b['spread_y']):.1f} | "
              f"{int(a['held']):,} | {int(b['tips']):,} | {a['limbs_live']} | {low} | {inc:,.0f} | {ch:,.0f} | "
              f"{ch / inc if inc else float('nan'):.2f} |")
