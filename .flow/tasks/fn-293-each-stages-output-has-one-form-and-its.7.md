---
satisfies: [R4, R5]
---
# fn-293-each-stages-output-has-one-form-and-its.7 Carried-over contracts reconciled, docs and the measured result

## Description
R4's reconciliation of each carried-over contract, the docs the earlier tasks made stale, and R5's after-measurements against task 1's baseline.

**Size:** M
**Files:** `README.md`, `docs/{pipeline,principles,field-package}.md`, `.flow/specs/fn-{257,242,15,28,41,123,125,126,171,176,240}-*.md`, `.flow/evidence/fn-293-each-stages-output-has-one-form-and-its/{MEASURE.md,CONTRACTS.md}`
**Touches:** [README.md, docs/**, .flow/specs/**, .flow/evidence/fn-293-each-stages-output-has-one-form-and-its/**]

### Approach
- `CONTRACTS.md` in the evidence folder: one row per item the audit's "Carried-over contracts" table and R4 list (K1-K10 at their current state): kept with its named reason, or removed, and where. A public wire contract a consumer reads (the leaf's twelve bytes if still the shipped encoding, the `{structure:true}` layout and its `kind` column pinned by `harness/parity.test.ts`, the deprecated rows, which fn-237 keeps) is kept and its reason named; changing one is not this spec's.
- Spec text: remove or mark superseded fn-257's R1/R1a/R1' (superseded by R8), fn-242's `telperion-field.wasm` and "one-thread coarse field" text, and each `[inferred]` byte-identity line in fn-15, 28, 41, 123, 125, 126, 171, 176, 240 that pins the pre-engine build or the removed growth path, citing AGENTS.md "Generator evolution" (owner, 2026-09-20). fn-125's plan layout built on `foliage/prepared.rs`: note that task 5 removed it from production.
- Docs: `docs/pipeline.md` (the scaffold line, `prepare_whole` and the CPU delivery, the device drawing stages' node table and handoff, the executor table), `docs/principles.md:19`, `README.md:13-16` migration notes whose audience is unnamed, `docs/field-package.md:63-66`.
- `MEASURE.md`: lines per crate, the packed Wasm modules' bytes (raw, gzip) and the release Wasm build's time, before (task 1) and after; summarise tasks 4 and 6's timings.

### Investigation targets
**Required:**
- `.flow/evidence/fn-293-each-stages-output-has-one-form-and-its/PIPELINE-AUDIT.md` (Carried-over contracts)
- `README.md:1-50`, `docs/pipeline.md`
**Optional:**
- `harness/parity.test.ts:15-35`, `crates/telperion-core/src/pipeline/stage.rs:90-125`

### Key context
No contract a shipped consumer reads changes here. A row whose fate needs the owner is listed under "Open" in `CONTRACTS.md` with the question, not decided.
## Acceptance
- [ ] Every R4 item has a row in `CONTRACTS.md` with its fate and reason.
- [ ] No doc names a route, table or pass the spec removed.
- [ ] `MEASURE.md` holds before and after lines, bytes and build time.
- [ ] Gate and `npm test` green.
## Done summary
R4 and R5 are closed. `CONTRACTS.md` now lists every carried-over contract (K1, K2 and K4 to K10; the audit has no K3) with its fate and the reason or the place where it was decided. The docs and specs no longer name a removed route as current, and `MEASURE.md` records lines, bytes and build time before and after.

- CONTRACTS.md:
  - Kept: K5, the `{structure:true}` layout (its `kind` column is now derived by the engine's conversion, `space.rs:576-585`); K6, the twelve-byte leaf, which is still the shipped encoding; K7, the catalogue identity digests; and K10, the deprecated wire rows (fn-237:74).
  - Removed: K1, K4 and the cube query of K2, all by fn-257 R8. K2's migration note is kept, with its audience now named.
  - Marked superseded: K8 (fn-242 R1, R2) and the K9 lines spec by spec, each citing AGENTS.md "Generator evolution" or fn-181. fn-41's refactor-equivalence check is kept on purpose.
  - The fn-289 note from task 6 (CPU and device leaves already differed) is recorded there.
  - Three owner questions are under Open: K7's changelog header, re-grounding or closing fn-28, and the stale memory entry `a-new-habitparams-row-changes-the-2026-09-26`.
- docs/pipeline.md: the scaffold line; the CPU delivery replaced by the tests' `prepare_whole_resident` route; the device's one drawn-record buffer and the packed curve the sweep writes (no node table, no handoff); the field reading the dress's records; `Expansion::mesh` no longer called a fallback; and the expansion's CPU station and wood steps marked as test-read only.
- README: `Delivery::Resident` is gone, and both migration notes now name their audience (fn-291 R4, fn-257 R8b). docs/field-package.md likewise.
- Outside Touches, as the conductor allowed: the `forkDivergence` catalogue note in `traits.rs` no longer names the `forks_placed` refusal, and `docs/parameters.md` was regenerated with `TELPERION_WRITE_REFERENCE=1`. The handoff mention in the `generation.rs:11` comment is removed.
- MEASURE.md "After", taken at b36086f05 in a temporary worktree, two cold runs:
  - Non-test Rust lines (`measure-cfg-lines.py`): 101911 to 91088 (core -2282, render -774, space -7736, wasm -31).
  - telperion.wasm: 1568583 to 1463135 bytes raw (-6.7 %), gzip -7.0 %.
  - telperion-render.wasm: 3571814 to 3399922 bytes raw (-4.8 %), gzip -5.5 %.
  - `wasm:build` 17.4 to 15.4 s; `render:build` 30.9 to 30.0 s. The machine was less loaded for the after runs.
  - Tasks 4 and 6's timings are summarised there too.
- Gate: `telperion-heavy 12 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast` gave 1401 passed, 0 failed and 111 ignored (`raw/gate-t7.log`). `npm test`: 170 passed. Baseline: none (this task changes docs plus one catalogue string).
- Follow-ups:
  - `scripts/benchmarks/generation.md:323` (outside Touches, a historical fn-91 benchmark record) still names `Delivery::Resident` and a CPU wood fallback.
  - `docs/growth-path.md` still calls `branching::Specimen` "retained".

stage: impl-review - ran (codex, three draws: correctness, contracts and integration, all SHIP with no findings)
Tier: not reported (model: claude-opus-5-5)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: b36086f051b5ab86c8da0f650e0a9b38d2d9d967, 5df505f92d5274dfb9670327e133c6eb88dc746a
- Tests: telperion-heavy 12 --threads 2 -- cargo test --profile ci --workspace --no-fail-fast, npm test
- PRs: