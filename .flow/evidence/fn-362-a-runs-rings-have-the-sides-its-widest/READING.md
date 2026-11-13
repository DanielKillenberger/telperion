# fn-362 R1: where the CPU mesh takes its sides, and who reads them

Read on 13 Nov 2026 at 038ce5548, before any product change. Every claim below names the code it was read from; nothing here was run.

## Where the count comes from

- One count for the whole wood: `surface::segments(params) = max(radialSegments, 4·lobes)` (`pipeline/surface/build.rs`). Every shipped preset has `radialSegments` 12 and `lobes` 5 (`src/browser/presets.generated.ts`), so the count is 20, the "cap 20" of the design. The cap this task applies is therefore `segments(params)`, not the bare `radialSegments` row; for every shipped preset they read 20 and 12, and the design's paper numbers use 20.
- `angular::samples(segments, params)` builds the one ring of `segments` angles every run is emitted with (`Swept.angular`, `build::Ring`, `emit_run`).
- `Rings.segments` carries the count from the ring step to the mesh step; `Layout::new`, `Piece::{vertices, indices, local}`, `chunks::record`, `budget::{check, need, scratch}`, `build::{shaded, split, shade, facing}`, `prepared::Run::visit_triangles`, `normals::accumulate`, `rings::{written, table}` and `parallel::{admitted, rings, faces}` all read it as the stride of every ring.

## Does the GPU path read it

No. The curve (`curve::traced`) records each run's sample points, frames and `largest_radius`; it builds `angular::samples(segments)` only to fill `Swept`, and nothing it stores depends on the count. The renderer draws only the curve (`docs/pipeline.md`, "The renderer's one wood path"); its sides are its own, per ring, from the screen error (`curve_walk.wgsl::sides`, `MOST_SIDES` 384), and the CPU reference tessellation's likewise (`curve/tessellate/plan.rs::sides`). So the GPU's drawn wood does not change, and `render::curve the_device_surfaces_the_wood_the_cpu_reference_does` still compares the device with the reference tessellation of the same curve; only the hero pose it reads from the CPU mesh's bounds can move, by at most a ring's flat.

`Curve::rings(segments)` (the curve drawn back into the sweep's rings, at one count) is a CPU proof, not a GPU path: it must take the same per-run rule for `every_presets_rings_are_drawn_from_its_curve_to_the_bit` to keep checking that the curve carries the whole wood. `CurveRun.largest_radius` is exactly the value the rule reads (the rank's widest sample radius), so the curve can decide each run's count on its own.

## Every consumer of the ring's vertex count

1. The chunk layout and the run table (`chunks.rs`, `rings::table`): a piece's vertices and indices are its rings times the count. Per-run counts go on the piece.
2. The ring writer (`rings::written`): ring-of-vertex arithmetic (`v / segments`), the repeated first ring of a split run, the distances per ring. Per piece.
3. The mesh step (`build::shaded`, `split`, `shade`, `facing`; `prepared::Run`; `normals::accumulate`): strip and cap triangles by stride. Per piece; the dropped-triangle allowance (`DROPPED_RINGS · 2 · segments`) stays at the cap, its largest.
4. The memory check (`budget::need`): rings per chunk are `vertices / segments`; with mixed counts they are counted from the pieces.
5. The parallel sweep's admission (`parallel::admitted`): extent and scratch from the layout; the scratch at the cap is its bound.
6. Bark coordinates: each vertex carries its own angle (`sample.angle`, `k / n` of a turn), so the bark round a stem reads the count only through the angles it is emitted with. No consumer divides by the count.
7. Caps: one centre vertex a cap, a fan of the run's own count (`visit_triangles`). Per run.
8. The contact surface for seated leaves (`attachment.rs`): `AttachmentSurface::on_wood` reads the drawn rings in place, `AttachmentSurface::selected` sweeps its own ("exactly the float32 vertices submitted by build()"). Both stride by one `segments`, and `segment_bounds` is keyed `lo / segments`. With mixed counts the stride is the ring's own (`hi - lo`, two neighbouring rings of one run), and the bound key must be the ring's ordinal, since `lo / 3` would make the table up to seven times as long. `selected` has each run's samples in hand, so it takes the same rule from their widest radius. The Norway spruce is the one shipped preset that seats leaves on wood (`surfaceContact` 1; first written here as the oak, corrected after MEASURED.md read the spruce's leaves move): its seated needles move with the polygon, by at most a ring's flat (0.2 mm); those are downstream of the surface stage.
9. The socket join (`samples::branch_run`): the child's sunk ring is contained in the bearer's polygon by `inscribed = pr (1 − lobeDepth) cos(π / segments)`, at the cap's count. It is not changed: it sets the sample radius, which the curve stores, so changing it would change the GPU's wood. With the bearer cut with `n` sides, its true inscribed radius is smaller by at most `pr (1 − cos(π/n)) ≤ FINEST`, since the bearer's widest drawn ring is at least `pr` (girth never below the node's radius, swell and flare at least 1). So a child's socket ring can stand at most 0.2 mm (times `1 − lobeDepth`) past its bearer's faces, below what the rule lets the drawing show; no gap opens, since the child's ring is inside, not apart. The R5 sheet looks at a join.
10. The collar: the fork swell is a sample radius (`branch_run`), not a ring count; untouched.
11. The field (`stage::field`): reads the tree, not the rings. The wire and distant levels: the curve and the GPU (above). Neither reads the count.
12. Lobes: the profile `1 + lobeDepth · cos(lobes (θ + phase))` is sampled at the run's own angles. Below `4·lobes` sides the lobes are undersampled; the design's numbers (1.5 mm at 7 sides with every preset at 5 lobes) take that, so the lobe floor is the cap's and does not bind below it.
13. Shaped runs (the palm's leaf-base cells, `section.rs`): a section's sample radius is its extent; the rule reads it like any run's.

## How a child joins a bearer with a different count

There is no stitching between runs today: every run is its own closed shell, capped at both ends, its first ring sunk into the bearer's socket (item 9). A child of 5 sides on a bearer of 20 joins the way a 20 joins a 20, by interpenetration. One count a run means no ring-to-ring change of count inside a run.

## Tests that count sides (expected to change with what they check named)

- `curve::tests::every_presets_rings_are_drawn_from_its_curve_to_the_bit`, `packed_points_stand_within_their_stated_error`: draw the curve back at one count; they take the per-run rule.
- `tests/ground_reach.rs`: walks the mesh by `run.count · segments + 2`; it needs each run's count, which the public curve run must then give.
- `suite/surface.rs` (coords of a ring of `segments`), `suite/leaf_base_lattice.rs` (`(high + 1 - low - 2) / segments`), `suite/surface_attachments.rs` (polygon sides 3, 7, 20): hand-built or preset trees; whether they change depends on their radii.
- Every pinned digest of the CPU mesh's bytes changes wherever a run is narrower than 1.5 cm; pins of the grown tree, its nodes, radii, the curve and unseated leaves must not.

## Not checked here

Whether any test tree's narrow runs fall on a boundary of the rule; the render crate's CPU-mesh tests' radii; the Wasm entry tests' memory. These are read by running them.
