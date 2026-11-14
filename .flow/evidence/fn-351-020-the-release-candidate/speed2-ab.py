"""fn-351 speed round 2: speed2-ab.sh's runs read back. Per tree, thread
count and side: the median of the tree's own clock (`cycle: … s`, growth
and settle), of the process's CPU seconds to the grown tree, of its
user-space instructions and cache misses, and its digests; then the after
side's gain on the before side by medians.

    python3 speed2-ab.py <raw/speed2/after/ab>
"""
import pathlib
import re
import statistics
import sys

out = pathlib.Path(sys.argv[1])
runs = {}
for f in sorted(out.glob("*.txt")):
    m = re.match(r"(.+?)-([a-z-]+-\d+-\d+)-t(\d+)-(\d+)$", f.stem)
    side, tree, threads = m.group(1), m.group(2), int(m.group(3))
    text = f.read_text()
    grab = lambda pattern: (re.search(pattern, text) or [None, None])[1]
    row = {
        "clock": grab(r"phytomers on \d+ axes, ([\d.]+) s"),
        "cpu": grab(r"  cpu: ([\d.]+) s"),
        "instructions": grab(r"instructions (\d+)"),
        "cycles": grab(r"cycles (\d+)"),
        "cache_misses": grab(r"cache_misses (\d+)"),
    }
    row = {k: float(v) for k, v in row.items() if v is not None}
    row["digest"] = grab(r"digest: (\d+)") or "none"
    runs.setdefault((tree, threads), {}).setdefault(side, []).append(row)

for (tree, threads), sides in sorted(runs.items()):
    med = {}
    for side, rows in sides.items():
        digests = sorted({r["digest"] for r in rows})
        med[side] = {
            k: statistics.median(r[k] for r in rows if k in r)
            for k in ("clock", "cpu", "instructions", "cycles", "cache_misses")
            if any(k in r for r in rows)
        }
        clocks = sorted(r["clock"] for r in rows if "clock" in r)
        print(
            f"{tree} t{threads} {side}: clock {med[side].get('clock', 0):.3f} s of {clocks}, "
            f"cpu {med[side].get('cpu', 0):.2f} s, "
            f"instructions {med[side].get('instructions', 0) / 1e9:.3f} G, "
            f"cycles {med[side].get('cycles', 0) / 1e9:.3f} G, "
            f"cache misses {med[side].get('cache_misses', 0) / 1e6:.2f} M, digest {digests}"
        )
    if len(sides) == 2:
        a, b = list(sides)
        gains = ", ".join(
            f"{k} {100 * (1 - med[b][k] / med[a][k]):+.1f}%"
            for k in med[a]
            if k in med[b] and med[a][k]
        )
        print(f"  {b} on {a}: {gains}")
