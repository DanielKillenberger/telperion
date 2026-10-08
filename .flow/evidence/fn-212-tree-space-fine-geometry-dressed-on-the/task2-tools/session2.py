#!/usr/bin/env python3
"""fn-212 task 2's measurements after the reset: the last state's digests
at 80, then the timings of every state alternated under one lock hold per
tree and repetition, the spruce's leaves dumped at the states either side
of the positions fix."""
import os, subprocess, sys, time

LOCK = "/tmp/claude-1000/-home-daniel-Projects-telperion/dd1a391e-8087-41d5-8d26-b3ea78d9cb37/scratchpad/gpu.lock"
R = "/home/daniel/Projects/telperion/.worktrees/readers/.flow/evidence/fn-212-tree-space-fine-geometry-dressed-on-the/raw/task2"
B = f"{R}/bin"
STATES = ["s0", "s1", "s2", "s3", "s4"]
steps = sys.argv[1:] or ["digest", "times"]


def locked(script, tag):
    """Runs `script` (a shell line) under the lock, retrying a lock timeout."""
    while True:
        r = subprocess.run(["flock", "-E", "99", "-w", "1800", LOCK, "bash", "-c", script])
        if r.returncode != 99:
            print(time.strftime("%H:%M:%S"), tag, r.returncode, flush=True)
            return


if "digest" in steps:
    locked(f"DIGEST_AGE=80 {B}/digest-s4 european-beech norway-spruce > {R}/digest-s4-80.jsonl 2> {R}/digest-s4-80.err", "digest s4 80")
if "times" in steps:
    trees = [("european-beech", "80", "1"), ("norway-spruce", "80", "1")]
    for rep in range(2):
        for tree in trees:
            order = STATES if rep == 0 else STATES[::-1]
            lines = []
            for st in order:
                name = f"time-{st}-{'-'.join(tree)}-r{rep}"
                dump = ""
                if rep == 0 and tree[0] == "norway-spruce" and st in ("s2", "s3"):
                    dump = f"DUMP={R}/leaves-{st}-{'-'.join(tree)}.bin "
                lines.append(f"{dump}{B}/time-{st} {' '.join(tree)} 2 > {R}/{name}.jsonl 2> {R}/{name}.err")
            locked("; ".join(lines), f"times {tree[0]} r{rep}")
