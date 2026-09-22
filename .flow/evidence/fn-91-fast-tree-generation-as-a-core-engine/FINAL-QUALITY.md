> Historical raw-output references: see the [archive and recovery instructions](README.md).

# Final quality audit

Base: `0d98f5d06c2a059e8436ee5b8f5858ce6cea66a8`. Reports below are preserved verbatim. They reviewed the production working tree while final aggregate validation ran; neither auditor ran builds or timing.

### Correctness axis

## Quality Audit — Correctness axis: GPU generation, fallback, native parallel surface, station frames

### Summary

- Files changed: 63 production/support files · Critical 0 · Should Fix 0 · Consider 0 · Ship: ✅ Ship

### Test Gaps

- [ ] Final aggregate suite remains pending; this audit did not run builds or timing.

### Test Budget

- Ratio: 1,796 test lines : 5,633 implementation lines (0.32:1).
- Modified existing test files: none detected.

### Security Notes

- No added secret, injection, or debug-code risk found in the reviewed production diff.

### What's Good

- **walked** `generation/request.rs`, `web/generation.rs`: busy, stale, synchronous replacement, and disposal paths reject before a stale prepared tree can commit.
- **walked** `generation/preparation.rs`, `positions.rs`: unsupported station/admission paths return a whole CPU fallback or named error, never partial resident foliage.
- **cited** `surface.rs:218` and `surface/parallel.rs:182`: native worker failure joins started workers and retries the serial builder; tests cover output identity and spawn failure.
- **executed** production diff review and whitespace check. Dispatch-reported core/renderer/browser results are **claimed**, not rerun.

### Standards axis

## Quality Audit — Standards axis: production diff

### Summary

- Files changed: 67 · Should Fix 2 · Consider 0 · Blocking: none possible (standards axis)
- Evidence: executed diff, inventory, API-surface, and file-size scans.

### Should Fix

- **crates/telperion-render/src/generation.rs:25** (Conf 100): Public `Delivery`, `Backend`, and 35-field `Metrics` lack a contract for units, overlapping memory domains, fallback strings, and lifecycle timing — document the public types and fields before external callers depend on ambiguous measurements.
- **src/browser/render.ts:114** (Conf 100): `setTreeGpu` returns `stages`, `previousTreeGpuBytes`, and `treeGpuBytes` at `web/generation.rs:48`, but its TypeScript return type hides them — add exported response and stage-metrics types matching the JSON payload.

### Out-of-axis observations

> Out-of-axis observation: **crates/telperion-render/src/generation/io.rs:356** — test code references undefined `_gpu` instead of `gpu`, so native test compilation fails.

### What's Good

- `scripts/benchmarks/generation.md` documents the eight-worker and 250,000-vertex admission gate plus serial fallback.

## Host disposition

Correctness axis: 0 findings; worst tier none.

Standards axis: 2 findings; worst tier Should Fix. Both are accepted for a bounded API documentation/type correction; no runtime or performance changes are needed.

The out-of-axis test typo was independently caught by the full native build and fixed before the auditors returned. The initial failed build remains in `final-rust-first-build-failure.log`. Final aggregate results are recorded separately; pending-test statements above describe the audit's observation time.

Both standards findings are resolved by Rust API contracts and exported TypeScript `GenerationStages`/`GpuSubmitted` interfaces. Host field comparison confirms all 33 serialized stage fields are represented; TypeScript checking and scoped formatting pass. These are documentation and erased-type changes only, so no performance retiming or full Rust rerun is needed. Final aggregate validation passed 742 Rust tests (20 skipped) and 108 JavaScript tests.
