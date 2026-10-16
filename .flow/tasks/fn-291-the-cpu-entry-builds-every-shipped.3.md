---
satisfies: [R2]
---
# fn-291-the-cpu-entry-builds-every-shipped.3 R2b: compact per-vertex wood (oct normals, 16-bit bark coords) with bounded precision

## Description
R2b. Store the wood's per-vertex data in the smallest layout the drawing allows: normals oct-encoded, bark coords in 16 bits. Each field's precision is bounded by a test at the drawn tolerance. The CPU entry hands JS positions, normals and indices (`src/browser/core.ts:91,161`). A changed normals type is a package format change, so the TypeScript types, README and a migration note change with it. Coords never reach JS.

**Size:** M
**Files:** `crates/telperion-core/src/pipeline/surface.rs`, `crates/telperion-core/src/pipeline/surface/{build,rings,normals}.rs`, `crates/telperion-core/src/pipeline/surface/curve/pack.rs`, `crates/telperion-wasm/src/lib.rs`, `src/browser/core.ts`, `README.md`, `docs/migration-*.md` (or the repo's existing migration notes), tests under `crates/telperion-render/tests/bark_*.rs`, `crates/telperion-core/src/suite/surface_*.rs`
**Touches:** [crates/telperion-core/src/**, crates/telperion-wasm/**, crates/telperion-render/**, src/browser/**, tests/browser/**, README.md, docs/**]

### Approach
- Reuse `octahedral`/`unoctahedral` in `surface/curve/pack.rs:161-186` (move them where both can read them; no copy). Bark coords quantise over a reference range, as `foliage::Reference` does for leaves.
- Write each compact field once at final size; no f32 staging buffer the size of the wood.
- Bound each field with its own test: the angular error of a decoded normal, and the coordinate error against the bark's texel at the drawn tolerance. The existing unit-normal 1e-6 asserts (`suite/surface_collapse.rs`, `suite/surface_reference.rs`) are restated against the decoded normal at the new bound.
- Change serial and parallel paths together. Update byte-pinning tests (`bark_detail.rs`, `bark_depth.rs`, `parallel/tests.rs`) with evidence that the change is the encoding alone: decoded values within the bound.
- Re-run task 1's probe on Telperion and Laurelin; record the high-water mark in PROBE.md.

### Investigation targets
**Required**:
- `crates/telperion-core/src/pipeline/surface/curve/pack.rs:140-190` — oct encoding to reuse
- `crates/telperion-core/src/foliage/packed.rs` — quantised words over a reference box
- `crates/telperion-wasm/src/lib.rs:147-258` — the buffer slots
- `src/browser/core.ts:85-165` — `TreeOutput.surface`
- `README.md:10-15` — the documented surface format
**Optional**:
- `crates/telperion-render/src/generation/wood.rs:71` — `cpu_bytes`

### Key context
The native stills draw from the curve, not from this mesh; the eye check of this change is task 4's still of the CPU entry's own mesh. Fewer sides on thin wood is not allowed.
## Acceptance
- [ ] Normals and bark coords are stored compactly at final size; no full-size f32 staging copy
- [ ] A test bounds each field's precision (decoded normal angle; coord error) and fails when the bound is broken
- [ ] If the JS normals type changed: `TreeOutput` types, README and a migration note state the new layout and how to decode it
- [ ] Serial and parallel paths agree bit for bit; byte-pinning tests updated with the decoded-within-bound evidence
- [ ] PROBE.md records Telperion and Laurelin's high-water mark after R2b
## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
