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
TBD

## Evidence
- Commits:
- Tests:
- PRs:
