"""Vision dispatch through the Claude CLI. Stream-json in, stream-json out.

One call, no tools, one image set inline as base64 in a single stdin user
message (see .flow/evidence/fn-68-tuning-loop-code-steps-the-dials-jev/
claude-comparison.py lines 120-137 for the invocation shape this mirrors).
Never prints or persists the base64 payload, never reads an environment
secret: authentication is the `claude` CLI's own concern.
"""
import base64
import json
import mimetypes
import subprocess
from pathlib import Path

_IMAGE_MEDIA_TYPES = ("image/png", "image/jpeg")


def _image_block(path: Path) -> dict:
    media_type = mimetypes.guess_type(str(path))[0]
    if media_type not in _IMAGE_MEDIA_TYPES:
        raise ValueError(f"unsupported image media type for {path}")
    return {"type": "image", "source": {"type": "base64", "media_type": media_type,
                                         "data": base64.b64encode(path.read_bytes()).decode()}}


def run(model, effort, paths, prompt, schema, timeout=600) -> dict:
    content = [{"type": "text", "text": prompt}] + [_image_block(Path(p)) for p in paths]
    message = {"type": "user", "message": {"role": "user", "content": content}, "parent_tool_use_id": None}
    command = ["claude", "-p", "--safe-mode", "--tools", "", "--strict-mcp-config",
               "--no-session-persistence", "--input-format", "stream-json", "--output-format", "stream-json",
               "--verbose", "--model", model, "--effort", effort, "--json-schema", json.dumps(schema)]
    completed = subprocess.run(command, input=json.dumps(message) + "\n", text=True,
                                capture_output=True, timeout=timeout)
    usage = None
    answer = None
    actual_model = None
    forbidden_tools = []
    result_event = None
    texts = []
    for line in completed.stdout.splitlines():
        line = line.strip()
        if not line:
            continue
        try:
            event = json.loads(line)
        except json.JSONDecodeError:
            continue
        if event.get("type") == "assistant":
            for block in event.get("message", {}).get("content", []):
                if block.get("type") == "tool_use":
                    forbidden_tools.append(block.get("name", "tool_use"))
                elif block.get("type") == "text":
                    texts.append(block.get("text", ""))
        elif event.get("type") == "result":
            result_event = event
    if result_event is not None and not result_event.get("is_error") and result_event.get("subtype") == "success":
        raw_usage = result_event.get("usage") or {}
        if (type(raw_usage.get("input_tokens")) is int and raw_usage["input_tokens"] >= 0
                and type(raw_usage.get("output_tokens")) is int and raw_usage["output_tokens"] >= 0):
            usage = {"input_tokens": raw_usage["input_tokens"], "output_tokens": raw_usage["output_tokens"]}
        models = list(result_event.get("modelUsage", {}))
        if len(models) == 1 and models[0]:
            actual_model = models[0]
        answer = result_event.get("structured_output")
        if answer is None and texts:
            try:
                answer = json.loads(texts[-1])
            except json.JSONDecodeError:
                answer = None
    return {"returncode": completed.returncode, "usage": usage, "answer": answer,
            "actual_model": actual_model, "forbidden_tools": forbidden_tools,
            "raw_events": completed.stdout, "stderr": completed.stderr}
