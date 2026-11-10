"""fn-342 R1: the kinks of one turns log as the whole-tree still draws them.
Each joint of 10 deg or more, on every walked path (laterals 1 cm or more at
their base), binned by its wood's width on screen: diameter times pixels per
metre, the still's 720 px frame holding the tree's height at 0.78 of it
(read off the bare stills). A fork is split by the wood it leaves: `fork-kink`
where the bearer's own continuation is under 0.7 of the lateral taken, so the
lateral reads as the line turning; `fork-Y` otherwise.

Run: python3 -I kinks.py <log> [<log> ...]
"""
import sys
from collections import defaultdict

T = 10.0
BINS = (1, 2, 4, 8)


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


for path in sys.argv[1:]:
    heights, rows, segs = {}, [], []
    for line in open(path):
        w = line.split()
        if w and w[0] == "TREE":
            d = kv(w[2:])
            heights[int(w[1])] = d["height"]
            age = int(d["age"])
        elif w and w[0] == "SEG":
            xs = [float(x) for x in w[4:10]]
            r = kv(w[10:])["r"]
            segs.append((int(w[1]), r, sum((a - b) ** 2 for a, b in zip(xs[:3], xs[3:])) ** 0.5))
        elif w and w[0] == "JOINT":
            d = kv(w[4:])
            d["seed"] = int(w[1])
            rows.append(d)
    count = defaultdict(lambda: [0] * len(BINS))
    for d in rows:
        if d["turn"] < T:
            continue
        c = d["cause"]
        if c == "fork":
            c = "fork-kink" if d["alt"] < 0.7 else "fork-Y"
        px = 2 * d["r"] * 0.78 * 720 / heights[d["seed"]]
        for i, b in enumerate(BINS):
            if px >= b:
                count[c][i] += 1
    n = len(heights)
    print(f"## oak at {age}: joints >= {T:.0f} deg per tree, by their wood's width in the whole still "
          f"({0.78 * 720 / (sum(heights.values()) / n):.0f} px per m)")
    print("| cause | " + " | ".join(f">= {b} px" for b in BINS) + " |")
    print("|---|" + "---|" * len(BINS))
    tot = [0] * len(BINS)
    for c, xs in sorted(count.items(), key=lambda x: -x[1][0]):
        print(f"| {c} | " + " | ".join(f"{x / n:.0f}" for x in xs) + " |")
        tot = [a + b for a, b in zip(tot, xs)]
    print("| all | " + " | ".join(f"{x / n:.0f}" for x in tot) + " |")
    metres = [0.0] * len(BINS)
    for seed, r, m in segs:
        px = 2 * r * 0.78 * 720 / heights[seed]
        for i, b in enumerate(BINS):
            if px >= b:
                metres[i] += m
    print("| walked wood that wide, m per tree | " + " | ".join(f"{m / n:.0f}" for m in metres) + " |")
    print("| joints >= 10 deg per m of it | " + " | ".join(f"{t / max(m, 1e-9):.2f}" for t, m in zip(tot, metres)) + " |")
    print("| per 1,000 px of drawn line | " + " | ".join(
        f"{t / max(m, 1e-9) * 100 / (0.78 * 720 / (sum(heights.values()) / n)) * 10:.1f}" for t, m in zip(tot, metres)) + " |")
    print()
