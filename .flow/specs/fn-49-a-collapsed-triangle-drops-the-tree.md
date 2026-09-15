## Conversation Evidence

> worker, fn-45 round 6 (2026-09-15): "at apical dominance 0.59, seeds 2181184680 and 2779011501 fail with `surface triangle collapsed in float32` (`surface.rs:351`) — a hard `InvalidInput`, so no tree at all."
> worker, fn-45 round 6c (2026-09-15): "The float32 surface collapse is not caused by apical dominance. One candidate table failed protocol seed 55 at 0.58 and three random seeds at 0.9. The failures follow geometry, about one seed in a hundred."

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 20% [user], 50% [paraphrase], 30% [inferred] -->

About one tree in a hundred fails to build at all: the surface builder finds one triangle whose three corners, rounded to float32, span no area, and returns an error for the whole tree. A game that asks for a beech at a seed gets nothing. Every value pass on a species rolls those dice across the protocol's 24 seeds, so tuning has been steering away from values for a reason that has nothing to do with the tree. [paraphrase]

This spec makes a degenerate triangle a local event: it is dropped from the mesh, its vertices take their normals from the triangles that remain, and the tree builds. Every tree that builds today is byte-identical. [inferred]

## Architecture & Data Models
<!-- scope: technical -->

- **Where it fails.** `surface::finish` (`crates/telperion-core/src/surface.rs:277-310`) accumulates each triangle's float64 cross product of its float32 corners into the vertex normals and returns `InvalidInput("surface triangle collapsed in float32")` when one is zero or not finite, then `"surface normal overflow or cancellation"` when a vertex's summed normal is zero. [inferred]
- **The fix.** A triangle whose float32 corners span no area is removed from the index buffer before normals are accumulated, and the run's index count is corrected; a vertex that then has no triangle, or whose normal still cancels, takes the normal of its ring (the direction from the run's axis to the vertex, which the sweep already knows) rather than failing. A non-finite position is still an error: that is a real fault upstream. [inferred]
- **Find the cause, not only the symptom.** Before changing `finish`, reproduce on one failing seed (the worker recorded protocol seed 55 on a candidate beech table and seeds 2181184680, 2779011501 at apical dominance 0.59) and record what geometry produces the zero-area triangle: a run whose radius falls below float32 resolution at its distance from the origin, two consecutive rings at the same position, a ring collapsed at a fork, or a tip cap. If the cause is a rule upstream that makes a ring zero-length, fix that rule too, and say which. [inferred]

## Edge Cases & Constraints
<!-- scope: technical -->

- **Byte identity.** Every shipped preset at every seed that builds today produces the same mesh bytes; the identity pins hold untouched. [paraphrase]
- **The run table stays honest.** Each run's first index and index count still address exactly its own triangles after a removal, so the renderer's per-run draws and the level-of-detail paths are unaffected. [inferred]
- **Counted, not silent.** The mesh reports how many triangles it dropped, and the species measure records it; a tree that drops more than a small bound (the implementer measures and proposes it) is a failure named by the count. [inferred]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** The cause of the zero-area triangle is reproduced on a named seed and recorded, with any upstream rule that produces it fixed. Errors: none beyond the record. [inferred]
- **R2:** A zero-area triangle in float32 is dropped with its run's index range corrected, vertices without a surviving triangle take their ring normal, and the tree builds; a non-finite position is still an error. Errors: a dropped count above the bound fails naming the count. [inferred]
- **R3:** Every shipped preset is byte-identical at every seed that built before, asserted by the identity pins and a sweep of the 24 protocol seeds per species; the seeds that failed before now build, and the 48-case protocol passes. Errors: a moved pin is a failure. [paraphrase]
- **R4:** The tests cover: a synthetic run with a ring collapsed to one point, a run whose radius falls below float32 resolution far from the origin, the index table after a removal, the ring-normal fallback, the non-finite error and the dropped count. Errors: a missing case is a review finding, not implementer discretion. [inferred]

## Boundaries
<!-- scope: business -->

- No change to the sweep's ring count, the radius model or any species' table. [inferred]

## Resolved via Codebase

- `crates/telperion-core/src/surface.rs:277-310` (`finish`: float64 cross products of float32 corners, the two `InvalidInput` returns); the file is 335 lines.
- Run seeding and ring sweep: `crates/telperion-core/src/surface/samples.rs`, `surface/paths.rs`.
- The failing seeds: `.flow/evidence/fn34/REPORT.md` round-6 section (fn-45), the float32 note.
