# The field answers a tree's flowers

## Goal & Context

The killenberger.com agent porting the grove to `telperion@0.2.0-next.20261024.2131d5b` found: "Flowers aren't in the field. In 0.2.0 only the CPU tree engine produces flowers." The owner (2026-10-25): "no we're testing 0.2.0-next to fix these issues..", so this is fixed in 0.2.0 and joins the release (fn-237), not 0.2.1.

Why the field has none (checked by the host, 2026-10-25, against the code):
- The field (fn-257 R8) builds the tree and splats it into a grid. It has four words per cell: flags, the thickest wood radius, a leaf estimate and the owning limb system. R8 names its inputs as "every drawn segment and leaf-bearing segment"; 0.1.4's field had no flowers, and R8 kept its answers.
- The device field pass (`crates/telperion-render/src/generation/resident/field.rs`, `field/splat.wgsl`) runs the resident stages to stations and reads wood and leaf records. Nothing in it names the organ.
- The flowers are grown, but outside the field. `generation/preparation.rs:82` takes them from `x.organ()` ("the flowers are the CPU step's on both executors (fn-216)"), and `bloom.wgsl` draws florets on the device (fn-229). fn-294 put them in `TreeEngine.build`'s output (`tree.organ`) and left the field out of scope.
- So a Telperion or Laurelin grove tree is voxelised without its flowers, while the harness and `TreeEngine.build` show them.

## Unknown, check before the plan

- Where the field path can take the florets from. Which of `x.organ()` (CPU) and the device florets (`bloom.wgsl`) exists on the field's build, and what each costs there.
- Whether the field's grid box already includes the flowers. The drawn tree's bounds union them (`preparation.rs:88`); the field's `Frame` may not.

## Requirements

- **R1 (the answer):** each cell answers `flowers`: the florets whose seat (the point the floret hangs from) lies in the cell, as a count, so the grid's sum equals the tree's floret count. This is on the device and in the CPU reference (`crates/telperion-core/src/pipeline/field.rs`), from the same florets the drawn tree hangs, through the one pipeline. There is no second floret placement for the field. A flowering cell sets a flags bit of its own. A tree that flowers nowhere answers zero everywhere.
- **R2 (the box):** the grid's box contains every floret seat. If the field's `Frame` leaves flowers out today, it takes them in, with the change shown on the field's existing tests.
- **R3 (the package):** `FieldAnswer` gains `flowers: Float32Array` (the binding goes from four words a cell to five). `telperion/field/voxelize`'s `Cubes` gains `flowers: Uint32Array`, the cells to draw as flowers: a cell that is not drawn as wood and has flowers, dropped with its limb system where thinning drops that system. Docs, the README, `docs/migration-0.2-flowers.md` and the field's migration note say so.
- **R4 (proof):**
  - Telperion and Laurelin, seeds 1 and 5678, at 24, 36 and 48 cells: the device answers equal the CPU reference's flowers per cell (counts are exact); the sum equals `tree.organ`'s floret count; the oak answers zero.
  - The existing field answers (wood, radius, leaves, limbs) stay byte-identical.
  - The field's time per species against fn-257's numbers is measured and reported. A cost over 5 % on a flowering preset is named for the owner.
  - The gate, `npm test` and CI's package job are green. Codex review runs inside telperion-heavy.
  - The owner looks at a voxelised Telperion in the harness or on the site, and the owner's words are recorded.

## Boundaries

- No change to what `TreeEngine.build`, the renderer or the drawn tree produce.
- No new request flag: a field of a flowering tree answers its flowers.
- Joins the release (fn-237). A new `next` is published only when the owner asks.
