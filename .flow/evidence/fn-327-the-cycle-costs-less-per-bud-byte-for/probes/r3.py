"""fn-327 R3: base against candidate, alternating batches, one pinned thread.

Usage: r3.py <base-binary> <candidate-binary> <cpu> <batches>. Each batch grows the
preset's own rows at fidelity 0 and 1 on held-out seeds 1, 7, 42, 1234, 5678.
Prints each preset and fidelity's median seconds and the median ratio of the batches.
"""
import os, re, statistics, subprocess, sys

EV = ".flow/evidence/fn-327-the-cycle-costs-less-per-bud-byte-for/raw/"
KEYS = ["first", "factor", "leaf", "pipe", "bud", "hazard", "shedding"]
base, candidate, cpu, batches = sys.argv[1], sys.argv[2], sys.argv[3], int(sys.argv[4])


def run(binary, preset, file):
    env = dict(os.environ, RAYON_NUM_THREADS="1", SEARCH_PRESET=preset, SEARCH_SEEDS="1,7,42,1234,5678", SEARCH_REPEATS="5", SEARCH_FILE=file)
    out = subprocess.run(["telperion-heavy", "8", "--threads", "1", "--", "taskset", "-c", cpu, binary, "fidelity_rows", "--ignored", "--nocapture", "--test-threads=1"], env=env, capture_output=True, text=True, check=True).stdout
    found = {}
    for line in out.splitlines():
        i = line.find("CAND ")
        if i >= 0:
            w = line[i:].split()[1:]
            r = {w[k]: w[k + 1] for k in range(0, len(w) - 1, 2)}
            found[r["fidelity"]] = float(r["seconds"])
    return found


print("| Preset | Fidelity | Base s | Candidate s | Median ratio | Worst batch |")
print("| --- | --- | --- | --- | --- | --- |")
for preset in ["oregon-white-oak", "norway-spruce", "silver-birch"]:
    text = open(f"crates/telperion-core/presets/{preset}.values").read()
    values = [re.search(rf"^/space/coarse/{k} = (\S+)$", text, re.M).group(1) for k in KEYS]
    file = f"{EV}r3-{preset}.txt"
    with open(file, "w") as f:
        f.write(" ".join(values) + " 0\n" + " ".join(values) + " 1\n")
    a, b = {"0": [], "1": []}, {"0": [], "1": []}
    for n in range(batches):
        order = [(a, base), (b, candidate)] if n % 2 == 0 else [(b, candidate), (a, base)]
        for sink, binary in order:
            for k, v in run(binary, preset, file).items():
                sink[k].append(v)
    for k in ("0", "1"):
        ratios = [y / x for x, y in zip(a[k], b[k])]
        print(f"| {preset} | {k} | {statistics.median(a[k]):.4f} | {statistics.median(b[k]):.4f} | {statistics.median(ratios):.3f} | {max(ratios):.3f} |", flush=True)
