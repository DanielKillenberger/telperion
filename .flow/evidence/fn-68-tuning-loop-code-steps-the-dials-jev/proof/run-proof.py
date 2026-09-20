#!/usr/bin/env python3
"""Evidence-only staged runner. Calls the existing adapter only after host release."""
import argparse
import json
import subprocess
import sys
import time
from pathlib import Path

from proof_lib import (
    ADAPTER,
    PROOF,
    RUNTIME,
    RUNTIME_SHA,
    WORKTREE,
    accounting,
    adapter,
    assert_whole_only_stage_a,
    assert_whole_only_stage_b,
    bind_inventory,
    digest,
    judge_negative,
    judge_positive,
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
    "stage-b-beech-negative-blind": "stage-b-beech-negative-blind-request.json",
    "r7-current": "stage-r7-current-request.json",
}
REPLACEMENT_RELEASE = WORKTREE / ".flow/tmp/cursor-fn68-replacement-release.json"
R7_RELEASE = WORKTREE / ".flow/tmp/cursor-fn68-r7-release.json"
RESUME = PROOF / "authorization-resume.json"
QUALIFICATION = PROOF / "host-qualification-blind-negative.json"


def ceilings():
    return load("ceilings.json")


def fresh_state():
    grant = load("owner-grant.json")
    counts = accounting()
    return {
        "status": "pending_host_release",
        "tokens": grant["previous_actual"]["tokens"],
        "token_cap": grant["granted_not_spent"]["token_cap"],
        "visual": grant["previous_actual"]["visual_passes"],
        "visual_cap": grant["granted_not_spent"]["visual_cap"],
        "visual_this_packet": 0,
        "visual_max": 6,
        "reserved": counts["reserved"],
        "actual_captures": counts["actual_captures"],
        "settled": [],
        "attempts": [],
        "terminal": None,
        "outstanding_reservation": None,
        "runtime_sha256": RUNTIME_SHA,
    }


def state_or_init():
    if STATE.exists():
        return json.loads(STATE.read_text())
    return fresh_state()


def write_state(state):
    STATE.write_text(json.dumps(state, indent=2) + "\n")


def stage_ceiling(stage):
    for row in ceilings()["stages"]:
        if row["id"] == stage:
            return row["ceiling"]
    raise KeyError(stage)


def replacement_released(stage):
    if not RESUME.exists() or not REPLACEMENT_RELEASE.exists():
        return False
    auth = json.loads(RESUME.read_text())
    release = json.loads(REPLACEMENT_RELEASE.read_text())
    return (
        auth.get("quotation") == "YES"
        and auth.get("stage") == stage
        and release.get("stage") == stage
        and release.get("retries") is False
    )


def r7_released():
    if not R7_RELEASE.exists() or not QUALIFICATION.exists():
        return False
    release = json.loads(R7_RELEASE.read_text())
    qual = json.loads(QUALIFICATION.read_text())
    return (
        release.get("stage") == "r7-current"
        and release.get("retries") is False
        and qual.get("qualification", {}).get("negative") == "FAIL"
        and qual.get("semantic_qualification") is True
    )


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


def known_usage(receipt):
    usage = receipt.get("usage") if isinstance(receipt, dict) else None
    if not usage:
        return None
    inp, out = usage.get("input_tokens"), usage.get("output_tokens")
    if type(inp) is int and type(out) is int and inp >= 0 and out >= 0:
        return inp + out
    return None


def charge(state, stage, used, receipt):
    state["tokens"] += used
    state["visual"] += 1
    state["visual_this_packet"] += 1
    state["attempts"].append(
        {
            "stage": stage,
            "used": used,
            "model": receipt.get("model"),
            "effort": receipt.get("effort"),
            "status": receipt.get("status"),
        }
    )
    state["outstanding_reservation"] = None


def terminate(state, stage, reason):
    state["terminal"] = {"stage": stage, "reason": reason}
    state["status"] = "terminal"


def execute(stage, **paths):
    state_file = Path(paths["state"]) if paths.get("state") else STATE
    inventory_file = Path(paths["inventory"]) if paths.get("inventory") else INVENTORY
    reserve_path = Path(paths["reserve"]) if paths.get("reserve") else PROOF / f"{stage}-reservation.json"
    stdout_path = Path(paths["stdout"]) if paths.get("stdout") else PROOF / f"{stage}-stdout.json"
    stderr_path = Path(paths["stderr"]) if paths.get("stderr") else PROOF / f"{stage}-stderr.txt"
    judgment_path = Path(paths["judgment"]) if paths.get("judgment") else PROOF / f"{stage}-judgment.json"

    def load_state():
        if state_file.exists():
            return json.loads(state_file.read_text())
        if state_file == STATE:
            return state_or_init()
        return fresh_state()

    def save_state(state):
        state_file.write_text(json.dumps(state, indent=2) + "\n")

    release = release_path()
    if release is None:
        print("execute refused: host review not released", file=sys.stderr)
        return 2
    require_runtime()
    if stage not in ENVELOPES:
        print(f"execute refused: {stage} is a template or not a visual stage", file=sys.stderr)
        return 2
    if stage == "stage-b-beech-negative":
        print("execute refused: contaminated request is immutable; use stage-b-beech-negative-blind", file=sys.stderr)
        return 2
    if stage == "r7-current" and not r7_released():
        print("execute refused: r7 waits on host inspection of comparison findings", file=sys.stderr)
        return 2
    state = load_state()
    if state.get("terminal"):
        if stage == "stage-b-beech-negative-blind" and not replacement_released(stage):
            print("execute refused: replacement not released", file=sys.stderr)
            return 2
        if stage == "r7-current" and r7_released():
            pass
        elif stage != "stage-b-beech-negative-blind" or not replacement_released(stage):
            print(f"execute refused: terminal {state['terminal']['reason']}", file=sys.stderr)
            return 2
    if state.get("outstanding_reservation"):
        print("execute refused: outstanding unknown-usage reservation blocks later stages", file=sys.stderr)
        return 2
    if state["visual_this_packet"] >= state["visual_max"] or state["visual"] >= state["visual_cap"]:
        print("execute refused: visual ceiling", file=sys.stderr)
        return 2
    prepared = prepare_envelope(ENVELOPES[stage])
    envelope = prepared["envelope"]
    if stage == "stage-a-birch":
        assert_whole_only_stage_a(envelope)
    if stage == "stage-b-birch-positive":
        assert_whole_only_stage_b(envelope)
        if not inventory_file.exists():
            print("execute refused: stage B waits for a code-validated Stage A receipt", file=sys.stderr)
            return 2
        bound = bind_inventory(json.loads(inventory_file.read_text()), load("stage-a-birch-request.json"))
        envelope = json.loads(json.dumps(envelope))
        envelope["request"]["inventory"] = bound
        _, schema, prompt = adapter().prepare(envelope)
        prepared = dict(prepared)
        prepared["envelope"] = envelope
        prepared["schema_sha256"] = __import__("hashlib").sha256(json.dumps(schema).encode()).hexdigest()
        prepared["dispatched_prompt_sha256"] = __import__("hashlib").sha256(prompt.encode()).hexdigest()
    ceiling = stage_ceiling(stage)
    if state["tokens"] + ceiling > state["token_cap"]:
        print("execute refused: token ceiling", file=sys.stderr)
        return 2
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
    stdout_path.write_bytes(run.stdout)
    stderr_path.write_bytes(run.stderr)
    print(json.dumps({"raw_stdout": str(stdout_path), "raw_stderr": str(stderr_path)}), flush=True)
    receipt = None
    try:
        receipt = json.loads(run.stdout)
    except json.JSONDecodeError:
        receipt = {}
    used = known_usage(receipt)
    if used is None:
        state["outstanding_reservation"] = stage
        terminate(state, stage, "unknown_usage")
        save_state(state)
        print("execute failed: unknown usage; reservation outstanding; later stages blocked", file=sys.stderr)
        return 2
    charge(state, stage, used, receipt)
    save_state(state)
    reason = None
    if run.returncode != 0:
        reason = "adapter_nonzero"
    elif used > ceiling:
        reason = "over_ceiling"
    elif receipt.get("status") != "ok":
        reason = "invalid_receipt"
    elif receipt.get("model") != "gpt-6-astra" or receipt.get("effort") != "medium":
        reason = "model_or_effort"
    elif receipt.get("request_sha256") != envelope["request_sha256"]:
        reason = "request_hash"
    if reason:
        terminate(state, stage, reason)
        save_state(state)
        print(f"execute failed: {reason}; known usage charged; later stages blocked", file=sys.stderr)
        return 2
    if stage == "stage-b-birch-positive":
        judged = judge_positive(
            receipt.get("answer") or {},
            envelope["request"]["comparison"]["required"],
        )
        judgment_path.write_text(json.dumps(judged, indent=2) + "\n")
        if judged["status"] != "positive_calibration_ok":
            terminate(state, stage, judged["status"])
            save_state(state)
            print(f"execute stopped: {judged['status']}; later stages blocked", file=sys.stderr)
            return 2
    if stage in ("stage-b-beech-negative", "stage-b-beech-negative-blind"):
        judged = judge_negative(receipt.get("answer") or {})
        judgment_path.write_text(json.dumps(judged, indent=2) + "\n")
        if judged["status"] != "negative_code_guard_ok":
            terminate(state, stage, judged["status"])
            save_state(state)
            print(f"execute stopped: {judged['status']}; later stages blocked", file=sys.stderr)
            return 2
    if stage == "stage-a-birch":
        try:
            bound = bind_inventory(receipt, load("stage-a-birch-request.json"))
        except ValueError as err:
            terminate(state, stage, "invalid_inventory")
            save_state(state)
            print(f"execute failed: {err}; known usage charged; later stages blocked", file=sys.stderr)
            return 2
        if inventory_file.exists():
            terminate(state, stage, "inventory_already_bound")
            save_state(state)
            print("execute failed: inventory already bound; no overwrite", file=sys.stderr)
            return 2
        inventory_file.write_text(json.dumps(bound, indent=2) + "\n")
    state["settled"].append(stage)
    state["status"] = "stage_settled"
    save_state(state)
    print(json.dumps({"stage": stage, "used": used, "elapsed": time.monotonic() - started, "visual": state["visual"]}))
    return 0


def dry():
    require_runtime()
    assert_whole_only_stage_a(load("stage-a-birch-request.json"))
    assert_whole_only_stage_b(load("stage-b-birch-positive-request.json"))
    if INVENTORY.exists():
        bind_inventory(json.loads(INVENTORY.read_text()), load("stage-a-birch-request.json"))
    prepared = {
        name: prepare_envelope(ENVELOPES[name])
        for name in ENVELOPES
        if name != "stage-b-beech-negative"
    }
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
