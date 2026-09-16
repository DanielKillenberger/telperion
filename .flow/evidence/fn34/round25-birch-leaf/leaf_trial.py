#!/usr/bin/env python3
"""One leaf trial on the integration worktree: set a species' element rows
(species.rs) and leaf material rows (materials.rs), render the leaf view, pair
it with the leaf photographs, restore both files.

    python3 leaf_trial.py <name> <species> el.row=value ... mat.row=value ...
"""
from __future__ import annotations

import subprocess
import sys
from pathlib import Path

REPO = Path("/home/daniel/Projects/telperion/.worktrees/fn-34-integration")
SPECIES = REPO / "crates/telperion-core/src/presets/species.rs"
MATERIALS = REPO / "crates/telperion-core/src/presets/materials.rs"
OUT = REPO / ".flow/evidence/fn34/measure/leaf-trials"
FN = {"silver-birch": ("pub(super) fn silver_birch(", "pub(super) fn birch() -> MaterialParams {", ("S-LEAF", "S-SHOOT")),
      "european-beech": ("pub(super) fn european_beech(", "pub(super) fn beech() -> MaterialParams {", ("B-LEAF", "B-LEAVES"))}


def patch(text: str, start_marker: str, rows: dict[str, str], within: str | None = None) -> str:
    start = text.index(start_marker)
    end = text.index("\n}\n", start)
    if within:  # only inside `within { ... }` of that function
        start = text.index(within, start)
        end = text.index("\n    };", start)
    body = text[start:end].split("\n")
    for row, value in rows.items():
        hits = [i for i, l in enumerate(body) if l.startswith(f"        {row}:")]
        if not hits:  # a row the table leaves to Default: state it
            body.insert(len(body) - 1, f"        {row}: {value},")
            continue
        assert len(hits) == 1, (row, len(hits))
        body[hits[0]] = f"        {row}: {value},"
    return text[:start] + "\n".join(body) + text[end:]


def main() -> None:
    name, sp = sys.argv[1], sys.argv[2]
    el = {a[3:].split("=")[0]: a.split("=", 1)[1] for a in sys.argv[3:] if a.startswith("el.")}
    mat = {a[4:].split("=")[0]: a.split("=", 1)[1] for a in sys.argv[3:] if a.startswith("mat.")}
    fn, mfn, refs = FN[sp]
    s0, m0 = SPECIES.read_text(), MATERIALS.read_text()
    OUT.mkdir(parents=True, exist_ok=True)
    png = OUT / f"{sp}-{name}-leaf.png"
    try:
        s = patch(s0, fn, el, within="    p.element = ElementParams {") if el else s0
        SPECIES.write_text(s)
        MATERIALS.write_text(patch(m0, mfn, mat) if mat else m0)
        build = subprocess.run(["cargo", "build", "--release", "-p", "telperion-render", "--example", "headless"],
                               cwd=REPO, capture_output=True, text=True)
        if build.returncode:
            print(build.stderr[-1500:]); return
        run = subprocess.run(["target/release/examples/headless", "--preset", sp, "--seed", "1", "--view", "leaf",
                              "--size", "960x720", "--out", str(png)], cwd=REPO, capture_output=True, text=True)
        print(run.stdout.strip()[-300:], run.stderr.strip()[-600:])
    finally:
        SPECIES.write_text(s0)
        MATERIALS.write_text(m0)
    code = r'''
import importlib.util, json, sys
from pathlib import Path
spec = importlib.util.spec_from_file_location("cmp", "scripts/compare-references.py"); cmp = importlib.util.module_from_spec(spec); spec.loader.exec_module(cmp)
sp, name, png, out = sys.argv[1:5]
recs = {r["id"]: r for r in json.loads(Path(f".flow/evidence/fn34/{sp}/references.json").read_text())["references"]}
rid = sys.argv[5]
photo = cmp.load(Path(".refs/fn34") / sp / recs[rid]["url"].rsplit("/", 1)[-1])
cmp.pair(photo, cmp.load(Path(png)), f"{rid} | {sp}-1 | leaf trial {name}", Path(out) / f"{sp}-{name}-{rid}-pair.png")
print(Path(out) / f"{sp}-{name}-{rid}-pair.png")
'''
    subprocess.run(["uv", "run", "--with", "pillow", "--with", "numpy", "python", "-c", code, sp, name, str(png), str(OUT), refs[0]],
                   cwd=REPO, check=True)


if __name__ == "__main__":
    main()
