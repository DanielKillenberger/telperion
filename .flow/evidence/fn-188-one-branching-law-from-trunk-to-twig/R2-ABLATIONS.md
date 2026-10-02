# fn-188 R2: ablations inside today's pipeline, 2026-10-02

## Setup

- **Branch:** `fn-188-one-branching-law-from-trunk-to-twig` at `da88cb3d`, with its own `target/`.
- **Exploratory code:** `98f99aed` and `eeaa800c`, reverted in `ff3f7a2f` and `b4c2724e`. The tree is byte-identical to `da88cb3d`.
- **The switches,** each off unless its variable is set:
  - **`FN188_WHOLE_PIPE`** applies (a) at the end of `finish`.
  - **`FN188_NO_EXTENSION`** applies (b) in `local/seed.rs`.
  - **`FN188_AXIS_FORK`** applies (c) in `scaffold.rs` and `scaffold/fork.rs`.
  - **`FN188_SHORT_EXTENSION`** is fn-182 R8's shortening, for comparison.
  - **`FN188_DRAW_NO_TWIGS`** draws the scaffold alone with its radii as solved. It is used only for the tone measure.
- **Two bases, both at seed 1:**
  - **S:** the shipped European beech.
  - **G:** fn-182's G2 overlay (`raw/families/G2.json`) with R8's shortening, i.e. the G2 + horn-fix candidate.
- **Measures:**
  - **Traced metrics:** as in fn-182 R7: scaffold thickness ratio by order, mid and end substantial divisions, len/rest, gap, tortuosity.
  - **Horn:** the leader path from the stem fork to its tip, through takeovers (fn-182 R8). On S, which has no fork, it is the trunk's path.
  - **Twig wood:** metres per metre of wood with radius ≥ 0.3 of the trunk's.
  - **Tone:** crown luminance with the scaffold drawn alone minus the full tree, native 960x720 at the pinned pose (`raw/pose.json`), over R1's crown box.
  - **Timing:** `mesh::build` (generate and mesh) cold is the first call in a process; warm is the median of the next three. RTX 3080 host, release profile.
- **Files:** the rows are in `raw/ablations.jsonl` and the per-node horn traces in `raw/horns-<name>.csv`. The tools are scratch and not committed.

## Results

| Variant | Base | scaffold o1 / o2 / o3 | mid / end | o3/o1 | len | Horn path (local) m | Tip y (env 32) | Pole m | Thick run m | Twig m per m thick | Tone | Nodes | Leaves | skel ms | mesh cold / warm ms |
|---|---|---|---|--:|--:|---|--:|--:|--:|--:|--:|--:|--:|--:|---|
| base | S | 0.57 / 0.68 / 0.69 | 0.63 / 1.00 | 0.049 | 1.15 | 31.91 (5.92) | 31.87 | 8.51 | 1.95 | 0 | 0.0224 | 213,044 | 5,828,744 | 71 | 3,326 / 3,312 |
| (a) whole pipe | S | 0.58 / 0.68 / 0.70 | 0.64 / 0.96 | 0.081 | 1.13 | 31.72 (5.74) | 31.21 | 0.25 | 1.95 | 0 | 0.0238 | 213,044 | 5,846,332 | 70 | 3,307 / 3,312 |
| (b) no extension | S | unchanged | 0.63 / 1.00 | 0.050 | 1.15 | 27.67 (1.69) | 27.27 | 0.49 | 1.95 | 0 | 0.0095 | 187,972 | 4,952,756 | 56 | 2,815 / 2,886 |
| R8 shortening | S | unchanged | 0.63 / 1.00 | 0.049 | 1.15 | 27.54 (1.56) | 27.51 | 4.14 | 1.95 | 0 | 0.0109 | 193,516 | 5,190,793 | 59 | 2,932 / 2,907 |
| (c) axis forks + (b) | S | unchanged | 0.63 / 1.00 | 0.050 | 1.15 | 27.67 (1.69) | 27.27 | 0.49 | 1.95 | 0 | 0.0095 | 187,972 | 4,952,756 | 77 | 2,842 / 2,931 |
| G2, no fix | G | 0.32 / 0.68 / 0.79 | 0.67 / 1.00 | 0.041 | 0.82 | 27.62 (13.34) | 37.13 | 17.22 | 7.91 | 0 | 0.0102 | 82,636 | 1,013,806 | 28 | 607 / 596 |
| base (G2 + R8) | G | 0.32 / 0.68 / 0.79 | 0.67 / 1.00 | 0.041 | 0.82 | 17.54 (3.26) | 27.87 | 7.14 | 4.49 | 0 | 0.0050 | 67,760 | 855,223 | 23 | 484 / 488 |
| (a) whole pipe | G | 0.64 / 0.65 / 0.72 | 0.65 / 0.88 | 0.128 | 0.82 | 17.57 (3.29) | 27.03 | 0.25 | 1.95 | 0 | 0.0056 | 67,760 | 856,528 | 31 | 474 / 501 |
| (b) no extension | G | 0.32 / 0.68 / 0.79 | 0.67 / 1.00 | 0.041 | 0.82 | 14.28 (0) | 24.90 | 3.88 | 3.25 | 0 | 0.0041 | 63,070 | 792,102 | 22 | 468 / 464 |
| (c) axis forks + (b) | G | 0.36 / 0.91 / 1.00 | 0.65 / 1.00 | 0.029 | 1.00 | 16.39 (2.10) | 25.84 | 0.51 | 3.25 | 5.82 | 0.0060 | 197,729 | 2,286,115 | 80 | 1,538 / 1,510 |

## (a) Whole-tree pipe

- **Topology and positions are frozen;** nodes are unchanged on both bases.
- **The convention:**
  - Every childless node, of any kind, demands one unit.
  - Each node's demand is the sum of its children's.
  - Radius ∝ demand^(1/e), with e = `forkExponent` (2.9 on the beech), and the root's radius is held.
  - Start radius equals radius.
  - No length taper, shares, hold or tip taper apply.
  - Leaf-bearing decisions still read the earlier pipe radii (`tree.pipe`).
  - Nothing feeds back into growth.
- **What it changes:**
  - **S:** scaffold ratios hardly move (0.57 / 0.68 / 0.69 → 0.58 / 0.68 / 0.70). End divisions move 1.00 → 0.96, and o3/o1 0.049 → 0.081.
  - **G:** G2's `lateralShare` stops acting, so order 1 rises from 0.32 to 0.64, and end divisions are 0.88.
- **The pole falls to 0.25 m on both bases.** Twig-layer daughters along the path now reach 0.3 of their node's radius, so they count as substantial divisions. The thick run on G falls from 4.49 to 1.95 m.

## (b) No terminal extension

- **The change:** a childless scaffold tip seeds no continuation. Its lateral buds are unchanged, and a tip without twig children takes the existing tip taper.
- **S:** the trunk path ends 4.2 m lower (27.27 m against 31.87 m) with 1.69 m of local wood, and the pole is 0.49 m.
  - Nodes fall 12% and leaves 15%.
  - Skeleton time is 71 → 56 ms; generate and mesh 3,326 → 2,815 ms.
- **G:** the horn is scaffold alone: 14.28 m, tip at 24.90 m.
  - The pole is 3.88 m, all scaffold; the thick run is 3.25 m.
- **Against R8's shortening:**
  - **Pole:** on S 0.49 against 4.14 m; on G 3.88 against 7.14 m.
  - **Size and time:** fewer nodes and leaves, and faster.
- **Twig tone falls with the continuation:** 0.0224 → 0.0095 on S, and 0.0050 → 0.0041 on G.

## (c) Axis-relative forks

Thick scaffold poles remained after (b) on G (3.88 m pole, 3.25 m run), so (c) was run on top of (b).

- **The smallest form:**
  - A fork off the root's axis is placed at `forkHeight` (± spread) of its own axis's length.
  - A fork's primary is placed at that share of its remaining length, measured from the fork.
  - The height-based refusal is skipped for those axes.
  - The root's own fork stays at absolute height.
- **On S** (codominance 0) it has no effect: the tree is identical to (b).
- **On G:**
  - **Leaders:** order-0 parts go from 4 to 71, because each part forks again along its own course.
  - **Thickness:** scaffold o2 / o3 rise to 0.91 / 1.00.
  - **Size and cost:** nodes rise 3.1x to 197,729, generate and mesh to 1.5 s, and skeleton time to 80 ms.
  - **Horn:** the pole is 0.51 m and the thick run is still 3.25 m.
  - **Twigs on thick wood:** 5.82 m per m, up from 0.

## Suite on the exploratory build

`cargo test --profile ci -p telperion-core --lib --no-fail-fast` at `eeaa800c`, with no variable set: 375 passed, 0 failed, 12 ignored, in 93 s. No test sets the variables.

## Stills

Bare, native 960x720, at the pinned pose (R1), under `raw/stills/`, all on base G:
- `beech-abl-base-s1-bare.png`
- `beech-abl-a-s1-bare.png`
- `beech-abl-b-s1-bare.png`
- `beech-abl-c-s1-bare.png`

G was chosen because (c) applies only there. The renders of the other variants, made for the tone measure, were deleted. Two were opened: (b) and (c).
