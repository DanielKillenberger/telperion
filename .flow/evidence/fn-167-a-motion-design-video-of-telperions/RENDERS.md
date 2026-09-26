# fn-167 render records

Generator revision `b1d15d931b773bc59c9dd5fc914c8a9944aa7657`, headless target built with `cargo build --profile ci -p telperion-render --example headless`, on the RTX 3080. Written by `video/render.py` from the table it renders; run from this folder. Every render uses one scene row (below) and no scale figure. Overlays are partial family rows laid over the preset.

Scene: `{"skyZenithRed":0.03,"skyZenithGreen":0.04,"skyZenithBlue":0.07,"skyHorizonRed":0.16,"skyHorizonGreen":0.17,"skyHorizonBlue":0.2,"groundRed":0.07,"groundGreen":0.07,"groundBlue":0.07,"sunElevation":24,"sunAzimuth":150}`

| Shot | Preset | Seed | Rows | View | Size | Walk | Note |
|---|---|---|---|---|---|---|---|
| telperion-hero | telperion | 1 | preset | whole | 2304x1296 | still | title, the DRAW stage and end card |
| telperion-hero-v | telperion | 1 | preset | whole | 1296x2304 | still | 9:16 title and end card |
| telperion-drift | telperion | 1 | preset | whole | 1920x1080 | to `telperion`, 5 s, hold 0.0 s, sweep 30 deg | the preset walked to itself: one tree, the camera drifting |
| telperion-drift-v | telperion | 1 | preset | whole | 1080x1920 | to `telperion`, 3 s, hold 0.0 s, sweep 20 deg | 9:16 |
| telperion-bark | telperion | 1 | preset | whole | 2304x1296 | still | camera `{"azimuth":200,"elevation":14,"fill":3.2,"targetHeight":0.1,"fov":30}` |
| telperion-leaf | telperion | 1 | preset | leaf | 2304x1296 | still | fidelity and the PLAN stage |
| date-palm-crown | date-palm | 1 | preset | whole | 2304x1296 | still | camera `{"azimuth":115,"elevation":8,"fill":2.0,"targetHeight":0.76,"fov":34}` |
| birch-crown | silver-birch | 1 | preset | whole | 2304x1296 | still | camera `{"azimuth":160,"elevation":10,"fill":1.8,"targetHeight":0.62,"fov":36}` |
| beech-leaf | european-beech | 1 | preset | leaf | 2304x1296 | still |  |
| oak-hero | oregon-white-oak | 1 | preset | whole | 2304x1296 | still |  |
| telperion-bare | telperion | 1 | preset | bare | 2304x1296 | still | the EXPAND stage |
| telperion-no-writhe | telperion | 1 | `{"skeleton":{"bias":{"supernatural":{"writheAmplitude":0}}}}` | whole | 2304x1296 | still | the bias field's writhe at zero; camera `{"azimuth":115,"elevation":12,"fill":0.86,"targetHeight":0.5,"fov":38}` |
| telperion-writhe | telperion | 1 | preset | whole | 2304x1296 | still | Telperion as shipped, the same camera; camera `{"azimuth":115,"elevation":12,"fill":0.86,"targetHeight":0.5,"fov":38}` |
| telperion-twist | telperion | 1 | `{"surface":{"twistRate":0}}` then to the preset | whole | 1920x1080 | to `telperion`, 5 s, hold 0.5 s, sweep 25 deg | /surface/twistRate 0 to 2.4 |
| telperion-twist-v | telperion | 1 | `{"surface":{"twistRate":0}}` then to the preset | whole | 1080x1920 | to `telperion`, 3 s, hold 0.25 s, sweep 15 deg | 9:16 |
| oak-seed-1 | oregon-white-oak | 1 | preset | whole | 960x1080 | still | camera `{"azimuth":115,"elevation":12,"fill":0.86,"targetHeight":0.5,"fov":38}` |
| oak-seed-2 | oregon-white-oak | 2 | preset | whole | 960x1080 | still | camera `{"azimuth":115,"elevation":12,"fill":0.86,"targetHeight":0.5,"fov":38}` |
| oak-seed-3 | oregon-white-oak | 3 | preset | whole | 960x1080 | still | camera `{"azimuth":115,"elevation":12,"fill":0.86,"targetHeight":0.5,"fov":38}` |
| oak-seed-4 | oregon-white-oak | 4 | preset | whole | 960x1080 | still | camera `{"azimuth":115,"elevation":12,"fill":0.86,"targetHeight":0.5,"fov":38}` |

Commands, as run:

```sh
../../../target/ci/examples/headless --preset telperion --seed 1 --size 2304x1296 --view whole --no-figure --scene '{"skyZenithRed":0.03,"skyZenithGreen":0.04,"skyZenithBlue":0.07,"skyHorizonRed":0.16,"skyHorizonGreen":0.17,"skyHorizonBlue":0.2,"groundRed":0.07,"groundGreen":0.07,"groundBlue":0.07,"sunElevation":24,"sunAzimuth":150}' --out raw/renders/telperion-hero/still.png
../../../target/ci/examples/headless --preset telperion --seed 1 --size 1296x2304 --view whole --no-figure --scene '{"skyZenithRed":0.03,"skyZenithGreen":0.04,"skyZenithBlue":0.07,"skyHorizonRed":0.16,"skyHorizonGreen":0.17,"skyHorizonBlue":0.2,"groundRed":0.07,"groundGreen":0.07,"groundBlue":0.07,"sunElevation":24,"sunAzimuth":150}' --out raw/renders/telperion-hero-v/still.png
../../../target/ci/examples/headless --preset telperion --seed 1 --size 1920x1080 --view whole --no-figure --scene '{"skyZenithRed":0.03,"skyZenithGreen":0.04,"skyZenithBlue":0.07,"skyHorizonRed":0.16,"skyHorizonGreen":0.17,"skyHorizonBlue":0.2,"groundRed":0.07,"groundGreen":0.07,"groundBlue":0.07,"sunElevation":24,"sunAzimuth":150}' --out raw/renders/telperion-drift/frame.png --to telperion --walk 5 --hold 0.0 --sweep 30
../../../target/ci/examples/headless --preset telperion --seed 1 --size 1080x1920 --view whole --no-figure --scene '{"skyZenithRed":0.03,"skyZenithGreen":0.04,"skyZenithBlue":0.07,"skyHorizonRed":0.16,"skyHorizonGreen":0.17,"skyHorizonBlue":0.2,"groundRed":0.07,"groundGreen":0.07,"groundBlue":0.07,"sunElevation":24,"sunAzimuth":150}' --out raw/renders/telperion-drift-v/frame.png --to telperion --walk 3 --hold 0.0 --sweep 20
../../../target/ci/examples/headless --preset telperion --seed 1 --size 2304x1296 --view whole --no-figure --scene '{"skyZenithRed":0.03,"skyZenithGreen":0.04,"skyZenithBlue":0.07,"skyHorizonRed":0.16,"skyHorizonGreen":0.17,"skyHorizonBlue":0.2,"groundRed":0.07,"groundGreen":0.07,"groundBlue":0.07,"sunElevation":24,"sunAzimuth":150}' --out raw/renders/telperion-bark/still.png --camera '{"azimuth":200,"elevation":14,"fill":3.2,"targetHeight":0.1,"fov":30}'
../../../target/ci/examples/headless --preset telperion --seed 1 --size 2304x1296 --view leaf --no-figure --scene '{"skyZenithRed":0.03,"skyZenithGreen":0.04,"skyZenithBlue":0.07,"skyHorizonRed":0.16,"skyHorizonGreen":0.17,"skyHorizonBlue":0.2,"groundRed":0.07,"groundGreen":0.07,"groundBlue":0.07,"sunElevation":24,"sunAzimuth":150}' --out raw/renders/telperion-leaf/still.png
../../../target/ci/examples/headless --preset date-palm --seed 1 --size 2304x1296 --view whole --no-figure --scene '{"skyZenithRed":0.03,"skyZenithGreen":0.04,"skyZenithBlue":0.07,"skyHorizonRed":0.16,"skyHorizonGreen":0.17,"skyHorizonBlue":0.2,"groundRed":0.07,"groundGreen":0.07,"groundBlue":0.07,"sunElevation":24,"sunAzimuth":150}' --out raw/renders/date-palm-crown/still.png --camera '{"azimuth":115,"elevation":8,"fill":2.0,"targetHeight":0.76,"fov":34}'
../../../target/ci/examples/headless --preset silver-birch --seed 1 --size 2304x1296 --view whole --no-figure --scene '{"skyZenithRed":0.03,"skyZenithGreen":0.04,"skyZenithBlue":0.07,"skyHorizonRed":0.16,"skyHorizonGreen":0.17,"skyHorizonBlue":0.2,"groundRed":0.07,"groundGreen":0.07,"groundBlue":0.07,"sunElevation":24,"sunAzimuth":150}' --out raw/renders/birch-crown/still.png --camera '{"azimuth":160,"elevation":10,"fill":1.8,"targetHeight":0.62,"fov":36}'
../../../target/ci/examples/headless --preset european-beech --seed 1 --size 2304x1296 --view leaf --no-figure --scene '{"skyZenithRed":0.03,"skyZenithGreen":0.04,"skyZenithBlue":0.07,"skyHorizonRed":0.16,"skyHorizonGreen":0.17,"skyHorizonBlue":0.2,"groundRed":0.07,"groundGreen":0.07,"groundBlue":0.07,"sunElevation":24,"sunAzimuth":150}' --out raw/renders/beech-leaf/still.png
../../../target/ci/examples/headless --preset oregon-white-oak --seed 1 --size 2304x1296 --view whole --no-figure --scene '{"skyZenithRed":0.03,"skyZenithGreen":0.04,"skyZenithBlue":0.07,"skyHorizonRed":0.16,"skyHorizonGreen":0.17,"skyHorizonBlue":0.2,"groundRed":0.07,"groundGreen":0.07,"groundBlue":0.07,"sunElevation":24,"sunAzimuth":150}' --out raw/renders/oak-hero/still.png
../../../target/ci/examples/headless --preset telperion --seed 1 --size 2304x1296 --view bare --no-figure --scene '{"skyZenithRed":0.03,"skyZenithGreen":0.04,"skyZenithBlue":0.07,"skyHorizonRed":0.16,"skyHorizonGreen":0.17,"skyHorizonBlue":0.2,"groundRed":0.07,"groundGreen":0.07,"groundBlue":0.07,"sunElevation":24,"sunAzimuth":150}' --out raw/renders/telperion-bare/still.png
../../../target/ci/examples/headless --preset telperion --seed 1 --size 2304x1296 --view whole --no-figure --scene '{"skyZenithRed":0.03,"skyZenithGreen":0.04,"skyZenithBlue":0.07,"skyHorizonRed":0.16,"skyHorizonGreen":0.17,"skyHorizonBlue":0.2,"groundRed":0.07,"groundGreen":0.07,"groundBlue":0.07,"sunElevation":24,"sunAzimuth":150}' --out raw/renders/telperion-no-writhe/still.png --camera '{"azimuth":115,"elevation":12,"fill":0.86,"targetHeight":0.5,"fov":38}' --family video/overlays/telperion-no-writhe.json
../../../target/ci/examples/headless --preset telperion --seed 1 --size 2304x1296 --view whole --no-figure --scene '{"skyZenithRed":0.03,"skyZenithGreen":0.04,"skyZenithBlue":0.07,"skyHorizonRed":0.16,"skyHorizonGreen":0.17,"skyHorizonBlue":0.2,"groundRed":0.07,"groundGreen":0.07,"groundBlue":0.07,"sunElevation":24,"sunAzimuth":150}' --out raw/renders/telperion-writhe/still.png --camera '{"azimuth":115,"elevation":12,"fill":0.86,"targetHeight":0.5,"fov":38}'
../../../target/ci/examples/headless --preset telperion --seed 1 --size 1920x1080 --view whole --no-figure --scene '{"skyZenithRed":0.03,"skyZenithGreen":0.04,"skyZenithBlue":0.07,"skyHorizonRed":0.16,"skyHorizonGreen":0.17,"skyHorizonBlue":0.2,"groundRed":0.07,"groundGreen":0.07,"groundBlue":0.07,"sunElevation":24,"sunAzimuth":150}' --out raw/renders/telperion-twist/frame.png --family video/overlays/telperion-no-twist.json --to telperion --walk 5 --hold 0.5 --sweep 25
../../../target/ci/examples/headless --preset telperion --seed 1 --size 1080x1920 --view whole --no-figure --scene '{"skyZenithRed":0.03,"skyZenithGreen":0.04,"skyZenithBlue":0.07,"skyHorizonRed":0.16,"skyHorizonGreen":0.17,"skyHorizonBlue":0.2,"groundRed":0.07,"groundGreen":0.07,"groundBlue":0.07,"sunElevation":24,"sunAzimuth":150}' --out raw/renders/telperion-twist-v/frame.png --family video/overlays/telperion-no-twist.json --to telperion --walk 3 --hold 0.25 --sweep 15
../../../target/ci/examples/headless --preset oregon-white-oak --seed 1 --size 960x1080 --view whole --no-figure --scene '{"skyZenithRed":0.03,"skyZenithGreen":0.04,"skyZenithBlue":0.07,"skyHorizonRed":0.16,"skyHorizonGreen":0.17,"skyHorizonBlue":0.2,"groundRed":0.07,"groundGreen":0.07,"groundBlue":0.07,"sunElevation":24,"sunAzimuth":150}' --out raw/renders/oak-seed-1/still.png --camera '{"azimuth":115,"elevation":12,"fill":0.86,"targetHeight":0.5,"fov":38}'
../../../target/ci/examples/headless --preset oregon-white-oak --seed 2 --size 960x1080 --view whole --no-figure --scene '{"skyZenithRed":0.03,"skyZenithGreen":0.04,"skyZenithBlue":0.07,"skyHorizonRed":0.16,"skyHorizonGreen":0.17,"skyHorizonBlue":0.2,"groundRed":0.07,"groundGreen":0.07,"groundBlue":0.07,"sunElevation":24,"sunAzimuth":150}' --out raw/renders/oak-seed-2/still.png --camera '{"azimuth":115,"elevation":12,"fill":0.86,"targetHeight":0.5,"fov":38}'
../../../target/ci/examples/headless --preset oregon-white-oak --seed 3 --size 960x1080 --view whole --no-figure --scene '{"skyZenithRed":0.03,"skyZenithGreen":0.04,"skyZenithBlue":0.07,"skyHorizonRed":0.16,"skyHorizonGreen":0.17,"skyHorizonBlue":0.2,"groundRed":0.07,"groundGreen":0.07,"groundBlue":0.07,"sunElevation":24,"sunAzimuth":150}' --out raw/renders/oak-seed-3/still.png --camera '{"azimuth":115,"elevation":12,"fill":0.86,"targetHeight":0.5,"fov":38}'
../../../target/ci/examples/headless --preset oregon-white-oak --seed 4 --size 960x1080 --view whole --no-figure --scene '{"skyZenithRed":0.03,"skyZenithGreen":0.04,"skyZenithBlue":0.07,"skyHorizonRed":0.16,"skyHorizonGreen":0.17,"skyHorizonBlue":0.2,"groundRed":0.07,"groundGreen":0.07,"groundBlue":0.07,"sunElevation":24,"sunAzimuth":150}' --out raw/renders/oak-seed-4/still.png --camera '{"azimuth":115,"elevation":12,"fill":0.86,"targetHeight":0.5,"fov":38}'
```

## Drawn shots

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
