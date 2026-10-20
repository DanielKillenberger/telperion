---
satisfies: [R1, R2]
---
# fn-298-a-walk-frame-builds-on-the-device.1 Reproduce each refusal and measure the stepped limits against the bound

## Description
Run every refusal of the spec's table as a device test on the base, record which reproduce and the census value against the limit it passed (R1); then measure the stepped limits against `Limits::full` and record the choice R2's rule gives. No production code changes in this task. It is the early proof point: tasks .2 and .3 build on its two tables.

**Size:** M
**Files:** `crates/telperion-render/src/generation/cpu_builds_tests.rs`, `.flow/evidence/fn-298-a-walk-frame-builds-on-the-device/R1.md`, `.flow/evidence/fn-298-a-walk-frame-builds-on-the-device/R2.md`
**Touches:** [crates/telperion-render/src/generation/cpu_builds_tests.rs, .flow/evidence/fn-298-a-walk-frame-builds-on-the-device/**]

### Approach
- Run the fn-276 tests first, unchanged: `walk_frames_near_the_beech_build_on_the_device` (t 0.00037, 0.01, 0.1 at seeds 7 and 1) and `the_laurelin_at_20_builds_on_the_device`, through `telperion-heavy 8 --threads 2 --` and `--profile ci -p telperion-render --lib`. Save full output under `raw/`.
- Add one test for each table row those do not cover: beech to spruce at 40, seed 7, t = 0.2160 (state the eased share it came from; `ease(at) = at*at*(3-2*at)`). Follow the `refused()` helper at `cpu_builds_tests.rs:18-46`. A row that reproduces stays in as a test marked `#[ignore = "fn-298 R1: red on the base; task .2 fixes it"]`, so this task's gate stays green; a row that does not reproduce is recorded and not tested further (R1).
- For each reproduced row, rerun with `CYCLE_TRACE=1` to read the failing cycle's `Limits` (`rec ...`) and the raised flag's value (`build_grow.rs` ~385, `census.rs:148` `raised`). For `UnitRounds` the value is the rounds left unfinished (`plan_units.rs` ~300); state the rounds used beside `unit_rounds`. If the trace lacks the census word, add a trace line under `CYCLE_TRACE` only, never on by default.
- R2: apply a scratch patch making `limits()` (`cycle/build_size.rs`) return `Limits::full(b)` always; save the patch in `raw/`, never commit it. Run `build_time` (`build_time_tests.rs`, `#[ignore]`) three times stepped and three times full; compare the warm column per preset and in total. Revert the patch by re-applying it in reverse (`git apply -R`), never `git restore`/`checkout`.
- Write R1.md (the table: row, reproduced or not, cycle, flag, census value, limit) and R2.md (both runs, the delta, the noise band, and the choice R2's rule gives). Borderline per R2's Errors line, or a reproduced `Given` row whose fix R2's rule does not settle: stop `NEEDS_HUMAN` with the numbers.

### Investigation targets
**Required:**
- `crates/telperion-render/src/generation/cpu_builds_tests.rs:1-90`
- `crates/telperion-render/src/generation/cycle/build_size.rs:180-236` (`limits`)
- `crates/telperion-render/src/generation/cycle/build_grow.rs:300-400` (where limits are taken and the trace prints)
- `crates/telperion-render/src/generation/build_time_tests.rs`
**Optional:**
- `crates/telperion-render/src/generation/cycle/given.rs` (the `Given` bound)
- `crates/telperion-render/src/generation/cycle/plan_units.rs` (UnitRounds flag)

### Key context
- One heavy run at a time on this desk (memory entry `knowledge/conventions/one-heavy-run-at-a-time-across-workers`). A wait past 30 minutes is a FRICTION entry and `NEEDS_HUMAN`.
- This worktree has its own `target/`; never point it at another checkout's.
- Do not read receipts, videos or more than four images.
## Acceptance
- [ ] R1.md lists every row of the spec's table: reproduced or not, and for each reproduced one the cycle, flag, census value and limit.
- [ ] Each reproduced row not already covered is a committed test, ignored with the fn-298 reason; the existing fn-276 tests are untouched.
- [ ] R2.md records three stepped and three full warm runs per shipped preset at its default age, the per-preset and total delta, the noise band, and the choice (delete, or derive from the job) with the line it rests on.
- [ ] No scratch switch is committed; the gate is green.
## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
