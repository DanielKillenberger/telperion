# fn-197 step 3d: sizes against the tree's own mean vigour, 2026-10-05

Host decisions 14 to 16 (DESIGN-OPTIONS.md, section 11) are built, and the spec's Design section is amended with decisions 11 to 16.

## What changed

- **The size (`allocation::sizes`):** a unit's size is (r / r̄)^ψ, with r = vigour / presence, normalised exactly: size_i = r_i^ψ / (Σ w r^ψ / Σ w).
  - The presence-weighted mean size is exactly 1 at every ψ.
  - Buds of no presence are whole and weigh nothing in the mean.
  - The mean depends on every bud, as one smooth function.
  - The fully lit reference pass of step 3c is gone.
- **Proved** (`allocation/tests.rs`):
  - `sizes_keep_the_mean_whole_and_favour_light`: the weighted mean is 1 to 1e-12 at ψ 0, 0.5, 1, 2 and 4; every bud is whole to the bit at ψ 0; and a lit bud (light 1.1) outgrows a shaded one (0.2).
  - `sizes_move_by_degree_as_a_bud_vanishes`: one bud's presence falling to nothing in 1,000 steps moves no other bud's size by more than 0.01 a step.
- **The walks:**
  - At λ walked from 0.2 to 0.8, the limbs' apical control failed the bound at 0.746 (slope 51.1). Far from 0.5, apical control's bias compounds over an axis's branching points, so vigour per presence spreads over orders of magnitude, and so do sizes against the mean.
  - λ is now walked over Pałubicki's range of forms, 0.46 to 0.54 (2009, Fig. 7), with a margin: 0.4 to 0.6. There every light walk passes: steepest λ limbs 18.8, leaf area 17.5, extinction 10.5, sky 10.3.
  - Neither the bound nor the window is changed. λ's validation still allows 0 to 1. Whether to narrow it is the host's call.

## Renders (oak, 80 years, sky 0.5, φ 1, ψ 1, λ 0.45, full lay on, under the GPU lock; I viewed every still)

`raw/step3d/seeds-bare.png` and `seeds-whole.png` hold the references, round 4, step 3c and step 3d for seeds 1, 7, 2, 3 and 4.

| Seed | Round 4: height, width (m), leaves | Step 3d: height, width (m), leaves, wood |
|--:|---|---|
| 1 | 21.9, 23.5 × 18.6, 2.16M | 13.2, 19.1 × 17.0, 0.79M, 6.7 km |
| 7 | 20.1, 19.9 × 21.5, 2.22M | 14.0, 17.5 × 15.6, 0.90M, 7.5 km |
| 2 | 20.7, 19.7 × 25.2, 3.51M | 14.4, 17.8 × 25.6, 1.30M, 10.7 km |
| 3 | 20.8, 22.0 × 22.1, 3.09M | 14.1, 19.6 × 15.9, 1.10M, 9.1 km |
| 4 | 19.0, 24.8 × 21.8, 0.87M | 14.3, 15.9 × 16.3, 0.18M, 5.4 km |

### Reading

- **The crown closes into a dome on four of five seeds.** This is the first time, in any round.
  - In leaf, seeds 1, 7, 2 and 3 read as broad, rounded, closed domes on a short trunk, as S2 does. Seed 7 is the clearest.
  - Seed 4 leans, lopsided on one heavy limb, and is sparse.
- **Bare:** a dense web under a rounded outline, on limbs that divide low into several heavy arms. It is nearer S1's massive trunk dividing into a few limbs under a broad dome than any round before, though the trunk is short and the outline still has a few long reaches (seeds 1 and 3).
- **Much fuller than step 3c.** Step 3c thinned into open tiers; step 3d fills.
- **New regression: the trees are 30 to 35% shorter.** They stand 13.2 to 14.4 m against round 4's 19 to 22 m, and 16 to 20 m wide (seed 2 25.6 m). Leaves are 25 to 40% of round 4's, and 20% at seed 4.
  - Likely cause, not checked:
    - at λ 0.45 the continuing axis is disfavoured at every branching point, so the trunk's and limbs' leaders get less vigour per presence than the mean, while the laterals, in the majority, get more;
    - the leader's units shrink, and the tree stays low and spreads.
  - The camera fits each tree, so the sheet hides the height. The scale post beside each tree shows it.

## Next

The host's rule: "if the dome closes on most seeds, go on to step 4." It closes on four of five, so step 4 (the carbon balance and shedding, hazard form) follows. The height loss is reported here for the host and is not addressed by step 4.
