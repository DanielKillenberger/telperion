#!/usr/bin/env python3
"""One beech value trial on the integration worktree: set rows, render the
matched stills, measure against the photographs, restore the preset.

    python3 trial.py <name> row=value [row=value ...]

Rows are the beech's own lines in presets/species.rs, found inside
`fn european_beech` by their field name; nothing else is touched."""
from __future__ import annotations

import json
import re
import subprocess
import sys
from pathlib import Path

REPO = Path("/home/daniel/Projects/telperion/.worktrees/fn-34-integration")
SPECIES = REPO / "crates/telperion-core/src/presets/species.rs"
MEASURE = REPO / ".flow/evidence/fn34/measure"
HERE = Path(__file__).parent
# Row name -> the text that starts its line inside the beech function.
ROWS = {
    "irregularity": "        irregularity:", "lobe_scale": "        lobe_scale:",
    "fullness": "        fullness:", "shoulder": "        shoulder:", "spread": "        spread:",
    "twig_tip_taper": "        twig_tip_taper:", "lateral_length_ratio": "        lateral_length_ratio:",
    "twig_laterals": "    p.skeleton.twigs.laterals =", "twig_length_ratio": "    p.skeleton.twigs.length_ratio =",
    "leaf_internode": "    p.skeleton.twigs.twig.internode_length =", "length_taper": "    p.radii.length_taper =",
    "short_shoot_spacing": "    p.canopy.short_shoot_spacing =", "short_shoot_leaves": "    p.canopy.short_shoot_leaves =",
    "limb_clumping": "    p.canopy.limb_clumping =", "crown_base": "        crown_base:",
}
BANDS = 6


def patch(text: str, values: dict[str, str]) -> str:
    start = text.index("pub(super) fn european_beech(")
    end = text.index("\n}\n", start)
    body = text[start:end].split("\n")
    for row, value in values.items():
        lead = ROWS[row]
        hits = [i for i, l in enumerate(body) if l.startswith(lead)]
        assert len(hits) == 1, (row, len(hits))
        sep = ":" if lead.endswith(":") else " ="
        tail = "," if sep == ":" else ";"
        body[hits[0]] = f"{lead[:-1] if sep == ':' else lead[:-2]}{sep} {value}{tail}"
    return text[:start] + "\n".join(body) + text[end:]


def bands() -> dict:
    """Occupied share per sixth of the tree box, top to bottom, for the still
    (its two-sun mask) and the photograph (dark pixels in the record's box),
    with the compare script's own definitions."""
    code = r'''
import json, sys, importlib.util
from pathlib import Path
spec = importlib.util.spec_from_file_location("cmp", sys.argv[1]); cmp = importlib.util.module_from_spec(spec); spec.loader.exec_module(cmp)
import numpy as np
refs = {r["id"]: r for r in json.loads(Path(sys.argv[2]).read_text())["references"] if r.get("shot")}
out = {}
for rid in ("B-WHOLE", "B-BARE"):
    r = refs[rid]; shot = r["shot"]
    still = cmp.load(Path(sys.argv[3]) / f"european-beech-1-{rid}.png"); twin = cmp.load(Path(sys.argv[3]) / f"european-beech-1-{rid}-twin.png")
    mask = cmp.tree_mask(still, twin); x, y, w, h = cmp.box_of(mask); m = mask[y:y+h, x:x+w]
    photo = cmp.crop(cmp.load(Path(sys.argv[4]) / r["url"].rsplit("/", 1)[-1]), shot.get("crop"))
    ph, pw = photo.shape[:2]; bx, by, bw, bh = shot["tree"]["box"]
    pc = photo[int(by*ph):int(by*ph)+max(1,int(bh*ph)), int(bx*pw):int(bx*pw)+max(1,int(bw*pw))]
    dark = (pc @ np.array([0.2126, 0.7152, 0.0722], dtype=np.float32)) < cmp.DARK
    cut = lambda a: [round(float(b.mean()), 3) for b in np.array_split(a, int(sys.argv[5]), axis=0)]
    out[rid] = {"still": cut(m), "photo": cut(dark)}
print(json.dumps(out))
'''
    res = subprocess.run(["uv", "run", "--with", "pillow", "--with", "numpy", "python", "-c", code,
                          str(REPO / "scripts/compare-references.py"),
                          str(REPO / ".flow/evidence/fn34/european-beech/references.json"),
                          str(MEASURE / "trial-capture"), str(REPO / ".refs/fn34/european-beech"), str(BANDS)],
                         capture_output=True, text=True, cwd=REPO)
    if res.returncode:
        return {"error": res.stderr[-400:]}
    return json.loads(res.stdout)


def run(cmd: list[str], log) -> int:
    return subprocess.run(cmd, cwd=REPO, stdout=log, stderr=subprocess.STDOUT).returncode


def main() -> None:
    name, pairs = sys.argv[1], dict(a.split("=", 1) for a in sys.argv[2:])
    original = SPECIES.read_text()
    log = open(HERE / f"trial-{name}.log", "w")
    result = {"name": name, "values": pairs}
    try:
        SPECIES.write_text(patch(original, pairs))
        cap = MEASURE / "trial-capture"
        cap.mkdir(parents=True, exist_ok=True)
        for f in (MEASURE / "protocol-round22").glob("*.jsonl"):
            (cap / f.name).write_bytes(f.read_bytes())
        for f in cap.glob("european-beech-1-*.json"):
            f.unlink()  # receipts from the last trial; the runner would reuse matching ones
        if run(["cargo", "build", "--release", "-p", "telperion-render", "--example", "headless"], log):
            result["error"] = "build failed"; return
        run(["node", "tests/species.mjs", "--profiles", ".flow/evidence/fn34/profiles.json",
             "--seeds", ".flow/evidence/fn34/seeds.json", "--output", str(cap), "--capture-only",
             "--case", "european-beech-1"], log)
        pairs_dir = MEASURE / f"trial-pairs-{name}"
        run(["uv", "run", "scripts/compare-references.py", "--references",
             ".flow/evidence/fn34/european-beech/references.json", "--captures", str(cap),
             "--refs", ".refs/fn34/european-beech", "--case", "european-beech-1", "--out", str(pairs_dir)], log)
        for rid in ("B-WHOLE", "B-BARE"):
            c = json.loads((pairs_dir / f"european-beech-1-{rid}-compare.json").read_text())
            s, p = c["still"], c["photograph"]
            result[rid] = {k: (round(p[k], 3) if isinstance(p[k], float) else p[k], round(s[k], 3) if isinstance(s[k], float) else s[k])
                           for k in ("width_over_height", "crown_base", "occupied", "outline_deviation")}
            result[rid]["centre"] = (round(p["centre"]["mean"], 1), round(s["centre"]["mean"], 1))
        result["bands"] = bands()
    finally:
        SPECIES.write_text(original)
        log.close()
        (HERE / f"trial-{name}.json").write_text(json.dumps(result, indent=1))
        print(json.dumps(result))


if __name__ == "__main__":
    main()
