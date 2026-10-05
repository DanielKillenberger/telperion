# fn-197 step 4c: sizes normalised per physiological age, 2026-10-05

Host decisions 23 to 25 (DESIGN-OPTIONS.md, section 13).

## What changed

- **Per-PA normalisation (`allocation::sizes`):** a unit's size is d_i · Σw_g / Σ(w·d)_g, with d = (vigour / presence)^ψ, g the bud's PA and w its presence.
  - Each PA's presence-weighted mean size is 1.
  - A PA with one growing bud is whole to the bit: its numerator and denominator are the same product.
- **Every λ is 0.45,** the leader's too.
- **Proved** (`allocation/tests.rs`):
  - `sizes_keep_each_orders_mean_whole_and_favour_light`: each group's weighted mean is 1 to 1e-12 at ψ 0, 0.5, 1, 2 and 4, in a tree with four groups, two of them single buds. The single buds are exactly 1 at every ψ, every bud is exactly 1 at ψ 0, and the lit bud of a group outgrows the shaded one.
  - `sizes_move_by_degree_as_a_bud_vanishes`: one bud's presence falling to nothing in 1,000 steps moves no other size by more than 0.01 a step.

## Walks (decision 24): green

- **Light walks:** every slope is under 30.
  - The limbs' λ is now 16.8 (it was 52.6), the trunk's 11.1 and the twigs' 1.0. The compounding is gone, as the host expected.
  - The steepest of the rest: twigs' upkeep 26.4, leaf area 22.0, sky 15.2.
- **The sampled in-leaf walks (sag, limb survival, sleeping probability):** green.
- **`cargo test --profile ci -p telperion-space`:** green, 148 s, one test ignored (the slow suite).

## Renders (oak, 80 years, sky 0.5, φ 1, ψ 1, λ 0.45, upkeep 0.35, κ 2, τ 0, persistence 5 years, full lay on; under the GPU lock; I viewed every still)

`raw/step4c/seeds-bare.png` and `seeds-whole.png` hold the references, round 4, step 3d and step 4c.

| Seed | Height (m) | Width (m) | Leaves | Fine wood (km) | Leaves a metre of fine wood | Round 4: height, leaves, leaves a metre |
|--:|--:|---|--:|--:|--:|---|
| 1 | 21.7 | 31.3 × 26.4 | 1.28M | 9.33 | 137 | 21.9, 2.16M, 135 |
| 7 | 20.8 | 24.8 × 26.0 | 1.35M | 9.80 | 137 | 20.1, 2.22M, 136 |
| 2 | 21.3 | 21.0 × 30.6 | 1.97M | 14.44 | 136 | 20.7, 3.51M, 134 |
| 3 | 19.5 | 26.4 × 24.8 | 1.55M | 11.26 | 138 | 20.8, 3.09M, 137 |
| 4 | 18.8 | 26.6 × 24.5 | 0.27M | 6.90 | **39** | 19.0, 0.87M, **83** |

### Reading

- **3d's domes at round 4's height.**
  - In leaf, seeds 1, 7, 2 and 3 are broad, rounded, closed crowns 19.5 to 21.7 m tall: as tall as round 4 and wider (21 to 31 m), the nearest yet to S2.
  - Bare, the outline is a broad dome over a fine web, the limbs dividing low and spreading.
  - The web is less dense than at 3d, because the crown is spread wider.
  - Seed 4 is a sparse, spreading crown on a few long limbs.
- **The bole:** a short trunk before the limbs divide, as in S1. No long clear bole forms, and the crown starts low.

### R2 (amended, decision 25): not met in leaf count; the cause is seed 4's leaf marking, not its wood

- **In leaf count light widens the spread:** 7.3× (0.27M to 1.97M), against round 4's 4.0× (0.87M to 3.51M).
- **Seed 4 is the whole difference.**
  - Without it, step 4c spreads 1.54× (1.28M to 1.97M), against round 4's 1.63× (2.16M to 3.51M).
  - In fine wood, step 4c spreads 2.1× (6.90 to 14.44 km), against round 4's 2.5× (10.5 to 26.2 km, from its leaves and leaves a metre). Light narrows the wood spread.
- **What differs about seed 4: its leaves per metre of fine wood.**
  - Every other seed carries 136 to 138 leaves a metre of fine wood, in round 4 and here. Seed 4 carried 83 in round 4 and carries 39 here.
  - Its wood is ordinary: 6.9 km of fine wood, 765k nodes, a long low limb.
  - The leaves are placed by the pipeline's leaf-bearing marking. Round 4 named seed 4's sparse foliage a phase F item, and that marking is outside the engine. Light halves seed 4's leaves a metre further. The cause is not checked: likely a narrower share of its fine wood counts as leaf-bearing once light reshapes its twigs.

## Not closed

The walks hold. R2, read as written (leaf count), does not: 7.3× against 4.0×. The host decides whether seed 4's leaf marking is R2's concern or phase F's. Step 5 was not started.
