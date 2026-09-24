#!/usr/bin/env python3
"""Unqualified reference-first adapter. One isolated image call; no retry or scheduler."""
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


# The comparison schema's version: v2 (fn-136) ties each finding and defect to
# an inventory trait, or null, and the request names the known gaps.
COMPARISON_VERSION = "reference-first-comparison-v2"


def trait_id():
    return {"type": ["string", "null"]}


def prepare(envelope):
    stage, request = envelope["stage"], envelope["request"]
    if stage == "inventory":
        if set(request) != {"protocol", "target_species", "references", "specimen_relationship"}:
            raise ValueError("reference-only allowlist violation")
        if request["specimen_relationship"] != "unknown":
            raise ValueError("unsupported specimen relationship")
        images = [r["image"] for r in request["references"]]
        # The receipt (reference_first.rs, Inventory::verify) holds at most 16
        # traits and 16 observations; the schema states the cap so the model
        # never returns an inventory the receipt then rejects.
        schema = object_schema({"traits": {"type": "array", "minItems": 1, "maxItems": 16, "items": object_schema({
            "id": {"type": "string", "maxLength": 64}, "priority": {"type": "string", "enum": ["core", "secondary", "variation"]},
            "observation": {"type": "string"}, "reference_ids": strings(), "uncertain": {"type": "boolean"}})},
            "observations": {**strings(), "maxItems": 16}})
    elif stage in ("comparison", "repair"):
        if request.get("protocol") != COMPARISON_VERSION:
            raise ValueError("stale comparison protocol")
        r = request["comparison"]
        # A repair (fn-80) is text only: the reviewer's own answer and the
        # exact tidiness rules it broke, answered in the comparison's schema.
        images = [] if stage == "repair" else r["images"] + r["references"] + [a["image"] for a in r["quality_anchors"]]
        schema = object_schema({
            "passes": {"type": "array", "minItems": len(r["required"]), "maxItems": len(r["required"]), "items": {"type": "string", "enum": ["pass", "fail", "unknown"]}},
            "defects": {"type": "array", "items": object_schema({"defect": {"type": "string"}, "trait_id": trait_id()})},
            "observations": strings(),
            # The receipt holds at most 16 findings and 16 coverage rows
            # (reference_first.rs, ComparisonResult::bind; joint.rs, verify_findings).
            "findings": {"type": "array", "maxItems": 16, "items": object_schema({"observation": {"type": "string"}, "evidence_ids": strings(),
                "impact": {"type": "string", "enum": ["supported", "blocker", "required_unknown", "variation", "optional"]},
                "uncertain": {"type": "boolean"}, "causal_hypothesis": {"type": ["string", "null"]}, "trait_id": trait_id()})},
            "coverage": {"type": "array", "maxItems": 16, "items": object_schema({"trait_id": {"type": "string"},
                "status": {"type": "string", "enum": ["pass", "fail", "unknown"]}, "evidence_ids": strings(), "explanation": {"type": "string"}})}})
    else:
        raise ValueError("unknown stage")
    if stage != "repair" and (not images or len(images) > 12):
        raise ValueError("invalid image count")
    paths = []
    for image in images:
        p = Path(image["path"]).resolve()
        if hashlib.sha256(p.read_bytes()).hexdigest() != image["sha256"]:
            raise ValueError("image hash mismatch")
        paths.append(p)
    if hashlib.sha256(envelope["prompt"].encode()).hexdigest() != envelope["prompt_sha256"]:
        raise ValueError("prompt hash mismatch")
    if stage == "repair":
        violations, answer = envelope.get("violations"), envelope.get("answer")
        if not isinstance(answer, dict) or not violations or not all(isinstance(v, str) for v in violations):
            raise ValueError("a repair needs the previous answer and its violations")
        prompt = (envelope["prompt"] + "\nDo not use tools or inspect files. Return JSON only.\nViolations:\n"
                  + json.dumps(violations) + "\nPrevious answer:\n" + json.dumps(answer))
        return paths, schema, prompt
    prompt = envelope["prompt"] + "\nDo not use tools or inspect files. Attached images follow metadata order. Return JSON only.\n" + json.dumps(request)
    if stage == "comparison":
        prompt += "\nReturn exactly one passes entry for each comparison.required cell, in that exact order; this is not an aggregate pass."
    return paths, schema, prompt


def main():
    p = argparse.ArgumentParser()
    p.add_argument("--model", required=True)
    p.add_argument("--effort", required=True)
    args = p.parse_args()
    envelope = json.load(sys.stdin)
    paths, schema, prompt = prepare(envelope)
    with tempfile.TemporaryDirectory(prefix="reference-first-") as scratch:
        out, schema_path = Path(scratch) / "answer.json", Path(scratch) / "schema.json"
        schema_path.write_text(json.dumps(schema))
        command = ["codex", "exec", "--ignore-user-config", "--ephemeral", "--skip-git-repo-check", "--sandbox", "read-only", "-C", scratch,
                   "-m", args.model, "-c", f'model_reasoning_effort="{args.effort}"', "--json", "--output-schema", str(schema_path), "-o", str(out)]
        for image in paths:
            command.extend(["--image", str(image)])
        command.extend(["--", prompt])
        run = subprocess.run(command, stdin=subprocess.DEVNULL, capture_output=True, timeout=300)
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
        valid_count = envelope["stage"] not in ("comparison", "repair") or (isinstance(answer, dict) and len(answer.get("passes", [])) == len(envelope["request"]["comparison"]["required"]))
        print(json.dumps({"request_sha256": envelope["request_sha256"], "prompt_sha256": envelope["prompt_sha256"],
            "dispatched_prompt_sha256": hashlib.sha256(prompt.encode()).hexdigest(), "schema_sha256": hashlib.sha256(json.dumps(schema).encode()).hexdigest(),
            "model": args.model, "model_identity_basis": "requested command argument; actual resolved identity not exposed", "effort": args.effort,
            "status": "ok" if run.returncode == 0 and not forbidden_tools and usage is not None and valid_count else "failed_or_tools_or_unknown_usage_or_cardinality",
            "forbidden_tools": forbidden_tools, "usage": usage, "answer": answer,
            "raw_events": run.stdout.decode(), "stderr": run.stderr.decode(), "image_sha256": [hashlib.sha256(p.read_bytes()).hexdigest() for p in paths]}))


if __name__ == "__main__":
    main()
