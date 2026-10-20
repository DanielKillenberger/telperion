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

## Requirements

- **R1 (repro first; AGENTS.md "Gates and checked claims"):**
  - Each refusal in the table above runs as a standalone device test on the base, before any fix.
  - Record which of them reproduce, and the census value that passed its limit against the limit it passed.
  - A refusal that does not reproduce is recorded as such and is not tested further.
- **R2 (question, then delete; `docs/principles.md`):** first measure what the projection buys.
  - Measure the build time of the shipped presets at their default ages two ways: with the stepped limits as they are, and with every stepped loop recorded at its bound (`Limits::full`). Record the result. Wherever a loop that has nothing left to do already ends early, the projection may not buy anything.
  - If the cost of building at the bound is within noise, or under 5% of build time, delete the projection for those limits.
  - If it is not, derive the limit from the job itself instead of from presets measured at their default ages. Examples are the job's own `floors()`, or a bound taken from its rows. Whichever way is chosen, a legal walk frame can no longer outrun its limit.
  - The refusal by name stays for a count past a real table or past the budget. The build never falls back to a second path and is never begun again.
- **R3 (the sweep that would have caught it):**
  - A device test builds the frames of the film's walks (`Path::frames`, eased, as fn-272's cut takes them) at seeds 1, 7 and 42, and the Laurelin at age 20. Every one of those builds succeeds.
  - It runs on the workspace test command as an ordinary test. If it is too slow for that, it runs a stated subset that includes every frame that failed in R1, and the full sweep is named in the evidence.
- **R4 (agreement):**
  - Each frame fixed under R1 agrees between the CPU and the device, to the tolerances `agree_tests` already holds.
  - The meshes of shipped presets at their default ages are byte-identical to the base. The limits only change how many loops are dispatched, never what a loop writes.
- **R5:** the gate and `npm test` are green, and each task gets a codex review.

## Boundaries

- This spec does not route the film's walks back to the device. That belongs to fn-272, once this spec is merged.
- It does not change any preset or any public API.
- It does not cover the walk whose species does not move reusing its grown structure, another of fn-272's friction proposals.
