"""Renders every generator shot of the fn-167 video with the headless target
and writes RENDERS.md from the same table, so the record is the command run.

Run from the evidence folder after `cargo build --profile ci -p
telperion-render --example headless` at the repository root:
    python3 video/render.py [name ...]      render these shots (all without names)
    python3 video/render.py --record        rewrite RENDERS.md only
Frames land in raw/renders/<name>/ (ignored); a still is still.png, a walk is
frame-0001.png onward."""
import json
import os
import shlex
import subprocess
import sys
from dataclasses import dataclass, field

ROOT = "../../.."
HEADLESS = f"{ROOT}/target/ci/examples/headless"
OUT = "raw/renders"
# The one studio every tree stands in: a dusk sky, a dark ground, a low sun.
SCENE = {
    "skyZenithRed": 0.03, "skyZenithGreen": 0.04, "skyZenithBlue": 0.07,
    "skyHorizonRed": 0.16, "skyHorizonGreen": 0.17, "skyHorizonBlue": 0.2,
    "groundRed": 0.07, "groundGreen": 0.07, "groundBlue": 0.07,
    "sunElevation": 24, "sunAzimuth": 150,
}
MASTER, TALL = "2304x1296", "1296x2304"  # 1.2x, room for a slow push
WALK_MASTER, WALK_TALL = "1920x1080", "1080x1920"


@dataclass
class Shot:
    name: str
    preset: str
    size: str
    seed: int = 1
    view: str = "whole"
    camera: dict | None = None
    family: str | None = None  # an overlay file laid over the preset
    to: str | None = None  # a walk's far end
    walk: float | None = None
    hold: float = 0.0
    sweep: float = 0.0
    note: str = ""
    extra: list[str] = field(default_factory=list)

    def command(self) -> list[str]:
        out = f"{OUT}/{self.name}/{'frame' if self.to else 'still'}.png"
        args = [HEADLESS, "--preset", self.preset, "--seed", str(self.seed),
                "--size", self.size, "--view", self.view, "--no-figure",
                "--scene", json.dumps(SCENE, separators=(",", ":")), "--out", out]
        if self.camera:
            args += ["--camera", json.dumps(self.camera, separators=(",", ":"))]
        if self.family:
            args += ["--family", self.family]
        if self.to:
            args += ["--to", self.to, "--walk", str(self.walk),
                     "--hold", str(self.hold), "--sweep", str(self.sweep)]
        return args + self.extra


BARK = {"azimuth": 200, "elevation": 14, "fill": 3.2, "targetHeight": 0.1, "fov": 30}
CROWN = {"azimuth": 115, "elevation": 8, "fill": 2.0, "targetHeight": 0.76, "fov": 34}
INTO = {"azimuth": 160, "elevation": 10, "fill": 1.8, "targetHeight": 0.62, "fov": 36}
FACING = {"azimuth": 115, "elevation": 12, "fill": 0.86, "targetHeight": 0.5, "fov": 38}
NO_WRITHE = "video/overlays/telperion-no-writhe.json"
NO_TWIST = "video/overlays/telperion-no-twist.json"

SHOTS = [
    Shot("telperion-hero", "telperion", MASTER, note="title, the DRAW stage and end card"),
    Shot("telperion-hero-v", "telperion", TALL, note="9:16 title and end card"),
    Shot("telperion-drift", "telperion", WALK_MASTER, to="telperion", walk=5, sweep=30,
         note="the preset walked to itself: one tree, the camera drifting"),
    Shot("telperion-drift-v", "telperion", WALK_TALL, to="telperion", walk=3, sweep=20,
         note="9:16"),
    Shot("telperion-bark", "telperion", MASTER, camera=BARK),
    Shot("telperion-leaf", "telperion", MASTER, view="leaf", note="fidelity and the PLAN stage"),
    Shot("date-palm-crown", "date-palm", MASTER, camera=CROWN),
    Shot("birch-crown", "silver-birch", MASTER, camera=INTO),
    Shot("beech-leaf", "european-beech", MASTER, view="leaf"),
    Shot("oak-hero", "oregon-white-oak", MASTER),
    Shot("telperion-bare", "telperion", MASTER, view="bare", note="the EXPAND stage"),
    Shot("telperion-no-writhe", "telperion", MASTER, camera=FACING, family=NO_WRITHE,
         note="the bias field's writhe at zero"),
    Shot("telperion-writhe", "telperion", MASTER, camera=FACING,
         note="Telperion as shipped, the same camera"),
    Shot("telperion-twist", "telperion", WALK_MASTER, family=NO_TWIST, to="telperion",
         walk=5, hold=0.5, sweep=25, note="/surface/twistRate 0 to 2.4"),
    Shot("telperion-twist-v", "telperion", WALK_TALL, family=NO_TWIST, to="telperion",
         walk=3, hold=0.25, sweep=15, note="9:16"),
    *[Shot(f"oak-seed-{s}", "oregon-white-oak", "960x1080", seed=s, camera=FACING)
      for s in (1, 2, 3, 4)],
]


DRAWN = """## Drawn shots

These draw a generator artifact rather than render a tree. Each reads only
what the pipeline or the field package returned; the drawing adds a camera
and colour, never geometry.

| Shot | Source | Command (from this folder) |
|---|---|---|
| grow, grow-v | `dump stages telperion 1`: the 1,600 attractors the grower scatters (its inset envelope restated, same count, seed and sampling attempts) and the 75,697 nodes in the order they were grown. An attractor is drawn going out once a scaffold node lands within the kill distance (6.51 m): a reading of the consume rule, not its record | `node video/draw/skeleton.ts raw/stages/telperion-1.json grow raw/drawn/grow 1920x1080 216 115 40`, and `... grow raw/drawn/grow-v 1080x1920 96 115 25` |
| skeleton (GROW) | the same dump's whole skeleton | `node video/draw/skeleton.ts raw/stages/telperion-1.json skeleton raw/drawn/skeleton 1920x1080 96 140 20` |
| leaves (CULL) | the same dump's kept leaves, every 7th of 534,778, over the skeleton | `node video/draw/skeleton.ts raw/stages/telperion-1.json leaves raw/drawn/leaves 1920x1080 96 160 20` |
| consumers-smooth, -blocks, -points | `growField("oregon-white-oak", 1)` from `src/field` (the slim Wasm built with `cargo build --release --target wasm32-unknown-unknown -p telperion-field`), one batch query on an 88, 36 and 120-cell grid; the blocks through the example voxelizer (`woodCutoff` 0.18 of a cell, `gap` thinning at 0.55) | `node --import ./video/draw/ts-resolve.ts video/draw/consumers.ts oregon-white-oak 1 <style> raw/drawn/consumers-<style> 620x940 288 100 70` |

The stage figures on screen come from `raw/dump-target/release/fn167-dump
stages telperion 1` on an idle machine, the second of three runs, on the CPU
reference: grow 476 ms, plan 10 ms, rings and wood 51 ms, placement and cull
587 ms. The draw figure is `headless --preset telperion --seed 1 --size
1920x1080 --no-figure --timing`: vegetation and selection p50 1.785 ms on the
RTX 3080.

## Composition

`video/compose.py master|vertical` builds every clip with ffmpeg n9.0.1
(scale, crop, drawtext, drawbox, overlay, xfade; Noto Sans and Adwaita Mono)
and joins the clips with 0.3 s cross-fades into H.264 (libx264, CRF 18,
yuv420p, 24 fps, no audio). The parameter catalogue column is every row
heading of `docs/parameters.md`, set by ImageMagick. The drawings are
TypeScript run by Node 26's type stripping; the dump is a standalone Rust
crate over `telperion-core`, its own workspace with its target in `raw/`.
All of it lives in this evidence folder rather than `scripts/`: it is a
one-off for this video, it reads this folder's raw outputs, and nothing in the
repository's builds or tests should compile or run it.
"""


def record(shots: list[Shot], revision: str) -> str:
    lines = [
        "# fn-167 render records",
        "",
        f"Generator revision `{revision}`, headless target built with "
        "`cargo build --profile ci -p telperion-render --example headless`, "
        "on the RTX 3080. Written by `video/render.py` from the table it renders; "
        "run from this folder. Every render uses one scene row (below) and no "
        "scale figure. Overlays are partial family rows laid over the preset.",
        "",
        f"Scene: `{json.dumps(SCENE, separators=(',', ':'))}`",
        "",
        "| Shot | Preset | Seed | Rows | View | Size | Walk | Note |",
        "|---|---|---|---|---|---|---|---|",
    ]
    for s in shots:
        rows = "preset"
        if s.family:
            with open(s.family) as f:
                rows = f"`{f.read().strip()}` then to the preset" if s.to else \
                    f"`{f.read().strip()}`"
        walk = f"to `{s.to}`, {s.walk} s, hold {s.hold} s, sweep {s.sweep} deg" \
            if s.to else "still"
        camera = f"camera `{json.dumps(s.camera, separators=(',', ':'))}`" if s.camera else ""
        note = "; ".join(part for part in (s.note, camera) if part)
        lines.append(f"| {s.name} | {s.preset} | {s.seed} | {rows} | {s.view} | "
                     f"{s.size} | {walk} | {note} |")
    lines += ["", "Commands, as run:", "", "```sh"]
    lines += [shlex.join(s.command()) for s in shots]
    lines += ["```", "", DRAWN]
    return "\n".join(lines)


def main() -> None:
    wanted = set(sys.argv[1:])
    if wanted == {"--record"}:
        wanted = {""}  # no shot has an empty name: render nothing
    revision = subprocess.run(["git", "rev-parse", "HEAD"], capture_output=True,
                              text=True, check=True).stdout.strip()
    for shot in SHOTS:
        if wanted and shot.name not in wanted:
            continue
        os.makedirs(f"{OUT}/{shot.name}", exist_ok=True)
        done = subprocess.run(shot.command(), capture_output=True, text=True)
        if done.returncode:
            sys.exit(f"{shot.name}: {done.stderr.strip()}")
        print(f"{shot.name}: {done.stdout.strip().splitlines()[-1][:160]}")
    with open("RENDERS.md", "w") as f:
        f.write(record(SHOTS, revision))


if __name__ == "__main__":
    main()
