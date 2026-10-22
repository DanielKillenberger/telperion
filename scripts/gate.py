#!/usr/bin/env python3
"""The local gate (fn-304): the workspace's tests under nextest's `gate`
profile, inside its budgets.

  telperion-heavy 8 --threads 4 -- python3 scripts/gate.py [--base <ref>]

Budgets (host, fn-304 R4, ruled 2026-10-22): a test's own wall time is cut
off at 120 s (the profile's slow-timeout) and its peak memory at 4 GiB
(`gate_test.py`, the run's target runner). A test over either sits in
`.config/gate-budgets.toml` with its figures; that list may only shrink
against `--base` (default `master`), and an entry for a test that is gone
or ignored fails as stale. The test run fails past `TOTAL`, 1.25 times
the declared median rounded up to the minute, so growth the size of a new
sweep fails the change that adds it. Like the median, it times the test
run: the build and the listing before it are reported beside it, not held
to it, so a cold build does not fail the gate. `CARGO_NEXTEST` names the nextest
binary (default `cargo-nextest`, pinned 0.9.145 as CI's).
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
# 1.25 x the declared median (AGENTS.md), rounded up to the minute.
TOTAL = 1620


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


RUSTDOC_RUNS = {"", "rust", "should_panic", "no_run", "compile_fail"}


def doc_tests() -> list[str]:
    """Doc comments that open a code fence rustdoc would test. nextest runs no
    doc-tests and the workspace has none (fn-304), so one added fails here."""
    found = []
    for top, _, files in os.walk(os.path.join(ROOT, "crates")):
        for name in (f for f in files if f.endswith(".rs")):
            path = os.path.join(top, name)
            inside = False
            for number, line in enumerate(open(path, encoding="utf-8"), 1):
                text = line.strip()
                if not text.startswith(("///", "//!")) or "```" not in text:
                    continue
                if not inside:
                    langs = {t.strip() for t in text.split("```", 1)[1].split(",")}
                    if langs & RUSTDOC_RUNS or any(t.startswith("edition") for t in langs):
                        found.append(f"{os.path.relpath(path, ROOT)}:{number}: a doc-test "
                                     "the gate would not run; mark its fence `text`")
                inside = not inside
    return found


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
    before = time.monotonic()
    errors = shrinks_only(base) + stale(listed) + doc_tests()
    print(f"gate: build and listing {time.monotonic() - before:.0f} s")
    if errors:
        print("gate: before the run\n  " + "\n  ".join(errors), file=sys.stderr)
        return 1
    env = dict(os.environ)
    env["CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUNNER"] = os.path.join(ROOT, "scripts", "gate_test.py")
    start = time.monotonic()
    run = subprocess.run(nextest("run", "--no-fail-fast", "--tool-config-file",
                                 f"gate:{timeouts(listed)}"), cwd=ROOT, env=env)
    total = time.monotonic() - start
    print(f"gate: the test run {total:.0f} s against {TOTAL} s")
    if run.returncode == 0 and total > TOTAL:
        print(f"gate: {total - TOTAL:.0f} s over its {TOTAL} s budget", file=sys.stderr)
        return 1
    return run.returncode


if __name__ == "__main__":
    sys.exit(main())
