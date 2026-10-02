---
satisfies: [R1, R2, R3, R4, R5, R6]
---
# fn-181-the-growth-path-is-removed-to-be.1 Implement The growth path is removed, to be rewritten later

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Acceptance Criteria is satisfied; judge this task against the spec's criteria directly.

## Done summary
Removed the growth path: the age timeline, keyframes, chronicle, change records, retention,
storage and snapshots, SpecimenStore/SpecimenView, the executor's present step, the render
web growth view, the Wasm specimen exports, the browser specimen binding and the harness's
?growth=1 and growth controls. Its braided branches in the direct build went too (twig-layer
clock, width queries, growing envelope, sleeping buds, scaffold pause, shoot growth state; a
node is 96 bytes, was 216). /growth is a retired wire row; age stays (owner, 2026-10-02).
README's growth section moved to docs/growth-path.md under the lessons; AGENTS.md records
the removal; fn-67 and fn-30 closed, fn-28 re-scoped, four specs' mentions edited.

R2: generation_digest for 8 presets at seeds 1 and 7, every artifact equal to master;
catalogue identity and 49 species pins re-pinned with proofs (.flow/evidence/fn-181/REPORT.md).
Shipped wasm: 1,419,351 -> 775,512 bytes (main), render 1,994,454 -> 1,462,336, field
368,213 -> 321,417.

stage: impl-review - ran (model: gpt-6.1-sol) SHIP, three draws, no findings

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: b3e1bb05f894428acf5072a8501e668fb9f5fbf7, a43efbba0f0f56c2f127888d5f99e9ff3c28827d, e2bd9a22a966670367bb53e5c87bdf43980dd9dc, eef0fc646aac371a119d39e520e6b112f791fdb4
- Tests: cargo test --profile ci --workspace --no-fail-fast (12 pin failures fixed, rerun focused), npm test, npm run typecheck, generation_digest x16 vs master
- PRs: