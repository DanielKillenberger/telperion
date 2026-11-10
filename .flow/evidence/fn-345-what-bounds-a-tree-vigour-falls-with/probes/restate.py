"""fn-345 R1 arithmetic, not a run: today's (A's) leader height each year,
read from fn-344's 300-year grow log (seed 1001, rough layout), replayed
with each year's gain multiplied by k (1 - H / 34.8), the path taken as
the height (a lower bound on the path, so an upper bound on the height).
Prints the k that holds year 39 at A's height, and both replays to 299.
Usage: python3 -I restate.py <A log> <H log>
"""
import re, sys

LMAX = 34.8


def heights(path):
    h = {}
    for line in open(path):
        m = re.match(r"YEAR (\d+) held=\d+ height=(\S+) ", line)
        if m and int(m[1]) > 0:
            h[int(m[1])] = float(m[2])
    return h


a, hh = heights(sys.argv[1]), heights(sys.argv[2])
last = max(a)


def replay(k):
    out, h, prev = {}, a[1], a[1]
    for y in range(2, last + 1):
        gain = a[y] - prev
        prev = a[y]
        h += k * max(gain, 0.0) * max(0.0, 1.0 - h / LMAX)
        out[y] = h
    return out


lo, hi = 1.0, 3.0
for _ in range(60):
    mid = (lo + hi) / 2
    lo, hi = (mid, hi) if replay(mid)[39] < a[39] else (lo, mid)
k = (lo + hi) / 2
r1, rk = replay(1.0), replay(k)
print(f"k holding year 39 at A's {a[39]:.2f} m: {k:.3f}")
print("| year | A today m | H run m | replay k=1 m | replay k m |")
print("|---|---|---|---|---|")
for y in (39, 59, 79, 99, 119, 159, 199, 259, 299):
    print(f"| {y} | {a[y]:.1f} | {hh.get(y, float('nan')):.1f} | {r1[y]:.1f} | {rk[y]:.1f} |")
print("A's mean gain a year: 0-39 %.3f, 39-119 %.3f, 119-299 %.3f m" % (
    a[39] / 39, (a[119] - a[39]) / 80, (a[299] - a[119]) / 180))
for L in (5, 10, 20, 30):
    f = 1 - L / LMAX
    print(f"L={L} m: factor {f:.3f}, cost {1 - f:.1%}; with k: {k * f:.3f}")
