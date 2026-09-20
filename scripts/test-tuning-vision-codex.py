"""Offline adapter contract: never launches Codex or consumes model tokens."""
import contextlib
import hashlib
import io
import json
from pathlib import Path
import runpy
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch


class VisionContract(unittest.TestCase):
    def test_reference_structure_priority_and_literal_output(self):
        with tempfile.TemporaryDirectory() as directory:
            image = Path(directory) / "image.png"
            image.write_bytes(b"mock image bytes")
            request = {"identity": "fixture", "required": [{"view": "WHOLE"}],
                       "images": [{"path": str(image), "sha256": hashlib.sha256(image.read_bytes()).hexdigest()}],
                       "references": [], "quality_anchors": [],
                       "checklist": "Owner identifies spreading architecture as defining reference character."}
            envelope = {"request": request, "request_sha256": "pinned-request"}
            answer = {"passes": ["fail"], "defects": ["structural blocker"],
                      "observations": ["acceptable variation"]}

            def fake_run(command, **kwargs):
                prompt = command[-1]
                for phrase in ("believable reference character", "relative improvement", "absolute readiness",
                               "leaf-bearing droop", "owner criterion", "rank blocking defects",
                               "Do not contradict", "unknown"):
                    self.assertIn(phrase, prompt)
                self.assertIn(request["checklist"], prompt)
                self.assertIn("Photorealism is NOT the goal", prompt)
                Path(command[command.index("-o") + 1]).write_text(json.dumps(answer))
                return subprocess.CompletedProcess(command, 0, json.dumps({"type": "turn.completed",
                    "usage": {"input_tokens": 12, "output_tokens": 3}}).encode(), b"")

            output = io.StringIO()
            with patch.object(sys, "argv", ["adapter", "--model", "fixture-model", "--effort", "medium"]), \
                 patch.object(sys, "stdin", io.StringIO(json.dumps(envelope))), \
                 patch("subprocess.run", side_effect=fake_run) as dispatch, contextlib.redirect_stdout(output):
                runpy.run_path(str(Path(__file__).with_name("tuning-vision-codex.py")), run_name="__main__")
            result = json.loads(output.getvalue())
            self.assertEqual(dispatch.call_count, 1)
            self.assertEqual(result["assessment"]["cells"], [[request["required"][0], "fail"]])
            self.assertEqual(result["assessment"]["defects"], answer["defects"])
            self.assertEqual(result["observations"], answer["observations"])
            self.assertEqual(result["request_sha256"], "pinned-request")
            self.assertEqual(result["usage"], {"input_tokens": 12, "output_tokens": 3})


if __name__ == "__main__":
    unittest.main()
