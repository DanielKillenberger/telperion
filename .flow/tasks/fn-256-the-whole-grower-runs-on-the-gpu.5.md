# fn-256-the-whole-grower-runs-on-the-gpu.5 Device budget guard, live-set census, fixed arena

## Description
COUNCIL.md round 4 steps 1-3: the guard (native budget, reserve before create incl. overlap, charges held until completion, count clamps, dropped build polled free; refusal and no-reset tests), the labelled peak census and live set per cycle (Telperion 82, oak 80), then a fixed arena from the measured bound compacted in place on a cadence, moves and restarts deleted.

## Acceptance
Zero table moves; peak at the measured bound; ten runs on a two-thirds-full card end in builds or named refusals, no reset; lineage agreement and repeatability held; per-preset warm build and peak memory reported.

## Done summary
Native builds are held to a device budget: 4 GiB by default, and no more than 85% of what the driver says other processes leave. Every buffer is reserved before it is made, and charges are held until the GPU is done. Every preset now grows in tables sized once from its closed-form hint and compacted in place, axes included, with no moves and no restarts at seeds 1, 7 and 42. Ten builds on a two-thirds-full card ended in four builds and six named refusals, with no reset.

- Guard: native budget (`TELPERION_DEVICE_BUDGET_MIB`); atomic reservation; scoped creation; charges released on GPU completion; polled free before and after a build; count clamps in the cycle's kernels.
  - Tests: `a_halted_cycle_leaves_later_dispatches_empty` (it was red on retirement's end word), `a_build_past_the_budget_submits_nothing`.
- Census and live set (GUARD.md): Telperion's live set is 0.210 of grown, the oak's at 80 0.178; the stop condition did not fire.
- The host's rulings, as built:
  - axes and units compact in place on a cadence, with lineage keys as identity;
  - per-build sizing from the hint, with one doubling move;
  - the light's scratch sized by nodes and columns (1,802 MB to 179 MB), and the verdict log only where a test asks;
  - the Two Trees fit the 4 GiB default (Telperion 3,990 MB, Laurelin 3,707);
  - restarts deleted and every overflow refused by name.
- Warm builds against task 4:
  - faster: ordinary 522 ms (604), oak 405 (525), spruce 554 (613), birch 351 (782);
  - slower: beech 326 (295), palm 136 (98), Telperion 1,958 (1,909), Laurelin 1,869 (1,782).
- Review: codex NEEDS_WORK on four findings (ledger record drop, overlapping caps, records outside the arena, hint overflow), all fixed; the re-review returned SHIP.
- Open: the oak at 80 no longer fits 4 GiB, because its hint over-sizes its axis tables; the sweep's bound comes from the lattice's nodes, as the species carry no height row; the Two Trees are 3 to 5% slower than task 4.

## Evidence
- Commits: c7ff2e08408af144224d90d757a7887f533fcd8a, d7c2be70804242be8494302824861e74cf55d4b4, e263a3348530e0e2f3151559b15934641e743eb2, 8daa8e79646b944651acacef709dae179d492c8a, c1980467bb40d88076bb8e3b52bb9d4d6c94f051, b10d1e2a884350732536d764a04735f5ddf99893, 8bcdd447d6217cfbcee0a83d4b193fd2003fb38e, 57316e48730f70b95f0518d384583bb77e7026ea, 69a5da5d68b59c0c623647e74e824c8f08b4afe0
- Tests: cargo test --profile ci --workspace --no-fail-fast, telperion-render lib under the GPU lock (169 passed), npm test (162 passed), every_preset_grows_in_its_hinted_arena (ignored gate, 24 builds, 0 moves)
- PRs:
