"""fn-342 R1: reduce `turns_measures` logs (probes/turns.patch) to each
cause's share of the joints' turns on the limbs' and boughs' thickest-wood
paths, per joint and at the eye's scale (chords along the path).

Run: python3 -I turns.py [--pa=6,7] <log> [<log> ...]   (one age per log;
--pa limits the walk to wood of those PAs, limb 6 and bough 7 by default)
Writes nothing; prints markdown tables.
"""
import math
import statistics
import sys
from collections import defaultdict

T = 10.0            # a joint turn this many degrees or more is counted
WINDOWS = (0.3, 1.0)  # chord lengths, m, for the eye-scale turns
CHORD_T = 15.0      # a chord turn this many degrees or more is counted
WOOD = {6, 7}       # limb and bough


def kv(words):
    out = {}
    for w in words:
        if "=" in w:
            k, v = w.split("=", 1)
            try:
                out[k] = float(v)
            except ValueError:
                out[k] = v
    return out


def load(path):
    segs, joints, trees = defaultdict(list), defaultdict(dict), {}
    for line in open(path):
        w = line.split()
        if not w:
            continue
        if w[0] == "TREE":
            trees[int(w[1])] = kv(w[2:])
        elif w[0] == "SEG":
            seed, p, k = int(w[1]), int(w[2]), int(w[3])
            xs = [float(x) for x in w[4:10]]
            d = kv(w[10:])
            segs[(seed, p)].append((k, xs, d["r"], int(d["pa"])))
        elif w[0] == "JOINT":
            seed, p, k = int(w[1]), int(w[2]), int(w[3])
            joints[(seed, p)][k] = kv(w[4:])
    return trees, segs, joints


def dominant(js):
    """The cause holding most of the summed turn of `js`."""
    s = defaultdict(float)
    for j in js:
        s[j["cause"]] += j["turn"]
    return max(s, key=s.get) if s else "none"


def angle(a, b):
    na, nb = math.hypot(*a), math.hypot(*b)
    c = sum(x * y for x, y in zip(a, b)) / max(na * nb, 1e-12)
    return math.degrees(math.acos(max(-1.0, min(1.0, c))))


def main():
    global WOOD
    args = sys.argv[1:]
    rmin = 0.0
    while args and args[0].startswith("--"):
        a = args.pop(0)
        if a == "--pa=all":
            WOOD = set(range(16))
        elif a.startswith("--pa="):
            WOOD = {int(x) for x in a[5:].split(",")}
        elif a.startswith("--rmin="):
            rmin = float(a[7:])
    for path in args:
        trees, segs, joints = load(path)
        age = int(next(iter(trees.values()))["age"])
        neutral = next(iter(trees.values()))["neutral"]
        length = 0.0
        rows = defaultdict(lambda: [0, 0.0, 0, 0.0])  # n, sum, n>T, sum>T
        radii, lens = [], []
        by_pa = defaultdict(lambda: [0.0, 0, defaultdict(int)])  # metres, joints >= T, causes
        chord = {w: defaultdict(int) for w in WINDOWS}
        chord_n = {w: 0 for w in WINDOWS}
        chord_turns = {w: [] for w in WINDOWS}
        for key, ss in segs.items():
            ss.sort()
            # The path's limb and bough wood: up to its first finer segment.
            wood, pas = [], {}
            for k, xs, r, pa in ss:
                if pa not in WOOD or r < rmin:
                    break
                wood.append((k, xs, r))
                pas[k] = pa
            if not wood:
                continue
            js = joints[key]
            s_at, s = [], 0.0
            pts = [wood[0][1][:3]]
            for k, xs, r in wood:
                seg = math.dist(xs[:3], xs[3:])
                s_at.append(s + seg)  # arc length at this segment's tip: joint k
                s += seg
                pts.append(xs[3:])
                radii.append(r)
                lens.append(seg)
                by_pa[pas[k]][0] += seg
            length += s
            for k, _, _ in wood[:-1]:
                j = js.get(k)
                if not j:
                    continue
                row = rows[j["cause"]]
                row[0] += 1
                row[1] += j["turn"]
                if j["turn"] >= T:
                    row[2] += 1
                    row[3] += j["turn"]
                    by_pa[pas[k + 1] if k + 1 in pas else pas[k]][1] += 1
                    by_pa[pas[k + 1] if k + 1 in pas else pas[k]][2][j["cause"]] += 1
            # Chords along the path: points every W of arc, the turn between
            # successive chords, its cause the joints inside the two chords.
            arcs = [0.0] + s_at
            def at(t):
                i = min(range(1, len(arcs)), key=lambda q: abs(arcs[q] - t))
                return pts[i], i
            for W in WINDOWS:
                t = 0.0
                while t + 2 * W <= s:
                    (a, ia), (b, ib), (c, ic) = at(t), at(t + W), at(t + 2 * W)
                    turn = angle([b[i] - a[i] for i in range(3)], [c[i] - b[i] for i in range(3)])
                    chord_n[W] += 1
                    chord_turns[W].append(turn)
                    if turn >= CHORD_T:
                        inside = [js[k] for k, _, _ in wood[ia - 1:ic - 1] if k in js]
                        chord[W][dominant(inside)] += 1
                    t += W
        n_all = sum(r[0] for r in rows.values())
        s_all = sum(r[1] for r in rows.values())
        n_t = sum(r[2] for r in rows.values())
        s_t = sum(r[3] for r in rows.values())
        print(f"## oak at {age}, seeds {sorted(trees)}, neutral {neutral}, PAs {sorted(WOOD)}")
        print(f"- limb and bough wood walked: {length:.1f} m; joints {n_all}; "
              f"median segment {statistics.median(lens):.4f} m; median radius {statistics.median(radii):.4f} m")
        print(f"- joints of {T:.0f} deg or more: {n_t}, {n_t / length:.2f} per m; "
              f"summed turn {s_all:.0f} deg, {s_all / length:.1f} deg per m")
        print()
        print(f"| cause | joints | share of joints >= {T:.0f} deg | per m >= {T:.0f} deg | share of summed turn | share of summed turn >= {T:.0f} deg | median turn, deg |")
        print("|---|---|---|---|---|---|---|")
        for c, r in sorted(rows.items(), key=lambda x: -x[1][1]):
            print(f"| {c} | {r[0]} | {r[2] / max(n_t, 1):.2f} | {r[2] / length:.3f} | "
                  f"{r[1] / s_all:.2f} | {r[3] / max(s_t, 1e-9):.2f} | {r[1] / max(r[0], 1):.2f} (mean) |")
        print()
        if len(by_pa) > 1:
            print(f"| PA of the wood after the joint | metres | joints >= {T:.0f} deg per m | their causes |")
            print("|---|---|---|---|")
            for pa, (m, n, cs) in sorted(by_pa.items()):
                print(f"| {pa} | {m:.1f} | {n / max(m, 1e-9):.2f} | "
                      + ", ".join(f"{c} {x}" for c, x in sorted(cs.items(), key=lambda y: -y[1])) + " |")
            print()
        for W in WINDOWS:
            ts = chord_turns[W]
            big = sum(chord[W].values())
            q = statistics.quantiles(ts, n=10) if len(ts) > 10 else [float("nan")] * 9
            print(f"- chords of {W} m: {chord_n[W]}, turn median {statistics.median(ts):.1f} deg, "
                  f"90th {q[8]:.1f} deg; of {CHORD_T:.0f} deg or more: {big} "
                  f"({big / max(chord_n[W], 1):.2f} of chords, {big / length:.2f} per m); dominant cause: "
                  + ", ".join(f"{c} {n} ({n / max(big, 1):.2f})" for c, n in sorted(chord[W].items(), key=lambda x: -x[1])))
        print()


if __name__ == "__main__":
    main()
