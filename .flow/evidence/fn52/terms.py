# /// script
# dependencies = ["pillow>=10", "numpy>=1.26"]
# ///
"""Per-term leaf-pixel statistics from the diagnostic renders of one shot."""
import json, sys
from pathlib import Path
import numpy as np
from PIL import Image

def srgb(path):
    return np.asarray(Image.open(path).convert("RGB"), dtype=np.float64) / 255.0

def linear(c):
    return np.where(c <= 0.04045, c / 12.92, ((c + 0.055) / 1.055) ** 2.4)

def lum(rgb):
    return rgb @ np.array([0.2126, 0.7152, 0.0722])

def centre(box):
    x, y, w, h = box
    return (slice(y + int(0.3 * h), y + int(0.7 * h)), slice(x + int(0.3 * w), x + int(0.7 * w)))

def run(d, photo=None, photo_box=None, crop=None):
    d = Path(d)
    mask = (srgb(d / "m1.png") * 255).round()
    leaf = (mask[..., 0] > 250) & (mask[..., 1] < 5) & (mask[..., 2] > 250)
    wood = (mask[..., 0] < 5) & (mask[..., 1] > 250) & (mask[..., 2] > 250)
    tree = leaf | wood
    rows, cols = np.flatnonzero(tree.any(1)), np.flatnonzero(tree.any(0))
    box = (cols[0], rows[0], cols[-1] - cols[0] + 1, rows[-1] - rows[0] + 1)
    c = centre(box)
    final = srgb(d / "m0.png")
    out = {"box": [int(v) for v in box], "leaf_px": int(leaf.sum()), "wood_px": int(wood.sum()),
           "centre_leaf_share": round(float(leaf[c].mean()), 3), "centre_wood_share": round(float(wood[c].mean()), 3)}
    m = final.mean(2) * 255
    out["centre_mean"] = round(float(m[c].mean()), 1)
    out["centre_above_half"] = round(float((m[c] > 127.5).mean()), 3)
    out["leaf_mean"] = round(float(m[leaf].mean()), 1)
    out["leaf_above_half"] = round(float((m[leaf] > 127.5).mean()), 3)
    cl = leaf.copy(); cl[:] = False; cl[c] = leaf[c]
    out["centre_leaf_mean"] = round(float(m[cl].mean()), 1)
    out["centre_leaf_above_half"] = round(float((m[cl] > 127.5).mean()), 3)
    out["wood_mean"] = round(float(m[wood].mean()), 1)
    top = np.zeros_like(tree); top[: final.shape[0] // 10] = True
    sky = top & ~tree
    out["sky_mean"] = round(float(m[sky].mean()), 1) if sky.any() else None
    names = {2: "albedo", 3: "sun_unshadowed", 4: "sun_shadowed", 5: "ambient_open", 6: "ambient_occluded",
             7: "transmission", 8: "specular", 14: "total_linear", 17: "transmission_diffuse"}
    terms = {}
    for k, name in names.items():
        v = linear(srgb(d / f"m{k}.png"))
        scale = 1.0 if k == 2 else 4.0
        g = v[..., 1] * scale
        terms[name] = {"leaf_green": round(float(g[leaf].mean()), 4), "centre_leaf_green": round(float(g[cl].mean()), 4)}
    out["linear_terms_green"] = terms
    raw = {9: "visibility", 10: "n_dot_sun", 11: "front", 12: "depth", 13: "n_dot_eye", 15: "n_dot_geometric", 16: "geometric_dot_sun", 18: "visibility"}
    stats = {}
    for k, name in raw.items():
        if k == 18: continue
        v = linear(srgb(d / f"m{k}.png"))[..., 1]
        if k in (13, 15, 16):
            v = 2 * v - 1
        stats[name] = {"mean": round(float(v[leaf].mean()), 3), "share_positive": round(float((v[leaf] > (0.02 if k not in (9, 12) else 0.5)).mean()), 3)}
    out["geometry"] = stats
    nl = linear(srgb(d / "m10.png"))[..., 1]
    vis = linear(srgb(d / "m9.png"))[..., 1]
    facing_sun = leaf & (nl > 0.02)
    out["facing_sun_share"] = round(float(facing_sun.mean() / leaf.mean()), 3)
    out["facing_sun_visibility"] = round(float(vis[facing_sun].mean()), 3)
    out["facing_sun_final_mean"] = round(float(m[facing_sun].mean()), 1)
    out["lit_share"] = round(float((leaf & (nl * vis > 0.05)).sum() / leaf.sum()), 3)
    if photo:
        p = srgb(photo)
        if crop:
            h, w = p.shape[:2]; x, y, cw, ch = crop
            p = p[int(y * h): int((y + ch) * h), int(x * w): int((x + cw) * w)]
        h, w = p.shape[:2]; bx, by, bw, bh = photo_box
        pb = (int(bx * w), int(by * h), max(1, int(bw * w)), max(1, int(bh * h)))
        pm = p.mean(2) * 255
        pc = centre(pb)
        out["photo_centre_mean"] = round(float(pm[pc].mean()), 1)
        out["photo_centre_above_half"] = round(float((pm[pc] > 127.5).mean()), 3)
        out["photo_top_sky"] = round(float(pm[:h // 20, :w // 10].mean()), 1)
    return out

if __name__ == "__main__":
    d, species, ref = sys.argv[1:4]
    refs = json.load(open(f".flow/evidence/fn34/{species}/references.json"))["references"]
    r = next(x for x in refs if x["id"] == ref)
    name = Path(r["url"]).name
    print(json.dumps(run(d, f".refs/fn34/{species}/{name}", r["shot"]["tree"]["box"], r["shot"].get("crop")), indent=1))
