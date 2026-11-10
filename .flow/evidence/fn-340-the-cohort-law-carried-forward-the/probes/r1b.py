"""fn-340 R1b: read the `TRACE_DUMP` lines (probes/coupled2.patch) and split the
upkeep the cohort ages hand their structural bearers' balance, the cycle's
against the probe's, by the age of the bearing unit, open against closed, and
by size and presence.

Run: python3 -I r1b.py <dump log>
"""
import sys
from collections import defaultdict

UPKEEP = 0.35  # the oak's, every age (oak.rs `state`)


def load(path):
    c, p = [], []
    for line in open(path):
        w = line.split()
        if not w:
            continue
        if w[0] == "DUMPC":
            v = [float(x) for x in w[1:]]
            c.append(dict(cycle=int(v[0]), key=(w[2], w[3], w[4]), pa=int(v[4]), open=int(v[5]),
                          cost=v[6], frozen=v[7], light=v[8], apexes=v[9], lit=v[10], count=v[11],
                          w1=v[12], w=v[13], s1=v[14], s2=v[15], s3=v[16], p1=v[17], wood=v[18]))
        elif w[0] == "DUMPP":
            v = [float(x) for x in w[1:]]
            p.append(dict(cycle=int(v[0]), key=(w[2], w[3], w[4]), pa=int(v[4]), count=v[5], scale=v[6],
                          size=v[7], wood=v[8], living=v[9], light=v[10], stand=v[11], dead=int(v[12]),
                          up=v[13]))
    return c, p


def main():
    c, p = load(sys.argv[1])
    for cyc in sorted({r["cycle"] for r in c}):
        cc = [r for r in c if r["cycle"] == cyc]
        pp = [r for r in p if r["cycle"] == cyc]
        up_c = lambda r: r["cost"] if r["open"] else r["frozen"]
        tot_c = sum(up_c(r) for r in cc)
        tot_p = sum(r["up"] for r in pp)
        print(f"## cycle {cyc}\n")
        print(f"Upkeep handed to structural bearers: cycle {tot_c:.1f}, probe {tot_p:.1f}, probe over cycle {tot_p / tot_c:.3f}.")
        op = sum(up_c(r) for r in cc if r["open"])
        cl = sum(up_c(r) for r in cc if not r["open"])
        print(f"Cycle: open subtrees {op:.1f} ({op / tot_c:.1%}), closed (frozen) {cl:.1f} ({cl / tot_c:.1%}).")
        print(f"Cycle light {sum(r['light'] for r in cc):.1f}; probe light "
              f"{sum(r['living'] * r['count'] * r['scale'] * r['light'] for r in pp if not r['dead']):.1f}.\n")
        print("By the bearing unit's age (cycle minus the cycle it was laid): upkeep, cycle open / cycle closed / probe.\n")
        print("| unit age | cycle open | cycle closed | probe | probe over cycle open | cycle open subtrees | probe classes |")
        print("|---|---|---|---|---|---|---|")
        by = defaultdict(lambda: [0.0, 0.0, 0.0, 0, 0])
        for r in cc:
            a = cyc - int(r["key"][2])
            by[a][0 if r["open"] else 1] += up_c(r)
            by[a][3] += r["open"]
        for r in pp:
            a = cyc - int(r["key"][2])
            by[a][2] += r["up"]
            by[a][4] += 1
        for a in sorted(by):
            o, cl_, pr, n, k = by[a]
            ratio = f"{pr / o:.3f}" if o else "-"
            print(f"| {a} | {o:.2f} | {cl_:.2f} | {pr:.2f} | {ratio} | {n} | {k} |")
        print()
        # Size and presence on the open subtrees, wood-weighted.
        w1 = sum(r["w1"] for r in cc if r["open"])
        w = sum(r["w"] for r in cc if r["open"])
        s2 = sum(r["s2"] * r["w1"] for r in cc if r["open"]) / w1
        s3 = sum(r["s3"] * r["w1"] for r in cc if r["open"]) / w1
        m1 = w / w1
        print(f"Cycle open subtrees, wood-weighted size moments: mean {m1:.3f}, second {s2:.3f} "
              f"(variance {s2 - m1 * m1:.3f}), third {s3:.3f}.")
        means = [r["s1"] for r in cc if r["open"] and r["w1"] > 0]
        mu = sum(means) / len(means)
        var_b = sum((x - mu) ** 2 for x in means) / len(means)
        print(f"Between subtrees, the spread of their mean sizes: mean {mu:.3f}, variance {var_b:.3f}.")
        pw = sum(r["size"] * r["wood"] * r["count"] * r["scale"] for r in pp)
        pw1 = sum(r["wood"] * r["count"] * r["scale"] for r in pp)
        print(f"Probe classes, wood-weighted size (bearer unit's): {pw / pw1:.3f}.")
        print(f"Wood at size 1 times presence: cycle open {w1:.2f} m (pencils' sized wood {sum(r['wood'] for r in cc if r['open']):.2f}), probe {pw1:.2f} m.")
        print(f"So of the open-subtree upkeep ratio, wood at size 1 {pw1 / w1:.3f} and size {(pw / pw1) / m1:.3f}.\n")


if __name__ == "__main__":
    main()
