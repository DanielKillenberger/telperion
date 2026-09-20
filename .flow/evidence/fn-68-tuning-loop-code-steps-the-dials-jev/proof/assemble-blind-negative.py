#!/usr/bin/env python3
"""Build the blinded beech-negative envelope from the frozen inventory. No model call."""
import hashlib
import json
from pathlib import Path

from proof_lib import (
    BLIND_IMAGES,
    COMPARISON_PROMPT_SHA,
    PROOF,
    adapter,
    assert_blind_payload,
    ensure_neutral_images,
    load,
    request_body_sha256,
    strip_grader_keys,
)

CHECKLIST = (
    "Recognizable mature European beech at the established catalogue finish floor. "
    "Whole-crown blocking: believable beech branching and foliage character, no obvious "
    "regular-shell construction, leaf-bearing weight consistent with the species. "
    "Photographs establish species character, not photorealism. Missing, clipped or "
    "ambiguous evidence is unknown."
)
IDENTITY = "7e69fda3e95b317fcb9d89e15e32179c54fdd316a57d1c3223d6e26639b8b490-seed1-whole"


def main():
    placed = {row["name"]: row for row in ensure_neutral_images()}
    old = load("stage-b-beech-negative-request.json")
    inventory = json.loads(json.dumps(old["request"]["inventory"]))
    inventory["request"]["references"][0]["image"]["path"] = placed["reference-0.jpg"]["path"]
    inventory["request"]["references"][1]["image"]["path"] = placed["reference-1.jpg"]["path"]
    request = {
        "inventory": inventory,
        "comparison": {
            "schema": "tuning-vision-v3",
            "target_species": "European beech / Fagus sylvatica",
            "identity": IDENTITY,
            "checklist": CHECKLIST,
            "images": [
                {
                    "path": placed["render-0.png"]["path"],
                    "sha256": placed["render-0.png"]["sha256"],
                    "view": "B-WHOLE",
                    "seed": 1,
                }
            ],
            "references": [
                {
                    "path": placed["reference-0.jpg"]["path"],
                    "sha256": placed["reference-0.jpg"]["sha256"],
                    "view": "B-WHOLE",
                    "seed": 1,
                },
                {
                    "path": placed["reference-1.jpg"]["path"],
                    "sha256": placed["reference-1.jpg"]["sha256"],
                    "view": "B-BARE",
                    "seed": 1,
                },
            ],
            "quality_anchors": [
                {
                    "image": {
                        "path": placed["anchor-0.png"]["path"],
                        "sha256": placed["anchor-0.png"]["sha256"],
                        "view": "whole",
                        "seed": 1,
                    },
                    "provenance": "established catalogue anchor image sha256:8e10ac1cf993a4cc005f7a1374e5763274c5dc636f42204e140464242ba027f1",
                    "scope": "finish/style only; not species morphology",
                }
            ],
            "required": [{"item": "reference_character", "view": "B-WHOLE", "seed": 1}],
            "joint": {
                "inputs": [
                    {
                        "id": "render-0",
                        "role": "render",
                        "view": "B-WHOLE",
                        "seed": 1,
                        "sha256": placed["render-0.png"]["sha256"],
                        "render_identity": IDENTITY,
                        "geometry_group": IDENTITY + ":seed:1",
                        "condition": "historical_reconstructed_still",
                        "visibility": "shown",
                        "framing": "clipped",
                    },
                    {
                        "id": "reference-0",
                        "role": "reference",
                        "view": "B-WHOLE",
                        "seed": 1,
                        "sha256": placed["reference-0.jpg"]["sha256"],
                        "render_identity": None,
                        "geometry_group": None,
                        "condition": "unknown",
                        "visibility": "unknown",
                        "framing": "unknown",
                    },
                    {
                        "id": "reference-1",
                        "role": "reference",
                        "view": "B-BARE",
                        "seed": 1,
                        "sha256": placed["reference-1.jpg"]["sha256"],
                        "render_identity": None,
                        "geometry_group": None,
                        "condition": "unknown",
                        "visibility": "unknown",
                        "framing": "unknown",
                    },
                    {
                        "id": "anchor-0",
                        "role": "anchor",
                        "view": "whole",
                        "seed": 1,
                        "sha256": placed["anchor-0.png"]["sha256"],
                        "render_identity": None,
                        "geometry_group": None,
                        "condition": "unknown",
                        "visibility": "unknown",
                        "framing": "unknown",
                    },
                ],
                "reference_relation": "unknown",
                "relation_source": None,
            },
        },
    }
    envelope = {
        "stage": "comparison",
        "prompt": old["prompt"],
        "prompt_sha256": old["prompt_sha256"],
        "inventory_bind": {
            "status": "pinned_existing",
            "path": old["inventory_bind"]["path"],
        },
        "request": request,
        "request_sha256": request_body_sha256(request),
    }
    if envelope["prompt_sha256"] != COMPARISON_PROMPT_SHA:
        raise ValueError("generic comparison prompt changed")
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
        "image_order": image_order,
        "model_visible_paths": [str(p) for p in paths],
        "grader_file": "stage-b-beech-negative-blind-grader.json",
        "contaminated_request_unaltered": True,
        "neutral_images": list(placed.values()),
        "dispatched_prompt_bytes": len(prompt.encode()),
        "forbidden_markers_present": [],
    }
    (PROOF / "model-visible-payload.json").write_text(json.dumps(record, indent=2) + "\n")
    print(json.dumps({k: record[k] for k in (
        "request_sha256",
        "dispatched_prompt_sha256",
        "schema_sha256",
        "request_file_sha256",
        "dispatched_prompt_bytes",
    )}, indent=2))


if __name__ == "__main__":
    main()
