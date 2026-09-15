# /// script
# dependencies = ["pillow>=10", "numpy>=1.26"]
# ///
"""Share above half brightness: photograph centre crop, still centre crop, still leaf pixels."""
import json, sys
import numpy as np
from PIL import Image
S = "/tmp/claude-1000/-home-daniel-Projects-telperion/b8523807-5331-4770-bb90-e9276669f6b1/scratchpad"
load = lambda p: np.asarray(Image.open(p).convert("RGB"), dtype=np.float64)
def centre(box):
    x, y, w, h = box
    return (slice(y + int(.3 * h), y + int(.7 * h)), slice(x + int(.3 * w), x + int(.7 * w)))
out = {}
for ref, species, mask, before in [("S-WHOLE", "silver-birch", "sw", "S-WHOLE"), ("B-WHOLE", "european-beech", "bw", "B-WHOLE")]:
    cmp = json.load(open(f".flow/evidence/fn34/measure/pairs-fn52/{species}-1-{ref}-compare.json"))
    rec = next(r for r in json.load(open(f".flow/evidence/fn34/{species}/references.json"))["references"] if r["id"] == ref)
    photo = load(f".refs/fn34/{species}/" + rec["url"].split("/")[-1])
    if rec["shot"].get("crop"):
        h, w = photo.shape[:2]; x, y, cw, ch = rec["shot"]["crop"]
        photo = photo[int(y * h): int((y + ch) * h), int(x * w): int((x + cw) * w)]
    pm = photo.mean(2)
    m = load(f"{S}/{mask}/m1.png")
    leaf = (m[..., 0] > 250) & (m[..., 1] < 5) & (m[..., 2] > 250)
    row = {"photograph_centre": round(float((pm[centre(cmp["photograph"]["box_px"])] > 127.5).mean()), 3)}
    for name, still in [("round8c", f"{S}/base/{before}.png"), ("round10", f".flow/evidence/fn34/measure/{species}-1-{ref}.png")]:
        sm = load(still).mean(2)
        c = centre(cmp["still"]["box_px"])
        cl = np.zeros_like(leaf); cl[c] = leaf[c]
        row[name] = {"centre": round(float((sm[c] > 127.5).mean()), 3), "leaf": round(float((sm[leaf] > 127.5).mean()), 3),
                     "centre_leaf": round(float((sm[cl] > 127.5).mean()), 3), "leaf_mean": round(float(sm[leaf].mean()), 1),
                     "centre_leaf_mean": round(float(sm[cl].mean()), 1)}
    out[ref] = row
print(json.dumps(out, indent=1))
