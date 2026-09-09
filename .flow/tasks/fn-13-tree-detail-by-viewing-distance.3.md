---
satisfies: [R2, R5, R6]
---
# fn-13-tree-detail-by-viewing-distance.3 Promote the working data pipeline into the browser build

## Description
Wire the existing production-importable task10 representation and task18 generator contract into the normal Wasm/browser build, keeping exact defaults and explicit approximate requests. The compiler/hierarchy/residency already live in src/browser; this task owns Wasm boundary, build loading and responsive preparation, not a renderer port. Reuse the existing geometry-residency ownership implementation; a duplicate resources module is unnecessary. Export the typed boundary from src/index.ts and verify normal built worker/Wasm loading.

Expose numeric geometry/placement identities, conservative bounds and explicit ownership to the renderer. Keep preparation responsive using the smallest suitable worker/yielding boundary; serialize mutable Wasm use, support cancellation/latest-request wins, and retire resources safely. Avoid speculative serialization/export/backends and do not expand per-tree foliage just to enter the optimized path.

**Size:** M
**Files:** src/index.ts, vite.config.ts, src/browser/core.ts, src/browser/render-data.ts, src/browser/render-data.test.ts, src/browser/geometry-residency.ts, experiments/fn13-rendering/tree-compile-check.mjs, src/browser/render-worker.ts, scripts/build-wasm.mjs, crates/telperion-wasm/src/lib.rs, tests/browser/integration.mjs
**Touches:** [src/index.ts, vite.config.ts, src/browser/core.ts, src/browser/render-data.ts, src/browser/render-data.test.ts, src/browser/geometry-residency.ts, experiments/fn13-rendering/tree-compile-check.mjs, src/browser/render-worker.ts, scripts/build-wasm.mjs, crates/telperion-wasm/src/lib.rs, tests/browser/integration.mjs]

## Acceptance
- [ ] Normal dev/build loads the assembly-capable artifact through the typed API; spruce/oak and supported parameter changes reach the existing working renderer contract without experimental URLs/manual rebuilds or Three materialization.
- [ ] Preparation remains responsive, rejects stale/canceled results and preserves exact-mode behavior; shared resources survive unrelated replacement/disposal and malformed input cannot publish invalid data.
- [ ] Focused boundary/lifetime tests and the normal browser loading/request path pass; document actual supported input limits and costs.

## Quick commands
```bash
npm run typecheck
node_modules/.bin/vitest run src/browser/render-data.test.ts
node experiments/fn13-rendering/tree-compile-check.mjs
npm run test:browser
```
Run focused checks during implementation; broader checks once on the integrated path. Reuse valid unchanged expensive evidence.

## Done summary
Promoted assembly transport and canonical fitting into src/browser and exported worker-backed TreePreparation, owned PreparedTree data, bounded exact-view preparation and pure middle selection. Normal dev/build uses the assembly-capable Wasm; experiment imports re-export the same implementation. Exact TreeEngine defaults and the viewer remain unchanged.

Baseline: typecheck green; original focused Quick command inherited red because planned test files did not exist. Conductor corrected the stale command to the delivered boundary tests and existing tree-compile check. Baseline software browser run stopped under owner policy; no pass claimed. First integrated attempt passed worker checks but hit Vite HMR duplicate core singleton; clean server restart resolved it. Final normal build, typecheck, 2 focused lifetime tests, existing hierarchy/residency and canonical-library checks, and full npm run test:browser all passed (exit 0). Browser exercises spruce/oak, seed changes, finite-input rejection, cancellation, owned lifetimes, main-thread responsiveness, bounded source cursor, both dev and emitted-library loading, exact binding/cardinality/default behavior, captures and viewer retry.

Evidence: .flow/tmp/fn13-3-browser/{preparation,bindings,viewer,captures}.json and .flow/tmp/fn13-3-canonical-check.json. Built small-fixture preparation approximately197ms oak/287ms spruce; these are CPU observations, no GPU performance verdict. README documents caps, per-specimen numeric ID namespace, fresh worker CPU library cost, and unavailable empty approximate foliage (exact empty output remains supported). Existing fn9 owner dependency override remains recorded. Public export/build configuration and reusable module moves are narrow conductor-authorized scope extensions.

stage: impl-review - skipped(owner policy: REVIEW_MODE=none)
stage: plan-sync - skipped(config false)

Runtime status restored on 2026-09-08 from this committed completion receipt; historical validation and scope above are unchanged.
## Evidence
- Commits:
- Tests: baseline: typecheck green; original focused command red (planned files absent), corrected by conductor; baseline browser stopped by owner policy, not a pass, npm run build, npm run typecheck, node_modules/.bin/vitest run src/browser/render-data.test.ts (2 passed), node experiments/fn13-rendering/tree-compile-check.mjs, node experiments/fn13-rendering/canonical-library-check.mjs, PLAYWRIGHT_MODULE=/tmp/fn13-browser/node_modules/playwright/index.mjs CHROMIUM_EXECUTABLE=/usr/lib/chromium/chromium BROWSER_URL=http://127.0.0.1:5196 BROWSER_EVIDENCE=.flow/tmp/fn13-3-browser npm run test:browser (exit 0; dev and built library)
- PRs: