#!/usr/bin/env python3
"""fn-336: compose the retained sheets from the stills `stills.sh` wrote.

One JPEG per species and view (whole, mid, close), the species' reference
photographs for that view on its first row, or a line saying the repository
holds none; and one sheet of the old oak with fn-332's trunk probe under each
tree. Run from the repository root after `stills.sh` and the trunk probes:

    python3 .flow/evidence/fn-336-sheets-of-020s-trees-as-they-stand/probes/sheets.py

Needs ImageMagick 7 (`magick`). Writes `<evidence>/sheet-*.jpg`.
"""
import re
import subprocess
import sys
from pathlib import Path

E = Path(".flow/evidence/fn-336-sheets-of-020s-trees-as-they-stand")
RAW, REFS = E / "raw", Path(".flow/references")
SEEDS = (1, 7, 42)
TILE = 360  # tile height in pixels; the stills are 960x720
# The shots the still example names (space/still.rs), per view: the mid view
# is the upper crown in leaf, the close view the leafy limb from 9 m and the
# trunk base in leaf from 6 m.
VIEWS = {
    "whole": [("whole", "whole tree, hero pose")],
    "mid": [("crown", "upper crown, in leaf")],
    "close": [("spray", "a limb from 9 m, in leaf"), ("low", "the trunk base from 6 m, in leaf")],
}
# Reference photographs per view, by the five scales of the reference sets
# (S1 and S2 whole, S3 mid-crown limbs, S4 and S5 twig and leafy spray).
SPECIES = {
    "oak": ("Oregon white oak (oregon-white-oak)", "oak-quercus-robur",
            {"whole": "S1-*,S2-*", "mid": "S3-*", "close": "S4-*,S5-*"}),
    "spruce": ("Norway spruce (norway-spruce)", "norway-spruce",
               {"whole": "S1-*,S2-*", "mid": "S3-*", "close": "S4-*,S5-*"}),
    "beech": ("European beech (european-beech)", "european-beech",
              {"whole": "S1-*,S2-*", "mid": "S3-*", "close": "S4-*,S5-*"}),
    "birch": ("Silver birch (silver-birch)", "silver-birch",
              {"whole": "S1-*,S2-*", "mid": "S3-*", "close": "S4-*,S5-*"}),
    "palm": ("Date palm (date-palm)", "date-palm",
             {"whole": "P-WHOLE-*", "mid": "", "close": "P-TRUNK-*,P-BASE-*"}),
    "ordinary": ("The ordinary tree (ordinary)", None, {}),
}


def magick(*args):
    subprocess.run(["magick", *map(str, args)], check=True)


def tile(path, label, out):
    """`path` scaled to the tile height with `label` beneath it."""
    magick(path, "-resize", f"x{TILE}", "-background", "white", "-fill", "black",
           "-font", "Adwaita-Sans", "-pointsize", "15", f"label:{label}",
           "-gravity", "center", "-append", "-bordercolor", "white", "-border", "6", out)
    return out


def text(line, out, size=20, width=1500):
    magick("-background", "white", "-fill", "black", "-font", "Adwaita-Sans",
           "-pointsize", size, "-size", f"{width}x", f"caption:{line}", out)
    return out


def row(tiles, out):
    magick(*tiles, "-background", "white", "-gravity", "north", "+append", out)
    return out


def sheet(parts, out):
    magick(*parts, "-background", "white", "-gravity", "west", "-append",
           "-bordercolor", "white", "-border", "10",
           "-resize", "2000x>", "-strip", "-quality", "82", out)
    print(out, f"{out.stat().st_size // 1024} KiB")


def references(folder, patterns):
    found = []
    for pattern in filter(None, patterns.split(",")):
        found += sorted((REFS / folder).glob(pattern + ".jpg"))
    return found


def species_sheets(work):
    for name, (title, folder, views) in SPECIES.items():
        for view, shots in VIEWS.items():
            parts = [text(f"{title}, age 40, full cycle (fidelity 0): {view} view", work / "title.png", 26)]
            refs = references(folder, views.get(view, "")) if folder else []
            if refs:
                parts.append(text(f"Reference photographs ({REFS / folder}):", work / "rl.png", 18))
                parts.append(row([tile(p, p.stem, work / f"r{i}.png") for i, p in enumerate(refs)], work / "refs.png"))
            elif folder:
                parts.append(text(f"The repository holds no {view}-view reference photograph for this species ({REFS / folder} has none at this scale).", work / "rl.png", 18))
            else:
                parts.append(text("The repository holds no reference photographs for this tree.", work / "rl.png", 18))
            for k, (shot, caption) in enumerate(shots):
                parts.append(text(f"Renders: {caption}, seeds 1, 7, 42", work / f"sl{k}.png", 18))
                tiles = []
                for seed in SEEDS:
                    still = RAW / name / f"{name}-40-{seed}-{shot}.png"
                    if not still.exists():
                        sys.exit(f"missing {still}")
                    tiles.append(tile(still, f"seed {seed}", work / f"t{k}{seed}.png"))
                parts.append(row(tiles, work / f"row{k}.png"))
            sheet(parts, E / f"sheet-{name}-{view}.jpg")


def probe(age, seed):
    log = (RAW / "trunk" / f"oak-{age}-{seed}.log").read_text()
    top = re.search(r"top ([\d.]+) m; crown mean ([\d.]+) m out from the base; trunk path ends ([\d.]+) high, ([\d.]+) out", log)
    if not top:
        sys.exit(f"no probe line for oak {age} {seed}")
    return top.groups()


def old_oak(work):
    parts = [text("Oregon white oak, full cycle (fidelity 0), whole tree at ages 120, 200 and 300, seeds 1, 7, 42", work / "title.png", 26),
             text("Under each tree, from fn-332's trunk probe (crates/telperion-core/examples/trunk.rs): height of the highest live tip; where the trunk path (the seed axis carried on by its thickest continuation or relay) ends, its height and its distance out from the base; and how far the mean of all live tips sits off the base.", work / "probe.png", 16),
             text("Known before this sheet (fn-332): the trunk bends, mostly sag with no reaction wood, and one relay chain carries the crown whatever bends it, so the crown stands off the base. No reference photographs: the reference sets show no oak of known age.", work / "known.png", 16)]
    for age in (120, 200, 300):
        tiles = []
        for seed in SEEDS:
            height, crown, trunk_h, trunk_out = probe(age, seed)
            # The hero pose clipped two bases; `stills.sh` remade them pulled back.
            remade = RAW / "remake" / f"oak-{age}-{seed}-whole.png"
            still = remade if remade.exists() else RAW / "oak-old" / f"oak-{age}-{seed}-whole.png"
            if not still.exists():
                sys.exit(f"missing {still}")
            camera = " (camera pulled back)" if remade.exists() else ""
            label = f"age {age}, seed {seed}{camera}\nheight {height} m; trunk ends {trunk_h} m high, {trunk_out} m out\ncrown offset {crown} m"
            tiles.append(tile(still, label, work / f"o{seed}.png"))
        parts.append(row(tiles, work / f"orow{age}.png"))
    sheet(parts, E / "sheet-oak-old-whole.jpg")


if __name__ == "__main__":
    work = RAW / "sheet-work"
    work.mkdir(parents=True, exist_ok=True)
    which = sys.argv[1:] or ["species", "old"]
    if "species" in which:
        species_sheets(work)
    if "old" in which:
        old_oak(work)
