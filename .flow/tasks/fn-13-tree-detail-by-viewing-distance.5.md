---
satisfies: [R2, R3, R5]
---
# fn-13-tree-detail-by-viewing-distance.5 Render and measure a forest of distinct specimens

## Description
Exercise the renderer with the pinned varied population and bounded residency, including approach to individual trees.

**Size:** M
**Files:** harness/forest-view.ts (new), harness/forest-view.test.ts (new), harness/stage.ts, scripts/benchmarks/forest.mjs (new), src/browser/render-data.ts
**Touches:** [harness/forest-view.ts, harness/forest-view.test.ts, harness/stage.ts, scripts/benchmarks/forest.mjs, src/browser/render-data.ts]

### Approach
- Materialize the task 1 manifest using a bounded generation/packing queue and task 3 representation ownership. Hash actual source structure to verify at least 1,000 distinct specimens; report duplicate fingerprints and failed generation without quietly omitting entries.
- Preserve the frozen species/size mix, layout and camera density. Track population count, visible count, source identities, residency and per-tree transforms through all stage subject-selection paths.
- Release transient full-detail output copies after packing; retain sufficient source identity/data for deterministic near-detail recovery. Pin and enforce task 2's resource ceilings, and include preparation/upload stalls in measurements.
- Measure ground traversal and canopy paths, individual-tree approach/departure, replacement and disposal. Expose errors or explicit preparation state instead of losing detail when residency is insufficient.
- Report all vegetation passes, GPU distributions, cold and steady CPU costs, measured versus estimated memory and 4 ms target misses. Serialize GPU jobs with fn12; repeat only measurements invalidated by contention or changed code.

### Investigation targets
**Required**
- harness/skeleton-view.ts:419 — current small comparison population, not a forest
- harness/stage.ts:627 — subject lifetime
- scripts/benchmarks/measure.mjs:16 — hardware evidence
- scripts/benchmarks/memory.mjs:6 — memory labels
- src/browser/core.ts:67 — Wasm high-water and release
- src/browser/render-data.ts — task 3 data ownership

### Quick commands
npm run typecheck
npx vitest run harness/forest-view.test.ts

## Acceptance
- [ ] The pinned 1,000-tree manifest renders and has 1,000 verified distinct botanical structures; shared prototypes do not replace unique specimens.
- [ ] Frozen paths retain coverage and recover individual anatomy without popping or silent residency omissions, including a cold approach and reversal.
- [ ] GPU/CPU/preparation/upload/memory receipts describe the actual varied scene, preserve invalid-run evidence and explicitly report the 4 ms forest outcome.
- [ ] Replacement, context-loss/interrupted preparation and disposal tests release partial resources and preserve unaffected trees; no uncontrolled expanded-array residency.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
