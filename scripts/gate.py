#!/usr/bin/env python3
"""The local gate (fn-304): the workspace's tests under nextest's `gate`
profile, inside its budgets, running only what a change can affect (fn-322).

  telperion-heavy 8 --threads 4 -- python3 scripts/gate.py [--base <ref>] [--full]

Each of `scripts/ci-key.mjs`'s suites keeps a receipt under its key in
`target/gate/receipts/`, written only after the suite passes. A suite whose key
has a receipt is skipped and printed with the key; every other suite runs. The
keys read the commit, so a dirty tree is refused, by name, except for `.flow/`
files no key reads. `--full` ignores the receipts, runs every suite and writes
them again. After a failure the failed tests rerun first, through nextest's
recording (`.config/nextest-user.toml`), and the suites only if those pass.

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
binary (default `cargo-nextest`); it must be 0.9.145, CI's pin, and the
gate stops first, naming the install command, when it is missing or another.

Two of CI's checks run here too, their commands copied from
`.github/workflows/tests.yml` (fn-314: both failed on the release branch's
PR while the gate passed). The package check (`npm run build`, then
`node scripts/artifact-budgets.mjs`, which holds the ceilings only, with no
PR to read growth from) costs 37 s after a change to the core crate's source
and 2 s warm, so it runs before the test run. The core without geometry
(`cargo build` and `cargo test --lib` with `--no-default-features`) costs
220 s after that change and 139 s warm, nearly all of it its lib tests, so it
runs after the test run, outside `TOTAL`, and also when the test run failed.
Measured 2026-10-24 at four threads, beside another checkout's gate. A check
that fails, or cannot start, fails the gate with its own output.
"""
import argparse
import hashlib
import json
import os
import subprocess
import sys
import time
import tomllib
import xml.etree.ElementTree as ET

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
BUDGETS = ".config/gate-budgets.toml"
# 1.25 x the declared median (AGENTS.md), rounded up to the minute.
TOTAL = 1560


NEXTEST = "0.9.145"
INSTALL = f"cargo install cargo-nextest --version {NEXTEST} --locked"


def binary() -> str:
    return os.environ.get("CARGO_NEXTEST", "cargo-nextest")


def nextest(*args: str, packages: tuple[str, ...] = ()) -> list[str]:
    scope = [w for p in packages for w in ("-p", p)] or ["--workspace"]
    return [binary(), "nextest", *args, "--cargo-profile", "ci", *scope, "--profile", "gate"]


def nextest_at_its_pin() -> str | None:
    """Why the gate's nextest is not CI's pin, in one line; None when it is."""
    try:
        out = subprocess.run([binary(), "nextest", "--version"], capture_output=True, text=True)
    except OSError:
        return f"gate: no cargo-nextest at {binary()!r}; install it: {INSTALL}"
    words = out.stdout.split()
    if out.returncode != 0 or len(words) < 2 or words[1] != NEXTEST:
        found = words[1] if len(words) > 1 else "an unreadable version"
        return f"gate: cargo-nextest {found}, not {NEXTEST}; install it: {INSTALL}"
    return None


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


def stale(listed: dict[str, dict]) -> tuple[list[str], dict[str, set[str]]]:
    """Allowlisted tests the gate no longer runs and any workspace package no
    suite would run, with each package's tests as its JUnit report names them."""
    out = subprocess.run(nextest("list", "--message-format", "json"), cwd=ROOT,
                         capture_output=True, text=True, check=True).stdout
    runs = set()
    errors = []
    expected: dict[str, set[str]] = {}
    covered = {package for packages in RUST.values() for package in packages}
    for suite in json.loads(out)["rust-suites"].values():
        binary = suite["binary-name"]
        package = suite["package-name"]
        if package not in covered:
            errors.append(f"{package}: in no suite, so the gate would not run it")
        for name, case in suite["testcases"].items():
            if not case["ignored"]:
                runs.add(f"{binary} {name}")
                expected.setdefault(package, set()).add(f"{suite['binary-id']} {name}")
    errors = sorted(set(errors))
    return errors + [f"{name}: stale, the gate runs no such test" for name in listed if name not in runs], expected


RUSTDOC_RUNS = {"", "rust", "should_panic", "no_run", "compile_fail"}


def doc_lines(path: str):
    """Each line of `path`'s doc comments, `///` and `//!` lines and the
    lines of `/** */` and `/*! */` blocks, with its number."""
    block = False
    for number, line in enumerate(open(path, encoding="utf-8"), 1):
        text = line.strip()
        if block:
            yield number, text
            block = "*/" not in text
        elif text.startswith(("///", "//!")):
            yield number, text
        elif text.startswith(("/**", "/*!")) and not text.startswith("/**/"):
            yield number, text
            block = "*/" not in text[3:]


def doc_tests() -> list[str]:
    """Doc comments that open a code fence rustdoc would test. nextest runs no
    doc-tests and the workspace has none (fn-304), so one added fails here."""
    found = []
    for top, _, files in os.walk(os.path.join(ROOT, "crates")):
        for name in (f for f in files if f.endswith(".rs")):
            path = os.path.join(top, name)
            inside = False
            for number, text in doc_lines(path):
                if "```" not in text:
                    continue
                if not inside:
                    langs = {t.strip() for t in text.split("```", 1)[1].split(",")}
                    if langs & RUSTDOC_RUNS or any(t.startswith("edition") for t in langs):
                        found.append(f"{os.path.relpath(path, ROOT)}:{number}: a doc-test "
                                     "the gate would not run; mark its fence `text`")
                inside = not inside
    return found


# `scripts/ci-key.mjs`'s Rust suites, with the packages
# `.github/workflows/tests.yml` runs under each. They share one nextest pool:
# run apart they took 1,649 s against the pool's 1,241 s median (measured
# 2026-11-01), so the pool's JUnit report says which suites came out green.
RUST = {
    "rust-jev": ("telperion-jev",),
    "rust-wasm": ("telperion-wasm", "telperion-space"),
    "rust-render": ("telperion-render",),
    "rust-core": ("telperion-core",),
}
RECEIPTS = os.path.join(ROOT, "target", "gate", "receipts")
FAILED_RUN = os.path.join(ROOT, "target", "gate", "failed-run")
JUNIT = os.path.join(ROOT, "target", "nextest", "gate", "junit.xml")
USER_CONFIG = os.path.join(ROOT, ".config", "nextest-user.toml")
LOCAL_INPUTS = (BUDGETS, "scripts/gate_test.py")
# nextest's exit code for a run with failing tests. Any other code from a rerun
# of the failed set (a pruned recording, no such test) falls through to the
# suites; from the suites themselves it means no report to trust, so no receipt.
TESTS_FAILED = 100

# CI's checks, as `.github/workflows/tests.yml` runs them: (job name, commands).
NODE = ("node", [["npm", "test"], ["npm", "run", "typecheck"]])
PACKAGE = ("package", [["npm", "run", "build"], ["node", "scripts/artifact-budgets.mjs"]])
NO_GEOMETRY = ("core without geometry", [
    ["cargo", "build", "--profile", "ci", "-p", "telperion-core", "--no-default-features"],
    ["cargo", "test", "--profile", "ci", "-p", "telperion-core", "--no-default-features", "--lib"],
])


def ci_check(name: str, commands: list[list[str]]) -> list[str]:
    """Run one of CI's checks with its output streamed; its failure, if any."""
    start = time.monotonic()
    error = None
    for argv in commands:
        try:
            code = subprocess.run(argv, cwd=ROOT).returncode
        except OSError as e:
            error = f"{name}: `{' '.join(argv)}` cannot start: {e}"
            break
        if code != 0:
            error = f"{name}: `{' '.join(argv)}` exited {code}"
            break
    print(f"gate: {name} {time.monotonic() - start:.0f} s")
    return [error] if error else []


def timeouts(listed: dict[str, dict]) -> str:
    """The allowlisted tests' own slow-timeouts, as a nextest tool config."""
    path = os.path.join(ROOT, "target", "gate", "budgets.toml")
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w") as f:
        f.write('[profile.gate.junit]\npath = "junit.xml"\n\n')
        for name, t in listed.items():
            binary, test = name.split(" ", 1)
            f.write("[[profile.gate.overrides]]\n")
            f.write(f"filter = '''binary({binary}) & test(={test})'''\n")
            f.write(f'slow-timeout = {{ period = "{t["seconds"]}s", terminate-after = 1 }}\n\n')
    return path


def git(*args: str) -> str:
    return subprocess.run(["git", *args], cwd=ROOT, capture_output=True, text=True, check=True).stdout


def node(*args: str) -> str:
    return subprocess.run(["node", *args], cwd=ROOT, capture_output=True, text=True, check=True).stdout.strip()


def flow_inputs() -> tuple[str, ...]:
    """The paths under `.flow/` the keys read."""
    script = ("import('./scripts/ci-key.mjs').then(({ SUITES }) => console.log("
              "Object.values(SUITES).flatMap(s => s.inputs).join('\\n')))")
    return tuple(p for p in node("-e", script).splitlines() if p.startswith(".flow/"))


def dirty() -> list[str]:
    """What a key built from the commit would not see."""
    read = flow_inputs()
    found = []
    for line in git("status", "--porcelain", "--untracked-files=all", "--no-renames").splitlines():
        path = line[3:].strip('"')
        if not path.startswith(".flow/") or path in read:
            found.append(path)
    return found


def has_receipt(suite: str, key: str) -> bool:
    return os.path.exists(os.path.join(RECEIPTS, f"{suite}.{key}"))


def remove_receipt(suite: str, key: str) -> None:
    if has_receipt(suite, key):
        os.remove(os.path.join(RECEIPTS, f"{suite}.{key}"))


def write_receipt(suite: str, key: str) -> None:
    os.makedirs(RECEIPTS, exist_ok=True)
    path = os.path.join(RECEIPTS, f"{suite}.{key}")
    with open(f"{path}.tmp", "w") as f:
        f.write(git("rev-parse", "HEAD"))
    os.replace(f"{path}.tmp", path)


def recorded_env() -> dict[str, str]:
    env = dict(os.environ)
    env["CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUNNER"] = os.path.join(ROOT, "scripts", "gate_test.py")
    env["NEXTEST_EXPERIMENTAL_RECORD"] = "1"
    # A driver cache at its 1 GB default keeps no new shader, so after a shader
    # change each fresh context recompiles: 166 s against 7 s for one test (fn-320).
    env.setdefault("__GL_SHADER_DISK_CACHE_SIZE", str(4 << 30))
    return env


def latest_run() -> str | None:
    """The id of the run nextest recorded last."""
    out = subprocess.run([binary(), "nextest", "store", "list", "--user-config-file", USER_CONFIG],
                         cwd=ROOT, capture_output=True, text=True, env=recorded_env()).stdout
    for line in out.splitlines():
        if "*latest" in line:
            return line.split()[0]
    return None


def green_suites(suites: list[str], expected: dict[str, set[str]]) -> list[str]:
    """The suites whose every listed test reported, none failed. nextest also
    exits 100 on a cancelled run, whose report holds only the tests that ended."""
    reported: set[str] = set()
    red: set[str] = set()
    for suite in ET.parse(JUNIT).getroot().iter("testsuite"):
        package = suite.get("name", "").split("::")[0]
        if int(suite.get("failures", "1")) or int(suite.get("errors", "1")):
            red.add(package)
        reported |= {f"{case.get('classname')} {case.get('name')}" for case in suite.iter("testcase")}
    def ran_whole(package: str) -> bool:
        return bool(expected.get(package)) and expected[package] <= reported and package not in red
    return [s for s in suites if all(ran_whole(p) for p in RUST[s])]


