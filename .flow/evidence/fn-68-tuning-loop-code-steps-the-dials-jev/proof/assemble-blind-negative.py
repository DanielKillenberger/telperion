#!/usr/bin/env python3
"""Build the blinded beech-negative envelope. Inventory stays exact. No model call."""
import hashlib
import json
from pathlib import Path

from proof_lib import (
    COMPARISON_PROMPT_SHA,
    PROOF,
    adapter,
    assert_blind_payload,
    ensure_candidate_transport,
    frozen_inventory,
    load,
    request_body_sha256,
    strip_grader_keys,
    verify_frozen_inventory,
)

CHECKLIST = (
    "Recognizable mature European beech at the established catalogue finish floor. "
    "Whole-crown blocking: believable beech branching and foliage character, no obvious "
    "regular-shell construction, leaf-bearing weight consistent with the species. "
    "Photographs establish species character, not photorealism. Missing, clipped or "
    "ambiguous evidence is unknown."
)


def main():
    old = load("stage-b-beech-negative-request.json")
    inventory = json.loads(json.dumps(frozen_inventory()))
    verify_frozen_inventory(inventory)
    projection = ensure_candidate_transport()
    comparison = json.loads(json.dumps(old["request"]["comparison"]))
    leak = "This case is the known negative; a pass is false-ready."
    if leak not in comparison["checklist"]:
        raise ValueError("expected leak sentence missing from frozen checklist")
    comparison["checklist"] = CHECKLIST
    comparison["images"][0]["path"] = projection["projected"]
    if comparison["images"][0]["sha256"] != projection["sha256"]:
        raise ValueError("candidate sha256 changed")
    request = {"inventory": inventory, "comparison": comparison}
    envelope = {
        "stage": "comparison",
        "prompt": old["prompt"],
        "prompt_sha256": old["prompt_sha256"],
        "inventory_bind": old["inventory_bind"],
        "request": request,
        "request_sha256": request_body_sha256(request),
    }
    if envelope["prompt_sha256"] != COMPARISON_PROMPT_SHA:
        raise ValueError("generic comparison prompt changed")
    if envelope["request"]["inventory"] != old["request"]["inventory"]:
        raise ValueError("inventory mutated")
    visible = strip_grader_keys(envelope)
    paths, schema, prompt = adapter().prepare(visible)
    assert_blind_payload(prompt, schema, paths)
    (PROOF / "stage-b-beech-negative-blind-request.json").write_text(
        json.dumps(envelope, indent=2) + "\n"
    )
    grader = {
        "stage": "stage-b-beech-negative-blind",
        "expected_ready": False,
        "known_negative": True,
        "label": "beech-round22 owner-rejected; laterally cropped; width unknown",
        "success": "fail_with_grounded_visible_morphology",
        "unknown_or_clipping_only": "abstention_not_successful_negative",
        "semantic_qualification": "host",
        "not_sent_to_model": True,
    }
    (PROOF / "stage-b-beech-negative-blind-grader.json").write_text(
        json.dumps(grader, indent=2) + "\n"
    )
    (PROOF / "transport-projection.json").write_text(json.dumps(projection, indent=2) + "\n")
    image_order = [
        {
            "order": i,
            "path": str(p),
            "name": Path(p).name,
            "sha256": hashlib.sha256(Path(p).read_bytes()).hexdigest(),
        }
        for i, p in enumerate(paths)
    ]
    record = {
        "status": "payload_ready_for_host_review",
        "paid_calls_this_turn": 0,
        "generic_prompt_sha256": envelope["prompt_sha256"],
        "request_sha256": envelope["request_sha256"],
        "request_file_sha256": hashlib.sha256(
            (PROOF / "stage-b-beech-negative-blind-request.json").read_bytes()
        ).hexdigest(),
        "request_body_sha256": request_body_sha256(visible["request"]),
        "dispatched_prompt_sha256": hashlib.sha256(prompt.encode()).hexdigest(),
        "schema_sha256": hashlib.sha256(json.dumps(schema).encode()).hexdigest(),
        "schema": schema,
        "inventory_request_sha256": inventory["request_sha256"],
        "inventory_verified": True,
        "transport_projection_sha256": projection["projection_sha256"],
        "image_order": image_order,
        "model_visible_paths": [str(p) for p in paths],
        "grader_file": "stage-b-beech-negative-blind-grader.json",
        "contaminated_request_unaltered": True,
        "dispatched_prompt_bytes": len(prompt.encode()),
        "forbidden_markers_present": [],
    }
    (PROOF / "model-visible-payload.json").write_text(json.dumps(record, indent=2) + "\n")
    print(json.dumps({
        "request_sha256": record["request_sha256"],
        "dispatched_prompt_sha256": record["dispatched_prompt_sha256"],
        "schema_sha256": record["schema_sha256"],
        "request_file_sha256": record["request_file_sha256"],
        "inventory_request_sha256": record["inventory_request_sha256"],
        "transport_projection_sha256": record["transport_projection_sha256"],
        "dispatched_prompt_bytes": record["dispatched_prompt_bytes"],
        "image_names": [row["name"] for row in image_order],
    }, indent=2))


if __name__ == "__main__":
    main()
