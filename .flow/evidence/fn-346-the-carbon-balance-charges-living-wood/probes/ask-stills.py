#!/usr/bin/env python3
"""Puts the same narrow questions to Jev about single stills.

Usage: ask-stills.py <out.json> <label=path.png>...
Each still is converted to a JPEG inside the limits, asked through the shared
caller (`jev ask --assets`), and the answers are written to <out.json>.
Jev's answers are evidence beside the host's own view, never a verdict.
"""
import json
import os
import subprocess
import sys

WORKTREE = "/home/daniel/Projects/telperion/.worktrees/fn-348"

QUESTIONS = {
    "crown": {
        "type": "choice",
        "instructions": "Where does the tree in `still` carry its leaves?",
        "criteria": {
            "through": "Leaves fill the crown through its depth, along the limbs as well as at their ends.",
            "ends": "Leaves sit mostly in tufts at the ends of long bare limbs.",
            "sparse": "The tree carries very few leaves anywhere.",
            "cannot_tell": "No tree is shown, or it cannot be told.",
        },
    },
    "limbs": {
        "type": "choice",
        "instructions": "How do the main limbs of the tree in `still` stand?",
        "criteria": {
            "held_up": "The main limbs rise or reach outward and are held clear of the ground.",
            "on_ground": "One or more main limbs droop down to the ground or lie along it.",
            "no_limbs": "The tree has a trunk with no large side limbs, only a top.",
            "cannot_tell": "No tree is shown, or it cannot be told.",
        },
    },
    "trunk": {
        "type": "choice",
        "instructions": "What is the shape of the trunk of the tree in `still`?",
        "criteria": {
            "straight": "A straight or nearly straight upright trunk.",
            "snaking": "A long trunk that bends or snakes from side to side on its way up.",
            "short_forked": "A short trunk that soon divides into several large limbs.",
            "cannot_tell": "No tree is shown, or it cannot be told.",
        },
    },
    "reads_as": {
        "type": "choice",
        "instructions": "Taken as a whole, what does the tree in `still` most look like?",
        "criteria": {
            "healthy_broadleaf": "A healthy broadleaved tree with a full crown, as an oak in a park or field.",
            "old_spreading": "An old, wide-spreading broadleaved tree with heavy limbs and a thinner crown.",
            "pole_with_tuft": "A tall bare stem with a tuft of crown at the top.",
            "broken": "A collapsed, tangled or broken tree that does not look like a living tree in good form.",
            "cannot_tell": "No tree is shown, or it cannot be told.",
        },
    },
}


def ask(label, path, work):
    jpg = os.path.join(work, label + ".jpg")
    subprocess.run(["magick", path, "-resize", "1568x1568>", "-strip", "-quality", "90", jpg], check=True)
    key = "still-" + label
    files = {}
    for name, value in (
        ("state", {"still": key, "about": "One rendered tree on open ground. The small dark figure is a person, for scale."}),
        ("questions", QUESTIONS),
        ("assets", {key: jpg}),
    ):
        files[name] = os.path.join(work, f"{label}-{name}.json")
        with open(files[name], "w") as handle:
            json.dump(value, handle)
    command = (
        f"cd {WORKTREE} && cargo run -q --profile ci -p telperion-jev --bin jev -- ask --tool ask-still "
        f"--state {files['state']} --questions {files['questions']} --assets {files['assets']}"
    )
    done = subprocess.run(["bash", "-ic", command], capture_output=True, text=True)
    text = done.stdout
    start = text.find("{")
    if done.returncode != 0 or start < 0:
        return {"error": (done.stderr or text)[-400:]}
    return json.loads(text[start:])


def main():
    out, pairs = sys.argv[1], sys.argv[2:]
    work = os.path.join(os.path.dirname(out), "ask-work")
    os.makedirs(work, exist_ok=True)
    answers = {}
    for pair in pairs:
        label, path = pair.split("=", 1)
        answers[label] = ask(label, path, work)
        print(label, "done", flush=True)
    with open(out, "w") as handle:
        json.dump(answers, handle, indent=1)


main()
