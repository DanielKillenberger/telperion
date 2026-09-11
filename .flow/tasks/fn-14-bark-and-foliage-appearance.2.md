---
satisfies: [R4, R10]
---
# fn-14-bark-and-foliage-appearance.2 The material row on the family and the scene row on the renderer

## Description
Add the material row to every preset as one more family struct with wire rows, and the scene row as a renderer-side value table with a default, a setter, a headless flag and a page setter, so the panel shows both through the generic controls (R4, R10's transport half). The values are stored and travel; task 4 makes the shaders read them.

**Size:** M
**Files:** `crates/telperion-core/src/presets.rs`, `crates/telperion-core/src/params.rs` (split if it passes 400 lines), `crates/telperion-core/src/blend.rs`, `crates/telperion-core/src/material.rs` (new), `crates/telperion-render/src/scene.rs` or a new `crates/telperion-render/src/scene/row.rs`, `crates/telperion-render/src/view.rs`, `crates/telperion-render/src/web.rs`, `crates/telperion-render/examples/headless/walk.rs`, `harness/GrowerDev.tsx` (split under 400 lines), `harness/rust-stage.ts`, `src/browser/render.ts`, `src/browser/presets.generated.ts` (regenerated), tests
**Touches:** [crates/telperion-core/src/presets.rs, crates/telperion-core/src/params.rs, crates/telperion-core/src/params/**, crates/telperion-core/src/blend.rs, crates/telperion-core/src/material.rs, crates/telperion-core/src/lib.rs, crates/telperion-core/tests/**, crates/telperion-render/src/scene.rs, crates/telperion-render/src/scene/**, crates/telperion-render/src/view.rs, crates/telperion-render/src/web.rs, crates/telperion-render/examples/headless/**, crates/telperion-render/tests/conformance.rs, harness/**, src/browser/**]

### Approach
- Material row: a struct of numeric fields (bark colour as three linear floats, roughness; leaf front and back colours, hue range low and high, brightness range low and high, interior darkening amount) with `validate` naming each field and a range-order check; joins `Family` at `presets.rs:18-26` with defaults at 27-42 and per-preset values at 65-294; rows in the `fields!` table at `params.rs:12-104`; the blend walks every field in `blend.rs`. Starting values for the oak and spruce come from fn-9's profile prose (a Garry oak's dark green blade with a paler back; the spruce's darker needle) and are calibrated in task 6.
- Scene row: a struct of sun azimuth and elevation, sun colour, sky zenith and horizon colours, ground colour, with a default that reproduces an outdoor midday look, stored on the renderer and set through a setter beside `set_view` (`view.rs`, `web.rs:200-211`); the headless command takes `--scene <json>` matching the row's field names; the page session takes one setter.
- Panel: a scene-row `Traits` block beside the existing three (`GrowerDev.tsx:339-343`) backed by a stage method in `rust-stage.ts`; the material row appears through the family's generic mapping with no new code. `GrowerDev.tsx` is 430 lines: move the trait blocks into a sibling component file first.
- Tests: wire round-trip for the material row, refusal naming each field, blend of two valid ranges stays valid; conformance fixtures gain the row.

### Investigation targets
**Required** (read before coding):
- `crates/telperion-core/src/presets.rs:18-42` and `params.rs:12-104` — the family and the wire table
- `crates/telperion-render/src/view.rs` and `web.rs:200-211` — the view setter pattern the scene row follows
- `harness/GrowerDev.tsx:62-82,320-345` — the generic Traits component and where it is used

**Optional** (reference as needed):
- `crates/telperion-core/src/blend.rs` — the field walk
- `.flow/evidence/fn9/REFERENCES.md` — the colour prose for starting values

### Key context
- The scene row is never part of a preset; the blend of two families does not touch it.
- No npm script outside the workspace; regenerate the metadata with `npm run wasm:build`.

## Acceptance
- [ ] Every preset carries a material row; wire round-trips it, refuses out-of-range or inverted-range values naming the field; the blend walks it
- [ ] The renderer holds a scene row with a default; `--scene <json>` on the headless command and one page setter set it
- [ ] The panel shows the material row through the family mapping and the scene row through one Traits block; the panel file is under 400 lines after a split
- [ ] `cargo test --release --workspace`, `npm run wasm:build && npm test`, `npm run typecheck` pass

## Done summary
Every preset now carries a material row - bark colour and roughness, leaf front and back colour, the hue and brightness offsets one leaf may take, and the interior darkening amount - as one more numeric struct on the family, with rows in the wire table, validation by field name, and the blend walking all fifteen fields. The renderer holds a scene row beside the view - sun azimuth and elevation, sun colour, sky zenith and horizon, ground - with an outdoor midday default, set through `--scene <json>` on the headless command and `setScene` on the page session; the panel shows the material row through the family's generic mapping and the scene row through one Traits block, its dials having moved to `harness/dials.tsx` to take the file under the line rule. The values are stored and travel; task 4 makes the shaders read them.

baseline: green via handoff (green (verified at 18859d1 by fn-14-bark-and-foliage-appearance.1))

stage: impl-review - skipped(policy: parallel-wave - the conductor reviews after integration)

stage: wave-join - ran (fast-forward 38d2506..8e001cd, no collision; worker edited outside declared Touches: render Cargo.toml, device.rs, lib.rs, tests/walk.rs, examples/headless.rs - no sibling in flight, no collision)
stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 0d3a3f42761061f14ae0e0de90448e5b69366974, 8e001cdacae1ce654c0e9f8d2a13bcf83e7e10ab
- Tests: cargo test --release --workspace, npm run wasm:build && npm test, npm run typecheck, cargo clippy --release --workspace --all-targets -- -D warnings, cargo fmt --all -- --check, cargo run --release -p telperion-render --example headless -- --preset oregon-white-oak --seed 7 --size 256x256 --scene '{"sunElevation":20.0,"sunAzimuth":300.0}' --out /tmp/fn14-2-scene.png
- PRs: