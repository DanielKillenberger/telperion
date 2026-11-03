"""fn-332 R1 and R2 again with the probe as reviewed: every seed and age,
and the four causes at 200 on seed 1, six at a time, one thread each,
through the queue. Logs land in raw/v2/."""
import os
import sys
import subprocess
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

HERE = Path(__file__).resolve().parent.parent
RAW = HERE / "raw" / os.environ.get("RERUN_OUT", "v2")
RAW.mkdir(parents=True, exist_ok=True)
RUNS = [("base", s, a) for s in (1, 7, 42) for a in (40, 60, 80, 100, 120, 160, 200)]
RUNS += [(v, 1, 200) for v in ("TRUNK_SAG=0", "TRUNK_WANDER=0", "TRUNK_TROPISM=3", "TRUNK_SHADE=0")]
# Named runs only, as `name:seed:age`, e.g. after one was killed at its claim.
if sys.argv[1:]:
    RUNS = [(n, int(s), int(a)) for n, s, a in (x.split(":") for x in sys.argv[1:])]


def run(job):
    name, seed, age = job
    env = dict(os.environ, RAYON_NUM_THREADS="1")
    if name != "base":
        key, value = name.split("=")
        env[key] = value
    cmd = ["target/ci/examples/trunk", "oregon-white-oak", str(age), str(seed)]
    # Through the desk's queue unless RERUN_CLAIM=0 (a machine with room).
    if os.environ.get("RERUN_CLAIM", "3") != "0":
        cmd = ["telperion-heavy", os.environ.get("RERUN_CLAIM", "3"), "--threads", "1", "--", *cmd]
    out = subprocess.run(cmd, env=env, capture_output=True, text=True)
    (RAW / f"{name}-s{seed}-a{age}.log").write_text(out.stdout + out.stderr)
    return name, seed, age, out.returncode


with ThreadPoolExecutor(int(os.environ.get("RERUN_AT_ONCE", "6"))) as pool:
    for r in pool.map(run, RUNS):
        print(*r, flush=True)
