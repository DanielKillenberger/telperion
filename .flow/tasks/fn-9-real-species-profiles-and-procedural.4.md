---
satisfies: [R4, R6]
---
# fn-9-real-species-profiles-and-procedural.4 Generate species-specific leaf and needle anatomy

## Description
Generate species-specific leaf and needle anatomy. See the parent spec for the botanical target and validation contract.

**Size:** M
**Files:** `crates/telperion-core/examples/species_metrics/mod.rs`, `crates/telperion-core/tests/species_metrics.rs`, `crates/telperion-core/src/foliage/element.rs`, `crates/telperion-core/src/foliage/placement.rs`, `crates/telperion-core/src/foliage.rs`, `crates/telperion-core/tests/foliage.rs`, `crates/telperion-core/tests/field.rs`
**Touches:** [crates/telperion-core/examples/species_metrics/mod.rs, crates/telperion-core/tests/species_metrics.rs, crates/telperion-core/src/foliage/element.rs, crates/telperion-core/src/foliage/placement.rs, crates/telperion-core/src/foliage.rs, crates/telperion-core/tests/foliage.rs, crates/telperion-core/tests/field.rs]

### Approach
- Integrate task 2 metrics with the actual blade/needle geometry subset, excluding petiole/peg connectors. Update analytic tests so new anatomy has exact measurable dimensions; do not promote whole-prototype estimates without evidence. Task 2 is scheduled first because it owns Cargo files.
- Extend element geometry and placement for the two researched anatomies. Needle count/grouping/orientation must follow the selected species; a narrow broadleaf card alone is insufficient.
- Prefer one reusable procedural element per selected family where it faithfully expresses the anatomy, including a compound element if appropriate. Add multiple element buckets only when the profile demonstrates the need.
- Keep biological unit metadata explicit for measurement. Coordinate any output-shape change with task 2 at integration; task 7 owns Wasm and adapter propagation.
- Validate transformed bounds, attachment, culling and conservative field coverage against actual geometry. Do not require wood surface generation to create or query foliage.
- Keep tests small and geometric: known attachment sites, needle/blade dimensions, orientation/grouping, empty results and invalid parameters.

### Investigation targets
**Required:**
- `crates/telperion-core/src/foliage/element.rs:4-134` — procedural prototype
- `crates/telperion-core/src/foliage/placement.rs:10-88` — twig placement
- `crates/telperion-core/src/foliage.rs` — composition and bounds
- `crates/telperion-core/src/field.rs` — consumes foliage geometry
- `crates/telperion-core/tests/foliage.rs:97-276` — validation patterns

### Quick commands
```bash
cargo test --release -p telperion-core --test foliage --test field
```

## Acceptance
- [ ] Both selected foliage anatomies have procedural geometry and attachment tests tied to profile traits.
- [ ] Counts distinguish leaves/needles from grouped render instances and remain usable by task 2.
- [ ] Bounds include transformed instances; field coverage and empty/invalid cases pass without requiring a surface mesh.
- [ ] Focused foliage/field regressions pass; any remaining unsupported anatomy is recorded.

## Done summary
Blocked:
Owner requested implementation stop and clean transfer to Forge. Worker has stopped; existing implementation is preserved in the handoff WIP. Temporarily blocking solely to release the claim, then resetting to todo for the successor to verify and complete. No external dependency or human decision blocks resumption.
## Evidence
- Commits:
- Tests:
- PRs:
