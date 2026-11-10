"""fn-340 R1: read `cohort_trace` logs (probes/coupled2.patch) and print the
year-by-year tables R1-R2-NOTE.md quotes.

Run: python3 -I r1.py <trace log> [...]

TRACEY row: cycle, structural apexes (count), structural presence,
cohort-age living (cycle: apexes' presence; probe: the cohorts' expected
living), base vigour, structural buds' vigour, cohort-age buds' vigour,
sum p (size - 1)^2 over structural apexes, cohorts' leaf deposited, all
leaf deposited, structural laterals shed by the balance, cohort-age
laterals shed by the balance.
TRACES: age, height, spread, trunk diameter, structure's current-year
leaf, cohorts' current-year leaf (settled tree).
"""
import sys

COLS = ["cycle", "s_n", "s_p", "c_live", "base", "s_vig", "c_vig", "s_var", "c_leaf", "leaf",
        "s_shed", "c_shed", "c_q", "c_c"]


def load(paths):
    y, s = {}, {}
    for path in paths:
        for line in open(path):
            w = line.split()
            if not w:
                continue
            if w[0] == "TRACEY":
                mode, seed = w[1], int(w[2])
                row = dict(zip(COLS, map(float, w[3:])))
                y.setdefault((mode, seed), {})[int(row["cycle"])] = row
            elif w[0] == "TRACES":
                mode, seed, age = w[1], int(w[2]), int(w[3])
                v = [float(x) for x in w[4:]]
                s.setdefault((mode, seed), {})[age] = dict(height=v[0], spread=v[1], trunk=v[2],
                                                           leaf=v[3] + v[4], leaf_s=v[3], leaf_c=v[4])
    return y, s


def r(a, b):
    return a / b if b else float("nan")


def first_part(series, tol=0.1, run=3):
    """The first year from which the ratio stays outside 1 +- tol for `run` reads."""
    keys = sorted(series)
    for i, k in enumerate(keys):
        window = keys[i:i + run]
        if len(window) == run and all(abs(series[j] - 1) > tol for j in window):
            return k
    return None


def main():
    y, s = load(sys.argv[1:])
    seeds = sorted({k[1] for k in y})
    modes = sorted({k[0] for k in y})
    for seed in seeds:
        base = y[("base", seed)]
        sb = s[("base", seed)]
        print(f"## seed {seed}: year by year, every 5 years, probe over cycle\n")
        print("| year | structural apexes | structural presence | struct. vigour share (cycle / probe) | size spread (cycle / probe) | leaf deposited | balance-shed structural laterals (cycle / probe, 5 y) | trunk | current-year leaf | height | spread |")
        print("|---|---|---|---|---|---|---|---|---|---|---|")
        p = y[("probe", seed)]
        sp = s[("probe", seed)]
        for yr in range(5, 121, 5):
            if yr not in base or yr not in p:
                b2 = sb.get(yr)
                p2 = sp.get(yr)
                if b2 and p2:
                    print(f"| {yr} | - | - | - | - | - | - | {r(p2['trunk'], b2['trunk']):.3f} | {r(p2['leaf'], b2['leaf']):.3f} | {r(p2['height'], b2['height']):.3f} | {r(p2['spread'], b2['spread']):.3f} |")
                continue
            b, q = base[yr], p[yr]
            share = lambda row: r(row["s_vig"], row["base"])
            var = lambda row: r(row["s_var"], row["s_p"])
            shed = lambda m: sum(m[k]["s_shed"] for k in range(yr - 4, yr + 1) if k in m)
            b2, p2 = sb.get(yr), sp.get(yr)
            tail = "- | - | - | -"
            if b2 and p2:
                tail = f"{r(p2['trunk'], b2['trunk']):.3f} | {r(p2['leaf'], b2['leaf']):.3f} | {r(p2['height'], b2['height']):.3f} | {r(p2['spread'], b2['spread']):.3f}"
            print(f"| {yr} | {r(q['s_n'], b['s_n']):.3f} | {r(q['s_p'], b['s_p']):.3f} | {share(b):.3f} / {share(q):.3f} | {var(b):.3f} / {var(q):.3f} | {r(q['leaf'], b['leaf']):.3f} | {shed(base):.0f} / {shed(p):.0f} | {tail} |")
        print()
        print(f"First year each ratio leaves 0.9 to 1.1 and stays out for three reads (seed {seed}):\n")
        for name, series in [
            ("structural apexes", {k: r(p[k]["s_n"], base[k]["s_n"]) for k in base if k in p and k >= 5}),
            ("structural presence", {k: r(p[k]["s_p"], base[k]["s_p"]) for k in base if k in p and k >= 5}),
            ("leaf deposited", {k: r(p[k]["leaf"], base[k]["leaf"]) for k in base if k in p and k >= 5}),
            ("structural vigour, sum", {k: r(p[k]["s_vig"], base[k]["s_vig"]) for k in base if k in p and k >= 5}),
            ("trunk (5-yearly)", {k: r(sp[k]["trunk"], sb[k]["trunk"]) for k in sb if k in sp and sb[k]["trunk"] > 0 and sp[k]["trunk"] == sp[k]["trunk"]}),
            ("current-year leaf (5-yearly)", {k: r(sp[k]["leaf"], sb[k]["leaf"]) for k in sb if k in sp and sp[k]["leaf"] == sp[k]["leaf"]}),
            ("height (5-yearly)", {k: r(sp[k]["height"], sb[k]["height"]) for k in sb if k in sp and sp[k]["height"] == sp[k]["height"]}),
        ]:
            print(f"- {name}: {first_part(series)}")
        print()
        print(f"Each switch alone at 40, 80 and 120 (seed {seed}), over the cycle:\n")
        print("| mode | trunk 40 / 80 / 120 | current-year leaf 40 / 80 / 120 | height 120 | spread 120 | structural apexes 119 | leaf deposited 119 |")
        print("|---|---|---|---|---|---|---|")
        for m in modes:
            if m == "base" or (m, seed) not in s:
                continue
            sm, ym = s[(m, seed)], y[(m, seed)]
            f = lambda k, a: r(sm[a][k], sb[a][k]) if a in sm else float("nan")
            print(f"| {m} | {f('trunk', 40):.3f} / {f('trunk', 80):.3f} / {f('trunk', 120):.3f} | {f('leaf', 40):.3f} / {f('leaf', 80):.3f} / {f('leaf', 120):.3f} | {f('height', 120):.3f} | {f('spread', 120):.3f} | {r(ym[119]['s_n'], base[119]['s_n']):.3f} | {r(ym[119]['leaf'], base[119]['leaf']):.3f} |")
        print()


if __name__ == "__main__":
    main()
