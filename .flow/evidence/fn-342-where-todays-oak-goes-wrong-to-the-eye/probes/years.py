"""fn-342 R3: reduce the yearly record (`PROBE_YEAR`, probes/turns.patch) of
one grow to 300: height and its gain, the stem line's limbs (PA 6) and boughs
(PA 7) with birth, death and their balance's last five years, the lowest
living limb. Heights are the rough layout's, before the settle's sag.

Run: python3 -I years.py <log>
"""
import sys
from collections import defaultdict


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


def main():
    seeds, cur = {}, None
    years, limbs = [], defaultdict(list)
    for line in open(sys.argv[1]):
        w = line.split()
        if not w:
            continue
        if w[0] == "YEAR":
            years.append((int(w[1]), kv(w[2:])))
        elif w[0] == "LIMBY":
            d = kv(w[2:])
            limbs[int(d["lineage"])].append((int(w[1]), d))  # an index moves when the grower compacts
        elif w[0] == "TREE":
            seeds[int(w[1])] = (years, limbs, kv(w[2:]))
            years, limbs = [], defaultdict(list)
    for seed, (years, limbs, tree) in seeds.items():
        print(f"## seed {seed}: settled height at 300 {tree['height']:.1f} m")
        h = {y: d["height"] for y, d in years}
        stem = {y: d["stem_top"] for y, d in years}
        print("| years | height at end, m | mean yearly gain, m | stem line's top, m | living limbs at end | lowest living limb, m |")
        print("|---|---|---|---|---|---|")
        dy = {y: d for y, d in years}
        for a, b in [(1, 20), (20, 40), (40, 80), (80, 120), (120, 160), (160, 200), (200, 250), (250, 299)]:
            if a in h and b in h:
                print(f"| {a}-{b} | {h[b]:.1f} | {(h[b] - h[a]) / (b - a):.3f} | {stem[b]:.1f} | "
                      f"{int(dy[b]['limbs_live'])} | {dy[b]['lowest_limb']:.1f} |")
        print()
        print("| lineage (last 6) | PA | born | base, m | last year living | shed by the balance | its last five years: light; upkeep of living wood; of dead wood standing; memory |")
        print("|---|---|---|---|---|---|---|")
        for axis, rows in sorted(limbs.items(), key=lambda x: x[1][0][1]["base"]):
            pa = int(rows[0][1]["pa"])
            living = [y for y, d in rows if d["live"] == 1]
            if pa == 7 and (not living or max(living) - min(living) < 30):
                continue
            last = max(living) if living else None
            shed = [y for y, d in rows if d["shed"] == 1]
            tail = [d for y, d in rows if last is not None and last - 4 <= y <= last]
            five = "; ".join(f"{d['light']:.3f}/{d['cost_live']:.3f}/{d['cost_dead']:.3f}/{d['memory']:.2f}" for d in tail)
            print(f"| {axis % 10**6} | {pa} | {int(rows[0][1]['birth'])} | {rows[0][1]['base']:.1f} | {last} | "
                  f"{shed[0] if shed else '-'} | {five} |")
        print()


if __name__ == "__main__":
    main()