def run_rust(suites: list[str], budgets: str, expected: dict[str, set[str]]) -> tuple[list[str], float]:
    """The suites in one pool, the last failed set first: the suites that came
    out green, and the seconds of the pool's run."""
    packages = tuple(p for s in suites for p in RUST[s])
    common = ["--no-fail-fast", "--user-config-file", USER_CONFIG, "--tool-config-file", f"gate:{budgets}"]
    if os.path.exists(FAILED_RUN):
        run_id = open(FAILED_RUN).read().strip()
        print(f"gate: rerunning the failed set of run {run_id} first")
        code = subprocess.run(nextest("run", "-R", run_id, *common, packages=packages),
                              cwd=ROOT, env=recorded_env()).returncode
        if code == TESTS_FAILED:
            return [], 0.0
    if os.path.exists(JUNIT):
        os.remove(JUNIT)
    start = time.monotonic()
    code = subprocess.run(nextest("run", *common, packages=packages), cwd=ROOT, env=recorded_env()).returncode
    seconds = time.monotonic() - start
    if code == 0 and os.path.exists(FAILED_RUN):
        os.remove(FAILED_RUN)
    if code == TESTS_FAILED:
        run_id = latest_run()
        if run_id:
            with open(FAILED_RUN, "w") as f:
                f.write(run_id)
    return (green_suites(suites, expected) if code in (0, TESTS_FAILED) else []), seconds


def local_key(key: str) -> str:
    """A Rust suite's key under the limits this gate holds it to: CI's key, and
    the budgets and runner CI does not read."""
    objects = git("rev-parse", *[f"HEAD:{path}" for path in LOCAL_INPUTS])
    return f"{key}-{hashlib.sha256(objects.encode()).hexdigest()[:12]}"


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--base", default="master")
    parser.add_argument("--full", action="store_true", help="ignore the receipts and run every suite")
    args = parser.parse_args()
    found = dirty()
    if found:
        print("gate: the keys read the commit; commit or remove first:\n  " + "\n  ".join(found), file=sys.stderr)
        return 1
    keys = {suite: local_key(node("scripts/ci-key.mjs", suite)) for suite in RUST}
    keys["node"] = keys["package"] = node("scripts/ci-key.mjs", "node")
    running = []
    for suite, key in keys.items():
        if args.full:
            # A run that fails must not leave an earlier green receipt behind it.
            remove_receipt(suite, key)
        if has_receipt(suite, key):
            print(f"gate: {suite} skipped, green receipt {key}")
        else:
            print(f"gate: {suite} runs, key {key}")
            running.append(suite)
    rust = [suite for suite in RUST if suite in running]
    listed = entries(open(os.path.join(ROOT, BUDGETS)).read())
    errors = shrinks_only(args.base) + doc_tests()
    if rust:
        missing = nextest_at_its_pin()
        if missing:
            print(missing, file=sys.stderr)
            return 1
        before = time.monotonic()
        problems, expected = stale(listed)
        errors += problems
        print(f"gate: build and listing {time.monotonic() - before:.0f} s")
    if "package" in running and not errors:
        package = ci_check(*PACKAGE)
        errors += package
        if not package:
            write_receipt("package", keys["package"])
    if errors:
        print("gate: before the run\n  " + "\n  ".join(errors), file=sys.stderr)
        return 1
    failed = []
    if rust:
        green, total = run_rust(rust, timeouts(listed), expected)
        print(f"gate: the test run {total:.0f} s against {TOTAL} s")
        after = ci_check(*NO_GEOMETRY) if "rust-core" in rust else []
        if total > TOTAL:
            print(f"gate: {total - TOTAL:.0f} s over its {TOTAL} s budget", file=sys.stderr)
            failed.append("the test run's budget")
            green = []
        for suite in rust:
            if suite in green and not (suite == "rust-core" and after):
                write_receipt(suite, keys[suite])
            else:
                failed.append(suite)
    if "node" in running:
        after = ci_check(*NODE)
        if after:
            failed.append("node")
        else:
            write_receipt("node", keys["node"])
    if failed:
        print("gate: failed: " + ", ".join(failed), file=sys.stderr)
        return 1
    print("gate: green")
    return 0


if __name__ == "__main__":
    sys.exit(main())
