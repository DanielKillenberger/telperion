---
satisfies: [R1, R2, R3, R4, R5, R6]
---
# fn-181-the-growth-path-is-removed-to-be.1 Implement The growth path is removed, to be rewritten later

## Description
Remove the growing-over-time layer and its braided branches from the direct build; keep the lessons.

## Acceptance
Every R-ID in the parent spec's Acceptance is satisfied.

## Done summary
Removed the growth path (timeline, keyframes, chronicle, change records, retention, storage, snapshots, SpecimenStore/SpecimenView, executor present, web growth view, Wasm specimen exports, harness ?growth=1) and its braided branches in the direct build; a node is 96 bytes, was 216. /growth is a retired wire row; age stays (owner, 2026-10-02). README's growth section moved to docs/growth-path.md; AGENTS.md records the removal; fn-67 and fn-30 closed, fn-28 re-scoped. R2: generation_digest equal to master for 8 presets at seeds 1 and 7; identity and 49 species pins re-pinned with proofs. Wasm: main 1,419,351 to 775,512, render 1,994,454 to 1,462,336, field 368,213 to 321,417. Impl-review SHIP, no findings.

## Evidence
- Commits: b3e1bb05f894428acf5072a8501e668fb9f5fbf7, a43efbba0f0f56c2f127888d5f99e9ff3c28827d, e2bd9a22a966670367bb53e5c87bdf43980dd9dc, eef0fc646aac371a119d39e520e6b112f791fdb4
- Tests: cargo test --profile ci --workspace --no-fail-fast (12 pin failures fixed, rerun focused), npm test, npm run typecheck, generation_digest x16 vs master
- PRs:
