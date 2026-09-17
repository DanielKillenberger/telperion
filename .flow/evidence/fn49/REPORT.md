# fn-49: a collapsed triangle drops, the tree still builds

Branch `fn-49-a-collapsed-triangle-drops-the-tree`, base `a0df57f`. CPU only;
no renders.

## R1: the reproduction and its geometry

**The shipped tables do not fail on this branch.** `species_measure` built all
800 cases of the shipped beech and birch at seeds 1 to 400. The variant the
spec names, the shipped beech at apical dominance 0.59, built seeds 1 to 1,000,
and at 0.9 it built seeds 1 to 1,000 too. The spec's seeds 2181184680,
2779011501 (at 0.59) and 55 (at 0.58) build. On fn-45 they failed on the
round-6c candidate beech, which also carries a twig generation rail
(`twigs.generations = 2`) that this branch lacks.

**The reproducing family.** It is a test-only family built from the shipped
beech plus the wood rows of fn-45's round-6c table: `laterals_per_station`
2, `lateral_pitch` 58, `rise_secondary` 0.1, `lateral_spacing` 2.2,
`lateral_length_ratio` 0.6, envelope `crown_base` 0.05, `fullness` 0.48 and
`shoulder` 1.5, twigs `laterals` 4, `length_ratio` 0.36, `angle` 32 and
`divergence` 180, radii `length_taper` 0.2 and `fork_exponent` 2.6. No shipped
table was edited.

| apical dominance | seeds swept | failed with `surface triangle collapsed in float32` |
|---|---|---|
| 0.58 | 1 to 1,000 | 266 |
| 0.9  | 1 to 600   | 501, 506, 515 |

**Seed 266 at 0.58, the collapse in numbers.** Run 42 of 90,549 is a limb of
57 rings leaving node 215. Its rings 31 and 32 are path nodes 162827 and
173817:

| ring | centre (m) | path distance (m) | radius (m) |
|---|---|---|---|
| 31 | (-7.683839857956721, 24.843681213409383, 4.568097187389429) | 31.318201725 | 0.03273594150 |
| 32 | (-7.683843241581841, 24.843688164451190, 4.568105547588854) | 31.318213112 | 0.03273586583 |

- The two centres are 11.39 µm apart. The radii differ by 7.6e-8 m.
- The limb is straight through them: the segments on either side point along
  (-0.297, 0.611, 0.735) to three places.
- Averaging the micrometre segment into the tangent tilts ring 32's frame
  2.05e-4 rad from ring 31's.
- At segment k = 1 that tilt carries the vertex back along the axis by almost
  exactly the 11.39 µm step. The float64 positions come out as
  (-7.674627310916917, 24.82146052220573, 4.590301054750391) and
  (-7.674627373847851, 24.82146063712204, 4.590301081420153).
- They differ by (6.3e-8, 1.15e-7, 2.7e-8) m, which is under half a float32
  step on every axis. The steps are 4.8e-7 m at |x| = 7.7 and |z| = 4.6, and
  1.9e-6 m at |y| = 24.8.
- Both round to (-7.6746273, 24.82146, 4.590301), so triangle
  [48165, 48185, 48184] and its neighbour across the shared edge span no area.

**The other three seeds are the same geometry:**

| seed | gap between the ring pair | height | radius |
|---|---|---|---|
| 501 | 60.4 µm | y = 20.85 m | 0.0354 m |
| 506 | 10.8 µm | y = 22.70 m | 0.0332 m |
| 515 | 27.0 µm | y = 12.39 m | 0.0131 m |

Of the candidates the spec lists, the geometry is **two consecutive rings at
nearly the same position**. It is not a radius below float32 resolution, a
ring collapsed at a fork or a tip cap.

**The rule upstream that makes the ring pair.** It is `Planner::run`'s
truncated-run resample (`crates/telperion-core/src/branching/local/planner.rs:148-169`).
When a limb's planned course is cut short by its room, the run is resampled
onto the union of the course's own stations and `count` uniform stations over
the shortened length. The union is deduplicated only below 1e-12 m. When the
cut lands just short of a whole number of planned internodes,
`count = ceil(internodes · actual / length)` equals that number. The uniform
spacing then falls micrometres short of the planned stride, and the stations
pair up, the k-th pair k times that shortfall apart.

In seed 266 the limb was planned at 4.3597 m in 23 internodes, a stride of
0.1895533 m, and cut to 3.0328069 m. Its 16 uniform stations are 0.1895504 m
apart. The shortfall of 2.85 µm per station gives 13 pairs under 0.1 mm, at
2.85, 5.69, 8.54 and 11.39 µm and on up to 42.7 µm. The 11.39 µm pair
collapsed.

**The rule is not changed, for two reasons.**

1. It does not make a zero-length ring. The paired stations are distinct in
   float64. The shortest resample gap seen anywhere is 0.31 µm, 300 times the
   paths builder's 1e-9 m merge. The collapse comes from float32 rounding in
   the surface.
