# fn-213 sheet 14: round 14, the short shoot's pipe (worker, 2026-10-08)

The host's decision on round 13's red bark gate was to change only the short shoot's `pipe` row. The material rows, the shares and the engine stay as they were. The reason: a short shoot adds a 0.7 mm pipe at every phytomer it lays, but bears its leaves only at its tip. That charges the wood below for leafless phytomers, while a real brachyblast stays a millimetre or two thick.

## What moved

| Row | 13 | 14 |
|---|--:|--:|
| `SHORT` `form.pipe`, `/space/ages/14/form/pipe` | 0.0007 | 0.0005 |

Nothing else moved. Round 13's 0.85 shares, three leaves a cluster, and every material and form row are unchanged. The catalogue digests are re-pinned and `presets.generated.ts` is regenerated.

## DBH (`birch_dbh.rs`; raw in `raw/measure-r14.txt`)

**The target.** No source read gives an open-grown birch's DBH at a stated age, so the target is round 12's stem. MODEL-BIRCH.md, "Round 14", sets out how the sources bracket it:

- **LO96:** stand-grown top diameters are 27.3 and 37.7 cm at 40 and 80 (class II).
- **FVA:** released crop trees grow 4 to 5 mm of radius a year, which gives 32 to 40 cm at 40.

| DBH, cm, seeds 1 / 7 / 42 | At 40 | At 80 |
|---|---|---|
| Round 12 (target) | 31.7 / 34.1 / 31.3 | 61.2 / 61.9 / 63.7 |
| Round 13 (pipe 0.0007) | 36.5 / 39.4 / 36.1 | 69.5 / 70.1 / 72.3 |
| Probe, pipe 0.0003 | 26.1 / 28.0 / 25.9 | 52.3 / 53.0 / 54.4 |
| **Round 14 (pipe 0.0005)** | **30.7 / 33.0 / 30.3** | **59.7 / 60.3 / 62.1** |

- **DBH:** round 14 sits 1.0 to 1.6 cm under round 12, within 3%. At 40 it is inside FVA's 32 to 40 cm band for seed 7 and just under it for seeds 1 and 42. At both ages it is above LO96's stand-grown top diameter.
- **Wood over 30 mm of radius, seed 1:** 35.1 m at 40 and 170.7 m at 80. Round 12 had 39.1 and 176.1 m; round 13 had 56.6 and 303.3 m.

**A short shoot's own base radius** is measured on runs under 0.05 m, at the base of their first edge:

| Pipe | p50, mm | p90, mm |
|---|---|---|
| 0.0007 (round 13) | 0.89 to 1.33 | 1.82 to 2.10 |
| 0.0005 (round 14) | 0.66 to 0.95 | 1.31 to 1.50 |

The p50 is inside the host's 0.5 to 1 mm at every seed and age.

**Clusters and cost.**

| Seed 1 | Clusters @40 | Clusters @80 | Nodes @80 |
|---|--:|--:|--:|
| Round 13 | 35,820 | 207,038 | 2,567,034 |
| Round 14 | 35,542 | 203,283 | 2,547,246 |

- **Clusters:** across the seeds they fall 1 to 5% against round 13. Seed 7 at 40 falls most, from 72,042 to 68,481. I did not isolate the cause.
- **Nodes:** the largest tree is 2.93 M at 80 (seed 7), under the 4 M cap.
- **Leaves at 80:** 1.07 to 1.20 M.

## The stills (`raw/sheet14/`, sheets in `raw/compare/`)

I made four sheets and viewed all four:

| Sheet | Contents |
|---|---|
| `compare14-40.png` | Per seed: old birch, round 13, round 14 (bare, in leaf, crown, limb) |
| `compare14-80.png` | As at 40 |
| `base14.png` | The base view, rounds 12, 13 and 14, every seed and age |
| `hero14.png` | S2-Oslo, the old birch, round 14 at 40 and 80 (seed 1) |

**My reading.**

- **Base at 40:** the stem is white above the flare at all three seeds. Round 13's black band at the flare is gone, and the views read as round 12's.
- **Base at 80:** the bark is black only on the flare and the lowest part of the stem, about a metre, with white above. That matches round 12. Round 13's fully black stems are gone; seed 42's black is the most extensive, as it was in round 12.
- **Bare views:** seed 42's trunk at 80 reads white-grey again rather than round 13's dark.
- **Crown:** the in-leaf, crown and limb views look the same as round 13's, with the same density. The crown is still see-through and grey-olive beside S2-Oslo and the old birch's green. Colour is left to the host, as the decision said.

## Gate (round 14)

- **Render half first, under the GPU lock:** `cargo test --profile ci -p telperion-render --no-fail-fast` exits 0, with 193 passed in 43 binaries (`raw/gate14-render.log`). `smooth_bark` passes again.
- **Then the workspace:** `cargo test --profile ci --workspace --no-fail-fast --exclude telperion-render` exits 0, with 1,038 passed in 92 binaries (`raw/gate14-core.log`).
- **Then npm test:** 143 of 143 pass (`raw/npm14.log`).
- **Limits:** RAYON_NUM_THREADS=4, CARGO_BUILD_JOBS=4, RUST_TEST_THREADS=4.
