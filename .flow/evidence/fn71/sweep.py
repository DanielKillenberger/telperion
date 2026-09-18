"""The structure that survives a walk: each far still against the near still
reduced by the matching factor, band by band, inside the near clay's wood.

    uv run --with numpy --with pillow .flow/evidence/fn71/sweep.py <dir> <preset>

A band ratio of one is the near draw minus what the pixel cannot hold; a
ratio well under one at a band the pixel still resolves is detail that left
early, the step the eye sees."""
import sys
from pathlib import Path

import numpy as np
from PIL import Image, ImageFilter

FACTORS = [1.5, 2, 3, 4, 6, 8]
BANDS = [2, 4, 8, 16]  # box widths in pixels of the reduced frame


def luminance(path):
    rgb = np.asarray(Image.open(path).convert("RGB"), dtype=np.float64)
    return rgb @ np.array([0.2126, 0.7152, 0.0722])


def reduce(array, factor):
    image = Image.fromarray(array.astype(np.float32), mode="F")
    size = (round(image.width / factor), round(image.height / factor))
    return np.asarray(image.resize(size, Image.BOX), dtype=np.float64)


def centre(array, size):
    h, w = array.shape
    x0, y0 = (w - size[0]) // 2, (h - size[1]) // 2
    return array[y0 : y0 + size[1], x0 : x0 + size[0]]


def blur(array, width):
    """A box mean of `width` pixels on each axis, edges padded by reflection."""
    def along(a, axis):
        pad = [(0, 0)] * a.ndim
        pad[axis] = (width // 2, width - 1 - width // 2)
        padded = np.pad(a, pad, mode="reflect")
        total = np.cumsum(padded, axis=axis)
        total = np.concatenate([np.zeros_like(np.take(total, [0], axis=axis)), total], axis=axis)
        head = np.take(total, range(width, width + a.shape[axis]), axis=axis)
        tail = np.take(total, range(0, a.shape[axis]), axis=axis)
        return (head - tail) / width
    return along(along(array, 0), 1)


def band_energy(array, mask, width):
    detail = array - blur(array, width)
    return detail[mask].std()


def main(folder, preset):
    """`<preset>-x<f>.png` is the camera walked to f times the distance, its
    centre crop compared; `<preset>-r<f>.png` is the same camera drawn at a
    fth of the size, registered pixel for pixel, the footprint walked alone."""
    folder = Path(folder)
    near = luminance(folder / f"{preset}-x1.png")
    clay = np.asarray(Image.open(folder / f"{preset}-clay.png").convert("RGB"), dtype=np.float64)
    wood = (clay[..., 0] > clay[..., 2]).astype(np.float64)
    print(f"| factor | pixels | mean near / far | std near / far | " + " | ".join(f"band {b}px" for b in BANDS) + " | rms diff |")
    print("|---|---|---|---|" + "---|" * len(BANDS) + "---|")
    for factor in FACTORS:
        reduced = reduce(near, factor)
        walked = folder / f"{preset}-x{factor:g}.png"
        if walked.exists():
            far = centre(luminance(walked), reduced.shape[::-1])
        else:
            far = luminance(folder / f"{preset}-r{factor:g}.png")
            assert far.shape == reduced.shape, (far.shape, reduced.shape)
        mask = reduce(wood, factor) > 0.999
        mask = np.asarray(Image.fromarray(mask.astype(np.uint8) * 255).filter(ImageFilter.MinFilter(9))) > 0
        mask &= centre(np.ones_like(far, dtype=bool), reduced.shape[::-1])
        ratios = []
        for width in BANDS:
            a, b = band_energy(reduced, mask, width), band_energy(far, mask, width)
            ratios.append(f"{a:.2f} / {b:.2f} = {b / a:.2f}")
        rms = np.sqrt(((far - reduced) ** 2)[mask].mean())
        print(
            f"| {factor:g} | {mask.sum()} | {reduced[mask].mean():.1f} / {far[mask].mean():.1f} "
            f"| {reduced[mask].std():.2f} / {far[mask].std():.2f} | " + " | ".join(ratios) + f" | {rms:.2f} |"
        )


if __name__ == "__main__":
    main(sys.argv[1], sys.argv[2])
