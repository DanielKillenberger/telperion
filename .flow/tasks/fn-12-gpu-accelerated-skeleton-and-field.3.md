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
Recorded the evidence-based rejection of production GPU queries and documented the optional owned f64 snapshot API. R1 CPU stage/query evidence is complete with GPU growth/construction and per-round costs explicitly inconclusive; R2 retains CPU behavior after precision and cold timing failures; R3 support, resource, lifecycle and whole-request failures are documented in .flow/evidence/fn12/REPORT.md.

Baseline: green (five focused field tests and typecheck before edits). Final validation: 106 harness tests, package build and CPU-only browser bindings passed. Native workspace tests (69 passed, six ignored), fmt and clippy reused conductor observations after the last Rust change; no Rust changes followed. Task 2 GPU hardware/failure evidence reused for this documentation-only decision. Durable command evidence: .flow/evidence/fn12/validation.json. No additional tests were skipped after the docs-only classification because these relevant checks had already run.

stage: impl-review - skipped(config: REVIEW_MODE=none)
## Evidence
- Commits: 080b8e07ad19ecce4fc337b0d7fd4b7e0888b8c5
- Tests: baseline: green, cargo test --release -p telperion-core --test field (5 passed), npm run typecheck, npm test (106 passed), npm run build, BINDINGS_ONLY=1 PLAYWRIGHT_MODULE=/tmp/fn9-browser/node_modules/playwright/index.mjs BROWSER_URL=http://127.0.0.1:5188 npm run test:browser, cargo test --release --workspace (conductor receipt reused; Rust unchanged; 69 passed, 6 ignored), cargo fmt --all -- --check (conductor receipt reused; Rust unchanged), cargo clippy --workspace --all-targets -- -D warnings (conductor receipt reused; Rust unchanged), Task 2 gpu-tests.json hardware and controlled failure checks reused; no GPU rerun for docs
- PRs:

## Owner-authorized removal of rejected experiment

On 2026-09-07 the owner requested removal of the rejected implementation and unused material. Removed the WebGPU candidate, dedicated runner/tests/operating guide, and superseded preliminary/internal timing dumps. README and final report now describe removal and point historical reproduction to commit 6f3adb2. Final raw timing, correctness and input-verification evidence remain.

Retained Field::snapshot(), Wasm/browser bindings, CPU baseline and generation-inputs helpers because fn18's exact/approximate oracles and measurements import them. Their bytes and the existing integration tests are unchanged. No fn18 worktree changes or GPU measurements were made. Cleanup verification: TypeScript checking, syntax checks for both CPU helper modules and browser integration, removed-import scan, history recoverability and git diff --check passed.
