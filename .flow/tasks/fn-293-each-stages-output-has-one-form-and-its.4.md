---
satisfies: [R1, R3]
---
# fn-293-each-stages-output-has-one-form-and-its.4 One sweep a mesh request, one station count, one Expansion preparation

## Description
Findings 7, 12 and 13 in the core pipeline: a mesh request sweeps the wood once and draws its `SurfaceMesh` from the curve; the leaf plan calls `station_count` instead of restating it; the two `Expansion` preparations share one helper and one `retains` rule.

**Size:** M
**Files:** `crates/telperion-core/src/pipeline/drawn.rs`, `pipeline/surface/curve.rs`, `pipeline/foliage/plan.rs`, `pipeline/foliage/station.rs`, `pipeline/executor.rs`, `pipeline/tests.rs`
**Touches:** [crates/telperion-core/src/pipeline/drawn.rs, crates/telperion-core/src/pipeline/surface/**, crates/telperion-core/src/pipeline/foliage/plan.rs, crates/telperion-core/src/pipeline/foliage/plan/**, crates/telperion-core/src/pipeline/foliage/station.rs, crates/telperion-core/src/pipeline/foliage/placement.rs, crates/telperion-core/src/pipeline/executor.rs, crates/telperion-core/src/pipeline/tests.rs, .flow/evidence/fn-293-each-stages-output-has-one-form-and-its/**]

### Approach
- Finding 7: `drawn.rs:22-33` sweeps rings, `drawn.rs:83-86` traces the curve again (`surface/curve.rs:116` `traced`). When a request asks for both, trace once and draw the rings (and the `SurfaceMesh`, seated leaves) from `Curve::rings` (`curve.rs:348`), which `curve/tests.rs` holds equal to the sweep's rings to the bit. A request without the curve keeps its single sweep. Update `one_sweep_and_one_element_serve_a_request` (`pipeline/tests.rs:57`) to count one sweep for `Request::mesh()`.
- Finding 12: `plan.rs:141-154` and `:210-223` restate `ceil(length/internode)·per` and the budget; call `station::station_count` (`station.rs:178`, make it visible to the plan module if needed). `placement.rs:133-134` restates `per`: use the same helper.
- Finding 13: `executor.rs:137-167` (`resident`) and `:195-216` (`Grown::expansion`) each build element, twig, shell check, reference and bearing. One private helper builds them; `retains` has one rule. Check first whether the two rules ever differ on a shipped preset (species `leaf_years` versus `!tree.leafing.is_empty()`); if they do, keep the digests identical by choosing the rule the resident build uses on the device, and report the difference.
- Measure finding 7's gain: `Request::mesh()` time on the spruce and oak, paired warm over 16 runs, before and after (CPU, `--profile ci`), in `MEASURE.md`.

### Investigation targets
**Required:**
- `crates/telperion-core/src/pipeline/drawn.rs:1-120`
- `crates/telperion-core/src/pipeline/surface/curve.rs:80-160,340-381`
- `crates/telperion-core/src/pipeline/foliage/plan.rs:130-230`
- `crates/telperion-core/src/pipeline/executor.rs:130-220`
**Optional:**
- `crates/telperion-core/src/pipeline/surface/curve/tests.rs`

### Key context
The CPU mesh is the stills' and Jev's reference: its bytes must not move. If `Curve::rings` differs from the sweep's rings for any preset at seeds 1, 7, 42, stop and report.
## Acceptance
- [ ] `Request::mesh()` sweeps the wood once (the counting test says so).
- [ ] The plan and placement call one station count; one `Expansion` preparation helper with one `retains` rule.
- [ ] Digests equal the baseline; the mesh time before and after is in `MEASURE.md`.
- [ ] Gate and `npm test` green.
## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
