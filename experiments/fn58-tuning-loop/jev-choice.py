#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.11"
# dependencies = ["pillow>=10", "numpy>=1.26"]
# ///
"""The second Jev framing: a Choice over candidates code has already measured.

Reads one sweep round from trials.tsv, hands Jev the owner's notes, the
photograph's numbers and every feasible candidate's still numbers (never the
score), and asks which candidate to carry forward, with `none`. Prints Jev's
pick beside code's own argmin so the two can be compared. This is the use the
project's TypeSafe rule allows: Jev may rank candidates code proposed from
their measured comparison; the value that ships is still one code proposed and
a render measured.

  python3 experiments/fn58-tuning-loop/jev-choice.py [--round 1]
"""
from __future__ import annotations

import argparse
import csv
import importlib.util
import json
import subprocess
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
spec = importlib.util.spec_from_file_location("loop", HERE / "loop.py")
loop = importlib.util.module_from_spec(spec)
spec.loader.exec_module(loop)


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--round", type=int, default=1)
    ap.add_argument("--stills", type=Path, default=Path("/tmp/fn58-jev-choice"))
    args = ap.parse_args()
    rows = list(csv.DictReader((HERE / "trials.tsv").open(), delimiter="\t"))
    shipped = next(r for r in rows if r["label"] == "shipped")
    sweep = [r for r in rows if r["arm"] == "sweep" and int(r["round"]) == args.round and r["feasible"] == "True" and r["label"] != "sweep:compound"]
    numbers = lambda r: {ref: {n: float(r[f"{ref}.{n}"]) for n in loop.NUMBERS} for ref in ("B-WHOLE", "B-BARE")}
    labels = {r["label"].removeprefix("sweep:").replace(":", "_"): r for r in sweep}
    state = {
        "notes": loop.NOTES,
        "photograph": loop.photograph_numbers(),
        "shipped": numbers(shipped),
        "numbers": {
            "width_over_height": "the crown's width over its height in the frame",
            "crown_base": "where the crown starts as a share of the tree's height",
            "occupied": "how much of the tree's box is tree; higher is denser",
            "outline_deviation": "how lumpy the silhouette is; near zero is a smooth dome",
            "centre": "mean brightness of the crown's middle, 0 to 255; darker is denser",
        },
        "candidates": {label: {"move": r["label"].removeprefix("sweep:"), "still": numbers(r)} for label, r in labels.items()},
    }
    criteria = {label: f"Carry forward the candidate that moved {r['label'].removeprefix('sweep:')}" for label, r in labels.items()}
    criteria["none"] = "No candidate's numbers sit nearer the photograph than `shipped` on what the notes name"
    questions = {"carry": {
        "type": "choice",
        "instructions": "Each entry in `candidates` is one value trial, rendered and measured the same way as `shipped` and `photograph`. Which candidate's `still` sits nearest `photograph` on the numbers the `notes` complain about?",
        "criteria": criteria,
    }}
    args.stills.mkdir(parents=True, exist_ok=True)
    (args.stills / "state.json").write_text(json.dumps(state, indent=1))
    (args.stills / "questions.json").write_text(json.dumps(questions, indent=1))
    proc = subprocess.run(["bash", "-ic", f"cd {ROOT} && cargo run --release -q -p telperion-jev -- ask --state {args.stills}/state.json --questions {args.stills}/questions.json --tool tune-probe-choice --ledger {loop.LEDGER}"], capture_output=True, text=True, timeout=300)
    answer = json.loads(proc.stdout[proc.stdout.find("{"):])
    carry = answer["answers"]["carry"]
    ranked = sorted(carry["probabilities"].items(), key=lambda kv: -kv[1])
    code = sorted(sweep, key=lambda r: float(r["score"]))
    result = {
        "round": args.round,
        "ledger": answer["reference"],
        "jev_pick": carry["choice"],
        "jev_confidence": carry["confidence"],
        "jev_top": ranked[:5],
        "code_argmin": code[0]["label"].removeprefix("sweep:"),
        "code_scores": {r["label"].removeprefix("sweep:"): float(r["score"]) for r in code},
    }
    (HERE / f"jev-choice-round{args.round}.json").write_text(json.dumps(result, indent=1) + "\n")
    print(json.dumps(result, indent=1))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
