#!/usr/bin/env python3
"""The local gate (fn-304): the workspace's tests under nextest's `gate`
profile, inside its budgets, against ten minutes.

  telperion-heavy 8 --threads 4 -- python3 scripts/gate.py [--base <ref>]

Budgets (host, fn-304 R4): a test's own time is at most 60 s (the profile's
slow-timeout ends it) and its peak memory at most 4 GiB (`gate_test.py`,
the run's target runner). A test over either sits in
`.config/gate-budgets.toml` with its figures; that list may only shrink
against `--base` (default `master`), and an entry for a test that is gone
or ignored fails as stale. The whole run is reported against 600 s and
fails past it. `CARGO_NEXTEST` names the nextest binary (default
`cargo-nextest`, pinned 0.9.145 as CI's).
"""
import argparse
import json
import os
import subprocess
import sys
import time
import tomllib

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
BUDGETS = ".config/gate-budgets.toml"
TOTAL = 600


def nextest(*args: str) -> list[str]:
    binary = os.environ.get("CARGO_NEXTEST", "cargo-nextest")
    return [binary, "nextest", *args, "--cargo-profile", "ci", "--workspace", "--profile", "gate"]


def entries(text: str) -> dict[str, dict]:
    return {t["name"]: t for t in tomllib.loads(text).get("test", [])}


def shrinks_only(base: str) -> list[str]:
    """The allowlist against its copy on `base`: no new entry, no figure raised."""
    ref = subprocess.run(["git", "rev-parse", "--verify", f"{base}^{{commit}}"],
                         cwd=ROOT, capture_output=True, text=True)
    if ref.returncode != 0:
        sys.exit(f"gate: the base {base!r} does not resolve: {ref.stderr.strip()}")
    now = entries(open(os.path.join(ROOT, BUDGETS)).read())
    old = subprocess.run(["git", "show", f"{base}:{BUDGETS}"], cwd=ROOT,
                         capture_output=True, text=True)
    if old.returncode != 0:
        print(f"gate: {BUDGETS} is not on {base}; this branch sets its baseline")
        return []
    was = entries(old.stdout)
    errors = [f"{name}: new on the allowlist" for name in now if name not in was]
    for name, t in now.items():
        for figure in ("seconds", "mib"):
            if name in was and t[figure] > was[name][figure]:
                errors.append(f"{name}: {figure} raised from {was[name][figure]} to {t[figure]}")
    return errors


def stale(listed: dict[str, dict]) -> list[str]:
    """Allowlisted tests the gate no longer runs."""
    out = subprocess.run(nextest("list", "--message-format", "json"), cwd=ROOT,
                         capture_output=True, text=True, check=True).stdout
    runs = set()
    for suite in json.loads(out)["rust-suites"].values():
        binary = suite["binary-name"]
        for name, case in suite["testcases"].items():
            if not case["ignored"]:
                runs.add(f"{binary} {name}")
    return [f"{name}: stale, the gate runs no such test" for name in listed if name not in runs]


def timeouts(listed: dict[str, dict]) -> str:
    """The allowlisted tests' own slow-timeouts, as a nextest tool config."""
    path = os.path.join(ROOT, "target", "gate", "budgets.toml")
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w") as f:
        for name, t in listed.items():
            binary, test = name.split(" ", 1)
            f.write("[[profile.gate.overrides]]\n")
            f.write(f"filter = '''binary({binary}) & test(={test})'''\n")
            f.write(f'slow-timeout = {{ period = "{t["seconds"]}s", terminate-after = 1 }}\n\n')
    return path


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--base", default="master")
    base = parser.parse_args().base
    listed = entries(open(os.path.join(ROOT, BUDGETS)).read())
    errors = shrinks_only(base) + stale(listed)
    if errors:
        print("gate: the budget allowlist\n  " + "\n  ".join(errors), file=sys.stderr)
        return 1
    env = dict(os.environ)
    env["CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUNNER"] = os.path.join(ROOT, "scripts", "gate_test.py")
    start = time.monotonic()
    run = subprocess.run(nextest("run", "--no-fail-fast", "--tool-config-file",
                                 f"gate:{timeouts(listed)}"), cwd=ROOT, env=env)
    total = time.monotonic() - start
    print(f"gate: {total:.0f} s against {TOTAL} s")
    if run.returncode == 0 and total > TOTAL:
        print(f"gate: over ten minutes by {total - TOTAL:.0f} s", file=sys.stderr)
        return 1
    return run.returncode


if __name__ == "__main__":
    sys.exit(main())
