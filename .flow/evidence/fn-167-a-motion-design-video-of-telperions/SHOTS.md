# fn-167 shot list

Two cuts from one set of renders, at 24 fps (the headless walk's rate). Every
tree on screen is a generator render or a drawing of a generator artifact;
`RENDERS.md` records each shot's preset, seed, rows and command. Titles,
labels, the stage strip and transitions are drawn around them.

Look: one dark studio scene for every render (a dusk sky, a dark ground, a low
warm sun, no scale figure), white type in Noto Sans, parameter rows in Adwaita Mono, one accent colour.

## 16:9 master, 1920x1080, 74.6 s

Clip lengths below; each cut overlaps the next by 0.3 s, so 80 s of clips
run 74.6 s.

| # | Sequence | Shot | Seconds | Source | Requirement |
|---|---|---|---|---|---|
| 01 | Title | "TELPERION / a runtime tree generator" over Telperion, dimmed | 4 | still `telperion-hero` | R1 |
| 02 | Fidelity | Telperion, the camera drifting 30 degrees round it | 5 | walk `telperion-drift` | R5 |
| 03 | Fidelity | Telperion's braided bark at the base, slow push | 2 | still `telperion-bark` | R5 |
| 04 | Fidelity | Telperion's leaf element | 2 | still `telperion-leaf` | R5 |
| 05 | Fidelity | Date palm crown, fronds and leaf bases | 2 | still `date-palm-crown` | R5 |
| 06 | Fidelity | Silver birch, into the crown | 2 | still `birch-crown` | R5 |
| 07 | Fidelity | European beech leaf element | 1.5 | still `beech-leaf` | R5 |
| 08 | Fidelity | Oregon white oak, whole | 1.5 | still `oak-hero` | R5 |
| 09 | Algorithm | Telperion's 1,600 attractors and its scaffold drawn in the order the grower laid it down, attractors going out as axes reach them; the branches and twigs the local rules add past the crossover come after in their own colours. (The real species set `attractorWeight` to 0; Telperion is the preset that colonizes.) | 9 | `stages` dump of telperion seed 1, drawn | R6 |
| 10 | Algorithm | The bias field: Telperion with the writhe at zero, wiped to Telperion as shipped (two renders at one camera; a comparison, not a blend) | 5 | stills `telperion-no-writhe`, `telperion-writhe` | R6 |
| 11 | Pipeline | One path on Telperion, five 4 s shots: a stage strip across the top lights each stage in turn while the catalogue's row paths scroll down the left. GROW: the skeleton, 75,697 nodes. PLAN: the leaf element and the plan's counts. EXPAND: the bare wood surface. CULL: the kept leaves as points, placed against kept. DRAW: the finished tree and its GPU time. Each stage shows its measured milliseconds | 20 | `stages` dump, drawn; stills `telperion-leaf`, `telperion-bare`, `telperion-hero` | R3 |
| 12 | Continuity | A live sweep of `/surface/twistRate` on Telperion, 0 to 2.4, the camera drifting | 6 | walk `telperion-twist` | R4 |
| 13 | Continuity | Four oaks, seeds 1 to 4: the seed picks a specimen | 4 | stills `oak-seed-1..4` | R4 |
| 14 | Consumers | The oak's field from `telperion/field`, turning, in three styles: smooth voxels, blocky block-game cubes, a point cloud | 12 | field queries, drawn | R9 |
| 15 | End | "One continuous tree space. One pipeline." over Telperion | 4 | still `telperion-hero` | R1 |

A preset-to-preset morph (R4) is held out: no pair of branching presets
measured smooth (`BLENDS.md`). Its slot sits between 12 and 13 once the host
decides what to show.

## 9:16 cut, 1080x1920, 16.5 s

The same overlap: 18 s of clips run 16.5 s.

| # | Shot | Seconds | Source |
|---|---|---|---|
| V1 | Title over Telperion | 2.5 | still `telperion-hero-v` |
| V2 | Telperion, camera drifting | 3 | walk `telperion-drift-v` |
| V3 | Telperion's crown growing from its attractors | 4 | `stages` dump, drawn tall |
| V4 | The twist sweep on Telperion | 3.5 | walk `telperion-twist-v` |
| V5 | The oak's field in three styles, side by side | 3 | the master's consumer frames 109 to 180, scaled |
| V6 | End card | 2 | still `telperion-hero-v` |
