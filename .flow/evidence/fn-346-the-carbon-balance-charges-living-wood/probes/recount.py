"""fn-346 run step 1: the restated charge recounted from R1's census files.

cost of a tip = size x L / LMAX (size = presence = base_scale x running),
light of a tip = size x lattice light; a subtree's cost is the sum over its
living tips over the whole path to the root; dead wood nothing.
Usage: recount.py <raw/V-SEED dir> <year>...
"""
import re
import sys
from collections import defaultdict

LMAX = 34.8


def tips(d, y):
    with open(f"{d}/tips-{y}.tsv") as f:
        next(f)
        for line in f:
            a = line.split("\t")
            # axis pa path height own presence light limb main
            yield float(a[2]), float(a[4]), float(a[5]), float(a[6]), int(a[7])


def income(d, y):
    for line in open(f"{d}/run.log"):
        if line.startswith(f"BAL {y} "):
            return float(re.search(r"income=([-\d.]+)", line).group(1))


d = sys.argv[1]
for y in map(int, sys.argv[2:]):
    t = list(tips(d, y))
    charge = sum(s * L / LMAX for L, o, s, l, m in t)
    inc = income(d, y)
    print(f"\n### {d.split('/')[-1]}, year {y}\n")
    print(f"Living tips {len(t):,}; income {inc:,.0f}; restated charge {charge:,.0f}; charge / income {charge / inc:.2f}.\n")
    bands = defaultdict(lambda: [0, 0])
    for L, o, s, l, m in t:
        b = bands[int(L // 5)]
        b[0] += 1
        b[1] += l < L / LMAX
    print("| L m | tips | share with lattice light < L / 34.8 |")
    print("|---|---|---|")
    for b in sorted(bands):
        n, below = bands[b]
        print(f"| {5*b} to {5*b+5} | {n:,} | {below / n:.2f} |")
    sub = defaultdict(float)
    for L, o, s, l, m in t:
        if m >= 0:
            sub[m] += s * L / LMAX
    rows = []
    with open(f"{d}/limbs-{y}.tsv") as f:
        next(f)
        for line in f:
            a = line.split("\t")
            axis, pa, base, q = int(a[0]), int(a[1]), float(a[3]), float(a[5])
            c = sub.get(axis, 0.0)
            rows.append((pa, axis, base, q, c, (q - c) / (q + c) if q + c > 0 else 1.0))
    big = [r for r in rows if r[0] <= 7]
    neg = [r for r in big if r[5] < 0]
    print(f"\nLimbs and boughs (rank <= 7) on the main line: {len(big)}; below zero {len(neg)}; "
          f"all laterals on the main line {len(rows)}, below zero {sum(1 for r in rows if r[5] < 0)}.")
    for r in sorted(big, key=lambda r: r[2])[:6]:
        print(f"  axis {r[1]} rank {r[0]} base path {r[2]:.1f} m: Q {r[3]:,.0f} C {r[4]:,.0f} balance {r[5]:+.2f}")
