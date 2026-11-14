"""fn-334 R3: the derived dial's read, one pinned process per preset.

Usage: python3 r3.py <test-binary> [presets...]. Runs `derived_rows` for each
preset of the catalogue (or those named) at its own age, fidelities 0.25, 0.5
and 1.0, seeds 1001 to 1012, five repeats, candidates D1 and D0, one thread,
each process through telperion-heavy and pinned to its own core. Raw logs go
to the evidence's ignored raw/ directory, one per preset.
"""
import os, subprocess, sys

EV = ".flow/evidence/fn-334-the-dials-rows-are-derived-from-the/raw/"
CATALOGUE = ["ordinary", "oregon-white-oak", "norway-spruce", "european-beech",
             "silver-birch", "date-palm", "telperion", "laurelin"]
BIG = {"telperion": "8", "laurelin": "8"}
binary, presets = sys.argv[1], sys.argv[2:] or CATALOGUE

jobs = []
for n, preset in enumerate(presets):
    env = dict(os.environ, RAYON_NUM_THREADS="1", DERIVED_PRESET=preset,
               DERIVED_SEEDS=",".join(str(s) for s in range(1001, 1013)),
               DERIVED_REPEATS="5", DERIVED_FIDELITIES="0.25,0.5,1.0",
               DERIVED_CANDIDATES="D1,D0")
    cpu = str(8 + n)
    cmd = ["telperion-heavy", BIG.get(preset, "4"), "--threads", "1", "--",
           "taskset", "-c", cpu, binary, "derived_rows", "--ignored",
           "--nocapture", "--test-threads=1"]
    log = open(f"{EV}r3-{preset}.log", "w")
    jobs.append((preset, subprocess.Popen(cmd, env=env, stdout=log, stderr=subprocess.STDOUT)))

for preset, job in jobs:
    print(preset, "exit", job.wait(), flush=True)
