"""Contract tests for the Claude vision twins: same prompt/schema/image set
as the codex twin, dispatched through `claude -p` instead of `codex exec`."""
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


def load_codex_module(name):
    spec = importlib.util.spec_from_file_location(name.replace("-", "_").replace(".py", ""), SCRIPTS / name)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def run_twin(script_name, envelope, model="fixture-model", effort="medium", fake_claude=None):
    """Execute a claude twin script under runpy, mocking subprocess.run."""
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

    def test_dispatch_matches_codex_prompt_and_schema(self):
        codex = load_codex_module("contact-sheet-codex.py")
        with tempfile.TemporaryDirectory() as tmp:
            envelope = self.build_envelope(tmp)
            paths, schema, prompt = codex.prepare(envelope)
            answer = {"priorities": [{"priority_id": "p1", "closest": "1", "ranking": ["1", "2"],
                                       "steps": [{"from": "1", "to": "2", "grade": "slight"}]}],
                      "overall": ["1", "2"], "wrong": [], "breaks": [], "improved": "", "missing": ""}
            result, calls = run_twin("contact-sheet-claude.py", envelope, fake_claude=success_claude_fake(answer))
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
            result, _ = run_twin("contact-sheet-claude.py", envelope,
                                 fake_claude=tool_using_claude_fake(answer, ["StructuredOutput"]))
            self.assertEqual(result["status"], "ok")
            self.assertEqual(result["forbidden_tools"], [])
            result, _ = run_twin("contact-sheet-claude.py", envelope,
                                 fake_claude=tool_using_claude_fake(answer, ["StructuredOutput", "Read"]))
            self.assertEqual(result["status"], "failed_or_tools_or_unknown_usage_or_cardinality")
            self.assertEqual(result["forbidden_tools"], ["Read"])

    def test_missing_usage_yields_failure_status(self):
        with tempfile.TemporaryDirectory() as tmp:
            envelope = self.build_envelope(tmp)
            result, calls = run_twin("contact-sheet-claude.py", envelope, fake_claude=failing_claude_fake())
            self.assertEqual(len(calls), 1)
            self.assertEqual(result["status"], "failed_or_tools_or_unknown_usage_or_cardinality")
            self.assertIsNone(result["usage"])
            self.assertIsNone(result["answer"])


class ProgressReviewClaude(unittest.TestCase):
    def build_envelope(self, tmp):
        ref = Path(tmp) / "pref.png"; ref.write_bytes(b"pref-bytes")
        a_img = Path(tmp) / "a.png"; a_img.write_bytes(b"a-bytes")
        b_img = Path(tmp) / "b.png"; b_img.write_bytes(b"b-bytes")
        request = {"schema": "tuning-progress-v2", "target_species": "Example", "view": "whole", "seed": 2,
                   "references": [{"path": str(ref), "sha256": sha256(ref.read_bytes()), "role": "reference"}],
                   "a": {"path": str(a_img), "sha256": sha256(a_img.read_bytes()), "role": "a"},
                   "b": {"path": str(b_img), "sha256": sha256(b_img.read_bytes()), "role": "b"},
                   "priorities": [{"id": "p1"}], "owner_notes": ""}
        prompt_request = {"schema": "tuning-progress-v2", "target_species": "Example", "view": "whole", "seed": 2,
                           "references": [{"role": "reference", "sha256": sha256(ref.read_bytes())}],
                           "a": {"role": "a", "sha256": sha256(a_img.read_bytes())},
                           "b": {"role": "b", "sha256": sha256(b_img.read_bytes())},
                           "priorities": [{"id": "p1"}], "owner_notes": ""}
        prompt = "Assess progress."
        return {"stage": "progress", "request": request, "request_sha256": "progress-hash",
                "prompt": prompt, "prompt_sha256": sha256(prompt.encode()), "prompt_request": prompt_request}

    def test_dispatch_matches_codex_prompt_and_schema(self):
        codex = load_codex_module("progress-review-codex.py")
        with tempfile.TemporaryDirectory() as tmp:
            envelope = self.build_envelope(tmp)
            paths, schema, prompt = codex.prepare(envelope)
            answer = {"verdicts": [{"priority_id": "p1", "verdict": "a_better"}],
                      "improved": "", "missing": "", "regressions": []}
            result, calls = run_twin("progress-review-claude.py", envelope, fake_claude=success_claude_fake(answer))
            self.assertEqual(len(calls), 1)
            assert_claude_command_and_images(self, calls[0]["command"], calls[0]["kwargs"], paths, prompt, schema)
            self.assertEqual(result["status"], "ok")
            self.assertEqual(result["answer"], answer)
            self.assertEqual(result["usage"], {"input_tokens": 11, "output_tokens": 7})

    def test_missing_answer_yields_failure_status(self):
        with tempfile.TemporaryDirectory() as tmp:
            envelope = self.build_envelope(tmp)
            result, calls = run_twin("progress-review-claude.py", envelope, fake_claude=success_claude_fake(None))
            self.assertEqual(len(calls), 1)
            self.assertNotEqual(result["status"], "ok")
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

    def test_dispatch_matches_codex_prompt_and_schema(self):
        codex = load_codex_module("reference-first-codex.py")
        with tempfile.TemporaryDirectory() as tmp:
            envelope = self.build_envelope(tmp)
            paths, schema, prompt = codex.prepare(envelope)
            answer = {"traits": [], "observations": ["literal"]}
            result, calls = run_twin("reference-first-claude.py", envelope, fake_claude=success_claude_fake(answer))
            self.assertEqual(len(calls), 1)
            assert_claude_command_and_images(self, calls[0]["command"], calls[0]["kwargs"], paths, prompt, schema)
            self.assertEqual(result["status"], "ok")
            self.assertEqual(result["answer"], answer)

    def test_missing_usage_yields_failure_status(self):
        with tempfile.TemporaryDirectory() as tmp:
            envelope = self.build_envelope(tmp)
            result, calls = run_twin("reference-first-claude.py", envelope, fake_claude=failing_claude_fake())
            self.assertEqual(len(calls), 1)
            self.assertEqual(result["status"], "failed_or_tools_or_unknown_usage_or_cardinality")
            self.assertIsNone(result["usage"])

    def test_repair_is_one_text_only_call_in_the_comparison_schema(self):
        """fn-80: a repair carries the previous answer and the exact broken
        rules, no images, and is answered in the comparison's schema."""
        codex = load_codex_module("reference-first-codex.py")
        with tempfile.TemporaryDirectory() as tmp:
            inventory = self.build_envelope(tmp)
            image = inventory["request"]["references"][0]["image"]
            request = {"protocol": codex.COMPARISON_VERSION,
                       "comparison": {"required": [{"view": "whole"}], "images": [image],
                                      "references": [image], "quality_anchors": []}}
            prompt = "Repair instruction"
            previous = {"passes": ["pass"], "findings": [], "coverage": [], "defects": [], "observations": []}
            violation = "finding 17 of 17: at most 16 findings allowed"
            envelope = {"stage": "repair", "request": request, "request_sha256": "bound-by-rust",
                        "prompt": prompt, "prompt_sha256": sha256(prompt.encode()),
                        "answer": previous, "violations": [violation]}
            paths, schema, repair_prompt = codex.prepare(envelope)
            _, comparison_schema, _ = codex.prepare(dict(envelope, stage="comparison"))
            self.assertEqual(paths, [])
            self.assertEqual(schema, comparison_schema)
            self.assertIn(violation, repair_prompt)
            self.assertIn(json.dumps(previous), repair_prompt)
            result, calls = run_twin("reference-first-claude.py", envelope, fake_claude=success_claude_fake(previous))
            self.assertEqual(len(calls), 1)
            assert_claude_command_and_images(self, calls[0]["command"], calls[0]["kwargs"], [], repair_prompt, schema)
            self.assertEqual(result["status"], "ok")
            with self.assertRaises(ValueError):
                codex.prepare(dict(envelope, violations=[]))


class TuningVisionClaude(unittest.TestCase):
    def build_envelope(self, tmp):
        image = Path(tmp) / "image.png"; image.write_bytes(b"mock image bytes")
        request = {"identity": "fixture", "required": [{"view": "WHOLE"}],
                   "images": [{"path": str(image), "sha256": sha256(image.read_bytes())}],
                   "references": [], "quality_anchors": [],
                   "checklist": "Owner identifies spreading architecture as defining reference character."}
        return {"request": request, "request_sha256": "pinned-request"}

    def capture_codex_dispatch(self, envelope):
        """What tuning-vision-codex.py would send to `codex exec`, unmodified."""
        captured = {}

        def dry_run(command, **kwargs):
            captured["prompt"] = command[-1]
            captured["schema"] = json.loads(Path(command[command.index("--output-schema") + 1]).read_text())
            captured["paths"] = [Path(command[i + 1]) for i, arg in enumerate(command) if arg == "--image"]
            Path(command[command.index("-o") + 1]).write_text(json.dumps({
                "passes": ["unknown"] * len(envelope["request"]["required"]),
                "defects": [], "observations": [], "findings": []}))
            return subprocess.CompletedProcess(command, 0,
                json.dumps({"type": "turn.completed", "usage": {"input_tokens": 0, "output_tokens": 0}}).encode(), b"")

        with patch.object(sys, "argv", ["adapter", "--model", "fixture-model", "--effort", "medium"]), \
             patch.object(sys, "stdin", io.StringIO(json.dumps(envelope))), \
             patch("subprocess.run", side_effect=dry_run), contextlib.redirect_stdout(io.StringIO()):
            runpy.run_path(str(SCRIPTS / "tuning-vision-codex.py"), run_name="__main__")
        return captured

    def test_dispatch_matches_codex_prompt_and_schema(self):
        with tempfile.TemporaryDirectory() as tmp:
            envelope = self.build_envelope(tmp)
            codex_dispatch = self.capture_codex_dispatch(envelope)
            answer = {"passes": ["pass"], "defects": [], "observations": ["fine"],
                      "findings": [{"observation": "matches reference", "evidence_ids": [], "impact": "supported",
                                    "uncertain": False, "causal_hypothesis": None}]}
            result, calls = run_twin("tuning-vision-claude.py", envelope, fake_claude=success_claude_fake(answer))
            self.assertEqual(len(calls), 1)
            assert_claude_command_and_images(self, calls[0]["command"], calls[0]["kwargs"],
                                              codex_dispatch["paths"], codex_dispatch["prompt"], codex_dispatch["schema"])
            self.assertEqual(result["assessment"]["cells"], [[{"view": "WHOLE"}, "pass"]])
            self.assertEqual(result["assessment"]["defects"], [])
            self.assertEqual(result["observations"], ["fine"])
            self.assertEqual(result["usage"], {"input_tokens": 11, "output_tokens": 7})
            self.assertIn("claude-opus-5", result["model_identity_basis"])

    def test_missing_answer_exits_nonzero(self):
        with tempfile.TemporaryDirectory() as tmp:
            envelope = self.build_envelope(tmp)
            with self.assertRaises(SystemExit):
                run_twin("tuning-vision-claude.py", envelope, fake_claude=failing_claude_fake())


if __name__ == "__main__":
    unittest.main()
