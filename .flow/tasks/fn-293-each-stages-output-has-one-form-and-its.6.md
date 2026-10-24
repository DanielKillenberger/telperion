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
The dress now writes every wave into one drawn-record buffer. Records sit by id; after them comes room for the retained leaf bases, then the axes by slot, and a record's axis word holds its slot. The sweep, stations, field, boots, folds and stems read that buffer. The 39-word node table, `gather` and the sweep-only field path are deleted. The sweep writes the renderer's packed points and clusters (`pack.wgsl`) and folds the demand and box while each frame is whole. `handoff.rs`/`handoff.wgsl` and their registration are deleted, and the rings decode their frames from the packed points (host decision (a)).

- Sizing: the buffer is sized from the chains' wave counts, which the plan already reads back, so no readback was added. The host named `given.rs`, but that file sizes the cycle's dispatches, not the dress. The bases' room is two records a base on every axis of the seed's wave (`boots::most`: the count's floor, plus one where it has a share). A hang that needs more room than that is refused by name (`leaf base nodes`).
- Digests (BASELINE.md, "After task 6"): core 24 of 24 equal. The render pipeline is byte-identical (`fb4ce911...`). Resident: 21 of 24 lines are equal. The three spruce lines are re-pinned. In them only the leaves hash moved: 1, 6 and 10 leaves move by at most 0.194, 0.197 and 0.195 mm, with no rotation or scale change, which is the probe's measurement. Points, clusters, sections and demand are unchanged, and every tree's box is equal to the base's (24 of 24, `DIGEST_BOUNDS`).
- Stills: the spruce at seeds 1, 7 and 42 is pixel-identical before and after.
- For fn-289: the CPU reference (`prepare_whole_resident`) did not match the device leaf for leaf before this change either. At seeds 1 and 7 its counts differ by 2 and every leaf index differs. The move is device-side, and no second path was built.
- MEASURE.md, 16 paired warm builds on a busy card: spruce 446 to 427 ms (paired median -15 ms, faster in 15 of 16), oak 236 to 231 ms (-5 ms, faster in 13 of 16). GPU peak bytes are equal on every preset before and after, because the growth cycle sets the peak.
- Tests: `the_device_packs_a_frame_as_the_cpu_does` moved to `pack_tests.rs` and now exercises `pack.wgsl`. The digest test gained the `DIGEST_STILLS`, `DIGEST_BOUNDS` and `DIGEST_CPU` switches.
- Gate (`telperion-heavy 12 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast`): 1400 passed, 1 failed, 111 ignored. The failure was `boots_tests::every_leaf_base_is_hung_as_the_cpu_hangs_it`: the test read a stem apex's key by its wave-local index. The fix is commit 4b9175efb, and the focused rerun passed. `npm test`: 170 passed. Baseline: green (render lib tests built at the base). The wasm32 lib check is clean.
- Follow-ups:
  - `src/generation.rs:11` still says "handoff" (outside Touches).
  - The browser budget's `PEAKS` table (`budget.rs`) was measured before this change. It is not re-measured here, and the peaks did not move.

Tier: not reported (model: claude-opus-5-5)

stage: impl-review - ran (codex, three draws: correctness, contracts, integration, all SHIP with no findings)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 2f437cbd4b1dfab3f428289af23bc7ac12171b55, 534a2fb649ae7b9c32bdfdd2da8f1d3bf770b691, 4b9175efb905011e024b07f9d61ed352c706e9c6
- Tests: telperion-heavy 12 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast, cargo test --profile ci -p telperion-render --lib boots_tests, npm test, resident digest_tests::digest (DIGEST_SEEDS=1,7,42 DIGEST_BOUNDS=1)
- PRs: