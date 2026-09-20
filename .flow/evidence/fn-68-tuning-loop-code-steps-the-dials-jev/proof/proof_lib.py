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
    "stage-b-beech-negative-blind",
    "r7-current",
    "r7-final",
)
GRADER_ENVELOPE_KEYS = frozenset(
    {
        "expected_ready",
        "label",
        "grader",
        "expected",
        "known_negative",
        "previous_verdict",
        "authorization",
    }
)
FORBIDDEN_PAYLOAD_MARKERS = (
    "known negative",
    "known-negative",
    "false-ready",
    "false ready",
    "expected_ready",
    "owner-rejected",
    "previous verdict",
    "owner answers",
)
BLIND_IMAGES = PROOF / "blind-images"
NEUTRAL_IMAGE_SOURCES = (
    (
        "render-0.png",
        ROOT / "local/replay-images/european-beech-B-WHOLE.png",
        "7e69fda3e95b317fcb9d89e15e32179c54fdd316a57d1c3223d6e26639b8b490",
    ),
    (
        "reference-0.jpg",
        Path("/home/daniel/Projects/telperion/.worktrees/lichen-trial/.refs/fn34/european-beech/fasy951.jpg"),
        "855fddf7d2974aff8f0bd9421223fdec990f9b6b9d229e714e511082e81157e6",
    ),
    (
        "reference-1.jpg",
        Path("/home/daniel/Projects/telperion/.worktrees/lichen-trial/.refs/fn34/european-beech/fasy896.jpg"),
        "7a269a2b43154bf2641fde94aba6853a8d5459d997e1fda29bf2d80733134e1d",
    ),
    (
        "anchor-0.png",
        WORKTREE / ".flow/evidence/fn9/final/preview/norway-spruce-1-whole.png",
        "8e10ac1cf993a4cc005f7a1374e5763274c5dc636f42204e140464242ba027f1",
    ),
)
CONTAMINATED_REQUEST_SHA = "2cb8319c54dd9f61c84e603a8157a5ad4fea64e6e26358d7429783a8267eaed5"
CONTAMINATED_STDOUT_SHA = "3f6af87b0595430887502db2fd1b5d3ae3bdb037d0d46d5d4de1f9c03e77918a"
COMPARISON_PROMPT_SHA = "847dd718e56258447ec5fcc89fdaadc9701165d02eab0b51001e616ebc1a0329"


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


def ensure_neutral_images():
    BLIND_IMAGES.mkdir(exist_ok=True)
    placed = []
    for name, source, expected in NEUTRAL_IMAGE_SOURCES:
        dest = BLIND_IMAGES / name
        if hashlib.sha256(source.read_bytes()).hexdigest() != expected:
            raise ValueError(f"source hash mismatch for {name}")
        if dest.exists() or dest.is_symlink():
            dest.unlink()
        try:
            dest.hardlink_to(source)
        except OSError:
            dest.write_bytes(source.read_bytes())
        if hashlib.sha256(dest.read_bytes()).hexdigest() != expected:
            raise ValueError(f"neutral path hash mismatch for {name}")
        placed.append({"name": name, "path": str(dest), "sha256": expected, "source": str(source)})
    return placed


def strip_grader_keys(envelope):
    visible = json.loads(json.dumps(envelope))
    for key in GRADER_ENVELOPE_KEYS:
        visible.pop(key, None)
    return visible


def payload_leaks(text):
    lowered = text.lower()
    return [marker for marker in FORBIDDEN_PAYLOAD_MARKERS if marker in lowered]


def assert_blind_payload(prompt, schema, paths):
    leaks = payload_leaks(prompt)
    leaks += payload_leaks(json.dumps(schema))
    leaks += payload_leaks(" ".join(str(p) for p in paths))
    if leaks:
        raise ValueError("model-visible payload contains grader labels: " + ", ".join(leaks))


def request_body_sha256(request):
    return hashlib.sha256(json.dumps(request).encode()).hexdigest()


def prepare_envelope(name):
    envelope = load(name)
    if name == "stage-b-beech-negative-blind-request.json":
        ensure_neutral_images()
        envelope = strip_grader_keys(envelope)
    paths, schema, prompt = adapter().prepare(envelope)
    if name == "stage-b-beech-negative-blind-request.json":
        assert_blind_payload(prompt, schema, paths)
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
        "request_body_sha256": request_body_sha256(envelope["request"]),
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


def judge_positive(answer, required):
    """All required cells PASS plus a supported finding that cites render and reference."""
    passes = answer.get("passes") or []
    if len(passes) != len(required) or any(p != "pass" for p in passes):
        return {
            "status": "positive_calibration_failed",
            "reason": "required cells are not all pass",
        }
    coverage = answer.get("coverage") or []
    if coverage and any(c.get("status") != "pass" for c in coverage):
        return {
            "status": "positive_calibration_failed",
            "reason": "coverage is not all pass",
        }
    supported = []
    for finding in answer.get("findings") or []:
        if finding.get("impact") != "supported":
            continue
        ids = finding.get("evidence_ids") or []
        if any(i.startswith("render-") for i in ids) and any(i.startswith("reference-") for i in ids):
            supported.append(finding)
    if not supported:
        return {
            "status": "positive_calibration_failed",
            "reason": "no supported finding citing render and reference",
        }
    return {"status": "positive_calibration_ok", "supported": len(supported)}


def judge_negative(answer):
    """Code guard only. FAIL plus render/reference blocker. Clipping substring is not semantic proof."""
    passes = answer.get("passes") or []
    findings = answer.get("findings") or []
    grounded = [
        f
        for f in findings
        if f.get("impact") == "blocker"
        and _has_render_and_reference(f.get("evidence_ids") or [])
    ]
    if "fail" in passes and grounded:
        return {
            "status": "negative_code_guard_ok",
            "grounded_findings": len(grounded),
            "semantic_qualification": False,
            "clipping_heuristic": "not semantic proof; host inspects raw findings",
        }
    if passes and all(p == "unknown" for p in passes):
        return {
            "status": "abstention_not_successful_negative",
            "reason": "UNKNOWN-only is not negative calibration",
            "semantic_qualification": False,
        }
    if "pass" in passes and "fail" not in passes:
        return {"status": "false_ready", "reason": "known-negative returned pass", "semantic_qualification": False}
    return {
        "status": "negative_calibration_failed",
        "reason": "no fail with render and reference blocker",
        "semantic_qualification": False,
    }


def _has_render_and_reference(ids):
    return any(i.startswith("render-") for i in ids) and any(i.startswith("reference-") for i in ids)


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
