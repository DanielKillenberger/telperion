---
satisfies: [R1, R5]
---
# fn-24-continuous-tree-space-habit-and-leaf-as.4 The flat habit on the wire, the panel and the tests

## Description
Restore the TypeScript surface to the flat trait shape after tasks 1, 2 and 3 remove the three enums from the core: regenerate the preset metadata, drop the kind label from the panel so every trait renders through the generic number-field mapping (R5), and rewrite the tests and probes that assert the old wire.

**Size:** S
**Files:** `scripts/build-wasm.mjs`, `src/browser/presets.generated.ts` (regenerated), `harness/GrowerDev.tsx`, `harness/params.ts`, `harness/params.test.ts`, `tests/browser/bindings.mjs`, `README.md`, `package.json`
**Touches:** [scripts/build-wasm.mjs, src/browser/presets.generated.ts, harness/**, tests/browser/bindings.mjs, README.md, package.json]

### Approach
- Remove the tagged-union branch (`'kind' in value`) from the type inference in `scripts/build-wasm.mjs` around line 15; flat numeric objects already infer generically. Regenerate `src/browser/presets.generated.ts` with `npm run wasm:build`.
- Delete the label line at `harness/GrowerDev.tsx:306`; the generic mapping at lines 307-315 already renders every numeric habit field, so extend the same mapping to the element and canopy trait objects instead of hand-writing inputs.
- Update `harness/params.test.ts:107` to assert the flat habit shape and the default rows; update the invalid-input probe at `tests/browser/bindings.mjs:143` so any `kind`, anatomy or attachment tag is rejected naming the field.
- Docs that go stale with the enums: the architecture paragraph at `README.md:60` and ownership table at `README.md:52`, and the description at `package.json:5`, now describe one builder from continuous traits with attractor pull as a trait.

### Investigation targets
**Required** (read before coding):
- `harness/GrowerDev.tsx:300-330` — the label line and the generic numeric mapping to reuse
- `scripts/build-wasm.mjs:1-33` — metadata to TypeScript generation
- `harness/params.test.ts:100-115` — the default-habit assertion

**Optional** (reference as needed):
- `tests/browser/bindings.mjs:130-150` — the invalid-input probe
- `README.md:45-100` — architecture and headless sections

## Acceptance
- [ ] `presets.generated.ts` carries flat numeric habit, element and canopy trait fields and no `kind`, `anatomy` or `attachment` string
- [ ] The panel shows every habit, element and attachment trait as a numeric control on every preset with no kind or anatomy label; no renderer file changed
- [ ] Harness tests and the browser binding probe pass against the flat shape; the probe rejects a stale tag naming the field
- [ ] README architecture prose and the package description describe one builder from continuous traits
- [ ] `npm run wasm:build && npm test` and `npm run typecheck` pass

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
