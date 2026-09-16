#!/usr/bin/env python3
"""Ask Jev which beech preset rows answer each of the owner's round-22 notes,
and which way to move them. Jev only picks raise / lower / neither; code wrote
every row description and will choose and measure every value."""
from __future__ import annotations

import json
import os
import sys
import urllib.request
from pathlib import Path

OUT = Path(__file__).parent

# The owner's round-22 beech verdict, split into its sentences by hand.
NOTES = {
    "outline": "The tree has clear regular outline/border that doesn't look natural.",
    "rim_taper": "The taper approaching the border needs to produce thinner branches twigs.",
    "profile": "It's also too dense at the shoulder of the crown. It's more sparse lower and gets more dense at the top.",
    "density": "It generally looks too dense everywhere?",
}

# Rows of the beech preset, described from their definitions in the core.
ROWS = {
    "irregularity": (0.18, "How far the crown's outline departs from a smooth shell, as a fraction of the radius there. 0 is a perfectly smooth, regular outline."),
    "lobe_scale": (0.7, "The size of those outline bumps across the crown: small values give many small lumps, 1 gives lobes as long as the tree is tall."),
    "fullness": (0.3, "Where the crown is widest, as a share of the crown's depth measured up from the crown base. 0.3 is widest low down; higher moves the widest part up toward the top."),
    "shoulder": (1.8, "The shape of the crown's sides between its base, widest point and top. 2 is an ellipse; higher is boxier with full, square shoulders; lower is more pointed and diamond-like."),
    "spread": (0.36, "The crown's greatest radius as a share of the tree's height."),
    "limb_clumping": (0.25, "How deep the leaf-free gaps between neighbouring limb systems reach. 0 is one continuous leaf mass; higher leaves each limb system its own rounded clump with open sky between."),
    "twig_laterals": (4, "Side twigs grown at each station along a branch. More twigs means denser fine wood and more leaves."),
    "twig_length_ratio": (0.23, "Length of a side twig against the branch that bears it."),
    "twig_tip_taper": (0.25, "Radius of a twig at its far tip against its nominal radius. Lower makes twigs thinner toward their ends."),
    "length_taper": (0.2, "How fast wood radius falls per metre along a branch. Higher makes branches thin out faster toward the crown's edge."),
    "lateral_length_ratio": (0.65, "Length of a side branch against the limb that bears it."),
    "leaf_internode": (0.05, "Metres of shoot between one leaf and the next on long shoots. Larger spacing means fewer leaves."),
    "short_shoot_spacing": (0.03, "Metres of limb wood between short spur shoots, each ending in a leaf cluster. Larger spacing means fewer clusters."),
    "short_shoot_leaves": (8, "Leaves in each spur's cluster, 1 to 8."),
}


def request(note_key: str) -> dict:
    state = {
        "tree": "A procedurally generated mature open-grown European beech (Fagus sylvatica), about 32 m tall, rendered beside a photograph of a real one. The owner judged the render and wrote a note.",
        "owner_note": NOTES[note_key],
        "rows": {k: {"current_value": v, "what_it_controls": d} for k, (v, d) in ROWS.items()},
    }
    questions = {
        key: {
            "type": "choice",
            "instructions": (
                f"Consider only the preset row `rows.{key}` and the owner's complaint in `owner_note`. "
                "If this row were changed on its own, which direction would make the rendered beech better answer the complaint? "
                "Choose neither when this row has little or nothing to do with what the note describes."
            ),
            "criteria": {
                "raise": f"Raising `rows.{key}` above its current value would directly answer the complaint.",
                "lower": f"Lowering `rows.{key}` below its current value would directly answer the complaint.",
                "neither": f"`rows.{key}` does not govern what the complaint describes, or changing it would not help.",
            },
        }
        for key in ROWS
    }
    return {"model": "jev-latest", "state": state, "questions": questions}


def call(body: dict) -> dict:
    key = os.environ.get("TYPESAFE_API_KEY")
    if not key:
        sys.exit("TYPESAFE_API_KEY is not set in this shell")
    req = urllib.request.Request(
        "https://api.typesafe.ai/v1/systemone",
        data=json.dumps(body).encode(),
        headers={"Authorization": f"Bearer {key}", "Content-Type": "application/json"},
    )
    with urllib.request.urlopen(req, timeout=120) as r:
        return json.loads(r.read())


if __name__ == "__main__":
    results = {}
    for note in NOTES:
        answers = call(request(note))["answers"]
        results[note] = {row: {"choice": a.get("choice"), "probabilities": a.get("probabilities"),
                               "confidence": a.get("confidence")} for row, a in answers.items()}
    (OUT / "mapping.json").write_text(json.dumps({"notes": NOTES, "rows": {k: v for k, (v, _) in ROWS.items()},
                                                  "answers": results}, indent=1))
    for note, rows in results.items():
        print(f"\n{note}: {NOTES[note]}")
        for row, a in sorted(rows.items(), key=lambda kv: -max((kv[1]['probabilities'] or {}).get(c, 0) for c in ('raise', 'lower'))):
            p = a["probabilities"] or {}
            if a["choice"] != "neither":
                print(f"  {a['choice']:>6} {row:22s} raise {p.get('raise', 0):.2f} lower {p.get('lower', 0):.2f} neither {p.get('neither', 0):.2f}")
