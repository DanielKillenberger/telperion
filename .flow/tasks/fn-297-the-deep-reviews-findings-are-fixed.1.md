---
satisfies: [R2, R4, R7]
---
# fn-297-the-deep-reviews-findings-are-fixed.1 Stations count and write a tip that is both, and flag a leaf count past u32

## Description
R2 and R4 B.3 both live in the resident stations kernels, so one task owns `stations.wgsl`. R2 first: run the repro as a standalone GPU test (red), then make `measure` and `emit` both produce what the CPU produces for a stem tip that is both a cluster tip and a rosette apex (the rosette in its run, the cluster appended after every run). Then B.3: the u32 `leaves` sums in `measure` overflow-flag so the host's instance budget refuses by name.

**Size:** M
**Files:** `crates/telperion-render/src/generation/resident/stations.wgsl`, `crates/telperion-render/src/generation/resident/organ_tests.rs` or `agree_tests.rs`, `crates/telperion-render/src/generation/resident/build.rs`
**Touches:** [crates/telperion-render/src/generation/resident/**]

### Approach
- Repro: `preset("date-palm")` at age 40, `canopy.cauline = 1`, `species.states[0].cluster_leaves = 1` (pattern `crates/telperion-core/src/suite/clusters.rs:16-25`); compare CPU and device counts and records as `organ_tests.rs` `agrees()` (~233) does. Commit the red test before the fix. If the state is unreachable, record which rule stops it (validation, `pipeline/space.rs:557`, apex predicate) in the test doc and test the nearest reachable case.
- Kernel: `measure` (~561, tip at 567, apex at 582) and `emit` (~917, apex at 928, tip at 932) must count and write both the rosette and the appended cluster; predicates at 353/364, run starts at 431-433, `runs[2u*k]` at 460.
- B.3: `leaves += k` (636) and the same shape at 574 and 587; detect wrap and flag it so `resident/build.rs:215-217` refuses with `"foliage instance budget"`. Test as a CPU unit test of the arithmetic if no parameter reaches 2^32 (`suite/clusters.rs:105-115` is the budget-refusal pattern).

### Investigation targets
**Required:**
- `crates/telperion-render/src/generation/resident/stations.wgsl:350-660,790-940`
- `crates/telperion-core/src/pipeline/foliage/prepared.rs:240-300`
- `crates/telperion-render/src/generation/resident/organ_tests.rs`
- `crates/telperion-render/src/generation/resident/build.rs:200-225`
**Optional:**
- `docs/gpu-tests.md`, `crates/telperion-render/src/generation.rs:232` (`test_gpu`)

### Key context
Swapping the test order alone drops foliage. Output bytes for the palm may change; that is allowed (AGENTS.md, Generator evolution) with CPU = device as the proof.

## Acceptance
- [ ] The R2 repro (or the recorded nearest reachable case) is committed red before the fix and green after; CPU = device on counts and records, plain date palm unchanged
- [ ] A leaf count past u32 is refused by name, tested at the boundary
- [ ] Gate green once at the end under telperion-heavy; codex impl review SHIP

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
