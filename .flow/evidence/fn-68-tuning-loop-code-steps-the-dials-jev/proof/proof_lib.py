"""Deterministic proof guards. No model call lives here."""
import hashlib
import importlib.util
import json
from pathlib import Path

PROOF = Path(__file__).resolve().parent
ROOT = PROOF.parent
WORKTREE = ROOT.parents[2]
ADAPTER = WORKTREE / "scripts" / "reference-first-codex.py"
RUNTIME = WORKTREE / ".flow/tmp/fn68-pilot-run/run.json"
RUNTIME_SHA = "d93259cd3e6980a19b09c3a1644d9f6114012ad7345bb0c02670941f151e885e"
RELEASE_CANDIDATES = (
    WORKTREE / ".flow/tmp/cursor-fn68-release.json",
    PROOF / "HOST-RELEASE.json",
)
WHOLE_REF = "cae1627afa33d8a881fe927090b222a37fc0dfa2febffa3e3f1edcc52aa309be"
INVENTORY_PROMPT_SHA = "b977396bce80653bb49b01ba0711ae5f59325eab09c2c5a681f87fff43ab3911"
ALLOWED_STAGE_A_VIEWS = frozenset({"S-WHOLE"})
VISUAL_STAGES = (
    "stage-a-birch",
    "stage-b-birch-positive",
    "stage-b-beech-negative",
    "r7-current",
    "r7-final",
)


def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def load(name):
    return json.loads((PROOF / name).read_text())


def adapter():
    spec = importlib.util.spec_from_file_location("adapter", ADAPTER)
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


def release_path():
    for path in RELEASE_CANDIDATES:
        if path.exists():
            return path
    return None


def require_runtime():
    if digest(RUNTIME) != RUNTIME_SHA:
        raise ValueError("original runtime SHA changed")


def prepare_envelope(name):
    envelope = load(name)
    paths, schema, prompt = adapter().prepare(envelope)
    return {
        "name": name,
        "envelope": envelope,
        "paths": [str(p) for p in paths],
        "schema": schema,
        "prompt": prompt,
        "prompt_sha256": hashlib.sha256(envelope["prompt"].encode()).hexdigest(),
        "dispatched_prompt_sha256": hashlib.sha256(prompt.encode()).hexdigest(),
        "schema_sha256": hashlib.sha256(json.dumps(schema).encode()).hexdigest(),
        "request_file_sha256": digest(PROOF / name),
        "adapter_sha256": digest(ADAPTER),
        "images": [{"path": str(p), "sha256": hashlib.sha256(p.read_bytes()).hexdigest()} for p in paths],
    }


def assert_whole_only_stage_a(envelope):
    refs = envelope["request"]["references"]
    if len(refs) != 1:
        raise ValueError("stage A must have exactly one whole-view reference")
    image = refs[0]["image"]
    if image["view"] != "S-WHOLE" or image["sha256"] != WHOLE_REF:
        raise ValueError("stage A reference is not the S-WHOLE photo")
    if refs[0]["id"] != "reference-0":
        raise ValueError("stage A reference id must be reference-0")
    if envelope["prompt_sha256"] != INVENTORY_PROMPT_SHA:
        raise ValueError("generic inventory prompt changed")


def assert_whole_only_stage_b(envelope):
    comparison = envelope["request"]["comparison"]
    views = {r["view"] for r in comparison["references"]}
    if views != {"S-WHOLE"}:
        raise ValueError("stage B positive references must be S-WHOLE only")
    if any(i["view"] != "S-WHOLE" for i in comparison["images"]):
        raise ValueError("stage B positive render must be S-WHOLE")
    if any(inp["view"] == "S-BARE" for inp in comparison["joint"]["inputs"]):
        raise ValueError("stage B joint still contains S-BARE")
    if envelope["request"]["inventory"] is not None:
        raise ValueError("stage B inventory placeholder is not a receipt")


def bind_inventory(receipt, stage_a_envelope):
    """Accept only a code-validated Stage A receipt. Never a fabricated placeholder."""
    if not isinstance(receipt, dict):
        raise ValueError("inventory receipt missing")
    frozen = stage_a_envelope["request"]
    if receipt.get("request") not in (None, frozen):
        raise ValueError("inventory receipt request does not match frozen Stage A")
    if receipt.get("request_sha256") != stage_a_envelope["request_sha256"]:
        raise ValueError("inventory receipt hash does not match Stage A")
    if receipt.get("status") not in (None, "ok"):
        raise ValueError("inventory receipt is not ok")
    traits = receipt.get("traits") or (receipt.get("answer") or {}).get("traits")
    if not traits:
        raise ValueError("inventory receipt has no traits")
    allowed = {"reference-0"}
    cores = 0
    for trait in traits:
        ids = set(trait.get("reference_ids") or [])
        if not ids or not ids <= allowed:
            raise ValueError("inventory trait cites a view outside whole-only Stage A")
        if trait.get("priority") == "core":
            cores += 1
    if cores < 1:
        raise ValueError("inventory receipt has no core trait")
    return {
        "request": frozen,
        "request_sha256": stage_a_envelope["request_sha256"],
        "prompt_sha256": receipt.get("prompt_sha256") or INVENTORY_PROMPT_SHA,
        "traits": traits,
        "observations": receipt.get("observations")
        or (receipt.get("answer") or {}).get("observations")
        or [],
        "model": receipt.get("model"),
        "effort": receipt.get("effort"),
        "ledger": receipt.get("ledger") or receipt.get("receipt_path"),
    }


def judge_negative(answer):
    """Known-defect FAIL with a grounded finding. UNKNOWN-only is reported, not success."""
    passes = answer.get("passes") or []
    findings = answer.get("findings") or []
    grounded = [
        f
        for f in findings
        if f.get("impact") == "blocker"
        and f.get("evidence_ids")
        and not _clipping_only(f)
    ]
    if "fail" in passes and grounded:
        return {"status": "negative_calibration_ok", "grounded_findings": len(grounded)}
    if passes and all(p == "unknown" for p in passes):
        return {
            "status": "abstention_not_successful_negative",
            "reason": "UNKNOWN-only or clipping abstention is not negative calibration",
        }
    if grounded == [] and any(_clipping_only(f) for f in findings) and "fail" not in passes:
        return {
            "status": "abstention_not_successful_negative",
            "reason": "clipping-only abstention is not negative calibration",
        }
    if "pass" in passes and "fail" not in passes:
        return {"status": "false_ready", "reason": "known-negative returned pass"}
    return {"status": "negative_calibration_failed", "reason": "no grounded fail finding"}


def _clipping_only(finding):
    text = (finding.get("observation") or "") + (finding.get("explanation") or "")
    return "clip" in text.lower() and finding.get("uncertain") is True


def accounting():
    return {
        "correction": "prior actual captures were 24, not 20; reserved 28 was already correct",
        "prior_actual_captures": 24,
        "prior_reserved": 28,
        "this_birch_captures": 2,
        "actual_captures": 26,
        "reserved": 30,
        "image_cap": 52,
        "remaining_reservations": 22,
        "original_runtime_reservations": 16,
        "side_import_reservations": 14,
        "original_runtime_sha256": RUNTIME_SHA,
        "applied_to_runtime": False,
        "no_reset": True,
    }
