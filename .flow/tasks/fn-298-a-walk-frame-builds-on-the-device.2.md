---
satisfies: [R2, R5]
---
# fn-298-a-walk-frame-builds-on-the-device.2 Apply the R2 choice so no walk frame outruns its limit

## Description
Apply the host's revised R2 ruling for `laygens` (spec R2, item 2, "Revised (host, 2026-10-20)"): an empty layout generation costs nothing, so `laygens` can record every pass to its bound. `rounds`, `passes`, `sweeps` and `unit_rounds` keep their projection unchanged. Un-ignore the t = 0.2160 repro test; it turns green here. The refusal by name stays.

**Size:** M
**Files:** `crates/telperion-render/src/generation/cycle/build_size.rs`, `crates/telperion-render/src/generation/cycle/plan_lay.rs`, `crates/telperion-render/src/generation/cycle/layout.wgsl` and the `lay_*.wgsl` passes it touches, `crates/telperion-render/src/generation/cycle/lay_device.rs`, `crates/telperion-render/src/generation/cpu_builds_tests.rs`, `.flow/evidence/fn-298-a-walk-frame-builds-on-the-device/R2.md`
**Touches:** [crates/telperion-render/src/generation/cycle/**, crates/telperion-render/src/generation/cpu_builds_tests.rs, .flow/evidence/fn-298-a-walk-frame-builds-on-the-device/**]

### Approach
1. **Measure first, change nothing.** Read how each layout generation pass is dispatched (`plan_lay.rs` ~160-290: the generation loop gated by `Cond` on `laygens`, its `Size`), and confirm or refute that a pass is dispatched over all of the cycle's new axes and filters by depth, so an empty pass costs as much as a full one. Back it with a trace (a per-pass timing or dispatch-size printout under an uncommitted scratch patch, at `laygens` at its bound, on a shipped preset where the +15.0% showed: beech, birch or palm). Write the finding with `file:line` and the trace numbers into R2.md.
2. **If confirmed:** size each generation pass from the device's own count of axes at that generation, written by the pass before it, through an indirect dispatch (follow how existing passes in `cycle/` already take device-sized indirect dispatches: `Size::Bounded`/`given.rs` is the host-bounded kind; find the indirect kind, e.g. in `plan.rs`/`vm.rs`, and reuse it, never a second mechanism). A generation with no axes dispatches 0 workgroups. `laygens` then records every pass to its bound (`Limits::full`'s value); delete the projection (`deepest`) and `STEP[2]` for it (shrink `STEP` and its docs); `LayGenerations` is still refused by name past the cap.
3. **If refuted, or an indirect dispatch cannot express it:** `laygens = min(cap, unit_rounds[0] + unit_rounds[1] + 1)`. Before relying on it, add a device test that asserts a units round adds at most one generation to a chain (relays included) on the t = 0.2160 frame and on every shipped preset at its default age; measure the cost in a paired run. If that passes 5% too, stop `NEEDS_HUMAN` with the numbers.
4. Tests: a unit test for the new `laygens` rule; un-ignore `fn298_the_beech_to_spruce_at_0_216_builds_on_the_device` (it already compares CPU and device instance counts; R4's tolerance check is task .3's); keep `a_build_past_the_budget_is_refused_by_name` and the `units_rounds` tests.
5. Cost and bytes: a paired `build_time` run (base beside change, three pairs, warm column, every shipped preset at its default age, as R2.md's second batch) within noise of the base; `DIGEST_SEEDS=1,7,42 digest` byte-identical to the base. Record both in R2.md. Use an uncommitted scratch env switch so base and change run in one build; never commit a switch.

### Investigation targets
**Required:**
- `.flow/evidence/fn-298-a-walk-frame-builds-on-the-device/R2.md` (task .2's reading of layout depth)
- `crates/telperion-render/src/generation/cycle/plan_lay.rs`
- `crates/telperion-render/src/generation/cycle/layout.wgsl:270-300`
- `crates/telperion-render/src/generation/cycle/build_size.rs:180-236`
**Optional:**
- `crates/telperion-render/src/generation/cycle/plan.rs`, `vm.rs` (dispatch sizes, indirect)
- `crates/telperion-render/src/generation/cycle/given.rs`

### Key context
- Depth is per cycle: `generation_main` counts only same-cycle ancestors; `T_DEEP` is cleared every cycle (`plan_lay.rs:90-94`).
- R1: at cycle 9, `laygens` 3, depth reached at least 4.
- `laygens` alone at its bound cost +15.0% in total (R2.md batch 2).
## Acceptance
- [ ] R2.md records whether an empty layout generation pass costs as much as a full one, from code (`file:line`) and a trace, before any change.
- [ ] Confirmed: each generation pass is an indirect dispatch sized by the device's count at that generation; `laygens` records every pass to its bound; `STEP[2]` and the `laygens` projection are gone; `LayGenerations` is still refused by name past the cap. Or refuted: `laygens = min(cap, unit_rounds[0] + unit_rounds[1] + 1)` with a device test asserting one generation per round on the t = 0.2160 frame and every shipped preset.
- [ ] `rounds`, `passes`, `sweeps`, `unit_rounds` unchanged.
- [ ] The t = 0.2160 test is un-ignored and green (CPU = device); the fn-276 tests pass; a unit test states the new rule.
- [ ] A paired `build_time` run shows every shipped preset within noise of the base; the digest at seeds 1, 7 and 42 matches the base; both in R2.md.
- [ ] The gate is green.
## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
