"""Re-evaluate fn71's band metric at full precision; run from the repo root."""
import importlib.util
import json
import sys
from pathlib import Path
import numpy as np
from PIL import Image, ImageFilter

spec = importlib.util.spec_from_file_location("sweep", ".flow/evidence/fn71/sweep.py")
sweep = importlib.util.module_from_spec(spec)
spec.loader.exec_module(sweep)
evidence = Path(__file__).parent
records = []
for name in (sys.argv[2:] or ["beech", "birch", "birchhero"]):
    directory = evidence / (sys.argv[1] if len(sys.argv) > 1 else "sweep") / name
    provenance = json.loads((directory / f"{name}-sweep.json").read_text())
    factors = [row["factor"] for row in provenance["reductions"]]
    near = sweep.luminance(directory / f"{name}-x1.png")
    clay = np.asarray(Image.open(directory / f"{name}-clay.png").convert("RGB"), dtype=np.float64)
    wood = (clay[..., 0] > clay[..., 2]).astype(np.float64)
    ratios = []
    for factor in factors:
        reduced = sweep.reduce(near, factor)
        far = sweep.luminance(directory / f"{name}-r{factor:g}.png")
        assert far.shape == reduced.shape
        mask = sweep.reduce(wood, factor) > 0.999
        mask = np.asarray(Image.fromarray(mask.astype(np.uint8) * 255).filter(ImageFilter.MinFilter(9))) > 0
        ratios.append([float(sweep.band_energy(far, mask, width) / sweep.band_energy(reduced, mask, width)) for width in sweep.BANDS])
    maximum = float(np.abs(np.diff(np.array(ratios), axis=0)).max())
    records.append(dict(species=name, factors=factors, bands=sweep.BANDS, ratios=ratios,
                        max_adjacent_step=maximum, bound=0.03, passed=maximum <= 0.03))
    print(f"{name}: max adjacent step {maximum:.6f}, bound 0.03")
(evidence / ((sys.argv[1] if len(sys.argv) > 1 else "sweep") + "-check.json")).write_text(json.dumps(records, indent=2) + "\n")
assert all(record["passed"] for record in records), "footprint continuity regression"
