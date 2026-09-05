---
satisfies: [R3, R7]
---
# fn-8-lean-rust-tree-generation-core.8 Expose mesh-free wood and foliage fields

## Description
Implement R3's mesh-free field output as a native consumer of solved tree and foliage data. This is a separate task because it adds a representation rather than porting an existing surface algorithm.

**Size:** M
**Files:** Rust field module and field tests
**Touches:** [crates/telperion-core/src/field/**, crates/telperion-core/src/field.rs, crates/telperion-core/tests/field*]

### Approach
- Read the foundation's domain contract and expose queries against existing solved structure and optional foliage data, without invoking surface construction.
- Provide separately distinguishable wood and foliage occupancy at world-space samples, usable to fill a caller-selected block grid. Choose the smallest explicit geometric semantics that support that consumer, document approximation and resolution limits, and keep them deterministic.
- Model wood from branch positions and solved radii; derive foliage occupancy from the placed/culling-retained foliage data using documented leaf extent. Do not disguise a mesh voxelizer as mesh-free generation.
- Bound queries to relevant tree bounds and use a simple reusable spatial lookup if needed for giant-tree queries. Avoid a general field framework or full game adapter.
- Handle empty trees/foliage, samples outside bounds, non-finite coordinates and resource limits explicitly. Boundary occupancy and increasing sample resolution must behave consistently.
- Demonstrate a small native block-grid consumer over an ordinary tree and representative giant-tree samples, with a test proving surface construction is not called. Task 6 exposes this operation through the browser binding; coordinate contracts in shared notes.

### Investigation targets
**Required:**
- `src/skeleton/colonize.ts` - current structural metadata
- `src/radius.ts` - radius meaning and endpoint taper
- `src/canopy/place.ts` - leaf transforms and placement
- `src/canopy/cull.ts` - retained foliage
- `src/envelope.ts` - existing spatial conventions

### Key context
The approved capture explicitly includes field output. Prior planning's deferral was rejected by the rewrite; a renderer-independent tree alone does not satisfy R3.

## Acceptance
- [ ] Native field queries distinguish wood and foliage, with documented units, approximation and boundary semantics.
- [ ] An ordinary-tree block-grid example and giant-tree sample tests work without generating a surface mesh.
- [ ] Empty/outside/non-finite cases and deterministic repeated queries pass focused tests.
- [ ] Query cost and storage are recorded for representative inputs; no full per-sample scan across all giant-tree leaves where a bounded lookup is needed.


## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
