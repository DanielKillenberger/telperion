"""fn-228 R3: the birch stem's rendered colour on a base still, beside the
leaves fn-213's leaf_colour.py reads. The stem is the run of columns about
the frame's middle (where the base shot centres it) that is not green over
nine tenths of the rows below the horizon, and within it every pixel that
is not green: the ground, lit or shaded, is green above red and blue, and the bark
(white, grey or black) is not. The hanging strands are thin and wander, so
no column of theirs stays off the ground that long. Above the horizon the
sky is as blue as the shaded bark, so that part of the stem is left out.

    uv run --with numpy --with pillow stem_colour.py <base still>...
"""
import sys

import numpy as np
from PIL import Image


def linear(srgb):
    c = srgb / 255.0
    return np.where(c <= 0.04045, c / 12.92, ((c + 0.055) / 1.055) ** 2.4)


for path in sys.argv[1:]:
    rgb = np.asarray(Image.open(path).convert("RGB")).astype(np.float64)
    green = rgb[..., 1] > np.maximum(rgb[..., 0], rgb[..., 2]) + 2.0
    # The horizon: the first row whose left-hand pixel is ground.
    horizon = int(np.argmax(green[:, 0]))
    below = ~green[horizon + 4 :]
    solid = below[: below.shape[0] // 2].mean(0) > 0.9
    middle = rgb.shape[1] // 2
    if not solid[middle]:
        print(f"{path}: no stem at the frame's middle")
        continue
    left = middle - int(np.argmin(solid[middle::-1]))
    right = middle + int(np.argmin(solid[middle:]))
    mask = ~green
    mask[: horizon + 4] = False
    mask[:, :left + 1] = False
    mask[:, right:] = False
    px = linear(rgb)[mask]
    m = px.mean(0)
    print(
        f"{path}: {mask.sum():>7} px, columns {left + 1}-{right - 1} below row {horizon}  linear {m[0]:.3f}/{m[1]:.3f}/{m[2]:.3f}"
        f"  R:G:B {m[0] / m[1]:.2f}:1:{m[2] / m[1]:.2f}"
    )
