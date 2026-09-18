#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.11"
# dependencies = ["pillow>=10", "numpy>=1.26"]
# ///
"""A tuning-loop probe for fn-58: does a loop of code and Jev move a preset's
matched stills toward the photograph's numbers, and at what cost?

One candidate is a partial wire object laid over the shipped preset. Code
measures it (the species example, for the gates and the node cap), renders
its matched stills (the headless example, still and twin per reference) and
reads the compare script's numbers off them. The score is the mean relative
distance from the photograph's own numbers. Two arms run from the shipped
rows:

  jev   Jev answers one Noul per candidate move, over the owner's verdict
        notes, the photograph's numbers and the current still's; code
        evaluates the four moves Jev put highest and keeps the best.
  sweep Code evaluates every single-step move on every dial and keeps the
        best, plus the compound of every improving move.
  direction
        The owner's framing: one Choice per dial over up, down and hold; code
        evaluates the four dials Jev moves most confidently and keeps the best.

Jev never writes a number: the step table is authored here, the moves are
code's, the score is code's. Stills go to a scratch directory and are never
looked at; the numbers are the record.

  uv run experiments/fn58-tuning-loop/loop.py --stills DIR [--arms jev,sweep]
      [--rounds 4] [--max-evals 70] [--seed 1] [--tag NAME]
"""
from __future__ import annotations

import argparse
import csv
import hashlib
import importlib.util
import json
import subprocess
import sys
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
MEASURE = ROOT / "target/release/examples/species_measure"
HEADLESS = ROOT / "target/release/examples/headless"
PROFILES = ROOT / ".flow/evidence/fn34/profiles.json"
REFERENCES = ROOT / ".flow/evidence/fn34/european-beech/references.json"
ROUNDS = ROOT / ".flow/evidence/fn34/rounds.tsv"
LEDGER = ROOT / ".flow/ledger/jev-tune-probe"
PRESET = "european-beech"
HEIGHT = 1440
NUMBERS = ("width_over_height", "crown_base", "occupied", "outline_deviation", "centre")

# The owner's verdict on the shipped beech (fn-62, 2026-09-16), verbatim.
NOTES = [
    "The tree has clear regular outline/border that doesn't look natural. The taper "
    "approaching the border needs to produce thinner branches twigs. It's also too dense "
    "at the shoulder of the crown. It's more sparse lower and gets more dense at the top. "
    "It generally looks too dense everywhere?",
    "We have trees where the branches point upwards when there are no leaves. But they "
    "droop under the weight of leaves. We still haven't achieved the reference for the "
    "bare one either. It still looks off.",
]

# The dials a value round may move: wire path, plain reading, step, bounds.
DIALS = {
    "lateral_pitch": ("skeleton/habit/lateralPitch", "degrees a limb leaves the trunk from vertical", 6.0, (10.0, 70.0)),
    "laterals_per_station": ("skeleton/habit/lateralsPerStation", "limbs born at each station up the trunk", 1, (1, 4)),
    "crookedness": ("skeleton/habit/crookedness", "how much a limb turns and zigzags as it runs", 3.0, (0.0, 15.0)),
    "twig_tip_taper": ("skeleton/habit/twigTipTaper", "how far the wood thins toward the crown's edge", 0.1, (0.05, 0.6)),
    "twig_laterals": ("skeleton/twigs/laterals", "twigs born at each station on a shoot", 1, (1, 8)),
    "twig_length_ratio": ("skeleton/twigs/lengthRatio", "a twig's length as a share of its parent's", 0.05, (0.1, 0.6)),
    "irregularity": ("skeleton/envelope/irregularity", "how lumpy the crown envelope is, lobes and hollows", 0.08, (0.0, 0.5)),
    "spread": ("skeleton/envelope/spread", "the crown's width as a share of its height", 0.04, (0.2, 0.6)),
    "crown_base": ("skeleton/envelope/crownBase", "where the crown starts, as a share of the height", 0.04, (0.02, 0.3)),
    "short_shoot_leaves": ("canopy/shortShootLeaves", "leaves in each cluster along the limbs", 2, (2, 12)),
    "short_shoot_spacing": ("canopy/shortShootSpacing", "metres between leaf clusters along a limb", 0.01, (0.01, 0.08)),
    "limb_clumping": ("canopy/limbClumping", "how much each limb keeps its own leaf mass, with gaps between", 0.1, (0.0, 0.6)),
}


def load_compare():
    spec = importlib.util.spec_from_file_location("compare_references", ROOT / "scripts/compare-references.py")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


CR = load_compare()


def photograph_numbers() -> dict[str, dict[str, float]]:
    """The photograph's numbers as the round table records them: the record's
    own box and crown base, the compare script's occupied, outline and centre."""
    rows = [r for r in csv.DictReader(ROUNDS.open(), delimiter="\t") if r["species"] == PRESET and r["photo_outline_deviation"]]
    out = {}
    for row in rows:
        out[row["reference"]] = {
            "width_over_height": float(row["photo_width_over_height"]),
            "crown_base": float(row["photo_crown_base"]),
            "occupied": float(row["photo_occupied"]),
            "outline_deviation": float(row["photo_outline_deviation"]),
            "centre": float(row["photo_centre_mean"]),
        }
    return out


def scene_of(light: dict, camera: dict, twin: bool) -> dict:
    """Ported from tests/species.mjs: the photograph's sun, or the twin's sun
    on the horizon behind the tree so its shadow leaves the frame."""
    o, dim = light["overcast"], 1 - 0.8 * light["overcast"]
    mix = lambda a, b: a + (b - a) * o
    return {
        "sunAzimuth": (camera["azimuth"] + 180) % 360 if twin else light["sunAzimuth"],
        "sunElevation": 5 if twin else light["sunElevation"],
        "sunRed": 3.0 * dim, "sunGreen": 2.85 * dim, "sunBlue": 2.6 * dim,
        "skyZenithRed": mix(0.18, 0.55), "skyZenithGreen": mix(0.30, 0.66), "skyZenithBlue": mix(0.62, 0.80),
    }


def run(argv: list[str], timeout: int) -> subprocess.CompletedProcess:
    return subprocess.run(argv, cwd=ROOT, capture_output=True, text=True, timeout=timeout)


def set_path(obj: dict, path: str, value) -> None:
    keys = path.split("/")
    for key in keys[:-1]:
        obj = obj.setdefault(key, {})
    obj[keys[-1]] = value


class Probe:
    def __init__(self, stills: Path, seed: int, max_evals: int, tag: str = ""):
        self.stills, self.seed, self.max_evals, self.tag = stills, seed, max_evals, tag
        self.records = [r for r in json.loads(REFERENCES.read_text())["references"] if r.get("shot") and r["id"] in ("B-WHOLE", "B-BARE")]
        self.photo = photograph_numbers()
        self.base = json.loads(run([str(MEASURE), "--print-family", PRESET], 60).stdout)
        self.cache: dict[str, dict] = {}
        self.trials: list[dict] = []
        self.evals = 0
        stills.mkdir(parents=True, exist_ok=True)
        self.ledger_refs: list[str] = []

    def current_value(self, dial: str, over: dict):
        path = DIALS[dial][0]
        node = over
        for key in path.split("/"):
            node = node.get(key) if isinstance(node, dict) else None
            if node is None:
                break
        if node is not None:
            return node
        node = self.base
        for key in path.split("/"):
            node = node[key]
        return node

    def move(self, dial: str, direction: int, over: dict) -> dict | None:
        path, _, step, (low, high) = DIALS[dial]
        now = self.current_value(dial, over)
        to = now + direction * step
        to = max(low, min(high, to))
        if isinstance(step, int):
            to = int(round(to))
        else:
            to = round(to, 4)
        if to == now:
            return None
        out = json.loads(json.dumps(over))
        set_path(out, path, to)
        return out

    def key(self, over: dict) -> str:
        return hashlib.sha256(json.dumps(over, sort_keys=True).encode()).hexdigest()[:12]

    def evaluate(self, over: dict, arm: str, round_no: int, label: str) -> dict:
        key = self.key(over)
        if key in self.cache:
            hit = dict(self.cache[key]); hit.update(arm=arm, round=round_no, label=label, cached=True)
            self.trials.append(hit)
            return self.cache[key]
        if self.evals >= self.max_evals:
            raise SystemExit(f"evaluation budget of {self.max_evals} spent")
        self.evals += 1
        started = time.time()
        family = self.stills / f"{key}.json"
        family.write_text(json.dumps(over, sort_keys=True) + "\n")
        result = {"key": key, "overrides": json.dumps(over, sort_keys=True), "feasible": True, "reason": ""}
        # Gates and the node cap, from the species example.
        out = self.stills / f"{key}-measure.jsonl"
        if out.exists():
            out.unlink()
        run([str(MEASURE), "--case", f"{key}:{PRESET}:{PRESET}:{self.seed}", "--profiles", str(PROFILES), "--family", str(family), "--output", str(out)], 300)
        done = None
        for line in out.read_text().splitlines():
            event = json.loads(line)
            if event.get("event") in ("completed", "failed"):
                done = event
        if done is None or done["event"] == "failed":
            result.update(feasible=False, reason=(done or {}).get("reason", "no completed case"))
        else:
            growth = done["metrics"]["growth"]
            failing = [k for k, v in done["checks"].items() if v["status"] == "fail"]
            capped = growth["node_capped"] or growth["level_capped"] or growth["attraction_capped"]
            result.update(
                height_m=round(done["metrics"]["height_m"]["value"], 2),
                dbh_m=round(done["metrics"]["dbh_m"]["value"], 3),
                wood_triangles=done["counts"]["wood_triangles"],
                leaves=done["metrics"].get("foliage_units", {}).get("value"),
                numeric=done["numeric_status"],
            )
            if failing or capped:
                result.update(feasible=False, reason=f"gates {failing} capped={capped}")
        # The matched stills and their numbers.
        if result["feasible"]:
            for record in self.records:
                shot = record["shot"]
                size = f"{round(HEIGHT * shot['aspect'][0] / shot['aspect'][1])}x{HEIGHT}"
                view = "bare" if shot["foliage"] == "hidden" else "whole"
                paths = {}
                for twin in (False, True):
                    png = self.stills / f"{key}-{record['id']}{'-twin' if twin else ''}.png"
                    argv = [str(HEADLESS), "--preset", PRESET, "--seed", str(self.seed), "--view", view, "--size", size,
                            "--out", str(png), "--family", str(family), "--camera", json.dumps(shot["camera"]),
                            "--scene", json.dumps(scene_of(shot["light"], shot["camera"], twin)), "--no-figure"]
                    proc = run(argv, 300)
                    if proc.returncode != 0:
                        result.update(feasible=False, reason=f"render {record['id']}: {proc.stderr[-200:]}")
                        break
                    paths[twin] = png
                if not result["feasible"]:
                    break
                still, twin_image = CR.load(paths[False]), CR.load(paths[True])
                mask = CR.tree_mask(still, twin_image)
                box = CR.box_of(mask)
                if box is None:
                    result.update(feasible=False, reason=f"{record['id']}: no tree in the still")
                    break
                numbers = CR.measure(still, box, mask, CR.crown_base(mask, box))
                numbers["centre"] = numbers["centre"]["mean"]
                for name in NUMBERS:
                    result[f"{record['id']}.{name}"] = numbers[name]
        result["score"] = self.score(result) if result["feasible"] else None
        result["seconds"] = round(time.time() - started, 1)
        self.cache[key] = result
        row = dict(result); row.update(arm=arm, round=round_no, label=label, cached=False)
        self.trials.append(row)
        print(f"  {label:32} {key} {'%.4f' % result['score'] if result['score'] is not None else 'infeasible: ' + result['reason']} ({result['seconds']}s)", flush=True)
        return result

    def score(self, result: dict) -> float:
        """Mean relative distance from the photograph over both references and
        all five numbers; an outline the script could not close counts as the
        photograph's full distance."""
        terms = []
        for record in self.records:
            for name in NUMBERS:
                still, photo = result.get(f"{record['id']}.{name}"), self.photo[record["id"]][name]
                terms.append(1.0 if still is None else abs(still - photo) / photo)
        return round(sum(terms) / len(terms), 4)

    def moves(self, over: dict) -> dict[str, tuple[str, int, dict]]:
        out = {}
        for dial in DIALS:
            for direction in (-1, 1):
                moved = self.move(dial, direction, over)
                if moved is not None:
                    out[f"{dial}:{'up' if direction > 0 else 'down'}"] = (dial, direction, moved)
        return out

    def ask_jev(self, over: dict, current: dict, moves: dict, history: list) -> tuple[dict[str, float], str]:
        """One Noul per move: would it take the still toward the photograph on
        what the notes name? Jev reads the notes, both numbers tables and the
        dial table; it never writes a value."""
        state = {
            "notes": NOTES,
            "photograph": self.photo,
            "still": {r["id"]: {n: current.get(f"{r['id']}.{n}") for n in NUMBERS} for r in self.records},
            "numbers": {
                "width_over_height": "the crown's width over its height in the frame",
                "crown_base": "where the crown starts as a share of the tree's height",
                "occupied": "how much of the tree's box is tree; higher is denser",
                "outline_deviation": "how lumpy the silhouette is; near zero is a smooth dome",
                "centre": "mean brightness of the crown's middle, 0 to 255; darker is denser",
            },
            "dials": {d: {"value": self.current_value(d, over), "what": DIALS[d][1]} for d in DIALS},
            "moves": {m: {"dial": dial, "to": self.current_value(dial, moved)} for m, (dial, _, moved) in moves.items()},
            "accepted_so_far": history,
        }
        questions = {
            m: {
                "type": "noul",
                "instructions": f"Would `moves.{m}`, setting `dials.{dial}` to `moves.{m}.to`, take `still` toward `photograph` on the numbers `notes` complain about?",
                "criteria": {"true": "The move answers what the notes name and moves those numbers toward the photograph's",
                             "false": "The move is beside the notes, or moves the numbers the wrong way, or the dial is not what the notes are about"},
            }
            for m, (dial, _, _) in moves.items()
        }
        state_path, q_path = self.stills / "jev-state.json", self.stills / "jev-questions.json"
        state_path.write_text(json.dumps(state, indent=1))
        q_path.write_text(json.dumps(questions, indent=1))
        argv = ["bash", "-ic", f"cd {ROOT} && cargo run --release -q -p telperion-jev -- ask --state {state_path} --questions {q_path} --tool tune-probe --ledger {LEDGER}"]
        proc = subprocess.run(argv, capture_output=True, text=True, timeout=300)
        text = proc.stdout[proc.stdout.find("{"):]
        answer = json.loads(text)
        probs = {m: answer["answers"][m].get("noul") for m in moves}
        self.ledger_refs.append(answer["reference"])
        return probs, answer["reference"]

    def arm_jev(self, rounds: int) -> dict:
        over, history, trajectory = {}, [], []
        current = self.evaluate(over, "jev", 0, "shipped")
        trajectory.append({"round": 0, "score": current["score"], "key": current["key"]})
        for round_no in range(1, rounds + 1):
            moves = self.moves(over)
            probs, ref = self.ask_jev(over, current, moves, history)
            ranked = sorted(probs.items(), key=lambda kv: -(kv[1] or 0))
            chosen = [m for m, p in ranked[:4] if (p or 0) >= 0.5]
            print(f"jev round {round_no}: {ref}; top {[(m, round(p or 0, 2)) for m, p in ranked[:6]]}", flush=True)
            if not chosen:
                trajectory.append({"round": round_no, "stop": "Jev proposed no move at or above 0.5", "ledger": ref})
                break
            best = None
            for m in chosen:
                r = self.evaluate(moves[m][2], "jev", round_no, f"jev:{m}")
                if r["score"] is not None and (best is None or r["score"] < best[1]["score"]):
                    best = (m, r)
            if best is None or best[1]["score"] >= current["score"] * 0.99:
                trajectory.append({"round": round_no, "stop": "no proposed move improved the score by 1%", "ledger": ref,
                                   "tried": {m: self.cache[self.key(moves[m][2])]["score"] for m in chosen}})
                break
            m, r = best
            over, current = moves[m][2], r
            history.append({"round": round_no, "move": m, "score": r["score"]})
            trajectory.append({"round": round_no, "accepted": m, "score": r["score"], "key": r["key"], "ledger": ref,
                               "tried": {mm: self.cache[self.key(moves[mm][2])]["score"] for mm in chosen}})
        return {"arm": "jev", "final": over, "trajectory": trajectory}

    def ask_direction(self, over: dict, current: dict, history: list) -> tuple[dict[str, dict], str]:
        """The owner's framing: one Choice per dial over up, down and hold, so
        Jev commits to a direction and code applies it. Same state as the
        per-move Noul; the shape differs, the knowledge does not."""
        state = {
            "notes": NOTES,
            "photograph": self.photo,
            "still": {r["id"]: {n: current.get(f"{r['id']}.{n}") for n in NUMBERS} for r in self.records},
            "numbers": {
                "width_over_height": "the crown's width over its height in the frame",
                "crown_base": "where the crown starts as a share of the tree's height",
                "occupied": "how much of the tree's box is tree; higher is denser",
                "outline_deviation": "how lumpy the silhouette is; near zero is a smooth dome",
                "centre": "mean brightness of the crown's middle, 0 to 255; darker is denser",
            },
            "dials": {d: {"value": self.current_value(d, over), "step": DIALS[d][2], "what": DIALS[d][1]} for d in DIALS},
            "accepted_so_far": history,
        }
        questions = {
            d: {
                "type": "choice",
                "instructions": f"To take `still` toward `photograph` on the numbers `notes` complain about, should `dials.{d}` move by one step, and which way?",
                "criteria": {"up": f"Raise {d} by one step", "down": f"Lower {d} by one step", "hold": f"Leave {d} where it is; it is not what the notes are about or its direction is unclear"},
            }
            for d in DIALS
        }
        state_path, q_path = self.stills / "jev-dir-state.json", self.stills / "jev-dir-questions.json"
        state_path.write_text(json.dumps(state, indent=1))
        q_path.write_text(json.dumps(questions, indent=1))
        argv = ["bash", "-ic", f"cd {ROOT} && cargo run --release -q -p telperion-jev -- ask --state {state_path} --questions {q_path} --tool tune-probe-direction --ledger {LEDGER}"]
        proc = subprocess.run(argv, capture_output=True, text=True, timeout=300)
        answer = json.loads(proc.stdout[proc.stdout.find("{"):])
        self.ledger_refs.append(answer["reference"])
        return {d: answer["answers"][d] for d in DIALS}, answer["reference"]

    def arm_direction(self, rounds: int) -> dict:
        over, history, trajectory = {}, [], []
        current = self.evaluate(over, "direction", 0, "shipped")
        trajectory.append({"round": 0, "score": current["score"], "key": current["key"]})
        for round_no in range(1, rounds + 1):
            answers, ref = self.ask_direction(over, current, history)
            moved = sorted(((d, a) for d, a in answers.items() if a["choice"] != "hold"), key=lambda da: -da[1]["confidence"])
            print(f"direction round {round_no}: {ref}; {[(d, a['choice'], round(a['confidence'], 2)) for d, a in moved]}", flush=True)
            chosen = moved[:4]
            if not chosen:
                trajectory.append({"round": round_no, "stop": "Jev held every dial", "ledger": ref})
                break
            best, tried = None, {}
            for d, a in chosen:
                candidate = self.move(d, 1 if a["choice"] == "up" else -1, over)
                if candidate is None:
                    continue
                r = self.evaluate(candidate, "direction", round_no, f"direction:{d}:{a['choice']}")
                tried[f"{d}:{a['choice']}"] = r["score"]
                if r["score"] is not None and (best is None or r["score"] < best[1]["score"]):
                    best = (f"{d}:{a['choice']}", r, candidate)
            if best is None or best[1]["score"] >= current["score"] * 0.99:
                trajectory.append({"round": round_no, "stop": "no proposed move improved the score by 1%", "ledger": ref, "tried": tried})
                break
            m, r, candidate = best
            over, current = candidate, r
            history.append({"round": round_no, "move": m, "score": r["score"]})
            trajectory.append({"round": round_no, "accepted": m, "score": r["score"], "key": r["key"], "ledger": ref, "tried": tried})
        return {"arm": "direction", "final": over, "trajectory": trajectory}

    def arm_sweep(self, rounds: int) -> dict:
        over, trajectory = {}, []
        current = self.evaluate(over, "sweep", 0, "shipped")
        trajectory.append({"round": 0, "score": current["score"], "key": current["key"]})
        for round_no in range(1, rounds + 1):
            moves = self.moves(over)
            scored = {}
            for m, (_, _, moved) in moves.items():
                r = self.evaluate(moved, "sweep", round_no, f"sweep:{m}")
                if r["score"] is not None:
                    scored[m] = r["score"]
            improving = [m for m, s in scored.items() if s < current["score"]]
            candidates = dict(scored)
            if len(improving) > 1:
                compound = json.loads(json.dumps(over))
                for m in improving:
                    dial, direction, _ = moves[m]
                    moved = self.move(dial, direction, compound)
                    if moved is not None:
                        compound = moved
                r = self.evaluate(compound, "sweep", round_no, "sweep:compound")
                if r["score"] is not None:
                    candidates["compound:" + "+".join(improving)] = r["score"]
                    moves["compound"] = (None, None, compound)
            best = min(candidates.items(), key=lambda kv: kv[1]) if candidates else None
            if best is None or best[1] >= current["score"] * 0.99:
                trajectory.append({"round": round_no, "stop": "no single step or compound improved the score by 1%", "tried": scored})
                break
            m, s = best
            over = moves["compound"][2] if m.startswith("compound") else moves[m][2]
            current = self.cache[self.key(over)]
            trajectory.append({"round": round_no, "accepted": m, "score": s, "key": current["key"], "tried": scored})
        return {"arm": "sweep", "final": over, "trajectory": trajectory}

    def write(self, arms: list[dict], seconds: float) -> None:
        columns = ["arm", "round", "label", "key", "cached", "feasible", "reason", "score", "seconds", "height_m", "dbh_m", "wood_triangles", "leaves"]
        columns += [f"{r['id']}.{n}" for r in self.records for n in NUMBERS] + ["overrides"]
        with (HERE / f"trials{self.tag}.tsv").open("w", newline="") as f:
            w = csv.DictWriter(f, fieldnames=columns, delimiter="\t", extrasaction="ignore")
            w.writeheader()
            for row in self.trials:
                w.writerow(row)
        (HERE / f"arms{self.tag}.json").write_text(json.dumps({
            "preset": PRESET, "seed": self.seed, "photograph": self.photo, "dials": {d: {"path": p, "step": s, "bounds": b} for d, (p, _, s, b) in DIALS.items()},
            "evaluations": self.evals, "wall_seconds": round(seconds), "ledger": self.ledger_refs, "arms": arms,
        }, indent=1) + "\n")


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--stills", required=True, type=Path)
    ap.add_argument("--arms", default="jev,sweep")
    ap.add_argument("--rounds", type=int, default=4)
    ap.add_argument("--max-evals", type=int, default=70)
    ap.add_argument("--seed", type=int, default=1)
    ap.add_argument("--tag", default="", help="suffix for trials and arms files, so a second run keeps the first")
    args = ap.parse_args()
    for binary in (MEASURE, HEADLESS):
        if not binary.exists():
            print(f"build first: {binary}", file=sys.stderr)
            return 2
    probe = Probe(args.stills, args.seed, args.max_evals, args.tag)
    print("photograph:", json.dumps(probe.photo), flush=True)
    started = time.time()
    arms = []
    try:
        for arm in args.arms.split(","):
            runner = {"jev": probe.arm_jev, "sweep": probe.arm_sweep, "direction": probe.arm_direction}[arm]
            arms.append(runner(args.rounds))
            probe.write(arms, time.time() - started)
    finally:
        probe.write(arms, time.time() - started)
    print(json.dumps({a["arm"]: [t.get("score", t.get("stop")) for t in a["trajectory"]] for a in arms}))
    return 0


if __name__ == "__main__":
    sys.exit(main())
