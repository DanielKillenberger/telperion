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


def main() -> int:
    cmd = sys.argv[1:]
    if "--list" in cmd:
        os.execv(cmd[0], cmd)
    rc = subprocess.call(cmd)
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
