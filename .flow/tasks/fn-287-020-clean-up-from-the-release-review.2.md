---
satisfies: [R2, R3, R4]
---
# fn-287-020-clean-up-from-the-release-review.2 Delete the envelope experiment, make the metadata true, declare the gate's one invocation

## Description
R2, R3, R4: the dead `closed_form/envelope` experiment goes; the pipeline doc, the space crate's header, `package.json` and the `FLOWERS` text say what ships; AGENTS.md declares the gate's one measured `telperion-heavy` invocation, measured by this task's own gate run. After task 1 so the gate measured is the final one.

**Size:** M
**Files:** `crates/telperion-space/src/closed_form.rs`, `crates/telperion-space/src/closed_form/envelope.rs` (deleted), `crates/telperion-space/src/closed_form/envelope/tests.rs` (deleted), `crates/telperion-space/src/lib.rs`, `docs/pipeline.md`, `package.json`, `crates/telperion-core/src/catalogue.rs`, `docs/parameters.md` and `src/browser/parameters.generated.ts` (regenerated), `AGENTS.md`, `.flow/memory/knowledge/conventions/one-heavy-run-at-a-time-across-workers-2026-10-12.md`
**Touches:** [crates/telperion-space/src/**, docs/pipeline.md, package.json, crates/telperion-core/src/catalogue.rs, docs/parameters.md, src/browser/parameters.generated.ts, AGENTS.md, .flow/memory/knowledge/conventions/**]

### Approach
- R2: delete the two files and the `cfg_attr(not(test), allow(dead_code)) mod envelope;` in `closed_form.rs`; reword the experiment's mentions in `along.rs:2`, `events.rs:4`, `kept/bound.rs:5`, `closed_form.rs:14`; check `sub`/`Sub`, `events` and `dormant`/`presence`/`schedule` imports for items left unused (remove those, never allow them). Re-run the reference search.
- R3: `docs/pipeline.md` (the cycle section's decision list and the coarse-tree paragraph): the arithmetic as `crates/telperion-space/src/contract.rs:1-30` states it; the field as `docs/field-package.md` and `src/field/index.ts` state it (device-held, `query(cells)` on the device, no `cut`, no CPU field). `lib.rs:9`: name the pipeline's use (`telperion-core`'s `pipeline/space.rs` calls `grow`). `package.json` description: one sentence matching README's lead and the engine (botanical growth through one cycle, two executors, device-held field). `catalogue.rs` `FLOWERS` doc comment and `Dial::Excluded` text: flowering is live on Telperion and Laurelin and stays excluded from tuning until a species' spec opens it (fn-216); eligibility unchanged. Regenerate with `TELPERION_WRITE_REFERENCE=1 cargo test --profile ci -p telperion-core --test parameter_reference`, then check the diff is only that text; run `npm run catalogue:check`.
- R4: AGENTS.md "Gates and checked claims": one sentence after the gate command declaring `telperion-heavy 12 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast`, with this task's gate run's revision, counts and wall time (time it with `date` around the run), its scope, and that a different size needs a new measurement. The convention memory note: replace its stacked size updates with a pointer to AGENTS.md (keep its one-heavy-run rule). fn-259/fn-237 evidence untouched; fn-271 (done) untouched.

### Investigation targets
**Required**:
- `crates/telperion-space/src/closed_form.rs` and `closed_form/envelope.rs`
- `docs/pipeline.md:60-75`
- `crates/telperion-space/src/contract.rs:1-30`
- `crates/telperion-core/src/catalogue.rs:276-286`
- `crates/telperion-core/tests/parameter_reference.rs` - the writer
- `.flow/memory/knowledge/conventions/one-heavy-run-at-a-time-across-workers-2026-10-12.md`
**Optional**:
- `docs/field-package.md`, `src/field/index.ts:1-14` - the field as built
- `.flow/specs/fn-274-the-gate-fits-beside-the-owners-desktop.md:10-20` - the 9 GiB measurement

### Key context
- The derived docs are written only through the writer; never hand-edit them.
- AGENTS.md is the owner's instruction file: add the one declaration, change nothing else there.

## Acceptance
- [ ] `closed_form/envelope.rs` and its tests are gone with their `dead_code` allow; the reference search finds nothing; `cargo test --profile ci -p telperion-space --lib` passes with no new warning
- [ ] `docs/pipeline.md`, `telperion-space/src/lib.rs`, `package.json` and the `FLOWERS` text are true to the code; `rg 'telperion_space::fixed|until fn-257 replaces|Nothing here is wired'` over docs/, crates/, src/ finds nothing
- [ ] The regenerated parameter reference differs from the base only in the `FLOWERS` text; `npm run catalogue:check` passes
- [ ] AGENTS.md declares the one gate invocation with revision, counts, wall time and scope; the memory note points at it; no `20 GB` left in AGENTS.md or `.flow/memory/`
- [ ] Workspace gate green under the declared invocation; `npm test` green

## Done summary
The closed-form envelope experiment is deleted with its module allow, and so are the fields only it read (`Stand::dominance`, `Event::Nodes::wood`, `along::dominance`). The `along` module's `dead_code` allow went too, since nothing is left for it to hide. `docs/pipeline.md`, `telperion-space`'s lib header, `package.json` and the `FLOWERS` text now describe what ships, and the parameter reference was regenerated through the writer. AGENTS.md declares the gate's one `telperion-heavy` invocation, and the heavy-run memory note points at it.

Evidence (measured): workspace gate once at 48dc77ae1, `telperion-heavy 12 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast`, 2026-10-16T07:11:19+02:00 to 07:45:48+02:00 (2,069 s): 1,405 passed, 0 failed, 103 ignored. That is the 1,407 at base less the 2 deleted envelope tests. `cargo test --profile ci -p telperion-space --lib`: 103/0/5. The warning set matches the base, which is one pre-existing `boundary.rs:68 laterals` warning in the non-test lib. The regenerated `docs/parameters.md` differs only in 30 identical FLOWERS lines; `parameters.generated.ts` is unchanged. `npm run catalogue:check` passes. `npm test` (under telperion-heavy 12 --threads 2) passes: 18 files, 169 tests. The reference and stale-text `rg` searches find nothing. Baseline: green via handoff (78768296, only .flow/ changed since).

Also in this change: the `pipeline.md` clause about the position upload overlapping `begin_positions` was dropped, because fn-287.1 made that route test-only. The misplaced `bounded` doc line above `FLOWERS` was moved onto `bounded`.

Follow-ups: `telperion_space::grow_coarse` is read only by its own tests (out of scope per spec). The worktree needed `npm ci` before `npm test` would run (FRICTION.md).

Tier: worker (conductor dispatch)

stage: impl-review - ran (codex, three draws: correctness, contracts, integration; SHIP, 0 findings)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 9c16ea71a9bea51d4039ed3fde18325c93bd4c23, 4289e85b0cca9ebc231e8213fb7f17338a469c1f, 48dc77ae1634bb1723a7acd5dfc255037d30fc6c, 02650094198c241207a2fa5027b006f8d56ec67b
- Tests: telperion-heavy 12 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast, cargo test --profile ci -p telperion-space --lib, TELPERION_WRITE_REFERENCE=1 cargo test --profile ci -p telperion-core --test parameter_reference, npm test
- PRs: