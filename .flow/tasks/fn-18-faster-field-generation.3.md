---
satisfies: [R1, R2, R3, R4]
---
# fn-18-faster-field-generation.3 Eliminate temporary foliage collections in complete field builds

## Description
Implement retained-placement streaming after integrating the selected growth result.

**Size:** M
**Files:** crates/telperion-core/src/foliage.rs, crates/telperion-core/src/foliage/placement.rs, crates/telperion-core/src/field.rs, crates/telperion-wasm/src/lib.rs, crates/telperion-core/tests/field.rs.
**Touches:** [crates/telperion-core/src/foliage.rs, crates/telperion-core/src/foliage/placement.rs, crates/telperion-core/src/field.rs, crates/telperion-wasm/src/lib.rs, crates/telperion-core/tests/field.rs, .flow/evidence/fn18/**]

## Approach
- Reuse the retained-placement predicate and field bounds conversion. Emit accepted field items without full placed/retained matrix collections, while keeping the ordered items needed by indexing.
- Preserve the sequential placement RNG stream: deriving new seeds from worker or ordinal changes the tree and is forbidden. Preserve surface-sensitive radial-needle attachments, f32 matrix packing, connector-inclusive local bounds, original eight-corner f64 transforms, cull validation/short-circuit and closed overlap.
- Count all placed instances against the original budget before culling. Preserve fallible checked allocations, diagnostics and Wasm failure invalidation. Select streaming only for field without foliage output; combined outputs use the same authoritative helpers.
- Replay the task 1 oracle and field-only/combined comparison against original and post-task-2 baseline, then measure complete latency, allocation lifetimes and query costs. Keep only qualified improvements; preserve rejected evidence.

## Investigation targets
**Required:**
- crates/telperion-core/src/foliage/placement.rs:81 — surface placement.
- crates/telperion-core/src/foliage/placement.rs:240 — placed budget.
- crates/telperion-core/src/foliage/placement.rs:338 — f32 packing.
- crates/telperion-core/src/foliage.rs:89 — cull.
- crates/telperion-core/src/field.rs:99 — bounds conversion.
- crates/telperion-wasm/src/lib.rs:210 — output selection.
- crates/telperion-core/tests/foliage_reference.rs — exact matrices/membership.

## Acceptance
- [ ] Full anchors/holdouts retain exact source, matrix-derived bounds, multiplicity, flags and diagnostics.
- [ ] Field-only avoids the full temporary matrix collections while combined outputs remain exact.
- [ ] Culled placements still consume budget; empty/zero/overflow/resource cases preserve complete success or failure.
- [ ] Paired original-versus-current and predecessor-versus-current evidence supports adoption under timing/query/memory guardrails.

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:

