#!/usr/bin/env python3
"""The reference-first reviewer: one isolated Claude call per stage
(inventory, comparison, repair), no retry and no scheduler. `prepare` holds the
allowlist, image checks and prompt and schema; `vision_claude.run` makes the
call.
"""
import argparse
import hashlib
import json
from pathlib import Path
import sys


def object_schema(properties):
    return {"type": "object", "additionalProperties": False, "properties": properties, "required": list(properties)}


def strings():
    return {"type": "array", "items": {"type": "string"}}


# The comparison schema's version: v2 (fn-136) ties each finding and defect to
# an inventory trait, or null, and the request names the known gaps.
COMPARISON_VERSION = "reference-first-comparison-v2"


# The photograph screen's version (fn-149, reference photographs found by the
# Profile stage).
SCREEN_VERSION = "reference-screen-v1"


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
    elif stage == "screen":
        # fn-149: one look over candidate photographs the Profile stage found;
        # per photograph, the species, maturity, open growth, framing and view.
        if request.get("protocol") != SCREEN_VERSION:
            raise ValueError("stale screen protocol")
        images = [c["image"] for c in request["candidates"]]
        verdict = object_schema({"id": {"type": "string"}, "species": {"type": "string", "enum": ["yes", "no", "unsure"]},
            "mature_open_grown": {"type": "boolean"}, "whole_tree": {"type": "boolean"},
            "view": {"type": "string", "enum": ["leaf-on", "bare", "bark", "other"]}})
        schema = object_schema({"candidates": {"type": "array", "minItems": len(images), "maxItems": len(images), "items": verdict}})
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
    import vision_claude  # the call itself; `prepare` needs no model

    parser = argparse.ArgumentParser()
    parser.add_argument("--model", required=True)
    parser.add_argument("--effort", required=True)
    args = parser.parse_args()
    envelope = json.load(sys.stdin)
    paths, schema, prompt = prepare(envelope)
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
        "forbidden_tools": result["forbidden_tools"], "error": result["error"], "usage": result["usage"], "answer": answer,
        "raw_events": result["raw_events"], "stderr": result["stderr"], "image_sha256": [hashlib.sha256(Path(p).read_bytes()).hexdigest() for p in paths]}))


if __name__ == "__main__":
    main()
