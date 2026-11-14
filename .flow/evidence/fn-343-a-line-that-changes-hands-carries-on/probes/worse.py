"""fn-343: what makes a bearer that runs on turn >= 10 deg under the linear
rule: the lateral-over-own radius ratio, lateral count and theta at those nodes.
Run: python3 -I worse.py <fork log>"""
import math, sys
import os
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
from share import parse, ang, unit, cross, rot, share
ratios, counts, thetas, single = [], [], [], 0
pending = None
for line in open(sys.argv[1]):
    w = line.split()
    if not w:
        continue
    if w[0] == "FORK":
        pending = w
    elif w[0] == "JOINT":
        d = dict(t.split("=", 1) for t in w[4:] if "=" in t)
        f, pending = pending, None
        if float(d["r"]) < 0.01 or float(d["turn"]) >= 10:
            continue
        below = [float(x) for x in f[4].split("=")[1].split(",")]
        woods = [parse(t) for t in f[5:]]
        taken = next(x for x in woods if x[0])
        if taken[1] != "own":
            continue
        ab = taken[2] ** 2
        bear, lats = [0.0] * 3, []
        for star, kind, r, sv, hv in woods:
            if kind != "lateral":
                continue
            n = unit(cross(below, hv))
            th = math.radians(ang(below, hv))
            xl, xb = share(th, ab, r * r, "linear")
            bear = [bear[i] - xb * n[i] for i in range(3)]
            lats.append((r / taken[2], math.degrees(th), math.degrees(xb)))
        if lats and ang(below, rot(taken[3], bear)) >= 10:
            ratios.append(max(l[0] for l in lats)); counts.append(len(lats))
            thetas.append(max(lats)[1]); single += len(lats) == 1
q = lambda v, f: sorted(v)[int(f * (len(v) - 1))]
print(f"{len(ratios)} bearers made to turn >= 10: largest lateral's radius over the bearer's own, "
      f"quartiles {q(ratios,.25):.2f} {q(ratios,.5):.2f} {q(ratios,.75):.2f}; laterals at the node median "
      f"{q(counts,.5)}, one lateral {single / len(ratios):.2f}; that lateral's theta median {q(thetas,.5):.0f} deg")
