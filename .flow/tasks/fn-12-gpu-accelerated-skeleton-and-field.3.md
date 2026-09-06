---
satisfies: [R1, R2, R3]
---
# fn-12-gpu-accelerated-skeleton-and-field.3 Apply the measured adoption decision and document support

## Description
Resolve the candidate decision and final R1–R3 evidence. Negative results complete this task with CPU behavior retained.

**Size:** M
**Files:** `src/browser/core.ts`, `src/browser/generation-gpu.ts` (conditional), `tests/browser/integration.mjs`, `README.md`, `.flow/evidence/fn12/REPORT.md`
**Touches:** [src/browser/core.ts, src/browser/generation-gpu.ts, crates/telperion-core/src/field.rs, crates/telperion-wasm/src/lib.rs, tests/browser/**, scripts/benchmarks/generation*, README.md, .flow/evidence/fn12/**]

### Approach
Only if task 2 qualifies a practical workload, integrate a small explicit asynchronous opt-in entry point, preserving synchronous CPU and revision/lifecycle contracts and whole-request fallback/error. Otherwise retain the reproducible experiment and document why adoption was rejected/inconclusive. Update README measurement/reproduction pointers and support/precision guarantees. Report native/browser/driver assumptions, allocation/resource limits and whether defaults changed. Reuse tests/browser/integration.mjs ownership/release patterns. Do not create a framework or unproven faster-default claim.

### Quick commands
`cargo test --release --workspace`; `cargo fmt --all -- --check`; `cargo clippy --workspace --all-targets -- -D warnings`; `npm run typecheck`; `npm test`; `npm run build`; `npm run test:browser` for changed bindings.

## Acceptance
- [ ] Evidence-based adoption decision recorded for ordinary/giant skeleton, construction and querying; only qualified paths ship, and negative/inconclusive results are honest complete findings.
- [ ] README/report state exact support, f64/f32/repeatability contract, resource accounting, fallback/error and API lifecycle behavior with reproducible commands and raw evidence links.
- [ ] Relevant native, Wasm/browser, harness and build checks pass; production botanical identity/geometry semantics remain unchanged; no automatic plan/implementation/completion review dispatched per owner instruction.


## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
