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
TBD

## Evidence
- Commits:
- Tests:
- PRs:
