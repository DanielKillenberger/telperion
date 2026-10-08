"""fn-213 round 13: the in-leaf crown's mean colour at distance, rendered
hero still against S2-Oslo, both sRGB to linear.

The rendered crown: background per row is the median of the still's left
and right margins (sky above the horizon, ground below); a pixel is tree
where it departs from that by more than 24 in any channel. The crown
band runs from the tree's top row down 0.7 of the way to its lowest row,
clear of the ground shadow; the mask is saved for checking by eye.

The photograph: S2-Oslo's crown box (leaf_colour.py's), sky out (blue
above green and red, or cloud white).

Both on their sunlit white stem's exposure: the preset's bark rows
(0.78/0.76/0.70) over the stem's bright grey pixels (S2-Oslo's below its
crown; the still's in the band's lower 0.7 to 1.0 of the tree).

    uv run --with numpy --with pillow crown_colour.py <mask dir> <whole.png>...
    uv run --with numpy --with pillow crown_colour.py --fit <rows a> <rows b> <a.png> <b.png>
"""
import sys
from pathlib import Path

import numpy as np
from PIL import Image

from leaf_colour import REFS, linear, load, region

BARK = np.array([0.78, 0.76, 0.70])
Y = np.array([0.2126, 0.7152, 0.0722])


def stem_scale(rgb, mask):
    lum = rgb @ np.array([0.299, 0.587, 0.114])
    spread = rgb.max(-1) - rgb.min(-1)
    stem = mask & (lum > 200) & (spread < 30)
    if stem.sum() < 20:
        return None, int(stem.sum()), None
    bark = linear(rgb[stem]).mean(0)
    clipped = float((rgb[stem].max(-1) >= 254).mean())
    return BARK / bark, int(stem.sum()), clipped


def line(name, lin, scale, n, gaps=None):
    m = lin.mean(0)
    y = lin @ Y
    q = np.quantile(y, [0.1, 0.5, 0.9])
    s = m * scale if scale is not None else m * np.nan
    extra = f"  gaps {gaps:.0%}" if gaps is not None else ""
    print(
        f"{name}: {n:>7} px  linear {m[0]:.3f}/{m[1]:.3f}/{m[2]:.3f}"
        f"  R:G:B {m[0] / m[1]:.2f}:1:{m[2] / m[1]:.2f}"
        f"  Y p10/50/90 {q[0]:.3f}/{q[1]:.3f}/{q[2]:.3f}"
        f"  on the stem's exposure {s[0]:.3f}/{s[1]:.3f}/{s[2]:.3f}{extra}"
    )
    return s


def gap_share(mask):
    """Background inside the crown's outline: per row, the share of
    pixels between the outermost tree pixels that are not tree."""
    inside = 0
    tree = 0
    for row in mask:
        idx = np.flatnonzero(row)
        if len(idx) > 1:
            inside += idx[-1] - idx[0] + 1
            tree += len(idx)
    return 1 - tree / max(inside, 1)


def still(path, out):
    rgb = np.asarray(Image.open(path).convert("RGB")).astype(np.float64)
    h, w, _ = rgb.shape
    margin = np.concatenate([rgb[:, :80], rgb[:, -80:]], 1)
    bg = np.median(margin, 1)[:, None, :]
    tree = np.abs(rgb - bg).max(-1) > 24
    tree[:, :80] = tree[:, -80:] = False
    rows = np.flatnonzero(tree.any(1))
    top, low = rows[0], rows[-1]
    cut = top + int(0.7 * (low - top))
    band = np.zeros_like(tree)
    band[top:cut] = True
    crown = tree & band
    below = tree & ~band
    scale, n, clipped = stem_scale(rgb, below)
    lin = linear(rgb[crown])
    name = f"{Path(path).parent.name}/{Path(path).name}"
    s = line(name, lin, scale, int(crown.sum()), gap_share(crown))
    if scale is not None:
        print(f"    stem {n} px, {clipped:.0%} clipped, scale {scale[0]:.2f}/{scale[1]:.2f}/{scale[2]:.2f}")
    img = (rgb * 0.25).astype(np.uint8)
    img[crown] = rgb[crown].astype(np.uint8)
    img[cut, :] = (255, 0, 0)
    stem_px = below & ((rgb @ np.array([0.299, 0.587, 0.114])) > 200)
    img[stem_px] = (255, 0, 255)
    Image.fromarray(img).save(Path(out) / f"mask-{Path(path).parent.name}-{Path(path).name}")
    return lin.mean(0), s


def photo(out):
    rgb = load("S2-oslo-lawn-leaf.jpg")
    r, g, b = rgb[..., 0], rgb[..., 1], rgb[..., 2]
    sky = ((b > g + 10) & (b > r + 20)) | (rgb.min(-1) > 225)
    crown = region(rgb.shape, (150, 60, 1150, 940)) & ~sky
    stem = region(rgb.shape, (470, 1250, 760, 1900))
    scale, n, clipped = stem_scale(rgb, stem)
    s = line("S2-oslo crown", linear(rgb[crown]), scale, int(crown.sum()), gap_share(crown))
    print(f"    stem {n} px, {clipped:.0%} clipped, scale {scale[0]:.2f}/{scale[1]:.2f}/{scale[2]:.2f}")
    img = (rgb * 0.25).astype(np.uint8)
    img[crown] = rgb[crown].astype(np.uint8)
    Image.fromarray(img).resize((rgb.shape[1] // 2, rgb.shape[0] // 2)).save(Path(out) / "mask-s2-oslo.png")
    return linear(rgb[crown]).mean(0)


def fit(rows_a, rows_b, a, b, target):
    """Per channel, rendered = gain * row + offset from two renders whose
    leaf rows differ, and the row that would draw the target; raw linear
    means, as the stem is too small to read at 80."""
    ra = np.array([float(x) for x in rows_a.split("/")])
    rb = np.array([float(x) for x in rows_b.split("/")])
    gain = (a - b) / (ra - rb)
    offset = a - gain * ra
    need = (target - offset) / gain
    print(f"  gain {gain[0]:.3f}/{gain[1]:.3f}/{gain[2]:.3f}"
          f"  offset (light from no leaf row) {offset[0]:.3f}/{offset[1]:.3f}/{offset[2]:.3f}"
          f"  row for the target {need[0]:.3f}/{need[1]:.3f}/{need[2]:.3f}")


if __name__ == "__main__":
    args = sys.argv[1:]
    if args[0] == "--fit":
        rows_a, rows_b, out = args[1], args[2], args[3]
        pa, pb = args[4], args[5]
        Path(out).mkdir(parents=True, exist_ok=True)
        target = photo(out)
        ma, _ = still(pa, out)
        mb, _ = still(pb, out)
        fit(rows_a, rows_b, ma, mb, target)
    else:
        out = args[0]
        Path(out).mkdir(parents=True, exist_ok=True)
        photo(out)
        for p in args[1:]:
            still(p, out)
