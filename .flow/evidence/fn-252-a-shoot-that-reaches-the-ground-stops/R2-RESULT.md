# fn-252 R2 as built (worker, 2026-10-14)

## The rule

The rule, as the host ruled it:
- A new per-age row, `ground_hazard` h, lies in 0 to 1 and is neutral at 0.
- While an apex's tip, as the rough layout laid it last cycle, rests on the ground, its yearly survival is its viability × (1 - h). Growth's existing draws then decide.
- **Contact:** a laid walk off the stem whose tip stands at the ground. Only the resting clamp puts it there, or the run along the ground the clamp began.
- **The final lay is unchanged.**
- **A tree with h > 0 grows its rough layout under any light.** Where no leaf shades, every bud reads whole light, so the hazard is never left dormant (Codex review, correctness 1).

Where it lives:
- **CPU:**
  - `geometry/lay.rs` (`Layer::rests`).
  - `grow/shoot.rs` (`Grower::rests`).
  - `grow/advance.rs` (the factor).
  - `grow.rs` (the rough layout grown wherever a ground hazard reads it).
  - The capture's fate thresholds (`grow/cycle/capture.rs`).
- **Device:**
  - `glue.wgsl` `inputs_main` writes each laid axis's contact into the units' inputs, a seventh column. The inputs buffers grew from 6 to 7 words an axis.
  - `arena.wgsl` adds `rests` and `R_GROUND`, word 19 of each PA's row, packed by `columns.rs` from the capture's `pa_ground_hazard`.
  - `advance.wgsl` applies the same f32 factor the CPU does.
- **Row plumbing:**
  - `species.rs`, `species/validate.rs` (a share) and `species/scale.rs` (a linear setting).
  - `chain.rs` (0 when passed).
  - Every species' literal at 0.
  - `reads_light` now counts it, since the rule reads the rough layout.

**Value.** The silver birch's is **1** on every age: a declared stand-in resting on the owner's verdict of 2026-10-14. The one contrary text, on the cultivar 'Youngii', is in `R2-PLAN.md`. Every other species' is 0.

## Proof

- **`telperion-space/tests/ground.rs`:**
  - `a_tip_on_the_ground_dies_at_one_and_runs_on_at_zero` grows the birch at 10 and 11, seed 1. At h = 0 most tips the rough layout rests on the ground live on. At h = 1 none does.
    - Red with the factor removed: 38 of 45 lived on.
    - Green with it.
  - `a_ground_hazard_acts_without_shade`: under neutral light the birch at 10 grows fewer phytomers at h = 1 than at 0.
- **`cargo test --profile ci -p telperion-space`:** green (`raw/space-tests.log`), fn-200's resting tests in `tests/sag.rs` among them.
- **h = 0 is byte-identical.** `space_hash_all.rs` grew every shipped preset at 10, 20 and 40 (seeds 1, 7) on the base (60b1201d2, its own worktree and target) and on the branch (`raw/hash-base.log`, `raw/hash-branch.log`).
  - Only the silver birch's hashes moved: all six trees.
  - The ordinary tree, oak, spruce, beech, palm, Laurelin and Telperion are the base's to the bit.
- **CPU and device agree.** `young_trees_grow_on_the_device_as_on_the_cpu` holds on the birch at 8, the test's age, and at 12, a temporary run in which contact deaths occur. `compile_size`'s shader checks are green.
- **Catalogue digests (`catalogue_identity.rs`) re-pinned.** Its doc comment records the check: with the new neutral row stripped from the text and the birch at 0, all four digests were the last pins to the bit.
- **`space_presets` green.**
- The full gate is the host's, on the merged branch.

## What changed in the birch

Seeds 1, 7 and 42. Before is R1 (`raw/birch.log`, `raw/birch80.log`); after is `raw/birch-after.log`.

| Age | Wood lying on the ground, before (m) | After (m) | Axes resting, before | After |
|--:|---|---|---|---|
| 5 | 0.02 / 0 / 0 | 0.02 / 0 / 0 | 2 / 0 / 0 | 2 / 0 / 0 |
| 10 | 9.3 / 12.1 / 5.7 | 3.2 / 3.2 / 2.2 | 71 / 80 / 43 | 38 / 39 / 29 |
| 15 | 37.4 / 46.8 / 40.2 | 13.2 / 15.2 / 11.8 | 232 / 263 / 274 | 125 / 118 / 111 |
| 20 | 85.0 / 87.5 / 80.9 | 61.1 / 49.2 / 42.9 | 485 / 441 / 433 | 357 / 290 / 259 |
| 30 | 135.2 / 115.9 / 98.4 | 119.3 / 88.0 / 64.1 | 422 / 388 / 360 | 361 / 295 / 263 |
| 40 | 22.6 / 2.1 / 22.1 | 5.6 / 2.1 / 0 | 70 / 13 / 70 | 30 / 13 / 0 |
| 80 | 0 / 8.1 / 0.5 | 0 / 8.0 / 0.5 | 0 / 99 / 5 | 0 / 98 / 5 |

- **First contact is unchanged:** age 5, 6 and 7 (`raw/birch-first-after.log`). The tip that first touches dies the next cycle.
- **The crown base is unchanged:** 0.61-0.79 m at 5 to 30. At 40: 4.62 / 4.53 / 6.05 m, against 1.74 / 4.53 / 0.72 before. At 80: 8.25 / 5.06 / 6.47 m.
  - The rule does not lift the crown. The young stem's passing branches are still there; that was cause 1, and R2a for it was withdrawn.
  - The 40-year lift in seeds 1 and 42 comes from the low passing branches' strands no longer living on the ground.

**What still rests, by sag.** `raw/rough-after.log` gives, for the axes the final lay rests, the share the rough layout holds above the ground; their contact is the settle's sag alone:
- Birch at 10: 21-34%, held 1-14 cm up.
- At 20: 82-87% (median 1.0 m up, up to 2.9 m).
- At 40: 90-100%.
- At 80: 100% (5-11 m up).

So after the rule, what lies on the ground is wood that growth's layout holds clear and the bearers' sag lowers:
- At 20: 43-61 m.
- At 30: 64-119 m.
- At 40: 0-6 m.
- At 80: 0-8 m (seed 7: 98 axes from 4 limbs at 5-10 m).

## Stills (seed 1, viewed; `raw/stills-r2/`)

- **10, whole and low.** The strands end at or just above the ground around the foot. The young tree's skirt stops at the soil. No strand runs along it.
- **20, low.**
  - The low passing branches at 0.6-1.7 m sag their curtains to the ground. Several strands still curve along the soil near the foot, fewer and shorter than before (R1's `raw/stills/birch-20-1-low.png`).
  - These are the sag-only contacts above.
- **20, whole.** The low skirt is lighter than R1's; the strands from the lowest limbs still reach the ground.
- **40, low.** The thick root-like runs along the soil in R1's still are gone. A few thin strand ends touch the ground under the crown and stop.
- **40, whole.** The crown is lifted clear of the ground. Thin strands hang to about the soil.
