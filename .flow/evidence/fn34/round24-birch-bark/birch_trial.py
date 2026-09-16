#!/usr/bin/env python3
"""One birch bark trial on the integration worktree: set rows in the birch's
material table, render its matched stills, measure them, restore the table.

    python3 birch_trial.py <name> row=value [row=value ...]

Besides the compare script's numbers, the S-BARK still and photograph are
read for bark contrast inside their tree box: the luminance spread and the
share of near-black pixels."""
from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path

REPO = Path("/home/daniel/Projects/telperion/.worktrees/fn-34-integration")
MATERIALS = REPO / "crates/telperion-core/src/presets/materials.rs"
MEASURE = REPO / ".flow/evidence/fn34/measure"
HERE = Path(__file__).parent
REFS = ("S-WHOLE", "S-BARE", "S-BARK")


def patch(text: str, values: dict[str, str]) -> str:
    start = text.index("pub(super) fn birch() -> MaterialParams {")
    end = text.index("\n}\n", start)
    body = text[start:end].split("\n")
    for row, value in values.items():
        hits = [i for i, l in enumerate(body) if l.startswith(f"        {row}:")]
        assert len(hits) == 1, (row, len(hits))
        body[hits[0]] = f"        {row}: {value},"
    return text[:start] + "\n".join(body) + text[end:]


CONTRAST = r'''
import json, sys, importlib.util
from pathlib import Path
import numpy as np
spec = importlib.util.spec_from_file_location("cmp", sys.argv[1]); cmp = importlib.util.module_from_spec(spec); spec.loader.exec_module(cmp)
r = next(x for x in json.loads(Path(sys.argv[2]).read_text())["references"] if x["id"] == "S-BARK")
shot = r["shot"]; lum = np.array([0.2126, 0.7152, 0.0722], dtype=np.float32)
still = cmp.load(Path(sys.argv[3]) / "silver-birch-1-S-BARK.png"); twin = cmp.load(Path(sys.argv[3]) / "silver-birch-1-S-BARK-twin.png")
mask = cmp.tree_mask(still, twin); x, y, w, h = cmp.box_of(mask)
s = (still[y:y+h, x:x+w] @ lum)[mask[y:y+h, x:x+w]]
photo = cmp.crop(cmp.load(Path(sys.argv[4]) / r["url"].rsplit("/", 1)[-1]), shot.get("crop"))
ph, pw = photo.shape[:2]; bx, by, bw, bh = shot["tree"]["box"]
p = (photo[int(by*ph):int(by*ph)+max(1,int(bh*ph)), int(bx*pw):int(bx*pw)+max(1,int(bw*pw))] @ lum).ravel()
stat = lambda a: {"mean": round(float(a.mean()) * 255, 1), "spread": round(float(a.std()) * 255, 1), "near_black": round(float((a < 0.12).mean()), 3), "p05": round(float(np.percentile(a, 5)) * 255, 1)}
print(json.dumps({"photo": stat(p), "still": stat(s)}))
'''


def run(cmd, log):
    return subprocess.run(cmd, cwd=REPO, stdout=log, stderr=subprocess.STDOUT).returncode


def main() -> None:
    name, values = sys.argv[1], dict(a.split("=", 1) for a in sys.argv[2:])
    original = MATERIALS.read_text()
    result = {"name": name, "values": values}
    log = open(HERE / f"birch-{name}.log", "w")
    try:
        MATERIALS.write_text(patch(original, values))
        cap = MEASURE / "birch-trial-capture"
        cap.mkdir(parents=True, exist_ok=True)
        for f in (MEASURE / "protocol-round22").glob("*.jsonl"):
            (cap / f.name).write_bytes(f.read_bytes())
        for f in cap.glob("silver-birch-1-*.json"):
            f.unlink()
        if run(["cargo", "build", "--release", "-p", "telperion-render", "--example", "headless"], log):
            result["error"] = "build failed"; return
        run(["node", "tests/species.mjs", "--profiles", ".flow/evidence/fn34/profiles.json", "--seeds",
             ".flow/evidence/fn34/seeds.json", "--output", str(cap), "--capture-only", "--case", "silver-birch-1"], log)
        pairs = MEASURE / f"birch-pairs-{name}"
        run(["uv", "run", "scripts/compare-references.py", "--references", ".flow/evidence/fn34/silver-birch/references.json",
             "--captures", str(cap), "--refs", ".refs/fn34/silver-birch", "--case", "silver-birch-1", "--out", str(pairs)], log)
        for rid in REFS:
            c = json.loads((pairs / f"silver-birch-1-{rid}-compare.json").read_text())
            result[rid] = {"centre": (round(c["photograph"]["centre"]["mean"], 1), round(c["still"]["centre"]["mean"], 1)),
                           "occupied": (round(c["photograph"]["occupied"], 3), round(c["still"]["occupied"], 3))}
        res = subprocess.run(["uv", "run", "--with", "pillow", "--with", "numpy", "python", "-c", CONTRAST,
                              str(REPO / "scripts/compare-references.py"), str(REPO / ".flow/evidence/fn34/silver-birch/references.json"),
                              str(cap), str(REPO / ".refs/fn34/silver-birch")], capture_output=True, text=True, cwd=REPO)
        result["bark"] = json.loads(res.stdout) if res.returncode == 0 else {"error": res.stderr[-300:]}
    finally:
        MATERIALS.write_text(original)
        log.close()
        (HERE / f"birch-{name}.json").write_text(json.dumps(result, indent=1))
        print(json.dumps(result))


if __name__ == "__main__":
    main()
