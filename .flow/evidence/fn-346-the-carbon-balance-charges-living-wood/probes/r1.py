"""fn-346 R1: the corrected charge's arithmetic from the census files.

Usage: r1.py <raw/V-SEED dir> <section> [years...]
Sections: whole, bands, ranks, limbs, top.
The corrected charge: cost of a living tip = K x its path L; dead wood
nothing. K = 1 / 34.8 m: a tip of whole presence in full light (own 1, the
unit `upkeep` is stated in, species.rs) earns its cost at L = 34.8 m.
"""
import math
import re
import sys
from collections import defaultdict
from statistics import mean, median

K = 1.0 / 34.8
HAZARD, TOLERANCE = 2.0, 0.0  # oak.rs: balance_hazard, tolerance
PA = {2: "trunk", 3: "low fork", 4: "leader", 5: "top fork", 6: "limb", 7: "bough",
      8: "sprig", 9: "branch", 10: "branchlet", 11: "spur", 12: "shoot", 13: "twig", 14: "short"}


def tips(d, y):
    with open(f"{d}/tips-{y}.tsv") as f:
        next(f)
        for line in f:
            a = line.split("\t")
            # axis pa path height own presence light limb main
            yield int(a[1]), float(a[2]), float(a[3]), float(a[4]), int(a[7])


def limbs(d, y):
    with open(f"{d}/limbs-{y}.tsv") as f:
        next(f)
        for line in f:
            a = line.split("\t")
            yield dict(axis=int(a[0]), pa=int(a[1]), birth=int(a[2]), base=float(a[3]),
                       z=float(a[4]), q=float(a[5]), c=float(a[6]), memory=float(a[7]))


def bal_lines(d):
    out = {}
    for line in open(f"{d}/run.log"):
        if line.startswith("BAL "):
            y = int(line.split()[1])
            out[y] = {k: float(v) for k, v in re.findall(r"(\w+)=([-\d.]+)", line)}
    return out


def years(d):
    import os
    return sorted(int(f[5:-4]) for f in os.listdir(d) if f.startswith("tips-"))


def bal(q, c):
    return 1.0 if q + c <= 0 else (q - c) / (q + c)


def hazard(b):
    h = HAZARD * max(0.0, TOLERANCE - b)
    return 1.0 - math.exp(-h)


def whole(d, ys):
    b = bal_lines(d)
    print("| year | living tips | income | Σ L km | mean L m | K·ΣL | K·ΣL / income | today living | today dead | today charged | today / income | K·ΣL / today living |")
    print("|---|---|---|---|---|---|---|---|---|---|---|---|")
    for y in ys:
        t = list(tips(d, y))
        sl = sum(r[1] for r in t)
        inc = b[y]["income"]
        live, dead, ch = b[y]["upkeep_live"], b[y]["upkeep_dead"], b[y]["charged"]
        print(f"| {y} | {len(t):,} | {inc:,.0f} | {sl/1000:,.0f} | {sl/len(t):.1f} | {K*sl:,.0f} | {K*sl/inc:.2f} | "
              f"{live:,.0f} | {dead:,.0f} | {ch:,.0f} | {ch/inc:.2f} | {K*sl/live:.1f} |")


