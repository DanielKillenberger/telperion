#!/usr/bin/env -S uv run --script
# /// script
# requires-python = ">=3.11"
# dependencies = ["pillow>=10", "numpy>=1.26"]
# ///
"""Pairs a matched still with the photograph it imitates and measures both.

For every reference record that carries a shot block, the species runner has
rendered `<case>-<id>.png` and a twin `<case>-<id>-twin.png` under the same
camera with the sun on the horizon behind the tree, so the tree stands in
both and its shadow lies in the frame of only one. This script writes `<case>-<id>-pair.png`,
the photograph and the still at one height, and `<case>-<id>-compare.json`,
the same numbers read off both images: the crown's width over its height,
its base as a fraction of its height, how much of the frame it fills, the
fraction of the tree box that is tree, and the centre-crop colour. The
photograph's tree box and crown base are the record's own, read by eye and
written down; the still's come from the two renders. Nothing here awards a
verdict.

  uv run scripts/compare-references.py --references FILE --captures DIR \
      --refs DIR --case ID --out DIR
  uv run scripts/compare-references.py --self-test
"""
from __future__ import annotations

import argparse
import hashlib
import json
import sys
from pathlib import Path

import numpy as np
from PIL import Image, ImageDraw

HEIGHT = 720
RULE = 12
CAPTION = 28
DARK = 0.35
EDGE = 0.06


def load(path: Path) -> np.ndarray:
    """An image as float RGB in 0..1, alpha dropped."""
    return np.asarray(Image.open(path).convert("RGB"), dtype=np.float32) / 255.0


def foreground(still: np.ndarray) -> np.ndarray:
    """Pixels that are not the sky or the ground of their own row. The room is
    uniform along a row, so the median of each row's outer columns is its
    background; a still is one tree, and the tree never reaches both edges."""
    edge = max(2, still.shape[1] // 50)
    outer = np.concatenate([still[:, :edge], still[:, -edge:]], axis=1)
    median = np.median(outer, axis=1, keepdims=True)
    return np.abs(still - median).max(axis=2) > EDGE


def tree_mask(still: np.ndarray, twin: np.ndarray) -> np.ndarray:
    """The tree and not its shadow: what stands out of the background under
    both suns. The shadow moves with the sun; the tree does not."""
    return foreground(still) & foreground(twin)


def box_of(mask: np.ndarray) -> tuple[int, int, int, int] | None:
    rows = np.flatnonzero(mask.any(axis=1))
    columns = np.flatnonzero(mask.any(axis=0))
    if rows.size == 0 or columns.size == 0:
        return None
    return int(columns[0]), int(rows[0]), int(columns[-1] - columns[0] + 1), int(rows[-1] - rows[0] + 1)


def crown_base(mask: np.ndarray, box: tuple[int, int, int, int]) -> float:
    """The lowest row whose tree run is at least three tenths of the widest
    row, as a fraction of the tree's height above its base. Below it is trunk."""
    x, y, w, h = box
    widths = mask[y : y + h, x : x + w].sum(axis=1)
    wide = np.flatnonzero(widths >= 0.3 * widths.max())
    lowest = int(wide[-1]) if wide.size else h - 1
    return round((h - 1 - lowest) / max(h - 1, 1), 4)


def centre_colour(image: np.ndarray, box: tuple[int, int, int, int]) -> dict:
    """The fn-29 measurement on the tree's own middle: the mean of the centre
    two fifths of the box, in 0..255, and which channel leads."""
    x, y, w, h = box
    inner = image[y + int(0.3 * h) : y + int(0.7 * h), x + int(0.3 * w) : x + int(0.7 * w)]
    rgb = [round(float(v) * 255, 1) for v in inner.reshape(-1, 3).mean(axis=0)]
    order = "".join(sorted("RGB", key=lambda c: -rgb["RGB".index(c)]))
    return {"rgb": rgb, "order": order, "mean": round(sum(rgb) / 3, 1)}


def measure(image: np.ndarray, box: tuple[int, int, int, int], mask: np.ndarray | None, base: float | None) -> dict:
    x, y, w, h = box
    inside = image[y : y + h, x : x + w]
    luminance = inside @ np.array([0.2126, 0.7152, 0.0722], dtype=np.float32)
    if mask is not None:
        occupied = float(mask[y : y + h, x : x + w].mean())
    else:
        occupied = float((luminance < DARK).mean())
    return {
        "box_px": [x, y, w, h],
        "width_over_height": round(w / max(h, 1), 4),
        "fill": round(h / image.shape[0], 4),
        "crown_base": base,
        "occupied": round(occupied, 4),
        "centre": centre_colour(image, box),
    }


def crop(photo: np.ndarray, fractions: list[float] | None) -> np.ndarray:
    if not fractions:
        return photo
    h, w = photo.shape[:2]
    x, y, cw, ch = fractions
    return photo[int(y * h) : int((y + ch) * h), int(x * w) : int((x + cw) * w)]


def scaled(image: np.ndarray, height: int) -> Image.Image:
    pil = Image.fromarray((image * 255).astype(np.uint8))
    width = max(1, round(pil.width * height / pil.height))
    return pil.resize((width, height), Image.LANCZOS)


def pair(photo: np.ndarray, still: np.ndarray, caption: str, out: Path) -> None:
    left, right = scaled(photo, HEIGHT), scaled(still, HEIGHT)
    canvas = Image.new("RGB", (left.width + RULE + right.width, HEIGHT + CAPTION), (238, 240, 234))
    canvas.paste(left, (0, 0))
    canvas.paste(right, (left.width + RULE, 0))
    ImageDraw.Draw(canvas).text((8, HEIGHT + 7), caption, fill=(31, 38, 32))
    canvas.save(out)


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def compare(record: dict, photo: np.ndarray, still: np.ndarray, twin: np.ndarray) -> dict:
    shot = record["shot"]
    cut = crop(photo, shot.get("crop"))
    ph, pw = cut.shape[:2]
    bx, by, bw, bh = shot["tree"]["box"]
    photo_box = (int(bx * pw), int(by * ph), max(1, int(bw * pw)), max(1, int(bh * ph)))
    mask = tree_mask(still, twin)
    still_box = box_of(mask)
    if still_box is None:
        raise SystemExit(f"{record['id']}: the still shows no tree")
    return {
        "reference": record["id"],
        "foliage": shot["foliage"],
        "photograph": measure(cut, photo_box, None, shot["tree"]["crownBase"]),
        "still": measure(still, still_box, mask, crown_base(mask, still_box)),
        "method": {
            "photograph": "tree box and crown base from the record, read by eye; occupied is the dark-pixel fraction of the box",
            "still": "tree mask is what stands out of the row background under both suns; occupied is the mask's share of its box",
        },
    }


def run(args: argparse.Namespace) -> int:
    references = json.loads(Path(args.references).read_text())
    out = Path(args.out)
    out.mkdir(parents=True, exist_ok=True)
    written = []
    for record in references["references"]:
        shot = record.get("shot")
        if not shot:
            continue
        photo_path = Path(args.refs) / record["url"].rsplit("/", 1)[-1]
        if not photo_path.exists():
            raise SystemExit(f"{record['id']}: photograph missing at {photo_path}")
        if record.get("asset_sha256") and sha256(photo_path) != record["asset_sha256"]:
            raise SystemExit(f"{record['id']}: photograph bytes do not match asset_sha256")
        stem = Path(args.captures) / f"{args.case}-{record['id']}"
        still_path, twin_path = stem.with_suffix(".png"), Path(f"{stem}-twin.png")
        for path in (still_path, twin_path):
            if not path.exists():
                raise SystemExit(f"{record['id']}: still missing at {path}")
        photo, still, twin = load(photo_path), load(still_path), load(twin_path)
        result = compare(record, photo, still, twin)
        pair_path = out / f"{args.case}-{record['id']}-pair.png"
        pair(crop(photo, shot.get("crop")), still, f"{record['id']}  |  {args.case}  |  {shot['foliage']}", pair_path)
        result["pair"] = {"path": str(pair_path), "sha256": sha256(pair_path)}
        result["still_sha256"] = sha256(still_path)
        (out / f"{args.case}-{record['id']}-compare.json").write_text(json.dumps(result, indent=2) + "\n")
        written.append(result)
        print(record["id"], json.dumps({k: result[k] for k in ("photograph", "still")}))
    if not written:
        raise SystemExit("no reference record carries a shot block")
    return 0


def synthetic(shadow_side: int) -> np.ndarray:
    """A still-shaped image: a sky gradient over a flat ground, a dark crown
    ellipse on a trunk, and a shadow ellipse to one side of the base."""
    h, w = 300, 240
    image = np.zeros((h, w, 3), dtype=np.float32)
    horizon = 200
    for row in range(h):
        image[row] = [0.55, 0.66, 0.80] if row < horizon else [0.35, 0.40, 0.25]
        if row < horizon:
            image[row] *= 1 - 0.3 * (1 - row / horizon)
    pil = Image.fromarray((image * 255).astype(np.uint8))
    draw = ImageDraw.Draw(pil)
    draw.ellipse([130 + shadow_side * 30, 232, 190 + shadow_side * 30, 268], fill=(40, 45, 30))
    draw.rectangle([116, 160, 124, 260], fill=(60, 50, 40))
    draw.ellipse([60, 40, 180, 200], fill=(30, 80, 30))
    return np.asarray(pil, dtype=np.float32) / 255.0


def self_test() -> int:
    still, twin = synthetic(1), synthetic(-1)
    mask = tree_mask(still, twin)
    box = box_of(mask)
    assert box is not None
    x, y, w, h = box
    assert abs(w - 121) <= 2 and abs(h - 221) <= 2, f"the mask box is {box}"
    assert abs(x - 60) <= 1 and abs(y - 40) <= 1, f"the shadow leaked into the box: {box}"
    stats = measure(still, box, mask, crown_base(mask, box))
    assert abs(stats["width_over_height"] - 121 / 221) < 0.02, stats
    assert 0.25 < stats["crown_base"] < 0.32, stats
    assert stats["centre"]["order"][0] == "G", stats
    photo = np.full((300, 240, 3), 0.7, dtype=np.float32)
    record = {"id": "T", "shot": {"foliage": "leaf-on", "tree": {"box": [0.25, 0.13, 0.5, 0.74], "crownBase": 0.28}}}
    result = compare(record, photo, still, twin)
    assert abs(result["photograph"]["width_over_height"] - 120 / 222) < 0.02, result
    print("self-test ok", json.dumps(result["still"]))
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--references")
    parser.add_argument("--captures")
    parser.add_argument("--refs")
    parser.add_argument("--case")
    parser.add_argument("--out")
    parser.add_argument("--self-test", action="store_true")
    args = parser.parse_args()
    if args.self_test:
        return self_test()
    if not all([args.references, args.captures, args.refs, args.case, args.out]):
        parser.error("--references, --captures, --refs, --case and --out are required")
    return run(args)


if __name__ == "__main__":
    sys.exit(main())
