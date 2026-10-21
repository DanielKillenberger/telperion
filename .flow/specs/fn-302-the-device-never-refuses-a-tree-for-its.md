# The device never refuses a tree for its hierarchy rounds

## Goal & Context

After fn-299, fn-298 and fn-301 merged into F4 (bb20f3877), the gate fails two tests on one frame. `the_film_walks_build_on_the_device` and `the_rounds_frames_have_the_cpus_counts` both refuse beech to spruce at 40, seed 1, frame 41 of 90 (t 0.4335089163237312): "the build passed its sizes at cycle 12: HierGens (0x1)". The release film's walk passes through this frame, so this spec joins the release (fn-237).

Measured by the host's probe (2026-10-21; uncommitted `zz_hier_probe.rs` in `.worktrees/f4-hier`, logs `bb20.log`, `bb20-cpu.log`, `c3ed.log`, `fn298.log` in the host scratchpad):

- **The tree is sound.** The CPU's hierarchy-generation count for the cycle-12 step is 4 (322 new axes). It is 4 on fn-298's tip too, and every other cycle reads 1 or 2. Nothing passes the plan's bound `b.hier.gens = max(GENS = 8, units)`.
- **Only the device's stepped projection is short.**
  - The projection is `rounds = max(read[GENS] + STEP[0], floors(units)[0])` (`cycle/build_size.rs:269`). `read` is the census two cycles back, which is 1, and `STEP[0]` is 2, so the stepped term is 3. The floor is `2·units − 1` (`cycle/build_arena.rs:127-129`).
  - The check that raises the flag is `GENS > rec::ROUNDS` (`cycle/plan_hier.rs:94`).
- **fn-299 exposed it; it did not cause it.** fn-299's rule (timing rows take the reaching end's values) drops the frame's `most_units` from 3 to 2. The floor therefore falls from 5 to 3. The ends each have `most_units` 2, yet the frame's step needs 4. So `2·units − 1` is not a bound on the generations one step adds. It covered this frame only while the old blend inflated `units`.

## Unknown, check before the plan (answered 2026-10-21, `raw/u1-cost.log`, `raw/survey-*.log`)

- What a hierarchy round costs when it has nothing left to do. Does the pass stop once no depth changes, or does it run every round it is sized for?
- Whether any other stepped limit relies on `most_units` in the same way.
- **Answer 1:** a round past the step's generations dispatches no workgroups, but the host still records every round within the limit, each with its passes' empty dispatches, scans, clears and tallies: about 20 to 33 µs a round-pass.
- **Answer 2:** the units rounds take the same `2·units − 1` floor and refuse by `UnitRounds`; over 70,284 censuses their projection alone falls short on 246 and their limit less their need is 0 on 812. The layout's generations take `units` as a floor but refuse only past the plan's cap.

## Design (host, 2026-10-21, after R2's measurement)

The bound is not free. Sizing the rounds at `b.hier.gens` costs +7.8% on beech 40, +12.4% on palm 12 and +1.7% on spruce 40, about 20 to 33 µs per idle round-pass of host recording. Deletion is therefore out.

The projection reads the wrong quantity. A cycle's generations are the layout's depth the cycle before (gens(c) = deep(c−1)). The census the host reads already carries that layout depth (`c::LAY_DEEP`, which `stepped` counts as `deepest` for `laygens`), and it is one cycle fresher than `c::GENS`. On frame 41 it reads 2 where `GENS` reads 1, and the cycle needs 4.

- **The rule.** `rounds` steps from the census's layout depth, not its `GENS`: `deepest + STEP[0]`.
  - `STEP[0]` is the largest rise from that read to the generations the cycle actually needs.
  - That rise is measured over every census of the full film-walk sweep (1,533 frames) and every shipped preset at its R1 ages, at seeds 1, 7 and 42. The measurement is recorded in `RESULT.md` with its maximum and where it occurs.
  - First verify gens(c) = deep(c−1) against the census on those builds. If it does not hold, stop and report.
- **The floor.** `2·units − 1` is no longer called a bound. The comment in `stepped` that claims "never past `2 units - 1`" is corrected, because frame 41 disproved it. The floor stays only if the sweep needs it with the new projection; otherwise it goes from `rounds`. `unit_rounds` keeps its floor.
- **`unit_rounds`.** No change. Report its measured headroom on the same sweep, the largest of `rounds_used(c) − rounds_used(read)` against `ROUND_SLACK`. If the headroom is 0 anywhere, stop and report.
- **Cost.** A paired timing of beech 40, palm 12 and spruce 40, base bb20f3877 against the change, must come out within noise.


## Design revised (host, 2026-10-21, after the census survey)

The survey covered 70,284 censuses across 1,587 builds (`raw/survey-both.log`, `raw/survey-units.log`). It confirms gens(c) = deep(c−1) everywhere. It also shows that the fresher read gains nothing: gens(c) − deep(c−2) peaks at 3, the same as gens(c) − gens(c−2). The units rounds sit at zero headroom on 812 censuses, held up only by the `2·units − 1` floor.

No projection from a census two cycles old is a bound, and a margin taken from a survey has zero headroom at its own maximum. The film samples a continuous walk at 1,533 points, the package builds any t, and fn-300 may shift these counts. So the stepped limits are a cost hint, not a guarantee, and the guarantee comes from the bound.

- **The rule.** When a device build is refused only because it passed stepped limits (the flags that name a stepped limit, such as `HierGens` and `UnitRounds`, never a table cap or a plan bound), the build runs once more from the start, through the same pipeline, with exactly those limits pinned at their bound.
  - A second refusal is an error, as today.
  - The rounds only size work, so the retried build is byte-identical to a build recorded with those limits at their bound.
  - This is the one pipeline sized larger, not a fallback path.
  - The retry lives in one place, where the device build turns its flags into a refusal. The plan checks for an existing retry or resize loop on the device path and uses it if there is one.
- **The projections are unchanged.** `STEP`, `ROUND_SLACK` and the floors stay as they are. The cost of a build that never overflows is therefore exactly today's. The `2·units − 1` comment that claims a bound is corrected, and the stepped limits are documented as a cost hint whose overflow is retried.
- **Proof.**
  - Frame 41 builds.
  - A test forces a stepped limit short (a test-only override, as `LAYGENS` has) and shows that the retried build is byte-identical to one recorded at the bound.
  - R3's digests and the full sweep are byte-identical to bb20f3877 wherever bb20f3877 built.
  - The full sweep and the shipped presets at their R1 ages report how many builds retried and which ones. Frame 41's paired timing records what a retry costs.
- **No timing pair for the non-retrying trees.** Their recording is unchanged, so their cost is unchanged.

Commit the survey tap as an ignored test that prints each stepped limit's rise and headroom, because the friction entry asks for it. The probe itself is not committed.


## Requirements

- **R1 (repro):** the two failing tests above are red on bb20f3877, and they are the repro. Run them before anything changes.
- **R2 (question, delete, then optimise):**
  - First try deleting the projection: size the hierarchy rounds at their bound, `b.hier.gens`. Measure the cost with paired `build_time` runs on beech 40, palm 12, spruce 40 and this frame. If it is within noise, the stepped projection and its floor for HierGens go.
  - If the bound costs more than noise, the projection must take its step from what can actually happen, never from `most_units`. Then bring the measurement to the host before choosing a rule.
- **R3 (output unchanged):** rounds only size work. Every artifact digest of the shipped presets at seeds 1, 7 and 42 is byte-identical to bb20f3877, and so is the full film-walk sweep (`every_film_walk_frame_builds_on_the_device`, 1,533 frames). Every frame builds, and none is refused.
- **R4:** the gate is green once, with `npm test`, and a codex review on each task.
- **R5 (one retry at the bound):** a device build refused only by flags that name a stepped limit runs once more from its first cycle, through the same pipeline, with exactly those limits pinned at their bound; the stepped limits are the cycle's `Limits` (units, draw and deposit ranges, sweeps, hierarchy rounds and passes, units rounds), named by `UnitRanges`, `DrawRanges`, `DepositRanges`, `Sweeps`, `HierGens`, `HierPasses` and `UnitRounds`. The retry serves every caller of the device's growth (the direct build and the growth path) from the one place a build's flags become a refusal. Projections, `STEP`, `ROUND_SLACK` and the floors are unchanged; the comment calling `2·units − 1` a bound is corrected, and the stepped limits are documented as a cost hint whose overflow is retried. Errors: a refusal that raises any other flag (a table cap, the budget, a fault, `LayGenerations` past the plan's cap, `Given`, `Points`, `Compaction`, light errors) is not retried and reads as today; a retried build refused again is an error naming its flags, as today; no build retries twice, across every call that grows one build (a growth paused and resumed included).
- **R6 (the retried build is the bound's build):** a test-only override forces a stepped limit short (as `with_laygens` forces the layout's generations); the build retries once and hands the renderer exactly what a build with that limit pinned at its bound from its first cycle hands it. Errors: an override that keeps the limit short through the retry ends in the refusal by name after exactly one retry.
- **R7 (survey):** an ignored test taps every census a device build reads and prints, for each stepped limit, its largest rise over the census two cycles back and its least headroom (the cycle's limit less its need), each with where it occurs, and the builds that retried, over the film-walk sweep and the shipped presets at their default age and at 12 and 40 where younger, seeds 1, 7 and 42. The probe `zz_hier_probe.rs` is never committed.
- **R8 (report):** `RESULT.md` records the retry count and the retrying builds over R7's set, frame 41's retry cost from a paired timing (the frame retrying against the frame built at the bound from its first cycle), and R3's comparison.

## Boundaries

- No change to growth, the blend or fn-299's rule.
- Other stepped limits change only if the Unknowns find the same `most_units` reliance. A finding there is brought to the host first.
- No change to a projection, `STEP`, `ROUND_SLACK`, a floor, a table cap or a plan bound (Design revised). No timing pair for builds that do not retry: their recording is unchanged.
- `Given` (a dispatch given too few workgroups) is not a `Limits` field and is not retried; a `Given` refusal is brought to the host.

## Strategy Alignment

Active tracks served by this plan:
- **The core and integration** — the device builds every tree the film's walk and the package ask for, with the CPU's tree, so the release (fn-237) is not refused on a frame.

## Decision Context

- Deleting the projection (rounds at the bound always) costs +7.8% beech 40, +12.4% palm 12, +1.7% spruce 40 (`raw/u1-cost.log`); a fresher projection from the layout depth peaks at the same rise of 3 and the units rounds sit at zero headroom on 812 censuses (`raw/survey-units.log`). No projection from a census two cycles old is a bound, so the bound is reached by one retry (Design revised).
- The device path already retries in one place: `Growing::within` tries smaller margins while the budget refuses. The stepped-limit retry sits beside it, in `Growing::grow_to`, which both the direct build and the growth path call.

## Early proof point

Task fn-302-the-device-never-refuses-a-tree-for-its.1 validates the core approach (frame 41 builds by one retry, and a forced-short build retried equals the bound's build). If it fails, re-evaluate the retry's restart (sizes and seeding) before fn-302-the-device-never-refuses-a-tree-for-its.2.

## Quick commands

```bash
# R1 repro, green after task 1
~/.local/bin/telperion-heavy 8 --threads 2 -- cargo test --profile ci -p telperion-render --lib -- generation::film_walk_tests::the_film_walks_build_on_the_device generation::film_walk_tests::the_rounds_frames_have_the_cpus_counts
# R3 digests (compare with raw/base-digests.log)
DIGEST_SEEDS=1,7,42 ~/.local/bin/telperion-heavy 8 --threads 2 -- cargo test --profile ci -p telperion-render --lib -- resident::digest_tests::digest --ignored --nocapture
```



## Requirement coverage

| Req | Task(s) | Note |
| --- | --- | --- |
| R1 | .1 | run red on bb20f3877 before the plan (`raw/r1-repro.log`); green after .1 |
| R2 | none | answered before the plan: the bound costs more than noise (`raw/u1-cost.log`); the host chose the retry (Design revised) |
| R3 | .2 | |
| R4 | .1, .2 | |
| R5 | .1 | |
| R6 | .1 | |
| R7 | .2 | |
| R8 | .2 | |
