#!/usr/bin/env python3
"""The gate's target runner (fn-304 R4): runs one test process and fails a
test whose peak resident memory passes its budget, 4 GiB or its figure in
`.config/gate-budgets.toml`. nextest runs each test in its own process, so
the process's peak is the test's. `scripts/gate.py` sets it as the runner;
it is never the build's runner.
"""
import os
import resource
import signal
import subprocess
import sys
import tomllib

MIB = 4096


def budget(key: str) -> int:
    here = os.path.dirname(os.path.abspath(__file__))
    path = os.path.join(here, "..", ".config", "gate-budgets.toml")
    with open(path, "rb") as f:
        listed = {t["name"]: t["mib"] for t in tomllib.load(f).get("test", [])}
    return listed.get(key, MIB)


def one_arena(env) -> dict:
    """The test's environment with glibc held to one malloc arena. A build
    grows on the pool's threads and draws its wood on another, glibc gives
    each thread an arena, and bytes one arena has freed are not handed to the
    next: 1.7 to 2.0 GiB of a Two Trees build's peak was resident and not
    live (fn-351 NATIVE-PEAK.md, 14 Nov 2026). One arena measures what the
    test keeps. A setting the caller already made stands."""
    env = dict(env)
    tunables = env.get("GLIBC_TUNABLES", "")
    if "glibc.malloc.arena_max" not in tunables:
        ours = "glibc.malloc.arena_max=1"
        env["GLIBC_TUNABLES"] = f"{tunables}:{ours}" if tunables else ours
    return env


def main() -> int:
    cmd = sys.argv[1:]
    if "--list" in cmd:
        os.execv(cmd[0], cmd)
    rc = subprocess.call(cmd, env=one_arena(os.environ))
    if rc < 0:
        signal.signal(-rc, signal.SIG_DFL)
        os.kill(os.getpid(), -rc)
    binary = os.path.basename(cmd[0]).rsplit("-", 1)[0]
    test = cmd[cmd.index("--exact") + 1] if "--exact" in cmd else ""
    key = f"{binary} {test}"
    peak_kib = resource.getrusage(resource.RUSAGE_CHILDREN).ru_maxrss
    peak = -(-peak_kib // 1024)
    report = os.environ.get("GATE_REPORT")
    if report:
        with open(report, "a") as f:
            f.write(f"{key}\t{peak}\n")
    limit = budget(key)
    if rc == 0 and peak_kib > limit * 1024:
        print(f"fn-304 memory budget: {key} peaked at {peak} MiB, over its "
              f"{limit} MiB (.config/gate-budgets.toml)", file=sys.stderr)
        return 1
    return rc


if __name__ == "__main__":
    sys.exit(main())
