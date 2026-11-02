"""fn-326 R5: the three presets at each manifest fidelity, through their own rows.

Usage: r5.py <binary> <cpu> <seeds>. Prints one table row per preset and fidelity:
six medians against the finest and one-thread seconds, native, five repeats less
the first (a grow over 0.5 s is not repeated).
"""
import os, re, subprocess, sys

EV = ".flow/evidence/fn-326-the-grower-takes-a-fidelity/"
PRESETS = ["oregon-white-oak", "norway-spruce", "silver-birch"]
KEYS = ["first", "factor", "leaf", "pipe", "bud", "hazard", "shedding"]
binary, cpu, seeds = sys.argv[1:4]
print("| Preset | Fidelity | Height | Spread | Trunk | Limbs | Shed | Area | Worst | Seconds |")
print("| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |")
for preset in PRESETS:
    text = open(f"crates/telperion-core/presets/{preset}.values").read()
    values = [re.search(rf"^/space/coarse/{k} = (\S+)$", text, re.M).group(1) for k in KEYS]
    file = f"{EV}raw/r5-{preset}.txt"
    with open(file, "w") as f:
        for fidelity in ("0", "0.05", "0.25", "1"):
            f.write(" ".join(values) + f" {fidelity}\n")
    env = dict(os.environ, RAYON_NUM_THREADS="1", SEARCH_PRESET=preset, SEARCH_SEEDS=seeds, SEARCH_REPEATS="5", SEARCH_FILE=file)
    out = subprocess.run(["telperion-heavy", "8", "--threads", "1", "--", "taskset", "-c", cpu, binary, "fidelity_rows", "--ignored", "--nocapture", "--test-threads=1"], env=env, capture_output=True, text=True, check=True).stdout
    for line in out.splitlines():
        i = line.find("CAND ")
        if i >= 0:
            w = line[i:].split()[1:]
            r = {w[k]: w[k + 1] for k in range(0, len(w) - 1, 2)}
            print(f"| {preset} | {r['fidelity']} | " + " | ".join(r[k] for k in ("height", "spread", "trunk", "limbs", "shed", "area", "worst", "seconds")) + " |", flush=True)
