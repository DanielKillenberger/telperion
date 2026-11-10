"""fn-339 R3: the read that counts, under PROTOCOL.md (frozen 10 Nov 2026).

Reads `cohort_replay` logs (probes/replay.patch), one per species, each
holding seeds 1001 to 1012, and prints R3-RESULT.md's tables.

Run: python3 -I r3.py <log> [<log> ...]
"""
import statistics
import sys

Q = ["grew", "living", "standing", "leaf_cur", "leaf_drawn", "wood", "section", "phyto"]
SIDE = Q + ["join", "roots", "balance"]
LAWS = ["root-1", "subtree-1", "subtree-size", "root-size", "subtree-1-drawn"]
READ = ["grew", "living", "standing", "leaf_cur", "leaf_drawn", "wood", "section", "join"]
EDGES = [0.0, 0.2, 0.4, 0.6, 0.8, 1.0000001]
CELL_AREA = 0.25
# R2's checked ranges (R2-REAL-TREES.md): (low, high, what).
R2 = {
    "norway-spruce": {"shoots": (691, 2372, "new shoots a tree a year, ~36 y, Bellan 2019"),
                      "lai": (10.5, 10.5, "projected LAI, 28 y plantation, Bolstad and Gower 1990")},
    "european-beech": {"lai": (5.6, 9.5, "LAI 5.6 to 9.5, Leuschner 2006 (7.1, 7.4 Glatthorn 2017)")},
}


def trees(path):
    out, t = [], None
    for line in open(path):
        w = line.split()
        if not w:
            continue
        if w[0] == "HEAD":
            t = dict(preset=w[1], seed=int(w[2]), age=int(w[4]), apex={}, cells=[], trace={}, cohort={})
            out.append(t)
        elif t is None:
            continue
        elif w[0] == "COHORT":
            t["cohort"][w[1]] = line.split(" ", 2)[2].strip()
        elif w[0] == "APEX":
            t["apex"][int(w[1])] = [int(x) for x in w[2:6]]
        elif w[0] == "TRACE":
            t["trace"][int(w[1])] = [float(x) for x in w[2:]]
        elif w[0] == "CELL":
            v = [float(x) for x in w[1:]]
            cell = dict(units=v[3], light=v[4])
            at = 5
            for side in ["cycle"] + LAWS:
                cell[side] = dict(zip(SIDE, v[at:at + len(SIDE)]))
                at += len(SIDE)
            cell["at_one"] = dict(leaf_cur=v[at], wood=v[at + 1])
            t["cells"].append(cell)
        elif w[0] == "TREE":
            v = [float(x) for x in w[1:7]]
            t["structure"] = dict(grew=v[0], leaf_cur=v[1], leaf_drawn=v[2])
            t["whole"] = dict(grew=v[3], leaf_cur=v[4], leaf_drawn=v[5])
            t["columns"] = (int(w[7]), int(w[8]))
            t["check"] = (int(w[9]), float(w[10]))
    return out


def total(cells, side, q):
    return sum(c[side][q] for c in cells)


def ratio(a, b):
    return a / b if b else float("nan")


def med(xs):
    xs = [x for x in xs if x == x]
    return statistics.median(xs) if xs else float("nan")


def binned(t, b):
    return [c for c in t["cells"] if EDGES[b] <= c["light"] < EDGES[b + 1]]


def sort(per_tree, bins):
    ok = 0.9 <= per_tree <= 1.1 and all(0.8 <= r <= 1.25 for r in bins)
    return "agrees" if ok else "differs"


