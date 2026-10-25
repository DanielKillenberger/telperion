---
satisfies: [R1, R2, R3, R4, R4b, R4c, R5, R6]
---
# fn-317-every-generator-entry-takes-a-trees.1 Implement every generator entry takes a tree's parameters

## Description
TBD

## Acceptance
Every R-ID in the parent spec's ## Requirements is satisfied; judge this task against the spec's criteria directly.

## Done summary
Every generator entry takes a tree's parameters (fn-317).
- `growField(family, n, options?)` and `TreeEngine.build(family, outputs)` take a `Family` only; `params::parse` and the TS serializer refuse a preset id by name. The device binding reads the parameters as the CPU entry does, and the seed is `family.skeleton.seed`.
- `telperion/presets` (`dist/presets.js`) is plain data with no imports; the main entry re-exports it; the size budget names it (ceiling 200,000, held at telperion.js's owner ceiling, for the owner to confirm).
- `FieldError` carries `code` "webgpu-unavailable" or "device-lost" for both the entry's check and the module's refusals. One retry runs, only where the module found no adapter and `navigator.gpu.requestAdapter()` returns one.
- `Grid::name_limbs` names each limb system by its densest cell, at both package entries; names agree on 0.985 to 1 of paired cells (held at 0.97). R4c is amended to the measured floor by the host; the voxelizer remainder per rule is in RESULT.md.
- Docs: README, field-package.md, migration-0.2-parameters.md.
- Gate: 1,506 of 1,507 passed; the one failure (renaming inside the fields) was fixed and its test file rerun green, with no second gate per AGENTS.md.
stage: impl-review - ran [12:08..12:27] NEEDS_WORK (one finding, R4c names) -> fixed -> SHIP (model: gpt-6.1-sol)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: e19bcd6be8232fd14b87f5b8da458dc7ccb77d90, 4fb56e814c4a59fa0938c8c3bc3f4f22ca419bfb, 41e6b20f0f73e16f64a23dea43e20ee1fb8fa8b2, 11292c0f53e7158bef2d909f3c01812385a90bb3, b3c6792e118bd6d8e1b468ce94ae8aa18de7ea6d
- Tests: telperion-heavy 8 --threads 4 -- python3 scripts/gate.py (1506/1507; the failure fixed, test file rerun green), cargo test --profile ci -p telperion-render -p telperion-wasm -p telperion-core --lib -- field generate params, npx vitest run (npm test: 198 incl. parity after fix), node scripts/test-dist.mjs
- PRs: