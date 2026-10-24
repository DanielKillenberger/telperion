# fn-188 R3: the competitive-growth probe, 2026-10-02

## What was built

- **Code:** a scratch example, `crates/telperion-render/examples/selforg_probe/` (5 files, 850 lines). It was committed as `1d5390ee` and reverted in `ae8640ec`, so the tree is byte-identical to `d766f430`. It is not wired into the pipeline.
- **Model:** Pałubicki et al. 2009, sections 4.1 to 4.5, read from the paper.
  - **Buds:** each metamer carries one axillary bud at the golden angle (137.5°), 40° off its axis. Each shoot tip carries a terminal bud.
  - **Light:** the shadow-propagation grid of 4.1. A bud casts a·b^−q into the (2q+1)² voxels of layer q below it, for q = 0..6. Light is Q = max(C − s + a, 0) with C = 1, and Q = 0 outside the beech's envelope.
  - **Allocation:** the extended Borchert-Honda model (4.2). Light is summed basipetally, then v_base = α·Q_base flows acropetally, split at each node as λQm : (1−λ)Ql.
  - **Extension (4.3):** a bud given v grows ⌊v⌋ metamers of length unit·v/⌊v⌋. Each metamer turns towards light (ξ, the negative shadow gradient) and up (η). A bud with v < 1 stays dormant.
  - **Shedding (4.4, Takenaka):** a lateral branch is shed when its light per internode falls below a threshold. Its pipe demand is kept as memory on the bearing node (4.5).
  - **Radii:** one pipe pass over the whole tree at the end. Each tip carries one unit of demand, r ∝ demand^(1/2.9) with `forkExponent` 2.9, and the root radius is held at 0.512 m. These are R2 (a)'s conventions.
- **Envelope and size:** the shipped beech's envelope: height 32 m, crown base 0.06 (1.92 m), spread 0.36, fullness 0.3, shoulder 1.8. The lobes are off.
- **Two additions outside the paper's text:**
  - A given bole of metamers without buds to 1.92 m, because the envelope holds no space below its crown base.
  - Extension stops at the envelope wall: a metamer that would leave it is not grown, and its bud stays. Without this, about 90% of the wood grown was outside the envelope and was then shed (see calibration).
- **Fixed for every row:** α 2, unit 0.25 m, voxel 0.5 m, a 0.03, b 1.8, qmax 6, ξ 0.3, η 0.15, shed threshold 0.01, 80 cycles, seed-free (the probe is deterministic, and reruns repeat to the digit).
- **Neutral allocation** is λ = 0.5, the paper's "not biased". The rows run λ 0.45, 0.48, 0.50, 0.52 and 0.55.

## Calibration (λ 0.5, growth only)

Each setting was run once to reach a tree that fills the envelope. The values above were then frozen.

| Setting | Retained / generated nodes | Note |
|---|---|---|
| a 0.2, α 2, 40 cycles | 1,175 / 3,414 | about 6 nodes per voxel saturate a voxel; the tree stays small |
| a 0.2, 80 cycles, α 2 / 3 / 4, no shedding | 6.5k / 20.8k / 12.9k | reaches 32 m |
| a 0.05, no wall stop, shed 0.01 / 0.001 | 13.8k / 13.0k of 160k / 150k | shedding removes the wood grown outside the envelope |
| a 0.05, wall stop, shed 0 / 0.01 | 105.7k / 16.1k of 121k | |
| a 0.03, wall stop, shed 0 / 0.01 | 190.2k / 35.7k of 257k | chosen |
| a 0.02, no wall stop, shed 0 | 323.6k, 1,039 ms | |

**Under shedding, retained wood is 15 to 38% of the wood grown** at every setting. Branches shaded to Q ≈ 0 die whatever the threshold: 0.01 and 0.05 retained the same.

## Results

The traced measures are R2's: the six thickest first-order systems, traced axis by axis. Notes on the columns:

- **No crossover.** Every node is wood, so the systems run to the tips. R2 traced scaffold only.
- **o1 / o2 / o3:** median birth girth ÷ parent, over laterals whose axis is 1 m or more. R2 counted scaffold laterals.
- **mid / end, len, gap, angle:** over substantial divisions (daughter ≥ 0.3 of the parent). Subst / div counts them.
- **Trunk top:** the height at which the trunk's own axis ends.
- **o1 / strong:** first-order axes off the trunk axis; strong ones are born at ≥ 0.4 of the trunk at the junction.
- **Twig m/m:** the length of the non-thick laterals born on thick wood (≥ 0.3 of the root radius), and their subtrees, per metre of thick wood.
- **Tone:** crown luminance with the fine wood removed minus the full tree. Fine wood is wood that carries 8 tips or fewer. It is drawn bare, native 960x720, at R1's pinned pose and over R1's crown box.
- **Time:** the whole growth, cold (the first run in the process), then the warm median and p95 of 7 runs.

| λ | o1 / o2 / o3 | mid / end | subst / div | o3/o1 | len | gap m | tort | angle | trunk top m | o1 / strong | twig m/m | tone | retained / generated | tips | tip r cm | ms cold / med / p95 |
|---|---|---|--:|--:|--:|--:|--:|---|--:|---|--:|--:|---|--:|--:|---|
| 0.45 | 1.00 / 1.00 / 1.00 | 0.87 / 1.00 | 23,153 / 23,175 | 0.77 | 1.29 | 0.31 | 1.009 | 31±7.8 | 3.8 | 2 / 2 | 70 | 0.0101 | 52,484 / 179,973 | 15,994 | 1.1 | 340 / 331 / 334 |
| 0.48 | 0.95 / 1.00 / 1.00 | 0.85 / 1.00 | 16,168 / 16,186 | 0.81 | 1.08 | 0.33 | 1.008 | 30±6.7 | 5.8 | 3 / 3 | 59 | 0.0108 | 46,512 / 242,337 | 13,170 | 0.9 | 327 / 315 / 323 |
| **0.50 neutral** | 0.58 / 0.86 / 0.79 | 0.79 / 1.00 | 8,033 / 8,051 | 0.30 | 1.00 | 0.31 | 1.010 | 30±6.3 | 31.8 | 40 / 34 | 84 | 0.0086 | 35,669 / 257,490 | 10,074 | 0.9 | 270 / 260 / 263 |
| 0.52 | 0.54 / 0.72 / 0.74 | 0.79 / 1.00 | 7,252 / 7,303 | 0.17 | 0.92 | 0.34 | 1.011 | 30±6.2 | 31.9 | 42 / 33 | 80 | 0.0072 | 37,776 / 150,563 | 10,524 | 1.1 | 233 / 219 / 221 |
| 0.55 | 0.47 / 0.53 / 0.61 | 0.69 / 1.00 | 3,737 / 3,867 | 0.14 | 0.81 | 0.40 | 1.015 | 30±6.1 | 32.0 | 50 / 38 | 63 | 0.0075 | 38,487 / 101,915 | 9,295 | 1.3 | 239 / 225 / 226 |

**The references:**

- **R2, shipped beech (S):** o1 / o2 / o3 0.57 / 0.68 / 0.69, mid / end 0.63 / 1.00, o3/o1 0.049, len 1.15, twig m/m 0, tone 0.0224 (twig layer removed), 213,044 nodes.
- **fn-182's G2 + R8 candidate:** o1 / o2 / o3 0.32 / 0.68 / 0.79, mid / end 0.67 / 1.00, 67,760 nodes, skeleton 23 ms (R2), 22 to 26 ms (R8).
- **B-BARE (fn-182 R1, qualitative):** trunk clear to about 1/5 of the height, dissolving at 30 to 40% into 4 to 6 limbs at 0.4 to 0.6 of the trunk, with a dense fine periphery.

**Without the memory of shed wood,** the pipe gives these differences:

- Tip radius rises to 1.8 to 2.2 cm.
- o1 falls by 0.01 to 0.07.
- The strong-limb count at λ ≥ 0.5 falls to 29 to 35.
- o3/o1 changes by −0.06 to +0.07.

All rows of both pipes are in `raw/probe.jsonl`.

## Runtime

**The machine:** AMD Ryzen 9 5950X, release profile, the same binary.

- **The shipped beech:** `pipeline::build` with the default request, `skeleton_ms`, measured fresh here: cold 67 ms, warm median 53 ms, p95 55 to 56 ms (7 runs, two processes).
- **The fn-182 candidate** was not rerun here: its R8 shortening switch is reverted, so the 23 ms is R2's measurement.
- **The probe is single-threaded.** At λ 0.5, the warm median of 260 ms divides by stage (ms over 80 cycles):

| Stage | ms |
|---|--:|
| extend (incl. shadow casting) | 116.8 |
| shed (incl. shadow removal, compaction) | 72.0 |
| light | 59.9 |
| basipetal | 8.9 |
| allocate | 8.2 |
| pipe | 0.7 |

- **Cumulative node visits:** 9.9M to 13.5M across the rows, against 35k to 52k retained nodes.
- **Shadow voxel updates:** 75M to 218M.
- **Generated against retained:** 2.6x to 7.2x.
- **Peak RSS:** 19 to 22 MB for one growth. The shipped binary's process was 55 MB for 8 builds.

**Cost against the shipped beech:** the probe is 4 to 6x slower than the shipped skeleton and 10 to 14x slower than the fn-182 candidate. It retains 17 to 25% of the shipped node count, and its finest wood is 0.9 to 1.3 cm in radius, at metamers 0.25 m long.

## What the structure showed

Read from the stills (two viewed) and the table:

- **λ 0.48 (bare):**
  - A short trunk, about 2 to 3 m in the still and 5.8 m on the axis trace, parts into a few thick limbs.
  - The limbs keep dividing into progressively finer, wavy branches out to a dense fine periphery that fills the envelope.
  - Thick wood carries fine wood along its length (59 m per m).
- **λ 0.5, neutral (bare):**
  - A trunk runs to the top (31.8 m), with 40 first-order axes and 34 strong ones.
  - Above about 3 m, many long, near-parallel upright branches of similar girth form a broom. Their subdivision is less graded.
- **Across λ:**
  - λ < 0.5 dissolves the trunk low (3.8 and 5.8 m), into 2 and 3 limbs.
  - λ ≥ 0.5 keeps an excurrent trunk with 33 to 38 strong limbs.
  - No row gives B-BARE's 4 to 6 limbs out of a trunk clear to about a fifth of the height. λ 0.48 is the nearest by count (3).
- **Thickness:**
  - End divisions are 1.00 in every row, as in R2: an axis that ends hands its whole pipe to its last lateral.
  - At exponent 2.9, a lateral that carries about 3% of its parent's tips is already "substantial", so 97 to 99.9% of the traced divisions count. The 0.3 threshold separates almost nothing here, and gap (0.31 to 0.40 m) is one to two metamers.
  - o3/o1 falls from 0.77 to 0.14 as λ rises.
- **The outline:**
  - The crown's outline is the envelope's: the wall stop leaves a smooth oval edge.
  - The top cap above the horizon reads as a lighter dome of tips.
- **Detail:** the fine wood is coarser than the shipped twig layer: tips 0.9 to 1.3 cm in radius, 0.25 m metamers, and 9k to 16k tips.
- **Tone:** fine wood adds 0.007 to 0.011 of crown tone, against the shipped twig layer's 0.0224, measured with a different cut.

**Not shown or not tested:**

- A tree within the shipped node count at comparable fine detail: a 0.02 tried 324k nodes at 1,039 ms, without the wall stop.
- The priority model of 4.2, and any apical control that changes over time.
- Lobed envelopes, codominance as a recorded fact, and leaves beyond the simple whole still (`beech-probe-0.48-s1-whole.png`, not viewed).

## What of today's pipeline it would stand in for

These map the probe's stages onto today's code; they are not a judgment of fit.

**Its single growth law covers:**

- **The scaffold's station rules:** one lateral per deeper station, lengths from the parent axis, and forks at absolute heights (`scaffold.rs`, `scaffold/fork.rs`).
- **The twig layer:** `local/seed.rs`, `local/advance.rs` and `twigs.rs`, with its `limbRadius` gate and terminal hand-off.
- **The crossover itself.**
- **Shedding by shell depth.**

**Its whole-tree pipe pass covers the radius solve's split and its patches:** `girthHold`, `girthFall` and `lateralShare`.

**It still needs from the pipeline:**

- The envelope.
- The surface mesh (the probe drew bare tubes).
- Leaf placement and short shoots.
- The materials.

**It has no counterpart for** today's node-budget search: its size is set by a, α and the cycle count.

## Files

- `raw/probe.jsonl`: every row, with both pipes, stage times, visits and parameters. Rows 6 and 7 repeat λ 0.48 and 0.5 for the stills.
- `raw/stills/`:
  - `beech-probe-0.48-s1-bare.png` (viewed)
  - `beech-probe-neutral-s1-bare.png` (viewed)
  - `beech-probe-0.48-s1-whole.png`
- **λ 0.48 was picked for the stills** by one stated criterion: strong-limb count nearest B-BARE's 4 to 6.
