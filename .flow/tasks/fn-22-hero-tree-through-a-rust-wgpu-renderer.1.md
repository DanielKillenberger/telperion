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
TBD

## Evidence
- Commits:
- Tests:
- PRs:
