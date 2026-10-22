# The F4 build's obvious friction fixes

## Goal & Context

AGENTS.md "Friction reports" (owner, 2026-09-18 and 2026-09-23) says the host specs and builds every obvious fix: one whose cause the entry names and whose remedy changes no product behaviour and no owner decision. The F4 build's 58 FRICTION.md entries (fn-298, fn-299, fn-301, fn-302, fn-303, fn-304) fall into 14 clusters (host, 2026-10-22). Four have obvious fixes, collected here. The rest go to the owner, are local setup, or are fixed already.

Who it affects: the agents and conductors who build in this repository. Nothing a user of the generator sees changes.

## Acceptance Criteria

- **R1 (gate-only guards run first):** fn-298 19:55 lost a 37-minute gate and fn-301 05:05 lost a gate 35 minutes in. Each failed on a sub-second guard that no narrow run includes. The two guards are the compile-size ceiling (`compile_size::tests::every_entry_inlines_under_the_ceiling` in `telperion-render`'s library tests) and the generation-limit inventory (`generation_limit_guard`, `telperion-core`'s integration test binary). The fix:
  - The `gate` profile in `.config/nextest.toml` gives both guards (the compile-size test and every live test in the `generation_limit_guard` binary) a priority override that starts them before every other test. The species suite already carries `priority = 100` from the default profile; if 100 is nextest's ceiling, the `gate` profile also ranks the species suite one step below the guards, so the guards never rest on a tie.
  - AGENTS.md names both tests, with the command that runs them, as the narrow run before any gate. AGENTS.md has no "Quick commands" section (checked, 2026-10-22); the line goes in "Gates and checked claims", beside the gate's declared invocation.
  - Proof: a filtered run under the `gate` profile at one thread, holding the guards, a species test and a test whose binary lists first, starts the guards first; the task's gate run shows them among its first results.
  - Errors: no error surface beyond nextest's own config parse. A later rename of a guard silently drops its priority; guarding that is out of scope.
- **R2 (`artifact_digest` takes an age):** fn-302's base digests cost 18 minutes and fn-301's R3 digests 25 minutes, because the example takes only `<preset> [seed]`. It takes an optional age:
  - The usage becomes `artifact_digest <preset> [seed] [age]`, in the module doc and the usage error.
  - The age sets `Family::age`, the field `build_time_tests`' `BUILD_AGE` sets.
  - Proof: a digest at the preset's own age equals the digest without the argument, and a digest at another age differs.
  - Errors: an age that does not parse as a number exits non-zero with a message naming the value, never a default. An age the family refuses (off `Family::age`'s range, or not a whole number of years) is refused by the pipeline's own check, and the example exits non-zero with that error; the example adds no second range rule. An argument after the age is a usage error. A bad seed keeps its existing error.
- **R3 (`build_time` measures a walk frame):** fn-302's design survey and fn-298 15:37 timed frames by hand. `build_time_tests` takes a `BUILD_FRAME` option naming one walk frame as `<from>,<to>,<t>`:
  - The frame's family is built as the film-walk tests build theirs: each end preset at the run's age and seed (`cpu_builds_tests::at`), blended by `telperion_core::blend::families` at `t`, then prepared by the same `Generator::prepare` call every other row of `build_time` uses. No new build path.
  - A frame needs an age and a seed, which come from `BUILD_AGE` and `BUILD_SEED`; with `BUILD_FRAME` set, both are required and apply to every arm.
  - The run's arms are the frame plus any presets `BUILD_PRESETS` names; with `BUILD_FRAME` set and no `BUILD_PRESETS`, the frame runs alone. The arms run interleaved in one batch, round by round (every arm's first build, then every arm's second), so their timings share the desk's load. "Cold" stays an arm's first build and "warm" its second. Each build is dropped and the device drained before the next, as today. The table keeps one row per arm, the frame labelled `<from> to <to> t <t>`.
  - The frame is parsed before the device is looked for, so a bad option fails on a desk without one too.
  - Errors: a `BUILD_FRAME` that is not three comma-separated fields, names a preset that is not shipped, or has a `t` that does not parse, or a missing or unparseable `BUILD_AGE` or `BUILD_SEED` beside it, fails the test with a message naming the value, never a default. A `t` the blend refuses (outside [0, 1]) fails with the blend's error and the frame named. A frame the device refuses in `prepare` is a measurement, not an error: it keeps today's refusal row, leaves the later rounds, and the other arms carry on.
  - Without `BUILD_FRAME`, the preset rows keep today's options and parse.
- **R4 (a worker never resets the review counter):** fn-301 04:00 recorded a worker resetting the review counter, against the host ruling of 2026-10-21. AGENTS.md's Flow-Next section gains one line, in the "Dispatch and escalation" section's terms: a dispatched agent that reaches a review's round cap returns to the session that dispatched it, and only the host session conducting the run resets a counter (host, 2026-10-21). The line sits under the `## Flow-Next` heading, outside the managed flow-next snippet, so setup never rewrites it. The line is consistent with the owner's 2026-10-09 ruling that the host resets the cap in autonomous runs without asking (`.flow/memory/knowledge/conventions/reset-review-round-caps-without-asking-2026-10-09.md`). Errors: none (a docs line).
- **R5:** the gate (`telperion-heavy 8 --threads 4 -- python3 scripts/gate.py`, as AGENTS.md declares it) is green, `npm test` is green, and each task gets a codex review. Errors: a red gate or `npm test` blocks the task.

## Early proof point

Task fn-306-the-f4-builds-obvious-friction-fixes.1 is the whole build; its gate run is R1's proof that the two guards start first. If they do not (nextest's priority range or override order does not behave as the plan assumes), re-evaluate the override before the task closes.

## Boundaries

- Test, example and docs only. No generator, renderer or preset change.
- Not the heavy queue (local), not fn-305's scope, and none of the owner-decision clusters.
- `BUILD_AGE`, `BUILD_SEED` and `BUILD_PRESETS` keep their present behaviour for preset rows; only `BUILD_FRAME` is new.
- No test-only limits override for `build_time` (fn-302's other ask): it would reach into the generator's limits, which is not an obvious fix.

## Strategy Alignment

_No active strategy track served — review for drift._ This is build-loop friction, not a track: it cuts the gate failures and hand-timed runs that slow every track's specs.

## Decision Context

- **Forward dependency on fn-304 not recorded.** R1 and R5 build on fn-304's `gate` profile and `scripts/gate.py`, which this branch's base (1418ffebb) already holds through the merge at 935c80f29. fn-304's spec is still open, so a recorded edge would hold fn-306 behind a closure that changes nothing here.
- **fn-305 overlap.** fn-305 reworks the same `gate` profile. Whichever merges second keeps the other's entries; fn-306 merging first lets fn-305's re-measurement include the guards' priority.
- **No walk-frame parse exists.** The film-walk tests address a frame by segment, seed and frame number and build it from `at` and `blend::families`. R3 reuses that build and adds only the `<from>,<to>,<t>` parse for the option.
- **Priority above the species suite.** The default profile's species override already uses `priority = 100`, which may be nextest's ceiling; if it is, the `gate` profile ranks the species suite just below the guards rather than lifting the guards past a ceiling. A tie would leave the order to the listing.
- **One age rule.** `Family::age` already refuses an age off its range or not whole, by name; the digest's age reaches that check instead of adding its own.
- **One task.** Four small edits in four files make one M task, so the spec pays for one gate, and that gate is also R1's proof.

## Quick commands

```bash
# The two guards, the narrow run before any gate (R1)
~/.local/bin/telperion-heavy 8 --threads 2 -- cargo test --profile ci -p telperion-core --test generation_limit_guard
~/.local/bin/telperion-heavy 8 --threads 2 -- cargo test --profile ci -p telperion-render --lib compile_size
# R2: the same digest with and without the preset's own age
~/.local/bin/telperion-heavy 8 --threads 2 -- cargo run --profile ci -p telperion-core --example artifact_digest -- european-beech 1 | sha256sum
# R3: a walk frame beside its two ends
BUILD_FRAME=european-beech,norway-spruce,0.5 BUILD_AGE=40 BUILD_SEED=1 BUILD_PRESETS=european-beech,norway-spruce \
  ~/.local/bin/telperion-heavy 8 --threads 2 -- cargo test --profile ci -p telperion-render build_time -- --ignored --nocapture
```

## Requirement coverage

| Req | Task(s) |
| --- | --- |
| R1 | fn-306-the-f4-builds-obvious-friction-fixes.1 |
| R2 | fn-306-the-f4-builds-obvious-friction-fixes.1 |
| R3 | fn-306-the-f4-builds-obvious-friction-fixes.1 |
| R4 | fn-306-the-f4-builds-obvious-friction-fixes.1 |
| R5 | fn-306-the-f4-builds-obvious-friction-fixes.1 |
