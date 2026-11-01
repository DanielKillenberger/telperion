# Measurement manifest for goal fn-321

Frozen 1 Nov 2026, 13:30 Zurich, before any measurement of fn-323 ran. Commit order shows that. Base: the integration branch at or after `e04c51ae5`. RTX 3080, driver 615.78.08. Every value here is the host's, unless quoted from the goal. A change is a dated, reasoned entry at the end, and no change follows a result it would affect.

## Fidelity

Fidelity is the size of the smallest feature the tree resolves, in metres. Wood and leaf area below it are held as statistics.

| Name | Value | Use |
| --- | --- | --- |
| grove | 1.0 m | R2, R3: a website's voxel grove |
| whole | 0.25 m | a tree filling the view |
| mid | 0.05 m | R5's walk at mid distance |
| finest | 0 | the full cycle, "the finest setting" |

A value below 0, or not a finite number, is refused by name.

## Ages and seeds

- Preset age: each preset's own.
- Oak ages: 40, 80, 120, 200, 300.
- R4's reference and its comparison: seeds 1-50, as the goal holds them out.
- This child's floor: seeds 1, 7, 42, 1234 and 5678. They are in R4's held-out set only as far as 1, 7 and 42, and nothing is fitted to them here.

## Cameras, viewport and the walk

- Whole view: `hero_pose` at the tree's bounds.
- Viewport: 1920 x 1080 at pixel ratio 1 for R6. Stills are 960 x 720.
- Grove view: a camera 60 m out, 10 m up, looking at the grove's centre, over a 16-tree square grid at 12 m spacing.
- The R5 walk: 120 frames, the camera's distance falling geometrically from the whole view's to 5 cm. Its target is the tip of the highest-order axis nearest 60% of the tree's height, at seed 1.
- R5's probe for child 2: wood length and leaf area realised against the coarser level's stated amount, at each of the walk's 120 frames.

## The first image's completion boundary

From the call that requests the tree to the GPU's report that the first frame holding that tree has finished (`on_submitted_work_done`). Warm means the same page or process has already grown a tree of that preset, with the shader cache state stated.

## Definitions

- **Limb count:** the distinct limb systems of the tree at the preset's `clump_system_order`, as the field's limbs name them, counted over its kept nodes.
- **Shed share:** one less the phytomers the finished tree keeps over the phytomers grown for it, the retired and shed ones included: `1 - kept / grown`.
- **Order:** an axis's count of lateral buds from the seed. A continuation and a relay keep their parent's order.
- **Skipped axes:** in reports by order, axes under 3 phytomers are counted apart. They are still grown and counted in costs.

A metric that cannot be read is an error, never a zero.

# The route's stop, frozen for fn-323

## Candidates

A candidate is a cutoff and a density.

- **Cutoffs:** keep axes of order k or less, for k in 1, 2, 3, 4, 5.
- **Densities,** for the wood and leaves of each subtree cut at the cutoff:
  - **Sphere:** spread evenly over the cells whose centres lie within a sphere at the subtree's leaf centroid, of radius its RMS leaf distance from that centroid.
  - **Gaussian:** a Gaussian at the same centroid with sigma the RMS distance over the square root of 3, cut at 2 sigma and renormalised.
  - A subtree that reaches no cell centre goes whole to its nearest cell.
- That is 10 candidates. A subtree's centroid and RMS come from the full tree, so this is an upper bound on what a closure could know. **This replays density on a tree grown whole. It cannot show that coupled growth under the density gives the same tree.**

A cell holds wood where kept wood reaches it or the wood deposited is at least 25% of the cell's side, and foliage where kept foliage reaches it or the leaves deposited are at least 0.5.

## The field limit

Against the full tree's 36-cell field, over the same frame, with the measures of `field_tests::compare`. A candidate passes at an age only if the median over the seeds meets all of these:

| Measure | Limit |
| --- | --- |
| wood, cells' Jaccard | at least 0.85 |
| foliage, cells' Jaccard | at least 0.85 |
| leaves, sum of cell differences over the total | at most 0.25 |
| total leaves against the full tree's | within 1% |
| limbs, the same partition | at least 0.85, for a cutoff at or past the preset's `clump_system_order` |

These are set without a measurement behind them, so they are a judgment. A field a website voxelises must keep its shape and limb masses, and need not keep each twig's cell.

## The timing protocol

- One thread: `taskset -c 2`, `RAYON_NUM_THREADS=1`, no GPU, through `telperion-heavy`.
- Native, `--profile ci`. Five repeats, the first thrown away as warm-up, reported as median and range.
- **Measured:** the full cycle's seconds, its light's seconds and each cycle's living apexes by order, from `grow_census`.
- **Scaffold cost, a model:** `(seconds - light seconds) / living apex-cycles`, times the living apex-cycles of order k or less. It assumes cost per apex holds across orders, which is unmeasured, and it is named a model wherever it is reported.
- **Light cost, a replay:** `Field::quantised` timed on the finished tree at every 10th year, on the leaves of its kept axes plus 8 points per cut subtree, and the tips of its kept axes, with the result scaled to all years. Named a replay wherever it is reported.
- A browser's Wasm build is not this protocol. An unmeasured cost is written as unknown.

## The verdict

Goal Route stop, applied at age 80, to the candidates that pass the field limit at both 40 and 80: go on to child 2, with the cheapest candidate, if any has scaffold plus light at 50 ms or less. Otherwise stop `NEEDS_HUMAN` with the numbers and next options. This child's limit is 3 hours, from 13:10 Zurich.
