---
satisfies: [R4, R5]
---
# fn-91-fast-tree-generation-as-a-core-engine.15 Use resident GPU generation in the interactive renderer

## Description
Owner reported slow renderer after serving it. The mature harness still calls synchronous Stage.setTree/Renderer.setTree, bypassing the qualified GPU path. Wire mature harness generation to setTreeGpu with serialized/coalesced latest-parameter scheduling, cleanup-safe async updates, existing capability fallback and correct first-frame camera behavior. Preserve opt-in growth path. No engine optimization or changes to benchmarks. Verify rapid species/seed edits, error recovery, disposal and an actual rendered GPU-backed tree. Update PR 50 with the fix after passing focused tests/typecheck/live QA. Use Astra low; leave unrelated fn94–97 files untouched.

## Acceptance
Mature UI invokes resident GPU generation; latest requested parameters win without busy errors or stale UI updates; disposal safe; growth unchanged; focused scheduling tests and typecheck pass; real browser render and species/seed switching verified.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
