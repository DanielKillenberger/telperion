---
satisfies: [R1, R2, R3]
---
# fn-258-a-prepared-tree-meets-masters-memory.1 A prepared tree meets master's memory and frame rate

## Description
Implement fn-258 R1-R3 as written; R1's byte and frame-time breakdown is committed before any cut.

## Acceptance
The spec's R1-R3.

## Done summary
R1's byte and frame breakdown (BYTES.md, GPU and CPU) was measured before any cut. Five R2 cuts landed, each naming its consumer: build scratch and the pool are released after preparation; the sun and the camera share one set of surfacing buffers; coarser error scales are measured only when the finest overruns; the count pass reuses the measure pass's counts; each view's budget grows from what the view asked, per kind (host decision 2026-10-10). The cuts take F4's process GPU memory from 839-4,849 MB to 583-1,355 MB at default age, seed 1. R3 was redone at default age and at master's age over both seeds, with motion and with coarser-frame counts (none at the hero pose; 0-9 of 30 zoom frames and 0-4 of 120 close-turn frames drawn at scale 2 for one frame). The remaining losses against master are the owner's, in fn-259 (RESULT.md). Astra: AGREE WITH NOTES.

stage: impl-review - ran (codex: three-reviewer panel, then re-reviews; SHIP)
Tier: host-dispatched worker (actual model: claude-opus-5-5)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 708c7f2dd0c6d556e91f285cd7cc0d7cf1511bf8, 1b0e7c6656e390aea1248581503d3a5039c41c72, a371641ed59be78f7224e0a292ee8d538541bb6f, 7e44ba29c6406aca8332d32eaea9458b3bcc9e40, d2566b5c01136bc10654ba4ff8413b5475d2fe75, 3f936c403c516145e2af242e95325a868e2474e8, 3731bb3c1270c8392131677442ba0b3c2ec63e02, 000b5c75223373469f4f50a5f1b12c82cc7bb4b4, 0eb9d39ae7f754cba2b3835a8a8532f1d3ec4cff, 2b8615a74604413d4de4f62ddccb145fc0560169, bebf8a173ee4d9583fe077b29aa54455db1be65f, 11f20d7c6fdef74a9873226b088b27a30e526b17, 1ba16e216e1a8fe6d327c565ee50c53a7178cbda, f7430fe0cdbfb2e4cd3e282e4c9b5727d58e9282
- Tests: cargo test --profile ci -p telperion-render --no-fail-fast (GPU lock): 49/49 green, cargo test --profile ci -p telperion-core/-space/-field/-wasm/-jev --no-fail-fast: green, npm test: 155/155 green
- PRs: