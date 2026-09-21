#!/usr/bin/env python3
"""Uncalibrated progress-review adapter: one isolated side-by-side call.

The envelope names no candidate and no round. Which render is the newer one is
decided and recorded by the caller, never sent here and never inferred.
"""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import sys
import tempfile


def object_schema(properties):
    return {"type": "object", "additionalProperties": False, "properties": properties, "required": list(properties)}


def strings():
    return {"type": "array", "items": {"type": "string"}}


def prepare(envelope):
    if envelope["stage"] != "progress":
        raise ValueError("unknown stage")
    request = envelope["request"]
    if set(request) != {"schema", "target_species", "view", "seed", "references", "a", "b", "priorities", "owner_notes"}:
        raise ValueError("progress allowlist violation")
    if request["schema"] != "tuning-progress-v1":
        raise ValueError("unknown progress schema")
    ids = [p["id"] for p in request["priorities"]]
    if not ids or len(ids) != len(set(ids)):
        raise ValueError("invalid priority list")
    # Metadata order: the references for this view, then render A, then render B.
    images = request["references"] + [request["a"], request["b"]]
    if len(images) > 12:
        raise ValueError("invalid image count")
    paths = []
    for image in images:
        p = Path(image["path"]).resolve()
        if hashlib.sha256(p.read_bytes()).hexdigest() != image["sha256"]:
            raise ValueError("image hash mismatch")
        paths.append(p)
    if hashlib.sha256(envelope["prompt"].encode()).hexdigest() != envelope["prompt_sha256"]:
        raise ValueError("prompt hash mismatch")
    schema = object_schema({
        "verdicts": {"type": "array", "minItems": len(ids), "maxItems": len(ids),
                     "items": object_schema({"priority_id": {"type": "string", "enum": ids},
                                             "verdict": {"type": "string", "enum": ["a_better", "b_better", "same", "unknown"]}})},
        "improved": {"type": "string"}, "missing": {"type": "string"}, "regressions": strings()})
    prompt = (envelope["prompt"]
              + "\nDo not use tools or inspect files. Attached images follow metadata order: the reference photographs, then render A, then render B. Return JSON only.\n"
              + json.dumps(request)
              + "\nReturn exactly one verdict for each listed priority id, in that order.")
    return paths, schema, prompt


def main():
    p = argparse.ArgumentParser()
    p.add_argument("--model", required=True)
    p.add_argument("--effort", required=True)
    args = p.parse_args()
    envelope = json.load(sys.stdin)
    paths, schema, prompt = prepare(envelope)
    ids = [q["id"] for q in envelope["request"]["priorities"]]
    with tempfile.TemporaryDirectory(prefix="progress-review-") as scratch:
        out, schema_path = Path(scratch) / "answer.json", Path(scratch) / "schema.json"
        schema_path.write_text(json.dumps(schema))
        command = ["codex", "exec", "--ignore-user-config", "--ephemeral", "--skip-git-repo-check", "--sandbox", "read-only", "-C", scratch,
                   "-m", args.model, "-c", f'model_reasoning_effort="{args.effort}"', "--json", "--output-schema", str(schema_path), "-o", str(out)]
        for image in paths:
            command.extend(["--image", str(image)])
        command.extend(["--", prompt])
        run = subprocess.run(command, stdin=subprocess.DEVNULL, capture_output=True, timeout=600)
        usage = None
        forbidden_tools = []
        for line in run.stdout.splitlines():
            event = json.loads(line)
            item = event.get("item", {})
            if item.get("type") in ("command_execution", "mcp_tool_call", "web_search", "collab_tool_call", "file_change"):
                forbidden_tools.append(item.get("type"))
            if event.get("type") == "turn.completed":
                if usage is not None:
                    raise ValueError("multiple model turns")
                usage = event.get("usage")
        if not usage or any(type(usage.get(k)) is not int or usage[k] < 0 for k in ("input_tokens", "output_tokens")):
            usage = None
        answer = json.loads(out.read_text()) if out.exists() else None
        answered = isinstance(answer, dict) and sorted(v.get("priority_id") for v in answer.get("verdicts", [])) == sorted(ids)
        print(json.dumps({"request_sha256": envelope["request_sha256"], "prompt_sha256": envelope["prompt_sha256"],
            "dispatched_prompt_sha256": hashlib.sha256(prompt.encode()).hexdigest(), "schema_sha256": hashlib.sha256(json.dumps(schema).encode()).hexdigest(),
            "model": args.model, "model_identity_basis": "requested command argument; actual resolved identity not exposed", "effort": args.effort,
            "status": "ok" if run.returncode == 0 and not forbidden_tools and usage is not None and answered else "failed_or_tools_or_unknown_usage_or_cardinality",
            "forbidden_tools": forbidden_tools, "usage": usage, "answer": answer,
            "uncalibrated": "comparative progress verdict; no replay qualifies this question",
            "raw_events": run.stdout.decode(), "stderr": run.stderr.decode(), "image_sha256": [hashlib.sha256(p.read_bytes()).hexdigest() for p in paths]}))


if __name__ == "__main__":
    main()
