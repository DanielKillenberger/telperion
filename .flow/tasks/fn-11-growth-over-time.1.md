---
satisfies: [R1, R2, R3, R4, R5, R6, R7, R9, R10, R12, R13]
---
# fn-11-growth-over-time.1 Implement Growth over time

## Description
Build growth over time in the spec's order (one task per spec): identities and retained frontiers, the yearly slice, thickening and shedding, the chronicle, bindings and the harness page, and the cost report.

## Acceptance
Every R-ID in the spec's Acceptance except R8 and R11 (moved to fn-30) is satisfied.

## Done summary
A first run hit its 240-minute timebox after step 3 (2026-09-12, NEEDS_HUMAN): mature build then 301x the envelope build, wrong amount of wood, no stills. The owner then approved the chronicle rewrite and the fn-30 split. Final: deterministic specimen with an age, the chronicle, yearly slices, keyframed thickening, vigour shedding, cohort foliage (R12), wasm and native handles, snapshot, live harness page. Mature oak build 1,273.5 ms, sparse oak advance 13.4 ms, record plus read 6.6 ms, snapshot round trip 320.7 ms at 121.9 MB. 36 commits; five gates green; native-to-wasm parity on every preset.

## Evidence
- Commits: 972bace, 2fd2668, d1aa548, e3e3e0d, 28b4295, bff3de7, 20fa5ec, a6e7ba2, 9b35f72
- Tests: cargo fmt --all -- --check, cargo clippy --workspace --all-targets -- -D warnings, cargo test --release --workspace, npm run wasm:build && npm test, npm run typecheck, harness/parity.test.ts
