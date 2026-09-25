"""Contract tests for the reviewer adapters: each builds its prompt, schema
and image set with its own `prepare` and dispatches through `claude -p`."""
import base64
import contextlib
import hashlib
import importlib.util
import io
import json
import runpy
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

SCRIPTS = Path(__file__).parent


def load_module(name):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_").replace(".py", ""), SCRIPTS / name)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def run_adapter(script_name, envelope, model="fixture-model", effort="medium", fake_claude=None):
    """Execute an adapter script under runpy, mocking subprocess.run."""
    stdout = io.StringIO()
    calls = []

    def recording_fake(command, **kwargs):
        calls.append({"command": command, "kwargs": kwargs})
        return fake_claude(command, **kwargs)

    with patch.object(sys, "argv", ["adapter", "--model", model, "--effort", effort]), \
         patch.object(sys, "stdin", io.StringIO(json.dumps(envelope))), \
         patch("subprocess.run", side_effect=recording_fake), \
         contextlib.redirect_stdout(stdout):
        runpy.run_path(str(SCRIPTS / script_name), run_name="__main__")
    return json.loads(stdout.getvalue()), calls


def success_claude_fake(structured_output, usage=None, model_id="claude-opus-5"):
    if usage is None:
        usage = {"input_tokens": 11, "output_tokens": 7}

    def fake(command, **kwargs):
        result_event = {"type": "result", "subtype": "success", "is_error": False,
                         "usage": usage, "modelUsage": {model_id: {}}, "structured_output": structured_output}
        return subprocess.CompletedProcess(command, 0, json.dumps(result_event) + "\n", "")
    return fake


def tool_using_claude_fake(structured_output, tool_names):
    """The CLI's real stream: an assistant turn carrying tool_use blocks, then the result."""
    def fake(command, **kwargs):
        blocks = [{"type": "tool_use", "name": name, "input": {}} for name in tool_names]
        assistant = {"type": "assistant", "message": {"role": "assistant", "content": blocks}}
        result_event = {"type": "result", "subtype": "success", "is_error": False,
                         "usage": {"input_tokens": 11, "output_tokens": 7}, "modelUsage": {"claude-opus-5": {}},
                         "structured_output": structured_output}
        return subprocess.CompletedProcess(command, 0, json.dumps(assistant) + "\n" + json.dumps(result_event) + "\n", "")
    return fake


def failing_claude_fake():
    def fake(command, **kwargs):
        result_event = {"type": "result", "subtype": "error_max_turns", "is_error": True}
        return subprocess.CompletedProcess(command, 0, json.dumps(result_event) + "\n", "")
    return fake


def assert_claude_command_and_images(test, command, kwargs, paths, expected_prompt, expected_schema):
    test.assertEqual(command[0], "claude")
    test.assertEqual(command[1], "-p")
    test.assertIn("--tools", command)
    test.assertEqual(command[command.index("--tools") + 1], "")
    test.assertIn("--strict-mcp-config", command)
    test.assertIn("--no-session-persistence", command)
    test.assertEqual(command[command.index("--input-format") + 1], "stream-json")
    test.assertEqual(command[command.index("--output-format") + 1], "stream-json")
    test.assertIn("--json-schema", command)
    schema = json.loads(command[command.index("--json-schema") + 1])
    test.assertEqual(schema, expected_schema)
    message = json.loads(kwargs["input"].strip())
    content = message["message"]["content"]
    test.assertEqual(content[0], {"type": "text", "text": expected_prompt})
    image_blocks = content[1:]
    test.assertEqual(len(image_blocks), len(paths))
    for block, path in zip(image_blocks, paths):
        test.assertEqual(block["type"], "image")
        test.assertEqual(block["source"]["media_type"], "image/png")
        test.assertEqual(base64.b64decode(block["source"]["data"]), Path(path).read_bytes())


class ContactSheetClaude(unittest.TestCase):
    def build_envelope(self, tmp):
        ref = Path(tmp) / "ref.png"; ref.write_bytes(b"ref-bytes")
        r1 = Path(tmp) / "r1.png"; r1.write_bytes(b"render-1")
        r2 = Path(tmp) / "r2.png"; r2.write_bytes(b"render-2")
        request = {"schema": "tuning-sheet-v2", "target_species": "Example", "view": "whole", "seed": 1,
                   "references": [{"path": str(ref), "sha256": sha256(ref.read_bytes()), "role": "reference"}],
                   "renders": [{"path": str(r1), "sha256": sha256(r1.read_bytes())},
                               {"path": str(r2), "sha256": sha256(r2.read_bytes())}],
                   "priorities": [{"id": "p1"}], "owner_notes": ""}
        prompt_request = {"schema": "tuning-sheet-v2", "target_species": "Example", "view": "whole", "seed": 1,
                           "references": [{"role": "reference", "sha256": sha256(ref.read_bytes())}],
                           "renders": [{"label": "1", "sha256": sha256(r1.read_bytes())},
                                       {"label": "2", "sha256": sha256(r2.read_bytes())}],
                           "priorities": [{"id": "p1"}], "owner_notes": ""}
        prompt = "Assess the sheet."
        return {"stage": "sheet", "request": request, "request_sha256": "sheet-hash",
                "prompt": prompt, "prompt_sha256": sha256(prompt.encode()), "prompt_request": prompt_request}

    def test_dispatch_carries_the_prepared_prompt_and_schema(self):
        prepared = load_module("contact-sheet.py")
        with tempfile.TemporaryDirectory() as tmp:
            envelope = self.build_envelope(tmp)
            paths, schema, prompt = prepared.prepare(envelope)
            answer = {"priorities": [{"priority_id": "p1", "closest": "1", "ranking": ["1", "2"],
                                       "steps": [{"from": "1", "to": "2", "grade": "slight"}]}],
                      "overall": ["1", "2"], "wrong": [], "breaks": [], "improved": "", "missing": ""}
            result, calls = run_adapter("contact-sheet.py", envelope, fake_claude=success_claude_fake(answer))
            self.assertEqual(len(calls), 1)
            assert_claude_command_and_images(self, calls[0]["command"], calls[0]["kwargs"], paths, prompt, schema)
            self.assertEqual(result["status"], "ok")
            self.assertEqual(result["answer"], answer)
            self.assertEqual(result["usage"], {"input_tokens": 11, "output_tokens": 7})
            self.assertEqual(result["model"], "fixture-model")
            self.assertIn("claude-opus-5", result["model_identity_basis"])
            self.assertEqual(result["dispatched_prompt_sha256"], sha256(prompt.encode()))
            self.assertEqual(result["schema_sha256"], sha256(json.dumps(schema).encode()))

    def test_structured_output_tool_is_the_answer_channel(self):
        answer = {"priorities": [{"priority_id": "p1", "closest": "1", "ranking": ["1", "2"], "steps": []}],
                  "overall": ["1", "2"], "wrong": [], "breaks": [], "improved": "", "missing": ""}
        with tempfile.TemporaryDirectory() as tmp:
            envelope = self.build_envelope(tmp)
            result, _ = run_adapter("contact-sheet.py", envelope,
                                 fake_claude=tool_using_claude_fake(answer, ["StructuredOutput"]))
            self.assertEqual(result["status"], "ok")
            self.assertEqual(result["forbidden_tools"], [])
            result, _ = run_adapter("contact-sheet.py", envelope,
                                 fake_claude=tool_using_claude_fake(answer, ["StructuredOutput", "Read"]))
            self.assertEqual(result["status"], "failed_or_tools_or_unknown_usage_or_cardinality")
            self.assertEqual(result["forbidden_tools"], ["Read"])

    def test_missing_usage_yields_failure_status(self):
        with tempfile.TemporaryDirectory() as tmp:
            envelope = self.build_envelope(tmp)
            result, calls = run_adapter("contact-sheet.py", envelope, fake_claude=failing_claude_fake())
            self.assertEqual(len(calls), 1)
            self.assertEqual(result["status"], "failed_or_tools_or_unknown_usage_or_cardinality")
            self.assertIsNone(result["usage"])
            self.assertIsNone(result["answer"])


class ReferenceFirstClaude(unittest.TestCase):
    def build_envelope(self, tmp):
        image = Path(tmp) / "reference.png"; image.write_bytes(b"reference")
        request = {"protocol": "reference-first-v1", "target_species": "Example species",
                   "specimen_relationship": "unknown",
                   "references": [{"id": "reference-0", "image": {"path": str(image), "sha256": sha256(b"reference"),
                                                                    "view": "whole", "seed": 0}}]}
        prompt = "Neutral inventory instruction"
        return {"stage": "inventory", "request": request, "request_sha256": "bound-by-rust",
                "prompt": prompt, "prompt_sha256": sha256(prompt.encode())}

    def test_dispatch_carries_the_prepared_prompt_and_schema(self):
        prepared = load_module("reference-first.py")
        with tempfile.TemporaryDirectory() as tmp:
            envelope = self.build_envelope(tmp)
            paths, schema, prompt = prepared.prepare(envelope)
            answer = {"traits": [], "observations": ["literal"]}
            result, calls = run_adapter("reference-first.py", envelope, fake_claude=success_claude_fake(answer))
            self.assertEqual(len(calls), 1)
            assert_claude_command_and_images(self, calls[0]["command"], calls[0]["kwargs"], paths, prompt, schema)
            self.assertEqual(result["status"], "ok")
            self.assertEqual(result["answer"], answer)

    def test_missing_usage_yields_failure_status(self):
        with tempfile.TemporaryDirectory() as tmp:
            envelope = self.build_envelope(tmp)
            result, calls = run_adapter("reference-first.py", envelope, fake_claude=failing_claude_fake())
            self.assertEqual(len(calls), 1)
            self.assertEqual(result["status"], "failed_or_tools_or_unknown_usage_or_cardinality")
            self.assertIsNone(result["usage"])

    def test_repair_is_one_text_only_call_in_the_comparison_schema(self):
        """fn-80: a repair carries the previous answer and the exact broken
        rules, no images, and is answered in the comparison's schema."""
        prepared = load_module("reference-first.py")
        with tempfile.TemporaryDirectory() as tmp:
            inventory = self.build_envelope(tmp)
            image = inventory["request"]["references"][0]["image"]
            request = {"protocol": prepared.COMPARISON_VERSION,
                       "comparison": {"required": [{"view": "whole"}], "images": [image],
                                      "references": [image], "quality_anchors": []}}
            prompt = "Repair instruction"
            previous = {"passes": ["pass"], "findings": [], "coverage": [], "defects": [], "observations": []}
            violation = "finding 17 of 17: at most 16 findings allowed"
            envelope = {"stage": "repair", "request": request, "request_sha256": "bound-by-rust",
                        "prompt": prompt, "prompt_sha256": sha256(prompt.encode()),
                        "answer": previous, "violations": [violation]}
            paths, schema, repair_prompt = prepared.prepare(envelope)
            _, comparison_schema, _ = prepared.prepare(dict(envelope, stage="comparison"))
            self.assertEqual(paths, [])
            self.assertEqual(schema, comparison_schema)
            self.assertIn(violation, repair_prompt)
            self.assertIn(json.dumps(previous), repair_prompt)
            result, calls = run_adapter("reference-first.py", envelope, fake_claude=success_claude_fake(previous))
            self.assertEqual(len(calls), 1)
            assert_claude_command_and_images(self, calls[0]["command"], calls[0]["kwargs"], [], repair_prompt, schema)
            self.assertEqual(result["status"], "ok")
            with self.assertRaises(ValueError):
                prepared.prepare(dict(envelope, violations=[]))


if __name__ == "__main__":
    unittest.main()
