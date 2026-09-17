#!/usr/bin/env python3
"""Minimal Jev caller for experiments. Reads a request JSON file, prints answers.

usage: bash -ic 'python3 jev.py request.json [--raw]'
The key is read from TYPESAFE_API_KEY and never printed.
"""
import json
import os
import sys
import time
import urllib.error
import urllib.request

URL = "https://api.typesafe.ai/v1/systemone"


def call(payload: dict) -> dict:
    key = os.environ.get("TYPESAFE_API_KEY")
    if not key:
        sys.exit("TYPESAFE_API_KEY not set (run through bash -ic)")
    payload.setdefault("model", "jev-latest")
    body = json.dumps(payload).encode()
    req = urllib.request.Request(
        URL,
        data=body,
        headers={"Authorization": f"Bearer {key}", "Content-Type": "application/json"},
        method="POST",
    )
    for attempt in range(4):
        try:
            with urllib.request.urlopen(req, timeout=60) as r:
                return json.load(r)
        except urllib.error.HTTPError as e:
            text = e.read().decode(errors="ignore")
            if e.code in (429, 529) and attempt < 3:
                time.sleep(2 ** attempt)
                continue
            sys.exit(f"HTTP {e.code}: {text[:800]}")
    raise RuntimeError("unreachable")


def summarize(ans: dict) -> str:
    t = ans.get("type")
    if t == "noul":
        return f"noul p={ans['noul']:.3f}"
    if t == "choice":
        probs = sorted(ans.get("probabilities", {}).items(), key=lambda kv: -kv[1])
        top = ", ".join(f"{k}={v:.2f}" for k, v in probs[:4])
        return f"choice={ans['choice']} conf={ans.get('confidence', 0):.2f} [{top}]"
    if t == "score":
        probs = ans.get("probabilities", {})
        top = ", ".join(f"{k}={v:.2f}" for k, v in sorted(probs.items(), key=lambda kv: -kv[1])[:3])
        return f"score={ans['score']:.2f} conf={ans.get('confidence', 0):.2f} [{top}]"
    return json.dumps(ans)


def main() -> None:
    path = sys.argv[1]
    raw = "--raw" in sys.argv
    with open(path) as f:
        spec = json.load(f)
    # spec may be a single request or a list of {"label":..., "request":...}
    cases = spec if isinstance(spec, list) else [{"label": path, "request": spec}]
    for case in cases:
        t0 = time.time()
        out = call(case["request"])
        dt = time.time() - t0
        u = out.get("usage", {})
        print(f"## {case['label']}  ({dt:.2f}s, in={u.get('input_tokens')} out={u.get('output_tokens')})")
        if "expect" in case:
            print(f"   expect: {case['expect']}")
        for qid, ans in out.get("answers", {}).items():
            print(f"   {qid}: {summarize(ans)}")
        if raw:
            print(json.dumps(out, indent=1))


if __name__ == "__main__":
    main()
