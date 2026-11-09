"""fn-335 R1 dry run: the recorder's lines read for sense, nothing chosen.

Reads the dry run's line format, the recorder as committed at b2e606b23;
the final recorder.patch prints read.py's format instead.

Usage: python3 -I dry.py <log>... Reads the `lateral_records` lines
(TREE, WEIGHTS, POT, NOPOT, REC) and prints, per preset and lateral age:
the laterals recorded, those with no first light (grew nothing of their
own), those the tables refuse (span 0), the shed share, the light's range,
and the realised share of the potential for four quantities, each summed
over the subtree's ages with that age's weight per phytomer:
  phytomers 1, leaf leaf_area, section pipe^exponent, wood internode;
counted (each phytomer 1) and by scale (each phytomer its scale).
Then, on the provisional light bins below, the median share and its
spread (p90/p10), and the share of log-share variance the light bins and
the light-by-span bins hold (eta squared). The bins are the dry run's only,
for reading sense; the frozen protocol names its own.
"""
import math, re, sys
from collections import defaultdict

BINS = [0.0, 0.2, 0.4, 0.6, 0.8, 0.9, 0.95, 1.0001]
QUANT = ["phytomers", "leaf", "section", "wood"]


def weights(w):
    leaf, inter, pipe, exp = w
    return [1.0, leaf, pipe ** exp, inter]


def parse(path):
    trees = {}
    for line in open(path):
        m = re.search(r"(TREE|WEIGHTS|POT|NOPOT|REC) (.*)", line)
        if not m:
            continue
        kind, rest = m.group(1), m.group(2).split()
        yield kind, rest


def quantile(v, q):
    v = sorted(v)
    if not v:
        return float("nan")
    k = (len(v) - 1) * q
    lo, hi = int(math.floor(k)), int(math.ceil(k))
    return v[lo] + (v[hi] - v[lo]) * (k - lo)


def eta2(groups):
    allv = [x for g in groups.values() for x in g]
    if len(allv) < 2:
        return float("nan")
    mean = sum(allv) / len(allv)
    total = sum((x - mean) ** 2 for x in allv)
    between = sum(len(g) * (sum(g) / len(g) - mean) ** 2 for g in groups.values() if g)
    return between / total if total > 0 else float("nan")


def binof(x):
    for k in range(len(BINS) - 1):
        if BINS[k] <= x < BINS[k + 1]:
            return k
    return len(BINS) - 2


def main(paths):
    for path in paths:
        w, pot, rows = {}, {}, defaultdict(list)
        nopot, preset = defaultdict(int), None
        for kind, r in parse(path):
            if kind == "TREE":
                preset = r[0]
            elif kind == "WEIGHTS":
                w[int(r[1])] = weights([float(r[3]), float(r[5]), float(r[7]), float(r[9])])
            elif kind == "POT":
                pot[(int(r[2]), int(r[3]))] = [float(x) for x in r[4].split(",")]
            elif kind == "NOPOT":
                nopot[int(r[2])] += 1
            elif kind == "REC":
                pa, span = int(r[3]), int(r[14])
                light = None if r[9] == "none" else float(r[9])
                count = [float(x) for x in r[15].split(",")]
                scale = [float(x) for x in r[16].split(",")]
                rows[pa].append((light, float(r[10]), span, r[13] == "1", count, scale))
        print(f"== {preset} ({path.split('/')[-1]})")
        for pa in sorted(rows):
            recs = rows[pa]
            nolight = sum(1 for x in recs if x[0] is None)
            spans0 = sum(1 for x in recs if x[2] == 0)
            shed = sum(1 for x in recs if x[3]) / len(recs)
            lights = [x[0] for x in recs if x[0] is not None]
            print(f"age {pa}: {len(recs)} laterals, {nolight} no first light, "
                  f"{spans0} span 0, shed {shed:.2f}, light "
                  f"{min(lights, default=float('nan')):.3f}..{max(lights, default=float('nan')):.3f} "
                  f"median {quantile(lights, 0.5):.3f}")
            for qi, q in enumerate(QUANT):
                for how, col in (("counted", 4), ("scaled", 5)):
                    share, by_l, by_ls = [], defaultdict(list), defaultdict(list)
                    zero = 0
                    for x in recs:
                        if x[0] is None or (pa, x[2]) not in pot:
                            continue
                        p = sum(pot[(pa, x[2])][a] * w[a][qi] for a in range(len(w)))
                        g = sum(x[col][a] * w[a][qi] for a in range(len(w)))
                        if p <= 0:
                            continue
                        s = g / p
                        if s <= 0:
                            zero += 1
                            continue
                        share.append((x[0], s))
                        by_l[binof(x[0])].append(math.log(s))
                        by_ls[(binof(x[0]), x[2] // 5)].append(math.log(s))
                    if not share:
                        continue
                    ss = [s for _, s in share]
                    bins = []
                    for b in sorted(by_l):
                        v = [math.exp(y) for y in by_l[b]]
                        bins.append(f"[{BINS[b]:.2f},{min(BINS[b+1],1):.2f}) n{len(v)} "
                                    f"med {quantile(v,0.5):.3g} p90/p10 "
                                    f"{quantile(v,0.9)/max(quantile(v,0.1),1e-300):.3g}")
                    print(f"  {q} {how}: median share {quantile(ss,0.5):.3g} "
                          f"(p10 {quantile(ss,0.1):.3g}, p90 {quantile(ss,0.9):.3g}), "
                          f"zero {zero}, eta2 light {eta2(by_l):.2f}, light+span {eta2(by_ls):.2f}")
                    if q == "leaf" and how == "counted":
                        for b in bins:
                            print(f"    {b}")
        print("NOPOT by age:", dict(nopot))


main(sys.argv[1:])
