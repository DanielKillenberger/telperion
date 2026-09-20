#!/usr/bin/env python3
"""stdin: typed vision request; stdout: typed assessment. Images go to --image."""
import argparse
import hashlib
import json
import subprocess
import sys
import tempfile
from pathlib import Path

p = argparse.ArgumentParser()
p.add_argument("--model", required=True)
p.add_argument("--effort", required=True)
a = p.parse_args()
envelope = json.load(sys.stdin)
request = envelope["request"]
paths = []
for image in request["images"] + request["references"] + [a["image"] for a in request["quality_anchors"]]:
    path = Path(image["path"]).resolve()
    if hashlib.sha256(path.read_bytes()).hexdigest() != image["sha256"]:
        raise SystemExit("image changed before vision dispatch")
    paths.append(path)
    if len(paths) > 12:
        raise SystemExit("visual request exceeds twelve-image input bound")
with tempfile.TemporaryDirectory(prefix="tuning-vision-") as scratch:
    out = Path(scratch) / "result.json"
    schema = Path(scratch) / "schema.json"
    schema.write_text(json.dumps({"type": "object", "additionalProperties": False,
        "properties": {"passes": {"type": "array", "items": {"type": "string", "enum": ["pass", "fail", "unknown"]}},
                       "defects": {"type": "array", "items": {"type": "string"}},
                       "observations": {"type": "array", "items": {"type": "string"}},
                       "findings": {"type":"array", "items":{"type":"object", "additionalProperties":False,
                           "properties":{"observation":{"type":"string"},"evidence_ids":{"type":"array","items":{"type":"string"}},
                               "impact":{"type":"string","enum":["supported","blocker","required_unknown","variation","optional"]},
                               "uncertain":{"type":"boolean"},"causal_hypothesis":{"type":["string","null"]}},
                           "required":["observation","evidence_ids","impact","uncertain","causal_hypothesis"]}}},
        "required": ["passes", "defects", "observations", "findings"]}))
    prompt = ("Assess believable reference character of the intended species at a finish quality "
              "comparable to the accepted catalogue anchors. Photorealism is NOT the goal. Distinguish "
              "relative improvement from absolute readiness: improvement alone never establishes readiness. "
              "Defining structural character is not optional realism. An explicit "
              "owner criterion outranks a model claim that its absence is acceptable variation. Apply the "
              "species-specific checklist without inventing requirements for other species. Attached images are "
              "candidate renders, photographic references, then accepted quality anchors in metadata order. "
              "Do not use tools or inspect files. For each required checklist cell return pass, fail or unknown. "
              "Review all views jointly, using the labelled joint packet. Render views sharing identity and seed "
              "share geometry; hiding foliage changes visibility, NOT an unloaded leaf-off geometry. Reference "
              "specimen/condition relationships are unknown unless documented; never infer the same specimen "
              "or a causal change from two photographs. Rank cross-view constraints and the largest reference "
              "gaps. Findings must cite packet evidence_ids and separate observed mismatch from optional causal "
              "hypotheses; no particular biological theory is required. "
              "For every required cell cite its render and reference evidence in an affirmative supported "
              "finding or a gap finding. Supported means a grounded reference match, not an invented defect; "
              "a clean candidate can pass with supported findings and zero defects. Missing/clipped evidence or unresolved "
              "required classification means unknown. Uncertain optional refinements alone do not block. "
              "Block only wrong species character, obvious construction artifacts, regression below the "
              "accepted quality anchors or an explicit unmet requirement. Ground each blocking defect "
              "in a specific view/seed and reference or quality anchor; rank blocking defects by impact on "
              "believability, main gap first. Separate acceptable natural variation from optional detail in "
              "observations. Do not contradict a defect's blocker status in observations. If classification "
              "is uncertain, return unknown rather than silently treating defining structure as optional. "
              "Optional realism/detail improvements "
              "go in observations and NEVER block readiness. If a blocker cannot be justified against this "
              "standard, return unknown for human clarification, not an invitation to endless polish. "
              "Numerical scores cannot confer readiness. Do not demand pixel equality or identical shape. "
              "Return JSON only.\n" + json.dumps(request))
    command = ["codex", "exec", "--ignore-user-config", "--ephemeral", "--skip-git-repo-check",
               "--sandbox", "read-only", "-C", scratch, "-m", a.model,
               "-c", f'model_reasoning_effort="{a.effort}"', "--json",
               "--output-schema", str(schema), "-o", str(out)]
    for path in paths:
        command.extend(["--image", str(path)])
    command.extend(["--", prompt])
    run = subprocess.run(command, stdin=subprocess.DEVNULL, capture_output=True, timeout=180)
    if run.returncode:
        sys.stderr.buffer.write(run.stderr)
        raise SystemExit(run.returncode)
    answer = json.loads(out.read_text())
    usage = None
    for line in run.stdout.splitlines():
        event = json.loads(line)
        if event.get("type") == "turn.completed":
            raw = event.get("usage", {})
            if "input_tokens" in raw and "output_tokens" in raw:
                usage = {"input_tokens": raw["input_tokens"], "output_tokens": raw["output_tokens"]}
    if len(answer["passes"]) != len(request["required"]):
        raise SystemExit("wrong checklist cardinality")
    result = {"request_sha256": envelope["request_sha256"],
              "assessment": {"identity": request["identity"], "model": a.model, "ledger": "adapter",
                             "cells": list(zip(request["required"], answer["passes"])),
                             "defects": answer["defects"], "findings":answer["findings"]},
              "effort": a.effort, "usage": usage, "observations": answer["observations"]}
    print(json.dumps(result))
