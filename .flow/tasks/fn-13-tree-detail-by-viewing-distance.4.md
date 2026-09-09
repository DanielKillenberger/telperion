---
satisfies: [R1, R2, R3, R4, R5, R6]
---
# fn-13-tree-detail-by-viewing-distance.4 Integrate and stabilize the renderer in the normal viewer

## Description
Connect the working renderer to normal parameter controls, using task3 preparation and the same camera/scene state. Keep exact inspection selectable and preserve parameters/identity/camera across mode switches. The result must be something the owner can navigate and judge.

Fix observed static/moving noise, ground/depth artifacts and loading holes in this actual path. Use short reproductions to separate geometry, sampling and residency failures, then implement the simplest effective filter/selection fix. Temporal filtering, crossfades or other mechanisms are allowed when useful; validate their actual ghosting/disocclusion behavior rather than requiring a separate filter project or a prescribed method. Measure total pass cost while iterating. Keep the previous valid scene during parameter rebuilds and commit only the newest valid result.

**Size:** M
**Files:** harness/stage.ts, harness/webgpu-stage.ts, harness/GrowerDev.tsx, harness/skeleton-view.ts, src/browser/webgpu/**, src/browser/render-selection.ts, src/browser/render-selection.test.ts, tests/browser/rendering.mjs, .flow/evidence/fn13/candidates/viewer/**
**Touches:** [src/index.ts, src/browser/runtime-scene.ts, src/browser/gpu-types.ts, experiments/fn13-rendering/webgpu-budget.js, experiments/fn13-rendering/compact-wood.ts, experiments/fn13-rendering/webgpu-scene.ts, README.md, harness/stage.ts, harness/webgpu-stage.ts, harness/GrowerDev.tsx, harness/skeleton-view.ts, src/browser/webgpu/**, src/browser/render-selection.ts, src/browser/render-selection.test.ts, tests/browser/rendering.mjs, .flow/evidence/fn13/candidates/viewer/**]

## Acceptance
- [x] Normal GrowerDev optimized mode renders both species, changes seed/botanical parameters and switches to exact inspection/back without expanded foliage materialization, stale results or lost camera/identity.
- [x] Representative slow/fast navigation, reversal, inside-crown approach, cold loading, resize and parameter replacement show convincing anatomy/coverage with no distracting noise, popping, ghosting or missing regions. Fix failures before adding more capture cases.
- [x] Ground/wood/foliage share correct depth and contact; required capability failure is explicit. Actual all-pass timings and scoped visual evidence are retained for final acceptance.

## Quick commands
```bash
npm run typecheck
node_modules/.bin/vitest run harness/stage.test.ts src/browser/render-selection.test.ts
npm run test:browser
```
Run focused checks during implementation; broader checks once on the integrated path. Reuse valid unchanged expensive evidence.

## Done summary
Normal GrowerDev now uses the shared WebGPU runtime by default, preserves exact inspection and transactional camera/identity state, and transitions bounded canonical detail to exact source poses through deterministic station correspondence. Cached selection, retained resources and packed uploads reduced the measured deep approach/reversal from 6.57 s to 1.87 s; the rejected history blend was reverted.

Baseline: green before edits (typecheck, 45 focused tests, normal browser suite). Final verification passed: typecheck, 147 full Vitest tests, 50 task Quick tests, native Rust libraries (6 passed, 2 ignored), compiler checks, normal production build and emitted-library/browser regression (actual command exit 0). Actual normal-viewer hardware captures cover both species, slow/fast/reversal/inside navigation, cold transition first/midpoint/moving reversal, parameter replacement, exact/back, resize and explicit capability failure. Scoped observations, rejected/inconclusive captures and gate logs are retained in `.flow/evidence/fn13/candidates/viewer/RESULT.md` and `gates/`.

All-pass GPU measurements include scene, vegetation, resolve and temporal work. They remain diagnostic because desktop contention is unqualified; occasional CPU updates remain measurable (deep-path p95 36.1 ms, max 57.7 ms). Final retained prepared/selector arrays total about 226 MB for spruce and 62 MB for oak; forest validation remains downstream.

stage: impl-review - skipped(config: REVIEW_MODE=none; user explicitly requested no review)
stage: plan-sync - skipped(config: false)

Runtime status restored on 2026-09-08 from this committed completion receipt; historical validation and scope above are unchanged.
## Evidence
- Commits: 0e40da7c162347ff62832a0f201ba45b4e7ad081, 6e62f5fc06516fc1916668acba5623f908a8a9a6, 72982b0d02fb3997c29109ebef3b3cdde1d96ac4, 991f803896ac20d62de76ffa5e36621f022ef9b4, 315023ad0ce306568698096f3460d02cdf245688, 86bb4c4eeb756fb0c6f7c4c6ede03478a3828bd3, f69fe48a6e57d87035b3b328639ec3d4c6070ef9, 682073fa1c3f201c5f5fc0ba378f1f371a39dca7, 4bbfece434ea8a4f8279c5af3f18f3065e65f52c, 83e64d0f6a9a4c9393ae3459b183f75d7a13645e, 3d19239febf7fc97a638dc1ea671483839d81d8b, 00d0a30ec58832831f75b6f803ebc51889d7b248, 55c245c20fa5f7726e45b458d3c319da848c892c, 36681df3fbb0740f8fe3c97102e28d1857f12492, 0d6d18090aa3f09381fc59fa0f6c788b20f9612c, ec11115edda9c0a502d6c80d87a129a5f4d55d86, 60cbb45107df07c4005351be88f3d55aeb02541c, 4c7def0b174d8ad842777266ea9369005fb4f518, 23a96615abf96b936b2f8e12e484812d9e508c4a, f2dba603d6d52a70f1553e889f490e7ddcdef990, e3ea8a0e25e862423eb56875cbde6cf88948101b
- Tests: baseline: green before edits — npm run typecheck; focused Vitest 45 tests; npm run test:browser exit 0, npm run typecheck — exit 0, node_modules/.bin/vitest run harness/stage.test.ts src/browser/render-selection.test.ts — exit 0, 50 tests, node_modules/.bin/vitest run — exit 0, 147 tests in 12 files, cargo test --release --workspace --features telperion-core/experimental-assemblies,telperion-wasm/experimental-assemblies --lib — exit 0, 6 passed, 2 ignored, node experiments/fn13-rendering/tree-compile-check.mjs — exit 0, npm run build — exit 0, PLAYWRIGHT_MODULE=/tmp/fn13-browser/node_modules/playwright/index.mjs BROWSER_URL=http://127.0.0.1:5184 BROWSER_EVIDENCE=.flow/evidence/fn13/candidates/viewer/browser-final npm run test:browser — actual process exit 0, node .flow/evidence/fn13/candidates/viewer/morph.mjs .flow/evidence/fn13/candidates/viewer/spruce-morph-visible-first first — exit 0, inspected, node .flow/evidence/fn13/candidates/viewer/morph.mjs .flow/evidence/fn13/candidates/viewer/spruce-morph-visible-middle middle — exit 0, inspected, node .flow/evidence/fn13/candidates/viewer/morph.mjs .flow/evidence/fn13/candidates/viewer/spruce-morph-visible-motion motion — exit 0, inspected; 8 warmup/120 valid settled samples, node .flow/evidence/fn13/candidates/viewer/capture.mjs .flow/evidence/fn13/candidates/viewer/oak-final oregon-white-oak — exit 0, 17 frames, node .flow/evidence/fn13/candidates/viewer/navigation.mjs .flow/evidence/fn13/candidates/viewer/spruce-navigation-final norway-spruce — exit 0, 480 real-RAF rows, no runtime/detail/residency errors, node .flow/evidence/fn13/candidates/viewer/lifecycle.mjs .flow/evidence/fn13/candidates/viewer/lifecycle-final — exit 0, all four checks passed, node .flow/evidence/fn13/candidates/viewer/capture.mjs .flow/evidence/fn13/candidates/viewer/spruce-measure-final norway-spruce measure — exit 0, 8 warmup/120 valid all-pass samples; diagnostic desktop contention, node .flow/evidence/fn13/candidates/viewer/capture.mjs .flow/evidence/fn13/candidates/viewer/oak-measure-final oregon-white-oak measure — exit 0, 8 warmup/120 valid all-pass samples; diagnostic desktop contention, Rejected blend and inconclusive capture attempts are explicitly retained in .flow/evidence/fn13/candidates/viewer/RESULT.md
- PRs:
## Owner-authorized accelerated implementation
Owner explicitly requested parallel subagents and higher reasoning to finish the renderer. Implement disjoint portions in isolated worktrees, with conductor integration and Flow completion for this one task. Runtime lane owns reusable src/browser/webgpu renderer/stage (except temporal files), render-selection, public exports and experiment GPU reexport. UI lane owns harness/GrowerDev, stage camera-pose seam, skeleton-view family conversion and harness/webgpu-stage adapter. Temporal lane owns only src/browser/webgpu/temporal.ts or .js/.d.ts plus its targeted tests. Agree the public runtime interface before wiring; no duplicate renderer. Conductor runs integrated checks after merging. GPU browser checks are serialized to avoid contention; workbench CPU coding can run concurrently.

## Owner-authorized cold-transition scope extension
The actual normal-viewer cold near capture showed canonical needles jumping to different source poses. A four-frame history blend produced duplicate needles and was rejected and reverted. The owner authorized the narrow metadata extension needed to preserve deterministic source owner/station correspondence and replace the pose jump with bounded geometry movement; acceptance still requires actual normal-viewer validation.

Additional Touches: crates/telperion-core/src/foliage/placement.rs, crates/telperion-wasm/src/lib.rs, src/browser/assembly-engine.js, src/browser/assembly-engine.d.ts, src/browser/tree-compiler.ts, src/browser/canonical-library.js, src/browser/source-owners.test.ts, src/browser/canonical-metadata.test.ts, experiments/fn13-rendering/tree-compile-check.mjs. Keep source-page culling/order unchanged, retain only metadata needed for the bounded camera-local transition, and do not expand the whole crown.
