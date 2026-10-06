"""fn-211 task 1: tables from raw/profile.jsonl (profile-probe.rs's output).
   python3 profile-tables.py raw/profile.jsonl [summary|passes|years|serial]"""
import json, sys, collections

GROUPS = [
    ("setup", ["setup.windows"]),
    ("balance", ["balance"]),
    ("allot", ["allot"]),
    ("grow", ["grow.homes", "grow.units", "grow.merge"]),
    ("wake", ["wake"]),
    ("sketch", ["sketch"]),
    ("relay", ["relay.settle", "relay.place", "relay.levers", "relay.sagplace", "relay.write"]),
    ("shade", ["shade.field", "shade.read"]),
    ("settle", ["settle.mark", "settle.assign", "settle.attachments", "settle.shed",
                "settle.disused", "settle.scale", "settle.thicken", "settle.leaf_mean"]),
    ("lay", ["lay.place", "lay.levers", "lay.sagplace"]),
]
ORDER = ["palm", "beech", "oak", "spruce"]

def load(path):
    runs = {}
    for line in open(path):
        d = json.loads(line)
        cores = "1" if d["threads"] == 1 else "all"
        runs[(d["species"], d["age"], d["seed"], cores, d["rep"])] = d
    return runs

def per_pass(d):
    agg = collections.defaultdict(lambda: [0, 0])
    for c, n, w, p in d["spans"]:
        agg[n][0] += w; agg[n][1] += p
    return agg

def ms(ns): return ns / 1e6

def fmt(v):
    if v >= 100: return f"{v:,.0f}"
    if v >= 10: return f"{v:.1f}"
    return f"{v:.2f}"

def keys(runs):
    return sorted({k[:3] for k in runs}, key=lambda k: (k[1], ORDER.index(k[0]), k[2]))

def summary(runs, rep=1):
    names = [g for g, _ in GROUPS]
    print("| Tree | Cores | Load | " + " | ".join(names) + " | other | **Total** | serial |")
    print("|---|--:|--:|" + "--:|" * (len(names) + 3))
    for sp, age, seed in keys(runs):
        for cores in ("all", "1"):
            d = runs.get((sp, age, seed, cores, rep))
            if not d: continue
            agg = per_pass(d)
            cells, acc, par = [], 0, 0
            for g, members in GROUPS:
                w = sum(agg[m][0] for m in members if m in agg)
                par += sum(agg[m][1] for m in members if m in agg)
                acc += w
                cells.append(fmt(ms(w)) if w else "–")
            other = d["total_ns"] - acc
            serial = 1 - par / d["total_ns"]
            print(f"| {sp} {age} s{seed} | {cores} | {d['load']:.2f} | " + " | ".join(cells)
                  + f" | {fmt(ms(other))} | **{fmt(ms(d['total_ns']))}** | {serial:.0%} |")

def passes(runs, sp, age, seed, rep=1):
    a = runs[(sp, age, seed, "all", rep)]; o = runs[(sp, age, seed, "1", rep)]
    A, O = per_pass(a), per_pass(o)
    print(f"| Pass | 1 core ms | all cores ms | speed-up | serial share (1 core) | serial share (all) |")
    print("|---|--:|--:|--:|--:|--:|")
    for g, members in GROUPS:
        for m in members:
            if m not in O: continue
            w1, p1 = O[m]; wa, pa = A[m]
            if w1 < 0.05e6 and wa < 0.05e6: continue
            print(f"| {m} | {fmt(ms(w1))} | {fmt(ms(wa))} | {w1/max(wa,1):.2f}x | {1-p1/w1:.0%} | {1-pa/max(wa,1):.0%} |")

def years(runs, sp, age, seed, cores="all", rep=1, step=10):
    d = runs[(sp, age, seed, cores, rep)]
    by = collections.defaultdict(lambda: collections.defaultdict(int))
    for c, n, w, p in d["spans"]:
        g = next(g for g, ms_ in GROUPS if n in ms_)
        by[c][g] += w
    names = [g for g, _ in GROUPS if g not in ("setup", "settle", "lay")]
    names = [g for g in names if any(by[c][g] for c in by)]
    print("| Year | " + " | ".join(names) + " | **Year** |")
    print("|--:|" + "--:|" * (len(names) + 1))
    marks = sorted({1, *range(step, age + 1, step)})
    for c in marks:
        row = [by[c][g] for g in names]
        print(f"| {c} | " + " | ".join(fmt(ms(v)) if v else "–" for v in row) + f" | **{fmt(ms(sum(row)))}** |")
    # Whole-run sums
    yearly = sum(by[c][g] for c in range(1, age + 1) for g in names)
    print(f"\nSum over the {age} years: {fmt(ms(yearly))} ms; settle and lay: "
          f"{fmt(ms(sum(by[age + 1].values())))} ms; setup {fmt(ms(sum(by[0].values())))} ms.")

if __name__ == "__main__":
    runs = load(sys.argv[1])
    what = sys.argv[2] if len(sys.argv) > 2 else "summary"
    if what == "summary": summary(runs)
    elif what == "passes": passes(runs, sys.argv[3], int(sys.argv[4]), int(sys.argv[5]))
    elif what == "years": years(runs, sys.argv[3], int(sys.argv[4]), int(sys.argv[5]),
                                sys.argv[6] if len(sys.argv) > 6 else "all")
