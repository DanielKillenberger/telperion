#!/usr/bin/env python3
"""fn-341 R4: the retained sheets from r4-stills.sh's stills, the cycle on
the left and the probe (G, P and U) on the right, in the form of fn-336's and
fn-340's sheets. Run from the repository root; needs ImageMagick 7 (`magick`).
"""
import subprocess
from pathlib import Path

E = Path(".flow/evidence/fn-341-the-earned-structure-the-second-proof")
RAW = E / "raw" / "stills"
PROBE = "probe: girth by rings (G), posture by gravitropic performance (P), upkeep on living wood (U)"


def sheet(out, title, rows):
    tiles = []
    for name, age, seed, shot, extra in rows:
        for mode in ("cycle", "probe"):
            path = RAW / mode / f"{name}-{age}-{seed}-{shot}.png"
            label = f"{'cycle' if mode == 'cycle' else 'probe'}, {name} {age}, seed {seed}, {shot}{extra}"
            tiles += ["-label", label, str(path) if path.exists() else "xc:gray80"]
    subprocess.run(["magick", "montage", *tiles, "-tile", "2x", "-geometry", "640x480+6+6",
                    "-pointsize", "18", str(out)], check=True)
    subprocess.run(["magick", str(out), "-background", "white", "-gravity", "north",
                    "-splice", "0x110", "-pointsize", "24", "-annotate", "+0+12", title,
                    "-strip", "-quality", "82", str(out)], check=True)
    print(out)


def main():
    for shot, view in (("whole", "whole"), ("crown", "mid")):
        sheet(E / f"sheet-oak-{view}.jpg",
              f"fn-341 R4: Oregon white oak, {view} view. Left: the cycle. Right: the {PROBE}.",
              [("oak", a, s, shot, "") for a in (40, 120, 300) for s in (1001, 1007)])
        sheet(E / f"sheet-beech-{view}.jpg",
              f"fn-341 R4: European beech, {view} view. Left: the cycle. Right: the probe; P and U are dormant\n"
              "on the beech (no sag, no upkeep), so it stands under G alone: its trunk is what its leaf earns,\n"
              "and its leaf is a fifth of a real beech's (fn-340).",
              [("beech", 40, s, shot, "") for s in (1001, 1007)])
    sheet(E / "sheet-others-whole.jpg",
          "fn-341 R4: every other preset at 40, whole view, seed 1001. Left: the cycle. Right: P and U only\n"
          "(no checked R and N, so the cycle's girth); P and U are dormant where a preset has no sag or upkeep.",
          [(n, 40, 1001, "whole", "") for n in ("spruce", "birch", "palm", "ordinary")])


if __name__ == "__main__":
    main()
