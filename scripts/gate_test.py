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


# Whole-catalogue builds, measured on four malloc arenas. A build grows on the
# pool's threads and draws its wood on another, glibc gives each thread an
# arena, and bytes one arena has freed are not handed to the next: 1.7 to
# 2.0 GiB of a Two Trees build's peak was resident and not live. On four arenas
# these read 359 to 1,032 MiB under their budgets at their unlimited times;
# on one they ran up to 1.8 times as long, and set for every test one arena
# cost the threaded walks their time limits, so the limit is set for
# these alone (fn-351 NATIVE-PEAK.md, 14 Nov 2026).
FEW_ARENAS = {
    "telperion_core pipeline::tests::"
    "every_shipped_preset_builds_every_artifact_through_the_pipeline",
    "telperion_core suite::mesh::"
    "every_preset_builds_a_mesh_whose_counts_and_bounds_describe_its_buffers",
    "telperion_core suite::packed_leaf::"
    "every_station_stands_inside_its_box_at_a_quarter_millimetre_step",
    "conformance every_shipped_family_renders_through_the_one_path",
    "telperion_wasm generate::tests::"
    "every_shipped_preset_builds_through_the_main_entry",
    "telperion_wasm generate::tests::"
    "flowers_cross_the_binding_where_the_pipeline_grows_them",
}


def environment(key: str) -> dict:
    """The test's environment; a malloc setting the caller made stands."""
    env = dict(os.environ)
    tunables = env.get("GLIBC_TUNABLES", "")
    if key in FEW_ARENAS and "glibc.malloc.arena_max" not in tunables:
        ours = "glibc.malloc.arena_max=4"
        env["GLIBC_TUNABLES"] = f"{tunables}:{ours}" if tunables else ours
    return env


def main() -> int:
    cmd = sys.argv[1:]
    if "--list" in cmd:
        os.execv(cmd[0], cmd)
    binary = os.path.basename(cmd[0]).rsplit("-", 1)[0]
    test = cmd[cmd.index("--exact") + 1] if "--exact" in cmd else ""
    key = f"{binary} {test}"
    rc = subprocess.call(cmd, env=environment(key))
    if rc < 0:
        signal.signal(-rc, signal.SIG_DFL)
        os.kill(os.getpid(), -rc)
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
