---
satisfies: [R4, R5]
---
# fn-91-fast-tree-generation-as-a-core-engine.15 Use resident GPU generation in the interactive renderer

## Description
Owner reported slow renderer after serving it. The mature harness still calls synchronous Stage.setTree/Renderer.setTree, bypassing the qualified GPU path. Wire mature harness generation to setTreeGpu with serialized/coalesced latest-parameter scheduling, cleanup-safe async updates, existing capability fallback and correct first-frame camera behavior. Preserve opt-in growth path. No engine optimization or changes to benchmarks. Verify rapid species/seed edits, error recovery, disposal and an actual rendered GPU-backed tree. Update PR 50 with the fix after passing focused tests/typecheck/live QA. Use Astra low; leave unrelated fn94–97 files untouched.

## Acceptance
Mature UI invokes resident GPU generation; latest requested parameters win without busy errors or stale UI updates; disposal safe; growth unchanged; focused scheduling tests and typecheck pass; real browser render and species/seed switching verified.

## Done summary
Mature harness generation uses the existing resident GPU API through a per-stage serial queue. Pending requests coalesce, cancelled effects and disposed stages suppress callbacks, and only the current successful build updates statistics and frames the camera. The growth path is unchanged; the unused synchronous Stage forwarding method was removed.

Similar code search: reused Renderer.setTreeGpu and its capability fallback; extended Stage forwarding; no existing harness serial/coalesced scheduler, so added latestBuild with deferred-promise tests.

baseline: green (npx tsc --noEmit). No parent Quick commands defined; focused verification follows conductor scope.
Validation: npx vitest run harness/latest-build.test.ts passes 4/4; npx tsc --noEmit passes; git diff --check passes. New test suite first failed because the scheduler module did not exist; this is a missing-feature reproduction, not a behavioral regression red. Tests cover serial/coalesced latest requests, cancellation, current failure recovery and disposal on success/rejection.

Live browser QA and commit remain with the conductor as explicitly dispatched. Task remains in_progress.
Tier: explicit Astra low.
stage: impl-review - skipped(policy: host-deferred - conductor owns the gate)

Host live QA passed actual React consumer flow (oak, rapid seeds17/18/19, spruce, oak): all4 executed builds Gpu/gpuPositions, maxActive1, CPU calls0, no errors/alerts, live devices1. Focused scheduling+params34/34 pass. Source and exterior screenshot reviewed. Evidence: harness/REPORT.md and live.json. Retained; no engine performance experiment.
## Evidence
- Commits: 85f4731147a8dbafc2d1179e77291e72f17c10d5
- Tests: baseline: npx tsc --noEmit (exit 0), npx vitest run harness/latest-build.test.ts (4/4, exit 0), npx tsc --noEmit (exit 0), git diff --check (exit 0), npx vitest run harness/latest-build.test.ts harness/params.test.ts:34/34 pass, node .flow/tmp/fn91-harness-live.mjs:actual UI oak/rapid seeds/spruce/oak; Gpu all4, maxActive1,cpuCalls0, no alerts/errors, one live device
- PRs: https://github.com/DanielKillenberger/telperion/pull/50