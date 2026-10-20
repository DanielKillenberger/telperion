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
  - The "at its bound" run sets only the stepped limits (`rounds`, `passes`, `laygens`, `sweeps`, `unit_rounds`) to their bounds; the list ranges (`units`, `draws`, `deposits`) and the hierarchy's folding (`top`, `tail`) stay as they are, so the run measures exactly the change the delete branch would make. If the combined run passes the line, each stepped limit is measured at its bound alone, and the rule is applied per limit.
  - The A/B uses the warm figure of `build_time` (each preset's second build in the process), every shipped preset at its default age and seed, three runs each way, and compares the total and each preset. "Within noise" means the full-limits run's delta is inside the spread of the stepped runs.
  - Errors: a measurement within 1 percentage point of the 5% line, or one where the total and a single preset disagree on the side of the line, is borderline: the run stops `NEEDS_HUMAN` with the numbers. The same goes for a `Given` repro whose fix the measure-then-choose rule above does not settle.
  - **Ruled (host, 2026-10-20)**, on R2.md's measurement (all five at the bound +32.9% in total; each alone: `rounds` +2.8%, `passes` +2.4%, `sweeps` +3.6%, `unit_rounds` +4.6%, `laygens` +15.0%):
    1. A per-preset regression over 5% counts against deletion. Building at the bound costs the beech, birch and palm 5–15% on `rounds`, `passes` and `sweeps`, and no frame failed on them, so `rounds`, `passes`, `sweeps` and `unit_rounds` keep their projection as it is. The R3 sweep still covers them; any of them that fails there gets the same treatment as `laygens`, below.
    2. **Revised (host, 2026-10-20)**, `laygens`: an empty generation costs nothing, rather than a guess at how many there will be.
       - First measure where `laygens`'s +15.0% at the bound comes from: what one layout generation pass costs when no axis sits at that generation. The expected answer is that each pass is dispatched over the whole set of new axes and filters by depth, so an empty pass costs as much as a full one. Confirm or refute this from the code and a trace before changing anything.
       - If confirmed: each generation pass is sized from the device's own count of axes at that generation, written by the pass before it, through an indirect dispatch, so a generation with no axes dispatches 0 workgroups. `laygens` records every pass up to its bound (the plan's cap of 15); the projection and `STEP[2]` are deleted for `laygens`; `LayGenerations` is still refused by name past the cap. ("Question, delete, then optimise": the limit was only ever a cost saving.)
       - If step one refutes the premise, or an indirect dispatch cannot express it: a per-cycle limit, `laygens = min(cap, unit_rounds[0] + unit_rounds[1] + 1)`. It rests on an unchecked claim, that a units round adds at most one generation to a chain, relays included; before relying on it, a device test asserts that claim on the t = 0.2160 frame and on every shipped preset, and its cost is measured in a paired run. If that cost passes 5% too, the run stops `NEEDS_HUMAN` with the numbers.
       - Acceptance: the t = 0.2160 test is green; a paired `build_time` run (a base run beside each change run, as R2.md's second batch) shows the shipped presets within noise of the base; meshes are byte-identical; CPU = device on the fixed frame.
       - Why the first ruling (`deepest + slack(job, two cycles)`) was dropped: it assumed layout depth accumulates across cycles. In the code each cycle's depth stands alone: a new axis's generation counts only ancestors born in the same cycle (`generation_main`), and the depth marks are cleared every cycle, so `deepest` is one cycle's depth two cycles back and nothing adds to it (task .2's reading, R2.md).
       - **Ruled (host, 2026-10-20), after step one confirmed the premise** (an empty pass costs a floor of about 32 µs plus a share of the cycle's new axes; bound costs beech +10.4%, birch +12.1%, palm +18.6%, spruce +34.6%; R2.md "Task .2, step 1"): the indirect branch is out, since DESIGN.md §7a's ban on indirect dispatch stands and the floor would miss noise anyway; a device-side early stop cannot be built while every dispatch is recorded ahead. Option (d): `laygens` becomes a cost hint, not a correctness limit.
         - First read the seven dispatches of a generation pass and name, with its line, every cross-axis read when laying an axis (siblings, a prefix sum or scan over the generation, collision or space queries against same-generation axes, atomics shared across axes).
         - If laying an axis is chain-local: one closing pass after the last recorded generation lays each new axis deeper than `laygens − 1`. One invocation walks up to its deepest laid ancestor and lays that chain top-down with the same functions in the same order of float operations, so the result is bit-identical to the per-generation path. Two deep axes sharing an unlaid ancestor never both write it: either one writer per chain, chosen by the chain's head, or the shared ancestors laid in a fixed order; the evidence says which. `LayGenerations` stays only for the plan's cap of 15. `STEP[2]` stays as the hint that keeps the per-generation passes cheap. Acceptance: the t = 0.2160 test is green; CPU = device on that frame; the shipped presets' meshes are byte-identical (the tail pass is empty on them); a paired `build_time` run is within noise (one extra direct dispatch per cycle); a unit test forces `laygens` to 1 on a shipped preset and gets the same mesh as the stepped build, the proof that the tail pass is exact.
         - **Host (2026-10-20), after (d) was built** (exact: `laygens` = 1 byte-identical, digest 24/24, paired `build_time` +0.7%; the t = 0.2160 frame then refused `HierGens` at cycle 10, `rounds` 3 against census `gens` 4): keep (d). Before fixing limits one at a time, survey: build every R3 frame (the film's walks, eased, seeds 1, 7, 42, and the Laurelin at 20) with `rounds`, `passes`, `sweeps`, `unit_rounds` at their bounds under an uncommitted scratch diagnostic, and record limit × frame × cycle where the census passes the stepped projection, and by how much. For each limit that fails, read its loop as the layout was read and name every cross-axis read with its line (`plan_hier.rs` for rounds and passes; the sweep layer ranges; the units rounds), with a cost guess. If only `rounds` fails and a hierarchy generation is chain-local, build its closing pass as (d) (acceptance: `rounds` forced to 1 on a shipped preset byte-identical, digest matches, paired `build_time` within noise) and continue to task .3; otherwise the run stops `NEEDS_HUMAN` with the table. Re-run the silver birch alone in one more paired batch and report whether it is still past its band.
         - **Host rulings (2026-10-20), after the survey** (R2.md: `rounds` past its projection on 9 frames, +1; `unit_rounds[0]` on 2 frames, +1; 42 oak-to-palm frames at t > 0.96 refused by `FRAMES` = 11; hierarchy placement not chain-local):
           1. `rounds`: every failing frame is a `laygens` frame one cycle later, so the layout's depth leads the hierarchy's generations by a cycle. First test, on every survey frame and every shipped preset, that the hierarchy generations at cycle c are at most the laid depth at cycle c−1 (recorded exactly since (d)) plus a constant k; give k from the data and say whether k = 0 holds. If it holds, `rounds`' floor comes from the latest read census's laid depth (a cycle fresher than `gens`), `STEP[0]` stays, and any constant the data does not need is dropped. Acceptance: the 9 frames build; paired `build_time` within noise; digest unchanged. If it fails, the run stops `NEEDS_HUMAN` with the counterexample.
           2. `unit_rounds[0]` and `FRAMES`: all fail at the palm end of oak to palm while the palm at 40 builds. Before touching a limit, read the most units per cycle on one apex and `unit_rounds[0]` at t = 0, 0.5, 0.9, 0.95, 0.962, 0.99, 0.99998 and 1, seeds 1, 7, 42, and the palm alone at 40. If frames go outside the range their two ends span, it is a defect in how the walk blends rows, not a limit to raise: find the blended row that causes it from the walk code (`blend`) and report it without fixing (it changes walk output and may need its own spec). If the palm end itself reaches 12 at some seed, report that; raising `FRAMES` is then the likely fix, for the host to rule.
           3. Build (1) first. Task .3 waits until (2) is settled.
         - **Host rulings (2026-10-20), after the k reading** (k = 0 on 67,380 rows; `limits()` reads census c−2, so a laid-depth floor still refuses 8 of 9 frames; the oak-to-palm refusals are a walk-blend defect):
           1. `rounds`: the floor is chosen offline from the recorded rows, with no GPU time on candidates. Candidates: (i) depth(c−2) + 3; (ii) a trend projection as the lists' `ahead` projects them, depth(c−2) + 2·max(rise, 1), rise = depth(c−2) − depth(c−3); (iii) max(gens(c−2) + `STEP[0]`, (ii)); and any variant the data suggests with no new constant. For each: the frames it still refuses, and total recorded rounds over the shipped presets against the stepped base as a cost proxy. Pick zero misses and the fewest recorded rounds, preferring no new constant; build it; acceptance: paired `build_time` within noise, digest unchanged, the 9 frames build. If no zero-miss candidate is within noise, the run stops `NEEDS_HUMAN`. The chosen rule and its data are written here.
              - **Chosen (task .2, 2026-10-20; R2.md "the `rounds` floor chosen from the rows")**: `rounds = max(gens(c − 2) + STEP[0], 2 units − 1)`, the units rounds' floor `floors()[0]` in place of `floors()[1]`. No new constant. Over 68,916 recorded cycle rows (1,536 frames), it refuses 0 frames, against the base's 9. It adds +1.5% recorded rounds over the shipped presets: 4,462 against 4,396. Of the other candidates, (i) also refuses 0 at +27.3%; (ii) refuses 3 at +2.4%; (iii) refuses 1 at +5.2%. No trend in the census reaches oak to palm, seed 1, i 48, cycle 40, whose depth reads 1, 1, 1 and then 4. On every frame whose floor the rows show, `deep(c) ≤ 2 units − 1` holds by at least 1. That is a fit to the data; no mechanism is proven. Measured on the change: the 9 frames build; digest 24 of 24 identical; paired `build_time` +0.1% in total over six pairs (base band 59 ms), every preset within its band except the palm (+1 ms against 1), whose limits the rule leaves unchanged. Open: on the t = 0.2160 frame the device's tree is not the CPU's (0.342 m apart). The base shows the same gap on that walk's frames that built (0.080 m at t = 0.1), so that half of its test waits on the host.
           2. The oak-to-palm blend defect is its own spec (fn-299, captured by the host). R3's sweep excludes the oak-to-palm frames refused on `FRAMES`, and the two `unit_rounds[0]` frames if they are shown to share the cause; each excluded frame gets an ignored test naming fn-299. Every other R3 frame builds.
         - If laying is not chain-local: (d) is not built. Instead read each preset's per-cycle `unit_rounds` and test the one-generation-per-round claim, and the run stops `NEEDS_HUMAN` with the numbers before building anything.
    3. The refusal by name stays.
- **R3:** (the sweep that would have caught it)
  - A device test builds the frames of the film's walks (`Path::frames`, eased, as fn-272's cut takes them) at seeds 1, 7 and 42, and the Laurelin at age 20. Every one of those builds succeeds.
  - It runs on the workspace test command as an ordinary test. If it is too slow for that, it runs a stated subset that includes every frame that failed in R1, and the full sweep is named in the evidence.
  - The frames are the film's own: each walk segment of `release.json` (beech to spruce, spruce to birch, birch to oak at 40 for 3 s; oak to palm at 40 for 4 s; palm to Laurelin at 1 for 1 s; Laurelin to Telperion at 80 for 3 s) at 30 fps, `t = ease((i + 1) / n)` with `ease` the smoothstep `Path::frames` uses. The test states the segment table; it does not import fn-272's example.
  - Errors: a refused frame fails the test with its name, seed, `t` and the refusal text.
  - Excluded (host, 2026-10-20): the oak-to-palm frames refused on `FRAMES` (frames 106–119 at every seed, t ≥ 0.962), and the two `unit_rounds[0]` frames (oak to palm, seed 42, frames 76–77) if shown to share their cause, a walk-blend defect owned by fn-299. Each is an ignored test naming fn-299.
- **R4:** (agreement)
  - Narrowed (host, 2026-10-20): each fixed frame (the t = 0.2160 frame and the 9 `rounds` frames) has equal node counts on the CPU and the device, and instance counts within 1. Agreement of positions on walk frames is not this spec's subject (a frame builds) and goes to fn-300, after the release: on t = 0.2160 both sides have 275,557 nodes, all matched, but positions are up to 0.342 m apart and radii up to 0.035 m against `agree_tests`' 1e-3 and 1e-5; the gap is on the base already (t = 0.1: 0.080 m, t = 0.5: 0.072 m) and is identical with every stepped limit at its bound, so the limits do not cause it. The position check stays an ignored test naming fn-300.
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

