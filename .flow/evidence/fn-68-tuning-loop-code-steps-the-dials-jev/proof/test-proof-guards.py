#!/usr/bin/env python3
"""Offline guards for whole-only scope, inventory bind, negative judgment, and release gate."""
import json
import subprocess
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

import proof_lib
from proof_lib import bind_inventory, judge_negative, judge_positive, load, prepare_envelope


def load_runner():
    import importlib.util
    spec = importlib.util.spec_from_file_location("run_proof", proof_lib.PROOF / "run-proof.py")
    mod = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(mod)
    return mod


def ok_inventory():
    envelope = load("stage-a-birch-request.json")
    receipt = {
        "request_sha256": envelope["request_sha256"],
        "status": "ok",
        "model": "gpt-6-astra",
        "effort": "medium",
        "usage": {"input_tokens": 80, "output_tokens": 20},
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
    return envelope, receipt, bind_inventory(receipt, envelope)


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
        self.assertEqual(ok["status"], "negative_code_guard_ok")
        self.assertFalse(ok["semantic_qualification"])
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
        self.assertTrue(
            "host review not released" in run.stderr or "terminal needs_human" in run.stderr
        )

    def test_positive_requires_pass_and_supported_coverage(self):
        required = [{"item": "reference_character", "view": "S-WHOLE", "seed": 1}]
        fail = judge_positive({"passes": ["fail"], "findings": []}, required)
        self.assertEqual(fail["status"], "positive_calibration_failed")
        transport = judge_positive(
            {
                "passes": ["pass"],
                "findings": [
                    {
                        "observation": "looks fine",
                        "evidence_ids": ["render-0"],
                        "impact": "supported",
                    }
                ],
            },
            required,
        )
        self.assertEqual(transport["status"], "positive_calibration_failed")
        ok = judge_positive(
            {
                "passes": ["pass"],
                "findings": [
                    {
                        "observation": "crown matches",
                        "evidence_ids": ["render-0", "reference-0"],
                        "impact": "supported",
                    }
                ],
            },
            required,
        )
        self.assertEqual(ok["status"], "positive_calibration_ok")

    def _exec(self, run_proof, stage, receipt, temp, returncode=0, extra=None):
        extra = extra or {}
        release = Path(temp) / "release.json"
        release.write_text(json.dumps({"release": stage}))
        called = []

        def fake_dispatch(envelope, model="gpt-6-astra", effort="medium"):
            called.append(envelope["request_sha256"])
            return type(
                "Done",
                (),
                {"returncode": returncode, "stdout": json.dumps(receipt).encode(), "stderr": b""},
            )()

        paths = {
            "state": Path(temp) / "state.json",
            "inventory": Path(temp) / "inv.json",
            "reserve": Path(temp) / f"{stage}-reservation.json",
            "stdout": Path(temp) / f"{stage}-stdout.json",
            "stderr": Path(temp) / f"{stage}-stderr.txt",
            "judgment": Path(temp) / f"{stage}-judgment.json",
        }
        paths.update(extra)
        with patch.object(run_proof, "release_path", return_value=release), patch.object(
            run_proof, "dispatch", side_effect=fake_dispatch
        ):
            code = run_proof.execute(stage, **paths)
        return code, json.loads(paths["state"].read_text()) if paths["state"].exists() else {}, called, paths

    def test_successful_settle(self):
        run_proof = load_runner()
        envelope, receipt, bound = ok_inventory()
        with tempfile.TemporaryDirectory() as temp:
            code, state, _, paths = self._exec(run_proof, "stage-a-birch", receipt, temp)
            self.assertEqual(code, 0)
            self.assertIsNone(state.get("terminal"))
            self.assertEqual(state["settled"], ["stage-a-birch"])
            self.assertEqual(state["tokens"], 552431 + 100)
            self.assertEqual(state["visual"], 21)
            self.assertTrue(paths["inventory"].exists())

    def test_invalid_known_usage_is_charged(self):
        run_proof = load_runner()
        envelope, receipt, _ = ok_inventory()
        receipt["status"] = "failed_or_tools_or_unknown_usage_or_cardinality"
        with tempfile.TemporaryDirectory() as temp:
            code, state, _, _ = self._exec(run_proof, "stage-a-birch", receipt, temp)
            self.assertEqual(code, 2)
            self.assertEqual(state["tokens"], 552431 + 100)
            self.assertEqual(state["visual"], 21)
            self.assertEqual(state["terminal"]["reason"], "invalid_receipt")
            code2, state2, _, _ = self._exec(run_proof, "stage-b-beech-negative", receipt, temp)
            self.assertEqual(code2, 2)
            self.assertEqual(state2["tokens"], 552431 + 100)
            self.assertEqual(state2["terminal"]["reason"], "invalid_receipt")

    def test_unknown_usage_blocks_next(self):
        run_proof = load_runner()
        with tempfile.TemporaryDirectory() as temp:
            code, state, _, paths = self._exec(run_proof, "stage-a-birch", {}, temp)
            self.assertEqual(code, 2)
            self.assertEqual(state["terminal"]["reason"], "unknown_usage")
            self.assertEqual(state["outstanding_reservation"], "stage-a-birch")
            self.assertEqual(state["tokens"], 552431)
            code2, state2, _, _ = self._exec(
                run_proof,
                "stage-b-beech-negative",
                {"usage": {"input_tokens": 1, "output_tokens": 1}, "status": "ok"},
                temp,
                extra={"reserve": Path(temp) / "neg-reservation.json"},
            )
            self.assertEqual(code2, 2)
            self.assertEqual(state2["tokens"], 552431)
            self.assertFalse((Path(temp) / "neg-reservation.json").exists())

    def test_positive_failure_stops_later_stages(self):
        run_proof = load_runner()
        envelope, receipt, bound = ok_inventory()
        fail = {
            "request_sha256": load("stage-b-birch-positive-request.json")["request_sha256"],
            "status": "ok",
            "model": "gpt-6-astra",
            "effort": "medium",
            "usage": {"input_tokens": 200, "output_tokens": 30},
            "answer": {
                "passes": ["fail"],
                "findings": [
                    {
                        "observation": "wrong species",
                        "evidence_ids": ["render-0", "reference-0"],
                        "impact": "blocker",
                    }
                ],
            },
        }
        with tempfile.TemporaryDirectory() as temp:
            inv = Path(temp) / "inv.json"
            inv.write_text(json.dumps(bound))
            code, state, _, _ = self._exec(
                run_proof,
                "stage-b-birch-positive",
                fail,
                temp,
                extra={"inventory": inv},
            )
            self.assertEqual(code, 2)
            self.assertEqual(state["terminal"]["reason"], "positive_calibration_failed")
            self.assertEqual(state["tokens"], 552431 + 230)
            self.assertEqual(state["settled"], [])
            code2, state2, _, _ = self._exec(
                run_proof,
                "stage-b-beech-negative",
                {
                    "request_sha256": "x",
                    "status": "ok",
                    "model": "gpt-6-astra",
                    "effort": "medium",
                    "usage": {"input_tokens": 1, "output_tokens": 1},
                    "answer": {"passes": ["fail"], "findings": []},
                },
                temp,
                extra={"reserve": Path(temp) / "neg-reservation.json"},
            )
            self.assertEqual(code2, 2)
            self.assertEqual(state2["tokens"], 552431 + 230)

    def test_runner_would_call_adapter_after_release(self):
        run_proof = load_runner()
        with tempfile.TemporaryDirectory() as temp:
            code, state, called, paths = self._exec(run_proof, "stage-a-birch", {}, temp, returncode=1)
            self.assertTrue(called)
            self.assertEqual(code, 2)
            self.assertTrue(paths["reserve"].exists())
            self.assertEqual(state["terminal"]["reason"], "unknown_usage")

    def test_contaminated_stage_and_unreleased_blind_refuse(self):
        contaminated = subprocess.run(
            ["python3", str(proof_lib.PROOF / "run-proof.py"), "--execute", "--stage", "stage-b-beech-negative"],
            cwd=str(proof_lib.PROOF),
            capture_output=True,
            text=True,
        )
        self.assertEqual(contaminated.returncode, 2)
        self.assertIn("contaminated request is immutable", contaminated.stderr)
        blind = subprocess.run(
            ["python3", str(proof_lib.PROOF / "run-proof.py"), "--execute", "--stage", "stage-b-beech-negative-blind"],
            cwd=str(proof_lib.PROOF),
            capture_output=True,
            text=True,
        )
        self.assertEqual(blind.returncode, 2)
        self.assertIn("replacement not released", blind.stderr)

    def test_contaminated_raw_unaltered(self):
        self.assertEqual(
            proof_lib.digest(proof_lib.PROOF / "stage-b-beech-negative-request.json"),
            proof_lib.CONTAMINATED_REQUEST_SHA,
        )
        self.assertEqual(
            proof_lib.digest(proof_lib.PROOF / "stage-b-beech-negative-stdout.json"),
            proof_lib.CONTAMINATED_STDOUT_SHA,
        )

    def test_blind_payload_is_actual_adapter_prompt(self):
        row = prepare_envelope("stage-b-beech-negative-blind-request.json")
        frozen = load("model-visible-payload.json")
        self.assertEqual(row["dispatched_prompt_sha256"], frozen["dispatched_prompt_sha256"])
        self.assertEqual(row["schema_sha256"], frozen["schema_sha256"])
        self.assertEqual(row["request_body_sha256"], frozen["request_sha256"])
        self.assertEqual(row["prompt_sha256"], proof_lib.COMPARISON_PROMPT_SHA)
        self.assertEqual([Path(p).name for p in row["paths"]], [
            "render-0.png",
            "reference-0.jpg",
            "reference-1.jpg",
            "anchor-0.png",
        ])
        self.assertEqual(
            [img["sha256"] for img in row["images"]],
            [img["sha256"] for img in frozen["image_order"]],
        )
        self.assertEqual(proof_lib.payload_leaks(row["prompt"]), [])
        self.assertNotIn("expected_ready", row["envelope"])
        self.assertNotIn("label", row["envelope"])

    def test_grader_labels_cannot_change_dispatched_hash(self):
        first = prepare_envelope("stage-b-beech-negative-blind-request.json")
        grader = proof_lib.PROOF / "stage-b-beech-negative-blind-grader.json"
        original = grader.read_text()
        try:
            grader.write_text(json.dumps({
                "expected_ready": True,
                "known_negative": False,
                "label": "this case is the known negative; a pass is false-ready",
                "previous_verdict": "fail",
            }))
            second = prepare_envelope("stage-b-beech-negative-blind-request.json")
        finally:
            grader.write_text(original)
        self.assertEqual(first["dispatched_prompt_sha256"], second["dispatched_prompt_sha256"])
        tainted = json.loads(json.dumps(first["envelope"]))
        tainted["expected_ready"] = False
        tainted["label"] = "owner-rejected known negative"
        stripped = proof_lib.strip_grader_keys(tainted)
        paths, schema, prompt = proof_lib.adapter().prepare(stripped)
        proof_lib.assert_blind_payload(prompt, schema, paths)
        self.assertEqual(
            hashlib_sha(prompt),
            first["dispatched_prompt_sha256"],
        )


def hashlib_sha(text):
    import hashlib
    return hashlib.sha256(text.encode()).hexdigest()


if __name__ == "__main__":
    unittest.main()
