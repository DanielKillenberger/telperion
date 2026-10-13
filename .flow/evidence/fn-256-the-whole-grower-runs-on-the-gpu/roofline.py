"""fn-256 task 7: ROOFLINE.md's tables from `the_roofline_is_measured`'s
files (raw/t7): stamps by cycle, phase and kernel (measured), each cycle's
census (measured), what each cycle visits from the CPU's reference
(computed, held to the census), and the bytes and operations each phase
moves (computed from the kernels' source-level words per record, an upper
estimate within about 2x). `python3 roofline.py raw/t7`."""
import csv
import collections
import statistics
import sys

DIR = sys.argv[1] if len(sys.argv) > 1 else "raw/t7"
CASES = [("telperion", "Telperion 82"), ("oregon-white-oak", "oak 40"), ("european-beech", "beech 40")]
PHASES = ["units and wake", "balance and allotment", "retirement", "layout", "light", "sizing", "compaction", "start"]
# Census words (census.rs).
AXES, UNITS, PHYTS, SLEEPERS, WOKEN, NEXT, BUILT = 1, 2, 3, 4, 5, 6, 8
AXES0, UNITS0, PHYTS0, WOKEN0, LIVE0, GREW = 11, 12, 13, 15, 16, 23
GRID = 128
DIRS = 9
# The light's kernels over the lattice, not its leaves or tips.
LATTICE = {"grid_main", "densities_main", "weights_main", "path_main", "sweep_main", "combine_main"}
# Assumed, not measured: zones a job's unit lays, rounds a job uses.
ZONES, ROUNDS = 2, 1.5


def peaks():
    text = open(f"{DIR}/peaks.txt").read().split()
    return float(text[text.index("GB/s") - 1]), float(text[text.index("TFLOP/s") - 1]), float(text[text.index("Tops") - 1])


def load(id):
    stamps = collections.defaultdict(float)  # (cycle, phase) -> ns
    lattice = collections.defaultdict(float)  # cycle -> the light's lattice kernels' ns
    kernels = collections.defaultdict(lambda: [0.0, 0])
    durations = collections.defaultdict(list)
    for r in csv.DictReader(open(f"{DIR}/{id}-stamps.csv")):
        ns = float(r["ns"])
        stamps[(int(r["cycle"]), r["phase"])] += ns
        if r["kernel"] in LATTICE:
            lattice[int(r["cycle"])] += ns
        kernels[(r["phase"], r["kernel"])][0] += ns
        kernels[(r["phase"], r["kernel"])][1] += 1
        durations[r["phase"]].append(ns)
    census = {int(r["cycle"]): [int(r[f"w{k}"]) for k in range(256)] for r in csv.DictReader(open(f"{DIR}/{id}-census.csv"))}
    lines = open(f"{DIR}/{id}-visits.csv").read().splitlines()
    head = lines[0]
    visits = {int(r["cycle"]): {k: int(v) for k, v in r.items()} for r in csv.DictReader(lines[1:])}
    return stamps, kernels, durations, census, visits, head, lattice


def records(w, lit, kern):
    """One cycle's records per phase, and bytes and operations (computed)."""
    L, A, B, P = w[LIVE0], w[AXES], w[BUILT], w[PHYTS] - w[PHYTS0]
    new, S, woke = w[AXES] - w[AXES0], w[SLEEPERS], w[WOKEN] - w[WOKEN0]
    G, C, grew, U, Ph = w[GRID + 6], w[GRID + 3] * w[GRID + 4], w[GREW], w[UNITS], w[PHYTS]
    job = 2 * (165 + ROUNDS * (230 + ZONES * 34 + 140)) + ROUNDS * ZONES * 52 + 50
    out = {}
    out["units and wake"] = (
        f"{L:,} jobs, {P:,} phyt, {new:,} axes, {S:,} sleepers",
        L * job + P * 6 + new * 100 + S * 100 + woke * 150,
        L * 2 * (100 + ROUNDS * (3000 + ZONES * 500 + 1000)) + P * 365 + new * 1800 + S * 3000,
    )
    out["balance and allotment"] = (
        f"{B:,} built axes, {L:,} buds, {new:,} placed",
        B * 55 + A * 29 + L * 5 + new * 100,
        B * 160 + L * (600 + 75) + new * 100,
    )
    out["retirement"] = (f"{A:,} axes", A * 41 + grew * 3, A * 100)
    out["layout"] = (f"{P:,} phyt, {grew:,} grown axes, {new:,} new", P * 54 + grew * 72 + new * 300, P * 1200 + new * 4000)
    pts = P + w[NEXT]
    lw = pts * 12 + P * 69 + G * 4 + w[NEXT] * 62
    lo = pts * 150 + P * 4500 + G * 50 + w[NEXT] * 4200
    if lit:
        lw += DIRS * C * 14 + DIRS * G * 43 + G * (3 + 2 * DIRS)
        lo += DIRS * C * 1200 + DIRS * G * (1300 + 900 + 3700)
    else:
        lw += G
    out["light"] = (f"{G:,} nodes ({DIRS} dirs{'' if lit else ', unlit'}), {P:,} leaves, {w[NEXT]:,} tips", lw, lo)
    out["sizing"] = ("~18 points", kern * 5 * 150, kern * 5 * 1500)
    return out, (A, U, Ph)


def main():
    bw, flop, iops = peaks()
    print(f"peaks: {bw} GB/s, {flop} TFLOP/s, {iops} Tops")
    for id, name in CASES:
        stamps, kernels, durations, census, visits, head, lattice = load(id)
        lit = id == "oregon-white-oak"
        total = sum(stamps.values())
        print(f"\n## {name}: {total / 1e6:.1f} ms stamped; {head}")
        by_phase = collections.defaultdict(float)
        for (c, p), ns in stamps.items():
            by_phase[p] += ns
        disp = collections.Counter()
        for (p, k), (ns, n) in kernels.items():
            disp[p] += n
        acc = {p: [0, 0] for p in PHASES}
        desc = {}
        compactions = [c for (c, p) in stamps if p == "compaction"]
        for c, w in census.items():
            per_cycle = sum(1 for (cc, p) in stamps if cc == c)
            kern = sum(n for (p, k), (ns, n) in kernels.items()) / len(census)
            out, (A, U, Ph) = records(w, lit, kern)
            for p, (d, b, o) in out.items():
                acc[p][0] += 4 * b
                acc[p][1] += o
                desc[p] = d
            if c in compactions:
                acc["compaction"][0] += 16 * (19 * Ph + 144 * A) + 12 * 18 * A
                acc["compaction"][1] += 5 * (19 * Ph + 144 * A)
        print("| Phase | GPU ms | Share | Dispatches | Median µs | Records at the last cycle | GB moved | GB/s | of peak | Gops | Gops/s | of peak | Roof ms | Measured / roof |")
        print("|---|--:|--:|--:|--:|---|--:|--:|--:|--:|--:|--:|--:|--:|")
        last = census[max(census)]
        out_last, _ = records(last, lit, 0)
        for p in sorted(PHASES, key=lambda p: -by_phase[p]):
            ms = by_phase[p] / 1e6
            if ms == 0:
                continue
            med = statistics.median(durations[p]) / 1e3
            gb, gops = acc[p][0] / 1e9, acc[p][1] / 1e9
            rate = gb / (ms / 1e3) if ms else 0
            orate = gops / (ms / 1e3) if ms else 0
            peak_ops = flop * 1e3 if p == "layout" else iops * 1e3
            d = out_last.get(p, ("compactions after " + str(sorted(set(compactions))),))[0]
            roof = 1e3 * max(gb / bw, gops / peak_ops)
            ratio = f"{ms / roof:.0f}x" if roof else "-"
            print(f"| {p} | {ms:.1f} | {100 * ms / (total / 1e6):.1f}% | {disp[p]:,} | {med:.1f} | {d} | {gb:.2f} | {rate:.1f} | {100 * rate / bw:.1f}% | {gops:.1f} | {orate:.0f} | {100 * orate / peak_ops:.2f}% | {roof:.1f} | {ratio} |")
        print("\nTop kernels:")
        for (p, k), (ns, n) in sorted(kernels.items(), key=lambda x: -x[1][0])[:12]:
            print(f"| {p} | {k} | {ns / 1e6:.1f} | {n} | {ns / n / 1e3:.1f} |")
        small = sum(d for ds in durations.values() for d in ds if d < 20e3)
        n_small = sum(1 for ds in durations.values() for d in ds if d < 20e3)
        print(f"dispatches under 20 µs: {n_small} of {sum(disp.values())}, {small / 1e6:.1f} ms")
        unchanged(name, census, visits, stamps, by_phase)
        short(name, head, visits, stamps, lattice, by_phase, total)
        lights = {k: v for (p, k), v in kernels.items() if p == "light"}
        print(f"{name} light kernels: " + ", ".join(f"{k} {v[0] / 1e6:.1f} ms/{v[1]}" for k, v in sorted(lights.items(), key=lambda x: -x[1][0])))
        under = {p: (sum(1 for d in ds if d < 20e3), sum(d for d in ds if d < 20e3) / 1e6) for p, ds in durations.items()}
        print(f"{name} dispatches under 20 µs by phase: " + ", ".join(f"{p} {n}/{len(durations[p])} {ms:.1f} ms" for p, (n, ms) in under.items()))
        sums = collections.Counter()
        for w in census.values():
            sums["jobs"] += w[LIVE0]; sums["built"] += w[BUILT]; sums["held"] += w[AXES]; sums["laid"] += w[PHYTS] - w[PHYTS0]
            sums["new"] += w[AXES] - w[AXES0]; sums["grew"] += w[GREW]; sums["nodes"] += w[GRID + 6]; sums["tips"] += w[NEXT]
        print(f"{name} visits over the build: " + ", ".join(f"{k} {v:,}" for k, v in sums.items()))


def unchanged(name, census, visits, stamps, by_phase):
    """Per axis-visiting phase, the held axes whose subtree did not grow."""
    held = sum(v["held"] for v in visits.values())
    changed = sum(v["changed"] for v in visits.values())
    stopped = sum(v["stopped"] for v in visits.values())
    built = sum(w[BUILT] for w in census.values())
    lastc = max(visits)
    v = visits[lastc]
    print(f"\n{name} unchanged: held axis-visits {held:,}, subtree unchanged {100 * (held - changed) / held:.1f}%, apex stopped {100 * stopped / held:.1f}%; last cycle {100 * (v['held'] - v['changed']) / v['held']:.1f}% unchanged, {100 * v['stopped'] / v['held']:.1f}% stopped; census held {sum(w[AXES] for w in census.values()):,} built {built:,}")
    jobs = sum(w[LIVE0] for w in census.values())
    sleep = sum(w[SLEEPERS] for w in census.values())
    woke = sum(w[WOKEN] - w[WOKEN0] for w in census.values())
    print(f"{name}: live jobs {jobs:,}, sleeper-visits {sleep:,}, woken {woke:,}")


def short(name, head, visits, stamps, lattice, by_phase, total):
    """Short-lived laterals: phytomer shares, and GPU time by records visited."""
    nums = [int(x) for x in head.split("phytomers [")[1].split("]")[0].split(",")]
    lat = [int(x) for x in head.split("laterals [")[1].split("]")[0].split(",")]
    print(f"\n{name} short-lived: phytomers {nums[0]:,}; under 3/5/10: " + ", ".join(f"{100 * n / nums[0]:.1f}%" for n in nums[1:]) + f"; laterals {lat[0]:,}: " + ", ".join(f"{100 * n / lat[0]:.1f}%" for n in lat[1:]))
    # Each phase's visited set: (column prefix of the visits file).
    by = {"units and wake": "grew", "layout": "laid", "light": "laid", "balance and allotment": "held", "retirement": "held", "sizing": "held", "compaction": "held"}
    att = [0.0, 0.0, 0.0]
    phase_att = {}
    for p, col in by.items():
        a = [0.0, 0.0, 0.0]
        for c, v in visits.items():
            t = stamps.get((c, p), 0.0)
            if p == "light":
                t -= lattice.get(c, 0.0)
            if v[col] == 0:
                continue
            for k, u in enumerate(["3", "5", "10"]):
                a[k] += t * v[f"{col}_{u}"] / v[col]
        phase_att[p] = a
        att = [x + y for x, y in zip(att, a)]
    print(f"{name} GPU time on short-lived laterals by records visited (of {total / 1e6:.1f} ms): " + ", ".join(f"under {u}: {x / 1e6:.1f} ms ({100 * x / total:.1f}%)" for u, x in zip([3, 5, 10], att)))
    # The lattice's kernels by the leaves' share too: the upper bound.
    up = [0.0, 0.0, 0.0]
    for c, v in visits.items():
        if v["laid"]:
            for k, u in enumerate(["3", "5", "10"]):
                up[k] += lattice.get(c, 0.0) * v[f"laid_{u}"] / v["laid"]
    print(f"{name} with the lattice by the leaves' share: " + ", ".join(f"under {u}: {(x + y) / 1e6:.1f} ms ({100 * (x + y) / total:.1f}%)" for u, x, y in zip([3, 5, 10], att, up)))
    for p, a in phase_att.items():
        print(f"   {p}: " + ", ".join(f"{x / 1e6:.1f}" for x in a) + f" of {by_phase[p] / 1e6:.1f} ms")


main()
