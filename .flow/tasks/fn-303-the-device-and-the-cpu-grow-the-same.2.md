---
satisfies: [R2, R4, R6, R7, R8]
---
# fn-303-the-device-and-the-cpu-grow-the-same.2 The rounds frames compare like with like, and the change is measured

## Description
Applies host rulings 1 and 2 to the count tests, puts all 9 `rounds` frames back under the ordinary test (fn-300's two apart), and measures the output change task 1 made. It ends the spec with the gate and `npm test`.

**Size:** M
**Files:** `crates/telperion-render/src/generation/cpu_builds_tests.rs`, `crates/telperion-render/src/generation/film_walk_tests.rs`, `.flow/evidence/fn-303-the-device-and-the-cpu-grow-the-same/RESULT.md`
**Touches:** [crates/telperion-render/src/generation/cpu_builds_tests.rs, crates/telperion-render/src/generation/film_walk_tests.rs, .flow/evidence/fn-303-the-device-and-the-cpu-grow-the-same/**]

### Approach
- Ruling 1 (R6): in `apart` (`cpu_builds_tests.rs:175-212`), count and match the CPU's nodes only up to `Tree::clothing` (`telperion-core/src/tree.rs:180-185`; `None` means every node).
  - `nodes_off` (`film_walk_tests.rs:223-229`) and both `cpu_builds_tests` callers (`:141`, `:157`) read it.
  - A comment names fn-283: the device draws the rachis in another pass, as `leaf_bases` already explains.
- Ruling 2 (R7): dissolve `FN303` and its ignored test (`film_walk_tests.rs:145-170`).
  - The rounds test filters out fn-300's frames 56 and 63.
  - Frame 63 gets its own ignored test beside `fn300_the_spruce_to_birch_frame_56_has_the_cpus_nodes` (`:207-219`), checking nodes and instances. Its reason names fn-300 and the counts measured on this task's code.
  - Fold `counts_off` back into the rounds test only if nothing else uses it.
- The relay survey: re-add task 1's scratchpad copy of `fn303_relay_joins` as an ignored test only if it reads public API alone (`telperion_space::grow`, `Axis::joined`) and stays cheap to keep. Otherwise run it from the scratchpad and log it in FRICTION.md as proposed tooling.
  - Run it on the presets at seeds 1, 7 and 42. Record joins at t == 1 before (from `raw/r1-relay-survey-presets.log`'s run, if counted) and after, and the joins left with t in [1 − 1e-4, 1).
  - Those stand below their span's end by `span_share`'s construction (R8).
- Digest (R8): run `digest` (default seeds 1, 7 and 42) and `palm_digest` on this code. Diff against `raw/base-digests.log` (6198fbc9a, captured 2026-10-22), and write the table to RESULT.md with each changed row's cause:
  - a relay at its parent's tip;
  - a join at an interior node's end;
  - layout drift from either.
  Stills (`DIGEST_STILLS`) only if a preset's default-age leaves change visibly. Otherwise give one changed row's numbers.
- Growth census: task 1 measured it on the final share rule (`raw/t1-census.log`). This task changes no generator code, so it is not re-run.
- Finish: the gate once, then `npm test` once, then RESULT.md's R2, R4 and R8 lines.

### Investigation targets
**Required:**
- `crates/telperion-render/src/generation/cpu_builds_tests.rs:135-212`
- `crates/telperion-render/src/generation/film_walk_tests.rs:140-229`
- `crates/telperion-core/src/tree.rs:180-185,223-224`
- `crates/telperion-render/src/generation/resident/digest_tests.rs:84-136`

**Optional:**
- `.flow/evidence/fn-302-the-device-never-refuses-a-tree-for-its/RESULT.md`: the digest-table shape.

### Key context
- `raw/` is gitignored. RESULT.md cites file names, and the tables live in RESULT.md.
- At most one 1,024-tree capture per commit (AGENTS.md); none is needed here.

## Acceptance
- [ ] `apart` counts the CPU tree up to `Tree::clothing`, with a comment naming fn-283. Oak to palm frames 36, 37 and 48 have exact node counts (R6).
- [ ] Frame 63 sits in its own ignored test whose reason names fn-300 and its counts measured on this code. No fn-303 ignored test remains (R7).
- [ ] `the_rounds_frames_have_the_cpus_counts` is green over the 9 frames but fn-300's 56 and 63: nodes exact, instances within 1 (R2).
- [ ] RESULT.md holds the digest table against 6198fbc9a with each changed row's cause, the stills decision, the relay survey's counts on the change, and fn-300's facts (R4, R8).
- [ ] The gate and `npm test` are green, each run once (R4).

## Done summary
The rounds frames compare like with like, and the change is measured.

- `apart` counts and matches the CPU tree only up to `Tree::clothing`, the leaf bases and fn-283's frond axes the device draws in a pass of its own (ruling 1). Oak to palm 36, 37 and 48 have exact node counts.
- `FN303` is dissolved (ruling 2):
  - Spruce to birch, seed 7, frame 63 sits in the ignored `fn300_the_spruce_to_birch_frame_63_has_the_cpus_counts`, with its counts on this code: 187,997 / 187,999 instances, 382,987 / 382,999 nodes.
  - `the_rounds_frames_have_the_cpus_counts` is green over the other 8 frames.
- The relay survey is kept as the ignored `relay_join_tests::relay_joins`.
- Digests against 6198fbc9a:
  - The ordinary tree, the oak and the birch change hashes at every seed, with equal counts.
  - The oak at seed 1 rebuilds the base's line exactly on the old rule, and its leaves move at most 0.21 mm. No stills are needed.
  - Every other row is unchanged.
- The gate: 1,475 passed, 0 failed. `npm test`: 177 tests passed.

stage: impl-review - ran (codex, one reviewer: correctness; SHIP, no findings) (model: gpt-6.1-sol)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 96a67939f37e35177b9ac20bafffab62f901a079, 2416619ebf500c9256298ba3252966ea021cdee6
- Tests: telperion-heavy 12 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast (1475 passed, 0 failed), npm test (177 passed), cargo test --profile ci -p telperion-render --lib -- --include-ignored --exact generation::film_walk_tests::the_rounds_frames_have_the_cpus_counts (pass)
- PRs: