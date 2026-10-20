---
satisfies: [R2, R5]
---
# fn-298-a-walk-frame-builds-on-the-device.2 Apply the R2 choice so no walk frame outruns its limit

## Description
Apply the host's R2 ruling (spec R2, "Ruled (host, 2026-10-20)"): `laygens` takes its slack from the job's own bounds instead of `STEP[2]`; `rounds`, `passes`, `sweeps` and `unit_rounds` keep their projection unchanged. Un-ignore the t = 0.2160 repro test; it turns green here. The refusal by name stays.

**Size:** M
**Files:** `crates/telperion-render/src/generation/cycle/build_size.rs`, `crates/telperion-render/src/generation/cycle/build_arena.rs`, `crates/telperion-render/src/generation/cycle/plan_lay.rs` / `lay_device.rs` (read; edit only if the bound lives there), `crates/telperion-render/src/generation/cpu_builds_tests.rs`, `.flow/evidence/fn-298-a-walk-frame-builds-on-the-device/R2.md` (append the paired build_time result)
**Touches:** [crates/telperion-render/src/generation/cycle/**, crates/telperion-render/src/generation/cpu_builds_tests.rs, .flow/evidence/fn-298-a-walk-frame-builds-on-the-device/**]

### Approach
- First, read the code for what bounds one cycle's deepening of the layout: how many layout generations a cycle can open (the units a job grows per cycle, `floors()` in `build_arena.rs:127`, the units rounds, the layout's own walk in `plan_lay.rs` / `lay_*.wgsl`, `DEEP` in `lay_device.rs`). Write what bounds it, with `file:line`, into R2.md. If nothing tighter than `DEEP` bounds it, stop `NEEDS_HUMAN` with what you found; never fall back to `DEEP` (host ruling).
- Then `limits()` (`build_size.rs`): `laygens = deepest + slack`, where `slack` is the most generations the job can add over the two cycles the limit looks ahead, from that bound; capped by `.within(b)` as now. Remove `STEP[2]` (shrink `STEP` to the three limits that still use it) and update the doc comments that mention it (`STEP` docs, `limits()` docs).
- Leave `rounds`, `passes`, `sweeps`, `unit_rounds`, the list ranges and `folds()` exactly as they are.
- Tests: a unit test in `build_size.rs` stating the new `laygens` rule (a census with a given deepest and a job whose bound allows k generations a cycle gives `deepest + 2k`, capped at the bound); keep `a_build_past_the_budget_is_refused_by_name` and the `units_rounds` tests unchanged; remove `#[ignore]` from `fn298_the_beech_to_spruce_at_0_216_builds_on_the_device` and run it with the fn-276 tests narrowly first.
- Cost: a paired `build_time` run (warm column, every shipped preset at its default age, base build beside each change build, three pairs as R2.md's second batch) must stay within noise of the base; append the table to R2.md. Use the R2 scratch-patch approach (an env switch in an uncommitted patch, or two builds) so base and change run in the same batch; never commit a switch.
- Bytes: `DIGEST_SEEDS=1,7,42` `digest` (`resident/digest_tests.rs`, `#[ignore]`) matches the base (run once on the base via the scratch switch or a stash-free WIP commit, once on the change); record both in R2.md.

### Investigation targets
**Required:**
- `.flow/evidence/fn-298-a-walk-frame-builds-on-the-device/R1.md`, `R2.md`
- `crates/telperion-render/src/generation/cycle/build_size.rs:180-236`
- `crates/telperion-render/src/generation/cycle/build_arena.rs:100-130`
- `crates/telperion-render/src/generation/cycle/plan_lay.rs:160-190`
**Optional:**
- `crates/telperion-render/src/generation/cycle/lay_device.rs` (`DEEP`)
- `crates/telperion-render/src/generation/cycle/plan_units.rs`

### Key context
- R1: at cycle 9 `laygens` was 3 (deepest 1 + `STEP[2]` 2); the layout reached depth >= 4. `floors(units)` gave less than 3.
- `Span::new` copies the limits into the census at `REC`; the `given` bounds see them.
## Acceptance
- [ ] What bounds one cycle's deepening is written in R2.md with `file:line`; `laygens` is `deepest` plus the job's own two-cycle slack, capped at the bound; `STEP[2]` is gone and no fallback to `DEEP` exists.
- [ ] `rounds`, `passes`, `sweeps`, `unit_rounds` are unchanged.
- [ ] The t = 0.2160 test is un-ignored and green on the device; the fn-276 tests pass; a unit test states the new `laygens` rule; the refusal by name is still tested.
- [ ] A paired `build_time` run shows every shipped preset within noise of the base; the digest at seeds 1, 7 and 42 matches the base; both in R2.md.
- [ ] The gate is green.
## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
