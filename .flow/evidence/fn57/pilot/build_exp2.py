#!/usr/bin/env python3
"""Experiments 4 and 5: owner-feedback triage and report section selection."""
import json
import pathlib
import re

REPO = pathlib.Path("/home/daniel/Projects/telperion")

SPECS = {
    "fn-4": "Limbs that pass through each other, and the blunt ends they stop at: close up, limbs pass through one another, meet in hard creases where they cross, and several stop dead",
    "fn-10": "Legendary tree profiles and supernatural traits: famous magical or legendary trees with reference material",
    "fn-15": "Wind and structural motion: coherent movement from trunk through branches to leaves",
    "fn-16": "Environment and damage response: light, competition, pruning and breakage influence subsequent growth",
    "fn-17": "First external engine integration: prove the generator in an external engine or block-world consumer",
    "fn-20": "Woody anatomy from roots through branch junctions: connected base, trunk and forks under close geometric inspection",
    "fn-21": "Developmental shoots and within-species variation: crown and shoot plausibility through developmental causes and controlled variation, beginning with oak and spruce",
    "fn-28": "A tree that grows smoothly as the page scrolls: the owner's website scrolls and a tree grows",
    "fn-29": "Colour, cavity and occlusion: trunk, branch and leaf scale relief, veins, translucency, colour and cavity shading in the stills",
    "fn-31": "Growth rule: sapling form, thickening by age, a shedding floor: the spec under QA right now; the oak and spruce at 1 to 30 years read as saplings that continue into the mature tree, trunk thickens with age, crown stays rounded and broad",
    "fn-43": "Sprouts and leaf size by age: every leaf and needle is drawn at mature full size from the day its shoot is born, so a seedling carries leaves as large as an old tree's",
    "fn-53": "No hardcoded caps on generation: node and leaf caps in tests and generator become validated parameters",
}

ROUTE_CRIT = {k: v for k, v in SPECS.items()}
ROUTE_CRIT["new_spec"] = "No open spec covers this defect; it needs a new spec"

ROUTE_CASES = [
    ("Looking at the 20-year oak, one side of the crown has a hollow under a single reaching limb; it fills in by 26.7 years.", "fn-31"),
    ("Two of the oak's limbs pass straight through each other near the fork and there is a hard crease where they cross.", "fn-4"),
    ("The one-year oak seedling carries leaves as big as the mature tree's; they dwarf the stem.", "fn-43"),
    ("The spruce bark reads flat; there is no shading in the cavity between the trunk and the branch collar, and the needles show no translucency against the light.", "fn-29"),
    ("At all ages the branching looks weird and too straight. Especially mature the branches grow upwards.", "fn-31 or fn-21 (posture of spruce primaries at every age; filed under fn-31 in QA)"),
    ("When I scroll the demo page the tree jumps between ages instead of growing continuously.", "fn-28"),
    ("The ten-year oak test still has a node cap of 700 in it.", "fn-53"),
    ("The oak's trunk should split and heal over where a limb was cut off two years earlier.", "fn-16"),
]

DUP_Q = {
    "same_defect": {
        "type": "noul",
        "instructions": "Is `new_observation` a report of the same defect as `prior_finding`, so that filing it again would be a duplicate? "
                        "Same defect means the same wrong form on the same tree, even if seen at a different age or seed; "
                        "a different wrong form, or the same kind of form on a different part of the tree, is a different defect.",
        "criteria": {
            "true": "The two describe one wrong form: the same part of the tree, the same species, the same failure; a fix for one would fix the other",
            "false": "A different part of the tree, a different failure, or a different species; a fix for one would leave the other",
        },
    },
}

NOTES = {
    "hollow": "After round 12, the oak's crown at 20 and 22 years has a hollow in one side under a single reaching limb: on the left of seed 7, and smaller in the middle-left of seed 42 at 22 years. It shrinks by 24 years and is gone by 26.7.",
    "tuft": "After round 11, the 20-year Oregon white oak carries a separate tuft of foliage above its main crown, with a gap between them, on seeds 7 and 42. By 26.7 years the crown is a broad dome with no gap.",
    "umbrella": "The 26.7-year Oregon white oak is a flat-topped umbrella: a wide crown on a bare stem with a separate small cluster of branches below it. At 24 y a flat spreading layer appears at the top of a thin leader, and at 26.7 y the crown is a wide flat top with a gap above a small lower cluster.",
    "bush": "After round 9, the ten-year Oregon white oak reads as a leafy bush with almost no visible stem on seeds 1 and 7. A 2.3 m bush, 2.2 m wide, with foliage from near the ground.",
    "shrub_r5": "Owner, round 5: the ten-year oak is a shrub with no visible stem and nothing leans.",
    "bottle": "The ten-year Oregon white oak's stem flares into a bottle at the ground in the live harness, on both seed 7 and seed 1. A thick flared base tapering sharply into the stem; the flare is below breast height.",
    "bell_r6": "Owner, round 6: the trunk base is bell-shaped, an awful regression.",
    "rays": "The primary branches of the Norway spruce are drawn as straight rays that rise from the trunk at every crown height, from 26.6 years to maturity. Mean chord elevation +21 to +28 degrees in the lower three fifths of the crown.",
    "gangly": "The 14.1-year Norway spruce does not read as a continuation between the 5-year and 26.6-year trees: it is a sparse, gangly tree with long, thin, straight laterals, and the 5-year sapling carries short straight stubs like a bottle brush.",
}

DUP_CASES = [
    ("bush", "shrub_r5", "true: same defect, ten-year oak with no stem"),
    ("bottle", "bell_r6", "true: same defect, flared trunk base"),
    ("hollow", "tuft", "false: side hollow vs tuft above"),
    ("tuft", "umbrella", "leaning true? same kind of crown gap at a different age; QA filed separately"),
    ("gangly", "rays", "false-ish: sparse sapling vs primary posture; QA marked related"),
    ("bottle", "bush", "false: base flare vs bushy crown"),
    ("hollow", "umbrella", "false: side hollow at 20 vs flat top at 26.7"),
]

SEV_Q = {
    "severity": {
        "type": "score",
        "instructions": "How severe is the defect in `observation` against the owner's standard in `standard`, for a spec whose acceptance rests on the owner's eye?",
        "criteria": [
            {"summary": "Cosmetic: a small local blemish the owner did not name and that resolves on its own within a few years of growth", "signals": ["minor", "otherwise continuous", "gone by a later age"]},
            {"summary": "Noticeable: a wrong form visible at the hero pose that a viewer would remark on, but the tree still reads as the species and age", "signals": ["visible at one age", "one seed"]},
            {"summary": "Blocking: a form the owner has already rejected by name, or one that makes the tree read as a different tree or age", "signals": ["owner rejected", "regression", "does not read as the species"]},
        ],
    },
}

SEV_CASES = [
    ("hollow", "The owner's round-6 decision: a rounded, broad oak crown. The QA note adds: minor, the crown is otherwise continuous.", "cosmetic to noticeable"),
    ("bush", "The owner rejected this form in the round-5 verdict: the ten-year oak is a shrub with no visible stem and nothing leans. Round 6 asked for a visible leaning stem and woody laterals.", "blocking"),
    ("bottle", "The owner rejected a bell-shaped trunk base in round 6 as an awful regression.", "blocking"),
    ("rays", "R1: every later age reads as a continuation of the same Norway spruce; the species carries ascending upper, horizontal mid and drooping lower branches.", "noticeable to blocking"),
]


def route_cases():
    out = []
    for obs, expect in ROUTE_CASES:
        out.append({"label": f"route: {obs[:70]}", "expect": expect, "request": {
            "state": {"owner_observation": obs, "open_specs": SPECS},
            "questions": {"spec": {"type": "choice",
                                   "instructions": "Which open spec in `open_specs` owns the defect the owner describes in `owner_observation`? Pick the spec whose scope the fix belongs to.",
                                   "criteria": ROUTE_CRIT}}}})
    return out


def dup_cases():
    out = []
    for a, b, expect in DUP_CASES:
        out.append({"label": f"dup: {a} vs {b}", "expect": expect, "request": {
            "state": {"new_observation": NOTES[a], "prior_finding": NOTES[b]}, "questions": DUP_Q}})
    return out


def sev_cases():
    out = []
    for a, std, expect in SEV_CASES:
        out.append({"label": f"severity: {a}", "expect": expect, "request": {
            "state": {"observation": NOTES[a], "standard": std}, "questions": SEV_Q}})
    return out


def section_cases():
    md = (REPO / ".flow/evidence/fn30/REPORT.md").read_text()
    parts = re.split(r"^## ", md, flags=re.M)
    sections = {}
    for p in parts[1:]:
        title, _, body = p.partition("\n")
        sections[title.strip()] = body.strip()[:700]
    qs = {
        "reference_table": {"type": "choice",
                             "instructions": "Which section of `report` holds the table of fetched reference source files with their SHA-256 and byte counts?",
                             "criteria": {k: None for k in sections}},
        "owner_slot": {"type": "choice",
                        "instructions": "Which section of `report` is the slot where the owner records a verdict?",
                        "criteria": {k: None for k in sections}},
        "composition": {"type": "choice",
                         "instructions": "Which section of `report` explains how the height and diameter curves were composed from the sources, including stand-grown fallbacks?",
                         "criteria": {k: None for k in sections}},
    }
    return [{"label": "sections of fn30 REPORT.md", "expect": "The references / Owner verdict / The curves and their composition",
             "request": {"state": {"report": sections}, "questions": qs}}]


if __name__ == "__main__":
    json.dump(route_cases() + dup_cases() + sev_cases(), open("exp4_triage.json", "w"), indent=1)
    json.dump(section_cases(), open("exp5_sections.json", "w"), indent=1)
    print("built")
