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
Implemented owned mesh-free wood and retained-foliage fields with separate occupancy flags and caller-resolution closed cell queries. Private median BVHs prune spatial queries without global giant-leaf scans; the field has no surface dependency and needs only a reusable leaf extent. API and approximation contract: /home/daniel/Projects/telperion/.git/flow-notes/fn8-rust-20260905/field.md.

baseline: none (task and parent define no Quick commands). The focused test first failed because Field was absent. Three final tests pass: empty/invalid queries; tapered wood, flat leaf boundary overlap, simultaneous materials, repeated queries, empty elements and malformed inputs; generated ordinary/giant native block consumers with source independence guards and retained-leaf samples. Formatting, strict core all-target clippy and diff checks pass. Classifier FULL; no nonexistent gate receipts/skips were fabricated.

R3 approximation: wood uses linearly tapered sphere sweeps with rounded ends, and cubic queries use circumsphere inflation, producing conservative corner false positives that reduce with resolution. Leaf occupancy is transformed local-AABB overlap, conservatively enclosing blades and preserving thin cards for finite cells. Its false positives do not vanish with resolution. Nonfinite/negative query values reject InvalidInput; coordinate overflow or allocation failure rejects ResourceLimit. Checked arithmetic/fallible allocations impose no arbitrary low leaf ceiling. Finite coordinates are bounded below the squared-distance overflow range. Both material flags are queried independently. Bounds describe these approximate primitives.

Release single-run field-only evidence (not whole-process RAM or full-generation timing): ordinary 13,264 nodes and 63,029 leaves builds in 15.19 ms; 32,768 cubic queries in 3.10 ms; owned capacity 8,478,152 bytes. Telperion 175,035 nodes and 1,349,630 retained leaves builds in 390.90 ms; same query count in 5.95 ms; owned capacity 163,668,696 bytes. Raw observations: /tmp/fn8-field-test.log. No giant wood mesh is allocated. Storage is O(n); median build O(n log n); worst-case heavily overlapping primitives can still require a scan.


Worker handover was followed by conductor integration and verified completion; the integration checks are recorded below.

Conductor merged the field and verified three release tests on the joined tree.
stage: impl-review - skipped(user: none)
stage: wave-join - ran(merge and focused integrated checks)
stage: plan-sync - skipped(config: false)
## Evidence
- Commits: d1c4419bb3d0a01a8470cd8cd3f2b7a7e72ba38b
- Tests: baseline: none (no Quick commands defined), RED: cargo test -p telperion-core --test field; missing Field API, /tmp/fn8-field-red.log, PASS: cargo test -p telperion-core --release --test field -- --nocapture; 3 tests, /tmp/fn8-field-test.log, PASS: cargo clippy -p telperion-core --all-targets -- -D warnings; /tmp/fn8-field-clippy.log, PASS: cargo fmt --all --check, PASS: git diff --check, gate classify: FULL; no spec-defined full gate commands, Conductor: cargo test --release -p telperion-core --test field: 3 passed on integrated tree
- PRs:
