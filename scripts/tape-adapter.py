#!/usr/bin/env python3
"""Record and replay one vision adapter call (fn-149, owner 2026-09-25).

The species runner wraps every adapter program in its configs as
`python3 scripts/tape-adapter.py record:<dir>|replay:<dir> -- <program> <args...>`.
The call's stdin envelope and the adapter's argv are the key, without what
names the run rather than the question: every `path`, the request hash the
caller took over those paths, every `ledger` reference (a fresh entry id per
call) and every run `identity` (a hash of a config that holds paths). Recording runs the adapter and stores its stdout, stderr
and exit status; replaying prints the stored stdout, bound to this call's
own request hash, and exits with the stored status, and fails, naming the
request, when the recording lacks it. Nothing here reaches a model.
"""
import hashlib
import json
from pathlib import Path
import subprocess
import sys


# What names the run rather than the question. Kept in step with
# `crate::tape::UNSTABLE`.
UNSTABLE = ("path", "request_sha256", "ledger", "identity", "run_identity",
            "current_identity", "render_identity")


def stable(value):
    if isinstance(value, dict):
        return {k: stable(v) for k, v in value.items() if k not in UNSTABLE}
    if isinstance(value, list):
        return [stable(v) for v in value]
    return value


def key(argv, stdin):
    try:
        envelope = stable(json.loads(stdin))
    except json.JSONDecodeError:
        envelope = stdin
    # A script named by path is named by its file: a checkout elsewhere asks
    # the same question.
    named = [a.rsplit("/", 1)[-1] for a in argv]
    canonical = json.dumps({"argv": named, "stdin": envelope}, sort_keys=True, separators=(",", ":"))
    return hashlib.sha256(canonical.encode()).hexdigest()


def bound(stdout, stdin):
    """The recorded answer bound to this call's own request hash, the one
    thing in it that named the recording's run directory."""
    try:
        answer, envelope = json.loads(stdout), json.loads(stdin)
    except json.JSONDecodeError:
        return stdout
    if isinstance(answer, dict) and "request_sha256" in answer and "request_sha256" in envelope:
        answer["request_sha256"] = envelope["request_sha256"]
        return json.dumps(answer) + "\n"
    return stdout


def main(argv):
    mode, rest = argv[0], argv[1:]
    if not rest or rest[0] != "--" or ":" not in mode:
        raise SystemExit("usage: tape-adapter.py record:<dir>|replay:<dir> -- <program> <args...>")
    kind, directory = mode.split(":", 1)
    program = rest[1:]
    stdin = sys.stdin.read()
    digest = key(program, stdin)
    entry = Path(directory) / "adapter" / f"{digest[:32]}.json"
    if kind == "replay":
        if not entry.exists():
            envelope = stdin[:300].replace("\n", " ")
            sys.stderr.write(f"replay: {directory} holds no adapter answer for {' '.join(program)} <<< {envelope} (key {digest})\n")
            return 3
        recorded = json.loads(entry.read_text())
        sys.stdout.write(bound(recorded["stdout"], stdin))
        sys.stderr.write(recorded["stderr"])
        return recorded["exit"]
    done = subprocess.run(program, input=stdin, text=True, capture_output=True)
    entry.parent.mkdir(parents=True, exist_ok=True)
    entry.write_text(json.dumps({"key": digest, "argv": program, "stdin": json.loads(stdin) if stdin.strip().startswith("{") else stdin,
                                 "stdout": done.stdout, "stderr": done.stderr, "exit": done.returncode}, indent=1, sort_keys=True))
    sys.stdout.write(done.stdout)
    sys.stderr.write(done.stderr)
    return done.returncode


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