2. Any merge tolerance large enough to catch these pairs would move shipped
   trees that build today. The shipped oak has resample gaps under 0.1 mm at
   37 of seeds 1 to 100, among them seed 56 at 0.31 µm, seed 21 at 2.1 µm and
   seed 39 at 3.8 µm, and it builds at every one. Merging would change those
   skeletons, their node counts and their ring counts. The spec rules that
   out twice: byte identity, and no change to the sweep's ring count.

The shipped beech, birch and spruce have no resample gap under 0.1 mm at
seeds 1 to 300. The fix is therefore the surface's alone (R2).

## R2: the fix and the bound

- **Where it happens.** Each run is shaded as soon as it is swept
  (`surface/normals.rs`). A triangle whose float64 cross product of its
  float32 corners is exactly zero leaves the index buffer, and the run's
  `index_count` counts only the triangles that remain.
- **Normals.** Vertex normals sum the remaining triangles in index order at
  float32 precision. A run with nothing to drop is therefore shaded exactly
  as before.
- **The fallback.** A vertex left with no triangle, or whose sum cancels,
  takes its ring's direction: out from the axis for a ring vertex, along the
  axis for a cap.
- **Errors that remain.** A position float32 cannot hold is still
  `surface float32 position overflow`. A summed normal that overflows is
  `surface normal overflow`.
- **The count.** `SurfaceMesh::dropped` reports how many triangles were
  dropped, and `species_measure` records it as `counts.wood_dropped`.
- **The bound.** It is two rings' worth, `4 × segments`: the strips on both
  sides of two rings collapsed to a point. That is 80 triangles for the
  20-segment beech and birch. Past it the build fails with
  `InvalidValue { field: "surface triangles collapsed in float32", value: <count> }`.
- **Measured against the bound.** Of the 1,600 reproducing-family trees above,
  1,596 drop nothing and 4 drop 2 each; the most any tree drops is 2. The
  bound is 40 times that.

| case | before | after |
|---|---|---|
| seed 266 at 0.58 | error | builds, 11,318,558 triangles, 2 dropped |
| seed 501 at 0.9 | error | builds, 10,415,998 triangles, 2 dropped |
| seed 506 at 0.9 | error | builds, 12,038,918 triangles, 2 dropped |
| seed 515 at 0.9 | error | builds, 11,282,878 triangles, 2 dropped |

In every rebuilt case the run spans tile the index buffer and every normal
has unit length.

## R3: byte identity

- **The hash sweep.** Each wood mesh was hashed with FNV-1a over its
  positions, normals, coordinates, indices, run table and bounds, at `a0df57f`
  and again after the fix. The sweep covers 716 preset-and-seed cases:
  - beech and birch at their 24 protocol seeds and at seeds 1 to 200
  - oak and spruce at seeds 1 to 100
  - ordinary, telperion and laurelin at seeds 1 to 30

  All 716 are identical, and none drops a triangle.
- **The identity pins.** `tests/identity.rs` passes unmoved.
- **The protocol.** The 48-case fn-34 protocol ran with
  `node tests/species.mjs --profiles .flow/evidence/fn34/profiles.json --seeds .flow/evidence/fn34/seeds.json --output .flow/evidence/fn34/measure/protocol-fn49 --measure-only`.
  All 48 cases completed and passed their numeric checks, with
  `wood_dropped` 0 on every case. The output directory is gitignored.

## R4: the tests

In `crates/telperion-core/tests/surface_collapse.rs`:

| case the spec requires | test |
|---|---|
| a ring collapsed to one point | `a_ring_collapsed_to_one_point_drops_the_strips_that_meet_it` (drops 2 × 12) |
| a radius below float32 resolution far from the origin | `a_tip_below_float32_resolution_far_out_drops_its_cap` (a 1 mm tip at 1e5 m; the cap vertex faces +y) |
| the index table after a removal | `the_run_table_shrinks_only_the_run_that_dropped` (two runs; only the limb's span shrinks, by 3 × 24 indices; spans stay disjoint) |
| the ring-normal fallback | `a_vertex_left_without_a_triangle_faces_out_from_its_ring` (three rings stacked a micrometre apart at 1e5 m) |
| the non-finite error | `a_position_float32_cannot_hold_is_still_an_error` |
| the dropped count and the bound | `past_two_rings_worth_the_tree_fails_naming_the_count` (three stacked rings drop 48, the bound, and build; four drop 72 and fail naming 72) |
| the reproducing seed | `the_reproducing_beech_builds_and_drops_two_triangles` (seed 266, test-only family) |

One existing assertion changed. In `tests/surface.rs`,
`root_only_and_float32_limits` builds a tree 1e-10 m thick at x = 1e10 m, and
that tree used to fail. Now its rings flatten into slivers that keep their
area, only its two caps drop (40 triangles, inside the bound), and it builds.
