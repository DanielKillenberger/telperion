---
satisfies: [R8b]
---
# fn-257-the-site-grows-its-trees-on-the-device.14 The package's field entry takes a grid and returns it

## Description
R8b: `telperion/field` takes the grid (resolution over the tree's bounds) and returns it; arbitrary-cube queries go, with a migration note beside `cut`'s.

**Size:** M
**Files:** `crates/telperion-render/src/web/field.rs`, `src/field/index.ts`, `src/field/field.test.ts`, `src/field/voxelize.ts`, `src/field/voxelize.test.ts`, `src/probe/field/worker.ts`, `docs/field-package.md`, `README.md`, `scripts/test-dist.mjs`, `scripts/artifact-budgets.json`
**Touches:** [crates/telperion-render/src/web/field.rs, src/field/**, src/probe/field/**, docs/field-package.md, README.md, scripts/test-dist.mjs, scripts/artifact-budgets.json, crates/telperion-wasm/src/**]

### Approach
- `WebField`: `grow(species, seed, limb_order)` then `grid(n)` (or the grid named in the grow call; one call shape, chosen to match the device pass) returns the cells' answers; `query(cells)` goes.
- TS: `FieldTree` / `FieldAnswer` types for the grid; `voxelize` consumes the grid; the probe worker times grow + grid.
- If the wasm package's CPU `tree.field` read the deleted core index (task 13), it takes the grid the same way, with its migration note.
- Docs: `docs/field-package.md` contract and migration note beside `cut`'s; README's field section; artifact budgets re-measured.

### Investigation targets
**Required**:
- `crates/telperion-render/src/web/field.rs`, `src/field/index.ts`, `docs/field-package.md`

## Acceptance
- [ ] The package's field entry takes the grid and returns it; no cube query in the API; TS types and README updated; migration note beside `cut`'s.
- [ ] `src/field/field.test.ts` (Node, no WebGPU refusal by name) and voxelize tests green; `scripts/test-dist.mjs` green; artifact sizes within budget or the budget's change measured and named.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
