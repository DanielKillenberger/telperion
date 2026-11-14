"""fn-340 R4: read the `cohort_coupled` logs (probes/coupled2.patch) and print
the tables RESULT.md quotes, with the cycle's own spread across seeds, the
ratio of medians R5 reads, and the timing, peak and phase lines.

Run: python3 -I r4.py <log> [<log> ...]   (READ, TIME, PEAK and PHASES lines)
"""
import statistics
import sys

F = ["height", "spread", "trunk", "limbs", "leaf_cur", "leaf_standing", "columns", "lai",
     "shoots", "structure_units", "cohort_units", "classes", "steps", "grown", "wshoots"]
SHOW = ["height", "spread", "trunk", "leaf_cur", "lai", "wshoots", "limbs", "structure_units", "classes", "grown"]
R2 = {"norway-spruce": {"wshoots": (691, 2372), "lai": (10.5, 10.5)}, "european-beech": {"lai": (5.6, 9.5)}}


def med(xs):
    return statistics.median(xs) if xs else float("nan")


def dist(x, lo, hi):
    return 1.0 if lo <= x <= hi else (x / hi if x > hi else x / lo)


def main():
    rows, times, other = [], [], []
    for path in sys.argv[1:]:
        tag = path.rsplit("/", 1)[-1].replace(".log", "")
        for line in open(path):
            w = line.split()
            if not w:
                continue
            if w[0] == "READ":
                rows.append(dict(log=tag, mode=w[1], seed=int(w[2]), age=int(w[3]),
                                 **{k: float(v) for k, v in zip(F, w[4:])}))
            elif "TIME " in line:
                t = line[line.index("TIME "):].split()
                times.append(dict(log=tag, seed=int(t[1]), base=float(t[3]), probe=float(t[4])))
            elif w[0] in ("PEAK", "PHASES"):
                other.append(f"- {tag}: {line.strip()}")
    groups = {}
    for r in rows:
        groups.setdefault(r["log"], []).append(r)
    for log, rs in sorted(groups.items()):
        base = {r["seed"]: r for r in rs if r["mode"] == "base"}
        probe = {r["seed"]: r for r in rs if r["mode"] == "probe"}
        seeds = sorted(set(base) & set(probe))
        print(f"## {log}: age {rs[0]['age']}, seeds {seeds[0]} to {seeds[-1]} ({len(seeds)})\n")
        print("| measure | cycle median (range) | probe median (range) | probe over cycle per seed, median (range) | ratio of medians | cycle's own spread, largest over smallest |")
        print("|---|---|---|---|---|---|")
        for k in SHOW:
            b = [base[s][k] for s in seeds]
            p = [probe[s][k] for s in seeds]
            ratio = [probe[s][k] / base[s][k] for s in seeds if base[s][k]]
            fmt = lambda xs: f"{med(xs):.4g} ({min(xs):.4g} to {max(xs):.4g})"
            rr = fmt(ratio) if ratio else "-"
            rom = f"{med(p) / med(b):.3f}" if med(b) else "-"
            spread = f"{max(b) / min(b):.2f}" if min(b) > 0 else "-"
            print(f"| {k} | {fmt(b)} | {fmt(p)} | {rr} | {rom} | {spread} |")
        print()
        key = next((n for n in R2 if log.startswith(f"r4-read-{n}")), None)
        if key:
            print("Beside R2's checked ranges (median over seeds; distance as a multiple of the nearer end, 1 inside):\n")
            print("| measure | range | cycle | probe |")
            print("|---|---|---|---|")
            for m, (lo, hi) in R2[key].items():
                dc = dist(med([base[s][m] for s in seeds]), lo, hi)
                dp = dist(med([probe[s][m] for s in seeds]), lo, hi)
                print(f"| {m} | {lo} to {hi} | {dc:.3g} | {dp:.3g} |")
            print()
    for log in sorted({t["log"] for t in times}):
        ts = [t for t in times if t["log"] == log]
        by = {}
        for t in ts:
            by.setdefault(t["seed"], []).append(t)
        ratios = [med([t["base"] for t in v]) / med([t["probe"] for t in v]) for v in by.values()]
        print(f"{log}: one-thread seconds, {len(ts)} batches over {len(by)} seeds: cycle median "
              f"{med([t['base'] for t in ts]):.3f} s, probe {med([t['probe'] for t in ts]):.3f} s; "
              f"cycle over probe per seed median {med(ratios):.2f} (range {min(ratios):.2f} to {max(ratios):.2f}).\n")
    for line in other:
        print(line)


if __name__ == "__main__":
    main()
