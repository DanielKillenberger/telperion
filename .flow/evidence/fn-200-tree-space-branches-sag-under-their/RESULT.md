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

## Follow-up, from the spruce's round 3 (host, 2026-10-04)

All the engine work the spruce needed is on this branch; fn-194 carries only values and stills.

### Host decision 1: wood lands on the ground as a curve

**Landing** (`583a391b`):
- Within a contact zone of four radii plus six internodes above the ground, wood's descent eases with the square root of its height. It lands tangent and runs along the ground, reaching it in a finite run.
- The rest is kept as a backstop: a step that would go below the ground stays on it, clamped in height only.
- The six internodes spread the landing turn over steps; a zone of radii alone is thinner than one internode on coarse wood.

**Sag** (`dd5cc47d`):
- Girth is now computed before placement, so the zone knows each radius on the first lay. Relay nodes are set with the scales.
- Sag turns each phytomer about its gravity torque. The torque, taken before bending, is carried by the whole-frame rotation from the phytomer's first-lay frame (heading and side) to its frame now, then levelled.
- The turn is limited to the room left to straight down along that rotation, less 0.15 rad. Wood past straight down is never lifted, and a vertical trunk bends towards its load.

### Host decision 2: tropism weakens to nothing at straight down (`1f491cfb`)

**The design asked for:** within a small cone about straight down, take the turning side from the axis's own carried frame, blended continuously into the direction's lean.

**Why that cannot be continuous:**
- The lean is a direction in the horizontal plane. Inside the cone it can point anywhere, its length shrinking to zero at straight down.
- A continuous side defined over the whole cone would have to match the lean's direction on the cone's rim and stay defined inside. No such field exists (the hairy-ball limit).
- So any blend of the lean with one carried side cancels at some lean: wherever the lean points opposite the side at the right length. It did, at half the cone, in the landing walk.

**What was built:** within 0.05 rad (about 3°) of straight down, tropism's share scales with the lean over sin(0.05), down to nothing at straight down. This is gravitropism's sine law.
- Outside the cone the old code path is unchanged.
- Sag stops 0.15 rad short of straight down, so sagging wood never enters the cone.
- With the taper effectively off, the spruce's stills match round 4 within 30 colour levels per pixel (fn-194 `raw/round4-notaper/`).
- The beech changes by 43 of 2.8M pixels, a few twigs (`raw/beech3/`).

**Host verdict (2026-10-04): accepted.** The cancellation argument holds, and gravitropism's sine law is the botanically sourced response.

**Test:** a direction swept through straight down turns by degree (`geometry/tests.rs`). It was red first: one 2e-4 rad step moved the turn by 0.79.

### Host decision 3: zero-sag neutrality amended, and wood vertical at the ground accepted

- **R1** now reads "`form.sag` at 0 adds no bending". **R4** now reads "the beech unchanged in look, its pixel difference recorded".
  - The landing curve and the taper are engine improvements for every tree, under AGENTS.md "Generator evolution". An intentional improvement may change bytes when the look holds, and 43 of 2.8M pixels holds.
  - The spec is amended with `flowctl spec set-plan` (host, 2026-10-04).
- **Wood standing exactly vertical when it reaches the ground still takes a side from its vanishing lean.** It is the same topological limit, and measure-zero: a lateral swept through an insertion of exactly 3π/4 ± 1e-7 lands about 0.8 m apart along the ground.
  - Leaving such wood standing would put it below the ground.
  - The host accepts the singularity.
  - It is kept out of the walks: the landing walk gives limbs 0.2 wander, so none stands exactly on the vertical.

### Codex review of the follow-up (base `d27b6ef2`)

**Round 1, NEEDS_WORK, three P1s:**
1. A vertical trunk never sagged: the in-plane pivot had no plane. **Fixed** (`dd5cc47d`) by turning about the frame-carried torque; test `a_vertical_trunk_bends_under_a_one_sided_crown`.
2. Zero-sag trees changed. **Resolved by host decision 3** (spec amended).
3. Landing normalised a vanishing lean above the ground. **Fixed above the ground** (`dd5cc47d`): the eased step keeps its lean in one vector. Test `a_lateral_swept_through_straight_down_lands_by_degree`.

**Round 2, NEEDS_WORK:** finding 1 confirmed fixed; findings 2 and 3 open.
- Finding 2 is zero-sag neutrality: **resolved by host decision 3.**
- Finding 3, now at the ground (a vanishing lean on wood that has landed), is the vertical-at-the-ground singularity: **accepted by host decision 3.**

The review stage is **OVERRIDDEN by host decisions 2 and 3**, as fn-192's override was recorded.
