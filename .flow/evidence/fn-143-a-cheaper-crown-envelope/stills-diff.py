"""R3 support: per-pair pixel difference of base and candidate stills.
Usage: uv run --with pillow --with numpy stills-diff.py <stills-dir> > stills-diff.json"""
import json, sys, hashlib, pathlib
import numpy as np
from PIL import Image
root = pathlib.Path(sys.argv[1]); rows = {}
for b in sorted((root / 'base').glob('*.png')):
    c = root / 'candidate' / b.name
    same = hashlib.sha256(b.read_bytes()).digest() == hashlib.sha256(c.read_bytes()).digest()
    x = np.asarray(Image.open(b).convert('RGB'), dtype=np.int16)
    y = np.asarray(Image.open(c).convert('RGB'), dtype=np.int16)
    d = np.abs(x - y).max(axis=2)
    rows[b.stem] = {'identical_png': same, 'changed_pixels_pct': round(float((d > 8).mean() * 100), 3),
                    'mean_abs_difference': round(float(np.abs(x - y).mean()), 3)}
print(json.dumps(rows, indent=1))
