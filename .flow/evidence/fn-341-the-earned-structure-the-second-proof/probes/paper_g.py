"""fn-341 G's paper check: a trunk at 1.3 m from the yearly leaf above it
(`structure_trace`, TRACE year leaf_above d_cycle height), each year's ring
the leaf over R N, summed. R in m² leaf per cm² sapwood; N rings.

Run: python3 -I paper_g.py <trace log> <N: n, age or n@year:n@year> <R or R_low@h_low:R_high@h_high> [ages]
"""
import math
import sys


def ratio(spec, height):
    if "@" not in spec:
        return float(spec)
    (r0, h0), (r1, h1) = [tuple(map(float, p.split("@"))) for p in spec.split(":")]
    t = min(1.0, max(0.0, (height - h0) / (h1 - h0)))
    return r0 + t * (r1 - r0)


def rings(spec, year):
    """N: a number; `age`, every ring the stem has; or steps `n@from:n@from`."""
    if spec == "age":
        return float(year)
    if "@" not in spec:
        return float(spec)
    n = None
    for part in spec.split(":"):
        v, start = map(float, part.split("@"))
        if year >= start:
            n = v
    return n


def main():
    path, n, spec = sys.argv[1], sys.argv[2], sys.argv[3]
    ages = [int(a) for a in sys.argv[4].split(",")] if len(sys.argv) > 4 else [40, 80, 120, 300]
    area = 0.0  # cm²
    rows = {}
    for line in open(path):
        w = line.split()
        if not w or w[0] != "TRACE":
            continue
        y, leaf, d, h = int(w[3]), float(w[4]), float(w[5]), float(w[6])
        if leaf == leaf:  # a year with no trunk at 1.3 m lays nothing there
            area += leaf / (ratio(spec, h) * rings(n, y))
        rows[y] = (leaf, d, h, 2 * math.sqrt(area / math.pi))
    for a in ages:
        if a in rows:
            leaf, d, h, g = rows[a]
            print(f"age {a}: height {h:.1f} m, leaf above {leaf:.0f} m2, cycle trunk {d * 100:.0f} cm, "
                  f"G trunk {g:.1f} cm (N {n}, R {spec})")


if __name__ == "__main__":
    main()
