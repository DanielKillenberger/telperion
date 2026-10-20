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
- [ ] A device test builds the film's walk frames at seeds 1, 7 and 42 and the Laurelin at 20, all succeeding; it runs on the workspace command, either whole or as a stated subset holding every R1 frame, with the full sweep's result in RESULT.md.
- [ ] Each R1-fixed frame agrees between CPU and device to `agree_tests`' tolerances.
- [ ] The digest at seeds 1, 7 and 42 matches the base byte for byte, recorded in RESULT.md with `space_hash` lines.
- [ ] The gate and `npm test` are green.
## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
