---
satisfies: [R3]
---
# fn-319-the-release-audits-slice-lands-before.4 The renderer takes a Family, and the field types have one definition

## Description
The four renderer methods take `Family` and serialize with `wire`; `FieldFrame`/`FieldQuery`/`Bounds` in `core.ts` and `Grid`/`FieldAnswer`/`FieldGrid`/`Bounds` in `src/field/` become one set.

**Size:** M
**Files:** `src/browser/render.ts`, `src/browser/core.ts`, `src/field/index.ts`, `src/field/voxelize.ts`, a types-only module, `harness/rust-stage.ts`, `harness/grow-in.ts`, `harness/GrowerDev.tsx`, `harness/family.ts`, tests, `scripts/benchmarks/*.mjs`, `tests/browser/render.mjs`, `docs/migration-0.2-parameters.md`, `README.md`
**Touches:** [src/browser/**, src/field/**, harness/**, scripts/benchmarks/**, tests/browser/**, docs/migration-0.2-parameters.md, README.md]

### Approach
- `render.ts:12-14,180-284`: interface and implementation take `Family` and call `wire` (`src/browser/wire.ts:10-18`); the Rust `WebRenderer` keeps its string.
- Replace the harness's `familyJson` (`harness/family.ts:40-47`) with `wire`; `harness/grow-in.ts:87,113` compares families with `!==`, so key them by their wire string.
- Callers: `harness/rust-stage.ts:154-156`, `harness/grow-in.ts:98-150`, `harness/GrowerDev.tsx:84`, `harness/family.test.ts`, `harness/grow-in.test.ts:27-56`, `src/browser/render.test.ts:59-131`, `tests/browser/render.mjs:175-184`, `scripts/benchmarks/generation-gpu-smoke.mjs:26-60` (its "not JSON" assertion becomes the wire refusal), `scripts/benchmarks/mature-generation.mjs:30,48`.
- One definition of the frame, answer, grid and bounds in a types-only module both entries import without loading the wasm (`src/field/index.ts:23` imports the module); check `tsconfig.build.json` and the vite entries; keep the exported names the package already publishes (`src/field/index.ts` exports `FieldAnswer`, `FieldGrid`).
- Add the renderer change to `docs/migration-0.2-parameters.md`; fix `README.md` prose (`:95-135`, `:292`).

### Investigation targets
**Required:**
- `src/browser/render.ts`
- `src/browser/core.ts:15-90`
- `src/field/index.ts:20-60`
- `src/browser/wire.ts`
**Optional:**
- `scripts/test-dist.mjs:120-140`, `src/browser/module.test.ts:37-46`

### Acceptance
- [ ] Typecheck clean; every caller updated; a preset id string is refused by `wire`'s message
- [ ] One definition per field type; the package's public type names unchanged
- [ ] Export, dist and harness tests pass; migration notes updated

## Acceptance
- [ ] TBD

## Done summary
The renderer's `setTree`, `setTreeGpu`, `admission` and `beginGrowth` now take a `Family` and serialize it with `wire`. A preset id is refused with wire's message ("not a preset id: pass presetById(id)") before the renderer sees it. The harness's `familyJson` is deleted, and grow-in keys a held growth by the family's wire text. `Grid`, `FieldAnswer`, `FieldGrid` and `Bounds` are now defined once, in the types-only `src/field/types.ts`. The main entry's `diagnostics.field` is a `Grid` and its `field` a `FieldGrid`, and the package's public type names are unchanged.

- Tests: `src/browser/render.test.ts` checks that each of the four methods refuses a preset id before anything is built. `harness/grow-in.test.ts` checks that a fresh copy of the same parameters plays its year on the held growth. `harness/family.test.ts` checks that a NaN dial is refused under its own name.
- Decision: `wire`'s refusal of a nonfinite value now names the key (`Tree parameters must be finite: "trunkRadius" is NaN`). This keeps the dial-naming that `familyJson` gave the harness, and its test. The old message is kept as the prefix, and nothing else asserts on it.
- Decision: `FieldFrame` and `FieldQuery` are deleted from `core.ts`. `src/index.ts` never exported them, so no public name is lost.
- Caveat: the browser scripts (`tests/browser/render.mjs` and the two benchmarks) have been syntax-checked only. They need WebGPU in a browser, which the host rules forbid. In `generation-gpu-smoke.mjs` the "not JSON" check is now the wire refusal of `'oregon-white-oak'`.
- Generated modules: a fresh worktree needed `render:build` and `wasm:build` before the baseline could run. The friction is logged in `.flow/evidence/fn-319-the-release-audits-slice-lands-before/FRICTION.md` (a new file in this commit; it may conflict with a sibling task's file when the wave is merged).

baseline: green (tsc; vitest src/field, render, module, presets, grow-in, family), measured after the generated modules were built
Tier: session model (model: claude-opus-5-5)

stage: impl-review - skipped(policy: parallel-wave - conductor owns the gate)

Integrated as 7d5b962037485a3860262644fa1d134fe3680ad4 on the spec branch. Review: codex, three lenses (correctness, contracts, integration), SHIP, no findings (rid 5704b7cc).
stage: plan-sync - skipped(config: planSync.enabled=false)
## Evidence
- Commits: 7d5b962037485a3860262644fa1d134fe3680ad4
- Tests: tsc --noEmit (rc 0), npx vitest run src/field src/browser harness (18 files, 148 tests, rc 0), tsc -p tsconfig.build.json && vite build (rc 0), node scripts/test-dist.mjs (rc 0, 24 cases + field refusals)
- PRs: