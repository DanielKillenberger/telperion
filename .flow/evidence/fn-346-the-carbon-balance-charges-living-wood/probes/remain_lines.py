"""fn-346 remain: lines shed by row every 10 years from a census log's
LINEEND lines: by their own balance (and how many had a paying child) and
with their bearer, and those that died.

Usage: remain_lines.py <log> [every]
"""
import re
import sys
from collections import defaultdict

every = int(sys.argv[2]) if len(sys.argv) > 2 else 10
by = defaultdict(lambda: defaultdict(int))
for line in open(sys.argv[1]):
    if not line.startswith("LINEEND"):
        continue
    y = int(line.split()[1])
    kv = dict(re.findall(r"(\w+)=([-\w.]+)", line))
    d = (y // every + 1) * every - 1
    row = "limb" if kv["pa"] == "6" else "bough"
    c = kv["cause"]
    key = "balance" if c == "balance" else "bearer" if c == "with_bearer" else "died"
    by[d][(row, key)] += 1
    if key == "balance" and int(kv["kids_paying"]) > 0:
        by[d][(row, "paying")] += 1
print(f"| years to | limbs shed by own balance (with a paying child) | limbs with bearer | limbs died | boughs shed by own balance (with a paying child) | boughs with bearer | boughs died |")
print("|---|---|---|---|---|---|---|")
for d in sorted(by):
    b = by[d]
    print(f"| {d} | {b[('limb','balance')]} ({b[('limb','paying')]}) | {b[('limb','bearer')]} | {b[('limb','died')]} | "
          f"{b[('bough','balance')]} ({b[('bough','paying')]}) | {b[('bough','bearer')]} | {b[('bough','died')]} |")
