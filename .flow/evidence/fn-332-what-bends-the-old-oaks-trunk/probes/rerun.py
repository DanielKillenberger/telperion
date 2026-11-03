"""fn-332 R1 and R2 again with the probe as reviewed: every seed and age,
and the four causes at 200 on seed 1, six at a time, one thread each,
through the queue. Logs land in raw/v2/."""
import os
import subprocess
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

HERE = Path(__file__).resolve().parent.parent
RAW = HERE / "raw" / "v2"
RAW.mkdir(parents=True, exist_ok=True)
RUNS = [("base", s, a) for s in (1, 7, 42) for a in (40, 60, 80, 100, 120, 160, 200)]
RUNS += [(v, 1, 200) for v in ("TRUNK_SAG=0", "TRUNK_WANDER=0", "TRUNK_TROPISM=3", "TRUNK_SHADE=0")]


def run(job):
    name, seed, age = job
    env = dict(os.environ, RAYON_NUM_THREADS="1")
    if name != "base":
        key, value = name.split("=")
        env[key] = value
    cmd = ["telperion-heavy", "3", "--threads", "1", "--",
           "target/ci/examples/trunk", "oregon-white-oak", str(age), str(seed)]
    out = subprocess.run(cmd, env=env, capture_output=True, text=True)
    (RAW / f"{name}-s{seed}-a{age}.log").write_text(out.stdout + out.stderr)
    return name, seed, age, out.returncode


with ThreadPoolExecutor(6) as pool:
    for r in pool.map(run, RUNS):
        print(*r, flush=True)
