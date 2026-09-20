#!/usr/bin/env python3
"""Dry/preflight for the fn-68 proof packet. Uses the existing adapter. No live call."""
import argparse
import hashlib
import importlib.util
import json
import subprocess
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
    if args.execute:
        runner = PROOF / "run-proof.py"
        return subprocess.run(
            [sys.executable, str(runner), "--execute", "--stage", "stage-a-birch"],
            cwd=str(PROOF),
        ).returncode

    assert digest(RUNTIME) == RUNTIME_SHA, "original runtime SHA changed"
    packet = json.loads((ROOT / "priority-review.json").read_text())
    assert packet["approval"] is None
    approval = json.loads((ROOT / "priority-approval.json").read_text())
    assert approval["quotation"] == "the 1,2,3 yes"
    assert approval["applied_to_runtime"] is False

    grant = load("owner-grant.json")
    assert grant["quotation"] == "ok approved"
    assert grant["granted_not_spent"]["token_cap"] == 902431
    assert grant["granted_not_spent"]["visual_cap"] == 26
    assert grant["granted_not_spent"]["retries"] is False
    assert grant["applied_to_runtime"] is False

    framing = load("framing.json")
    assert framing["birch_positive"]["sha256"] == framing["birch_positive"]["expected"]
    assert framing["beech_negative"]["sha256"] == framing["beech_negative"]["expected"]
    assert framing["birch_positive"]["framing"] == "clipped"
    assert framing["birch_positive"]["owner_accepted"] is True
    assert framing["beech_negative"]["framing"] == "clipped"
    assert framing["beech_negative"]["width"] == "unknown"
    assert digest(framing["birch_positive"]["path"]) == framing["birch_positive"]["expected"]
    assert digest(framing["beech_negative"]["path"]) == framing["beech_negative"]["expected"]
    reframed = framing["birch_positive_reframed"]
    assert reframed["framing"] == "complete"
    assert reframed["owner_accepted"] is False
    assert reframed["new_raster_owner_accepted"] is False
    assert digest(reframed["path"]) == reframed["sha256"]
    assert digest(reframed["twin_path"]) == reframed["twin_sha256"]

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

    birch_a = load("stage-a-birch-request.json")
    assert [r["image"]["view"] for r in birch_a["request"]["references"]] == ["S-WHOLE"]
    assert birch_a["factual_scope"]["views"] == ["S-WHOLE"]
    birch_b = load("stage-b-birch-positive-request.json")
    assert birch_b["request"]["inventory"] is None
    assert not Path(birch_b["inventory_bind"]["path"]).exists()
    assert "Do not apply European-beech owner priorities" in birch_b["request"]["comparison"]["checklist"]
    assert [r["view"] for r in birch_b["request"]["comparison"]["references"]] == ["S-WHOLE"]
    assert birch_b["request"]["comparison"]["images"][0]["sha256"] == framing["birch_positive_reframed"]["sha256"]
    assert birch_b["render_provenance"]["assessed_raster_owner_accepted"] is False
    assert birch_b["render_provenance"]["historical_geometry_owner_accepted_sha256"] == framing["birch_positive"]["expected"]
    negative = load("negative-acceptance.json")
    assert negative["success"] == "fail_with_grounded_finding"
    assert negative["unknown_or_clipping_only"] == "report_separately_not_success"

    final = load("stage-r7-final-template.json")
    assert final["request"]["comparison"]["images"] == []
    assert final["request"]["comparison"]["identity"] is None

    ceilings = load("ceilings.json")
    assert ceilings["sum_stage_ceilings"] == 291000
    assert ceilings["sum_stage_ceilings"] <= ceilings["planning_envelope_additional_tokens"]
    assert ceilings["current"]["token_cap"] == 902431
    assert ceilings["granted"]["quotation"] == "ok approved"
    assert ceilings["granted"]["retries"] is False
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

    counts = json.loads((PROOF / "capture-accounting.json").read_text())
    assert counts["prior_actual_captures"] == 24
    assert counts["actual_captures"] == 26
    assert counts["reserved"] == 30
    assert counts["side_import_reservations"] == 14
    assert counts["original_runtime_reservations"] == 16

    ready = json.loads((WORKTREE / ".flow/tmp/cursor-fn68-ready.json").read_text())
    assert ready["stop_dispatch"] is True
    assert ready["paid_calls_this_invocation"] == 0
    assert ready["first_paid_stage"] == "stage-a-birch"
    assert ready["grant"]["quotation"] == "ok approved"
    assert ready["framing"]["birch_owner_accepted"]["framing"] == "clipped"
    assert ready["framing"]["birch_assessed"]["owner_accepted"] is False
    assert ready["stage_a"]["scope"]["views"] == ["S-WHOLE"]
    assert ready["accounting"]["actual_captures"] == 26
    assert ready["execute"]["without_release"] == 2

    out = {
        "status": "dry-ok",
        "execute": False,
        "runtime_sha256": RUNTIME_SHA,
        "prepared": prepared,
        "stage_ceilings": ceilings["stages"],
        "sum_stage_ceilings": 291000,
        "granted": ceilings["granted"],
        "dispatch": load("dispatch-pending.json")["status"],
        "r8_order": ["stage-a-birch", "stage-b-birch-positive", "stage-b-beech-negative"],
        "r7_order": ["r7-current", "route", "continuation", "proposal-magnitude", "up-to-4-eval", "r7-final"],
        "stop": "calibration fail or ceiling exceed stops; no retry; no redesign",
        "ready": str(WORKTREE / ".flow/tmp/cursor-fn68-ready.json"),
    }
    print(json.dumps(out, indent=2))
    return 0


if __name__ == "__main__":
    sys.exit(main())
