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
- `WebField`: the field request names the grid (`n`, the spec's grid definition); it returns `{ n, cell, origin }` and one answer per cell; `query(cells)` goes. `n` outside 3..=256 or past the device's limits is refused by name before any work (Node test for each).
- TS: `FieldTree` / `FieldAnswer` types for the grid; `voxelize` consumes the grid; the probe worker times grow + grid.
- `voxelize.ts`'s `grid` stays the definition; `voxelize` reads the returned grid instead of building cells to query.
- The main package's CPU `tree.field` (`src/browser/core.ts`) is unchanged (spec, "The CPU field of the main package").
- Docs: `docs/field-package.md` contract and migration note beside `cut`'s; README's field section; artifact budgets re-measured.

### Investigation targets
**Required**:
- `crates/telperion-render/src/web/field.rs`, `src/field/index.ts`, `docs/field-package.md`
## Acceptance
- [ ] The package's field entry takes the grid and returns it; no cube query in the API; TS types and README updated; migration note beside `cut`'s.
- [ ] The returned grid's `n`, `cell`, `origin` and cell order equal voxelize's `grid` for the same bounds (test).
- [ ] `src/field/field.test.ts` (Node, no WebGPU refusal by name) and voxelize tests green; `scripts/test-dist.mjs` green; artifact sizes within budget or the budget's change measured and named.
## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
