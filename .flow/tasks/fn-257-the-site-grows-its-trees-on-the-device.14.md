---
satisfies: [R8b]
---
# fn-257-the-site-grows-its-trees-on-the-device.14 The package's field entry takes a grid and returns it

## Description
R8b: `telperion/field` takes the grid (resolution over the tree's bounds) and returns it; arbitrary-cube queries go, with a migration note beside `cut`'s.

**Size:** M
**Files:** `crates/telperion-render/src/web/field.rs`, `src/field/index.ts`, `src/field/field.test.ts`, `src/field/voxelize.ts`, `src/field/voxelize.test.ts`, `src/probe/field/worker.ts`, `docs/field-package.md`, `README.md`, `scripts/test-dist.mjs`, `scripts/artifact-budgets.json`
**Touches:** [src/browser/**, tests/browser/**, crates/telperion-render/src/web/field.rs, src/field/**, src/probe/field/**, docs/field-package.md, README.md, scripts/test-dist.mjs, scripts/artifact-budgets.json, crates/telperion-wasm/src/**]

### Approach
- The Rust half (Wasm field exports, `WebField`) moved to task 13 (host); this task is the TypeScript entry, its types and tests, the voxelizer, the docs, `test-dist`, the budgets and the browser binding tests against the rebuilt Wasm.
- `WebField`: the field request names the grid (`n`, the spec's grid definition); it returns `{ n, cell, origin }` and one answer per cell; `query(cells)` goes. `n` outside 3..=256 or past the device's limits is refused by name before any work (Node test for each).
- TS: `FieldTree` / `FieldAnswer` types for the grid; `voxelize` consumes the grid; the probe worker times grow + grid.
- `voxelize.ts`'s `grid` stays the definition; `voxelize` reads the returned grid instead of building cells to query.
- The main package's CPU `tree.field` (`src/browser/core.ts:102`: `query(cells)`, `snapshot()`; `crates/telperion-wasm/src/**` exports) takes the grid the same way (spec, "Both executors"); `snapshot()` goes; `tests/browser/bindings.mjs:74-113,212-245` move to the grid; README's CPU field section (`README.md:26-43`) and its migration note.
- Docs: `docs/field-package.md` contract and migration note beside `cut`'s; README's field section; artifact budgets re-measured.

### Investigation targets
**Required**:
- `crates/telperion-render/src/web/field.rs`, `src/field/index.ts`, `docs/field-package.md`
## Acceptance
- [ ] The package's field entry takes the grid and returns it; no cube query in the API; TS types and README updated; migration note beside `cut`'s.
- [ ] The returned grid's `n`, `cell`, `origin` and cell order equal voxelize's `grid` for the same bounds (test).
- [ ] The main package's `tree.field` returns the grid; no `snapshot()` or cube query; `tests/browser/bindings.mjs` green against the rebuilt Wasm.
- [ ] `src/field/field.test.ts` (Node, no WebGPU refusal by name) and voxelize tests green; `scripts/test-dist.mjs` green; artifact sizes within budget or the budget's change measured and named.
## Done summary
R8b on the TypeScript side. `telperion/field`'s `growField(species, seed, n, { limbOrder?, source? })` now resolves to the grid: `{ species, seed, n, cell, origin, answer }`. It copies the WebField's words and frees the handle, so nothing stays on the device. `query(cells)`, `bounds`, `release()` and `deviceCells` are gone. A side outside 3..256 is refused by name (`field grid side`) before the device check. `voxelize.ts` holds the one grid definition (`Grid`, `side`, `grid(bounds, n)`, `centre`), and `voxelize(field, dials)` reads the returned grid. The main package's `tree.field` takes `{ n, limbOrder? }` and returns `{ n, cell, origin, answer }` as owned copies. `snapshot()`, the cube query, their TypeScript bindings, `fieldBounds`, `fieldBytes` and `fieldSource` are gone, and `Diagnostics.field` names the grid. README, docs/field-package.md, the probe worker, tests/browser/field.mjs, test-dist and the binding tests all moved to the grid, with the migration notes beside `cut`'s.

The release tip 23fb243e3 (fn-291) was merged in at the conductor's request (4b371f99c). The resolution keeps fn-291's refusals, budgets and chunked wood beside R8's grid field: the grid slots 8/19/20/25 stay, `chunk_ptr`/`chunk_len` come in, and no snapshot or query export comes back. test-dist runs fn-291's 24 cases and the field entry's two refusals. The inventory holds both sides' sites.

Tests (measured, after the merge):
- `cargo check --workspace --all-targets` and `cargo check --target wasm32-unknown-unknown -p telperion-wasm -p telperion-render`: green. One warning, an unused `math` import in examples/occupancy_audit.rs, was already on the pre-merge HEAD.
- `cargo test --profile ci -p telperion-core --test generation_limit_guard`: 2 passed. `cargo test --profile ci -p telperion-wasm`: green. Core `pipeline::tests`: 9 passed, both the field refusal test and fn-291's spare-capacity test among them.
- `npx tsc --noEmit` and `npx tsc -p tsconfig.build.json --noEmit`: green. Pre-task baseline: red, inherited from task 13.
- The src/field Vitest files: 12 passed. They include the new voxelize test that holds TS `grid` to the core's frame (n, cell and origin to 1e-12, cell order on every node's cell), and the side refusals for 2, 257, 24.5 and NaN.
- `npm run build` and `node scripts/test-dist.mjs`: green. All 24 preset/seed cases are at or under 3,584 MiB, the 64-side Telperion is refused by name, and field.js refuses without WebGPU and refuses a side of two by name.
- `node scripts/artifact-budgets.mjs`: every artifact under its ceiling, no budget change.
- tests/browser/bindings.mjs, run alone against the rebuilt Wasm: every field check passes. The checks that stay red were measured red on the release tip itself (detached worktree, own target dir), and none of them reads field output.
- tests/browser/field.mjs: updated, not run (it needs WebGPU in a browser).

OPEN_ITEM: tests/browser/bindings.mjs: 11 checks read no field output and are red on the release tip 23fb243e3 too (measured). They are `ordinary has local branches and a leaf plan`, `bad height`, `caps explicitly partial`, `valid empty outputs`, `complete identity catalogue`, `the beech in work is not listed`, `the beech in work is not built by name`, `refuses crookedness` (oak, spruce, birch) and `native empty recovery`. The suite is in no CI job; the fixture has rotted (`growth.maxNodes` is ignored, the beech is listed, crookedness is not range-checked).
OPEN_ITEM: tests/browser/bindings.mjs: the species loop's `maxInstances = 40000` is refused with `foliage instance budget` on this branch and on the release tip (measured), so the strict suite stops there.
OPEN_ITEM: tests/browser/bindings.mjs: `an empty tree has no field` is red because `maxNodes = 0` no longer empties the tree. The release tip's `empty snapshot` checks are red for the same cause (measured there).

Device-limit refusal: Node has no device, so it is not a Node test. It is the native `within` check from task 13.
Worker decisions: `n` is a positional argument. The grid definition lives in voxelize.ts, so dist/field.js imports `side` from voxelize.js. The probe page's "Query" column reads "-", because field-probe.html is outside the Touches.
Follow-ups (outside the Touches, still on the removed query and snapshot): scripts/benchmarks/field.mjs, generation.mjs and generation-inputs.mjs, experiments/voxel-field/voxels.mjs, tests/migration/README.md's `{ field: true }`, and field-probe.html's column header. Feature-map routes: unchanged.

stage: impl-review - ran (codex, three lenses SHIP with 0 findings on the task diff; three lenses SHIP with 0 findings on the merge delta c5f859e36..4b371f99c)
Tier: (none given) (model: claude-opus-5-5)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 544d0d1597f403b3a1104a29631b628f89378db2, 5c5169f0b8a8d4fe7f92530837062d90a61a7fbe, df35b25cde31991a99e325e4e3d7fe6d2b29f3c6, 6697f7d255e73be3334d9d8c63978268a2d0319d, a73fbb802d3077d81eff4c8aa8a97be0c326aadd, 6e5af736070a0bc435f2d7848fb6c25a4fa689a3, 2bf2390356bb58be004e960217c9bd39cbbaa77f, b37ed570169253d597ec85a3bf616408cda2875b, 3e10c49bf72d40e012acf502e873319444caa4fa, 2db376e1ec66a30c5087b46ca54706d2a915b315, 38a18b753801ee9350381d5fc4e486f4a9cdc1b2, 9d344192c8ea7fc503d2cfc6d253f300c01b33f1, b9e4377fb0c30daaed035e53b0a7028d7aa20458, d9cf2d59de068ad4f91624810aff07ee50f0d371, c9f788b92996106a10d7e818add40eeafcfc2bca, de6cac85d57cf93d41154eb60b024ebf55a80b1c, 5e9910d34ffdce974c7dd647844300c2220a4230, bf4527a2311eb5d5ec78af1b4eb0bf03a323f760, 9eed854d0821071fe48745704ed4a6fa6ecde247, 5b2636d622676362cbf955e5c416c8816c235da7, cab9e7ee0f9b368724f5f8650e0d5db69141e197, e73108ea448a086fef7400891e50d194f44c2a2b, 2f2452469f3ec66fc28aff167651c31dcfd6ea54, e8418b92872567062830291f8df419ceb81e643d, bf1921372937c766d4d46a352fd30e95ee84d2b9, 69271d9399c9066a449b636070df9643224a3447, e0a0c70d5394546195dfbe5c67dd8e5b6d0f9412, 6d374b93c8a9e2fc97ecbe3118498a4b697d5711, 293f6225bd64eab325c7dde29f17326742377fcd, cd0f2bd6d3664c99001cafd2688028fd973fc571, 58d6f02540fa0ae8c0c6c13ba97c9ed1d851f196, 500f0108ba826ee967bc96241112a0d678af6101, 007fb6155065fe831ebfe9c29ed403ee47aa14bf, 8c706cd63862f36764faad02e520513d39a46c87, 84233482789b36437fbb506b0c796d18e1164319, fff95943e08adfb649721d05cf05f63a6ffe542a, 50ea727f27024d5754dae21decc4c93c66c2ca19, 51700113c26cc84046dc54b992ebab2c2478d34a, 141a39f9cc9b2f56251faaece40ab732644fcc46, 6473b5e9ef9e09c380b911a63f5f5cb81042004e, 1331e63ddbfaa5e1ed423a5a6ab9618f834faeeb, 72af1afb79b602282f778c9c21becf2929b87406, 9e4cddc1f62e80fc2fcdabc68471c16be76c9892, 9a915b0e4b14b58b8bf4382355a9aab05e25e064, ea3b6e0ab62b4a5c863df18794b23a439880479b, 9445fc8e8c40d32a2b0780e742452fe8938de924, c3df8ed573e938a078526f63da0744877b0cb6dc, c5f859e360fa0389b44dc771dc91a864d16c0dde, b741134ad68ba834af1aea4bb2c98f4de3982e66, 741cada44001c472364f164697e0425b25239acc, 23fb243e3a1d63682c73c5f97bea9367f767aa1e, 4b371f99cd052c2cdad74ea964fb59f6c70194be, 29c41abf9cb0ad2edfe8d32c61302faacce7a6a2
- Tests: npx vitest run (src/field/*.test.ts), npx tsc --noEmit; npx tsc -p tsconfig.build.json --noEmit, node scripts/test-dist.mjs, cargo test --profile ci -p telperion-core --test generation_limit_guard; cargo test --profile ci -p telperion-wasm, tests/browser/bindings.mjs (field checks green; inherited non-field reds measured on release tip, see OPEN_ITEM)
- PRs: