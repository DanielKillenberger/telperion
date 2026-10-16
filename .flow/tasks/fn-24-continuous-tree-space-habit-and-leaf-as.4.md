---
satisfies: [R1, R5]
---
# fn-24-continuous-tree-space-habit-and-leaf-as.4 The flat habit on the wire, the panel and the tests

## Description
Restore the TypeScript surface to the flat trait shape: regenerate preset metadata, drop the habit/anatomy label so every trait renders as a numeric control (R5), and rewrite the tests that assert the old wire.

## Acceptance
- `presets.generated.ts` carries flat numeric traits and no `kind`, `anatomy` or `attachment` string
- Panel shows every habit, element and attachment trait on every preset; no renderer file changed
- Harness tests and the browser binding probe pass against the flat shape

## Done summary
`scripts/build-wasm.mjs` lost its tagged-variant and string-tag branches; the regenerated metadata is flat. One `Traits` component renders every numeric term of habit, element and canopy; canopy terms already driven by sliders are filtered (`CANOPY_FROM_SLIDERS`), `maxInstances` skipped as a resource limit. npm test 65 passed, typecheck clean.

## Evidence
- Commits: bd0c8a2
- Tests: worker, workspace: npm run wasm:build then npm test - 65 passed, 3 files, worker, workspace: npm run typecheck - clean, worker, workspace: npm run rust:test:wasm - browser binding suite green in headless Chromium, ...
- PRs:
