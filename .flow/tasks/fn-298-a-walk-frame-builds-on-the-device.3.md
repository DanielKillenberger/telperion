---
satisfies: [R3, R4, R5]
---
# fn-298-a-walk-frame-builds-on-the-device.3 Sweep the film's walk frames and check agreement and bytes

## Description
The sweep that would have caught it (R3) and the agreement checks (R4): build every frame of the film's walks at seeds 1, 7 and 42 and the Laurelin at 20 on the device; check each R1-fixed frame against the CPU to `agree_tests`' tolerances; and check the shipped presets' digests against the base. Ends with the gate and `npm test` (R5).

**Size:** M
**Files:** `crates/telperion-render/src/generation/cpu_builds_tests.rs` (or a new `film_walk_tests.rs` beside it if that file passes ~400 lines), `crates/telperion-render/src/generation/resident/agree_tests.rs` (only if the agreement check lives there), `.flow/evidence/fn-298-a-walk-frame-builds-on-the-device/RESULT.md`
**Touches:** [crates/telperion-render/src/generation/cpu_builds_tests.rs, crates/telperion-render/src/generation/film_walk_tests.rs, crates/telperion-render/src/generation/generation.rs, crates/telperion-render/src/generation.rs, crates/telperion-render/src/generation/resident/agree_tests.rs, .flow/evidence/fn-298-a-walk-frame-builds-on-the-device/**]

### Approach
- State the film's walk segments as a table (spec R3) and generate frames as `Path::frames` does: `n = round(seconds * 30)`, `t = ease((i + 1) / n)` for i in 0..n. Build each with `telperion_core::blend::families(&a, &b, t)` from `at(id, age, seed)` (`cpu_builds_tests.rs:9`), on the device alone (see `refused_on_device`, `cpu_builds_tests.rs` ~166-200).
- Time the full sweep once (510 frames x 3 seeds). If it is too slow for the workspace command (judge against the gate's current wall time; state the number), the ordinary test runs a stated subset including every R1 frame and the first and last frames of each walk at each seed, and the full sweep is an `#[ignore]` test named in RESULT.md with its result.
- R4 agreement: for each R1-fixed frame, compare device and CPU with the comparison `agree_tests` uses (`resident/agree_tests.rs`: `drawn()`, `device_nodes`, its tolerances); reuse its helpers rather than restating tolerances.
- R4 bytes: run `DIGEST_SEEDS=1,7,42` `digest` (`resident/digest_tests.rs`, `#[ignore]`) on this branch and on the base (a separate worktree or the recorded `fn-293` BASELINE.md if it matches the base commit; check it does first) and diff. Write `space_hash` lines into RESULT.md per `docs/evidence-retention.md`.
- Run `npm test` and the gate once at the end.

### Investigation targets
**Required:**
- `crates/telperion-render/src/generation/cpu_builds_tests.rs`
- `crates/telperion-render/src/generation/resident/agree_tests.rs:1-200`
- `crates/telperion-render/src/generation/resident/digest_tests.rs`
**Optional:**
- `.flow/evidence/fn-293-each-stages-output-has-one-form-and-its/BASELINE.md`
- `crates/telperion-render/examples/space_hash.rs`

### Key context
- Never share `target/` with another checkout; a base digest run in a second worktree uses its own target.
- A digest difference not traced to the limits stops `NEEDS_HUMAN` (spec R4 Errors).
## Acceptance
- [ ] A device test builds the film's walk frames at seeds 1, 7 and 42 and the Laurelin at 20, all succeeding except the frames R3 excludes for fn-299 (oak to palm refused on `FRAMES`, and frames 76–77 at seed 42 if shown to share the cause); each excluded frame is an ignored test naming fn-299. It runs on the workspace command, whole or as a stated subset holding every R1 frame and the 9 `rounds` frames, with the full sweep's result in RESULT.md.
- [ ] Each R1-fixed frame (and the 9 `rounds` frames) agrees between CPU and device to `agree_tests`' tolerances.
- [ ] The digest at seeds 1, 7 and 42 matches the base byte for byte, recorded in RESULT.md with `space_hash` lines.
- [ ] The gate and `npm test` are green.
## Done summary
R3: `film_walk_tests.rs` holds the film's segment table at 30 fps, eased. The full sweep is an ignored test: 1,489 frames built, 0 refused. The ordinary test builds a 48-frame subset: each walk's first and last frame at each seed, the 9 `rounds` frames and the Laurelin at 20. fn-299's 44 oak-to-palm frames are one ignored test naming fn-299. Frames 76–77 at seed 42 are excluded on an inferred cause, as the host accepted.

R4: all 9 `rounds` frames hold instances within 1 of the CPU. 8 of them hold exact node counts. Spruce to birch, seed 42, frame 56 is one node more on the device, the same with every limit at its bound. Its node check is an ignored test naming fn-300 (host, 2026-10-21; recorded in the spec). Resident digest at seeds 1, 7 and 42 against the spec base `7422b110b`, built in its own target: 24 of 24 lines identical.

R5: gate 1,410 passed, 0 failed, 115 ignored. `npm test`: 175 passed.

stage: impl-review - ran (codex, one correctness reviewer, round 1: SHIP) (model: gpt-6.1-sol)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 584ea94157a1e300a68b9468c6ef962f0c7d7b29, bc0066e078dd60614b00940f1bf1485f13bcf4fa, 0e9b4f2e121025c80abbf3edbe7f97d138404ad6
- Tests: telperion-heavy 8 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast, telperion-heavy 8 --threads 2 -- npm test, telperion-heavy 8 --threads 2 -- cargo test --profile ci -p telperion-render --lib film_walk -- --include-ignored --nocapture, DIGEST_SEEDS=1,7,42 cargo test --profile ci -p telperion-render --lib generation::resident::digest_tests::digest -- --exact --ignored (branch and base 7422b110b)
- PRs: