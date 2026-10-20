---
satisfies: [R3, R7]
---
# fn-297-the-deep-reviews-findings-are-fixed.2 Flower bounds contain the drawn stalk

## Description
The prepared tree's flower bounds fold only the stored element vertices, which for a stalk sit near the origin; the drawn stalk reaches `rachis + pedicel` plus the ribbon half width (droop is a signed, dimensionless bend share, not a length). Fix every site that folds a stalk element into bounds, with the reach taken from the values `select.wgsl` uses.

**Size:** M
**Files:** `crates/telperion-core/src/pipeline/foliage.rs`, `crates/telperion-render/src/generation/place.wgsl`, `crates/telperion-render/src/generation/preparation.rs`, a test beside `resident/stalk_tests.rs` or `generation/tests.rs`
**Touches:** [crates/telperion-core/src/pipeline/foliage.rs, crates/telperion-core/src/pipeline/foliage/**, crates/telperion-render/src/generation/place.wgsl, crates/telperion-render/src/generation/preparation.rs, crates/telperion-render/src/generation/compute.rs, crates/telperion-render/src/generation/tests.rs, crates/telperion-render/src/generation/resident/stalk_tests.rs, crates/telperion-render/src/lib.rs]

### Approach
- Red first: Laurelin's `Prepared::bounds()` (`generation.rs:113`) must contain every drawn stalk vertex, computed on the CPU from the stalk lengths `lib.rs:362-368` uploads and `shaders/stalk.wgsl` draws. Record how far the old bounds fell short in the test output and the task summary.
- CPU: `Instances::bounds` (`telperion-core/src/pipeline/foliage.rs:209-253`) is what `preparation.rs:83-89` reads for flowers on both executors.
- Device: `place.wgsl` `written()` (48-72) is reused by the floret placement (`compute.rs` `Placement::Florets`, fn-229); add the same reach there when that path's bounds are read.
- One source for the reach: selection's uniform is laid out `[rachis, pedicel, levels, droop]` (`select.rs:462-467`, set by `lib.rs:362-368` via `organ.set_stalk(rachis, pedicel, droop)`), so `select.wgsl`'s `u.stalk.x + u.stalk.y` is `rachis + pedicel`; the draw uniform (`shaders/stalk.wgsl`) is `[rachis, droop, half width, pedicel]`. Use one helper for the reach; no second constant. Cover a negative-droop case in the containment test.

### Investigation targets
**Required:**
- `crates/telperion-render/src/generation/preparation.rs:75-95`
- `crates/telperion-core/src/pipeline/foliage.rs:200-255`
- `crates/telperion-render/src/shaders/stalk.wgsl`, `crates/telperion-render/src/select.wgsl:85-100`
- `crates/telperion-render/src/generation/place.wgsl:40-75`
**Optional:**
- `crates/telperion-core/src/pipeline/foliage/bloom/stalk.rs`, `crates/telperion-core/src/pipeline/foliage/bounds_tests.rs`

## Acceptance
- [ ] A test, red before the fix, shows Laurelin's prepared bounds contain every drawn stalk vertex; the old shortfall is recorded
- [ ] Zero-length stalks add nothing; non-flowering trees' bounds are unchanged
- [ ] Gate green once at the end under telperion-heavy; codex impl review SHIP

## Done summary
Flower bounds now hold the drawn stalks on every site. The organ element carries its stalks' reach (rachis + pedicel + half the stamen width, from one helper `bloom::stalk_reach`; zero when both lengths are zero). `Instances::bounds` adds that reach around each origin, which covers the prepared tree, the CPU mesh (`mesh.rs`) and the comparison route without touching their call sites. `place.wgsl` adds the same reach on the device's floret placement, reading it from the element vertex's w (`compute.rs`).

Old shortfall on Laurelin (measured): 0 m at her shipped rows (seed 1, at ages 80 and 30), because her crown already holds every stalk. The red case is one floret on a 1 m rachis, 0.364 m short at droop 0.5 and 0.365 m short at droop -0.5, on both executors. After the fix it is 0 m.
Side effect (measured): the uniform reach widens Laurelin's shipped bounds by up to 0.67 m a side (on a box of about 45 m) even though nothing was outside them. A tighter fold would use the pedicel width around each floret and the axis around each rachis.
Tests: `stalk_tests::prepared_bounds_hold_every_drawn_stalk` (GPU: shipped rows, droop 0.5, droop -0.5) and `bounds_tests::stalks_reach_from_each_origin_and_none_without_length` (zero-length stalks add nothing; leaf elements keep a zero reach).
Gate: one run, 1409 passed, 1 failed, 111 ignored. The failure was `generation_limit_guard`, which flagged the fold's presence test as a limit. The test was restated (90619ba65), and the guard and bounds tests were re-run green on their own.

stage: impl-review - ran (codex, three-lens panel, round 1 SHIP)
Tier: host dispatch (model: claude-opus-5-5)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 8ccb516d0e909484ea3d471a0b76a4054551e46b, 6a6f74ffbc4447af46c2225904b53eff9b0332aa, 90619ba6502e5757f853d6de40e5ac0f30326a69
- Tests: cargo test -p telperion-render --profile ci --lib prepared_bounds_hold_every_drawn_stalk, cargo test -p telperion-core --profile ci --lib bounds_tests, cargo test -p telperion-core --profile ci --test generation_limit_guard, telperion-heavy 8 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast (1409 passed, 1 failed: generation_limit_guard, fixed in 90619ba65 and re-run focused)
- PRs: