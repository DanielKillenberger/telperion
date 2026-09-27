# Whole-tree level of detail as prefixes of ordered data

> HTML render lens: `.flow/artifacts/fn-176-whole-tree-level-of-detail-as-prefixes/spec.html` (gitignored — open locally; regenerable, markdown is the record). <!-- flow-next:artifact-link -->

## Conversation Evidence

> user (2026-09-27): "yes but also we need to enable lod for this motion data. As we provide for rendering (do we do that as of now? or is the renderer choosing to simplify the full tree when rendered?). So simpler trees should also have simpler motion data. If we don't have lods in generator before rendering then i guess we can't really do that yet?"
> user (2026-09-27, on a whole-tree LOD spec, selected): "Explore first"
> user (2026-09-27, after the research on tree LOD techniques): "yes"

## Goal & Context
<!-- scope: business -->
<!-- Goal & Context: 70% [paraphrase], 30% [inferred] -->

Today the only level of detail is each leaf's shape: the generator builds nested levels of the leaf mesh and the renderer picks one per leaf, per frame, at half a pixel of error. Every leaf and every run of wood is drawn at any distance. One tree fits the frame (fn-5 measured 8 branch orders at 1.28 ms of a 16.7 ms frame), but a forest, fn-86's and fn-6's target, does not, and nothing makes a distant tree cheaper. [paraphrase]

The owner's rule binds any simplification: the far draw is the near draw minus what the eye cannot resolve at that distance, and nothing more, so a walk from the base to the hero pose shows no step in fidelity (decision of 2026-09-18). The owner also asked that simpler trees carry simpler motion data, which fn-15's motion levels begin. [paraphrase]

This spec gives the whole tree level of detail: fewer leaves and simpler wood with distance, as prefixes of data the generator orders once, so motion, shadows and the leaf-shape levels stay consistent with what is drawn. [paraphrase]

## Architecture & Data Models
<!-- scope: technical -->
<!-- Architecture: 50% [paraphrase], 50% [inferred] -->

