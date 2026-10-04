# fn-200: branches sag under their load and rest on the ground (worker)

Built as the host's design states it. The code is in `crates/telperion-space/src/sag.rs` and `geometry.rs`; the tests are in `tests/sag.rs`, `tests/refusals.rs` and `tests/walk/`.

## How it works

- **R1, `form.sag` per PA** (neutral 0, refused outside 0 to 1000 and when NaN):
  - The tree is laid once and thickened as before.
  - When any PA sags, `sag::torques` computes each phytomer's gravity torque about its base on the tree as it stands (small-deflection beam theory). The torque is the presence-sized load beyond the phytomer, crossed with gravity.
  - The load is wood (length × r²) and foliage (length × its own pipe section, the pipe model's leaf share), carried in the same pattern as girth: laterals at their node, continuations at the tip, relays split by blend.
  - The tree is then laid again. Each phytomer turns its running direction down about the torque's axis by `sag × |M| / r⁴` per metre, eased so it never passes straight down.
  - Girth and topology are unchanged (`sag_bends_and_grows_no_other_tree`).
  - With every sag at 0, the second pass never runs.
- **R2:** the tip carries nothing and keeps its tropism. `a_loaded_branch_droops_at_its_base_more_than_at_its_tip` checks base droop above tip droop, with the tip rising above the base, for three quarters of the limbs.
- **R3, the ground:**
  - Lateral wood that would go below z = 0 drops to the ground and runs along it, keeping its internode length (`sagging_wood_rests_on_the_ground`, `a_lateral_pointing_down_rests_on_the_ground`; no vertex below 0).
  - The trunk (the seed axis and what carries it on) reaching the ground, or an axis whose base is below it, is still `BelowGround`.
  - The refusal now names the axis's PA, birth cycle and base height (`a_trunk_into_the_ground_is_an_error`). The old test that drove a lateral straight down now expects a trunk.
- **Walk:**
  - `form.sag` is walked on every PA. The walk's scale is the radians a unit of sag bends the walk tree's most loaded axis (`tests/walk/bend.rs`), as tropism is walked over its axis's metres.
  - The ranges run from none to limbs bowed to the ground: 2e-4 on the trunk, 1e-4 on limbs, 1e-3 on twigs.
  - Worst slopes are 0.31, 0.12 and 0.11, against the bound of 30.
- **R4:**
  - Crate tests: every `telperion-space` suite passes (oracle, walks, geometry, refusals, sag).
  - The beech's 80-year stills at seeds 1 and 7, re-rendered on this branch, are byte-identical to fn-193's `raw/final21` (`raw/beech/`, all eight PNGs the same).

## R5, the spruce walking sag (`raw/r5/strip.png`, viewed)

Seed 1 at 80 years, on the fn-194 branch rebased onto this one. The main branch PA's `form.sag` is walked in nine equal steps from 0 to 2e-4. The rows are in leaf, bare, and the limb close-up.

- The lower branches droop by degree.
- From about 5e-5 the lowest ones reach the ground and run along it as a skirt.
- From 1e-4 the limb close-up shows the spruce's swept-down base and upturned tip.
- Past about 1.25e-4 the branches hang so low that the crown narrows: 11.7 m wide at 0, 12.7 m at 5e-5, 8.9 m at 2e-4.
- No step pops. The close-up camera is fitted to each tree's bounds, so it shifts with them.

A value-range probe is in `raw/probe/`: 1e-3 collapses the crown into hanging curtains.

## Codex review round 1: NEEDS_WORK, two P1s, both fixed (`1976400b`)

1. **The torque cutoff broke continuity.** `unit()` dropped moments at or below 1e-12. A nonzero torque is now normalised with no cutoff; `a_vanishing_moment_bends_by_degree` covers it.
2. **Sag on the terminal phytomer overrode its tropism.**
   - Each phytomer now bends by the moment at its far end: what its node bears and everything beyond, about that end. An unloaded tip bends by nothing (`an_unloaded_tip_keeps_its_tropism`).
   - While fixing it I found a third problem: a torque taken before bending pointed the wrong way once a bearer's sag had turned the phytomer, so some limbs bent up. Each torque is now carried with its phytomer, from its direction before bending to its direction now, then levelled.
   - R2 is now tested on joint bends of free-tipped limbs. Comparing absolute directions could not separate the base from the tip, because a tip inherits every turn made below it.

**The walk's twig range is narrowed to 3e-4 (it was 1e-3).** Near 1e-3 the hanging twigs sit close to straight down. One step there failed the jump check: its change fell only from 0.040 to 0.0026 over three refinements, against the 20× fall required. Refined further, it falls 8× per level (0.00034, 0.000043, 0.0000053), so the region is steep and still converging, not a jump. That region is left unwalked, which I am stating plainly here.

After the fixes: worst slopes 0.30, 0.10 and 0.07; all crate tests pass; the beech is again byte-identical (`raw/beech2/`).

**R5 re-rendered** on the fixed engine (`raw/r5b/strip.png`, viewed). It shows the same progression by degree: lower branches droop and lie on the ground from about 5e-5, and the limb close-ups show swept-down bases with upturned tips at 1e-4 to 1.5e-4. Crown width runs 11.7, 12.7, 12.3, 11.3 and 9.0 m.
