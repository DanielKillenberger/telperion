---
satisfies: [R1, R3]
---
# fn-8-lean-rust-tree-generation-core.4 Port the final swept surface to independent Rust output

## Description
Port surface generation against frozen solved-tree fixtures, independently of the live growth port.

**Size:** M
**Files:** Rust surface/path/frame/normal modules and geometry tests
**Touches:** [crates/telperion-core/src/surface/**, crates/telperion-core/tests/surface*, tests/migration/surface*]

### Approach
- Adapt FN7's flat preallocated output patterns to the final FN6 surface contract; account for final endpoint-radius and all current surface details.
- Keep surface generation independent of foliage and Three objects, exposing positions, indices, normals, bounds and needed diagnostics.
- Use checked size arithmetic and explicit allocation failure; verify every generated index and vertex.
- Reuse the task-1 fixture runner for exact topology/winding and tolerance-bound attributes, including degenerate edges, forks, taper and empty surfaces.

### Investigation targets
**Required:**
- `src/mesh/surface.ts:195`
- `src/mesh/paths.ts`
- `src/mesh/frames.ts`
- `src/mesh/surface.test.ts`
- `experiments/rust-surface-benchmark/rust/lib.rs:69`
- `experiments/rust-surface-benchmark/REPORT.md`

## Acceptance
- [ ] Final-FN6 surface fixtures pass geometry equivalence, including normals and bounds.
- [ ] Empty, degenerate and invalid-input outcomes match the new contract without partial successful buffers.
- [ ] Surface creation performs no foliage work and imports no renderer/binding types.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
