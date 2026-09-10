---
satisfies: [R5]
---
# fn-14-bark-and-foliage-appearance.1 Surface coordinates from the core, as one more buffer

## Description
Emit the two coordinate floats per wood and leaf vertex the spec's R5 names, as a separate buffer beside position and normal, through the mesh contract, the wasm slots and the renderer's vertex layouts. Nothing about positions, normals, indices, levels or placements changes; this task makes the coordinates exist and flow, and the shaders ignore them until task 4.

**Size:** M
**Files:** `crates/telperion-core/src/surface.rs`, `crates/telperion-core/src/surface/attachment.rs`, `crates/telperion-core/src/foliage/element.rs`, `crates/telperion-core/src/foliage/levels.rs` (carry only), `crates/telperion-core/src/mesh.rs`, `crates/telperion-wasm/src/lib.rs`, `crates/telperion-render/src/wood.rs`, `crates/telperion-render/src/foliage.rs`, `crates/telperion-render/src/submit.rs`, `crates/telperion-render/src/buffer.rs`, `crates/telperion-render/tests/submit.rs`, core tests
**Touches:** [crates/telperion-core/src/surface.rs, crates/telperion-core/src/surface/**, crates/telperion-core/src/foliage/element.rs, crates/telperion-core/src/foliage/levels.rs, crates/telperion-core/src/mesh.rs, crates/telperion-core/tests/**, crates/telperion-wasm/src/lib.rs, crates/telperion-render/src/wood.rs, crates/telperion-render/src/foliage.rs, crates/telperion-render/src/submit.rs, crates/telperion-render/src/buffer.rs, crates/telperion-render/tests/submit.rs]

### Approach
- Wood: the vertex loop at `crates/telperion-core/src/surface.rs:230-248` already computes the arc length along the axis and the angle around it and drops them; write both into a new `coords` array on the surface mesh (`surface.rs:63-70`). The mirrored `AttachmentSurface::new` at `surface/attachment.rs:54-72` must keep producing identical geometry; it needs no coordinates.
- Leaf: `build_element` at `foliage/element.rs:126-246` has `t` per row and `u` per column; write them per vertex into a `coords` array on the element. The base and tip vertices take (0, 0) and (1, 0). The levels module reads sections and indices only and needs no change beyond carrying the array through.
- Mesh: `mesh.rs:72-103` passes the arrays through into `TreeMesh`; add the wasm C-ABI slots for wood and element coordinates after the existing ones at `crates/telperion-wasm/src/lib.rs:133-200` without renumbering any.
- Renderer: upload the two buffers in `submit.rs`/`buffer.rs` and bind them as a third vertex buffer at location 2 in `wood.rs:13-19` and `foliage.rs:18-19`; the shaders declare and ignore the attribute for now.
- Tests: `crates/telperion-render/tests/submit.rs` pins buffer layouts; extend, never loosen. Core tests: a coordinate test asserting arc length is monotonic along each axis and angle spans a full turn per ring, and that leaf coords lie in the unit square; the identity pins must not move.

### Investigation targets
**Required** (read before coding):
- `crates/telperion-core/src/surface.rs:200-260` — the vertex loop and the mesh struct
- `crates/telperion-core/src/foliage/element.rs:126-246` — the element build with t and u
- `crates/telperion-render/src/submit.rs` and `buffer.rs` — the upload path
- `crates/telperion-wasm/src/lib.rs:133-200` — the slot table

**Optional** (reference as needed):
- `crates/telperion-render/tests/submit.rs` — the layout pins
- `crates/telperion-core/tests/identity.rs` — the pins that must not move

### Key context
- fn-4 and fn-20 will edit the surface builder later; keep the coordinate write to the one loop so those merges stay simple.
- The wasm binding's Three.js consumer path is legacy; the numbered slots stay as they are.

## Acceptance
- [ ] Wood and leaf meshes carry a coordinate buffer of two floats per vertex; the surface and element builders write it in their existing loops
- [ ] Two new wasm slots expose the buffers; existing slots and strides unchanged
- [ ] The renderer uploads and binds the buffers as a third vertex buffer; the shaders compile with the attribute declared
- [ ] Identity pins for positions, indices, counts, skeleton and placement unchanged; a new test checks the coordinates' ranges
- [ ] `cargo test --release --workspace`, fmt and clippy pass

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
