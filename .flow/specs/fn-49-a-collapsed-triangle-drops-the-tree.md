## Goal

About one tree in a hundred failed to build: the surface builder found a triangle whose float32 corners spanned no area and returned an error for the whole tree, so tuning steered away from values for reasons unrelated to the tree. This spec makes a degenerate triangle a local event (dropped, counted, normals from what remains) while every tree that already builds stays byte-identical.

## Decisions

- worker, fn-45 round 6 (2026-09-15): "at apical dominance 0.59, seeds 2181184680 and 2779011501 fail with `surface triangle collapsed in float32` (`surface.rs:351`) — a hard `InvalidInput`, so no tree at all."
- worker, fn-45 round 6c (2026-09-15): "The float32 surface collapse is not caused by apical dominance. One candidate table failed protocol seed 55 at 0.58 and three random seeds at 0.9. The failures follow geometry, about one seed in a hundred."
- Find the cause, not only the symptom. The upstream resample rule was left unchanged: any merge tolerance catching the pairs would move shipped oaks that build today (oak resample gaps under 0.1 mm at 37 of seeds 1 to 100).

## Acceptance Criteria

- **R1:** cause reproduced on a named seed and recorded — met (test-only beech from fn-45's round-6c rows, seed 266 at apical dominance 0.58; also seeds 501, 506, 515 at 0.9).
- **R2:** zero-area float32 triangle dropped with the run's index range corrected, orphan vertices take the ring normal, non-finite position still an error, count above a bound fails — met (bound 4 x segments = 80 on beech/birch).
- **R3:** every preset byte-identical at every seed that built; failing seeds now build; 48-case protocol passes — met (716 preset-seed cases hash identical, none drops a triangle).
- **R4:** tests for a collapsed ring, sub-float32 radius far out, index table after removal, ring-normal fallback, non-finite error, dropped count — met (`crates/telperion-core/tests/surface_collapse.rs`).

## Result

Cause: `Planner::run`'s truncated-run resample (`branching/local/planner.rs`) unions planned and uniform stations, so when a limb is cut just short of whole internodes the stations pair up micrometres apart (11.39 µm at seed 266); a frame tilt then rounds two vertices to one float32 point. The shipped tables did not fail on this branch (800 beech/birch cases and the spec's named seeds built). Fix in `surface/normals.rs`: drop the triangle, shrink the run's `index_count`, `SurfaceMesh::dropped` reported as `counts.wood_dropped` in `species_measure`. Of 1,600 reproducing-family trees, 4 dropped 2 triangles each and the rest none. One test changed: `root_only_and_float32_limits` now builds (two caps dropped, 40 triangles).

## Open

None.
