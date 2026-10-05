# fn-208 step 1: the curve data (worker, 2026-10-05)

## Built

- `telperion_core::surface::Curve` (`crates/telperion-core/src/pipeline/surface/curve.rs`), reached through `Expansion::curve()`.
  - **Points:** centre, radius, distance along, and the frame.
  - **Runs:** first point, count, trunk, an optional section index, largest radius.
  - **Clusters:** at most 32 points, sharing ends, with a bounding sphere, largest and smallest radius, and a monotone six-level ring-removal error ladder.
  - **Sections:** the shaped runs' cells.
  - **Tree-wide rows:** lobes, lobe depth, twist rate, height.
- **Production:** the sweep's own steps (`paths`, `rank`, `Scratch::sweep`, which squares a shaped run's frames). No sweep code is duplicated: `emit_run` now takes a `Ring` (rows, height, angular samples) instead of the whole `Swept`, so the curve can draw rings without a tree.
- **The 28-byte packed point** (`curve/pack.rs`): float32 centre, radius and along; octahedral 16-bit normal and binormal.
- **Documented** in `docs/pipeline.md`, "The wood's curve".

## Tests (`curve/tests.rs`, every values preset: the seven catalogue presets and the beech)

| Test | Result |
|---|---|
| `every_presets_rings_are_drawn_from_its_curve_to_the_bit`: the rings drawn from the curve alone against `surface::build`'s positions and coords | Bit-identical for all eight, at each preset's side count (12, or 4 × lobes for the lobed legends), the palm's shaped cells included |
| `clusters_bound_every_ring_and_their_ladder_rises` | Green |
| `packed_points_stand_within_their_stated_error`: within 1e-4 of the run's radius plus 2.4e-7 of the distance from the origin | Green; the worst share is 0.49 of that bound |

The `surface` tests (43) are green after the `emit_run` change.

**Measured packing error, worst vertex per preset:**

| Preset | Worst error |
|---|--:|
| Norway spruce | 2.1e-6 m |
| Silver birch | 5.3e-6 m |
| Date palm | 8.9e-6 m |
| European beech | 1.1e-5 m |
| Oregon white oak | 1.1e-5 m |
| Ordinary | 1.9e-5 m |
| Telperion | 2.8e-4 m |
| Laurelin | 5.7e-4 m |

**The frame's quantisation scales with the radius.** It is 1e-4 of it, so 0.57 mm on Laurelin's metres-wide trunk. That is about 12 px on a camera 5 cm from that bark, though the silhouette is not in view there. If close-up bark on the legends' trunks shows it, the frame needs more bits (for example 21 bits a component in 8 B, making the point 32 B), and that change belongs to the host.

## Host decision 6: the palm's leaf-base cells

**The 28-byte point cannot carry a cell's cross-section; the curve carries it per run.**
- A shaped run's vertices are not `centre + frame · radius`. They come from `Section::vertex`, which blends the ellipse toward the diamond through the cell's corners and wraps it on the stem's cylinder (`crates/telperion-core/src/tree/section.rs:60-71`).
- The curve therefore keeps the cell itself, `tree::Section` (origin, axis, radial, across, two corners, flatness, three rings), in `Curve::sections`, and the run points to it.
- With that, the palm's rings are reproduced to the bit.
- **On the GPU,** a section is about 26 float32 (104 B) per shaped run, 256 on today's palm. The tessellator evaluates `Section::vertex` for those runs with its side count chosen by the same error rule. There is no Hermite subdivision, since a cell has three rings.

**For the host:** this is a per-run profile beside the 28-byte point, not inside it. Under decision 6 I am reporting it before step 3. Step 3 proceeds on this basis unless the host decides otherwise.

## Gate

`cargo test --profile ci --workspace --no-fail-fast` after steps 1 and 2 had two failures (`raw/gate.log`):
- **`generation_limit_guard`:** the curve's nine limit sites were unclassified. They are now declared in `docs/generation-limits-inventory.json`, and the test is green.
- **`fixed_beeches_pass_geometry_and_profile_gates_with_repeatable_varied_specimens`:** a peak resident of 5.91 GB against its 5.76 GB ceiling under the parallel gate. It is green rerun alone (FRICTION.md). The test is unchanged by this work.

Everything else passed.
