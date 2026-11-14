---
satisfies: [R1, R5, R6]
---
# fn-302-the-device-never-refuses-a-tree-for-its.1 A stepped-limit overflow rebuilds once at the bound

## Description
The retry itself (R5) and its proof (R6). A build refused only by stepped-limit flags begins again from its first cycle with exactly those limits pinned at their bound; a second refusal is the error it is today. Frame 41's two R1 tests go green.

**Size:** M
**Files:** `crates/telperion-render/src/generation/cycle/build_grow.rs`, `cycle/build_size.rs`, `cycle/gate.rs`, `cycle/census.rs`, a test beside `cycle/build_size.rs`' tests or in `generation/cpu_builds_tests.rs`
**Touches:** [crates/telperion-render/src/generation/cycle/build_grow.rs, crates/telperion-render/src/generation/cycle/build_size.rs, crates/telperion-render/src/generation/cycle/gate.rs, crates/telperion-render/src/generation/cycle/census.rs, crates/telperion-render/src/generation/cycle/tree.rs, crates/telperion-render/src/generation/cpu_builds_tests.rs, crates/telperion-render/src/generation/resident/digest_tests.rs, crates/telperion-render/src/generation.rs, crates/telperion-render/src/generation/zz_hier_probe.rs (deleted)]

### Approach
- Name which flags are stepped: one function from `Flag` to the `Limits` field it checks (`UnitRanges` units, `DrawRanges` draws, `DepositRanges` deposits, `Sweeps` sweeps, `HierGens` rounds, `HierPasses` passes, `UnitRounds` both units rounds); every other flag is not stepped. The checks are at `plan_units.rs:141-143`, `plan_lay.rs:185-188`, `plan_lay.rs:275-281`, `plan_hier.rs:94`, `plan_hier.rs:171`, `plan_units.rs:300`.
- A pinned set travels in `Sizes` (it is `Copy` and survives moves through `build_size::grown`'s `..n`); `limits()` sets each pinned field to `Limits::full(b)`'s value. Follow the test-only override already there (`LAYGENS`, `build_size.rs:205-230`) for a test-only override that forces a stepped limit short; a pin must win over it unless the test asks it not to (R6's error case).
- The refusal keeps its words (`build_size::refusal`) but the growth loop must know it was stepped-only without parsing text: carry it typed inside the cycle module (for example an internal stop type with `From<RenderError>` so `?` still works in `cycle`, `read`, `finish`), and turn it into today's `RenderError` where `grow_to` returns.
- Retry in `Growing::grow_to` (`build_grow.rs:351-370`), beside the margin retry in `Growing::within` (`build_grow.rs:275-295`): on a stepped-only stop at the first attempt, begin again (`Growing::begin`) from the sizes the attempt started from, with the stop's limits pinned, and grow to the same cycle; the refused build is dropped first. A second stop of any kind is the error. The retry is spent for the build's life, not per call: the growth path calls `grow_to` again and again on one paused build (`growth.rs:189-226`, to `to - 1` then `to`, then on later requests), so the spent retry lives in the build's own state that survives pauses and moves (for example the persistent pinned set: a build that has pinned anything never retries again), never in a local of one call. Both `grow_hinted` (`tree.rs:120`) and the growth path (`growth.rs:131, 222`) reach it.
- Correct the comment in `stepped` (`build_size.rs` around the `floors` call) that says the generations never pass `2 units - 1`, and the `limits`/`a_species_floor_holds_the_limits_up` doc lines that call the floor a bound; document the stepped limits as a cost hint whose overflow is retried (module docs of `build_size.rs` and `build_grow.rs` that say a build never begins again).
- A test-visible retry count (cfg(test), as `Grew::censuses` is) so tests and task 2's survey can see a retry happened.

### Investigation targets
**Required:**
- `crates/telperion-render/src/generation/cycle/build_grow.rs:186-211, 275-370, 440-460` — refusal, margin retry, grow loop, census read
- `crates/telperion-render/src/generation/cycle/build_size.rs:195-300` — `limits`, `stepped`, the floor comment
- `crates/telperion-render/src/generation/cycle/gate.rs:30-110` — `Limits`, `full`, `within`
- `crates/telperion-render/src/generation/cycle/census.rs:85-155` — `Flag`, `raised`
**Optional:**
- `crates/telperion-render/src/generation/cpu_builds_tests.rs:220-235` — `with_laygens` used through the generator
- `crates/telperion-render/src/generation/resident/digest_tests.rs:52-82` — `line`, what the renderer receives, to compare two builds

### Key context
- The limit a cycle records comes from the census two cycles back; cycles 1 and 2 record the full bounds.
- The scratch probe `crates/telperion-render/src/generation/zz_hier_probe.rs` and its `mod` line in `generation.rs` are the host's; never stage or commit them. Delete the file (`rm` its absolute path) and remove its `mod` line by editing before the first commit.
- Base digests and per-frame sweep digests from bb20f3877's code are in `.flow/evidence/fn-302-the-device-never-refuses-a-tree-for-its/raw/base-digests.log`.
- Heavy cargo runs only via `~/.local/bin/telperion-heavy 8 --threads 2 -- <cmd>`; the gate runs once at the end of the task.

## Acceptance
- [ ] The R1 tests (`the_film_walks_build_on_the_device`, `the_rounds_frames_have_the_cpus_counts`) pass; frame 41 builds by one retry.
- [ ] A test forces a stepped limit short through the generator, sees exactly one retry, and gets what the renderer receives byte-identical to a build with that limit pinned at its bound from its first cycle.
- [ ] A growth-path test (`GrowthState`) retries once, resumes to a later age, meets a second stepped overflow (the test-only override), and returns the named refusal without beginning again.
- [ ] A test keeps the forced limit short through the retry and gets the refusal naming the flag after exactly one retry; a non-stepped refusal (an existing refusal test) does not retry.
- [ ] No projection, `STEP`, `ROUND_SLACK`, floor, table cap or plan bound changes; the `2 units - 1` bound comment is corrected and the stepped limits are documented as a cost hint whose overflow is retried.
- [ ] The probe and its `mod` line are not in any commit.
- [ ] The gate (`cargo test --profile ci --workspace --no-fail-fast`) is green once.

## Done summary
A device build refused only by stepped-limit flags (UnitRanges, DrawRanges, DepositRanges, Sweeps, HierGens, HierPasses, UnitRounds) begins again once from its first cycle with exactly those limits pinned at their bound (`Growing::grow_to`, shared by the direct build and the growth path); the pins live in `Sizes`, so the retry is spent for the build's life. Any other flag, or a second overflow, refuses by name as before. Projections, STEP, ROUND_SLACK and floors unchanged; the `2 units - 1` bound comment corrected. Frame 41 builds by one retry and matches the CPU exactly. Host ruling: four rounds frames whose counts differ (bb20f3877's builds to the byte, no retry) moved to an ignored fn-303 test.

Tests: cycle::retry_tests (4), build_size, growth, cpu_builds, film_walk subset and counts pass. Gate once: 1,472 passed, 1 failed (every_device_preset_builds_the_same_leaves_each_time, device Out of Memory under desktop VRAM pressure; passes alone), 116 ignored.

stage: impl-review - ran (codex, 3 draws: correctness, contracts, integration, all SHIP) (model: gpt-6.1-sol)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: b20deba80baa087e375fad16944da2dae8d2a492, e98843a5c9e7fd28f6a8a5b06f179f8e49a19d04
- Tests: telperion-heavy 8 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast
- PRs: