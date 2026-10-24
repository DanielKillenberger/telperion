"""fn-213 round 11: the silver birch's leaf colour, read by code from the
in-leaf reference photographs (.flow/references/silver-birch, S2 and S5).

Each photograph's leaf pixels are picked by a mask (a region, green above
red and blue, and, on the close-ups, in focus or lit above the black
ground), converted from sRGB to linear and averaged. On the in-focus
spray (S5-hanging) the leaf pixels are split by k-means into three
classes, written as a map so the classes can be checked by eye.

    uv run --with numpy --with pillow leaf_colour.py <out dir>
"""
import sys
from pathlib import Path

import numpy as np
from PIL import Image

REFS = Path(__file__).resolve().parents[2] / "references" / "silver-birch"


def linear(srgb):
    c = srgb / 255.0
    return np.where(c <= 0.04045, c / 12.92, ((c + 0.055) / 1.055) ** 2.4)


def load(name):
    return np.asarray(Image.open(REFS / name).convert("RGB")).astype(np.float64)


def greenish(rgb, margin=4.0):
    r, g, b = rgb[..., 0], rgb[..., 1], rgb[..., 2]
    return (g > r - margin) & (g > b + margin)


def sharp(rgb, radius=28, floor=2.5):
    """In focus: the local mean gradient of luminance above a floor."""
    y = rgb @ np.array([0.299, 0.587, 0.114])
    gy, gx = np.gradient(y)
    mag = np.hypot(gx, gy)
    k = 2 * radius + 1
    pad = np.pad(mag, radius, mode="edge")
    cs = pad.cumsum(0).cumsum(1)
    cs = np.pad(cs, ((1, 0), (1, 0)))
    box = cs[k:, k:] - cs[:-k, k:] - cs[k:, :-k] + cs[:-k, :-k]
    return box / (k * k) > floor


def save_mask(rgb, mask, path):
    img = (rgb * 0.3).astype(np.uint8)
    img[mask] = rgb[mask].astype(np.uint8)
    Image.fromarray(img).resize((rgb.shape[1] // 3, rgb.shape[0] // 3)).save(path)


def region(shape, box):
    m = np.zeros(shape[:2], bool)
    x0, y0, x1, y1 = box
    m[y0:y1, x0:x1] = True
    return m


def report(name, lin, mask):
    px = lin[mask]
    mean = px.mean(0)
    print(
        f"{name}: {mask.sum():>8} px  linear mean {mean[0]:.3f}/{mean[1]:.3f}/{mean[2]:.3f}"
        f"  R:G:B {mean[0] / mean[1]:.2f}:1:{mean[2] / mean[1]:.2f}"
    )
    return px


def kmeans(px, k=3, rounds=30, seed=1):
    rng = np.random.default_rng(seed)
    centres = px[rng.choice(len(px), k, replace=False)]
    for _ in range(rounds):
        label = ((px[:, None, :] - centres[None]) ** 2).sum(-1).argmin(1)
        centres = np.array([px[label == j].mean(0) for j in range(k)])
    return label, centres


def main(out):
    out = Path(out)
    out.mkdir(parents=True, exist_ok=True)
    print("Leaf pixels, sRGB to linear, mean per photograph")
    # S2-Oslo, August: the crown above the lawn, the sky and the stem out.
    rgb = load("S2-oslo-lawn-leaf.jpg")
    crown = region(rgb.shape, (150, 60, 1150, 940)) & greenish(rgb, 6.0)
    oslo = report("S2-oslo crown (Aug)", linear(rgb), crown)
    save_mask(rgb, crown, out / "s2-oslo-mask.png")
    # Its exposure: the sunlit white stem (bright, grey) below the crown,
    # against the preset's own white-bark rows (0.78/0.76/0.70).
    lum = rgb @ np.array([0.299, 0.587, 0.114])
    spread = rgb.max(-1) - rgb.min(-1)
    stem = region(rgb.shape, (470, 1250, 760, 1900)) & (lum > 200) & (spread < 30)
    bark = report("S2-oslo sunlit stem", linear(rgb), stem).mean(0)
    scale = np.array([0.78, 0.76, 0.70]) / bark
    c = oslo.mean(0) * scale
    print(f"S2-oslo crown on the stem's exposure: {c[0]:.3f}/{c[1]:.3f}/{c[2]:.3f}")
    # Sunlit only (host, 2026-10-08: the renderer shades again): leaf
    # pixels unclipped and above the crown's median luminance, where the
    # shadow mode ends; the upper cuts add sheen and blue.
    lin = linear(rgb)
    lit = crown & (rgb.max(-1) < 250)
    y = lin[lit] @ np.array([0.2126, 0.7152, 0.0722])
    for q in (0.5, 0.75, 0.9):
        c = lin[lit][y > np.quantile(y, q)].mean(0) * scale
        print(
            f"S2-oslo sunlit leaves above p{int(q * 100)}, stem's exposure: "
            f"{c[0]:.3f}/{c[1]:.3f}/{c[2]:.3f}  R:G:B {c[0] / c[1]:.2f}:1:{c[2] / c[1]:.2f}"
        )
    # S2-Uusimaa, end of September: yellowing in mist (README caveat).
    rgb = load("S2-uusimaa-field-leaf.jpg")
    report(
        "S2-uusimaa crown (late Sep, yellowing)",
        linear(rgb),
        region(rgb.shape, (180, 130, 1090, 1100)) & greenish(rgb, 4.0),
    )
    # S5-hanging, August: the sharp spray on the right, bokeh out.
    rgb = load("S5-hanging-leafy-shoot.jpg")
    lin = linear(rgb)
    mask = region(rgb.shape, (960, 0, 1920, 1280)) & greenish(rgb, 4.0) & sharp(rgb)
    hanging = report("S5-hanging spray, in focus (Aug)", lin, mask)
    save_mask(rgb, mask, out / "s5-hanging-mask.png")
    # S5-short-shoots, April: young leaves lit on a black ground.
    rgb = load("S5-short-shoots-catkins.jpg")
    lum = rgb @ np.array([0.299, 0.587, 0.114])
    report("S5-short-shoots (Apr, young)", linear(rgb), greenish(rgb, 8.0) & (lum > 40))
    summer = np.concatenate([oslo, hanging])
    m = summer.mean(0)
    print(f"summer (S2-oslo + S5-hanging pixels): linear mean {m[0]:.3f}/{m[1]:.3f}/{m[2]:.3f}")
    per = (oslo.mean(0) + hanging.mean(0)) / 2
    print(f"summer (mean of the two photographs): {per[0]:.3f}/{per[1]:.3f}/{per[2]:.3f}")
    # Three classes on the spray, by linear colour, and their map.
    label, centres = kmeans(hanging)
    order = np.argsort(centres.sum(1))
    print("S5-hanging classes, darkest first:")
    colours = [(40, 40, 200), (200, 40, 40), (240, 220, 40)]
    mapped = np.zeros_like(rgb[..., 0], int) - 1
    idx = np.flatnonzero(mask.ravel())
    for rank, j in enumerate(order):
        c = centres[j]
        share = (label == j).mean()
        print(
            f"  class {rank} ({['blue', 'red', 'yellow'][rank]} on the map): {share:.0%}"
            f"  linear {c[0]:.3f}/{c[1]:.3f}/{c[2]:.3f}  R:G:B {c[0] / c[1]:.2f}:1:{c[2] / c[1]:.2f}"
        )
        mapped.ravel()[idx[label == j]] = rank
    img = (rgb * 0.35).astype(np.uint8)
    for rank, col in enumerate(colours):
        img[mapped == rank] = col
    Image.fromarray(img[:, 960:]).resize((480, 640)).save(out / "s5-hanging-classes.png")




def rendered(paths):
    """The same leaf mask on rendered stills: green above red and blue,
    in the upper two thirds (the sky and ground out by hue)."""
    for path in paths:
        rgb = np.asarray(Image.open(path).convert("RGB")).astype(np.float64)
        mask = greenish(rgb, 6.0) & region(rgb.shape, (0, 0, rgb.shape[1], rgb.shape[0]))
        ground = (np.abs(rgb - np.array([150, 152, 112])).max(-1) < 18)
        report(Path(path).parent.name + "/" + Path(path).name, linear(rgb), mask & ~ground)


if __name__ == "__main__":
    if len(sys.argv) > 2 and sys.argv[1] == "--rendered":
        rendered(sys.argv[2:])
    else:
        main(sys.argv[1] if len(sys.argv) > 1 else ".")
