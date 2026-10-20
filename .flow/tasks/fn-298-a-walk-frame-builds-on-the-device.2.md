---
satisfies: [R2, R5]
---
# fn-298-a-walk-frame-builds-on-the-device.2 Apply the R2 choice so no walk frame outruns its limit

## Description
Apply the host's option (d) for `laygens` (spec R2 item 2, "Ruled (host, 2026-10-20), after step one confirmed the premise"): `laygens` becomes a cost hint; a closing tail pass lays any new axis deeper than the recorded generations, bit-identically. `rounds`, `passes`, `sweeps`, `unit_rounds` unchanged. Un-ignore the t = 0.2160 repro test; it turns green here. Step one of the earlier ruling is done (R2.md "Task .2, step 1"): an empty pass is not free; indirect dispatch is ruled out (DESIGN.md §7a).

**Size:** M
**Files:** `crates/telperion-render/src/generation/cycle/plan_lay.rs`, `crates/telperion-render/src/generation/cycle/layout.wgsl` (and the `lay_*.wgsl` it uses), `crates/telperion-render/src/generation/cycle/lay_device.rs`, `crates/telperion-render/src/generation/cycle/build_size.rs` (docs only: `STEP[2]` is now a hint), `crates/telperion-render/src/generation/cpu_builds_tests.rs`, `.flow/evidence/fn-298-a-walk-frame-builds-on-the-device/R2.md`
**Touches:** [crates/telperion-render/src/generation/cycle/**, crates/telperion-render/src/generation/cpu_builds_tests.rs, .flow/evidence/fn-298-a-walk-frame-builds-on-the-device/**]

### Approach
1. **Read first, change nothing.** The seven dispatches of a generation pass (`plan_lay.rs:140-163`, `layout.wgsl`, the `lay_*.wgsl` they call). List every cross-axis read when laying an axis, with its line: siblings, a prefix sum or scan over the generation, collision or space queries against same-generation axes, atomics shared across axes. Write the list into R2.md and conclude chain-local or not.
2. **Chain-local:** add one closing direct dispatch after the last recorded generation (sized like the generation passes, over the cycle's new axes). Each new axis with generation > `laygens − 1` is laid there by one invocation that walks up to its deepest laid ancestor and lays the chain top-down, calling the same functions in the same order of float operations as the per-generation path. Race rule: either one writer per chain chosen by the chain's head, or shared ancestors laid in a fixed order; record which in R2.md and in the shader's doc comment. `LayGenerations` is flagged only past the plan's cap of 15. `STEP[2]` stays; update its doc to say it is a cost hint. Keep files under ~400 lines (split a new `lay_tail.wgsl` if needed).
3. **Not chain-local:** build nothing. Read each shipped preset's per-cycle `unit_rounds` (default age and seed) and test the one-generation-per-round claim (relays included) on the t = 0.2160 frame and every shipped preset; record in R2.md, commit, and stop `NEEDS_HUMAN` with the numbers.
4. Tests (chain-local branch): un-ignore `fn298_the_beech_to_spruce_at_0_216_builds_on_the_device` (CPU = device); a test that forces `laygens` to 1 on a shipped preset (a test-only hook confined to `#[cfg(test)]`, never a runtime switch) and asserts the same mesh bytes as the stepped build; keep the refusal-by-name tests.
5. Cost and bytes: paired `build_time` (base beside change, three pairs, warm column, every shipped preset at default age) within noise; `DIGEST_SEEDS=1,7,42 digest` byte-identical to the base. Record both in R2.md. Use an uncommitted scratch switch so base and change run in one build; revert with `git apply -R`.

### Investigation targets
**Required:**
- `.flow/evidence/fn-298-a-walk-frame-builds-on-the-device/R2.md` (task .2's readings and the step-1 trace)
- `crates/telperion-render/src/generation/cycle/plan_lay.rs`
- `crates/telperion-render/src/generation/cycle/layout.wgsl`
- `crates/telperion-render/src/generation/cycle/lay_device.rs`
**Optional:**
- `crates/telperion-render/src/generation/cycle/lay_walk.wgsl`, `lay_frame.wgsl`, `lay_state.wgsl`, `lay_stand.wgsl`, `lay_draw.wgsl`, `lay_pencil.wgsl`
- `crates/telperion-render/src/generation/resident/digest_tests.rs`

### Key context
- Every dispatch is direct (`plan.rs:4-8`; DESIGN.md §7a rule 1); the tail pass is a direct dispatch too.
- Depth is per cycle: `generation_main` counts same-cycle ancestors; `T_DEEP` is cleared each cycle.
## Acceptance
- [ ] R2.md lists every cross-axis read in a generation pass with its line, and concludes chain-local or not, before any change.
- [ ] Chain-local: a closing direct pass lays every new axis deeper than `laygens − 1` with the same functions and float order; the race rule is stated; `LayGenerations` flags only past the cap of 15; `STEP[2]` documented as a cost hint. Not chain-local: nothing built; per-preset `unit_rounds` and the one-generation-per-round test are in R2.md, and the task stops `NEEDS_HUMAN`.
- [ ] `rounds`, `passes`, `sweeps`, `unit_rounds` unchanged.
- [ ] The t = 0.2160 test is un-ignored and green (CPU = device); a test forcing `laygens` to 1 on a shipped preset matches the stepped build's mesh bytes; refusal by name still tested.
- [ ] A paired `build_time` run is within noise of the base; the digest at seeds 1, 7 and 42 matches the base; both in R2.md.
- [ ] The gate is green.
## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
