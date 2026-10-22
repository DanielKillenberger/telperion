---
satisfies: [R1, R2, R3, R4, R5]
---
# fn-306-the-f4-builds-obvious-friction-fixes.1 The guards run first, the digest takes an age, build_time times a walk frame, and the review-cap line

## Description
One task for the whole spec: four small edits in four files (test config, docs, one example, one ignored measurement test). One task means one gate, and that gate is R1's proof as well.

**Size:** M
**Files:** `.config/nextest.toml`, `AGENTS.md`, `crates/telperion-core/examples/artifact_digest.rs`, `crates/telperion-render/src/generation/build_time_tests.rs`
**Touches:** [.config/nextest.toml, AGENTS.md, crates/telperion-core/examples/artifact_digest.rs, crates/telperion-render/src/generation/build_time_tests.rs, .flow/evidence/fn-306-the-f4-builds-obvious-friction-fixes/**]

### Approach

- **R1, nextest.** Add a `[[profile.gate.overrides]]` entry with `priority` for `(binary(telperion_render) & test(=compile_size::tests::every_entry_inlines_under_the_ceiling)) | binary(generation_limit_guard)`. Keep it a separate override from the `test-group` ones (`.config/nextest.toml:28-65`): nextest resolves each setting from the first matching override, so a priority-only entry does not disturb the groups. The default profile's species override is `priority = 100` (`.config/nextest.toml:4-6`), and the gate profile inherits it. Check nextest 0.9.145's priority range before choosing values (`$CARGO_NEXTEST nextest show-config` or its docs); if 100 is the ceiling, add a gate-profile override that sets the species suite to 99 and the guards to 100, with a comment saying why.
- **R1, proof.** Show the order with a cheap filtered run under `--profile gate` (`--test-threads 1`, a filter holding the two guards, one or two species tests and a test whose binary sorts first), saved to `.flow/evidence/<spec>/raw/`, and confirm it again from the task's gate output. Write the result in the evidence summary.
- **R1, AGENTS.md.** One sentence in "Gates and checked claims" (around `AGENTS.md:39`) naming both tests and their narrow command (the spec's Quick commands) as the run before any gate.
- **R4, AGENTS.md.** One line under the `## Flow-Next` heading, placed after `<!-- END FLOW-NEXT -->` (`AGENTS.md:83`) so `/flow-next:setup` never overwrites it: a worker at the review cap returns to its conductor, and only the host resets a counter (host, 2026-10-21).
- **R2.** In `artifact_digest.rs:26-35`, parse a third optional argument as `f64` into `family.age` (`format!("no age {age}")` on a parse failure, as the seed does); call `family.validate()` (`crates/telperion-core/src/family.rs:91`) before `build`, so an age off the declared range (`family.rs:16-29`) is refused with the shared rule (`build` checks only whole years within `u32`, `pipeline/space.rs:162`); leave the whole-year check to the build; reject any argument after the age with the usage line. Update the doc line (`:7`) and the usage string (`:29`).
- **R3.** In `build_time_tests.rs`, read `BUILD_FRAME` as `<from>,<to>,<t>` before `test_gpu()`, check both names with `by_identity` (so an unknown name fails with its name, not `at`'s `expect`), require `BUILD_AGE` and `BUILD_SEED`, and build its family with `super::cpu_builds_tests::at` and `telperion_core::blend::families`, as `film_walk_tests.rs:49-56` does, wrapping a blend error with the frame named. Collect the arms first (frame plus `BUILD_PRESETS` presets; the frame alone when `BUILD_PRESETS` is unset), then run two rounds over all arms: round one is each arm's cold build, round two its warm build. Keep the per-build `drop` and device drain. A device refusal in `prepare` keeps today's refusal row, drops that arm from later rounds and lets the others carry on. Parse failures `panic!` with the value named (the test is `#[ignore]`d, so a panic is its error). Without `BUILD_FRAME`, preset options parse as today. Update the module doc (`:1-8`).

### Investigation targets

**Required** (read before coding):
- `.config/nextest.toml` — profiles and overrides
- `crates/telperion-render/src/generation/build_time_tests.rs` — the measurement
- `crates/telperion-render/src/generation/film_walk_tests.rs:17-60` — how a frame is built
- `crates/telperion-render/src/generation/cpu_builds_tests.rs:12-18` — `at`
- `crates/telperion-core/examples/artifact_digest.rs`

**Optional:**
- `scripts/gate.py:42-43,160-180` — how the gate invokes nextest
- `.flow/evidence/fn-298-a-walk-frame-builds-on-the-device/FRICTION.md:21-26,63-68`, `.flow/evidence/fn-301-a-tree-rests-on-its-ground/FRICTION.md:66-78` — the friction this answers

### Key context

- Every cargo build or test goes through `~/.local/bin/telperion-heavy 8 --threads 2 -- <cmd>`; the gate through `telperion-heavy 8 --threads 4 -- python3 scripts/gate.py`, once, at the end, with `CARGO_NEXTEST` set to the desk's 0.9.145 binary.
- This worktree has its own `target/` (cold). Never point at another checkout's.
- R3's run needs a GPU; one short `build_time` run with a frame and its two ends is enough proof, its table saved in the evidence directory.

## Acceptance
- [ ] Under the `gate` profile, both guards start before every other scheduled test, the species suite included; the filtered run's output and the gate's are cited in the evidence (R1).
- [ ] AGENTS.md "Gates and checked claims" names both guards and their narrow command (R1).
- [ ] `artifact_digest <preset> [seed] [age]`: the digest at the preset's own age equals the digest without it, another age differs, and `abc`, `-1`, `1000001`, `12.5` and an argument after the age each exit non-zero naming the problem (R2).
- [ ] `BUILD_FRAME=<from>,<to>,<t>` with `BUILD_AGE` and `BUILD_SEED` times the frame beside any `BUILD_PRESETS` arms, interleaved by round; a malformed frame, an unknown preset, a `t` outside [0, 1] or a missing age or seed fails naming the value, also with no device (R3).
- [ ] AGENTS.md's Flow-Next section carries the review-cap line, outside the managed snippet (R4).
- [ ] The gate under its declared invocation and `npm test` are green; the codex impl-review is SHIP (R5).


## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
