#!/usr/bin/env python3
"""Emit explicitly authored calibration labels, then freeze via the Rust question generator."""
import argparse
import json
from pathlib import Path

p = argparse.ArgumentParser()
p.add_argument("--out", type=Path, required=True)
a = p.parse_args()
a.out.mkdir(parents=True, exist_ok=True)
dials = [
    dict(id="limbs", path="/skeleton/habit/lateralsPerStation", meaning="limbs born at each station", min=1, max=4, integer=True, small=1, substantial=2),
    dict(id="leaves", path="/canopy/shortShootLeaves", meaning="leaves per cluster", min=2, max=12, integer=True, small=2, substantial=4),
    dict(id="spacing", path="/canopy/shortShootSpacing", meaning="metres between leaf clusters", min=0.01, max=0.08, integer=False, small=0.01, substantial=0.02),
    dict(id="irregularity", path="/skeleton/envelope/irregularity", meaning="crown envelope lobes and hollows", min=0, max=0.5, integer=False, small=0.08, substantial=0.16),
    dict(id="crookedness", path="/skeleton/habit/crookedness", meaning="limb turning and zigzag amount", min=0, max=15, integer=False, small=3, substantial=6),
    dict(id="taper", path="/skeleton/habit/twigTipTaper", meaning="remaining wood thickness toward the crown edge; lower means thinner tips", min=0.05, max=0.6, integer=False, small=0.1, substantial=0.2),
]

def write(name, value):
    with (a.out / name).open("x") as f:
        json.dump(value, f, indent=2)

def case(id, split, dial, current, note, expected, provenance="authored bounded-action interpretation; not biological efficacy", unsafe=()):
    return dict(id=id, split=split, provenance=provenance,
                state=dict(dial=dial, current=current, owner_observation=note), questions={},
                expected={"adjustment": expected}, unsafe_answers={"adjustment": list(unsafe)})

def manifest(kind, cases, version):
    return dict(schema="tuning-calibration-v1", question_version=version, table_sha256="",
                kind=kind, model="jev-1.13.0", min_accuracy=0.8, min_confidence=0.5, cases=cases)

write("dials.json", dials)
magnitude = [
    case("example-up", "tuning", dials[0], 2, "Increase by the small authored adjustment, to three limbs.", "small_increase"),
    case("example-none", "tuning", dials[0], 2, "No observations are available.", "insufficient_evidence"),
    case("small-down", "heldout", dials[0], 3, "A prior test supports exactly two limbs; make that bounded move.", "small_decrease"),
    case("large-down", "heldout", dials[0], 3, "The measured intervention supports exactly one limb per station.", "substantial_decrease"),
    case("large-up", "heldout", dials[0], 1, "Evidence supports exactly three limbs per station, not two.", "substantial_increase"),
    case("hold", "heldout", dials[0], 2, "The current limb count satisfies the requirement; do not change it.", "hold"),
    case("ambiguous-size", "heldout", dials[1], 8, "Density seems wrong but neither direction nor adjustment size is supported.", "insufficient_evidence"),
    case("upper-bound", "heldout", dials[0], 3, "Exactly four limbs is supported. Never exceed the authored maximum.", "small_increase", unsafe=["substantial_increase"]),
    case("lower-bound", "heldout", dials[0], 1, "Fewer limbs might help but the required value would be below the minimum; there is no supported allowed adjustment.", "insufficient_evidence", unsafe=["small_decrease", "substantial_decrease"]),
    case("fractional-small", "heldout", dials[2], 0.03, "A prior measured trial supports cluster spacing exactly 0.04 metres.", "small_increase"),
]
write("magnitude-authoring.json", manifest("magnitude", magnitude, "tuning-adjustment-v1"))
historical = "historical fn34 round22 owner notes, experiments/fn58-tuning-loop/loop.py NOTES; correlated cases from one verdict, no independent species-generalization claim"
direction = [
    case("example-up", "tuning", dials[0], 2, "Need more limbs at each station.", "increase"),
    case("example-hold", "tuning", dials[0], 2, "Limb count is already right.", "hold"),
    case("dense-limbs", "heldout", dials[0], 2, "It's too dense at the shoulder of the crown. It generally looks too dense everywhere.", "decrease", historical),
    case("dense-leaves", "heldout", dials[1], 8, "It generally looks too dense everywhere.", "decrease", historical),
    case("dense-spacing", "heldout", dials[2], 0.03, "It generally looks too dense everywhere.", "increase", historical),
    case("regular-outline", "heldout", dials[3], 0.1, "The tree has clear regular outline/border that doesn't look natural.", "increase", historical),
    case("thick-tips", "heldout", dials[5], 0.3, "The taper approaching the border needs to produce thinner branches twigs.", "decrease", historical),
]
write("direction-authoring.json", manifest("direction", direction, "tuning-direction-v1"))
continuation = []
for id, split, note, expected in [
    ("bounded-dial", "tuning", "Measured defect; existing dial supported by prior trial; bounded 1000-token adjustment; first attempt; isolated preset and numeric/visual verification.", ("supported", "supported", "bounded")),
    ("unknown", "tuning", "No defect evidence, no cost estimate, effects unknown.", ("insufficient_evidence", "insufficient_evidence", "insufficient_evidence")),
    ("progress", "heldout", "Prior dial trial improved measured and visual defect. New bounded adjustment uses same isolated preset and verified gates; 1000 tokens estimated from previous request.", ("supported", "supported", "bounded")),
    ("complex-supported", "heldout", "Complex feature has reviewed design, bounded implementation plan, tests and isolated effects. First evidenced attempt, estimate 4000 tokens from reviewed scope.", ("supported", "supported", "bounded")),
    ("repeated-failure", "heldout", "Same dial change failed twice, no new evidence or changed approach. Diagnosis contradicted by measurements; effects remain limited to preset.", ("unsupported", "repeated_failure", "bounded")),
    ("unusual-risk", "heldout", "Diagnosis supported, first evidence-based attempt, but proposed shared renderer rewrite risks all species and no bounded verification of those effects exists.", ("supported", "supported", "unusual")),
]:
    continuation.append(dict(id=id, split=split, provenance="authored continuation-policy case, host decision table 2026-09-19", state={"evidence": note}, questions={},
        expected=dict(zip(["tractability", "progress", "risk"], expected)), unsafe_answers={}))
write("continuation-authoring.json", manifest("continuation", continuation, "continuation-v1"))
