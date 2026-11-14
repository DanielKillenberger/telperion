---
satisfies: [R4, R7]
---
# fn-297-the-deep-reviews-findings-are-fixed.3 The cycle's job sizing and block assignment refuse before they write

## Description
B.1 and B.2 are both in the cycle (`generation/cycle/`). B.1: `counts()` narrows the job offset and size to u32; compute in u64 and refuse by name above the device's buffer limit. B.2: `assign_main` writes `opened` into shape columns unguarded; guard against the stride and raise the existing `X_FAULT`.

**Size:** M
**Files:** `crates/telperion-render/src/generation/cycle/device.rs`, `crates/telperion-render/src/generation/cycle/hier_blocks.wgsl`, a sizing test in `cycle/`
**Touches:** [crates/telperion-render/src/generation/cycle/**]

### Approach
- B.1: `device.rs:100-113` (`zone_at` at 105, `ZW * jobs * zones` at 106, `b_at` at 66-68, `job_at` at 104). Callers `build_move.rs:211`, `build_pause.rs:113`, `build.rs:253`, `plan_units.rs:42`. Refuse with `Error::ResourceLimit("...")` (pattern `resident/build.rs:217`) before any buffer is created. Unit test the sizing at one below, at, and above the limit; no device-limit probe.
- B.2: `hier_blocks.wgsl:166` `opened`; writes at 178, 180, 183-187; `at(col,i)` is `col*cfg.stride + i` (`hier_build.wgsl:76-78`); fault `atomicOr(&tally[X_FAULT], 1u)` at 42-43, reaching the host via `plan_hier.rs:198-200` and `build_grow.rs:195`. Skip the write at `opened >= cfg.stride`. Test with the smallest configuration that reaches it; if no allowed parameter reaches it, say so and test the guard's arithmetic or the nearest reachable stride.

### Investigation targets
**Required:**
- `crates/telperion-render/src/generation/cycle/device.rs:55-115`
- `crates/telperion-render/src/generation/cycle/hier_blocks.wgsl:30-190`
- `crates/telperion-render/src/generation/cycle/hier_members.wgsl:145-155`
- `crates/telperion-render/src/generation/cycle/plan_hier.rs:190-205`
**Optional:**
- `crates/telperion-render/src/generation/cycle/hier_math_tests.rs`, `docs/gpu-tests.md`

## Acceptance
- [ ] B.1: sizing computed in u64 and refused by name above the limit, tested at the boundary, red before the fix
- [ ] B.2: the write is guarded against the stride and raises the fault the host refuses by name; tested, or the unreachability recorded with the nearest case tested
- [ ] Gate green once at the end under telperion-heavy; codex impl review SHIP

## Done summary
B.1: `cycle/device.rs` `counts()` sizes the jobs' state in u64 and refuses with `ResourceLimit("the cycle's job counts")` before any buffer is made when the words pass the device's storage limit (a buffer exactly at the limit is accepted) or what a u32 index reaches. B.2: `hier_blocks.wgsl` `assign_main` skips the write at `opened >= stride` and raises `X_FAULT`.
Tests: unit tests of the sizing at one below, at and one past the limit, and of the 7.47M-sleeper wrap; a GPU test of a block opened past the stride. fn-256's held-device test (`route_tests::a_buffer_over_the_devices_limit_is_refused_by_name`) now also accepts the earlier named refusal; the gate found it red.
Gate: 1412 passed, 1 failed (that test), 111 ignored; after the fix the render lib's filtered run is 6/6 green. Codex review SHIP twice (b0862780b, then 0232a46ad).

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: fc29edf1e, b0862780b, 0232a46ad
- Tests: telperion-heavy 8 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast (1412 passed, 1 failed, 111 ignored; the one fixed in 0232a46ad), telperion-heavy 8 --threads 2 -- cargo test -p telperion-render --profile ci --lib -- route_tests::a_buffer_over cycle::device cycle::hier_tests (6 passed)
- PRs: