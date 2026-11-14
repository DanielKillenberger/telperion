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
The device now treats a stem tip that is both a cluster tip and a rosette apex the way the CPU does. `keys` gives the crown its own run, sorted after every root, and keys any run of wood or cluster the same node starts among the wood's runs. `starts` records whether a run is a crown, as a seventh run word, and `measure` and `emit` both read that word. The u32 leaf sums in `measure` now carry into a 64-bit total held in counts[6..7], and `build::instances` refuses any count past u32 by name ("foliage instance budget").

R2 reachability: codex's case (`states[0].cluster_leaves = 1`) does not reach the state. The palm passes through chain age 0, and its single age is the chain's trunk (`chain::TRUNK` = 2). With `states[2].cluster_leaves = 1` and `canopy.cauline = 1`, the date palm at 40 does reach it (measured). Red at 81eec8142: device wood 0 / 113, leaves 113 / 6667. Green after the fix: wood 113 / 113, crown 6554, leaves 6667 / 6667. The plain palm is unchanged (crown 6554, apart 4.06e-5 before and after).

Sibling fixed in the same predicate: an apex that also starts a bearing run used to lose that run on the device. It now keeps it.

The keys buffer is rebuilt at the counted size when two runs per node outgrow `ids.next_power_of_two()`. No test reaches that path. The runs buffer is now sized by the run count.

B.3 is tested only on the host side, at the boundary (2^32-1 passes; 2^32 and u64::MAX refuse), as the spec prescribes. The WGSL carry in `add_leaves` has no device test of its own.

Tests: organ_tests::a_tip_that_is_both_bears_its_cluster_and_its_rosette (R2) and organ_tests::a_leaf_count_past_u32_is_refused_by_name (R4 B.3). Gate (telperion-heavy, once): 1408 passed, 0 failed, 111 ignored.

Tier: host dispatch (model: claude-opus-5-5)

stage: impl-review - ran (codex, round 1: SHIP on all three draws: correctness, contracts, integration)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 81eec8142f5aed864b7d8fa340e14ff9560e0365, 924c9f7be113951cdc6e3023e2dbd3dac5ac4185
- Tests: telperion-heavy 8 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast (1408 passed, 0 failed, 111 ignored), cargo test -p telperion-render --profile ci -- stations a_tip_that_is_both the_device_draws_counts_of_organs a_leaf_count_past_u32 every_shader_parses_and_validates
- PRs: