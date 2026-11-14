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
`laygens` is now a cost hint. Option (d) adds a closing direct layout pass (`tail_main`, layout.wgsl) after the last recorded generation. It lays every new axis deeper than `laygens - 1` with the generation pass's own steps in their order, one writer per chain, chosen by its head. `LayGenerations` is refused only past the cap of 15, and `STEP[2]` is documented as a hint. The rounds floor (xii) gives the hierarchy's generations the units rounds' floor, `rounds = max(gens(c - 2) + STEP[0], 2 units - 1)`. `passes`, `sweeps` and `unit_rounds` are unchanged.

Tests:
- `one_layout_generation_lays_the_stepped_tree` (spruce, `laygens` forced to 1) gives the stepped build's bytes.
- `fn298_the_beech_to_spruce_at_0_216_builds_on_the_device` is un-ignored and green. Node counts are equal (275,557), and instance counts are equal (96,524; the narrowed R4 allows a difference of 1).
- The position and radius check is a separate test, ignored under fn-300.
- The refusal-by-name tests are kept.
- R2.md holds the digest (24/24 identical) and the paired `build_time` (+0.7% for (d), +0.1% for the floor).

The first gate run failed on `compile_size`: `tail_main` inlined 69,121 expressions against the 50,000 ceiling. Fixed in b60657e84, which leaves one call site of `lay`; the lay order is the same and the forced-1 test is still byte-identical. FRICTION.md has the entry. The gate rerun is green: 1,408 passed, 0 failed, 112 ignored.

Follow-up for the host: codex's contracts draw notes, as P2, that task .3's acceptance still asks for position agreement on the fixed frames, which R4 now defers to fn-300. That text is outside this task's Touches.

stage: impl-review - ran (codex, 3 draws, round 1: SHIP; reviewed head f5bf82b7f; b60657e84, the compile-ceiling fix, landed after the review)
Tier: session (jev long_running 0.40) at default effort (model: claude-opus-5-5)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 22e7a8418c5bb570a4acc0b983153703b46adf03, e8470899ab2b233dcbac00f4b123043c3da052fe, 9c1cbc818ce29f5f2f9aa1ef9a56e6338477db4b, 1867e1a0cd10427e9688492343927494e9e38656, e982004962dfd739c61c2481ae4a26e89800e792, e5d4055f7c831e43f8ad8767ecdd4f8b4abcab87, 808cfe4cb72f281e3bf6dac7f422c7076c855478, 1d9ba85b7f05de91d6ca80c10e64198fa948e068, 95c2fa37216dab9d3164091066fc58e4ac730627, a606f866bc5778b610b5bf32a7b3d15ae1ef4d2f, e69b0183f1d07235f378287683158b860b803276, bc70aaebdb4c736896dd0415ee8ecd45c8a98754, 30de58fc71bcddc4a96f74768bc506020f9b11de, b227997cf3e38cd8ec83e46c1aaac27b9c6400c4, 2cd02c362161946f958548ac9c8c10a0516c07cc, 12b79da7add6a0c756618b826bc636a6b327216d, 433eaddbe394e61d1a0e3609705841147fd253e1, 9eaac8cf424cae0905fa9ecd869acb9f074a498b, f5bf82b7fdc652004ce5488504bd9b97bfd88895, b60657e84cb06b9dbe4121012b6baca25d6c2b7b
- Tests: telperion-heavy 8 --threads 2 -- cargo test --profile ci -p telperion-render --lib -- fn298 --include-ignored --nocapture, telperion-heavy 8 --threads 2 -- cargo test --profile ci -p telperion-render --lib -- compile_size one_layout_generation_lays_the_stepped_tree fn298 --nocapture, telperion-heavy 8 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast
- PRs: