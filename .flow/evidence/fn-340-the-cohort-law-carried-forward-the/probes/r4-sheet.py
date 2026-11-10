#!/usr/bin/env python3
"""fn-340 R4: the oak's sheet from r4-runs.sh's stills, the cycle on the left
and the corrected, carried probe on the right, a row per age and seed (fn-339's
probes/r4-sheet.py, the oak alone). Run from the repository root; needs
ImageMagick 7 (`magick`). Writes <evidence>/sheet-r4-oak.jpg.
"""
import subprocess
from pathlib import Path

E = Path(".flow/evidence/fn-340-the-cohort-law-carried-forward-the")
RAW = E / "raw" / "stills"
NOTE = ("Left: the cycle. Right: the cohort probe, its wood and the structure's own leaves;\n"
        "the cohorts' leaf is counted, not drawn: drawing it needs twig axes built from them (the third proof's).")


def main():
    tiles = []
    for age in (40, 80, 120):
        for seed in (1001, 1007):
            for mode in ("base", "probe"):
                path = RAW / mode / f"oak-{age}-{seed}-whole.png"
                label = f"{'cycle' if mode == 'base' else 'cohort probe'}, age {age}, seed {seed}"
                tiles += ["-label", label, str(path)]
    out = E / "sheet-r4-oak.jpg"
    subprocess.run(["magick", "montage", *tiles, "-tile", "2x", "-geometry", "640x480+6+6",
                    "-pointsize", "18", str(out)], check=True)
    subprocess.run(["magick", str(out), "-background", "white", "-gravity", "north",
                    "-splice", "0x120", "-pointsize", "26",
                    "-annotate", "+0+12", f"fn-340 R4: Oregon white oak, whole view.\n{NOTE}",
                    "-quality", "85", str(out)], check=True)
    print(out)


if __name__ == "__main__":
    main()
