---
satisfies: [R2]
---
# fn-211-tree-space-f2b-growth-years-cheap.2 Every serial growth pass on every core, exactly

## Description
Layer 1 (spec Design). PROFILE.md: every pass but growth and the lay is serial (balance, allot, sketch, the re-lay's settle, sag levers, the light sweep, assign, attachments, shed, scale, thicken), and the lay's framing loop is 20-25 % serial. Make each run on the process's core budget (cores.rs) with output identical to the bit; reductions in a fixed order so float sums do not move. Add per-pass timing to the stage reports (the friction entry in FRICTION.md), so the next profile needs no hand patch. Also find the oak's unattributed 7 % (347 ms at age 80 seed 1).

## Acceptance
- The fn-210 hash table (measures hash 80, seeds 1 and 7, neutral and lit) identical at every commit. - Per-pass timings before and after, 1 and all cores, ages 40 and 80, four species, in RESULT.md. - Per-pass stage reports in place and used for the after table. - Workspace gate green.

## Done summary
Layer 1 built.

- **Pass reports:** `Stage::Pass(Pass)` reports 26 passes, and `measures passes` times them.
- **Levers:** light sweep and reads; shedding's fades in one exact backward pass; assign; `upward::Tree` (depth-at-a-time child sums in the serial order) for attachments, thicken and sag levers; the lay's framing; the rough layout; packed rows for balance and allotment; the full lay's copy, write-back and leaves; scale.
- **Hash proof:** the hash table (80 years, seeds 1 and 7, neutral and lit) is identical for every lever on all cores, and at HEAD on one core.
- **All cores:** oak 80 is 32% faster (5.09 to 3.48 s, 10.8 to 7.3 s); spruce 80 26 to 28%; beech 80 23 to 26%.
- **One core:** within -6% to +2%; the by-depth passes cost 10 to 40% more there.
- **The unattributed 7%:** freeing (tail about 180 ms, about 45 ms in RelayWrite) plus the gaps between passes.
- **Stayed serial:** balance and allotment sums (memory-bound; layer 2), merge, shed's drain and freeing, the light lattice (spawn cost ate the gain), smaller passes.
- **Host questions:** a thread pool (spread spawns about 15 us a thread), one-core paths for wasm, the per-axis allocation layout.
- **Workspace gate:** green (1,113 passed). Codex review not run.

See `.flow/evidence/fn-211-tree-space-f2b-growth-years-cheap/RESULT.md`.

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 239612a2, fb5974ea, f3375582, 3c1c2ce4, c4c4ef39, ec0986bd, eee788d7, f271387d, 55ff41de, 6944917c, 63ce2c59, 91c7896a, 305dca0d, 6b34fd6d, 90ad91d9, 53445e1d, 78708e4d
- Tests: cargo test --profile ci --workspace --no-fail-fast, measures hash 80 1,7 all both (all cores and taskset -c 0)
- PRs: