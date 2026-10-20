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
TBD

## Evidence
- Commits:
- Tests:
- PRs:
