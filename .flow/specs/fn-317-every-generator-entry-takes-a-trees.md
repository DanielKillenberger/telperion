# Every generator entry takes a tree's parameters

## Goal & Context

The killenberger.com agent porting the grove to `telperion@0.2.0-next.20261024.2131d5b` sent a list (2026-10-25): the field may not serve european-beech, a CPU field for Node tooling, a typed "no WebGPU" error, and the bloom colours without the 1.5 MB CPU engine. On the first item the owner said: "Wouldn't we want the field generator accept params and we just have a helper that fetches params from the species list? We shouldn't hand a species name to the generator and if it's not typed we can't do it. This seems architected badly." Asked whether the string form should go everywhere, the owner chose "Remove everywhere".

What the code does today (checked by the host, 2026-10-25):
- `growField(species: string, seed, n, options)` (`src/field/index.ts`) passes the string to `WebField.grow` (`crates/telperion-render/src/web/field.rs:84`), which resolves it inside the wasm with `telperion_core::presets::by_identity`. A tuned or user-made parameter set cannot reach the field, and the type does not say which ids are valid. This breaks AGENTS.md: "Generator parameters are primary; templates are reusable parameter presets".
- `TreeEngine.build(family: Family | string, outputs)` (`src/browser/core.ts:138`) takes either, and the string goes into the request JSON for the core to resolve.
- The helpers already exist: `presetById(id)`, `PRESETS` and the named constants (`TELPERION`, `LAURELIN`, ...) in `src/browser/core.ts`, built on `presets.generated.ts`. Each preset carries its `material.bloom*` colours.
- The render wasm already depends on `serde_json`; the core has a `json` feature for a `Family`.
- Beech is served today: `field_tests.rs` grows `european-beech` on the device. The agent could not see it because ids resolve inside the wasm.
- A CPU field exists: `TreeEngine.build(family, { field: { n, limbOrder } })` runs the same reference field (`pipeline/field.rs`) in Node and returns `result.field` with the same frame and answers. The field's docs do not say so.
- Without WebGPU, `growField` throws a plain `Error("WebGPU is not available: …")`; a caller can only match the message.

## Requirements

- **R1 (the field takes parameters):** `growField(family: Family, n, options?)` grows the tree the parameters describe, with the seed taken from `family.skeleton.seed` as `TreeEngine.build` takes it, and the same validation and errors as the CPU entry for the same parameters. There is no id lookup inside the wasm. The result carries no species name. A preset is grown by passing `presetById(id)` or a constant.
- **R2 (no entry takes a name):** `TreeEngine.build` takes a `Family` only. Every package entry, test, harness call and doc that passes an id string moves to `presetById`. A string is refused by the type, and at run time with a named error.
- **R3 (presets without the engine):** a `telperion/presets` entry exports `presetById`, `PRESETS`, the named constants and the `Family`/`TreePreset` types as plain data, with no wasm and no engine code. The main entry re-exports them unchanged. CI's size budget names the new artifact.
- **R4 (a typed error):** the field's no-WebGPU and lost-device refusals are an exported error class carrying a stable `code` (`"webgpu-unavailable"`, `"device-lost"`). The message stays as it is.
- **R5 (docs):** the README, the field's doc comments and the migration notes say: the field takes parameters; the CPU field is `TreeEngine.build(family, { field })` for Node and tooling; presets come from `telperion/presets`; the error codes. The 0.1.4 migration note covers the removed string form.
- **R6 (proof):**
  - Every shipped preset grows through `growField(presetById(id), 24)` on the device. Its answers equal today's `growField(id, …)` answers byte for byte at seeds 1 and 5678.
  - A parameter set that changes one dial (not a preset) changes the field.
  - `TreeEngine.build("telperion", …)` is refused by name.
  - `telperion/presets` loads in Node with no wasm fetch.
  - The gate, `npm test` and CI's package job are green. Codex review runs inside telperion-heavy.

## Boundaries

- A field-only wasm build is not part of this spec. It is noted for fn-285.
- No change to what any tree looks like.
- Joins the release (fn-237), after fn-316. A new `next` is published only when the owner asks.
