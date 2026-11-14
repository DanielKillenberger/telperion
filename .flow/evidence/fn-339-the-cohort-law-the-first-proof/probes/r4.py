"""fn-339 R4: read `cohort_coupled` logs (probes/coupled.patch) and print the
tables RESULT.md quotes.

Run: python3 -I r4.py <log> [<log> ...]   (READ, TIME and PEAK lines)
"""
import statistics
import sys

F = ["height", "spread", "trunk", "limbs", "leaf_cur", "leaf_standing", "columns", "lai",
     "shoots", "structure_units", "cohort_units", "classes", "steps", "grown", "wshoots"]
R2 = {
    "norway-spruce": {"shoots": (691, 2372), "lai": (10.5, 10.5)},
    "european-beech": {"lai": (5.6, 9.5)},
}


def med(xs):
    return statistics.median(xs) if xs else float("nan")


def dist(x, lo, hi):
    return 1.0 if lo <= x <= hi else (x / hi if x > hi else x / lo)


def main():
    rows, times = [], []
    preset = None
    for path in sys.argv[1:]:
        name = path.split("r4-read-")[-1].split(".log")[0]
        for line in open(path):
            w = line.split()
            if not w:
                continue
            if w[0] == "READ":
                rows.append(dict(log=name, mode=w[1], seed=int(w[2]), age=int(w[3]),
                                 **{k: float(v) for k, v in zip(F, w[4:])}))
            elif "TIME " in line:
                t = line[line.index("TIME "):].split()
                name = path.split("r4-time-")[-1].split(".log")[0]
                times.append(dict(log=name, seed=int(t[1]), batch=int(t[2]), base=float(t[3]), probe=float(t[4])))
    groups = {}
    for r in rows:
        groups.setdefault(r["log"], []).append(r)
    for log, rs in groups.items():
        base = {r["seed"]: r for r in rs if r["mode"] == "base"}
        probe = {r["seed"]: r for r in rs if r["mode"] == "probe"}
        seeds = sorted(set(base) & set(probe))
        age = rs[0]["age"]
        print(f"## {log}: age {age}, seeds {seeds}\n")
        print("| measure | cycle (median, range) | probe (median, range) | probe over cycle, median (range) |")
        print("|---|---|---|---|")
        for k in F:
            b = [base[s][k] for s in seeds]
            p = [probe[s][k] for s in seeds]
            ratio = [probe[s][k] / base[s][k] for s in seeds if base[s][k]]
            fmt = lambda xs: f"{med(xs):.4g} ({min(xs):.4g} to {max(xs):.4g})"
            rr = fmt(ratio) if ratio else "-"
            print(f"| {k} | {fmt(b)} | {fmt(p)} | {rr} |")
        print()
        sp = log.rsplit("40", 1)[0] if log.endswith("40") else log
        key = {"spruce40": "norway-spruce", "beech40": "european-beech"}.get(log)
        if key:
            print("Beside R2's checked ranges (median over seeds; distance as a multiple of the nearer end, 1 inside):\n")
            print("| measure | range | cycle | probe | nearer |")
            print("|---|---|---|---|---|")
            for m, (lo, hi) in R2[key].items():
                field = "wshoots" if m == "shoots" else "lai"
                dc = dist(med([base[s][field] for s in seeds]), lo, hi)
                dp = dist(med([probe[s][field] for s in seeds]), lo, hi)
                import math
                near = "same" if abs(abs(math.log(dc)) - abs(math.log(dp))) < 1e-3 else ("probe" if abs(math.log(dp)) < abs(math.log(dc)) else "cycle")
                print(f"| {m} | {lo} to {hi} | {dc:.3g} | {dp:.3g} | {near} |")
            print()
    for log in sorted({t["log"] for t in times}):
        ts = [t for t in times if t["log"] == log]
        if ts:
            by = {}
            for t in ts:
                by.setdefault(t["seed"], []).append(t)
            ratios = [med([t["base"] for t in v]) / med([t["probe"] for t in v]) for v in by.values()]
            print(f"{log}: one-thread seconds, {len(ts)} batches over {len(by)} seeds: cycle median "
                  f"{med([t['base'] for t in ts]):.3f} s, probe {med([t['probe'] for t in ts]):.3f} s; "
                  f"cycle over probe per seed median {med(ratios):.2f} (range {min(ratios):.2f} to {max(ratios):.2f}); "
                  f"per batch {min(t['base'] / t['probe'] for t in ts):.2f} to {max(t['base'] / t['probe'] for t in ts):.2f}.\n")


if __name__ == "__main__":
    main()