def bands(d, ys):
    for y in ys:
        by = defaultdict(list)
        for pa, L, z, own, limb in tips(d, y):
            by[int(L // 5)].append((own, own < K * L))
        print(f"\n{d.split('/')[-1]}, year {y}:\n")
        print("| L m | tips | share below K·L | mean own | median own | K·L at band middle |")
        print("|---|---|---|---|---|---|")
        for b in sorted(by):
            v = by[b]
            below = sum(1 for o, x in v if x) / len(v)
            owns = [o for o, _ in v]
            print(f"| {5*b} to {5*b+5} | {len(v):,} | {below:.2f} | {mean(owns):.3f} | {median(owns):.3f} | {K*(5*b+2.5):.3f} |")


def ranks(d, ys):
    for y in ys:
        by = defaultdict(list)
        for pa, L, z, own, limb in tips(d, y):
            by[pa].append((own, L))
        print(f"\n{d.split('/')[-1]}, year {y}:\n")
        print("| rank | tips | mean own | mean L m | share below K·L | L where the mean tip earns its cost (mean own / K) m |")
        print("|---|---|---|---|---|---|")
        for pa in sorted(by):
            v = by[pa]
            mo = mean(o for o, _ in v)
            below = sum(1 for o, L in v if o < K * L) / len(v)
            print(f"| {pa} {PA.get(pa, '')} | {len(v):,} | {mo:.3f} | {mean(L for _, L in v):.1f} | {below:.2f} | {mo / K:.1f} |")


def limb_rows(d, y):
    sub = defaultdict(lambda: [0, 0.0, 0.0])  # tips, Σ L, Σ own
    for pa, L, z, own, limb in tips(d, y):
        if limb >= 0:
            s = sub[limb]
            s[0] += 1
            s[1] += L
            s[2] += own
    rows = []
    for m in limbs(d, y):
        n, sl, so = sub.get(m["axis"], [0, 0.0, 0.0])
        ca = K * sl
        cb = K * (sl - n * m["base"])
        rows.append(dict(m, tips=n, ca=ca, cb=cb, now=bal(m["q"], m["c"]),
                         a=bal(m["q"], ca), b=bal(m["q"], cb)))
    return rows


def limbs_table(d, ys, all_rows):
    for y in ys:
        rows = limb_rows(d, y)
        rows.sort(key=lambda r: -r["q"])
        print(f"\n{d.split('/')[-1]}, year {y}: {len(rows)} living laterals on the main line\n")
        print("| axis | rank | base path m | base height m | tips | Q | today C | today now | today memory | hazard today | (a) C | (a) balance | hazard (a) | (b) C | (b) balance | hazard (b) |")
        print("|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|")
        shown = rows if all_rows else [r for r in rows if r["tips"] >= 50 or r["pa"] <= 7]
        for r in shown:
            print(f"| {r['axis']} | {r['pa']} {PA.get(r['pa'], '')} | {r['base']:.1f} | {r['z']:.1f} | {r['tips']:,} | {r['q']:,.1f} | {r['c']:,.1f} | "
                  f"{r['now']:+.2f} | {r['memory']:+.2f} | {hazard(r['memory']):.2f} | {r['ca']:,.1f} | {r['a']:+.2f} | {hazard(r['a']):.2f} | "
                  f"{r['cb']:,.1f} | {r['b']:+.2f} | {hazard(r['b']):.2f} |")
        small = [r for r in rows if r not in shown]
        if small:
            neg = lambda key: sum(1 for r in small if r[key] < 0)
            print(f"\n{len(small)} more laterals with under 50 tips (sprigs and smaller): "
                  f"below zero today (memory) {neg('memory')}, (a) {neg('a')}, (b) {neg('b')}.")
        tot = lambda key: sum(1 for r in rows if r[key] < 0)
        print(f"\nAll {len(rows)}: below zero today (memory) {tot('memory')}, (a) {tot('a')}, (b) {tot('b')}.")


def top(d, ys):
    """Summary of limbs (rank <= 7, the limb and bough rows and the fork rows)."""
    print("| year | limbs (rank ≤ 7) | below 0 today | below 0 (a) | below 0 (b) | mean shift (a) − today now | mean shift (b) − today now |")
    print("|---|---|---|---|---|---|---|")
    for y in ys:
        rows = [r for r in limb_rows(d, y) if r["pa"] <= 7]
        if not rows:
            continue
        neg = lambda key: sum(1 for r in rows if r[key] < 0)
        sa = mean(r["a"] - r["now"] for r in rows)
        sb = mean(r["b"] - r["now"] for r in rows)
        print(f"| {y} | {len(rows)} | {neg('memory')} | {neg('a')} | {neg('b')} | {sa:+.2f} | {sb:+.2f} |")


if __name__ == "__main__":
    d, sec = sys.argv[1], sys.argv[2]
    ys = [int(y) for y in sys.argv[3:]] or years(d)
    {"whole": whole, "bands": bands, "ranks": ranks, "top": top,
     "limbs": lambda d, ys: limbs_table(d, ys, False),
     "limbsall": lambda d, ys: limbs_table(d, ys, True)}[sec](d, ys)
