#!/usr/bin/env python3
"""Evidence-only staged runner. Calls the existing adapter only after host release."""
import argparse
import json
import subprocess
import sys
import time

from proof_lib import (
    ADAPTER,
    PROOF,
    RUNTIME,
    RUNTIME_SHA,
    accounting,
    adapter,
    assert_whole_only_stage_a,
    assert_whole_only_stage_b,
    bind_inventory,
    digest,
    judge_negative,
    load,
    prepare_envelope,
    release_path,
    require_runtime,
)

STATE = PROOF / "run-state.json"
INVENTORY = PROOF / "stage-a-birch-inventory.json"
ENVELOPES = {
    "stage-a-birch": "stage-a-birch-request.json",
    "stage-b-birch-positive": "stage-b-birch-positive-request.json",
    "stage-b-beech-negative": "stage-b-beech-negative-request.json",
    "r7-current": "stage-r7-current-request.json",
}


def ceilings():
    return load("ceilings.json")


def state_or_init():
    if STATE.exists():
        return json.loads(STATE.read_text())
    grant = load("owner-grant.json")
    counts = accounting()
    return {
        "status": "pending_host_release",
        "tokens": grant["previous_actual"]["tokens"],
        "token_cap": grant["granted_not_spent"]["token_cap"],
        "visual": grant["previous_actual"]["visual_passes"],
        "visual_cap": grant["granted_not_spent"]["visual_cap"],
        "visual_this_packet": 0,
        "visual_max": 5,
        "reserved": counts["reserved"],
        "actual_captures": counts["actual_captures"],
        "settled": [],
        "runtime_sha256": RUNTIME_SHA,
    }


def write_state(state):
    STATE.write_text(json.dumps(state, indent=2) + "\n")


def stage_ceiling(stage):
    for row in ceilings()["stages"]:
        if row["id"] == stage:
            return row["ceiling"]
    raise KeyError(stage)


def bind_stage_b(envelope):
    if not INVENTORY.exists():
        raise ValueError("stage B waits for a code-validated Stage A receipt")
    bound = bind_inventory(json.loads(INVENTORY.read_text()), load("stage-a-birch-request.json"))
    envelope = json.loads(json.dumps(envelope))
    envelope["request"]["inventory"] = bound
    return envelope


def dispatch(envelope, model="gpt-6-astra", effort="medium"):
    return subprocess.run(
        ["python3", str(ADAPTER), "--model", model, "--effort", effort],
        input=json.dumps(envelope).encode(),
        capture_output=True,
        cwd=str(PROOF.parents[2]),
        timeout=330,
    )


def execute(stage):
    release = release_path()
    if release is None:
        print("execute refused: host review not released", file=sys.stderr)
        return 2
    require_runtime()
    if stage not in ENVELOPES:
        print(f"execute refused: {stage} is a template or not a visual stage", file=sys.stderr)
        return 2
    if stage == "r7-current":
        print("execute refused: r7-current waits on both Stage B settlements", file=sys.stderr)
        return 2
    state = state_or_init()
    if state["visual_this_packet"] >= state["visual_max"] or state["visual"] >= state["visual_cap"]:
        print("execute refused: visual ceiling", file=sys.stderr)
        return 2
    prepared = prepare_envelope(ENVELOPES[stage])
    envelope = prepared["envelope"]
    if stage == "stage-a-birch":
        assert_whole_only_stage_a(envelope)
    if stage == "stage-b-birch-positive":
        assert_whole_only_stage_b(envelope)
        envelope = bind_stage_b(envelope)
        prepared = dict(prepared)
        prepared["envelope"] = envelope
        _, schema, prompt = adapter().prepare(envelope)
        prepared["schema_sha256"] = __import__("hashlib").sha256(json.dumps(schema).encode()).hexdigest()
        prepared["dispatched_prompt_sha256"] = __import__("hashlib").sha256(prompt.encode()).hexdigest()
    ceiling = stage_ceiling(stage)
    if state["tokens"] + ceiling > state["token_cap"]:
        print("execute refused: token ceiling", file=sys.stderr)
        return 2
    reserve_path = PROOF / f"{stage}-reservation.json"
    reservation = {
        "stage": stage,
        "release": str(release),
        "release_sha256": digest(release),
        "ceiling": ceiling,
        "prior_tokens": state["tokens"],
        "model": "gpt-6-astra",
        "effort": "medium",
        "protocol": "reference-first-v1",
        "request_sha256": envelope["request_sha256"],
        "request_file_sha256": prepared["request_file_sha256"],
        "adapter_sha256": prepared["adapter_sha256"],
        "schema_sha256": prepared["schema_sha256"],
        "dispatched_prompt_sha256": prepared["dispatched_prompt_sha256"],
        "status": "reserved_before_dispatch",
        "runtime_sha256": RUNTIME_SHA,
    }
    if reserve_path.exists():
        print("execute refused: reservation already exists; no retry", file=sys.stderr)
        return 2
    reserve_path.write_text(json.dumps(reservation, indent=2) + "\n")
    started = time.monotonic()
    run = dispatch(envelope)
    (PROOF / f"{stage}-stdout.json").write_bytes(run.stdout)
    (PROOF / f"{stage}-stderr.txt").write_bytes(run.stderr)
    if run.returncode != 0:
        print("execute failed: adapter nonzero; reservation retained; no retry", file=sys.stderr)
        return 2
    receipt = json.loads(run.stdout)
    used = None
    if receipt.get("usage") and isinstance(receipt["usage"].get("input_tokens"), int):
        used = receipt["usage"]["input_tokens"] + receipt["usage"]["output_tokens"]
    if used is None:
        print("execute failed: unknown usage; reservation retained; stop", file=sys.stderr)
        return 2
    if used > ceiling or receipt.get("status") != "ok":
        print("execute failed: over ceiling or adapter status; stop", file=sys.stderr)
        return 2
    if receipt.get("model") != "gpt-6-astra" or receipt.get("effort") != "medium":
        print("execute failed: model or effort mismatch; stop", file=sys.stderr)
        return 2
    if receipt.get("request_sha256") != envelope["request_sha256"]:
        print("execute failed: request hash mismatch; stop", file=sys.stderr)
        return 2
    if stage == "stage-b-beech-negative":
        judged = judge_negative(receipt.get("answer") or {})
        (PROOF / "stage-b-beech-negative-judgment.json").write_text(json.dumps(judged, indent=2) + "\n")
        if judged["status"] != "negative_calibration_ok":
            print(f"execute stopped: {judged['status']}", file=sys.stderr)
            return 2
    if stage == "stage-a-birch":
        bound = bind_inventory(receipt, load("stage-a-birch-request.json"))
        if INVENTORY.exists():
            print("execute failed: inventory already bound; no overwrite", file=sys.stderr)
            return 2
        INVENTORY.write_text(json.dumps(bound, indent=2) + "\n")
    state["tokens"] += used
    state["visual"] += 1
    state["visual_this_packet"] += 1
    state["settled"].append(stage)
    state["status"] = "stage_settled"
    write_state(state)
    print(json.dumps({"stage": stage, "used": used, "elapsed": time.monotonic() - started, "visual": state["visual"]}))
    return 0


def dry():
    require_runtime()
    assert_whole_only_stage_a(load("stage-a-birch-request.json"))
    assert_whole_only_stage_b(load("stage-b-birch-positive-request.json"))
    assert not INVENTORY.exists(), "placeholder inventory must not exist"
    prepared = {name: prepare_envelope(ENVELOPES[name]) for name in ENVELOPES}
    out = {
        "status": "dry-ok",
        "release": None if release_path() is None else str(release_path()),
        "execute_without_release": "exit 2",
        "accounting": accounting(),
        "sum_stage_ceilings": ceilings()["sum_stage_ceilings"],
        "prepared": {
            name: {
                "images": len(row["images"]),
                "prompt_bytes": len(row["prompt"].encode()),
                "schema_bytes": len(json.dumps(row["schema"]).encode()),
                "schema_sha256": row["schema_sha256"],
                "dispatched_prompt_sha256": row["dispatched_prompt_sha256"],
            }
            for name, row in prepared.items()
        },
    }
    print(json.dumps(out, indent=2))
    return 0


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--execute", action="store_true")
    parser.add_argument("--stage", choices=list(ENVELOPES) + ["r7-final"])
    args = parser.parse_args()
    if args.execute:
        if not args.stage:
            print("execute refused: --stage required", file=sys.stderr)
            return 2
        return execute(args.stage)
    return dry()


if __name__ == "__main__":
    sys.exit(main())
