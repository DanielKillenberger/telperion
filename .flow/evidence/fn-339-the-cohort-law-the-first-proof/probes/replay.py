"""fn-339 R3 dry run: read one tree's `cohort_replay` output (the probe in
probes/replay.patch) and print the tables R3-DRY.md quotes.

Run: python3 -I replay.py <log> [<log> ...]
"""
import statistics
import sys

Q = ["grew", "living", "standing", "leaf_cur", "leaf_drawn", "wood", "section", "phyto"]
SIDE = Q + ["join", "roots", "balance"]
SIDES = ["cycle", "expected", "drawn"]
CELL_AREA = 0.25


def read(path):
    out = dict(reach={}, cohort={}, apex={}, cells=[], tree=None, head=None)
    for line in open(path):
        w = line.split()
        if not w:
            continue
        if w[0] == "HEAD":
            out["head"] = dict(preset=w[1], seed=int(w[2]), age=int(w[4]), seconds=float(w[6]))
        elif w[0] == "REACH":
            out["reach"][int(w[1])] = float(w[2])
        elif w[0] == "COHORT":
            out["cohort"][w[1]] = line.split(" ", 2)[2].strip()
        elif w[0] == "APEX":
            out["apex"][int(w[1])] = [int(x) for x in w[2:6]]
        elif w[0] == "CELL":
            v = [float(x) for x in w[1:]]
            cell = dict(key=tuple(int(x) for x in v[:3]), units=v[3], light=v[4])
            at = 5
            for side in SIDES:
                cell[side] = dict(zip(SIDE, v[at:at + len(SIDE)]))
                at += len(SIDE)
            cell["at_one"] = dict(leaf_cur=v[at], wood=v[at + 1])
            out["cells"].append(cell)
        elif w[0] == "TREE":
            v = [float(x) for x in w[1:7]]
            out["tree"] = dict(structure=dict(grew=v[0], leaf_cur=v[1], leaf_drawn=v[2]),
                               whole=dict(grew=v[3], leaf_cur=v[4], leaf_drawn=v[5]),
                               columns=(int(w[7]), int(w[8])), checked=int(w[9]), light_diff=float(w[10]))
    return out


def pct(xs, p):
    xs = sorted(xs)
    if not xs:
        return float("nan")
    k = (len(xs) - 1) * p
    lo = int(k)
    hi = min(lo + 1, len(xs) - 1)
    return xs[lo] + (xs[hi] - xs[lo]) * (k - lo)


def corr(a, b):
    if len(a) < 3:
        return float("nan")
    ma, mb = statistics.fmean(a), statistics.fmean(b)
    sa = sum((x - ma) ** 2 for x in a) ** 0.5
    sb = sum((y - mb) ** 2 for y in b) ** 0.5
    return sum((x - ma) * (y - mb) for x, y in zip(a, b)) / (sa * sb) if sa and sb else float("nan")


