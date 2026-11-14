"""fn-346 rule A/B run: one variant's census every 10 years from its
renewal log: height, spread, held, living tips (all, and the framework
rows' tips: limb, bough, branch), the stem line's living limbs and lowest
one, leaves, income, charge, births over deaths (the decade), sleeping
buds held and woken (the decade).

Usage: renew_curves.py <log> [every]   (every: 10 or 20 years; 10 unset)
"""
import re
import sys
from collections import defaultdict

path = sys.argv[1]
every = int(sys.argv[2]) if len(sys.argv) > 2 else 10
year, bal, living, asleep = {}, {}, defaultdict(dict), {}
births, deaths, woken = defaultdict(int), defaultdict(int), defaultdict(int)
for line in open(path):
    w = line.split()
    if not w:
        continue
    kv = dict(re.findall(r"(\w+)=([-\w.]+)", line))
    if w[0] == "YEAR":
        year[int(w[1])] = kv
    elif w[0] == "BAL":
        bal[int(w[1])] = kv
    elif w[0] == "LIVING":
        living[int(w[1])][int(kv["pa"])] = int(kv["tips"])
    elif w[0] == "ASLEEP":
        asleep[int(w[1])] = kv
    elif w[0] == "BIRTH":
        births[int(w[1])] += int(kv["n"])
        if kv["code"] == "1":
            woken[int(w[1])] += int(kv["n"])
    elif w[0] == "DEATH":
        deaths[int(w[1])] += int(kv["n"])

print("| year | height m | spread m | held | living tips | limb / bough / branch tips | limbs on stem | lowest limb m | leaves | income | charge | births / deaths | sleeping held | woken |")
print("|---|---|---|---|---|---|---|---|---|---|---|---|---|---|")
for y in sorted(bal):
    if (y + 1) % every:
        continue
    a, b, lv = year.get(y, {}), bal[y], living.get(y, {})
    low = a.get("lowest_limb", "nan")
    low = "-" if low.lower() == "nan" else f"{float(low):.1f}"
    bd = f"{births[y] / deaths[y]:.2f}" if deaths[y] else "-"
    print(f"| {y} | {float(a['height']):.1f} | {float(b['spread_x']):.1f} × {float(b['spread_y']):.1f} | "
          f"{int(a['held']):,} | {int(b['tips']):,} | {lv.get(6, 0)} / {lv.get(7, 0)} / {lv.get(9, 0):,} | "
          f"{a['limbs_live']} | {low} | {int(b['leaves']):,} | {float(b['income']):,.0f} | {float(b['charged']):,.0f} | "
          f"{bd} | {asleep.get(y, {}).get('held', '-')} | {woken[y]} |")
