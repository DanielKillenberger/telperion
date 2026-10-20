# A walk frame builds on the device

## Goal & Context

The owner (2026-10-20): "we need to fix beech->spruce". It is part of the 0.2.0 release (fn-237).

The device build (`Generator::prepare`, the resident build) refuses frames of the beech to spruce walk. fn-272 logged each case in its FRICTION.md, on 2026-10-14 and again at cut 4:

| Frame | Refusal |
|---|---|
| Age 40, seed 7, t = 0.00037 | "the build passed its sizes at cycle 9: UnitRounds (0x1)" |
| The same at t = 0.01 and t = 0.1, and at seed 1 | the same |
| Age 40, seed 7, t = 0.2160 (eased) | "LayGenerations (0x1)" |
| The Laurelin at age 20, on its own | "Given (0x1)" |

The frames t = 0.5 and t = 0.9 build, and so do both ends of the walk. Because of the refusals, the film builds its walks with `mesh::build` on the CPU instead. That costs about 2.9 s a frame at age 40 and about 32 minutes a cut, against roughly 0.2–0.6 s a frame on the device.

The cause, from the code at `ee2fe86b5`:
- `cycle/build_size.rs` `limits()` projects every limit that steps with the tree from the census, which is read a cycle late. Those limits are the hierarchy's generations and passes, the layout's generations, the sweep's layer ranges and the units rounds.
- Each projection adds a fixed slack: `STEP` and `ROUND_SLACK`. That slack was measured only on shipped presets at their default ages, at seeds 1, 7 and 42 (fn-256 task 12 and fn-276).
- A walk frame is a legal tree whose rows lie between two presets. Its counts can rise faster in two cycles than any shipped preset's do, so it outruns the slack and is refused by name.

Nobody has yet run this as a test. The cause above is read from the code, and R1 checks it.

## Acceptance Criteria

- **R1:** (repro first; AGENTS.md "Gates and checked claims")
  - Each refusal in the table above runs as a standalone device test on the base, before any fix.
  - Record which of them reproduce, and the census value that passed its limit against the limit it passed.
  - A refusal that does not reproduce is recorded as such and is not tested further.
  - Errors: a device without an adapter skips (`test_gpu()` returns none) and the row is recorded "not run", never "does not reproduce". The `Given` row comes from the host's dispatch bound (`given.rs`), not from the stepped limits; its census word and bound are recorded the same way.
- **R2:** (question, then delete; `docs/principles.md`) first measure what the projection buys.
  - Measure the build time of the shipped presets at their default ages two ways: with the stepped limits as they are, and with every stepped loop recorded at its bound (`Limits::full`). Record the result. Wherever a loop that has nothing left to do already ends early, the projection may not buy anything.
  - If the cost of building at the bound is within noise, or under 5% of build time, delete the projection for those limits.
  - If it is not, derive the limit from the job itself instead of from presets measured at their default ages. Examples are the job's own `floors()`, or a bound taken from its rows. Whichever way is chosen, a legal walk frame can no longer outrun its limit.
  - The refusal by name stays for a count past a real table or past the budget. The build never falls back to a second path and is never begun again.
  - The A/B uses the warm figure of `build_time` (each preset's second build in the process), every shipped preset at its default age and seed, three runs each way, and compares the total and each preset. "Within noise" means the full-limits run's delta is inside the spread of the stepped runs.
  - Errors: a measurement within 1 percentage point of the 5% line, or one where the total and a single preset disagree on the side of the line, is borderline: the run stops `NEEDS_HUMAN` with the numbers. The same goes for a `Given` repro whose fix the measure-then-choose rule above does not settle.
- **R3:** (the sweep that would have caught it)
  - A device test builds the frames of the film's walks (`Path::frames`, eased, as fn-272's cut takes them) at seeds 1, 7 and 42, and the Laurelin at age 20. Every one of those builds succeeds.
  - It runs on the workspace test command as an ordinary test. If it is too slow for that, it runs a stated subset that includes every frame that failed in R1, and the full sweep is named in the evidence.
  - The frames are the film's own: each walk segment of `release.json` (beech to spruce, spruce to birch, birch to oak at 40 for 3 s; oak to palm at 40 for 4 s; palm to Laurelin at 1 for 1 s; Laurelin to Telperion at 80 for 3 s) at 30 fps, `t = ease((i + 1) / n)` with `ease` the smoothstep `Path::frames` uses. The test states the segment table; it does not import fn-272's example.
  - Errors: a refused frame fails the test with its name, seed, `t` and the refusal text.
