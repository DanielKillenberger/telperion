# fn-213 sheet 12: round 12, leaves in clusters at the short shoots (worker, 2026-10-08)

The owner's verdict on round 10 was: "structurally the birch looks almost right though the leaves look strange ... small and numerous to appear like they just fill the twigs length". Round 11 fixed leaf size and colour, and that left the crown see-through. Round 12 uses fn-223's clusters: a short shoot bears its leaves at its tip.

## The merge

fn-223 (`ce17b621`, after `026d6e30`) is merged at `6bff48b9`. The conflicts and how each was resolved:

- **Engine presets:** fn-223 carries fn-215, which made the ordinary tree an engine preset. This branch made the birch one. Both are now engine presets, so Telperion and Laurelin are the presets without a species. This touched `outline.rs`, `space_presets.rs` and `parity.test.ts`.
- **The sweep:** `sweep.rs`'s held rows were recomputed by the test. `/age` and `/skeleton/bias/supernatural/enabled` are held, and the birch's six old-bark rows are no longer held.
- **The birch species:** `birch.rs` takes the new neutral fields `flowering`, `cluster_leaves` and `cluster_spread`.
- **Pins and generated files:** the catalogue digests are re-pinned, and `presets.generated.ts` is regenerated.

**The F4 tip.** The brief said fn-223 carries fn-198's F4 tip at `1f88d724`. It does not: fn-223 lacks fn-229 (florets on the device) and the last fn-228 merge. Only fn-223 was merged, as asked. The merge did bring fn-198's grown-wood fix (`1b74db15`) and fn-228's white sun, so the trees and the light differ from round 11b's stills. Seed 1 at 40 has 282,041 nodes now against 280,885 before.

## What moved (against round 11b, `b64872a2`; commit `975ef1c8`)

| Row | 11b | 12 | Source |
|---|---|---|---|
| `/space/ages/14/clusterLeaves` (`SHORT`) | 0 (leaves along the shoot) | 3 | KU03 on *B. pendula*: "In short shoots, terminal bud-set occurs shortly after the bud-break and development of two or three leaves" (`raw/lit/kull2003.md`). SU03 on *B. maximowicziana*: "two or three leaves" |
| `/space/ages/14/clusterSpread` | 0 (dormant) | 0.6 rad | **Estimated**: no source gives the fan. It is fn-223's own test value. Leaves turn ±0.6 rad about the tip's level bearing |

- **Three, not two:** the row takes a whole number. Three is the upper of "two or three", chosen because round 11's crown read see-through. Two is the host's alternative.
- **Unchanged:** the long shoots' spacing stays at 0.06 m (`/skeleton/twigs/twig/internodeLength`), and no wood row moved.
- **Records:** MODEL-BIRCH.md rule 9 and its "Round 12" section record the change.

## Measures (`birch_measure.rs`, seed 1; raw in `raw/measure-r11b-merged.txt`, `raw/measure-r12.txt`)

Round 12 adds a direct count of clusters, read from `tree.clusters`. Both rounds are measured on the merged code.

| Measure | 11b @40 | 12 @40 | 11b @80 | 12 @80 |
|---|--:|--:|--:|--:|
| Nodes | 282,041 | the same | 1,862,185 | the same |
| Leaves | 81,161 | 116,328 | 631,030 | 820,004 |
| Clusters (short-shoot tips) | none | 21,448 | none | 122,746 |
| Leaves in clusters | none | 64,344 (55%) | none | 368,238 (45%) |
| Leaves along the shoots | 81,161 | 51,984 | 631,030 | 451,766 |
| Leaves per metre of hanging strand (nearest wood) | 22.7 | 14.0 | 22.9 | 14.7 |
| Leaves per short run (< 5 cm; nearest wood) | 0.16 | 3.27 | 0.27 | 3.31 |
| Blade p50, mm | 50 | 50 | 50 | 50 |

**Leaves rise** by 43% at 40 and 30% at 80.

**Per metre of strand falls** from 22.7 to 14.0, though the spacing row did not move. The measure gives each leaf to its nearest wood:

- In 11b, a short shoot's one leaf sat at its base on the strand and was counted there.
- In 12, a short shoot bears none along its length; its three sit at its tip and are counted on the short run.
- The strand's own leaves are about 14 a metre, a leaf a 0.06 m node at 4 to 6 a year. This explanation is consistent with the numbers but was not separately measured.

**Leaves per short run:** 3.27 and 3.31 against the row's 3. A neighbouring strand leaf now and then falls nearest a tip.

## Cost (native, `--profile ci`, limits 4; `raw/sheet12/render.log`, `raw/sheet12/r11b/render.log`, three repeats in `raw/sheet12/timing/`)

| | 11b | 12 |
|---|--:|--:|
| Leaves at 40, seeds 1 / 7 / 42 | 81,161 / 157,043 / 98,336 | 116,328 / 223,965 / 141,074 |
| Leaves at 80 | 631,030 / 693,273 / 710,171 | 820,004 / 908,595 / 924,539 |
| Dressed at 40, best of 4, ms | 257 / 503 / 214 | 207 / 435 / 224 |
| Dressed at 80, best of 4, ms | 1,348 / 1,523 / 1,563 | 1,416 / 1,642 / 1,619 |

**The timings cannot separate the rounds.** The machine was shared (load 12 to 14 of 32), and repeats of identical work scattered by up to 2×. The growth time, the same work in both rounds, scattered as much. At 80 the best dressing times are 4 to 8% apart, inside the noise.

**Leaf instances are the cost that can be stated:** 30 to 43% more for the renderer to draw. fn-223 measured the CPU side on the beech, where clustering cost nothing.

## The stills

| Sheet | Path | Rows and contents |
|---|---|---|
| Compare at 40 | `raw/compare/compare12-40.png` | Per seed (1, 7, 42): the old birch, round 11b's rows on the merged code (`raw/sheet12/r11b/`), round 12. Columns: bare, in leaf, crown, limb. Built with `compare_sheet.sh 11b 12 sheet12/r11b`; the script now takes a previous-round folder |
| Compare at 80 | `raw/compare/compare12-80.png` | As at 40 |
| Leaf close-up | `raw/compare/leaf12.png` | S5 hanging spray and S2-Oslo crown, beside 11b and 12 at 40 and 80, seed 1 (`leaf_closeup.rs`) |
| Spray and base | `raw/compare/rest12.png` | 11b beside 12, every seed and age |
| Round 12's own stills | `raw/sheet12/` | |

The worker viewed every still.

**The worker's reading.**

- **Close-ups:** at 80, the leaves now stand in threes at the short shoots' tips, small fans of triangular blades along the hanging strands, with more leaf than 11b. At 40 the strands are still lightly leaved: clusters read as pairs and triples, spaced apart. Beside S5's spray, the leaves are still sparser and smaller in the frame, and the colour is unchanged pale sage.
- **Crown and in-leaf views:** a little fuller and darker inside than 11b at both ages, with more shadow on the ground under the tree. Seed 42 at 80 is the clearest case. The crown still reads see-through at distance at both ages, much lighter than S2-Oslo's leaf mass and the old birch's green.
- **Limb and bare views:** unchanged, as expected.
- **Overall:** round 12 moves toward the reference, but it does not close the gap.

## For the host and the owner

- **Leaf mass.**
  - **Where the leaves sit:** clustering moved about half the leaves to short-shoot tips, and the count rose by a third. The crown is still see-through.
  - **What the rows allow:** the row's literature range stops at three, so more leaf mass from clusters alone would need four or more, past both sources. More short shoots per strand (`BRANCH` lateral `SHORT` 0.5) is a wood row and was not touched this round.
  - **Who decides:** which lever, if any, is the host's call.
- **Two or three.** Three was taken. The row is whole, so a mix of two and three per shoot cannot be stated. Whether the engine should draw a per-shoot count is a design question for the host.
- **The spread** of 0.6 rad is estimated, and no source states it.

## Gate (round 12)

- **Render half first, under the GPU lock:** `cargo test --profile ci -p telperion-render --no-fail-fast` exits 0, 193 passed in 43 binaries (`raw/gate12-render.log`).
- **Then the workspace:** `cargo test --profile ci --workspace --no-fail-fast --exclude telperion-render` exits 0, 1,038 passed in 92 binaries (`raw/gate12-core.log`).
- **Limits:** RAYON_NUM_THREADS=4, CARGO_BUILD_JOBS=4, RUST_TEST_THREADS=4.
- **Generated files:** `npm run wasm:build` regenerated `presets.generated.ts` at the merge and again for round 12's rows.
- **npm test:** 1 of 143 failed (`raw/npm12.log`). The failure was `field.test.ts`'s named refusal: fn-215 had named the birch as the preset with no engine species, and the merge left it so. It now names Telperion, and the file reruns 5 of 5 passing. This is a merge fix, not a round-12 change.
- **Not run:** Astra, as the brief asked; the owner judges.
