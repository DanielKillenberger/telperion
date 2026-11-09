"""fn-335 R4, R5: the verdict rule of PROTOCOL.md on read.py's cell sums.

Usage: python3 -I verdict.py <dir> > verdict.txt
Reads <dir>/<preset>.json for the four presets. For each preset, standing
quantity and predictor, each test tree's fine laterals are given their fit
cell's mean and the tree's total compared with its own: the whole-tree error,
and the worst bearer-age group's (groups of at least 5% of the tree's fine
laterals), each the median over the six test trees. The one curve takes the
other three presets' fit shares, pooled by span bin and light bin over their
fine ages, times each test lateral's own potential. A test cell with no fit
cell is counted and named; it is given nothing.
"""
import json, sys
from collections import defaultdict

PRESETS = ["oregon-white-oak", "norway-spruce", "silver-birch", "european-beech"]
QUANT = ["leaf", "section", "wood"]
TEST = list(range(1007, 1013))
# Each predictor's cell: the key fields of (age, sb, lb, order) it reads.
PREDICTORS = {"span": (0, 1), "span+light": (0, 1, 2), "span+light+order": (0, 1, 2, 3)}

d = {p: json.load(open(f"{sys.argv[1]}/{p}.json")) for p in PRESETS}


def keyed(table):
    return [(tuple(int(x) for x in k.split(",")), v) for k, v in table.items()]


def fine_ages(p):
    sets = {tuple(v) for v in d[p]["fine"].values()}
    assert len(sets) == 1, f"{p}: fine ages differ by seed"
    return set(next(iter(sets)))


def median(v):
    s = sorted(v)
    n = len(s)
    return (s[(n - 1) // 2] + s[n // 2]) / 2


def errors(pred_actual):
    """Whole and worst-group error of one tree from {bearer: [pred, actual, n]}."""
    pred = sum(x[0] for x in pred_actual.values())
    act = sum(x[1] for x in pred_actual.values())
    n = sum(x[2] for x in pred_actual.values())
    whole = abs(pred - act) / act if act > 0 else float("inf")
    groups = [abs(x[0] - x[1]) / x[1] if x[1] > 0 else float("inf")
              for x in pred_actual.values() if x[2] >= 0.05 * n]
    return whole, max(groups) if groups else float("nan")


def read(p, means, shares=None):
    """Per quantity: the median whole and worst-group errors over the test
    trees, and the test laterals with no fit cell. `means(key)` gives a fit
    cell's means of (standing leaf, section, wood), or `shares(key)` its
    mean shares, multiplied by the test cell's potential."""
    fine = fine_ages(p)
    trees = defaultdict(lambda: [defaultdict(lambda: [0.0, 0.0, 0]) for _ in QUANT])
    unfit = 0
    for k, v in keyed(d[p]["test"]):
        seed, age, sb, lb, order, bearer = k
        if age not in fine:
            continue
        m = (means or shares)((age, sb, lb, order))
        if m is None:
            unfit += v[0]
        for q in range(3):
            g = trees[seed][q][bearer]
            if m is not None:
                g[0] += m[q] * (v[0] if means else v[4 + q])
            g[1] += v[1 + q]
            g[2] += v[0]
    out = []
    for q in range(3):
        per = [errors(trees[s][q]) for s in TEST if s in trees]
        out.append((median([x[0] for x in per]), median([x[1] for x in per]), len(per)))
    return out, unfit


def fit_means(p, fields, value_at):
    """A fit cell's means over the fields `fields` of (age, sb, lb, order)."""
    sums = defaultdict(lambda: [0, 0.0, 0.0, 0.0])
    fine = fine_ages(p)
    for k, v in keyed(d[p]["fit"]):
        if k[0] not in fine:
            continue
        c = sums[tuple(k[i] for i in fields)]
        c[0] += v[0]
        for q in range(3):
            c[1 + q] += v[value_at + q]
    return lambda key: (None if tuple(key[i] for i in fields) not in sums
                        else [x / sums[tuple(key[i] for i in fields)][0]
                              for x in sums[tuple(key[i] for i in fields)][1:]])


def one_curve(p):
    sums = defaultdict(lambda: [0, 0.0, 0.0, 0.0])
    for o in PRESETS:
        if o == p:
            continue
        fine = fine_ages(o)
        for k, v in keyed(d[o]["fit"]):
            if k[0] not in fine:
                continue
            c = sums[(k[1], k[2])]
            c[0] += v[0]
            for q in range(3):
                c[1 + q] += v[5 + q]
    return lambda key: (None if (key[1], key[2]) not in sums
                        else [x / sums[(key[1], key[2])][0] for x in sums[(key[1], key[2])][1:]])


def level(rows):
    """go, near or no-go over presets' (whole, worst) errors on all quantities."""
    worst_whole = max(r[0] for pr in rows for r in pr)
    worst_group = max(r[1] for pr in rows for r in pr)
    if worst_whole <= 0.05 and worst_group <= 0.10:
        return "go"
    if worst_whole <= 0.10 and worst_group <= 0.20:
        return "near"
    return "no-go"


RANK = {"go": 0, "near": 1, "no-go": 2}
results = {}
for name, fields in list(PREDICTORS.items()) + [("one curve", None)]:
    rows = []
    print(f"== {name}")
    for p in PRESETS:
        # fit values: [n, grown share, sleaf, ssec, swood, sleaf share, ssec share, swood share]
        if fields is None:
            got, unfit = read(p, None, one_curve(p))
        else:
            got, unfit = read(p, fit_means(p, fields, 2), None)
        rows.append([(w, g) for w, g, _ in got])
        trees = min(t for _, _, t in got)
        cells = " ".join(f"{q} {w:.3f}/{g:.3f}" for q, (w, g, _) in zip(QUANT, got))
        print(f"{p}: whole/worst-group {cells}; test trees {trees}; unfit laterals {unfit}")
        if trees < 6:
            print(f"  UNRESOLVED: {trees} test trees")
    results[name] = level(rows)
    print(f"level: {results[name]}")
best = min((r for k, r in results.items() if k in PREDICTORS), key=RANK.get)
print("VERDICT (best of three predictors):", best)
print("span alone:", results["span"], "| one curve:", results["one curve"])