def report(ts):
    p = ts[0]["preset"]
    seeds = [t["seed"] for t in ts]
    print(f"## {p}, age {ts[0]['age']}, seeds {seeds[0]} to {seeds[-1]} ({len(ts)} trees)\n")
    print("Cohort sets: " + "; ".join(f"{k} m {v}" for k, v in ts[0]["cohort"].items()))
    checks = [t["check"] for t in ts]
    print(f"Lattice check: {sum(c[0] for c in checks)} apex reads, largest difference {max(c[1] for c in checks):.1e}.\n")

    print("Shares of the cycle's units inside the cohorts (median over seeds, min to max):\n")
    print("| tree age | at 0.25 m | at 0.5 m | at 1.0 m |")
    print("|---|---|---|---|")
    for c in (10, 20, 30, 40):
        cols = []
        for k in (1, 2, 3):
            xs = [t["apex"][c][k] / t["apex"][c][0] for t in ts]
            cols.append(f"{med(xs):.3f} ({min(xs):.3f} to {max(xs):.3f})")
        print(f"| {c} | " + " | ".join(cols) + " |")
    print()

    units = [sum(c["units"] for c in t["cells"]) for t in ts]
    big = [b for b in range(5) if med([sum(c["units"] for c in binned(t, b)) / u for t, u in zip(ts, units)]) >= 0.05]
    print(f"Light bins holding at least 5% of bearing units (median over seeds): {[f'{EDGES[b]:.1f}-{min(EDGES[b+1],1):.1f}' for b in big]}.\n")
    print("Law over cycle, per tree: median over seeds (min to max); then each counted light bin's median; sorted by PROTOCOL.md's rule.\n")
    print("| quantity | law reading | per tree | light bins | sorted |")
    print("|---|---|---|---|---|")
    for q in READ:
        for law in LAWS:
            per = [ratio(total(t["cells"], law, q), total(t["cells"], "cycle", q)) for t in ts]
            bins = [med([ratio(total(binned(t, b), law, q), total(binned(t, b), "cycle", q)) for t in ts]) for b in big]
            print(f"| {q} | {law} | {med(per):.3f} ({min(per):.3f} to {max(per):.3f}) | "
                  + ", ".join(f"{r:.2f}" for r in bins) + f" | {sort(med(per), bins)} |")
    print()

    bal = [total(t["cells"], "subtree-1", "balance") for t in ts]
    print(f"Roots the balance hazard alone sheds (law, subtree-1), per tree: median {med(bal):.3g}, most {max(bal):.3g}.\n")

    print("Size trace, per cohort age, summed over seeds: mean size of the last cycle's units weighted by one, by the survival's presence, by the pencil's presence, and by the settled scale (which holds the size):\n")
    print("| age | units | by one | by survival | by pencil | by settled scale |")
    print("|---|---|---|---|---|---|")
    for pa in sorted(ts[0]["trace"]):
        s = [sum(t["trace"].get(pa, [0] * 9)[k] for t in ts) for k in range(8)]
        print(f"| {pa} | {s[0]:.0f} | {s[1] / s[0]:.3f} | {s[3] / s[2]:.3f} | {s[5] / s[4]:.3f} | {s[7] / s[6]:.3f} |")
    print()

    print("Per lattice node (subtree-1 against the cycle; median over seeds of each tree's figure):\n")
    print("| quantity | median ratio | p10 | p90 | share within 25% |")
    print("|---|---|---|---|---|")
    for q in ("grew", "leaf_cur", "wood", "section"):
        rows = []
        for t in ts:
            pairs = [(c["cycle"][q], c["subtree-1"][q]) for c in t["cells"] if c["cycle"][q] > 0 or c["subtree-1"][q] > 0]
            rs = sorted(e / cy for cy, e in pairs if cy > 0)
            within = sum(1 for r in rs if 0.75 <= r <= 1.25) / len(pairs) if pairs else float("nan")
            k = lambda f: rs[int(f * (len(rs) - 1))] if rs else float("nan")
            rows.append((k(0.5), k(0.1), k(0.9), within))
        print(f"| {q} | " + " | ".join(f"{med([r[i] for r in rows]):.3f}" for i in range(4)) + " |")
    print()

    print("Whole tree beside R2 (median over seeds, min to max). Law = the structure's own plus the law's cohorts (subtree-1). The index is over the tree's own leaf-bearing 0.5 m columns: a lone tree's, not a stand's.\n")
    rows = {"new shoots, weighted": ([], []), "new shoots, unweighted (cycle)": ([], []),
            "current-year leaf, m2": ([], []), "standing leaf, m2": ([], []), "leaf area index": ([], [])}
    for t in ts:
        s, w = t["structure"], t["whole"]
        law = lambda q: s[q] + total(t["cells"], "subtree-1", q)
        rows["new shoots, weighted"][0].append(w["grew"])
        rows["new shoots, weighted"][1].append(law("grew"))
        rows["new shoots, unweighted (cycle)"][0].append(t["apex"][t["age"]][0])
        rows["current-year leaf, m2"][0].append(w["leaf_cur"])
        rows["current-year leaf, m2"][1].append(law("leaf_cur"))
        rows["standing leaf, m2"][0].append(w["leaf_drawn"])
        rows["standing leaf, m2"][1].append(law("leaf_drawn"))
        rows["leaf area index"][0].append(w["leaf_drawn"] / (t["columns"][0] * CELL_AREA))
        rows["leaf area index"][1].append(law("leaf_drawn") / (t["columns"][1] * CELL_AREA))
    print("| measure | cycle | law | R2 checked | cycle against R2 | law against R2 | nearer |")
    print("|---|---|---|---|---|---|---|")
    keys = {"new shoots, weighted": "shoots", "leaf area index": "lai"}
    for name, (cy, lw) in rows.items():
        f = lambda xs: f"{med(xs):.4g} ({min(xs):.4g} to {max(xs):.4g})" if xs else "-"
        r2 = R2.get(p, {}).get(keys.get(name, ""), None)
        if r2:
            lo, hi, what = r2
            dist = lambda x: 1.0 if lo <= x <= hi else (x / hi if x > hi else x / lo)
            dc, dl = dist(med(cy)), dist(med(lw))
            far = lambda d: abs(__import__("math").log(d))
            nearer = "same" if abs(far(dc) - far(dl)) < 1e-3 else ("law" if far(dl) < far(dc) else "cycle")
            print(f"| {name} | {f(cy)} | {f(lw)} | {what} | {dc:.3g} | {dl:.3g} | {nearer} |")
        else:
            print(f"| {name} | {f(cy)} | {f(lw)} | no checked source | - | - | - |")
    print()


def main():
    for path in sys.argv[1:]:
        report(trees(path))


if __name__ == "__main__":
    main()
