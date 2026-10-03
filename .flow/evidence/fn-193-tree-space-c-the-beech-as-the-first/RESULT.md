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

## Host verdict 1 (2026-10-04, overnight run): not confident; not sent to Astra

Viewed `raw/final/sheet-beech.png`. Better than every earlier probe: the 80-year tree at seed 1 bare has a clear bole, a fork near a quarter of the height into ascending limbs and a fine twig crown. Rejected on: (1) in-leaf crown, leaves drawn as pale stretched streaks in sheets, not a dense dark mass (Nettleden); (2) appearance, white bark and pale leaves, so the conversion to the pipeline tree evidently drops the preset's appearance rows; (3) girth about 1.4 to 1.7 m at the base against the sourced 1.3 m maximum; (4) young trees (10 and 20 years) a pole with rods, because Troll's module stacking with relays from the curvature zone is not modelled; (5) seed 7 a flat-topped umbrella, not a dome. Rejection count per trait: one each. Speed for F: about 85% of grown wood is shed, growth 19 to 33 times today's skeleton.

## Round 2, after host verdict 1 (worker)

The sheet is `raw/final2/sheet-beech.png` (ignored, on disk). It keeps the same layout and adds row 6: today's beech, rendered on this branch by `examples/headless` with the preset's own appearance. I viewed every still.

| Item | Change aimed at it | Result |
|---|---|---|
| 1. Leaves drew as streaks | **Cause found.** Almost every leaf came from the preset's synthetic short-shoot clusters: 8 level leaves fanned every 3 cm along all wood under 0.7 of the stem radius, held 20 cm off. On a tree whose fine wood is long and plagiotropic, that made combs. With them off, only 4,625 leaves remained at 40 years, because leaves on twig runs stand 5 cm apart (the twig internode). **Change:** the stills overlay four preset rows (`ROWS` in `space_beech.rs`). Short-shoot clusters and limb clumping are off. Leaves stand on the grown short shoots and on wood under 0.012 of the root radius, one every 6 mm, the beech's short-shoot internode. | Leaves now read as leaves, with no streaks. The crown is still not a dense dark mass like Nettleden: it is pale and has holes where the inner crown has shed its fine wood. Young trees carry very few leaves (385 at 10 years), because the bearing radius is a share of the root's. |
| 2. Bark white and leaves pale | **None needed in the conversion.** It already sets the preset's material. Row 6 shows today's beech, drawn with the preset's appearance, with the same white bark and pale leaves. The pinned baseline stills (row 3) show tan bark and saturated green because the probe that drew them (fn-190's `growth_law today`) never called `set_material`: that is the renderer's default look. | The two trees match in appearance on this branch. The darker look is not the preset's. Making the preset darker is a preset change for the host. |
| 3. Girth | Pipes scaled to 0.57 mm (1.07 mm for short shoots). Breast-height diameter is now measured at 1.3 m by `examples/beech`. | **1.03 m at seed 1, 1.20 m at seed 7,** within TSO's dbh of up to 1.3 m. |
| 4. Young trees: Troll modules | **Not done.** The existing settings cannot express it. A relay restarts its apex's growth-unit count, so a trunk built of relays never reaches the fork's lifespan. A relay also stands at the last node, not at a bud in the curvature zone on the upper side. Modelling it needs new engine mechanisms: an apex's age kept across relays, and a relay position along the module with epitony. That design is the host's, and it needs walk tests. The time box did not allow it. | 10 and 20 years still read as a conifer sapling. |
| 5. Seed 7 crown | Reiterate elevations raised: limbs 1.0 to 1.1 rad, boughs 0.8 to 0.95, spurs 0.6 to 0.75. | Seed 7 is rounder at the top and 20.7 × 21.5 m, though still broad and slightly flat. Seed 1 is unchanged in shape (an ovoid) and girth-thinner. |

**Trait reading, round 2, at 80 years:**
- **Fork:** reads at both seeds.
- **Ascending limbs:** read; they reach the edge at seed 7.
- **Clear bole:** reads, now slimmer.
- **Crown outline:** seed 1 ovoid; seed 7 near a dome, broad, with thin stragglers below the crown on the right.
- **Flat two-ranked systems:** still not readable at this framing.
- **In leaf:** real leaves, but too airy and pale for Nettleden.
- **Young trees:** fail. No change was aimed at them this round.

**Wood shed (for phase F).** At 80 cycles the closed form expects 5.84M phytomers grown. The tree keeps 0.56M (seed 1) to 0.86M (seed 7), so **85 to 90% of grown wood is shed**. By physiological age, short shoots are 4.13M (71%), long shoots 1.07M (18%) and branches 0.62M (11%); trunk and reiterates are under 20k. The cause is turnover by design: every branch system lives 10 + 5 + 3 cycles and is then shed whole, while the reiterates bearing new ones live 15 to 70. The engine grows every phytomer of every system that will be shed and decides shedding at the end. Growing less of what will be shed needs the shedding decision before growth: the subtree's lifespan is known from the PA's lifespan and viability, not from light, so this is a candidate for phase F.