- **R4:** (agreement)
  - Each frame fixed under R1 agrees between the CPU and the device, to the tolerances `agree_tests` already holds.
  - The meshes of shipped presets at their default ages are byte-identical to the base. The limits only change how many loops are dispatched, never what a loop writes.
  - Errors: a digest line that differs from the base fails R4 unless the difference is traced to a written word (a counter or census word a loop now records), in which case the run stops `NEEDS_HUMAN`: R4 states the limits write nothing.
- **R5:** the gate and `npm test` are green, and each task gets a codex review. Errors: no error surface beyond the gate's own failures.

## Boundaries

- This spec does not route the film's walks back to the device. That belongs to fn-272, once this spec is merged.
- It does not change any preset or any public API.
- It does not cover the walk whose species does not move reusing its grown structure, another of fn-272's friction proposals.
- No switch between stepped and full limits survives the build; the R2 A/B runs on a scratch patch kept under `raw/`.
- Every GPU or workspace cargo run goes through `telperion-heavy 8 --threads 2 -- <cmd>`; a wait past 30 minutes is a FRICTION entry and `NEEDS_HUMAN`.

## Quick commands

```bash
telperion-heavy 8 --threads 2 -- cargo test --profile ci -p telperion-render --lib -- cpu_builds_tests --nocapture
CYCLE_TRACE=1 telperion-heavy 8 --threads 2 -- cargo test --profile ci -p telperion-render --lib -- fn298 --ignored --nocapture
telperion-heavy 8 --threads 2 -- cargo test --profile ci -p telperion-render --lib build_time -- --ignored --nocapture
DIGEST_SEEDS=1,7,42 telperion-heavy 8 --threads 2 -- cargo test --profile ci -p telperion-render --lib digest -- --ignored --nocapture
```

## Early proof point

Task fn-298-a-walk-frame-builds-on-the-device.1 validates the core approach (which refusals reproduce, which limit each passes, and what building at the bound costs). If no refusal reproduces, the fix tasks shrink to the R3 sweep alone; if the bound costs past the line, re-evaluate the job-derived limit before continuing with .2+.

## Strategy Alignment

Active tracks served by this plan:
- **The core and integration** — the device build is the one generation core's fast path; a legal tree it refuses breaks the contract that every tree builds through one pipeline.
- **The catalogue** — walk frames are the points between species; the space is continuous only if every point builds.

## Decision Context

- The film's frame list is restated as a table in the test rather than imported from fn-272's `examples/film`: that code is not on this base, and an example is not a library the render crate's tests may depend on.
- The existing fn-276 tests (`cpu_builds_tests.rs`) already cover t = 0.00037, 0.01 and 0.1 near the beech and the Laurelin at 20 at seed 7; R1 runs them first and adds only the rows they do not cover (t = 0.2160 eased).
## Requirement coverage

| Req | Description | Task(s) | Gap justification |
| --- | --- | --- | --- |
| R1 | (repro first; AGENTS.md "Gates and checked claims") | fn-298-a-walk-frame-builds-on-the-device.1 | — |
| R2 | (question, then delete; `docs/principles.md`) first measure what the projection buys. | fn-298-a-walk-frame-builds-on-the-device.1, fn-298-a-walk-frame-builds-on-the-device.2 | — |
| R3 | (the sweep that would have caught it) | fn-298-a-walk-frame-builds-on-the-device.3 | — |
| R4 | (agreement) | fn-298-a-walk-frame-builds-on-the-device.3 | — |
| R5 | the gate and `npm test` are green, and each task gets a codex review. Errors: no error surface beyond the gate's own failures. | fn-298-a-walk-frame-builds-on-the-device.2, fn-298-a-walk-frame-builds-on-the-device.3 | — |

