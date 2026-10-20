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
TBD

## Evidence
- Commits:
- Tests:
- PRs:
