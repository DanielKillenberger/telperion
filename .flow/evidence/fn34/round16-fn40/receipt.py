#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.11"
# ///
"""fn-32's measurement receipt on fn-34's two bark close-ups, B-BASE and S-BARK.

For each pair: the centre 400x400 crop's mean RGB and channel order (fn-29's
ImageMagick command, unchanged) and fn-32's six-component structure vector
with the distance to the photograph (`bark_score`). Two framings:

- `native`: the still at its own 1440-px rig size and the photograph at its
  own size, as fn-32 measured. A photograph narrower than 400 px (the B-BASE
  half, 267 px) is enlarged to 400 wide first, because the crop needs it.
- `matched`: both resampled to one frame height of 600 px, so the 400 px crop
  covers the same stretch of bark in the still and in the photograph the shot
  block imitates. This is the fair structure comparison.

S-BARK's photograph is a diptych, so it is also measured half by half.
The photograph is converted and resampled into a scratch directory and never
written anywhere else. Run from the repository root after the capture:

  uv run .flow/evidence/fn34/round16-fn40/receipt.py <captures> <scratch>
"""
from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path

SCORE = "target/release/examples/bark_score"
BEECH = ".refs/fn34/european-beech/fasy896.jpg"
BIRCH = ".refs/fn34/silver-birch/bepe3243.jpg"
PAIRS = [
    # id, still, photograph, crop of the photograph (x, y, w, h fractions)
    ("B-BASE", "european-beech-1-B-BASE.png", BEECH, (0.5, 0.0, 0.5, 1.0)),
    ("S-BARK", "silver-birch-1-S-BARK.png", BIRCH, None),
    ("S-BARK-left", "silver-birch-1-S-BARK.png", BIRCH, (0.0, 0.0, 0.5, 1.0)),
    ("S-BARK-right", "silver-birch-1-S-BARK.png", BIRCH, (0.5, 0.0, 0.5, 1.0)),
]
COMPONENTS = ["orientation_entropy", "area_variation", "furrow_curvature",
              "junction_arms", "dark_fraction", "furrow_period"]


def magick(*args: str) -> str:
    return subprocess.run(["magick", *args], check=True, capture_output=True,
                          text=True).stdout


def size(path: Path) -> tuple[int, int]:
    w, h = magick("identify", "-format", "%w %h", str(path)).split()
    return int(w), int(h)


def prepared(source: Path, crop, height: int | None, out: Path) -> Path:
    args = [str(source)]
    if crop is not None:
        w, h = size(source)
        x, y, cw, ch = crop
        args += ["-crop", f"{round(cw * w)}x{round(ch * h)}+{round(x * w)}+{round(y * h)}",
                 "+repage"]
    if height is not None:
        args += ["-resize", f"x{height}"]
    magick(*args, str(out))
    if size(out)[0] < 400:
        magick(str(out), "-resize", "400x", str(out))
    return out


def crop_rgb(path: Path) -> list[int]:
    text = magick(str(path), "-gravity", "center", "-crop", "400x400+0+0", "+repage",
                  "-resize", "1x1!", "-format",
                  "%[fx:int(255*r)] %[fx:int(255*g)] %[fx:int(255*b)]", "info:")
    return [int(v) for v in text.split()]


def order(rgb: list[int]) -> str:
    return "".join(c for _, c in sorted(zip(rgb, "RGB"), key=lambda p: -p[0]))


def score(still: Path, photo: Path) -> dict:
    out = json.loads(subprocess.run([SCORE, str(still), str(photo)], check=True,
                                    capture_output=True, text=True).stdout)
    pick = lambda v: [v[c] for c in COMPONENTS]
    return {"still": pick(out["vector"]),
            "photograph": pick(out["references"][0]["vector"]),
            "distance": out["score"]}


def main() -> int:
    captures, scratch = Path(sys.argv[1]), Path(sys.argv[2])
    scratch.mkdir(parents=True, exist_ok=True)
    result = {"components": COMPONENTS}
    for name, still_name, photo_name, crop in PAIRS:
        row = {}
        for framing, height in (("native", None), ("matched", 600)):
            s = prepared(captures / still_name, None, height,
                         scratch / f"{name}-{framing}-still.png")
            p = prepared(Path(photo_name), crop, height,
                         scratch / f"{name}-{framing}-photo.png")
            s_rgb, p_rgb = crop_rgb(s), crop_rgb(p)
            row[framing] = {"still_rgb": s_rgb, "still_order": order(s_rgb),
                            "photograph_rgb": p_rgb, "photograph_order": order(p_rgb),
                            **score(s, p)}
        result[name] = row
    print(json.dumps(result, indent=1))
    return 0


if __name__ == "__main__":
    sys.exit(main())
