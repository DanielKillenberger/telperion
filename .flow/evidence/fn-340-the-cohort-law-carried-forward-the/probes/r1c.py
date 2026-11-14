"""fn-340 R1c: read the ledger lines (`LEDGER label cycle pa` then nine lines,
probes/coupled2.patch) and find where the cycle's structural population and
the probe's part, by physiological age and line.

Run: python3 -I r1c.py <trace log> [cohort ages, comma list; the oak's 13,14]
"""
import sys
from collections import defaultdict

LINES = ["living", "sprouted", "woken", "grown on", "relayed", "lifespan end",
         "stopped in growth", "shed by balance", "retired"]
NAMES = {2: "trunk", 3: "fork", 4: "leader", 6: "limb", 7: "bough", 8: "sprig", 9: "branch",
         12: "shoot", 13: "twig", 14: "short"}


def load(path):
    led = defaultdict(lambda: defaultdict(lambda: [0.0] * 13))
    for line in open(path):
        w = line.split()
        if w and w[0] == "LEDGER":
            led[w[1]][(int(w[2]), int(w[3]))] = [float(x) for x in w[4:]]
    return led


def main():
    led = load(sys.argv[1])
    skip = {int(x) for x in (sys.argv[2] if len(sys.argv) > 2 else "13,14").split(",")}
    b, p = led["base"], led["probe"]
    pas = sorted({k[1] for k in b} - skip)
    # The first closed year of a run held the last run's final year until
    # the ledger was cleared per run; it is left out either way.
    years = sorted({k[0] for k in b} - {1})
    # Cumulative counts per (pa, line); the first year a line's cumulative
    # ratio leaves 0.9..1.1 for five years running, with at least 30 counted.
    first = []
    for pa in pas:
        for li in range(1, 9):
            cb = cp = 0.0
            run = 0
            for y in years:
                cb += b[(y, pa)][li]
                cp += p[(y, pa)][li]
                bad = max(cb, cp) >= 30 and (cb == 0 or abs(cp / cb - 1) > 0.1)
                run = run + 1 if bad else 0
                if run == 5:
                    first.append((y - 4, NAMES.get(pa, pa), LINES[li], cb, cp))
                    break
        # Living apexes per year, same rule.
        run = 0
        for y in years:
            lb, lp = b[(y, pa)][0], p[(y, pa)][0]
            bad = max(lb, lp) >= 30 and (lb == 0 or abs(lp / lb - 1) > 0.1)
            run = run + 1 if bad else 0
            if run == 5:
                first.append((y - 4, NAMES.get(pa, pa), "living (per year)", lb, lp))
                break
    first.sort()
    print("First parting per age and line (cumulative to that year, cycle / probe):\n")
    print("| year | age | line | cycle | probe |")
    print("|---|---|---|---|---|")
    for y, pa, li, cb, cp in first[:25]:
        print(f"| {y} | {pa} | {li} | {cb:.0f} | {cp:.0f} |")
    print()
    for pa in pas:
        print(f"## {NAMES.get(pa, pa)}: by decade, sums over the decade, cycle / probe\n")
        print("| years | " + " | ".join(LINES) + " |")
        print("|---|" + "---|" * len(LINES))
        for d0 in range(0, 120, 10):
            ys = [y for y in years if d0 < y <= d0 + 10]
            cb = [sum(b[(y, pa)][li] for y in ys) for li in range(9)]
            cp = [sum(p[(y, pa)][li] for y in ys) for li in range(9)]
            if not any(cb) and not any(cp):
                continue
            print(f"| {d0 + 1}-{d0 + 10} | " + " | ".join(f"{x:.0f}/{y:.0f}" for x, y in zip(cb, cp)) + " |")
        print()
        if len(b[(years[0], pa)]) < 13:
            continue
        print(f"Tip inputs over the living apexes ({NAMES.get(pa, pa)}), cycle / probe: mean light, mean size (presence-weighted), vigour per presence over the year's base (from TRACEY col 4 not read: raw vigour per presence).\n")
        print("| years | light | size | vigour per presence | deaths in growth per living |")
        print("|---|---|---|---|---|")
        for d0 in range(0, 120, 10):
            ys = [y for y in years if d0 < y <= d0 + 10]
            def m(led_, i, j):
                num = sum(led_[(y, pa)][i] for y in ys)
                den = sum(led_[(y, pa)][j] for y in ys)
                return num / den if den else float("nan")
            if not sum(b[(y, pa)][9] for y in ys):
                continue
            print(f"| {d0 + 1}-{d0 + 10} | {m(b, 10, 9):.3f} / {m(p, 10, 9):.3f} | {m(b, 11, 9):.3f} / {m(p, 11, 9):.3f} | {m(b, 12, 9):.3g} / {m(p, 12, 9):.3g} | {m(b, 6, 0):.4f} / {m(p, 6, 0):.4f} |")
        print()


if __name__ == "__main__":
    main()
