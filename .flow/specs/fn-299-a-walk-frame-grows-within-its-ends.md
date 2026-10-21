# A walk frame grows within its ends

## Goal & Context

fn-298's survey (host, 2026-10-20) built every frame of the film's walks on the device. On oak to palm at age 40, the frames from t = 0.962 to t = 0.99998 (frames 106–119 of 120) are refused at all three seeds, 1, 7 and 42. Each grows 12 units in one cycle on a single apex, past the 11 a units job holds (`FRAMES`, `build_arena.rs`).

Neither end of the walk comes close:

| t | Most units per cycle on one apex |
|---|---|
| 0 (oak) | 2 |
| 0.5 | 3 |
| 0.9 | 7 |
| 0.95 | 9 |
| 0.962 to 0.99998 | 12 |
| 1 (palm) | 1 |

So a walk frame grows outside the range its two ends span. STRATEGY.md's tree space treats a walk as continuous between its ends; a count shooting past both of them as t approaches the palm breaks that. On the CPU the same frames do build, since it has no `FRAMES` limit. Whether those frames look wrong in the film is unknown.

The suspected cause comes from fn-298's reading of the code (`telperion-space/src/blend.rs`, `species/scale.rs`, and each frame's printed blended state). It has not been traced step by step:
- `lifespan` blends on a log scale toward the palm's 0 for the ages the palm skips (3 to 14). At t = 0.962, those ages last only 0.04 to 0.27 of a cycle.
- Age 2's `continuation` blends linearly, so it is still 0.038 at that t, and the oak's later ages stay reachable.
- One apex then passes through every one of those short ages in a single cycle.

fn-298's two `unit_rounds[0]` misses, oak to palm at seed 42, frames 76 and 77, may share this cause; nobody has checked.

Joins the release (host, 2026-10-20): the film shows this walk, and fn-298's device sweep leaves these frames ignored until this spec lands.

## Acceptance Criteria

The quantity throughout is `telperion_space::most_units` on the grown species of a walk frame (`growth(families(a, b, t)).species`), the count the device holds against `FRAMES`. It reads the species alone, so the three seeds are a check that the species does not depend on the seed, not three samples.

- **R1:** A test reads the most units per cycle on one apex along oak to palm at age 40, at t = 0, 0.5, 0.9, 0.95, 0.962, 0.99, 0.99998 and 1, at seeds 1, 7 and 42. It fails on the base wherever a frame goes outside the range its ends span. A trace confirms which blended row or rows cause the overshoot, or refutes the suspected cause above, stepping `most_units` through the chain it counts at t = 0.962 and naming each age's blended `lifespan`, `continuation` and `relay_ended`. Errors: a frame `families` refuses fails the test by name, never skips it; the trace also reads beech to spruce at t = 0.00037 (the existing `the_floor_counts_the_ages_a_cycle_chains` case, 4 against the beech's 2) and states whether it is the same cause.
- **R2:** A blended walk frame's per-cycle growth, meaning units per apex in a cycle, stays within the range of its two ends, up to a tolerance stated in the test, at every t. The fix lies in how the rows blend (`blend.rs` or `scale.rs`), as a property of the blend, not as a clamp on the grown count. A row's blend that takes an age the other end does not have must not shorten that age toward zero while the age is still reachable. Question first: whether any blend of `lifespan` and `continuation` is needed at all for ages one end lacks. The rule and the tolerance are the host's call once R1's trace names the cause (host, 2026-10-20). **Ruling (host, 2026-10-20): rule B with tolerance (a).** Rule B: strictly between the ends, an age one end never reaches (no bud of it is ever in that age, entered or passed through, from the seed's PA 0 by successors and by lateral and dormant buds) takes the other end's settings, every row, before the rows mix; an age both ends reach, or neither, mixes as before. Reason: a placeholder at an age the species never reaches is not data, so blending it is the defect; an age only one end reaches keeps that end's settings, and the walk fades how often a bud enters it, through the continuations and laterals that already blend, which changes the tree by degree. Tolerance (a): the ends' max + 2, the measured worst case under rule B (R1.md); the test names the two mechanisms it leaves, pass-through slivers and mixed stop-and-move outcomes, keeps `the_floor_counts_the_ages_a_cycle_chains` at 4 with the sliver named as the reason, and holds every film frame at or below `FRAMES` (11). **Narrowed (host, 2026-10-21, option 1): rule B swaps only the timing rows.** For an age one end never reaches, only the rows that decide whether and for how long a bud lives that age are swapped. Every other row, including the leaf, size and shape rows, blends as on the base. Why: R2 names the defect as a timing blend that shortens a still-reachable age toward zero. Swapping every row went past that. It made unlike cluster counts within one tree (`walk_leaves`), a jump at t = 0⁺ in the leaf law (`walk_law`), and a leaf-wrapped pole at oak to palm t = 0.5. The timing rows, and why each one counts:
  - `lifespan`: how long a bud lives the age. Its placeholder 0 made the slivers.
  - `continuation`: whether an apex whose lifespan is spent moves on. Its placeholder 1 kept the chain past the oak's stops.
  - `relay_ended`: the relay share of an apex that stops at the end of its lifespan. `outcomes` mixes it jointly with `continuation`'s stop share, so the two move together.
  - `abortion` and `relay`: the abortion stop and its relay share, which decide whether the apex leaves the age early. `outcomes` mixes them as one pair.
  - `viability` and `relay_failed`: whether a growth unit fails, which stops the apex, and the relay share of that stop. `outcomes` mixes them as one pair.
  - `abortion_rise`: how the abortion hazard rises along the age's units, and so when that stop falls. Errors: the rule holds in both directions of a walk; it covers ages a species passes through on the way to a later age (beech to spruce) as well as ages behind a stop (oak to palm), or names the case it leaves out and why.
- **R3:** Every shipped walk is checked by the same test at the film's frames (t = ease((i + 1) / n), n = 90, 90, 90, 120, 30, 90 per walk, as fn-298's survey built them). fn-298's ignored oak-to-palm frames are un-ignored and build on the device. The two `unit_rounds[0]` frames either build, or are named as a different cause. Errors: fn-298 is not on this branch's base; if it has not landed when the last task runs, the CPU test asserts `most_units` at most `FRAMES` on those frames, and the un-ignore is recorded as open for whichever of fn-298 and fn-299 merges second.
- **R4:** Shipped presets at their walk ends (t = 0 and t = 1) are byte-identical to the base, for every shipped walk pair. Walk frames between the ends may change, under AGENTS.md "Generator evolution". Show the changed frames as stills: oak to palm at t = 0.5, 0.9 and 0.97, base against change. Errors: no end byte changes; a still that shows a perceptible regression is reported, never waved through.
- **R5:** The gate (`cargo test --profile ci --workspace --no-fail-fast`) and `npm test` are green, with a codex review per task.

## Early proof point

Task fn-299-a-walk-frame-grows-within-its-ends.1 validates the cause (the R1 repro red on the base and the step trace). If the trace refutes the suspected cause, re-evaluate R2's rule before fn-299-a-walk-frame-grows-within-its-ends.2.

## Quick commands

```bash
telperion-heavy 8 --threads 2 -- cargo test --profile ci -p telperion-core --test units_bound
telperion-heavy 8 --threads 2 -- cargo test --profile ci -p telperion-space blend
```

## Boundaries

- Not the device limits (fn-298).
- Not the film's routing back to the device (fn-272).
- Not `most_units` itself: its count is the measure, and R2's fix lies in the blend.
- Not fn-273's size undershoot on the same walk (t = 0.64 to 0.72), though the rule may move it.
- Open, after the release (host, 2026-10-20): option (c), a rule for the two mechanisms rule B leaves, pass-through slivers (an age one end passes through and the other lives counts a whole unit at t = 0+) and mixed stop-and-move outcomes (a blended age that may both stop with a relay and move on). Not built here.

## Strategy Alignment

Active tracks served by this plan:
- **The tree space** - a walk frame stays between its ends, the continuity the track claims for every walk.

## Decision Context

- The blend rule is a design call (AGENTS.md "Dispatch and escalation"; host, 2026-10-20): task .1 ends with a proposed rule and its effect on oak to palm at t = 0.5, 0.9 and 0.97, and returns to the host before task .2 builds it, unless the trace shows one obvious rule whose only effect is to keep the frame within its ends.
- The ends at oak to palm are 2 and 1, yet t = 0.5 already reads 3; the tolerance decides whether mid-walk frames are held to [1, 2] or to the ends plus a margin.
- The existing test `the_floor_counts_the_ages_a_cycle_chains` pins a frame outside its ends (4 against 2); R2 rewrites it if the rule covers pass-through ages.
- A clamp on `most_units` or on the grown count was rejected in the spec text: it hides the blend's discontinuity rather than removing it.


## Requirement coverage

| Req | Task(s) |
| --- | --- |
| R1 | fn-299-a-walk-frame-grows-within-its-ends.1 |
| R2 | fn-299-a-walk-frame-grows-within-its-ends.2 |
| R3 | fn-299-a-walk-frame-grows-within-its-ends.3 |
| R4 | fn-299-a-walk-frame-grows-within-its-ends.3 |
| R5 | fn-299-a-walk-frame-grows-within-its-ends.3 |

