"""fn-346 R1: what the lit tips at the top of the crown read.

Usage: lit.py <raw/V-SEED dir> <year>...
Per year: the presence and own light of tips in full light (light >= 0.95),
and of the tips within 1 m of the tree's top tip.
"""
import sys
from statistics import mean, median


def rows(path):
    with open(path) as f:
        next(f)
        for line in f:
            a = line.split("\t")
            yield int(a[1]), float(a[2]), float(a[3]), float(a[4]), float(a[5]), float(a[6])


def q(xs):
    if not xs:
        return "-"
    s = sorted(xs)
    pick = lambda p: s[min(len(s) - 1, int(p * len(s)))]
    return f"n={len(s)} mean={mean(s):.3f} p10={pick(.1):.3f} median={median(s):.3f} p90={pick(.9):.3f} max={s[-1]:.3f}"


d = sys.argv[1]
for y in sys.argv[2:]:
    t = list(rows(f"{d}/tips-{y}.tsv"))
    top = max(r[2] for r in t)
    full = [r for r in t if r[5] >= 0.95]
    high = [r for r in t if r[2] >= top - 1.0]
    print(f"year {y}: tips {len(t)}, top tip {top:.2f} m")
    print("  light>=0.95 presence", q([r[4] for r in full]))
    print("  light>=0.95 own     ", q([r[3] for r in full]))
    print("  top 1 m presence    ", q([r[4] for r in high]))
    print("  top 1 m own         ", q([r[3] for r in high]))
    print("  top 1 m light       ", q([r[5] for r in high]))
    print("  all presence        ", q([r[4] for r in t]))
