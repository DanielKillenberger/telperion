#!/usr/bin/env python3
"""Dry/preflight for the fn-68 proof packet. Uses the existing adapter. No live call."""
import argparse
import hashlib
import importlib.util
import json
import sys
from pathlib import Path

PROOF = Path(__file__).resolve().parent
ROOT = PROOF.parent
WORKTREE = ROOT.parents[2]
ADAPTER = WORKTREE / "scripts" / "reference-first-codex.py"
RUNTIME = WORKTREE / ".flow/tmp/fn68-pilot-run/run.json"
RUNTIME_SHA = "d93259cd3e6980a19b09c3a1644d9f6114012ad7345bb0c02670941f151e885e"
AUTHORITY = PROOF / "AUTHORITY.json"


def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def load(name):
    return json.loads((PROOF / name).read_text())


def adapter():
    spec = importlib.util.spec_from_file_location("adapter", ADAPTER)
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--execute", action="store_true")
    args = parser.parse_args()
    if args.execute or AUTHORITY.exists():
        print("execute refused: no live authority in this packet", file=sys.stderr)
        return 2

    assert digest(RUNTIME) == RUNTIME_SHA, "original runtime SHA changed"
    packet = json.loads((ROOT / "priority-review.json").read_text())
    assert packet["approval"] is None
    approval = json.loads((ROOT / "priority-approval.json").read_text())
    assert approval["quotation"] == "the 1,2,3 yes"
    assert approval["applied_to_runtime"] is False

    framing = load("framing.json")
    assert framing["birch_positive"]["sha256"] == framing["birch_positive"]["expected"]
    assert framing["beech_negative"]["sha256"] == framing["beech_negative"]["expected"]
    assert framing["birch_positive"]["framing"] == "complete"
    assert framing["beech_negative"]["framing"] == "complete"
    assert digest(framing["birch_positive"]["path"]) == framing["birch_positive"]["expected"]
    assert digest(framing["beech_negative"]["path"]) == framing["beech_negative"]["expected"]

    ad = adapter()
    prepared = {}
    for name in (
        "stage-a-birch-request.json",
        "stage-b-birch-positive-request.json",
        "stage-b-beech-negative-request.json",
        "stage-r7-current-request.json",
    ):
        envelope = load(name)
        paths, schema, prompt = ad.prepare(envelope)
        for image in envelope.get("request", {}).get("comparison", {}).get("images", []):
            assert digest(image["path"]) == image["sha256"]
        prepared[name] = {
            "images": len(paths),
            "prompt_bytes": len(prompt.encode()),
            "schema_bytes": len(json.dumps(schema).encode()),
        }

    birch_b = load("stage-b-birch-positive-request.json")
    assert birch_b["request"]["inventory"] is None
    assert not Path(birch_b["inventory_bind"]["path"]).exists()
    assert "European-beech owner priorities" in birch_b["request"]["comparison"]["checklist"] or "Do not apply European-beech owner priorities" in birch_b["request"]["comparison"]["checklist"]

    final = load("stage-r7-final-template.json")
    assert final["request"]["comparison"]["images"] == []
    assert final["request"]["comparison"]["identity"] is None

    ceilings = load("ceilings.json")
    assert ceilings["sum_stage_ceilings"] == 291000
    assert ceilings["sum_stage_ceilings"] <= ceilings["planning_envelope_additional_tokens"]
    prior = json.loads((ROOT / "r7-r8-offline-preflight.json").read_text())["stages"]
    by_id = {s["id"]: s["ceiling"] for s in ceilings["stages"]}
    assert by_id["route"] >= prior["route"]["tokens"]
    assert by_id["continuation"] >= prior["continuation"]["tokens"]
    assert by_id["proposal-magnitude"] >= prior["proposal_magnitude"]["tokens"]
    assert by_id["r7-current"] >= 41411

    scope = load("scope.json")
    assert scope["six_cell_readiness_from_two_views"] is False
    assert scope["owner_approval"]["applied_to_original_run"] is False
    assert len(scope["required_scope_extension_not_in_packet"]) == 4

    out = {
        "status": "dry-ok",
        "execute": False,
        "runtime_sha256": RUNTIME_SHA,
        "prepared": prepared,
        "stage_ceilings": ceilings["stages"],
        "sum_stage_ceilings": 291000,
        "proposed_not_granted": ceilings["proposed_not_granted"],
        "dispatch": load("dispatch-pending.json")["status"],
        "r8_order": ["stage-a-birch", "stage-b-birch-positive", "stage-b-beech-negative"],
        "r7_order": ["r7-current", "route", "continuation", "proposal-magnitude", "up-to-4-eval", "r7-final"],
        "stop": "calibration fail or ceiling exceed stops; no retry; no redesign",
    }
    print(json.dumps(out, indent=2))
    return 0


if __name__ == "__main__":
    sys.exit(main())
