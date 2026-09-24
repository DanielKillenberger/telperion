#!/usr/bin/env python3
"""Claude twin of reference-first-codex.py.

Imports the codex script for its `prepare` (same allowlist, image handling
and prompt/schema construction, unchanged) and reuses its cardinality rule
verbatim; only the model call differs, going through vision_claude.run
(Claude) instead of `codex exec`.
"""
import argparse
import hashlib
import importlib.util
import json
import sys
from pathlib import Path

import vision_claude

_spec = importlib.util.spec_from_file_location("reference_first_codex", Path(__file__).with_name("reference-first-codex.py"))
_codex = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(_codex)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--model", required=True)
    parser.add_argument("--effort", required=True)
    args = parser.parse_args()
    envelope = json.load(sys.stdin)
    paths, schema, prompt = _codex.prepare(envelope)
    result = vision_claude.run(args.model, args.effort, paths, prompt, schema, timeout=300)
    answer = result["answer"]
    valid_count = (envelope["stage"] not in ("comparison", "repair")
                   or (isinstance(answer, dict) and len(answer.get("passes", [])) == len(envelope["request"]["comparison"]["required"])))
    model_identity_basis = (f"claude CLI result event modelUsage key: {result['actual_model']}"
                             if result["actual_model"] else "requested command argument; actual resolved identity not exposed")
    print(json.dumps({"request_sha256": envelope["request_sha256"], "prompt_sha256": envelope["prompt_sha256"],
        "dispatched_prompt_sha256": hashlib.sha256(prompt.encode()).hexdigest(), "schema_sha256": hashlib.sha256(json.dumps(schema).encode()).hexdigest(),
        "model": args.model, "model_identity_basis": model_identity_basis, "effort": args.effort,
        "status": "ok" if result["returncode"] == 0 and not result["forbidden_tools"] and result["usage"] is not None and valid_count else "failed_or_tools_or_unknown_usage_or_cardinality",
        "forbidden_tools": result["forbidden_tools"], "usage": result["usage"], "answer": answer,
        "raw_events": result["raw_events"], "stderr": result["stderr"], "image_sha256": [hashlib.sha256(Path(p).read_bytes()).hexdigest() for p in paths]}))


if __name__ == "__main__":
    main()
