---
satisfies: [R1, R4, R5, R7]
---

# fn-8-lean-rust-tree-generation-core.6 Switch the browser harness to the Rust core

## Description
Join the completed stages and cut the browser over to Wasm with a thin Three adapter.

**Size:** M
**Files:** browser adapter, public entry, harness build bridge and UI integration tests
**Touches:** [crates/telperion-wasm/**, Cargo.lock, .gitignore, src/index.ts, src/browser/**, harness/skeleton-view.ts, harness/skeleton-view.test.ts, harness/stage.ts, harness/stage.test.ts, harness/GrowerDev.tsx, harness/params.ts, harness/params.test.ts, package.json, package-lock.json, vite.config.ts, scripts/build-wasm.mjs, scripts/test-wasm.mjs, tests/browser/**]

## Approach
- Replace production generator imports with Wasm operations and construct Three objects solely in the adapter. Preserve all currently supported controls, compare mode and diagnostics.
- Keep parameter translation thin; adapt the existing panel schema to the new core contract without duplicating generation or presets.
- Handle initialization and build errors visibly; retain a coherent previous scene and dispose replaced/stale owned resources. If rebuilds become asynchronous, attach request identity and discard stale completions; a worker is not required.
- Preserve stage timing semantics and explicitly include transfer and materialization in full-build measurement.
- Run the shared full-pipeline equivalence harness and matched clay captures on the actual browser; record remaining pokey geometry as unchanged reference behaviour.

## Investigation targets
**Required:**
- `src/index.ts:27`
- `harness/skeleton-view.ts:320`
- `harness/GrowerDev.tsx:73`
- `harness/params.ts`
- `harness/stage.ts:643`
- `harness/skeleton-view.test.ts`

## Approved capture alignment
The rewritten parent capture is authoritative. Baselines diagnose drift; exact old topology or bytes are not a compatibility requirement, and known structural defects need not be reproduced. Preserve meaningful botanical and geometric invariants and report visual/numeric differences. Keep the core lean and simple.

Integrate the full native API into browser bindings here, including field queries from task 8. Exercise mesh-free field sampling in a browser integration test even though the viewer renders meshes; its public consumer API must expose it. Cover malformed requests, buffer validation and memory reuse in binding tests.


Stage integration finding: `stage.setTree()` currently disposes the previous scene before calling the builder. Make replacement transactional so a Rust/binding/build failure preserves the previous subject; test this behavior. The foundation Wasm test must migrate with the replaced ABI.

## Acceptance
- [ ] Actual browser builds ordinary trees, both giant presets and comparison through Rust.
- [ ] Dials, diagnostics, clay/foliage modes and measurement controls remain functional.
- [ ] Load/build/replacement error tests leave coherent state and no stale scene replacement.
- [ ] Matched full-tree equivalence and clay captures show no unexplained migration drift.

## Done summary
Implemented the complete Rust/Wasm browser consumer API and cut the live viewer over to it. Independent owned representations, mesh-free wood/foliage fields, native presets/bounds/diagnostics, transactional scene replacement and recoverable load/build errors preserve the viewer controls and comparison.

baseline: none (task and parent define no Quick commands). Formatting, strict clippy, typecheck, build, 100 harness tests, the migrated native diagnostics test, actual Chromium binding/UI checks and all four full-tree browser cases pass. Native request/field validation rejects malformed, nonfinite and stale inputs; empty and copied/released/rebuilt outputs are covered. Field-only constructs neither wood surface nor render transfer buffers. Red reproductions and successful logs are linked in the evidence file.

Matched final FN6 clay captures are visually unchanged: ordinary is pixel-identical, Laurelin differs at one pixel by one channel level, Telperion at438/1.6M pixels with mean channel error0.000495/255. Indices and retained foliage membership match. Small coordinate differences amplify into at most0.06789degrees of normal change near small triangles; native normals exactly match Three recomputation on new positions. Existing pokey geometry is unchanged reference behavior. Detailed evidence: /tmp/fn8-browser/visual-diff.json, /tmp/fn8-normal-diagnose/migration.json, /tmp/fn8-browser-headless/migration.json.

Ordinary/giant matched clay checks ran headed before the user's permission concern; subsequent functional/error/retry and full comparison checks ran headless. No permission auto-accept or bypass flags were added. The exact reported Allow prompt remains unidentified; the test's separate cross-port fetch failure was resolved through same-origin serving. A stopped software-rendered attempt is recorded as inconclusive, not green. All browser instances are closed.

API, ownership, build, normal-drift diagnosis and timing details for task7: /home/daniel/Projects/telperion/.git/flow-notes/fn8-rust-20260905/browser.md. No performance conclusion is drawn from single runs. Task7 owns paired warm measurements/memory/rendering analysis, old TypeScript deletion/test-owner migration and published docs; old production files remain present but are absent from the live browser generator path.

stage: impl-review - skipped(policy: parallel-wave; conductor owns lifecycle, REVIEW_MODE=none)

Task remains in_progress. No review verdict, tracker mutation, integration or flowctl done was performed. Workspace: /home/daniel/Projects/telperion/.worktrees/fn8-browser.

Conductor merged the browser cutover, reran the complete headless binding/UI proof and typecheck on the integrated tree, and inspected both matched giant clay captures.
stage: impl-review - skipped(user: none)
stage: wave-join - ran(merge and integrated browser/typecheck verification)
stage: plan-sync - skipped(config: false)
## Evidence
- Commits: 2ced9cbd3ea08a2328c73d037d93ee920e2e7ea6
- Tests: baseline: none (task and parent define no Quick commands), cargo test -p telperion-wasm (1 passed; /tmp/fn8-browser-native-tests-final.log), cargo fmt --all -- --check (passed), cargo clippy --workspace --all-targets -- -D warnings (passed; /tmp/fn8-browser-clippy.log), npx tsc --noEmit (passed), npm run build (passed; /tmp/fn8-browser-build.log), npx vitest run harness/skeleton-view.test.ts harness/params.test.ts harness/stage.test.ts (100 passed; /tmp/fn8-browser-harness-final.log), node scripts/test-wasm.mjs (headless Chromium binding and UI error/retry checks passed, including automatic build/server lifecycle; /tmp/fn8-browser-test-entrypoint.log), REFERENCE_DIR=/tmp/fn8-surface-reference BROWSER_EVIDENCE=/tmp/fn8-browser-headless node tests/browser/migration.mjs (all 4 cases passed headless Chromium; /tmp/fn8-browser-migration-gpu-headless.log), Headed Chromium matched ordinary/Telperion/Laurelin geometry, retained foliage and clay captures; /tmp/fn8-browser/visual-diff.json and /tmp/fn8-normal-diagnose/migration.json, Red-to-green regression evidence: /tmp/fn8-browser-transaction-red.log, /tmp/fn8-browser-field-selection-red.log, /tmp/fn8-native-bounds-red.log and /tmp/fn8-native-bounds-green.log; failed UI retry was offscreen before panel placement fix, Explained normal drift: native normals exactly equal Three recomputation on new positions; old float32 coordinate rounding changes normals near tiny triangles; no normal implementation change, Inconclusive attempts retained: cross-origin fixture fetch failed in headless; replaced test serving with one origin. Software-rendered giant capture run stopped before completion; subsequent headless RTX run passed. No false green receipts recorded., flowctl gate classify --base ade8e702bcc8670ccf8f5484bf63b9699bb7c7ea (FULL; no spec-defined test/smoke receipt commands), git diff --check (passed), Conductor integrated npm run rust:test:wasm: all headless binding/UI checks passed; npm run typecheck passed; both giant matched clay captures inspected
- PRs: