#!/usr/bin/env python3
"""fn-371 R5: the ordinary tree at 40 on seeds 1001 and 1007, whole and mid
(the scoreboard's `crown` shot), a column per run, height, trunk and leaf
under each tile. Built on fn-352's variants.py (its montage); fn-364's
sheet.py adapted for runs whose stills are named differently.

    python3 -I sheet.py <out.jpg> <name>=<measures dir>:<stills dir>:<stem>[:<leaf.jsonl>] ...

<stem> is the stills' name before `-<seed>-<shot>.png` (`ordinary-preset`
on the scoreboard's runs, `ordinary-40` on this spec's). The optional
leaf.jsonl is fn-370's leaf probe's lines, for the grower's leaf.
"""
import json
import sys
from pathlib import Path

E = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(E / "fn-352-the-scoreboard-a-fixed-panel-of-trees" / "scoreboard"))
import variants as v  # noqa: E402

SEEDS = [1001, 1007]
SHOTS = [("whole", "whole"), ("crown", "mid")]


def lines(path):
    if not path.exists():
        return []
    return [json.loads(l) for l in path.read_text().splitlines() if l.strip()]


def run(arg):
    name, rest = arg.split("=", 1)
    parts = rest.split(":")
    measures, stills, stem = Path(parts[0]), Path(parts[1]), parts[2]
    trees = {t["seed"]: t for t in lines(measures / "measures.jsonl")
             if t["preset"] == "ordinary" and float(t["age"]) == 40}
    grown = {}
    if len(parts) > 3:
        grown = {l["seed"]: l["grower_m2"] for l in lines(Path(parts[3])) if l["age"] == 40}
    return name, stills, stem, trees, grown


def label(name, seed, view, t, grown):
    head = f"{name}: ordinary 40, seed {seed} ({view})"
    if t is None:
        return head + "\nnot measured in this run"
    dbh = t["dbh"]
    g = f"grown {grown:.0f} m2" if grown is not None else "grown: not measured"
    return (f"{head}\n{t['height']:.1f} m tall, trunk {dbh * 100:.1f} cm, "
            f"height/trunk {t['height'] / dbh:.0f}\nleaf drawn {t['leaf_area']:.0f} m2, {g}")


def main():
    out, *args = sys.argv[1:]
    rs = [run(a) for a in args]
    tiles = []
    for seed in SEEDS:
        for shot, view in SHOTS:
            for name, stills, stem, trees, grown in rs:
                png = stills / f"{stem}-{seed}-{shot}.png"
                tiles += ["-label", label(name, seed, view, trees.get(seed), grown.get(seed)),
                          str(png) if png.exists() else "xc:gray80[960x720]"]
    names = ", ".join(r[0] for r in rs)
    v.montage(tiles, len(rs), f"ordinary 40, whole and mid; columns {names}", Path(out))


if __name__ == "__main__":
    main()
