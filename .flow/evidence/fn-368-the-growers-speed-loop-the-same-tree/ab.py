"""fn-368: ab.sh's runs read back: per tree and side, the median grown
tree's CPU seconds, its spread, the most peak memory and the digests; then
the after side's gain on the before side, by medians and by the median of
the pairs' own ratios.

    python3 ab.py <raw/after/ab>
"""
import pathlib
import re
import statistics
import sys

out = pathlib.Path(sys.argv[1])
runs = {}
for f in sorted(out.glob("*.txt")):
    m = re.match(r"(.+?)-(.+-\d+-\d+)-(\d+)$", f.stem)
    side, tree, k = m.group(1), m.group(2), int(m.group(3))
    text = f.read_text()
    cpu = re.search(r"  cpu: ([\d.]+) s", text)
    peak = re.search(r"PEAK_MIB (\d+)", text)
    digest = re.search(r"digest: (\d+)", text)
    runs.setdefault(tree, {}).setdefault(side, {})[k] = (
        float(cpu.group(1)) if cpu else None,
        int(peak.group(1)) if peak else 0,
        digest.group(1) if digest else "none",
    )

for tree, sides in runs.items():
    names = sorted(sides, key=lambda s: min(sides[s]))
    for side in sides:
        cpus = [v[0] for v in sides[side].values() if v[0] is not None]
        peak = max(v[1] for v in sides[side].values())
        digests = sorted({v[2] for v in sides[side].values()})
        med = statistics.median(cpus) if cpus else float("nan")
        print(f"{tree} {side}: median {med:.2f} of {sorted(cpus)}, peak {peak} MiB, digest {digests}")
    if len(sides) == 2:
        a, b = list(sides)
        pairs = [
            sides[b][k][0] / sides[a][k][0]
            for k in sides[a]
            if k in sides[b] and sides[a][k][0] and sides[b][k][0]
        ]
        ma = statistics.median(v[0] for v in sides[a].values() if v[0])
        mb = statistics.median(v[0] for v in sides[b].values() if v[0])
        print(f"  {b} on {a}: medians {100 * (1 - mb / ma):+.1f}%, pairs' ratio {100 * (1 - statistics.median(pairs)):+.1f}%")
