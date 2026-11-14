# A shoot that reaches the ground stops there

## Goal

The owner, looking at the birch in the harness (2026-10-09, screenshot `.flow/evidence/fn-198-tree-space-f-the-engine-in-production/owner-birch-ground.jpg`): "lol what happend to this birch? i guess we accidentally invented roots? looks awesome but not like a birch". Hanging long shoots reached the ground and grew on along it in loops: the lay's resting rule (`geometry/lay.rs`, fn-200's "wood a step would take below the ground stays on it") turns a non-trunk step along the soil. fn-200's design stays right for a sagging spruce skirt. The owner likes the ground-running growth as a feature for trees that do it and was unsure about silver birch ("or maybe birches do do this"); dwarf birch does layer. Moved off the release map (host, 2026-10-09) as first a sourcing question; back on it (owner, 2026-10-14): "we might also want to fix the birch's branches drooping all the way to the ground"; "that's mainly for younger specimen. Older ones will be too high to reach the ground". Scope: the birch's low branches and the lay's ground rule, and their measurement.

## Decisions

- R1 (host, 2026-10-14): the young birch's ground wood hangs from the young stem's passing branches (`SPRIG`, lifespan 14), alive at 29-39 years at 0.6-0.7 m because a lateral bearing its own age makes its bearer's life unbounded (`shed.rs`, `lasting`). The crown base does not lift between 5 and 30 (0.6-0.8 m; stand-grown sources give 3-7 m at 17-23 years, an upper bound for an open-grown tree).
- R2a (finite life ends an axis with its laterals) was ruled, narrowed, recounted and withdrawn (host, 2026-10-14): it would remove up to 29-50 % of the branch systems clothing the oak, beech and ordinary tree's limbs; a branch living while what it bears lives is right for them. The birch's case goes through the ground rule.
- The open design point (sag comes after growth, so growth cannot see sag-only contact) was measured: in growth's rough lay 83-90 % of the birch's resting axes at 10 already touch; at 80 and on the ordinary tree at 80 contact is sag alone.
- R2 as ruled (host, 2026-10-14): no ending in the final lay (unchanged resting rule); growth answers contact: a tip's yearly survival takes the factor (1 - h) while its rough-lay tip rests on the ground, through the existing draws, by degree, both executors. A per-age `ground_hazard` h in [0, 1], neutral at 0.
- Value: no source reached says whether a wild silver birch's hanging shoots end at the ground; the birch's `ground_hazard` 1 is a declared stand-in resting on the owner's verdict of 2026-10-14, with the one contrary text (Hammersley Inlet Farm nursery on the cultivar 'Youngii': branches "will eventually root in the soil or create an untidy mat on the ground if not lifted periodically") recorded beside it. Every other species 0 (spruce keeps fn-200's skirt).
- Owner's verdict (2026-10-14): accepted the ground rule for the release ("ok", to: ship R2 as built). The young birch's sag-only contact at 20-30 and its persisting low passing branches go to a general crown-lift spec (fn-282); no birch-only value change, because a value chosen for its effect rather than a source would break the design rule.

## Acceptance Criteria

- **R1:** for the birch at 5-80 (seeds 1, 7, 42) and every shipped species, axes and wood resting on the ground and the age they reach it, both executors' resting rule, why lower strands carry few leaves, crown base against sourced crown lift — met.
- R2: a per-age `ground_hazard` row, 0 neutral, acting by degree through the existing draws on both executors; values sourced first — met (as ruled 2026-10-14; birch 1 a declared stand-in).
- R3: red/green tests (0 byte-identical, fn-200's resting test holds; at high values no tip in contact survives its next cycle); birch stills viewed; `space_hash` changes listed; CPU and device agree; gate and npm test green; the owner's verdict — met (full gate on the merged branch by the host).

## Result

- Birch wood lying on the ground (seed 1/7/42, m), before → after: at 10, 9.3/12.1/5.7 → 3.2/3.2/2.2; at 15, 37.4/46.8/40.2 → 13.2/15.2/11.8; at 20, 85.0/87.5/80.9 → 61.1/49.2/42.9; at 30, 135/116/98 → 119/88/64; at 40, 22.6/2.1/22.1 → 5.6/2.1/0; at 80 unchanged (sag-only contact). Crown base unchanged at 5-30 (0.61-0.79 m).
- Tests: `telperion-space/tests/ground.rs` `a_tip_on_the_ground_dies_at_one_and_runs_on_at_zero` (red: 38 of 45 lived on), `a_ground_hazard_acts_without_shade`; fn-200's resting tests green; device agreement `young_trees_grow_on_the_device_as_on_the_cpu`. Only the birch's `space_hash` moved; catalogue digests re-pinned. Impl-review SHIP after one fix.
- Why lower strands go bare (from fn-264): shaded short shoots, which bear the birch's leaves, die under the shade hazard while lit long shoots run on; ψ stays dormant for the birch (host, 2026-10-12).

## Open

- fn-282: general crown lift (the young birch's persisting low passing branches and sag-only contact at 20-30).

## Friction

- Probe memory per age unknown, runs OOM-killed -> a memory estimate per age in the probe's usage (done by hand: young ages at 2 GB, 80 alone).
- `telperion-heavy` passed on the script's status after an OOM kill -> report the kill (proposed).
- Measuring ground contact grew the whole spruce at 80 at once and admission waits were indefinite -> a cheaper contact measure or a queue position in the wrapper (proposed).

## Evidence

- `.flow/evidence/fn-252-a-shoot-that-reaches-the-ground-stops/R2-PLAN.md` (the sourcing for `ground_hazard`, the 'Youngii' text; cited by `silver-birch.values`)
