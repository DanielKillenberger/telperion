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
The device now places every leaflet of a station or cluster compound leaf at the station point, as the CPU has since task 1. It keeps the axis, tangent, normal and share that `on_rachis` gives each leaflet. The only site was `pack.wgsl`'s station path. The frond sites (`pack.wgsl` `leaflet`, `resident/field.wgsl` `blade`) are unchanged.

Tests: `organ_tests::the_device_draws_counts_of_organs_as_the_cpu_does` now matches each of the wood's leaves to a CPU leaf by place, within `APART` of the height, and holds their headings to the crown's turn bound. The film's frame 526 (oak to palm at 0.34, age 40, seed 7) joins its families. On the base it was red: for the oak with 1.3 leaflets, 2,006 of 4,280 leaves stood off every CPU leaf. After the change it is green, with 0 off on every family except the film frame (5 of 740 off). The device digests of every shipped preset at seeds 1, 7 and 42 are byte-identical, base against change (24 lines, one sha256; `.flow/evidence/fn-309-a-compound-leaf-draws-its-rachis/RESULT.md`, "Task 2").

Follow-up, not this task: on the film frame, one placement at the stem's top (y 9.52 m) stands 0.44 m from every CPU leaf. It is the same with `leafletCount` 1 and `rachisLength` 0, so the disagreement predates fn-309. It falls inside the test's one-in-a-hundred allowance.

stage: impl-review - ran (codex, one correctness reviewer, SHIP, no findings; the reviewer read the committed evidence and ran no build)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: d9f30002d7695d9aa417502bc5b27a1e91dd6d79, f3c93ac6a96de576f18ae87dadf7c0591f6e81f2, 6114a53ce3bbda615fc5ddb27484115d73997ba1
- Tests: telperion-heavy 8 --gpu 3 --threads 2 -- cargo test --profile ci -p telperion-render --lib generation::resident::organ_tests::the_device_draws_counts_of_organs_as_the_cpu_does (red on base, green after), telperion-heavy 8 --gpu 3 --threads 2 -- cargo test --profile ci -p telperion-render --lib generation::resident::digest_tests::digest -- --ignored (base vs change identical, 24 lines), telperion-heavy 8 --threads 4 -- python3 scripts/gate.py (1477 passed, 1291 s)
- PRs: