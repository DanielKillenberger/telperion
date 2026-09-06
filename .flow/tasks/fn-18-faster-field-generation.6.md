---
satisfies: [R1, R2, R3, R4]
---
# fn-18-faster-field-generation.6 Publish complete-build results and final lifecycle validation

## Description
Validate the integrated result and report the several-fold target honestly.

**Size:** M
**Files:** tests/browser/integration.mjs, tests/browser/field-generation.mjs, scripts/benchmarks/field-generation.md, README.md, tests/migration/README.md, .flow/evidence/fn18/REPORT.md.
**Touches:** [tests/browser/integration.mjs, tests/browser/field-generation.mjs, scripts/benchmarks/field-generation.md, README.md, tests/migration/README.md, .flow/evidence/fn18/**]

### Approach
- Extend browser lifecycle checks to the optimized path: release/dispose, successful/failed rebuild invalidation, snapshot mutation isolation, query-error recovery, allocation/overflow failure and successful retry. Reacquire Wasm views after memory growth.
- Run the frozen paired protocol on final clean binaries versus the original, exclusive of fn13 benchmarks. Include the cold lifecycle denominator, complete build, stages, query/combined-output medians, allocation lifetimes and instrumentation overhead.
- Report achieved/missed/unproven 3×/10× results, remaining serial limits, native parallel support/failure, separate throughput/edit findings and all rejected candidates. A useful sub-3× improvement may ship as partial progress, but R1/spec completion cannot be claimed.
- Update README Measurements and limits with the receipt; preserve fn12 historical evidence. Add the new oracle to tests/migration ownership documentation if retained.
- Run focused suites during implementation, then full native/browser/harness/typecheck/build plus required fmt/clippy once on final changes. No plan, implementation, completion or renamed quality review is requested.

### Investigation targets
**Required:**
- tests/browser/integration.mjs:86 — snapshots/lifecycle.
- tests/browser/integration.mjs:192 — failed builds/output masks.
- README.md:84 — measurement claims.
- tests/migration/README.md — oracle ownership.
- .flow/evidence/fn12/REPORT.md — evidence precedent.
- scripts/benchmarks/field-generation.md — frozen protocol.

### Quick commands
cargo test --release --workspace; npm run test:browser; npm test; npm run typecheck; npm run build.

### Acceptance
- [ ] Final exact source/field and lifecycle/resource checks pass, including memory growth and failed-build recovery.
- [ ] Raw paired evidence, binary/input identity, all allocation domains and each target's reached/missed/unproven status are retained.
- [ ] Browser first-build claims are separated from native, warm, throughput and edit results; docs match actual adoption.
- [ ] Required final tests/checks pass or explicit blockers remain. Missed 3× target is not marked satisfied or hidden by completion bookkeeping.

## Acceptance
- [ ] TBD

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
