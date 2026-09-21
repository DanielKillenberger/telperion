#!/usr/bin/env python3
"""Uncalibrated contact-sheet adapter: one isolated call over several renders.

The envelope names no candidate, no round and no strength. Which render the
loop is standing on is decided and recorded by the caller, never sent here.

Two requests travel in the envelope. `request` is what was hashed and where the
image files are; `prompt_request` is the caller's redacted projection of it,
with every image reduced to its label and its digest, and only that one is
written into the prompt. A render's file name can carry the trial key.
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


def redacted(envelope, keys, images):
    """The projection the prompt is allowed to carry, checked before it is used."""
    request = envelope.get("prompt_request")
    if not isinstance(request, dict):
        raise ValueError("missing redacted prompt_request")
    if set(request) != keys:
        raise ValueError("prompt request allowlist violation")
    for image in images(request):
        if set(image) != {"role", "sha256"} and set(image) != {"label", "sha256"}:
            raise ValueError("the prompt request names more than a label and a digest")
    return request


def prepare(envelope):
    if envelope["stage"] != "sheet":
        raise ValueError("unknown stage")
    request = envelope["request"]
    if set(request) != {"schema", "target_species", "view", "seed", "references", "renders", "priorities", "owner_notes"}:
        raise ValueError("contact-sheet allowlist violation")
    if request["schema"] != "tuning-sheet-v2":
        raise ValueError("unknown contact-sheet schema")
    prompt_request = redacted(envelope, set(request),
                              lambda r: r["references"] + r["renders"])
    ids = [p["id"] for p in request["priorities"]]
    if not ids or len(ids) != len(set(ids)):
        raise ValueError("invalid priority list")
    labels = [str(i + 1) for i in range(len(request["renders"]))]
    if not 2 <= len(labels) <= 5:
        raise ValueError("a sheet shows two to five renders")
    # Metadata order: the references for this view, then the renders in label order.
    images = request["references"] + request["renders"]
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
    label = {"type": "string", "enum": labels}
    schema = object_schema({
        "priorities": {"type": "array", "minItems": len(ids), "maxItems": len(ids),
                       "items": object_schema({
                           "priority_id": {"type": "string", "enum": ids},
                           "closest": label,
                           "ranking": {"type": "array", "minItems": len(labels), "maxItems": len(labels), "items": label},
                           "steps": {"type": "array", "minItems": len(labels) - 1, "maxItems": len(labels) - 1,
                                     "items": object_schema({"from": label, "to": label,
                                                             "grade": {"type": "string", "enum": ["clear", "slight", "none"]}})}})},
        "overall": {"type": "array", "minItems": len(labels), "maxItems": len(labels), "items": label},
        "wrong": {"type": "array", "maxItems": 2 * len(labels),
                  "items": object_schema({"render": label, "text": {"type": "string"}})},
        "breaks": {"type": "array", "items": object_schema({"render": label, "text": {"type": "string"}})},
        "improved": {"type": "string"}, "missing": {"type": "string"}})
    prompt = (envelope["prompt"]
              + "\nDo not use tools or inspect files. Attached images follow metadata order: the reference photographs, then the renders in the order they are numbered. Return JSON only.\n"
              + json.dumps(prompt_request)
              + "\nReturn one entry for each listed priority id. Each ranking lists every render number exactly once, and each steps list grades the adjacent pairs of that ranking in order. The overall list also names every render number exactly once, best first. Give at most two wrong entries for any one render.")
    return paths, schema, prompt


def main():
    p = argparse.ArgumentParser()
    p.add_argument("--model", required=True)
    p.add_argument("--effort", required=True)
    args = p.parse_args()
    envelope = json.load(sys.stdin)
    paths, schema, prompt = prepare(envelope)
    ids = [q["id"] for q in envelope["request"]["priorities"]]
    labels = [str(i + 1) for i in range(len(envelope["request"]["renders"]))]
    with tempfile.TemporaryDirectory(prefix="contact-sheet-") as scratch:
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
        answered = (isinstance(answer, dict)
                    and sorted(p.get("priority_id") for p in answer.get("priorities", [])) == sorted(ids)
                    and all(sorted(p.get("ranking", [])) == sorted(labels) for p in answer.get("priorities", []))
                    and sorted(answer.get("overall", [])) == sorted(labels)
                    and all(sum(1 for w in answer.get("wrong", []) if w.get("render") == r) <= 2 for r in labels))
        print(json.dumps({"request_sha256": envelope["request_sha256"], "prompt_sha256": envelope["prompt_sha256"],
            "dispatched_prompt_sha256": hashlib.sha256(prompt.encode()).hexdigest(), "schema_sha256": hashlib.sha256(json.dumps(schema).encode()).hexdigest(),
            "model": args.model, "model_identity_basis": "requested command argument; actual resolved identity not exposed", "effort": args.effort,
            "status": "ok" if run.returncode == 0 and not forbidden_tools and usage is not None and answered else "failed_or_tools_or_unknown_usage_or_cardinality",
            "forbidden_tools": forbidden_tools, "usage": usage, "answer": answer,
            "uncalibrated": "contact-sheet ranking and grades; no replay qualifies this question",
            "raw_events": run.stdout.decode(), "stderr": run.stderr.decode(), "image_sha256": [hashlib.sha256(p.read_bytes()).hexdigest() for p in paths]}))


if __name__ == "__main__":
    main()
