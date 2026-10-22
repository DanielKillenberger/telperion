# The F4 build's obvious friction fixes

## Goal & Context

AGENTS.md "Friction reports" (owner, 2026-09-18 and 2026-09-23) says the host specs and builds every obvious fix: one whose cause the entry names and whose remedy changes no product behaviour and no owner decision. The F4 build's 58 FRICTION.md entries (fn-298, fn-299, fn-301, fn-302, fn-303, fn-304) fall into 14 clusters (host, 2026-10-22). Four have obvious fixes, collected here. The rest go to the owner, are local setup, or are fixed already.

## Requirements

- **R1 (gate-only guards run first):** fn-298 19:55 lost a 37-minute gate and fn-301 05:05 lost a gate 35 minutes in. Each failed on a sub-second guard that no narrow run includes. The fix:
  - In `.config/nextest.toml`, give the compile-size and generation-limit guard tests a priority override on the `gate` profile so they run first. Check the exact test names in the code; the profile already uses `priority` for the species suite.
  - Name both tests in AGENTS.md's Quick commands as the narrow run before any gate.
  - Proof: a gate listing shows both tests first.
- **R2 (`artifact_digest` takes an age):** fn-302's base digests cost 18 minutes and fn-301's R3 digests 25 minutes, because the example takes only `<preset> [seed]`. Add an optional age:
  - The usage becomes `artifact_digest <preset> [seed] [age]`.
  - The age sets the preset's age exactly as `build_time_tests`' `BUILD_AGE` does, through the same field.
  - A bad value errors, never defaults.
  - Proof: a digest at the preset's own age equals the digest without the argument.
- **R3 (`build_time` measures a walk frame):** fn-302's design survey and fn-298 15:37 timed frames by hand. Add a `BUILD_FRAME` option to `build_time_tests` that names a walk frame (from, to, t) by the same parse the film-walk tests use. Paired arms run interleaved in one batch, so the timings share the desk's load. No new build path: the frame builds through the pipeline exactly as the film-walk tests build it.
- **R4 (a worker never resets the review counter):** fn-301 04:00 recorded a worker resetting the review counter, against the host ruling of 2026-10-21. Add one line to AGENTS.md's Flow-Next section: a worker that reaches the review cap returns to its conductor, and only the host resets a counter. Name the ruling.
- **R5:** the gate (`scripts/gate.py` as AGENTS.md declares it) is green, `npm test` is green, and each task gets a codex review.

## Boundaries

- Test, example and docs only. No generator, renderer or preset change.
- Not the heavy queue (local), not fn-305's scope, and none of the owner-decision clusters.
