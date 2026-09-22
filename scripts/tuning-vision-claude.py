#!/usr/bin/env python3
"""Claude twin of tuning-vision-codex.py.

tuning-vision-codex.py has no separable `prepare`: it is one procedural
script with a single subprocess.run call, checked and exercised as a whole
by test-tuning-vision-codex.py via runpy. This twin runs that exact script
unmodified via runpy, intercepting only its one subprocess.run call so the
dispatch goes through vision_claude.run (Claude) instead of `codex exec`;
every path check, schema, prompt and cardinality rule is the literal codex
script executing. Because vision_claude.run also calls subprocess.run
internally (this time for the real `claude` command), the interception
nests: while our fake stands in for the outer (codex-shaped) call, it
temporarily restores whatever subprocess.run resolved to before we patched
it -- the real CLI in production, a test's mock in a test -- for the
duration of the inner vision_claude.run call.

The codex script's own stdout is captured and republished with one field
added: model_identity_basis, naming the actual model the Claude events
reported.
"""
import argparse
import contextlib
import io
import json
import runpy
import subprocess
from pathlib import Path
from unittest.mock import patch

import vision_claude

CODEX_SCRIPT = Path(__file__).with_name("tuning-vision-codex.py")


def _fake_run(model, effort, real_run):
    def run(command, **kwargs):
        image_paths = [Path(command[i + 1]) for i, arg in enumerate(command) if arg == "--image"]
        prompt = command[-1]
        schema = json.loads(Path(command[command.index("--output-schema") + 1]).read_text())
        out_path = Path(command[command.index("-o") + 1])
        timeout = kwargs.get("timeout", 600)
        with patch("subprocess.run", side_effect=real_run):
            dispatch = vision_claude.run(model, effort, image_paths, prompt, schema, timeout=timeout)
        returncode = dispatch["returncode"]
        if dispatch["answer"] is None and returncode == 0:
            returncode = 1
        out_path.write_text(json.dumps(dispatch["answer"] if dispatch["answer"] is not None else {}))
        events = []
        if dispatch["usage"] is not None:
            events.append(json.dumps({"type": "turn.completed", "usage": dispatch["usage"]}))
        stdout = ("\n".join(events) + "\n").encode() if events else b""
        run.identity = dispatch["actual_model"]
        return subprocess.CompletedProcess(command, returncode, stdout, dispatch["stderr"].encode())
    run.identity = None
    return run


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--model", required=True)
    parser.add_argument("--effort", required=True)
    args = parser.parse_args()
    real_run = subprocess.run
    fake = _fake_run(args.model, args.effort, real_run)
    captured = io.StringIO()
    with patch("subprocess.run", side_effect=fake), contextlib.redirect_stdout(captured):
        runpy.run_path(str(CODEX_SCRIPT), run_name="__main__")
    result = json.loads(captured.getvalue())
    result["model_identity_basis"] = (f"claude CLI result event modelUsage key: {fake.identity}"
                                       if fake.identity else "requested command argument; actual resolved identity not exposed")
    print(json.dumps(result))


if __name__ == "__main__":
    main()
