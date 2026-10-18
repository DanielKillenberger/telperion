---
satisfies: [R1]
---
# fn-293-each-stages-output-has-one-form-and-its.6 The device sweep reads the drawn records and writes the renderer's curve form

## Description
Findings 2 and 3: the sweep's `gather` copies every drawn record into a 39-word node table that stations, rings and the device field read; then a handoff pass repacks the sweep's points (11 words) and clusters (16) into the renderer's `CurvePoint` (8 words) and clusters. The readers read the drawn records (plus at most a narrow side table for what `gather` derives: leader, ties, run starts) and the sweep writes the renderer's form directly.

**Size:** M (GPU)
**Files:** the dress's files in `crates/telperion-render/src/generation/cycle/` (`dress.wgsl`, its Rust, `given.rs`'s made-count sizing) and `crates/telperion-render/src/generation/resident/{sweep.rs,sweep.wgsl,handoff.rs,handoff.wgsl,build.rs,stations.wgsl,rings.rs,rings.wgsl,field.wgsl,field.rs}`, `generation/resident.rs`, `resident/{agree_tests,drawn_memory_tests}.rs`
**Touches:** [crates/telperion-render/src/generation/cycle/**, crates/telperion-render/src/generation/resident/**, crates/telperion-render/src/generation/resident.rs, crates/telperion-render/src/curve/**, .flow/evidence/fn-293-each-stages-output-has-one-form-and-its/**]

### Approach
- **Host decisions after task 5 (spec) govern this task.** The probe is done (BASELINE.md "Task 6 probe", `a3b69dece`).
- Finding 2, host decision (a): the dress writes every wave into one drawn-record buffer at aligned offsets, with room for the leaf bases hung after the dress, sized from the existing made-count sizing (`given.rs`) with no extra readback or round trip; the sweep's readers (stations, rings, the device field) read it by the record's own layout, derived words in the narrowest table. If one buffer needs a readback the per-wave layout avoids, or costs measurably more peak memory, stop and report the numbers. No sweep-side copy of the waves.
- Finding 3, host decision (a): the sweep writes the renderer's packed form; the rings read frames from it; the measured spruce leaf move (1, 6, 10 of ~4M at seeds 1, 7, 42, at most 0.197 mm) is accepted. Re-pin the affected resident digest lines in BASELINE.md with the measured difference beside them; every other line stays equal. Compare stills of the spruce at seeds 1, 7, 42 before and after (at most four images per capture, no full forest) and report any visible difference. If the CPU executor now differs from the device on those leaves, note it for fn-289; build no second path.
- Small before large: first a probe on one preset (spruce, seed 1) that the change keeps `generation_digest`'s resident line; then all presets at seeds 1, 7, 42.
- Finding 2: list each of the 39 words (`sweep.wgsl:122-153`) as a copy of a drawn-record word (`CV=38`), an axis-record word (`CA=12`), or a derived value. Readers (`stations.wgsl`, `rings`, `field.wgsl:64,96-154,392,402`) read copied words from the drawn and axis records by the record's own layout; derived words stay in the narrowest table that holds them. Every reader moves in this task.
- Finding 3: the sweep writes points and clusters in the renderer's packed form (`CurvePoint::pack`, `packed_clusters`; the arithmetic in `handoff.wgsl` moves, including its `signum`), and folds the stretches, bounds and sag partials it reads back; delete `handoff.rs`, `handoff.wgsl`, the CPU-block upload (`handoff.rs:55-60`) and its registration in `resident.rs:204,244`. (The full-precision question is settled: host decision (a).)
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
- [ ] The dress writes one drawn-record buffer, sized by the made-count sizing with no extra readback; no 39-word node table and no handoff pass; every reader reads the drawn records or one narrow derived table.
- [ ] Resident digests equal the baseline except the re-pinned spruce leaf lines, each with its measured difference; core and pipeline digests equal.
- [ ] Spruce stills at seeds 1, 7, 42 compared before and after; device build time (spruce, oak, paired warm x16) and GPU peak bytes before and after in MEASURE.md; any CPU/device leaf difference noted for fn-289.
- [ ] Gate and `npm test` green; codex review SHIP.
## Done summary
Blocked:
NEEDS_HUMAN (2026-10-18, probe in BASELINE.md "Task 6 probe", commit a3b69dece):

Q3 (finding 3, sweep writes the renderer's packed curve form): the rings read the sweep's frames at full precision. Decoding them from the packed form moves spruce leaves only (1, 6, 10 of ~4M at seeds 1, 7, 42; largest 0.194-0.197 mm, one quantisation step; no rotation or scale change; curve lines unchanged). (a) accept the move; (b) keep bytes by having the sweep write the ring words itself while it holds full frames (ring buffer, ~76 MB on spruce, coexists with the node table, so the build peak rises); (c) another route.

Q2 (finding 2, the 39-word node table goes): the drawn records are not one array; the dress writes one buffer per lateral order plus the root's and the leaf bases', and stations already bind seven storage buffers. (a) the dress writes every wave into one buffer at aligned offsets (in cycle/, outside this task's Touches; room needed for leaf bases hung after the dress); (b) the sweep copies the waves into one buffer in their own layout plus a narrow offsets/derived table (still a copy, about the same memory as today).
## Evidence
- Commits:
- Tests:
- PRs:
