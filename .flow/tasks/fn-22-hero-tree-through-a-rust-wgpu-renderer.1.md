---
satisfies: [R3]
---
# fn-22-hero-tree-through-a-rust-wgpu-renderer.1 Core mesh contract and parameter JSON in the core

## Description
Give the core the one engine-neutral mesh call (spec §API Contracts, Mesh output) and move the JSON parameter mirror into the core behind a `json` feature so the renderer wasm module can parse the panel's JSON without linking the C-ABI binding. Split first because every later task consumes `mesh::build` and `params::parse`.

**Size:** M
**Files:** `crates/telperion-core/src/mesh.rs` (new), `crates/telperion-core/src/params.rs` (moved from the wasm crate), `crates/telperion-core/src/lib.rs`, `crates/telperion-core/Cargo.toml`, `crates/telperion-wasm/src/lib.rs`, `crates/telperion-wasm/src/params.rs` (removed), `crates/telperion-wasm/Cargo.toml`, `crates/telperion-core/tests/mesh.rs` (new)
**Touches:** [crates/telperion-core/**, crates/telperion-wasm/**]

### Approach
- `mesh::build(&Family, Detail) -> Result<TreeMesh>` assembles the chain the binding runs at `crates/telperion-wasm/src/lib.rs:187-232`: `branching::generate` → `surface::build` with the envelope height → `foliage::build_element` + `place_on_surface` (twig placement from `skeleton.twigs.resolved()`) → `foliage::cull` with `shell_depth`. `TreeMesh { wood: SurfaceMesh, foliage: Foliage { element: Element, instances: Instances }, bounds: Bounds }` plus derived count accessors (wood vertices, wood triangles, foliage instances). `Detail` is a unit-like enum with the single variant `Full`.
- Bounds: union of `wood.bounds` and `instances.bounds(&element)`; reuse the two existing `Bounds` types rather than adding a third (pick `surface::Bounds` or unify with a `From`).
- Move `crates/telperion-wasm/src/params.rs` verbatim into `crates/telperion-core/src/params.rs`, gated by `#[cfg(feature = "json")]` with `serde_json` as an optional dependency; the wasm crate enables the feature and does `pub use telperion_core::params;` (or `use`) so `CATALOGUE`, `preset`, `by_identity`, `metadata`, `parse` keep their paths inside the binding.
- Leave the binding's own generate chain in place (it also produces structure, field and diagnostics). The equivalence pin is a test, not a refactor.
- Errors stay `telperion_core::Error` (`crates/telperion-core/src/lib.rs:17-30`).

### Investigation targets
**Required** (read before coding):
- `crates/telperion-wasm/src/lib.rs:187-260` — the generate chain and the metadata counts to pin against
- `crates/telperion-core/src/surface.rs:59-70` — `Bounds`, `SurfaceMesh`
- `crates/telperion-core/src/foliage.rs:15-70` — `Bounds`, `Instances`, `bounds(element)`
- `crates/telperion-wasm/src/params.rs:1-30,86-130` — the wire schema macro and public entry points being moved
- `crates/telperion-core/src/presets.rs:10-70` — `Preset`, `Family`

**Optional** (reference as needed):
- `crates/telperion-core/tests/species.rs:150-200` — how tests run the chain per preset today
- `scripts/build-wasm.mjs` — regenerates `presets.generated.ts` from the catalogue (the byte-identical pin)

### Key context
- The core has no serde dependency today; keep `json` off by default so consumers that never enable it see no new dependency.
- Full-detail spruce places about 5.2 million instances; the test that compares against the binding should run in release or on oak and spruce with the tests' existing seeds, not on many seeds.

## Acceptance
- [ ] `mesh::build(&preset.parameters(), Detail::Full)` succeeds for all five presets and a test asserts, for oak and spruce, that wood vertex count, wood triangle count, foliage instance count and bounds equal what the wasm binding's `generate` reports for the same family (`crates/telperion-core/tests/mesh.rs`)
- [ ] `telperion_core::params` compiles only with `--features json`; `cargo build -p telperion-core` (default features) has no serde dependency
- [ ] `npm run wasm:build` leaves `src/browser/presets.generated.ts` byte-identical (`git diff --exit-code` on it) and the wasm crate exposes the same C-ABI exports
- [ ] `cargo test --release --workspace`, `npm run rust:test:wasm` and `npm test` pass; `cargo clippy --workspace --all-targets` is clean
- [ ] Every touched file stays under 400 lines

## Done summary
The core gained `mesh::build(&Family, Detail::Full) -> Result<TreeMesh>`, the one engine-neutral mesh call: it runs the same chain the C-ABI binding runs (growth, the plaited wood surface, the culled foliage) and returns the wood surface, one foliage element with its instance matrices, union bounds and the derived wood-vertex, wood-triangle and foliage-instance counts. The JSON parameter mirror moved from the binding into the core behind a `json` feature, so the renderer wasm module will parse the panel's schema without linking the C-ABI, while the default core stays serde-free.

Scope notes for the reviewer:
- `foliage::Bounds` converts into `surface::Bounds` (a `From` impl in `mesh.rs`); no third bounds type was added. `TreeMesh::bounds` is non-optional and a mesh with no geometry at all is `Error::InvalidInput("mesh has no geometry")`.
- The binding's own `generate` chain is unchanged in behaviour but now lives in `crates/telperion-wasm/src/generate.rs`. The split is what brings the touched `lib.rs` from 524 to 280 lines, which the task's "every touched file stays under 400 lines" criterion required.
- The binding-equivalence pin lives in that new module's unit tests rather than in `crates/telperion-core/tests/mesh.rs` as the acceptance text suggested: `generate` is private to the binding, and reaching it from a core test would need a dev-dependency cycle back onto the crate the core is supposed to know nothing about. `crates/telperion-core/tests/mesh.rs` carries the five-preset contract and the generator-rejection case; `crates/telperion-wasm/src/generate.rs` carries the oak and spruce count-and-bounds equality against the binding (oak 2,795,666 wood vertices / 5,416,480 triangles / 567,801 instances; spruce 3,589,724 / 6,938,160 / 7,923,516).
- `serde` left the wasm crate's own manifest; it now arrives through the core's `json` feature. `Cargo.lock` changed accordingly. `src/browser/presets.generated.ts` is byte-identical.

stage: impl-review - skipped(config: REVIEW_MODE=none; parallel wave - conductor reviews after integration)

stage: plan-sync - skipped(config: planSync.enabled != true)
stage: wave-join - ran (commit 6243121 already on target; no integration needed)
## Evidence
- Commits: 6243121df520d0a9544c5c9b6dd03f77db37b0d7
- Tests: cargo test --release --workspace (76 passed, 0 failed; +3 new: mesh contract, generator-rejection, binding equivalence), cargo clippy --workspace --all-targets (clean; 2 inherited geometry_benchmark warnings, unchanged from baseline), cargo fmt --all -- --check (clean), cargo build -p telperion-core (default features; cargo tree --edges normal shows no serde), cargo test --release -p telperion-core --features json --lib (params::tests::catalogue_roundtrips_all_controls_and_identities ok), npm run wasm:build && git diff --exit-code -- src/browser/presets.generated.ts (byte-identical), npm test (3 files, 106 passed), npm run rust:test:wasm (browser integration, all 16 checks true), red-first: perturbing mesh::build's placement seed failed generate::tests::mesh_build_matches_the_binding_chain_for_oak_and_spruce on bounds, then reverted
- PRs: