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
- R2: apply a scratch patch to `limits()` (`cycle/build_size.rs`) that sets only `rounds`, `passes`, `laygens`, `sweeps` and `unit_rounds` to their bounds (`Limits::full(b)`'s values for those fields) and keeps `units`, `draws`, `deposits`, `top` and `tail` as computed; save the patch in `raw/`, never commit it. Run `build_time` (`build_time_tests.rs`, `#[ignore]`) three times stepped and three times at the bound; compare the warm column per preset and in total. If the combined change passes the line, measure each of the five limits at its bound alone and apply the rule per limit. Revert the patch by re-applying it in reverse (`git apply -R`), never `git restore`/`checkout`.
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
- [ ] R2.md records three stepped and three at-the-bound warm runs (stepped limits only; list ranges and folding untouched) per shipped preset at its default age, the per-preset and total delta, the noise band, and the choice per stepped limit (delete, or derive from the job) with the line it rests on.
- [ ] No scratch switch is committed; the gate is green.
## Done summary
On the base, the beech to spruce at t 0.216 (seed 7) reproduces the device refusal. It is pinned as an ignored test, `fn298_the_beech_to_spruce_at_0_216_builds_on_the_device`, until task .2 fixes it. R1.md records every row of the spec's table: whether it reproduced, and the cycle, flag, census value and limit. R2.md records the stepped-against-bound warm runs, the deltas, the noise band and the choice per limit. The host has since ruled on that choice; the ruling is task .2's work. No production code or scratch switch is committed.

Gate: `telperion-heavy 8 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast` passed, 1406 passed, 0 failed, 112 ignored across 146 suites. It waited 34 minutes in the heavy queue before running. Full log: raw/gate-task1.log.

stage: impl-review - ran (codex, one correctness reviewer, SHIP first round)
Tier: session (jev long_running 0.40) at default effort (model: claude-opus-5-5)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: a5a1819906586589df3bc953e4a81ffc119f6410, 929cbea019fb604204e16424498abccd8f51b82b, 91b203ae2a2b1b27fa67f970e1717e2a92293e99
- Tests: telperion-heavy 8 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast
- PRs: