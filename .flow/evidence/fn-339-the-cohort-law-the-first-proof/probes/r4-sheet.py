#!/usr/bin/env python3
"""fn-339 R4: one retained sheet per species from r4-stills.sh's stills,
the cycle on the left and the coupled probe on the right, a row per age and
seed. Run from the repository root; needs ImageMagick 7 (`magick`).
Writes <evidence>/sheet-r4-<species>.jpg.
"""
import subprocess
from pathlib import Path

E = Path(".flow/evidence/fn-339-the-cohort-law-the-first-proof")
RAW = E / "raw" / "stills"
NOTE = ("Left: the cycle. Right: the cohort probe, its wood and the structure's own leaves;\n"
        "the cohorts' leaf is counted, not drawn: drawing it needs twig axes built from them (the third proof's).")
SHEETS = {
    "oak": ("Oregon white oak", [40, 80, 120]),
    "spruce": ("Norway spruce", [40]),
    "beech": ("European beech", [40]),
}


def main():
    for name, (title, ages) in SHEETS.items():
        tiles = []
        for age in ages:
            for seed in (1001, 1007):
                for mode in ("base", "probe"):
                    path = RAW / mode / f"{name}-{age}-{seed}-whole.png"
                    label = f"{'cycle' if mode == 'base' else 'cohort probe'}, age {age}, seed {seed}"
                    tiles += ["-label", label, str(path)]
        out = E / f"sheet-r4-{name}.jpg"
        subprocess.run(["magick", "montage", *tiles, "-tile", "2x", "-geometry", "640x480+6+6",
                        "-pointsize", "18", str(out)], check=True)
        # The title over the tiles, in a band of its own.
        subprocess.run(["magick", str(out), "-background", "white", "-gravity", "north",
                        "-splice", "0x120", "-pointsize", "26",
                        "-annotate", "+0+12", f"fn-339 R4: {title}, whole view.\n{NOTE}",
                        "-quality", "85", str(out)], check=True)
        print(out)


if __name__ == "__main__":
    main()
