"""The girth rule on paper (fn-351, OAK-SAPWOOD-BY-SIZE.md): girth.rs's ring,
year by year, on a tree's leaf and height at five ages. Leaf is quadratic to
40 and straight lines between the ages; height is straight lines from the
seed. Both are assumptions: no yearly leaf or height is recorded.

    python3 -I paper.py [data.json]

data.json: {"<seed>": [[leaf m2 at 40, 80, 120, 200, 300], [height m ...]]}.
Without it, the recorded figures of TREES-AGAINST-NATURE.md and fn-364's
MEASURES.md ("fn-363" rows) are used; where a seed has no record at 40 and
80, seed 1's and seed 7's stand in (1012: seed 1's leaf times 1,389 / 968).
"""
import json
import math
import sys

AGES = [40, 80, 120, 200, 300]
DATA = {
    1001: ([113.5, 514, 968, 1041, 590], [10.7, 17.0, 27.4, 36.8, 36.0]),
    1007: ([180, 1016, 1974, 2198, 1111], [11.1, 20.2, 28.4, 37.1, 38.8]),
    1012: (
        [113.5 * 1389 / 968, 514 * 1389 / 968, 1389, 1887, 1371],
        [10.7, 17.0, 26.0, 39.4, 38.2],
    ),
}
if len(sys.argv) > 1:
    with open(sys.argv[1]) as f:
        DATA = {int(k): tuple(v) for k, v in json.load(f).items()}
# White 1998, Table 2 rows 70/5 and 100/3.5, as PHASE2-SURVEY.md computes them.
WHITE = {40: (28, 40), 80: (56, 79), 120: (83, 109), 200: (121, 152), 300: (156, 192)}


def at(xs, ys, x):
    if x <= xs[0]:
        return ys[0]
    for x0, y0, x1, y1 in zip(xs, ys, xs[1:], ys[1:]):
        if x <= x1:
            return y0 + (y1 - y0) * (x - x0) / (x1 - x0)
    return ys[-1]


def leaf(seed, y):
    l = DATA[seed][0]
    return l[0] * (y / 40) ** 2 if y <= 40 else at(AGES, l, y)


def height(seed, y):
    return at([0] + AGES, [0] + list(DATA[seed][1]), y)


def w(h, lo=10.0, hi=25.0):
    """The rule's weight: 0 to 10 m, 1 from 25 m, straight between."""
    return min(1.0, max(0.0, (h - lo) / (hi - lo)))


def lerp(a, b, t):
    return a + (b - a) * t


def classes(y, a, b, c):
    return a if y < 100 else b if y < 200 else c


# Each option: (sapwood cm2 per m2 of leaf, sapwood rings) from age and height.
OPTIONS = {
    "today: 1/0.30, 20 rings": lambda y, h: (1 / 0.30, 20.0),
    "A: sapwood by height, 20 rings": lambda y, h: (
        lerp(1 / 0.23, 1 / 0.37, w(h)),
        20.0,
    ),
    "A': as A, straight in leaf per sapwood": lambda y, h: (
        1 / lerp(0.23, 0.37, w(h)),
        20.0,
    ),
    "B: A, rings 16.0 to 25.9 (Hollstein)": lambda y, h: (
        lerp(1 / 0.23, 1 / 0.37, w(h)),
        lerp(16.0, 25.9, w(h)),
    ),
    "C: A, rings 12 to 35 (Phillips's trees)": lambda y, h: (
        lerp(1 / 0.23, 1 / 0.37, w(h)),
        lerp(12.0, 35.0, w(h)),
    ),
    "R: the reading's age classes": lambda y, h: (
        1 / classes(y, 0.23, 0.30, 0.37),
        classes(y, 16.0, 20.4, 25.9),
    ),
}


def run(seed, option):
    t = s = 0.0
    out = {}
    for y in range(1, AGES[-1] + 1):
        sapwood, rings = option(y, height(seed, y))
        need = leaf(seed, y) * sapwood
        kept = s * (1 - 1 / rings)
        laid = max(0.0, need - kept)
        t, s = t + laid, kept + laid
        if y in AGES:
            out[y] = 2 * math.sqrt(t / math.pi)
    return out


print("| seed | rows | " + " | ".join(str(a) for a in AGES) + " |")
print("|---|---|" + "---|" * len(AGES))
for seed in DATA:
    for name, option in OPTIONS.items():
        r = run(seed, option)
        print(f"| {seed} | {name} | " + " | ".join(f"{r[a]:.0f}" for a in AGES) + " |")
print("| White | open-grown | " + " | ".join(f"{WHITE[a][0]} to {WHITE[a][1]}" for a in AGES) + " |")