def report(d):
    h, t = d["head"], d["tree"]
    print(f"## {h['preset']}, seed {h['seed']}, age {h['age']} (replay {h['seconds']:.2f} s, four threads)\n")
    print("Cohort sets: " + "; ".join(f"{k} m {v}" for k, v in d["cohort"].items()))
    print(f"Lattice kept against the cycle's own reads: {t['checked']} apex reads, largest difference {t['light_diff']:.1e}.\n")

    print("| tree age | units grown | in cohorts at 0.25 m | at 0.5 m | at 1.0 m |")
    print("|---|---|---|---|---|")
    for c in (10, 20, 30, 40):
        if c in d["apex"]:
            a = d["apex"][c]
            print(f"| {c} | {a[0]} | {a[1] / a[0]:.3f} | {a[2] / a[0]:.3f} | {a[3] / a[0]:.3f} |")
    print()

    tot = {s: {q: sum(c[s][q] for c in d["cells"]) for q in SIDE} for s in SIDES}
    units = sum(c["units"] for c in d["cells"])
    print(f"Bearing units of the structure at 0.5 m: {units:.0f} in {len(d['cells'])} lattice nodes.\n")
    print("| quantity at the tree's age | cycle | law, births expected | ratio | law, the cycle's births | ratio |")
    print("|---|---|---|---|---|---|")
    for q in ["roots", "grew", "living", "standing", "leaf_cur", "leaf_drawn", "wood", "section", "join", "phyto"]:
        cy, ex, dr = tot["cycle"][q], tot["expected"][q], tot["drawn"][q]
        r = lambda x: f"{x / cy:.3f}" if cy else "-"
        print(f"| {q} | {cy:.4g} | {ex:.4g} | {r(ex)} | {dr:.4g} | {r(dr)} |")
    print(f"| roots the balance alone sheds (law) | - | {tot['expected']['balance']:.4g} | - | {tot['drawn']['balance']:.4g} | - |")
    one = {q: sum(c["at_one"][q] for c in d["cells"]) for q in ("leaf_cur", "wood")}
    print(f"\nThe cycle's cohorts at size 1: current-year leaf {one['leaf_cur']:.4g} "
          f"({one['leaf_cur'] / tot['cycle']['leaf_cur']:.3f} of sized), wood {one['wood']:.4g} "
          f"({one['wood'] / tot['cycle']['wood']:.3f}).\n")

    s, w = t["structure"], t["whole"]
    cols = t["columns"]
    law_units = s["grew"] + tot["expected"]["grew"]
    law_cur = s["leaf_cur"] + tot["expected"]["leaf_cur"]
    law_drawn = s["leaf_drawn"] + tot["expected"]["leaf_drawn"]
    print("| whole tree | cycle | structure + law (births expected) |")
    print("|---|---|---|")
    print(f"| new shoots in its last year | {w['grew']:.0f} | {law_units:.0f} |")
    print(f"| of which the structure's | {s['grew']:.0f} | {s['grew']:.0f} |")
    print(f"| current-year leaf, m2 | {w['leaf_cur']:.2f} | {law_cur:.2f} |")
    print(f"| standing leaf, m2 | {w['leaf_drawn']:.2f} | {law_drawn:.2f} |")
    print(f"| leaf-bearing 0.5 m columns | {cols[0]} | {cols[1]} |")
    print(f"| leaf area index over them (standing leaf) | {w['leaf_drawn'] / (cols[0] * CELL_AREA):.2f} | "
          f"{law_drawn / (cols[1] * CELL_AREA):.2f} |\n")

    print("Per lattice node, law (births expected) against the cycle, nodes where either holds leaf:\n")
    print("| quantity | nodes | median ratio | p10 | p90 | within 25% | correlation |")
    print("|---|---|---|---|---|---|---|")
    for q in ("grew", "leaf_cur", "leaf_drawn", "wood", "section"):
        pairs = [(c["cycle"][q], c["expected"][q]) for c in d["cells"] if c["cycle"][q] > 0 or c["expected"][q] > 0]
        ratios = [e / cy for cy, e in pairs if cy > 0]
        within = sum(1 for r in ratios if 0.75 <= r <= 1.25) / len(pairs) if pairs else float("nan")
        print(f"| {q} | {len(pairs)} | {pct(ratios, .5):.3f} | {pct(ratios, .1):.3f} | {pct(ratios, .9):.3f} | "
              f"{within:.3f} | {corr([p[0] for p in pairs], [p[1] for p in pairs]):.3f} |")
    print()

    lights = sorted(c["light"] for c in d["cells"])
    cuts = [pct(lights, p) for p in (0.2, 0.4, 0.6, 0.8)]
    print("By the node's light at the tree's age (equal-count fifths), totals:\n")
    print("| light fifth | light range | nodes | cycle leaf | law leaf | ratio | cycle new shoots | law new shoots | ratio |")
    print("|---|---|---|---|---|---|---|---|---|")
    for b in range(5):
        lo = cuts[b - 1] if b else 0.0
        hi = cuts[b] if b < 4 else 1.0
        sel = [c for c in d["cells"] if (c["light"] >= lo if b else True) and (c["light"] < hi if b < 4 else True)]
        cl = sum(c["cycle"]["leaf_cur"] for c in sel)
        ll = sum(c["expected"]["leaf_cur"] for c in sel)
        cg = sum(c["cycle"]["grew"] for c in sel)
        lg = sum(c["expected"]["grew"] for c in sel)
        f = lambda a, b: f"{a / b:.3f}" if b else "-"
        print(f"| {b + 1} | {lo:.2f} to {hi:.2f} | {len(sel)} | {cl:.3f} | {ll:.3f} | {f(ll, cl)} | {cg:.0f} | {lg:.0f} | {f(lg, cg)} |")
    print()


def main():
    for path in sys.argv[1:]:
        report(read(path))


if __name__ == "__main__":
    main()
