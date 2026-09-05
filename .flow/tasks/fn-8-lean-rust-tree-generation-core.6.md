---
satisfies: [R1, R4, R5, R7]
---

# fn-8-lean-rust-tree-generation-core.6 Switch the browser harness to the Rust core

## Description
Join the completed stages and cut the browser over to Wasm with a thin Three adapter.

**Size:** M
**Files:** browser adapter, public entry, harness build bridge and UI integration tests
**Touches:** [crates/telperion-wasm/**, src/index.ts, src/browser/**, harness/skeleton-view.ts, harness/skeleton-view.test.ts, harness/GrowerDev.tsx, harness/params.ts, harness/params.test.ts, package.json, package-lock.json, vite.config.ts, tests/browser/**]

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


## Acceptance
- [ ] Actual browser builds ordinary trees, both giant presets and comparison through Rust.
- [ ] Dials, diagnostics, clay/foliage modes and measurement controls remain functional.
- [ ] Load/build/replacement error tests leave coherent state and no stale scene replacement.
- [ ] Matched full-tree equivalence and clay captures show no unexplained migration drift.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
