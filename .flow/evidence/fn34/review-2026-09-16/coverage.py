#!/usr/bin/env python3
"""Jev coverage check: for each note the owner recorded on the beech and the
birch, which planned work item addresses it, if any. Jev picks from the items
code lists; code tallies."""
from __future__ import annotations

import json
import os
import sys
import urllib.request
from pathlib import Path

HERE = Path(__file__).parent

# The owner's words, verbatim, from the fn-34 spec, the task files, the vault
# handoff and this session. (tree, source, words)
NOTES = [
    ("beech", "round 5", "I still have an issue with the beech which is clearly not structurally sound. The reference grows relatively straight up and out. Our generation bends too much."),
    ("beech", "handoff 09-15", "material is bad structure is off"),
    ("both", "handoff 09-15", "it's all still to plasticesque need to make it rough less reflective. All the materials have this problem."),
    ("birch", "round 22 verdict", "It's good but the texturing regressed. It's too much now too much contrast. The texture in the reference isn't black."),
    ("beech", "round 22 verdict", "The tree has clear regular outline/border that doesn't look natural."),
    ("beech", "round 22 verdict", "The taper approaching the border needs to produce thinner branches twigs."),
    ("beech", "round 22 verdict", "It's also too dense at the shoulder of the crown. It's more sparse lower and gets more dense at the top."),
    ("beech", "round 22 verdict", "It generally looks too dense everywhere?"),
    ("beech", "after round 23", "We have trees where the branches point upwards when there are no leaves. But they droop under the weight of leaves."),
    ("beech", "after round 23", "We still haven't achieved the reference for the bare one either. It still looks off."),
    ("beech", "after round 23", "But the bigger issue is with the whole tree that doesn't match the reference at all."),
    ("both", "after round 23", "Also i haven't seen any leaf side by side with a reference?"),
]

ITEMS = {
    "fn59_leaf_load": "fn-59: branches bend under the weight of the leaves they carry, only in the leaf-on state; the beech's crown base goes back up to the bare photograph's, so it stands on a clear trunk in winter and hangs low in summer.",
    "fn60_leaves": "fn-60: leaf blades get toothed margins and a smooth outline, the leaf view is checked for lighting, and a shoot view is added; each species' leaf is compared beside its leaf and shoot photographs.",
    "fn55_matte": "fn-55: every material gets a physical, matte highlight and a pixel-scale grain, so bark and leaves stop reading as plastic.",
    "beech_values": "The next beech value round: more crooked, zigzagging branches, twigs thinning toward the crown's edge, a lumpier irregular outline, a thinner crown with its widest part higher, and the pale lichen that whitens the wood toned down.",
    "birch_bark_values": "The birch bark value round: the near-black peeled patches and dark lenticel dashes lightened toward the photograph's grey.",
    "leaf_pairs_done": "Already done this session: the leaf renders were paired beside the leaf and shoot photographs and put on the judging page.",
    "uncovered": "None of the planned items addresses this note.",
}


def request(tree: str, source: str, words: str) -> dict:
    state = {
        "project": "A procedural tree generator. Its European beech and silver birch presets are being judged by the owner against photographs of real trees.",
        "note": {"tree": tree, "when": source, "owner_words": words},
        "planned_work": ITEMS,
    }
    return {
        "model": "jev-latest",
        "state": state,
        "questions": {
            "addressed_by": {
                "type": "choice",
                "instructions": "Which single item in `planned_work` most directly addresses the complaint in `note.owner_words`, for the tree in `note.tree`? Choose uncovered when no item would fix what the owner describes.",
                "criteria": {k: v for k, v in ITEMS.items()},
            },
            "fully": {
                "type": "noul",
                "instructions": "Taken together, would the items in `planned_work` other than uncovered fully resolve what `note.owner_words` complains about, for the tree in `note.tree`, rather than only part of it?",
                "criteria": {"true": "The planned items together cover the whole complaint.", "false": "Some part of the complaint is left unaddressed by every planned item."},
            },
        },
    }


def call(body: dict) -> dict:
    key = os.environ.get("TYPESAFE_API_KEY")
    if not key:
        sys.exit("TYPESAFE_API_KEY is not set in this shell")
    req = urllib.request.Request("https://api.typesafe.ai/v1/systemone", data=json.dumps(body).encode(),
                                 headers={"Authorization": f"Bearer {key}", "Content-Type": "application/json"})
    with urllib.request.urlopen(req, timeout=120) as r:
        return json.loads(r.read())


if __name__ == "__main__":
    rows = []
    for tree, source, words in NOTES:
        a = call(request(tree, source, words))["answers"]
        pick = a["addressed_by"]
        probs = pick.get("probabilities") or {}
        second = sorted(probs.items(), key=lambda kv: -kv[1])[1] if len(probs) > 1 else (None, 0)
        rows.append({"tree": tree, "when": source, "words": words, "addressed_by": pick.get("choice"),
                     "p": round(probs.get(pick.get("choice"), 0), 2), "runner_up": [second[0], round(second[1], 2)],
                     "fully": round(a["fully"].get("noul", 0), 2)})
    (HERE / "coverage.json").write_text(json.dumps(rows, indent=1))
    for r in rows:
        print(f"{r['tree']:5} {r['when']:17} -> {r['addressed_by']:17} {r['p']:.2f} (next {r['runner_up'][0]} {r['runner_up'][1]:.2f}) fully {r['fully']:.2f} | {r['words'][:70]}")
