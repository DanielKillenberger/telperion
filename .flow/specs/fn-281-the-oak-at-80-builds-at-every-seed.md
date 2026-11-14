# The oak at 80 builds at every seed

## Goal

fn-236's round 4 (2026-10-14): the Oregon white oak at age 80, seed 7, was refused with "invalid input: unsolved radii", predating the round. The harness's age dial reaches 80 and the site's grove draws seeded oaks, so a shipped preset refused at a reachable age is a release defect. Reproduce, sweep, find the cause, fix it by degree with no fallback radius and no species branch. Release item (fn-237); the oak's values are fn-236's.

## Decisions

- Host (2026-10-14), fix A in the conversion: the code is brought to the module's stated rule that wood thinner than `FINEST` is not drawn; the engine's f32 girth sum is not changed.
- Host (2026-10-14), bytes: R3's "bytes unchanged where they solved before" is replaced by listed byte changes (AGENTS.md, generator evolution: a correctness fix may change bytes).
- Host (2026-10-14), the engine's phytomer budget: the oak at 80 is also refused by the 20,000,000-phytomer budget at 6 of 50 seeds; not this fix — fn-236 is changing the oak's values, the 80 sweep is re-run after them, and the refusal by name stays meanwhile.
- Host (2026-10-15): visibility judged from mesh counts, not stills (most-changed tree at its own age loses 0.024 % of leaves and wood under 0.2 mm radius, bounds unchanged); accepted as below any supported view.

## Acceptance Criteria

- **R1:** the refusal as a standalone test, red on CPU and device; a sweep of the oak at 60-100 and every preset at its age and at 80, seeds 1-50 — met (`radii_tests::the_oak_at_80_seed_7_solves_every_radius`; unsolved radii only in the oak: 60 seed 25, 70 seeds 25 and 34, 80 seeds 3, 7, 30, 39; the device builds it because it runs no `validate_solved`).
- **R2:** the solver's failure condition against the tree's structure, checked in code, reported before any fix — met (below).
- **R3:** every tree's radii solved by degree: the conversion and `dress.wgsl` judge `min(p.radius, above)` against `FINEST`; the device draws no radius-zero node at seeds 3, 7, 30, 39; byte changes listed; red test and sweep kept as the regression guard — met.
- **R4:** how many of the 486 nodes thinner than `FINEST` in the oak at 80, seed 7 the fix removes — met: all 486, plus the one at radius zero (2,193,278 → 2,192,791 nodes).

## Result

Cause: at a relay's inside join, `girth::thicken_given` sets `Join::above` to the f32 difference of the running section with the relay's share added; where the stopped module's head carries a section below f32 resolution of the relay's share (4.1e-13 vs 1.2e-5), the subtraction is exactly 0. The conversion keeps the phytomer by its relay-inclusive radius (`space.rs:426`) and draws the head at `min(own.radius, j.above)` = 0 (`space.rs:443`), a lateral node at radius zero that `validate_solved` refuses. Fix A ends the axis at the join when the drawn radius is under `FINEST`. Sweep after the fix: no unsolved radii; 463 of 950 trees change, each only by wood thinner than `FINEST`; the palm and the spruce unchanged; engine-budget refusals unchanged (oak at 80: seeds 14, 25, 34, 43, 46, 48; at 90: 39 of 50; at 100: 46 of 50). Codex SHIP (three reviewers); the workspace gate is the host's.

## Open

- At 80 the device and CPU oak node counts differ by a few nodes in 2.3 million (seed 3: +15; others +19, +35, −69), already before fix A; untraced; a release-check item for fn-259.
- The ordinary at 80, seed 48 keeps one node thinner than `FINEST`; untraced.
- The oak's phytomer-budget refusals at 80+ -> fn-236 / fn-285.

## Friction

- The probe's first build and the device check queued behind sweeps for room -> smaller admission for a compile, shared sccache, printed host peak per device build.
- The device check refused for memory, then shader swaps -> test helper drawing a family on the device under budget, plus a device-node readback for the CPU probe.
- Focused tests starved behind the worker's own two sweeps -> one sweep binary building base and fix in one process.

## Evidence

- `.flow/evidence/fn-281-the-oak-at-80-builds-at-every-seed/FIX.md` (fix A, byte changes, R4 count; cited by `thin_tests.rs`)
- `.flow/evidence/fn-281-the-oak-at-80-builds-at-every-seed/SWEEP.md` (R1 sweep, phytomer-budget refusals)
- `.flow/evidence/fn-281-the-oak-at-80-builds-at-every-seed/sweep.sh`, `compare.py` (the sweep and its digest comparison; regression guard)
