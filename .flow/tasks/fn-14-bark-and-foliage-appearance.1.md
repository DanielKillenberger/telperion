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
# fn-14-bark-and-foliage-appearance.1 — surface coordinates from the core

Wood and leaf vertices now carry the two coordinates R5 asks for, in a buffer
of their own beside position and normal, through the core, the wasm slots and
the renderer's vertex layouts. Nothing about positions, normals, indices,
levels or placements moved; the shaders declare the attribute and ignore it
until task 4.

Two commits. `0fd2edb` (inherited, verified here rather than redone) writes the
coordinates and carries them to the device. `18859d1` (this continuation) pins
the coordinate buffer's length on every shipped preset.

### What each acceptance item rests on

- **Coordinates written in the existing loops.** The wood loop at
  `crates/telperion-core/src/surface.rs:252` writes `s.d`, metres along the
  branch from the root, and the ring angle in radians, in the same loop that
  already computed both and dropped them; the two cap vertices at 273 and 277
  take their sample's distance and a zero angle, the axis having none. The
  blade writes the fraction along it and the fraction across it from the
  midrib per row in `foliage/element.rs`; the base and tip take (0,0) and
  (1,0), the peg takes the base's pair so the join has no seam, and a card
  takes its own extent. There are exactly three vertex-push sites in the
  surface builder and each one writes a coordinate pair, so a short array is
  unreachable by construction; `Element::validate` rejects a coords array that
  is neither empty nor two floats a vertex.
- **Wasm slots.** 14 for the wood, 15 for the element, appended after 13.
  Slots 0–13 and their lengths are untouched, and the lengths are float counts
  like slots 0 and 1, so `src/browser/core.ts` reads them the way it reads the
  rest. A consumer that ignores the slots reads as before.
- **The renderer binds them at location 2.** Three vertex buffers on both
  pipelines, stride taken from a float count rather than the fixed 3 it was.
  Proven on the real adapter in this workspace, not read off the source: the
  conformance suite built both pipelines and rendered 20 generated parameter
  sets plus all five shipped families, so the WGSL compiled with the attribute
  declared and the layouts matched.
- **Pins unchanged, ranges tested.** `tests/identity.rs` passes untouched.
  Two new tests: the surface one walks three rings of a straight trunk at 0, 4
  and 8 m with each vertex's angle at k/8 of a turn, then a flared, twisted,
  lobed profile where distance never goes backwards; the blade one holds every
  coordinate in the unit square, the midrib at nought and both margins at one,
  and the peg on the base's pair.
- **Gates.** `cargo test --release --workspace` 150 passed, 0 failed, 7
  ignored; `cargo fmt --check` clean; `cargo clippy --workspace --all-targets`
  no warnings. Receipt `.flow/tmp/green-receipts/18859d1f-unittest.json`.

### What this continuation added, and why

The renderer pads a coordinate array whose length does not match its vertices
with zeros rather than let the device read past a buffer's end
(`buffer::attributes`). That is right for a mesh built by hand in a test, but
it means a short array from a real preset would go up as silent zeros with
every gate still green. The one test that says the reported counts describe
the buffers a renderer uploads, `crates/telperion-core/tests/mesh.rs`, now
counts the coordinates too, for the wood and for the blade, across all five
presets — two assertions inside a loop that already runs, no new test. Run red
first with a deliberately wrong factor: the ordinary preset's 242,770 wood
vertices gave 485,540 floats against a demanded 728,310.

### Deviations from the task's file list, with reasons

- `crates/telperion-core/src/mesh.rs` needed no edit: `TreeMesh` moves the
  whole `SurfaceMesh` and `Element`, so the array rides through.
- `crates/telperion-core/src/foliage/levels.rs` needed no edit: a level is a
  set of triangles over the same vertex array, never a re-ordering of it, so
  every level reads the same coordinates. That is the level-shimmer constraint
  holding structurally rather than by a test.
- `crates/telperion-core/src/surface/attachment.rs` needed no edit: it is an
  exact swept-polygon query structure, not a mesh, and carries no vertex
  attributes. Its geometry test passes unchanged.
- No test pins the wasm slot table. There is none for slots 0–13 either, and
  the acceptance does not ask for one; adding the first would be new scope.
  The binding-chain test still checks counts and bounds only.

### Follow-ups for later tasks

- Task 4 reads `@location(2)` in both shaders; the attribute is already there,
  named `coord`, unread.
- fn-21's per-leaf shape wants the placement index, not this buffer.

stage: impl-review - skipped(policy: parallel wave, REVIEW_MODE=none - the conductor reviews after integration)

stage: wave-join - ran (fast-forward 3078983..18859d1, no collision)
stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 0fd2edb33830319921133b4d4735923d936e9bff, 18859d1f523e6631ba0227e7f0edd474a6c5f6b5
- Tests: cargo test --release --workspace (150 passed, 0 failed, 7 ignored), cargo fmt --check, cargo clippy --workspace --all-targets (0 warnings), cargo test --release -p telperion-render --test conformance parameter_sets_nobody_wrote_by_hand -- --nocapture (20 sets rendered on a real adapter: the three-buffer pipelines built)
- PRs: