#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.11"
# dependencies = ["pillow>=10", "numpy>=1.26"]
# ///
"""The pictures a found photograph's shot is chosen over (fn-157, host
2026-09-26). The species runner owns every number; this script measures a
render and draws what it is handed, and decides nothing.

  uv run scripts/shot-frames.py measure --still S --twin T
      the render's tree box [x, y, w, h] and crown base, as fractions, by
      the same mask `compare-references.py` measures a still with
  uv run scripts/shot-frames.py overlay --photo P --boxes JSON --lines JSON \
      --out-boxes A --out-lines B
      the photograph with each labelled box, and with each labelled
      horizontal line, drawn over it
  uv run scripts/shot-frames.py --self-test
"""
from __future__ import annotations

import argparse
import importlib.util
import json
import sys
import tempfile
from pathlib import Path

import numpy as np
from PIL import Image, ImageDraw

HEIGHT = 720
COLOURS = [(230, 25, 75), (60, 180, 75), (0, 130, 200), (245, 130, 48), (145, 30, 180), (240, 50, 230)]


def compare():
    path = Path(__file__).with_name("compare-references.py")
    spec = importlib.util.spec_from_file_location("compare_references", path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def measure(still: Path, twin: Path) -> dict:
    c = compare()
    a, b = c.load(still), c.load(twin)
    mask = c.tree_mask(a, b)
    box = c.box_of(mask)
    if box is None:
        raise SystemExit("the render shows no tree")
    h, w = mask.shape
    x, y, bw, bh = box
    fractions = [round(x / w, 4), round(y / h, 4), round(bw / w, 4), round(bh / h, 4)]
    return {"box": fractions, "crownBase": c.crown_base(mask, box)}


def scaled(photo: Path) -> Image.Image:
    image = Image.open(photo).convert("RGB")
    width = max(1, round(image.width * HEIGHT / image.height))
    return image.resize((width, HEIGHT), Image.LANCZOS)


def overlay(photo: Path, boxes: list, lines: list, out_boxes: Path, out_lines: Path) -> None:
    base = scaled(photo)
    w, h = base.size
    framed = base.copy()
    draw = ImageDraw.Draw(framed)
    for i, (label, (x, y, bw, bh)) in enumerate(boxes):
        colour = COLOURS[i % len(COLOURS)]
        draw.rectangle([x * w, y * h, (x + bw) * w - 1, (y + bh) * h - 1], outline=colour, width=3)
        draw.text((x * w + 6, y * h + 4 + 14 * i), label, fill=colour)
    framed.save(out_boxes)
    ruled = base.copy()
    draw = ImageDraw.Draw(ruled)
    for i, (label, y) in enumerate(lines):
        colour = COLOURS[i % len(COLOURS)]
        draw.line([(0, y * h), (w, y * h)], fill=colour, width=3)
        draw.text((6 + 18 * i, y * h - 14), label, fill=colour)
    ruled.save(out_lines)


def self_test() -> int:
    with tempfile.TemporaryDirectory() as tmp:
        tmp = Path(tmp)
        still = np.full((200, 100, 3), 200, np.uint8)
        still[40:180, 30:70] = 20
        twin = still.copy()
        Image.fromarray(still).save(tmp / "s.png")
        Image.fromarray(twin).save(tmp / "t.png")
        got = measure(tmp / "s.png", tmp / "t.png")
        assert got["box"] == [0.3, 0.2, 0.4, 0.7], got
        overlay(tmp / "s.png", [["A", [0.3, 0.2, 0.4, 0.7]]], [["1", 0.8]], tmp / "b.png", tmp / "l.png")
        assert Image.open(tmp / "b.png").size == (360, HEIGHT)
        assert np.asarray(Image.open(tmp / "l.png"))[576].tolist() != np.asarray(Image.open(tmp / "b.png"))[576].tolist()
    print("self-test ok")
    return 0


def main() -> int:
    if sys.argv[1:] == ["--self-test"]:
        return self_test()
    parser = argparse.ArgumentParser()
    parser.add_argument("command", choices=["measure", "overlay"])
    parser.add_argument("--still")
    parser.add_argument("--twin")
    parser.add_argument("--photo")
    parser.add_argument("--boxes")
    parser.add_argument("--lines")
    parser.add_argument("--out-boxes")
    parser.add_argument("--out-lines")
    args = parser.parse_args()
    if args.command == "measure":
        print(json.dumps(measure(Path(args.still), Path(args.twin))))
    else:
        overlay(Path(args.photo), json.loads(args.boxes), json.loads(args.lines),
                Path(args.out_boxes), Path(args.out_lines))
    return 0


if __name__ == "__main__":
    sys.exit(main())
