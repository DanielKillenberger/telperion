#!/usr/bin/env python3
"""fn-212 task 1: the baseline, one tree a process, three builds each (cold,
then two warm), the device's memory sampled every 100 ms beside it.

Run from the worktree with baseline-marks.patch (and build-fix.patch)
applied and baseline-probe.rs copied to examples/f212_baseline.rs, under
the GPU lock:  baseline-run.py <out dir>
"""
import pathlib
import subprocess
import sys
import time

TREES = [
    ("european-beech", "80", "1"),
    ("european-beech", "80", "7"),
    ("norway-spruce", "80", "1"),
    ("norway-spruce", "80", "7"),
]

out = pathlib.Path(sys.argv[1])
out.mkdir(parents=True, exist_ok=True)
for tree in TREES:
    name = "-".join(tree)
    with open(out / f"{name}.smi", "w") as smi_file:
        smi = subprocess.Popen(
            ["nvidia-smi", "--query-gpu=memory.used", "--format=csv,noheader,nounits", "-lms", "100"],
            stdout=smi_file,
        )
        time.sleep(1.0)
        with open(out / f"{name}.jsonl", "w") as o, open(out / f"{name}.err", "w") as e:
            subprocess.run(["./target/ci/examples/f212_baseline", *tree, "3"], stdout=o, stderr=e, check=False)
        time.sleep(0.5)
        smi.terminate()
        smi.wait()
