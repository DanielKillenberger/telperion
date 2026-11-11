# Wood that lasts: the budget, the girth and the sag

## Goal

An old tree stands on wood it has laid and never loses, pays only for its living wood, and bends as green wood bends. The budget fix, the girth rule and the sag value land together, judged as one against the scoreboard, because each alone makes some tree worse.

## Context

- Child of fn-351, Phase 2 (criteria P2, P8, L6, part of P6 and P15). Owner, 10 Nov 2026: "ok well then we need to fix this bug properly. And we don't need byte identical trees to come from the fix."
- The design and its sources: `.flow/evidence/fn-351-020-the-release-candidate/PHASE2-DESIGN.md` and `PHASE2-SURVEY.md`. The leaf findings it depends on: `LEAF-SURVEY.md`. Read all three first.
- The budget fix is built and tested as fn-349 on branch `fn-349-the-corrected-balance-in-the-product` (worktree `.worktrees/fn-349`, head `210f97006`), together with a port into the GPU grower that no longer exists. fn-349 closes as superseded by this spec.
- Work happens on branch `fn-354-wood-that-lasts` in worktree `.worktrees/fn-354`, cut from the one-grower branch. The host merges.
- The budget fix alone is known to make two trees worse (the oak at 300 on seed 1001 laid over; the oak at 200 on seed 1012 refused), so nothing of this spec merges until the whole is judged.

## Acceptance Criteria

- **R1 (the budget on one grower):** fn-349's CPU work is carried onto this branch: dead wood charged nothing; a living tip charged its size times the square of its path over the species' potential height; a part earning what its paying children leave over; the main line tested; the path carried per laid phytomer; the lattice's refusal taking its node limit as an argument; the walks measured in path over potential height. Nothing of the device port is carried. Its tests are green.
- **R2 (the Two Trees take a potential height):** Laurelin and Telperion carry an inverse height like any tree, taken from the species each is built on if the presets record one; the film map's label test keeps checking what it promised. If no base species is recorded, the builder returns with what the presets say.
- **R3 (leftovers of the device, deleted):** `telperion-core/tests/units_bound.rs` and `most_units` if nothing else reads it, `telperion_space::Error::Executor`, `Crown::rows`, `closed_form::expected_axes` with the test that alone reads it if that test holds nothing else.
- **R4 (the probe, before any product rule):** as `PHASE2-DESIGN.md`, "The probe": the girth rule with its yearly pass and sag at today's rows and at `4ρg/E`, as a patch kept in evidence; drawn and measured on the scoreboard's panel for the oak, beech, birch and spruce; yearly leaf area recorded for every seed; cost with and without the yearly pass. The host reads it and writes the verdict before R5 to R7 start.
- **R5 (one leaf account):** on the host's word after R4. The drawn leaf is the grower's leaf: one quantity sets what a shoot's leaf shades with, what is drawn and what sizes its wood, and a test holds the drawn leaf area of a tree to the grower's.
- **R6 (the girth and the sag as product):** on the host's word after R4. The rule of `PHASE2-DESIGN.md` as product code with tests; `pipe`, `exponent`, `ripening`, `secondary`, `retained` and `leaf_girth` deleted from every preset, the scale table and the docs; one sag value per species from measured stiffness and density, or what R4 shows is needed, each value with its source or listed as an estimate.
- **R7 (the beech's branching):** on the host's word after R4. The cause of the beech's shoot deficit found by measurement (which rows carry it) and fixed from sources for open-grown or dominant trees, or returned to the host if no source exists.
- **R8 (judged):** the scoreboard redrawn; no tree worse than at `base-2026-11-11` by the host's eye on every sheet; the oak standing at 200 and 300 on twelve seeds with none refused; the oak's trunk inside White's three open classes from 80 to 300 on most seeds; the oak on seed 7 judged.
- **R9 (green):** the two guards, the full gate from a clean tree, `npm test`, and a codex review of this spec's diff inside the queue.

## Boundaries

- No value fitted to hit a target. A row with no source is listed as an estimate with its reason.
- Phase 3's work (a crown that renews) is not done here: the oak's crown at 200 and 300 may still be thin, and R8 judges standing and girth, not fullness.
- Nothing is pushed. Nothing merges until R8.

## Evidence

`RESULT.md`; the probe's patch, tables and sheets; `FRICTION.md` as it happens.
