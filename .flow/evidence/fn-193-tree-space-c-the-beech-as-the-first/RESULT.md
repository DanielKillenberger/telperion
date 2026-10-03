# fn-193 result: the beech as the first point (worker, candidate for the host and Astra)

The beech is values only (`crates/telperion-space/src/beech.rs`; sources in `SOURCES.md`). It is drawn through the pipeline's own expansion and the headless renderer by `crates/telperion-render/examples/space_beech.rs`; the pipeline is untouched. Measured on 2026-10-03 on the desk (AMD Ryzen 9 5950X, RTX 3080), release profile, other sessions running.

**The species is not passed.** The host and Astra judge the sheet (R3); this file is the worker's own reading.

## What the engine gained (geometry and the rendering path)

- **`Form`, per PA, each continuous and neutral where it changes nothing:**
  - `tropism` and `elevation`: the axis bends toward an elevation.
  - `wander`: each node turns the axis by a keyed draw.
  - `plane`: the turn of the plane a parent's laterals branch in.
  - `pipe`: each phytomer's section.
- **Laterals leave from their parent's direction at their node**, carried along the bending axis. fn-192's decision was "from the parent's heading". That was the same where axes were straight, and is not on a bent limb. This is a decision for the host.
- **Girth** comes from the pipe model (`girth.rs`); presence scales each phytomer's section.
- **The rendering path** is `examples/space/tree.rs`, which turns the structure into a pipeline `Tree` and hands it to `executor::expand(..).mesh()`. Leaves, bark and the leaf element are the european-beech preset's rows. Rules in the conversion:
  - Wood under 0.2 mm radius is not drawn.
  - Internodes under 1 mm merge into their node.
  - A lateral of 0.4 to 0.7 of its parent's radius grows in as a codominant fork, so the fork has no collar.
- **Tests:**
  - `tests/form.rs`: tropism, wander, plane, local frames, pipe model, and the beech growing at every sheet age.
  - Refusals for the five new settings.
  - The walk test walks the four geometric `Form` settings on every PA. Rates per metre are walked in radians over the metres the axis grows.
  - The whole crate is green (`raw/space-tests.log`, `raw/walks.log`).

## The sheet (R2)

`raw/final/sheet-beech.png` (ignored, on disk; regenerate with `cargo run --release -p telperion-render --example space_beech -- <dir> 10 20 40 80` and the `montage` line in this task's summary):

- Row 1: the photographs S1 Entzia, S1 Rostock, S2 Hrachoviště and S2 Nettleden.
- Row 2: the space beech at 80 cycles, seed 1 bare, seed 7 bare, seed 1 in leaf, seed 7 in leaf.
- Row 3: today's beech from the pinned baseline, in the same order.
- Rows 4 and 5: 10, 20 and 40 cycles bare and 40 in leaf, at seed 1 and then seed 7.

The camera is fitted to each tree. The figure is 1.8 m tall.

## Trait by trait (worker's reading of every still)

| Trait | Seed 1 at 80 | Seed 7 at 80 | Reading |
|---|---|---|---|
| Trunk division and fork | The bole divides at about 0.25 H into four or five ascending limbs, blended at the fork | The same, with a slight collar left where thinner limbs leave | **Reads as a beech**, closest to S1 Rostock and S1 Entzia. Today's beech has no such division. |
| Limbs reaching the outer crown | The limbs rise steeply, and the crown is ovoid and narrow (16.7 m) | Limbs run out to a 21 to 22 m crown | **Partly.** Seed 7 reaches out; seed 1's limbs stay inside an ovoid. |
| Flat two-ranked branch systems | Present in the structure; not readable at whole-tree scale. Leaves show as flat horizontal slabs | Horizontal layered sprays at the periphery | **Not judged readable** at this framing. A closer view (S3, S4) would be needed. |
| Crown outline | Irregular ovoid, lobed, open to one side | Broad and flat-topped: umbrella-like rather than domed | **Seed 1 plausible; seed 7 not a beech dome.** In leaf neither has the dense, dark, rounded mass of S2 Nettleden. The foliage is pale and streaky; it is the preset's leaf rows, unchanged. |
| Clear bole | Clear to the fork | Clear, with a few thin stragglers below the crown | **Reads.** |
| Girth | Stout, pale bole; the base radius is 0.72 m | 0.85 m | **Too thick**: about 1.4 to 1.7 m across at the ground, above TSO's dbh of up to 1.3 m. The pipe radii are estimated. |
| Younger ages (10, 20) | A straight pole with near-horizontal straight rods | The same | **Does not read as a beech: it reads as a conifer sapling,** the fn-190 failure. The trunk is monopodial and the branch systems show only as rods. A change aimed at it was made: longer branch and shoot internodes, with more long shoots tried and reverted. It did not change the look. **That is one rejection of this trait by the worker.** |
| 40 | Several limbs, a few looping hanging shoots | A broad crown on a long bole | Transitional. Seed 1's looping shoots (wander) look wrong. |

**Overall:** the mature seed 1 bare tree is the candidate. It shows the fork, the ascending limbs and the clear bole a beech has. Seed 7's flat-topped umbrella, the in-leaf texture and the young trees are the weak points I would expect a judge to name.

## Time, fine wood and memory against the pinned baseline (R4)

| Seed | Kept nodes (baseline) | Grown phytomers incl. shed (expected) | Engine ms (baseline skeleton ms) | Pipeline dress ms (baseline full build ms) | Fine wood km (baseline) | Peak RSS MiB (baseline) |
|---|--:|--:|--:|--:|--:|--:|
| 1 | 561,136 (213,044) | 5.8M | 1,080 to 1,130 (57.3) | 842 (3,471) | 10.38 (46.66) | 1,962 (727) |
| 7 | 861,370 (215,470) | 5.8M | 1,800 to 1,930 (57.8) | 1,342 (3,474) | 15.55 (47.15) | 3,261 (733) |

- **Engine time** is `grow` (growth, shedding, geometry, girth) at 80 cycles, three runs of `examples/beech`. It is **19 to 33 times** today's skeleton. Most of it is wood grown and later shed: the closed form expects 5.8M phytomers grown against 0.6 to 0.9M kept.
- **Dress** is the pipeline's expansion to wood and leaves. It is shorter than today's full build, because there are 1.3 to 2.0M leaves against today's 5.8M.
- **Fine wood** is wood on nodes under 0.05 of the root's radius, by the baseline's rule. Everything but the bole and the main limbs falls under it here, because the root is so thick.
- **Peak RSS is not like for like.** Ours includes the GPU renderer and both stills; the baseline's is mesh builds only.
- Phase F owns speed. The cheapest cut is growing less wood that is later shed.

## Decisions for the host

1. Laterals leave from the parent's local direction, not the axis heading (see above).
2. The trunk is monopodial until the fork. Troll's module stacking is not modelled, which is the likely cause of the conifer-like young tree.
3. Reiterates are four PAs (leader, limb, bough, spur) rather than one PA whose physiological age drifts.
4. The codominant-fork weight and the drawing floors live in the rendering example, not the engine.
5. Wood below the ground forced the branches' elevation above level (0.25 rad), where [M98] has the low A2 drooping.
