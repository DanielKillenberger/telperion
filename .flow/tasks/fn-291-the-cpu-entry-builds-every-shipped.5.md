---
satisfies: [R2, R4]
---
# fn-291-the-cpu-entry-builds-every-shipped.5 R4 core (R2d): the wood written as chunks of at most 65,536 vertices with u16 indices, values unchanged

## Description
R4 in the core (the host's R2d), and the plan's proof point. The CPU wood surface is written as chunks of at most 65,536 vertices. Each chunk has its own positions, normals and coords, plus `u16` indices local to the chunk. Each chunk is allocated as it is written, so no wood buffer needs one large free run, and the index bytes halve. The decoded values stay equal: every vertex's position, normal and coord, and every triangle in draw order. The GPU path (`Curve`, `compact.rs`, `PreparedSurface`) and the harness are unchanged.

**Size:** L (the type change reaches about 35 files of tests and examples, mechanically)
**Files:** `crates/telperion-core/src/pipeline/surface.rs`, `surface/rings.rs`, `surface/build.rs`, `surface/parallel.rs`, `surface/parallel/*.rs`, `surface/normals.rs`, `surface/prepared.rs`, `crates/telperion-core/src/mesh.rs`, `crates/telperion-core/src/suite/*.rs`, `crates/telperion-core/examples/*.rs`, `crates/telperion-render/src/generation/wood.rs`, `crates/telperion-render/tests/bark_*.rs`, `crates/telperion-render/src/generation/tests.rs`, `crates/telperion-wasm/src/{generate,lib}.rs` (compile only), `.flow/evidence/fn-291-the-cpu-entry-builds-every-shipped/PROBE.md`
**Touches:** [crates/telperion-core/**, crates/telperion-render/src/generation/wood.rs, crates/telperion-render/src/generation/tests.rs, crates/telperion-render/tests/**, crates/telperion-render/examples/**, crates/telperion-wasm/src/**, .flow/evidence/fn-291-the-cpu-entry-builds-every-shipped/**]

### Approach
- Layout:
  - Runs (one per path, `rings * segments + 2` vertices) fill a chunk in the existing run order until the next run does not fit.
  - A run larger than 65,536 vertices splits at a ring boundary, with that ring duplicated in both chunks. Each split keeps its caps in the chunk that holds the cap vertices.
  - Normals are always accumulated over the whole run first, as `shade` already does per run (`build.rs:165-185`), then copied. A split therefore changes no value.
  - Measure the longest shipped run first. The scout found it unmeasured.
- `SurfaceRun` becomes a chunk and span pair.
- Dropped triangles keep today's rule: the count and the `DROPPED_RINGS` failure. The spans are per chunk.
- Parallel path (`parallel.rs:72-92`): cut groups at chunk boundaries, with one `Unfilled` per chunk. Update `admitted`'s byte arithmetic to the new layout. Serial and parallel stay equal bit for bit (`parallel/tests.rs:92`).
- `extent_of`/`WoodExtent` predict per-chunk sizes. `suite/species.rs` checks the prediction against actual lengths.
- Equivalence harness:
  - Before changing code, record each preset's flat mesh at seed 1 natively. Use a test that expands the chunks back to flat positions, normals, coords and absolute indices.
  - Compare with `to_bits` against the flat mesh from the base commit, with the digest recorded in the test or computed in the same run from a base-commit binary.
  - Byte-pinning tests (`bark_*`, `suite/species.rs` digests) re-pin only after that equality is shown.
- Proof point:
  - Run the traced wasm (`trace-pages.py`, adapted to the chunk reservation) on telperion seed 7 and on laurelin seeds 7 and 42. Record the high-water mark in PROBE.md.
  - Estimate R2b's saving from the measured normals and coords bytes. If that projection is still over 3,584 MiB, report it to the host before task 6.

### Investigation targets
**Required**:
- `crates/telperion-core/src/pipeline/surface/build.rs:14-160,165-270` — extent, faces, shade, emit_run
- `crates/telperion-core/src/pipeline/surface/rings.rs:40-143,200-330` — rings, table, into_mesh
- `crates/telperion-core/src/pipeline/surface/parallel.rs:36-230`
- `crates/telperion-core/src/pipeline/surface/prepared.rs:10-75` — `visit_triangles`
- `crates/telperion-core/src/pipeline/surface.rs:70-120,245-265` — `WoodExtent`, `SurfaceMesh`
**Optional**:
- `crates/telperion-core/src/suite/species.rs:377-479` — extent against actual lengths
- `crates/telperion-render/src/generation/wood.rs:71-85`

### Key context
AttachmentSurface reads `Rings`, not the mesh. Leaves seat on rings, so rings may stay flat only if their own reservation is not the failing one. The failing reservations are `rings.rs:140` coords and `build.rs:112` normals, so rings' positions and coords are chunked too.

## Acceptance
- [ ] Every chunk holds at most 65,536 vertices with `u16` indices, and each chunk's buffers are reserved separately at exact size
- [ ] A test expands the chunks to flat buffers and matches the base commit's flat mesh with `to_bits` for every preset (positions, normals, coords, absolute indices, draw order)
- [ ] A run longer than 65,536 vertices splits with no value changed (a test with such a run)
- [ ] Serial and parallel paths agree bit for bit; byte-pinning tests re-pinned only after the equality above
- [ ] PROBE.md records the traced Wasm high-water mark for telperion seed 7 and laurelin seeds 7 and 42 with chunks, and R2b's projected effect


## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
