---
satisfies: [R2, R5]
---
# fn-298-a-walk-frame-builds-on-the-device.2 Apply the R2 choice so no walk frame outruns its limit

## Description
Apply the choice R2.md records to the stepped limits, so no legal walk frame outruns its limit, and fix a reproduced `Given` row if R1.md has one and the host settled its fix. Un-ignore the R1 repro tests; they turn green here. Refusal by name for a real table or the budget stays.

**Size:** M
**Files:** `crates/telperion-render/src/generation/cycle/build_size.rs`, `crates/telperion-render/src/generation/cycle/gate.rs`, `crates/telperion-render/src/generation/cycle/build_arena.rs`, `crates/telperion-render/src/generation/cycle/given.rs` (only if a `Given` row reproduced), `crates/telperion-render/src/generation/cpu_builds_tests.rs`, `crates/telperion-render/src/generation/cycle/halt_tests.rs`
**Touches:** [crates/telperion-render/src/generation/cycle/**, crates/telperion-render/src/generation/cpu_builds_tests.rs]

### Approach
- Delete branch (R2.md says the bound costs within noise or under 5%): `limits()` records each stepped loop (`rounds`, `passes`, `laygens`, `sweeps`, `unit_rounds`) at its bound; remove `STEP`, `ROUND_SLACK`, `sweeps_ahead`, `passes()` and the `floors()` call where nothing reads them any more, and rewrite their doc comments (module header `build_size.rs:1-6`, `gate.rs` Limits docs, `build_arena.rs:108`, `plan_units.rs` ~200). Keep the projected list ranges (`units`, `draws`, `deposits`) and `folds()`: R2 is about the stepped limits only.
- Derive branch: the limit comes from the job (`floors()` in `build_arena.rs:127`, or a bound from its rows), per R2.md's recorded choice; if R2.md recorded no specific derivation, stop `NEEDS_HUMAN`.
- Rewrite `units_rounds_follow_the_rounds_used` and `a_species_floor_holds_the_limits_up` (`build_size.rs` tests) to state the new rule, and keep a test that a count past a real table or the budget is still refused by name (`a_build_past_the_budget_is_refused_by_name`). Check `halt_tests.rs` callers of `limits`.
- Remove the `#[ignore]` from task .1's repro tests; run them and the fn-276 tests narrowly first.

### Investigation targets
**Required:**
- `.flow/evidence/fn-298-a-walk-frame-builds-on-the-device/R1.md`, `R2.md`
- `crates/telperion-render/src/generation/cycle/build_size.rs`
- `crates/telperion-render/src/generation/cycle/gate.rs:25-130`
**Optional:**
- `crates/telperion-render/src/generation/cycle/plan_lay.rs`, `plan_hier.rs`, `plan_units.rs` (the flags each limit raises)
- `crates/telperion-render/src/generation/cycle/given.rs` (`Span::new` writes the limits into the census at `REC`)

### Key context
- `Span::new` copies the limits into the census words at `REC`, so `given` bounds see them; check the dispatch bounds still hold with the new limits.
- `within(b)` clamps to the plan's bounds; a frame past the bound is still refused by name, which is the intended behaviour for a real table.
## Acceptance
- [ ] The stepped limits follow R2.md's choice; nothing preset-measured (`STEP`, `ROUND_SLACK`) bounds a stepped loop any more.
- [ ] Every R1 repro test and the fn-276 tests pass on the device; a count past a real table or the budget is still refused by name in a unit test.
- [ ] Doc comments describe the new rule; no stale mention of the removed slack.
- [ ] The gate is green.
## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
