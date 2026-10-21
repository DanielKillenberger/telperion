---
satisfies: [R2, R5]
---
# fn-303-the-device-and-the-cpu-grow-the-same.1 A span's share is 1 where the length reaches its end, on both executors

## Description
The CPU's span position (`stand::node_x`) and the device's mirror (`attach` in `lay_stand.wgsl`) pick a span by `before + scale >= length` and then divide, so a length exactly at the span's summed end reads 1 − δ on the CPU (oak to palm frame 36, relay 978: 0.9999992) where the device reads 1. R5's rule makes the comparison that chose the span also decide the share. This task also removes every fn-303 probe-only edit before its first commit (R1's measurement is done; RESULT.md and `raw/` hold it).

**Size:** M
**Files:** `crates/telperion-space/src/geometry/stand.rs`, `crates/telperion-space/src/geometry/tests.rs`, `crates/telperion-render/src/generation/cycle/lay_stand.wgsl`, `crates/telperion-render/src/generation/cycle/settle_tests.rs`, plus the probe removals listed below
**Touches:** [crates/telperion-space/src/geometry/**, crates/telperion-space/src/grow/cycle.rs, crates/telperion-space/src/grow/cycle/capture_grown.rs, crates/telperion-space/src/lib.rs, crates/telperion-render/src/generation/cycle/**, crates/telperion-render/src/generation/cycle.rs, crates/telperion-render/src/generation/resident.rs, crates/telperion-render/src/generation/resident/fn303_probe_tests.rs, crates/telperion-render/src/generation/film_walk_tests.rs, crates/telperion-core/src/pipeline/foliage.rs, crates/telperion-core/src/pipeline/foliage/fn303_probe_tests.rs]

### Approach
- In `stand.rs:166-176`, give the share one named function that both executors call by the same name, for example `span_share(length, before, scale)`.
  - It returns exactly 1.0 when `length >= before + scale`, else `(length - before) / scale`, keeping today's clamp.
  - `node_x` keeps its span pick and fall-through unchanged.
  - Its doc comment names the fold: the left fold of the phytomers' f32 scales from the base, the same one `total` (`length_of`, `stand.rs:150-152`) runs.
- In `lay_stand.wgsl:267-281` (`attach`), call a WGSL function of the same name with the same rule.
  - A comment names the same fold. `tip` is `summed` from `neg0()` (`lay_stand.wgsl:84-90`) and the loop's `before` starts at 0.0; the values are equal (−0 + x = x).
  - Keep plain `+`; `before + scale` must be the exact f32 sum on both sides.
  - `settle_thick.wgsl:59-66`'s fall-through stays.
- CPU unit test in `geometry/tests.rs`: a hand-built axis (pattern: `girth/tests.rs:224-292`). Choose scales whose `(total - before) / last_scale` rounds below 1 in f32; the test asserts that it does, so it can fail.
  - `node_x(axis, total(axis))` gives `(last, 1.0)`.
  - A length 1 ulp below `total` keeps the computed share (< 1).
  - A length at an interior node's summed end gives `(k, 1.0)`.
- Device unit test in `settle_tests.rs`, following `a_step_straight_down_rests_along_its_side_on_the_device` (`settle_tests.rs:138-170`): slice the WGSL share function out of `LAY_STAND`'s source and run it on the same values as the CPU test. The device's result equals the CPU's bit for bit.
- Measure on the final share rule, while the R1 probes are still in the tree (they are uncommitted):
  - Frame 36: run the ignored `fn303_the_four_rounds_frames_have_the_cpus_counts` with `--nocapture`, and read frame 36's printed instance line (157 against 160 before). Nodes stay apart by the rachis tail until task 2.
  - Growth census (R2, R4): run `generation::cycle::fn303_probe_tests::fn303_where_the_four_frames_part` with `--ignored --nocapture`. On each of the four frames, the device census and `grow_census` must hold equal axes and phytomers grown at every cycle from 1 to 40. Save the log as `raw/t1-census.log` and record the result in RESULT.md. Task 2 changes no generator code, so this census stands for the spec.
  - Relay survey: run `fn303_relay_joins` (presets, seeds 1, 7 and 42) as `raw/t1-relay-survey.log`. Copy its source into the scratchpad for task 2.
- Then, before the first commit, remove the probe-only edits and confirm `git diff --stat` shows none left:
  - `grow_capture_grown_at`'s `pub` and its re-exports: `telperion-space/src/grow/cycle/capture_grown.rs:30`, `grow/cycle.rs:24`, `lib.rs:61`.
  - `stepped_survey::drain`.
  - `film_walk_tests::frame`'s visibility.
  - The module lines in `generation/cycle.rs`, `generation/resident.rs` and `telperion-core/src/pipeline/foliage.rs`.
  - The three `fn303_probe_tests.rs` files.

### Investigation targets
**Required** (read before coding):
- `crates/telperion-space/src/geometry/stand.rs:150-211`: `length_of`, `node_x`, `total`, `split`.
- `crates/telperion-space/src/geometry.rs:277-297`: `attach` and its doc.
- `crates/telperion-render/src/generation/cycle/lay_stand.wgsl:84-90,267-300`: `summed`, `attach`, `stand_at`.
- `crates/telperion-render/src/generation/cycle/settle_thick.wgsl:45-72`: `attach_main`.
- `crates/telperion-render/src/generation/cycle/settle_tests.rs:138-170`: the probe-kernel pattern.

**Optional:**
- `crates/telperion-core/src/pipeline/space.rs:431-447,540`: the CPU's start, inside and end join tests.
- `crates/telperion-render/src/generation/cycle/dress.wgsl:243-253,698-712`: `at_start`, `at_end`, `next_join`.
- `.flow/evidence/fn-303-the-device-and-the-cpu-grow-the-same/RESULT.md`: the frame 36 trace.

### Key context
- `attach` also places every relay's base (`geometry.rs:227` with `stand::at`; `lay_frame.wgsl:184`), so growth's layout moves by last bits wherever a relay stands at a span's end. That is expected (AGENTS.md "Generator evolution"); task 2 measures it.
- `zone_node` (`lay_stand.wgsl:102`) and the sag's assignment (`settle_assign.wgsl:125`) have the same pick shape. They decide no join and are out of scope.

## Acceptance
- [ ] No probe-only edit remains: `git diff 6198fbc9a --stat` shows only this task's files and no `fn303_probe_tests.rs` (R1's probes removed before the first commit).
- [ ] CPU unit test: a length equal to the summed tip, over scales whose `(tip - before)/scale` rounds below 1 (asserted in the test), gives `(last, 1.0)`. A length 1 ulp below keeps share < 1. An interior node's end gives `(k, 1.0)` (R5).
- [ ] Device unit test: the same inputs give the same share on the device, bit for bit (R5).
- [ ] Both sides name one fold order in a comment and share one function name (R5).
- [ ] The four frames' growth census on the final share rule holds equal axes and phytomers grown at every cycle from 1 to 40, recorded in RESULT.md (R2, R4).
- [ ] Oak to palm, seed 1, frame 36: device instances within 1 of the CPU's, read from the ignored fn-303 test's output and recorded in RESULT.md (R2).
- [ ] The gate `cargo test --profile ci --workspace --no-fail-fast` is green, run once at the end of the task.

## Done summary
A span's share is 1 where the length reaches its end, on both executors.

- `stand::span_share` (CPU) and `span_share` in `lay_stand.wgsl` (device) decide the whole span by the comparison that chose it. `node_x` and `attach` call them. Both name the left fold of the f32 scales.
- Unit tests:
  - CPU: `geometry::tests::a_length_at_a_span_end_is_the_whole_span`.
  - Device: `cycle::settle_tests::a_length_at_a_span_end_is_the_whole_span_on_the_device`.
  - They use scales whose division reads the tip as 0.9999994.
- Measured on the change, with the probes still in the tree:
  - Oak to palm frame 36 has 157 instances on both executors, and all 20,205 of its device nodes match.
  - The four frames' growth census is equal at every cycle from 1 to 40.
  - The relay survey moves 1,281 end joins from 1 − δ to exactly 1; 24 joins short of their span's end stay below 1.
- Then the R1 probes were removed.
- The gate's one failure was `walk_sizes`' oak-to-birch exception: +94 kept phytomers from the moved layout. That row is re-measured (host ruling 2026-10-22), and `walk_sizes` passes.

stage: impl-review - ran (codex, three reviewers: correctness, contracts, integration; SHIP, no findings) (model: gpt-6.1-sol)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 89ceb5ebc436c396b9d23d30f46be659fb50553c, b2c433cafa12b61de748b3342a15dfa053351bfc, f85378805bb1736a710df698b4c29184c8960ba0
- Tests: telperion-heavy 12 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast (1474 passed, 1 failed: walk_sizes exception re-measured), cargo test --profile ci -p telperion-core --test walk_sizes (pass), cargo test --profile ci -p telperion-space --lib a_length_at_a_span_end (pass), cargo test --profile ci -p telperion-render --lib a_length_at_a_span_end (pass)
- PRs: