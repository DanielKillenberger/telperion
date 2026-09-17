# /// script
# dependencies = ["pillow>=10", "numpy>=1.26"]
# ///
import sys, numpy as np
from PIL import Image
S = "/tmp/claude-1000/-home-daniel-Projects-telperion/b8523807-5331-4770-bb90-e9276669f6b1/scratchpad"
still, ref = sys.argv[1:3]
box = {"S-WHOLE": (49, 71, 1285, 1369), "B-WHOLE": (0, 97, 951, 1280)}[ref]
mdir = {"S-WHOLE": "sw", "B-WHOLE": "bw"}[ref]
st = np.asarray(Image.open(still).convert("RGB"), dtype=float).mean(2)
m = np.asarray(Image.open(f"{S}/{mdir}/m1.png").convert("RGB"), dtype=float)
leaf = (m[..., 0] > 250) & (m[..., 1] < 5) & (m[..., 2] > 250)
sx, sy, sw, sh = box
out = []
for i in range(6):
    sel = np.zeros_like(leaf); sel[sy + int(sh * i / 6): sy + int(sh * (i + 1) / 6), sx + int(.2 * sw): sx + int(.8 * sw)] = True; sel &= leaf
    out.append(f"{st[sel].mean():.0f}")
print("rows", " ".join(out))
