#!/usr/bin/env python3
"""Offline guards for whole-only scope, inventory bind, negative judgment, and release gate."""
import json
import subprocess
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

import proof_lib
from proof_lib import bind_inventory, judge_negative, load, prepare_envelope


class Guards(unittest.TestCase):
    def test_stage_a_whole_only_prepare(self):
        row = prepare_envelope("stage-a-birch-request.json")
        self.assertEqual(len(row["images"]), 1)
        self.assertEqual(row["envelope"]["request"]["references"][0]["image"]["view"], "S-WHOLE")
        self.assertEqual(row["prompt_sha256"], proof_lib.INVENTORY_PROMPT_SHA)

    def test_stage_b_whole_only_prepare(self):
        row = prepare_envelope("stage-b-birch-positive-request.json")
        self.assertEqual(len(row["images"]), 3)
        views = [img["path"] for img in row["envelope"]["request"]["comparison"]["references"]]
        self.assertTrue(all("bepe339A" in p for p in views))
        self.assertFalse(any("bepe340A" in p for p in views))
        self.assertIsNone(row["envelope"]["request"]["inventory"])

    def test_bind_rejects_missing_and_bare_traits(self):
        envelope = load("stage-a-birch-request.json")
        with self.assertRaises(ValueError):
            bind_inventory({}, envelope)
        with self.assertRaises(ValueError):
            bind_inventory(
                {
                    "request_sha256": envelope["request_sha256"],
                    "status": "ok",
                    "answer": {
                        "traits": [
                            {
                                "id": "bark",
                                "priority": "core",
                                "reference_ids": ["reference-1"],
                                "observation": "bare bark",
                                "uncertain": False,
                            }
                        ]
                    },
                },
                envelope,
            )

    def test_bind_accepts_validated_receipt_only(self):
        envelope = load("stage-a-birch-request.json")
        receipt = {
            "request_sha256": envelope["request_sha256"],
            "status": "ok",
            "model": "gpt-6-astra",
            "effort": "medium",
            "answer": {
                "traits": [
                    {
                        "id": "crown_outline",
                        "priority": "core",
                        "reference_ids": ["reference-0"],
                        "observation": "light irregular crown",
                        "uncertain": False,
                    }
                ],
                "observations": ["whole-view only"],
            },
        }
        bound = bind_inventory(receipt, envelope)
        self.assertEqual(bound["request"], envelope["request"])
        self.assertEqual(bound["traits"][0]["id"], "crown_outline")

    def test_negative_requires_grounded_fail(self):
        ok = judge_negative(
            {
                "passes": ["fail"],
                "findings": [
                    {
                        "observation": "regular columnar foliage vs layered reference",
                        "evidence_ids": ["render-0", "reference-0"],
                        "impact": "blocker",
                        "uncertain": False,
                    }
                ],
            }
        )
        self.assertEqual(ok["status"], "negative_calibration_ok")
        unknown = judge_negative({"passes": ["unknown"], "findings": []})
        self.assertEqual(unknown["status"], "abstention_not_successful_negative")
        clip = judge_negative(
            {
                "passes": ["unknown"],
                "findings": [
                    {
                        "observation": "clipped top, cannot judge",
                        "evidence_ids": ["render-0"],
                        "impact": "required_unknown",
                        "uncertain": True,
                    }
                ],
            }
        )
        self.assertEqual(clip["status"], "abstention_not_successful_negative")
        ready = judge_negative({"passes": ["pass"], "findings": []})
        self.assertEqual(ready["status"], "false_ready")

    def test_execute_without_release_is_two(self):
        run = subprocess.run(
            ["python3", str(proof_lib.PROOF / "run-proof.py"), "--execute", "--stage", "stage-a-birch"],
            cwd=str(proof_lib.PROOF),
            capture_output=True,
            text=True,
        )
        self.assertEqual(run.returncode, 2)
        self.assertIn("host review not released", run.stderr)

    def test_runner_would_call_adapter_after_release(self):
        import importlib.util
        spec = importlib.util.spec_from_file_location("run_proof", proof_lib.PROOF / "run-proof.py")
        run_proof = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(run_proof)

        with tempfile.TemporaryDirectory() as temp:
            release = Path(temp) / "release.json"
            release.write_text(json.dumps({"release": "stage-a-birch"}))
            called = []

            def fake_dispatch(envelope, model="gpt-6-astra", effort="medium"):
                called.append(envelope["request"]["protocol"])
                return type("Done", (), {"returncode": 1, "stdout": b"{}", "stderr": b"blocked-test"})()

            reserve = proof_lib.PROOF / "stage-a-birch-reservation.json"
            stdout = proof_lib.PROOF / "stage-a-birch-stdout.json"
            stderr = proof_lib.PROOF / "stage-a-birch-stderr.txt"
            self.assertFalse(reserve.exists())
            try:
                with patch.object(run_proof, "release_path", return_value=release), patch.object(
                    run_proof, "dispatch", side_effect=fake_dispatch
                ), patch.object(run_proof, "INVENTORY", Path(temp) / "inv.json"), patch.object(
                    run_proof, "STATE", Path(temp) / "state.json"
                ):
                    code = run_proof.execute("stage-a-birch")
                self.assertEqual(called, ["reference-first-v1"])
                self.assertEqual(code, 2)
                self.assertTrue(reserve.exists())
            finally:
                for path in (reserve, stdout, stderr):
                    if path.exists():
                        path.unlink()


if __name__ == "__main__":
    unittest.main()
