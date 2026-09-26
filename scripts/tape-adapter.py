#!/usr/bin/env python3
"""Record and replay one vision adapter call (fn-149, owner 2026-09-25).

The species runner wraps every adapter program in its configs as
`python3 scripts/tape-adapter.py record:<dir>|replay:<dir>|extend:<dir> -- <program> <args...>`;
`extend` replays what the recording holds and records what it lacks;
`tape-adapter.py rekey:<dir>` refiles a recording's answers under the keys
this script computes now.
The call's stdin envelope and the adapter's argv are the key, without what
names the run rather than the question: every `path`, the request hash the
caller took over those paths, every `ledger` reference (a fresh entry id per
call), every run `identity` (a hash of a config that holds paths), a render's
`geometry_group` (that identity and a seed) and a `shot_source` (a hash of a
references file holding the run's paths and dates). Recording runs the adapter and stores its stdout, stderr
and exit status; replaying prints the stored stdout, bound to this call's
own request hash, and exits with the stored status, and fails, naming the
request, when the recording lacks it. Nothing here reaches a model.
"""
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys


# What names the run rather than the question. Kept in step with
# `crate::tape::UNSTABLE`.
UNSTABLE = ("path", "request_sha256", "ledger", "identity", "run_identity",
            "current_identity", "render_identity", "geometry_group", "shot_source")


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


def rekey(directory):
    """Files every recorded answer under the key this script computes for
    it now, so a recording follows a change of what names the run; of two
    answers that become one question, the newer stays."""
    moved = 0
    # Oldest first: when two answers become one question, the newer is the
    # run's and is written last.
    entries = sorted((Path(directory) / "adapter").glob("*.json"), key=lambda p: (p.stat().st_mtime, p.name))
    for entry in entries:
        if not entry.exists():
            continue
        recorded = json.loads(entry.read_text())
        stdin = recorded["stdin"]
        digest = key(recorded["argv"], stdin if isinstance(stdin, str) else json.dumps(stdin))
        target = entry.with_name(f"{digest[:32]}.json")
        filed = json.loads(target.read_text()) if target != entry and target.exists() else None
        if filed is not None and filed.get("key") == digest and target.stat().st_mtime >= entry.stat().st_mtime:
            # The answer already filed under its right key is the newer.
            entry.unlink()
            moved += 1
            continue
        if target != entry or recorded["key"] != digest:
            recorded["key"] = digest
            when = entry.stat().st_mtime
            entry.unlink()
            target.write_text(json.dumps(recorded, indent=1, sort_keys=True))
            os.utime(target, (when, when))  # its age decides a later collision
            moved += 1
    print(f"rekeyed {moved} adapter answers")
    return 0


def main(argv):
    if argv and argv[0].startswith("rekey:") and len(argv) == 1:
        return rekey(argv[0].split(":", 1)[1])
    mode, rest = argv[0], argv[1:]
    if not rest or rest[0] != "--" or ":" not in mode:
        raise SystemExit("usage: tape-adapter.py record:<dir>|replay:<dir>|extend:<dir> -- <program> <args...>")
    kind, directory = mode.split(":", 1)
    program = rest[1:]
    stdin = sys.stdin.read()
    digest = key(program, stdin)
    entry = Path(directory) / "adapter" / f"{digest[:32]}.json"
    if kind == "extend" and entry.exists():
        kind = "replay"
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
