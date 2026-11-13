# GATE-3, fn-360 worktree, 13 Nov 2026

## Step 1: the pin
`pipeline::cycle_cost_tests::every_preset_grows_the_tree_it_grew_at_both_ends_of_the_dial` (telperion-core lib).
- 5033ed05a (gate-bisect, detached): passes.
- 19a86ceca (gate-bisect): fails; produced values identical to this branch's head.
- Hypothesis holds. Repinned in 1958c708c (only cycle_cost_tests.rs). Rows changed: ordinary (both ends), Oregon white oak (both), Norway spruce (coarsest), European beech (both), silver birch (coarsest). Date palm, Telperion, Laurelin unchanged. Test green after.
- gate-bisect left detached at 19a86ceca; tracked tree clean (it already held two untracked example files, not mine).

## Step 2: guards, then `gate.py --full` (log: logs/gate-full.log)
- `compile_size`: ok (2 passed).
- `generation_limit_guard`: FAILS. `generation_limits_are_classified`: five unclassified limit sites, all in crates/telperion-core/src/pipeline/surface/samples.rs (`.max(2.0 * own)`, `.max(below - (at(k) - at(k-1)))`, `.min(end - at(j-1))`, and two more of that kind), from fn-360's commits (6d336051d, 7a1cd78fb). Not fixed (outside step 1). They need classifying in docs/generation-limits-inventory.json or moving to ranges.rs.
- The gate ran regardless (package check built; core-without-geometry check did not run).
- The gate was KILLED by telperion-heavy ("KILLED (exit 143, signal 15): likely the 8 GB cap") at 03:03, while `telperion-space::form the_beech_grows_at_every_age_of_its_sheet` ran (SIGTERM at 63.7 s). yield.log has no kill line and ends at 02:04 (stale); no PAUSE falls inside the run (02:46:54 to about 03:03).
- I did not rerun: GATE-FAILURES.md item 7 records that test at 8,631 MiB alone against the 8 GB scope, so a rerun should die the same way.

## Step 3: totals
Nextest summary: 979.3 s (budget 1,560 s; incomplete because of the kill). 1398 of 1404 run: 1392 passed, 6 failed (one of them is the SIGTERM), 87 skipped, 0 timed out at 120 s, 6 not run due to signal (which five never started: unknown, not listed). Over memory: 2 reported (ground_reach, smooth_bark); the beech form test is killed, its peak unmeasured here. PAUSE lines inside the run: 0 (0 s). Not run at all: the core-without-geometry check; node/other post-test checks: unknown.

| Test | What failed | In GATE-FAILURES.md? |
|---|---|---|
| core::generation_limit_guard generation_limits_are_classified | 5 unclassified limit sites in surface/samples.rs, 0.095 s | No (new, from fn-360's samples.rs commits) |
| render::bark_distance resolved_scales_survive_until_the_two_pixel_boundary | "distance series aliases on the OregonWhiteOak 4x: mean 6.0038, p95 16.25" (6.9 s) | Yes, item 5 and item 6 (unfixed at the time; item 5 quotes the 2x figure 5.5257 / 13.50) |
| render::curve the_device_surfaces_the_wood_the_cpu_reference_does | "ordinary close: over budget", left 3 right 0 (2.4 s) | Yes, item 5 and item 6 (overrun 3 vs 0 on the base) |
| core::ground_reach nothing_drawn_passes_below_the_ground | assertions pass (70.8 s, under 120 s); peak 6,035 MiB against 5,184 MiB | Yes, item 7 (5,984 then) and item 5 (timeout then) |
| render::smooth_bark the_smooth_barks_hold_the_resolution_and_redraw_contracts | assertions pass (25.8 s); peak 5,817 MiB against 4,096 MiB | Yes, item 5 (6,001) and item 7 (5,883) |
| space::form the_beech_grows_at_every_age_of_its_sheet | killed by the scope (SIGTERM at 63.7 s); not a verdict | Yes, item 7 (8,631 MiB against 4,096) and item 5 (cut off) |

Timed out tests (120 s): none, so no alone-run was due. Slowest passes: packed_leaf station test 116.9 s, every_shipped_preset_builds_every_artifact_through_the_pipeline 69.1 s.

Unknown: which five tests never started; whether the remaining checks (core without geometry) pass; the beech form test's peak this run.
