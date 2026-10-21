---
satisfies: [R3, R4, R7, R8]
---
# fn-302-the-device-never-refuses-a-tree-for-its.2 Survey the stepped limits and record the retries

## Description
The committed survey (R7), R3's byte comparison against bb20f3877, the retry report and frame 41's retry cost (R8), `npm test` (R4).

**Size:** M
**Files:** a new ignored test file under `crates/telperion-render/src/generation/` (registered in `generation.rs` beside `film_walk_tests`), `generation/film_walk_tests.rs` (share its sweep), `cycle/build_grow.rs` (a cfg(test) census tap), `.flow/evidence/fn-302-the-device-never-refuses-a-tree-for-its/RESULT.md`
**Touches:** [crates/telperion-render/src/generation.rs, crates/telperion-render/src/generation/film_walk_tests.rs, crates/telperion-render/src/generation/*survey*.rs, crates/telperion-render/src/generation/cycle/**, crates/telperion-render/src/generation/resident/digest_tests.rs, .flow/evidence/fn-302-the-device-never-refuses-a-tree-for-its/**]

### Approach
- The tap: a cfg(test) record of every census a build on this thread reads, by cycle (the read in `Growing::read`, `build_grow.rs:445-459`, already keeps `grew.censuses` under cfg(test)); summarise it inside `cycle/` where the census words are visible, as the host's scratch did (`raw/survey-*.log`).
- Per stepped limit: need(c) is what its flag compares (rounds: `GENS`; passes: `build_size::passes`; units rounds: `rounds_used`; units, draws, deposits ranges and sweeps: the counts their flags divide by the range or `LAYERS`); limit(c) is the census's `REC` word. Print each limit's largest need(c) − need(c−2) and least limit(c) − need(c), with where; and the builds that retried. Set: the film-walk sweep (share `film_walk_tests::sweep`) and every shipped preset at its default age and at 12 and 40 where younger, seeds 1, 7, 42. An env var prints one digest line a build (`digest_tests::line`) for R3.
- Run it once over the whole set; compare its digest lines and `resident::digest_tests::digest` / `palm_digest` with `raw/base-digests.log` (same format, `SWEEPDIGEST <name>: <line>`); frames bb20f3877 refused are listed, not compared.
- Frame 41's retry cost: a paired, alternating timing in one process of the frame as it builds (retrying) against the frame with its stepped limits pinned at the bound from the first cycle (task 1's override); may be a scratch measurement, never committed if it is.
- `RESULT.md`: the survey's table, retries, R3 result, frame 41's timing.

### Investigation targets
**Required:**
- `crates/telperion-render/src/generation/film_walk_tests.rs:1-130` — the sweep and its frames
- `crates/telperion-render/src/generation/resident/digest_tests.rs:1-130` — digest lines
- `.flow/evidence/fn-302-the-device-never-refuses-a-tree-for-its/raw/survey-units.log` — the scratch survey's output shape
**Optional:**
- `.flow/evidence/fn-298-a-walk-frame-builds-on-the-device/RESULT.md` — prior RESULT shape

### Key context
- The full sweep plus presets is about 15 minutes; run it once (budget rule). Raw logs go in `raw/` (gitignored).

## Acceptance
- [ ] An ignored survey test prints each stepped limit's largest rise and least headroom with where, and the retrying builds, over R7's set.
- [ ] Every preset digest (seeds 1, 7, 42) and palm digest line equals `raw/base-digests.log`; every sweep frame bb20f3877 built has its base digest line; every frame builds.
- [ ] `RESULT.md` records the survey, the retry count and retrying builds, R3's result and frame 41's paired retry timing.
- [ ] `npm test` passes; the gate is green once.

## Done summary
An ignored survey test (`stepped_survey`) taps every census a device build reads and prints each stepped limit's most rise and least headroom, with where, and the builds that began again, over the walk frames and the presets at their default age and at 12 and 40 where younger; SURVEY_DIGEST prints digest lines, SURVEY_ONLY filters. RESULT.md records: one build of 1,605 began again (frame 41, seed 1); all 1,532 frames bb20f3877 built have equal digest lines and frame 41 builds; the 34 preset and palm digest lines are equal; frame 41's retry costs 43.8 ms (317.7 against 273.9 ms). 145 Laurelin to Telperion frames refused by the VRAM-following device budget in the first run built on a rerun with a free GPU.

Tests: stepped_survey (full set and rerun), digest and palm_digest, npm test (177 passed). Gate once: 1,473 passed, 0 failed, 117 ignored.

stage: impl-review - ran (codex, 1 draw: correctness, SHIP) (model: gpt-6.1-sol)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: e8d33b3ce037ba8a131dec7b6ad97d116c5b3b9e, 642d03478ed5de50df21606c2f1aec871513cbb4
- Tests: telperion-heavy 8 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast, npm test
- PRs: