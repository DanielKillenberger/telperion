"""Evidence-only Fable/Opus comparison. Default preparation never launches a model."""
import argparse
import base64
import contextlib
import hashlib
import io
import json
import mimetypes
from pathlib import Path
import runpy
import subprocess
import sys
import tempfile
from unittest.mock import patch

ROOT = Path(__file__).resolve().parent
WORKTREE = ROOT.parents[2]
REQUEST_HASH = "c82e452cf172c97481433192b34db4833d9213abca5499055d45c196e8275f87"
PROTOCOL_HASH = "d3d17c0f98d48a9e2543807127bc414d3bf7fedd0c605325359a7be9f1c137cb"


def digest(data):
    return hashlib.sha256(data).hexdigest()


def frozen_payload():
    script = WORKTREE / "scripts/tuning-vision-codex.py"
    assert digest(script.read_bytes()) == PROTOCOL_HASH
    request = json.loads((ROOT / "joint-blind-request.json").read_text())
    audit = json.loads((ROOT / "joint-blind-audit.json").read_text())
    assert audit["request_sha256"] == REQUEST_HASH
    assert digest((ROOT / "joint-blind-request.json").read_bytes()) == audit["file_sha256"]
    captured = {}

    def dry_run(command, **kwargs):
        captured["prompt"] = command[-1]
        captured["schema"] = json.loads(Path(command[command.index("--output-schema") + 1]).read_text())
        captured["paths"] = [command[i + 1] for i, arg in enumerate(command) if arg == "--image"]
        Path(command[command.index("-o") + 1]).write_text(json.dumps({
            "passes": ["unknown"] * len(request["required"]), "defects": [], "observations": [], "findings": []}))
        return subprocess.CompletedProcess(command, 0, b'{"type":"turn.completed","usage":{"input_tokens":0,"output_tokens":0}}', b"")

    with patch.object(sys, "argv", ["adapter", "--model", "gpt-5.6-sol", "--effort", "medium"]), \
         patch.object(sys, "stdin", io.StringIO(json.dumps({"request": request, "request_sha256": REQUEST_HASH}))), \
         patch("subprocess.run", side_effect=dry_run) as mocked, contextlib.redirect_stdout(io.StringIO()):
        runpy.run_path(str(script), run_name="__main__")
    assert mocked.call_count == 1
    images = request["images"] + request["references"] + [a["image"] for a in request["quality_anchors"]]
    assert len(images) == len(captured["paths"]) == 5
    for image, path in zip(images, captured["paths"]):
        assert Path(image["path"]).resolve() == Path(path)
        assert digest(Path(path).read_bytes()) == image["sha256"]
    return request, captured


def parse_result(stdout):
    events = [json.loads(line) for line in stdout.splitlines() if line.strip()]
    results = [event for event in events if event.get("type") == "result"]
    if len(results) != 1 or results[0].get("is_error") or results[0].get("subtype") != "success":
        raise ValueError("missing successful terminal result; retain reservation")
    final = results[0]
    usage = final.get("usage", {})
    fields = ("input_tokens", "cache_creation_input_tokens", "cache_read_input_tokens", "output_tokens")
    if any(type(usage.get(k)) is not int or usage[k] < 0 for k in fields):
        raise ValueError("unknown usage; retain reservation")
    models = list(final.get("modelUsage", {}))
    if len(models) != 1 or not models[0]:
        raise ValueError("ambiguous actual model; retain reservation")
    answer = final.get("structured_output")
    if answer is None:
        texts = [block["text"] for event in events if event.get("type") == "assistant"
                 for block in event.get("message", {}).get("content", []) if block.get("type") == "text"]
        if not texts:
            raise ValueError("no structured output or assistant text")
        answer = json.loads(texts[-1])
    for key in ("passes", "defects", "observations", "findings"):
        if not isinstance(answer.get(key), list):
            raise ValueError("invalid structured answer")
    return {"answer": answer, "actual_model": models[0], "usage": {k: usage[k] for k in fields},
            "actual_tokens": sum(usage[k] for k in fields), "model_usage": final["modelUsage"]}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--model", choices=["fable", "opus"], required=True)
    parser.add_argument("--authority", type=Path)
    args = parser.parse_args()
    effort = "medium" if args.model == "fable" else "high"
    request, payload = frozen_payload()
    prompt_hash = digest(payload["prompt"].encode())
    schema_bytes = json.dumps(payload["schema"], sort_keys=True).encode()
    manifest = {"requested_model": args.model, "effort": effort, "request_sha256": REQUEST_HASH,
                "protocol_sha256": PROTOCOL_HASH, "user_prompt_sha256": prompt_hash,
                "output_schema_sha256": digest(schema_bytes),
                "images": [{"path": p, "sha256": digest(Path(p).read_bytes())} for p in payload["paths"]],
                "full_system_prompt_identical": False,
                "system_difference": "Provider CLI/system scaffolding differs; exact shared user prompt/schema/image bytes only.",
                "status": "prepared_no_call"}
    if args.authority is None:
        print(json.dumps(manifest))
        return
    authority = json.loads(args.authority.read_text())
    assert authority["scope"] == "one-frozen-claude-comparison"
    assert authority["requested_model"] == args.model and authority["effort"] == effort
    assert authority["request_sha256"] == REQUEST_HASH and authority["view_issue_resolved"] is True
    assert authority["prior_tokens"] >= 380304 and authority["reserved_tokens"] == 40000
    assert authority["prior_tokens"] + 40000 <= authority["cumulative_cap"]
    assert authority["owner_authorization"].strip()
    prior_bytes=Path(authority["prior_journal"]).read_bytes()
    assert digest(prior_bytes)==authority["prior_journal_sha256"]
    assert json.loads(prior_bytes)["cumulative_actual_tokens"]==authority["prior_tokens"]
    journal = ROOT / f"joint-blind-{args.model}-journal.json"
    manifest.update(authority=authority, status="reserved_before_dispatch")
    runtime=WORKTREE/".flow/tmp/fn68-pilot-run/run.json"
    manifest["original_runtime_sha256"]=digest(runtime.read_bytes())
    with journal.open("x") as handle:
        json.dump(manifest, handle, indent=2)
    # No input base64 is persisted or printed. Raw provider response stays in ignored evidence.
    content = [{"type": "text", "text": payload["prompt"]}]
    for path in payload["paths"]:
        media = mimetypes.guess_type(path)[0]
        assert media in ("image/png", "image/jpeg")
        content.append({"type": "image", "source": {"type": "base64", "media_type": media,
                        "data": base64.b64encode(Path(path).read_bytes()).decode()}})
    message = {"type": "user", "message": {"role": "user", "content": content}, "parent_tool_use_id": None}
    with tempfile.TemporaryDirectory(prefix="fn68-claude-blind-") as scratch:
        command = ["claude", "-p", "--safe-mode", "--tools", "", "--strict-mcp-config",
                   "--no-session-persistence", "--input-format", "stream-json", "--output-format", "stream-json",
                   "--verbose", "--model", args.model, "--effort", effort,
                   "--json-schema", json.dumps(payload["schema"])]
        result = subprocess.run(command, input=json.dumps(message) + "\n", text=True,
                                capture_output=True, cwd=scratch, timeout=300)
    raw = ROOT / "local" / f"joint-blind-{args.model}-stdout.jsonl"
    with raw.open("x") as handle:
        handle.write(result.stdout)
    (ROOT / "local" / f"joint-blind-{args.model}-stderr.txt").write_text(result.stderr)
    if result.returncode:
        raise RuntimeError("provider failed; preserve reservation and raw receipt")
    parsed = parse_result(result.stdout)
    assert len(parsed["answer"]["passes"]) == len(request["required"])
    assert all(value in ("pass","fail","unknown") for value in parsed["answer"]["passes"])
    ids={i["id"] for i in request["joint"]["inputs"]}
    for finding in parsed["answer"]["findings"]:
        refs=finding["evidence_ids"]
        assert refs and len(refs)==len(set(refs)) and set(refs)<=ids
        assert finding["impact"] in ("supported","blocker","required_unknown","variation","optional")
        assert type(finding["uncertain"]) is bool and finding["observation"].strip()
    assert digest(runtime.read_bytes())==manifest["original_runtime_sha256"]
    manifest.update(parsed, cumulative_actual_tokens=authority["prior_tokens"] + parsed["actual_tokens"],
                    raw_response_sha256=digest(result.stdout.encode()),
                    status="settled" if parsed["actual_tokens"] <= 40000 else "over_reservation_stop")
    journal.write_text(json.dumps(manifest, indent=2))
    print(json.dumps({k: manifest[k] for k in ("status", "actual_model", "usage", "actual_tokens", "cumulative_actual_tokens")}))
    assert parsed["actual_tokens"]<=40000,"over reservation: stop"


if __name__ == "__main__":
    main()
