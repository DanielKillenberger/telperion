---
satisfies: [R1]
---
# fn-293-each-stages-output-has-one-form-and-its.6 The device sweep reads the drawn records and writes the renderer's curve form

## Description
Findings 2 and 3: the sweep's `gather` copies every drawn record into a 39-word node table that stations, rings and the device field read; then a handoff pass repacks the sweep's points (11 words) and clusters (16) into the renderer's `CurvePoint` (8 words) and clusters. The readers read the drawn records (plus at most a narrow side table for what `gather` derives: leader, ties, run starts) and the sweep writes the renderer's form directly.

**Size:** M (GPU)
**Files:** `crates/telperion-render/src/generation/resident/{sweep.rs,sweep.wgsl,handoff.rs,handoff.wgsl,build.rs,stations.wgsl,rings.rs,rings.wgsl,field.wgsl,field.rs}`, `generation/resident.rs`, `resident/{agree_tests,drawn_memory_tests}.rs`
**Touches:** [crates/telperion-render/src/generation/resident/**, crates/telperion-render/src/generation/resident.rs, crates/telperion-render/src/curve/**, .flow/evidence/fn-293-each-stages-output-has-one-form-and-its/**]

### Approach
- Small before large: first a probe on one preset (spruce, seed 1) that the change keeps `generation_digest`'s resident line; then all presets at seeds 1, 7, 42.
- Finding 2: list each of the 39 words (`sweep.wgsl:122-153`) as a copy of a drawn-record word (`CV=38`), an axis-record word (`CA=12`), or a derived value. Readers (`stations.wgsl`, `rings`, `field.wgsl:64,96-154,392,402`) read copied words from the drawn and axis records by the record's own layout; derived words stay in the narrowest table that holds them. Every reader moves in this task.
- Finding 3: the sweep writes points and clusters in the renderer's packed form (`CurvePoint::pack`, `packed_clusters`; the arithmetic in `handoff.wgsl` moves, including its `signum`), and folds the stretches, bounds and sag partials it reads back; delete `handoff.rs`, `handoff.wgsl`, the CPU-block upload (`handoff.rs:55-60`) and its registration in `resident.rs:204,244`. If a reader of the sweep's points (rings, stations) needs the full-precision frame the packed form drops, the device trees would change: stop and report the measured difference to the host instead of keeping a second layout.
- Measure: device build time on spruce and oak, paired warm over 16 runs, before and after (`examples/prepare_timing.rs`), and the build's GPU peak bytes; in `MEASURE.md`.

### Investigation targets
**Required:**
- `crates/telperion-render/src/generation/resident/sweep.rs`, `sweep.wgsl:100-160`
- `crates/telperion-render/src/generation/resident/handoff.rs`, `handoff.wgsl`
- `crates/telperion-render/src/generation/resident/build.rs:180-320`
- `crates/telperion-render/src/generation/resident/field.wgsl:50-160`
**Optional:**
- `.flow/evidence/fn-212-*/CONTRACT.md:240-250`; `crates/telperion-core/src/pipeline/surface/curve.rs` (`CurvePoint::pack`)

### Key context
AGENTS.md budget: no full-forest capture; GPU tests and examples run natively. `drawn_memory_tests` pins the drawn stage's memory; the node table's removal should lower it, so update its expected numbers with the measurement.
## Acceptance
- [ ] No 39-word node table and no handoff pass; every reader reads the drawn records or one narrow derived table.
- [ ] Resident digests (every preset, seeds 1, 7, 42) equal the baseline, or the task stopped with the measured difference.
- [ ] Device build time and GPU peak before and after in `MEASURE.md`.
- [ ] Gate and `npm test` green.
## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
