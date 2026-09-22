#!/usr/bin/env python3
"""Bounded historical capture replay. Never assigns an owner verdict."""
import argparse
import hashlib
import json
import subprocess
from pathlib import Path

p = argparse.ArgumentParser()
p.add_argument("--source", type=Path, required=True)
p.add_argument("--preset", required=True)
p.add_argument("--views", nargs="+", required=True)
p.add_argument("--out", type=Path, required=True)
a = p.parse_args()
if len(a.views) > 4:
    p.error("at most four images per invocation")
records = json.loads((a.source / ".flow/evidence/fn34" / a.preset / "references.json").read_text())["references"]
a.out.mkdir(parents=True, exist_ok=True)
for view in a.views:
    record = next(r for r in records if r["id"] == view)
    shot = record["shot"]
    light, camera = shot["light"], shot["camera"]
    overcast = light["overcast"]
    dim = 1 - 0.8 * overcast
    scene = {"sunAzimuth": light["sunAzimuth"], "sunElevation": light["sunElevation"],
             "sunRed": 3 * dim, "sunGreen": 2.85 * dim, "sunBlue": 2.6 * dim,
             "skyZenithRed": 0.18 + 0.37 * overcast,
             "skyZenithGreen": 0.30 + 0.36 * overcast,
             "skyZenithBlue": 0.62 + 0.18 * overcast}
    path = a.out / f"{a.preset}-{view}.png"
    if path.exists():
        raise SystemExit(f"refusing to overwrite {path}")
    cmd = [str(a.source / "target/release/examples/headless"), "--preset", a.preset,
           "--seed", "1", "--view", "bare" if shot["foliage"] == "hidden" else "whole",
           "--size", f"{round(1440 * shot['aspect'][0] / shot['aspect'][1])}x1440",
           "--out", str(path), "--camera", json.dumps(camera),
           "--scene", json.dumps(scene), "--no-figure"]
    subprocess.run(cmd, cwd=a.source, timeout=300, check=True, capture_output=True)
    print(json.dumps({"view": view, "seed": 1, "path": str(path),
                      "sha256": hashlib.sha256(path.read_bytes()).hexdigest()}), flush=True)