- **One ordering, many prefixes.** The generator orders each kind of data once, and every coarser draw reads a prefix of it; there are no separate reduced models. This is the pattern fn-15's motion levels and the leaf-shape levels already follow. [paraphrase]
- **Leaves: stochastic pruning per cluster** (Cook & Halstead, Pixar 2007). A cluster is the leaves one run bears. Each cluster stores a stratified random pruning order over its own leaves, fixed per tree and uncorrelated with position, as an index list into the leaf buffer; the leaf buffer itself is never reordered, so every leaf keeps its index, and with it the `vary(id)` colour and mottle and fn-15's flutter phase. Each frame, each cluster picks its own kept fraction from its own projected size, as the renderer already judges each leaf by its own depth (`select.wgsl`), so a hero tree prunes its far clusters and keeps its near ones whole; within a cluster the kept leaves are a prefix of its order. A cluster keeps at least one leaf. Survivors are scaled so the cluster's drawn leaf area equals its full leaf area: the scale is the full area divided by the kept area, with transition weights counted, not `1/u`, because leaves differ in area. Leaves in the transition window shrink out rather than vanish, and colours move toward the cluster mean by the paper's contrast correction. Pruning starts only where a leaf's projected size reaches the pixel, the line leaf-shape selection already draws. (One fraction per tree was rejected: a tree spans near and far in one frame. A prefix over a whole-tree order was rejected: a near leaf late in the order would force every leaf before it to be drawn.) [paraphrase]
- **Wood: runs shrink, then drop.** Runs are ordered by fn-15's significance order (trunk, limbs by girth, fine branches). A run whose projected radius falls below a pixel shrinks toward its centreline and then drops. [paraphrase]
- **Wood: nested ring levels over shared vertices.** A run's rings keep their finest vertices; coarser levels are separate index lists over the same vertices, each ring at a level using every second vertex of the level finer than it, so every level's rings close by construction, the leaf-shape levels' pattern. Caps join the ring they close at each level. The level per run follows projected error. Shaped runs (the palm's leaf-base cells, fn-125) keep their cell at every level and are exempt from ring reduction. [inferred]
- **Geometry and motion drop separately.** Dropping a run's wood never drops its bone. A bone is evaluated while fn-15's R11 test keeps it (its motion would move something still drawn by half a pixel or more), whatever its wood's projected size: a thin, long branch can be below a pixel wide and still swing its surviving leaves by several pixels. A bone past the evaluated level rides its nearest evaluated ancestor, as R11 states; pruned leaves carry no motion. [paraphrase]
- **Impostors are out of scope** except, if ever, for trees a few pixels wide, since swapping mesh for a card is a visible step up close. [paraphrase]
- **Generator versus renderer.** The generator owns the orderings and the per-element data a prefix needs (the prune order, the area and contrast factors' inputs, run significance and projected-size bounds); the renderer, like any consumer, picks the prefix per frame. [inferred]

## API Contracts
<!-- scope: technical -->

- **Plan layout.** The prune order and run significance are part of fn-125's versioned layout; a consumer that ignores them draws the full tree unchanged. [inferred]
- **Views and commands unchanged.** [inferred]

## Edge Cases & Constraints
<!-- scope: technical -->

- **A hero tree spans near and far in one frame,** so the kept fraction cannot be a single number per tree without starving the near crown or wasting the far one; how it is chosen is settled per cluster (Architecture) and checked by R8. [paraphrase]
- **Shadows** draw the same prefix as the view unless the decision below says otherwise, so a thinned crown does not cast a full shadow. [inferred]
- **Full detail is the default.** At the hero pose and any distance where a leaf or run is at least a pixel, the draw is the full tree, byte-identical to what fn-125 and fn-15 draw. [inferred]
- **Budget.** Proof is a continuous walk of stills on the 8-tree scale at most; no full-forest capture. [paraphrase]

## Acceptance Criteria
<!-- scope: both -->

- **R1:** The plan layout carries each cluster's pruning order as an index list into the unchanged leaf buffer, the runs in fn-15's significance order, and each run's nested ring index lists. The stored leaf and wood buffers are byte-identical to fn-125's and fn-15's, and a full-detail draw matches today's draw pixel for pixel, colour, mottle and fixed-time wind included. Errors: no error surface beyond fn-125's layout errors. [inferred]
- **R2:** For each cluster at any kept fraction, the drawn leaves are a prefix of its pruning order with at least one leaf, their drawn area (transition weights counted) equals the cluster's full leaf area within 1%, and the contrast correction holds the crown's mean linear colour within 2% of the full crown's at the same view. Tests cover clusters of unequal leaf areas, kept fractions down to one leaf, and fractions at both edges of the transition window. Errors: a kept fraction outside (0, 1] is refused by name. [paraphrase]
- **R3:** A run below the pixel threshold shrinks to its centreline before dropping; its bone stays evaluated while fn-15's R11 test keeps it, and a test with a thin, long branch whose wood is dropped but whose surviving leaves move more than half a pixel keeps that motion; no leaf is drawn floating off wood that is still drawn. Errors: no error surface beyond R2. [paraphrase]
- **R7:** Every ring level of every run is closed: a topology test finds no open edge on any level of every catalogue preset's wood at seeds 1 and 7, caps included, and shaped runs draw their full cell at every level. Errors: no error surface beyond R1. [inferred]
- **R8:** Before the layout is fixed, a probe on the oak at the hero pose and at a far view measures per-cluster pruning against the full draw: the near crown is drawn whole (R4) and the far clusters prune. If it fails either, the layout is not fixed and the spec stops with `NEEDS_HUMAN` and the probe's numbers. [paraphrase]
- **R4:** Nothing is simplified while a leaf or run projects to a pixel or more: at the hero pose the draw equals the full tree. Errors: no error surface beyond R2. [paraphrase]
- **R5:** Frame cost falls with distance, measured on the 8-tree forest at three distances (near, mid, far) against the full draw, and reported with the drawn leaf and vertex counts. Errors: no error surface beyond R2. [inferred]
- **R6:** A sweep of stills across a continuous walk from the far distance to the hero pose, with and without wind, shows no visible step, judged by the owner in the harness; a rejecting verdict stops the spec with the owner's words. [paraphrase]

## Boundaries
<!-- scope: business -->

- No impostors or billboards for trees larger than a few pixels. [paraphrase]
- No separate reduced models of a tree; every level is a prefix of the one ordered tree. [paraphrase]
- No full-forest capture; captures follow the budget rules. [inferred]

## Decision Context
<!-- scope: both -->

Prefixes over one ordering because every alternative the research found either swaps representations, which the owner's no-step rule forbids (discrete LOD copies with cross-fades, billboards, octahedral impostors), or is a whole virtual-geometry engine (Nanite foliage). Pixar's stochastic pruning and SpeedTree's centreline shrink both start at the pixel and keep appearance by construction, and Weber & Penn 1995 already drew coarser trees by changing draw-loop limits over one description. [paraphrase]

Depends on fn-15, whose significance-ordered bone table and motion levels this spec's wood ordering reuses, and through it on fn-125's plan layout. [paraphrase]

## Resolved via Research
<!-- provenance: research subagent on tree LOD techniques, 2026-09-27 -->

### literature
- **Stochastic pruning:** survivors scale by `1/u` in area; contrast corrected by α with α² the ratio of pruned to unpruned elements per pixel; pruning order random or stratified and consistent; starts when elements are about a pixel. Source: Cook & Halstead, Pixar 2007, https://graphics.pixar.com/library/StochasticPruning/paper.pdf
- **Real-time port:** a vertex-shader version with one draw count. Source: Macklin 2010, https://blog.mmacklin.com/2010/01/12/stochastic-pruning-for-real-time-lod/
- **SpeedTree runtime:** leaves shrink away while the rest scale up; branches scale down to their centreline under one LOD value; wind steps down with LOD; billboards cross-fade. Source: https://docs.unity3d.com/speedtree-runtime-sdk/manual/level-of-detail.html
- **Weber & Penn 1995:** one description with changing draw-loop limits, grouped in masses so pruning spreads across the tree, avoiding "resolution waves". Source: https://courses.cs.duke.edu/cps124/fall01/resources/p119-weber.pdf
- **Nanite foliage (UE 5.7+, experimental):** assemblies, clusters turning into near-pixel voxels, wind through bone skinning at about 0.1 ms per 100k bones. Source: https://dev.epicgames.com/documentation/unreal-engine/nanite-foliage
- **Octahedral impostors:** baked-view atlas, break down up close. Source: https://shaderbits.com/blog/octahedral-impostors

### repo history
- **An LOD ladder was cut early:** "a far impostor only if the measurement demands one". Source: `.flow/specs/fn-1-the-canopy-real-leaf-geometry-culled-to.md`
- **One tree fits the frame:** 8 orders at 1.28 ms of 16.7 ms. Source: `.flow/tasks/fn-5-branch-until-the-tips-bear-leaves-one.5.md`
- **Forest needs LOD next:** cluster cards, LOD tiers, impostors named as the forest's next need. Source: fn-86; fn-6 on rendering at scale; fn-23 on projected-error LOD without stepping.

## Strategy Alignment

- **Surface and rendering at scale:** a forest needs distant trees to cost less, drawn from the same generated tree. [strategy:Surface and rendering at scale]
- **Approach:** every tree passes through one pipeline, and a level is a prefix of its data, never a second model. [strategy:Surface and rendering at scale]

## Strategy Conflicts

None found.

## Parked unknowns

- Whether the pruning order fits beside fn-86's 12-byte leaf and the leaf-shape levels without a second leaf word; resolved when the layout is drafted.
- Whether shadows and leaf flutter use the view's kept fraction; resolved by the walk's stills.
- Whether contrast correction runs on shader inputs or on shaded colour; resolved by the R2 colour test.
- The ring-segment error metric for wood; resolved in planning against the leaf-shape levels' deviation rule.

## Requirement coverage

| R-ID | Task |
|------|------|
| R1 | fn-N.M (TBD - populate via /flow-next:plan) |
| R2 | fn-N.M (TBD - populate via /flow-next:plan) |
| R3 | fn-N.M (TBD - populate via /flow-next:plan) |
| R4 | fn-N.M (TBD - populate via /flow-next:plan) |
| R5 | fn-N.M (TBD - populate via /flow-next:plan) |
| R6 | fn-N.M (TBD - populate via /flow-next:plan) |
| R7 | fn-N.M (TBD - populate via /flow-next:plan) |
| R8 | fn-N.M (TBD - populate via /flow-next:plan) |
