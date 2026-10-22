---
satisfies: [R1, R2]
---
# fn-309-a-compound-leaf-draws-its-rachis.2 Device: the station path stands its leaflets at the station point

## Description
The device mirror of task 1. The station placement in `pack.wgsl` calls `on_rachis(..., 0.0)` for a placement on wood; with the rule, its leaflets stand at the station point with the heading `on_rachis` gives them. Frond call sites (`pack.wgsl:194`, `resident/field.wgsl:347`) are unchanged.

**Size:** S/M
**Files:** `crates/telperion-render/src/generation/pack.wgsl`, any other WGSL site that places station or cluster leaflets (check `resident/stations.wgsl`, `resident/field.wgsl`), the agreement tests in `crates/telperion-render/src/generation/` (e.g. `resident/agree_tests.rs`, `resident/organ_tests.rs`)
**Touches:** [crates/telperion-render/src/generation/**]

### Approach
- At `pack.wgsl:120-140` ("A placement's rachis on wood draws no bark"), place the leaf at the station `point` and keep `l.axis`, `l.tangent`, `l.normal`, `l.share` from `on_rachis` (the CPU computes the same run at the same `t`). Grep every other `on_rachis` / leaflet site for station or cluster leaves on the device and the field executor; mirror there too.
- Red first: an agreement test that builds a walk frame bearing station compound leaves (oak to palm at 40, t = 0.34, seed 7, or a small hand-built family with `leafletCount` > 1, `rachisLength` > 0, `cauline` 1) on both executors and compares placed leaves within the existing tolerances; red on the base after task 1 (CPU moved, device not), green after. Prefer extending an existing agreement test's case list over a new harness.
- Device digests of every shipped preset unchanged (`resident/digest_tests.rs` `digest`, ignored: run it base vs change with `DIGEST_PRESETS` covering every shipped preset).

### Investigation targets
**Required**:
- `crates/telperion-render/src/generation/pack.wgsl:120-220` — station path and `on_rachis`
- `crates/telperion-render/src/generation/resident/field.wgsl:280-350` — field mirror
- `crates/telperion-render/src/generation/resident/rachis_tests.rs` — frond agreement pattern

**Optional**:
- `crates/telperion-render/src/generation/resident/digest_tests.rs:87,159`

### Key context
- Same machine rules as task 1: every cargo command through `telperion-heavy 8 --threads 2`, one at a time, after checking for a running test binary; the gate once at the end with `--threads 4`.
- A device that disagrees fails its test; never fall back to the CPU.

## Acceptance
- [ ] The device places station and cluster compound leaflets at the station point with the CPU's headings; an agreement test on a frame bearing them is red before and green after.
- [ ] Frond leaflets on the device and the field executor unchanged; every shipped preset's device digest unchanged (base vs change recorded).
- [ ] The gate is green, once.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
