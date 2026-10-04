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

## Host verdict 2 (2026-10-04, overnight run): not yet confident; not sent to Astra

Viewed `raw/final2/sheet-beech.png`. At 80 years both seeds read as mature broadleaf trees: bole, fork, ascending limbs to a rounded crown, girth within the sourced 1.3 m; structurally nearer Rostock and Entzia than today's beech. Colour is the preset's (row 6 draws today's beech the same way), not this spec's to change. Still short: the in-leaf crown is airy rather than a dense mass (the streaks are fixed by the change aimed at them; density has had no change aimed at it yet, so this is not counted as a second rejection: a host judgement, recorded), and the young trees still read as conifer saplings (no change aimed at them this round). Decisions: build Troll's module mechanism (the trunk's growth age carried across relays; relays from a bud on the upper side of the curvature zone; walk-tested); leaf density from the sources (short shoots of 3 to 5 leaves carrying about 77% of the leaf area); preset colours unchanged, the owner's call.

## Round 3, after host verdict 2: BLOCKED (wrong-path stop on the young-tree trait)

**Built (engine, committed, all crate tests green including the walks):**
- **Growth units carry across relays.** A relay continues the units its axis has spent in its PA, so a stem of relays reaches its next PA when an unbroken stem would (`a_relay_carries_on_its_axis_growth_units`).
- **Lifespan before abortion.** An apex that has spent its lifespan moves on and does not also abort. The closed form counts relays by the units they carry: it is a memoised recursion over (age, PA, units spent). Two tests pass unchanged: the oracle (A) and the engine mean against the closed form with abortion and relays (B, `tests/settings.rs`).
- **`relay_at`** (neutral 1, the last node): where along the stopped axis the relay bud stands, interpolated within a span so that it moves by degree.
- **`epitony`** (neutral 0): how far the relay bud turns toward the parent's upper side.
- Both are refused by name outside 0 to 1, and both are walked. Neutral values leave every earlier structure unchanged.

**Tried on the beech** (`raw/v3b/module-beech.patch`; not committed, reversed):
- The trunk as Troll modules: abortion 0.4 and then 0.9 per growth unit, relay 1, `relay_at` 0.5, epitony 1, insertion 0.3, straightening 1, tropism toward 0.35 and then 0.6 rad.
- Short shoots of 3 to 5 nodes, and leaves only on the grown short shoots, one every 6 mm.

**Result: the tree collapses.**
- At 40 cycles, seed 1 grows 284,642 phytomers and draws them 0.4 m tall. Seed 7 is 1.4 m.
- At 20 cycles both draw under 3 m, as crooked horizontal arms with almost no spray (`raw/v3a`, `raw/v3b`; I viewed every still).

**Cause, measured** (`raw/v3b/chain.log`). Each relay's size is the presence of the stop draw that made it, and fn-192's presence window widens with the wood a draw decides. A trunk relay decides the whole crown above it, so its window is the widest there is (2 log-odds). Under a module-ending probability, most relays are therefore born partly grown, and the sizes multiply up the stem:

| Seed | Trunk-module vigours (first modules, then 1.0 mostly) | Base scale of the 12th module |
|---|---|---|
| 1 | 0.026, 0.171, 0.120, …, 0.244, … | 1.2e-4 |
| 7 | 0.442, 0.788, 0.281, …, 0.883, …, 0.252 | 2.2e-2 |

The crown above the fork is drawn at that scale.

This is a conflict between phase B's continuity mechanism and Troll's module stacking. A stem built from a chain of drawn stops cannot keep its size while every stop grows in by degree over a crown-sized window. It is a design decision for the host, and the wrong-path stop: the young-tree trait failed again after a change aimed at it. Possible directions (not tried; host's call):
1. **A relay of a stop that is near certain to relay is whole.** Its existence follows the stop, and since the relay replaces what stopped, the wood decided is the difference between the two continuations, not the crown.
2. **Module ends by a deterministic count of units within the PA**, with no draw, so nothing grows in.
3. **Keep modules for the young tree only**, as their own PA before the trunk's, with a short chain.

**Leaves (item B): not finished.** The three to five leaves a short shoot carries and their 77% share were wired into the attempt, but the collapse made the in-leaf stills meaningless, and the change was reversed with the beech values. The pipeline places leaves along bearing wood at a fixed spacing, not one per node. A sourced 77% share on short shoots therefore needs the leaves placed by the engine's own nodes, or a bearing rule per node. That is a design question beside the collapse.

The committed beech is round 2's, unchanged: its relays are off, so the engine changes leave it as it was.

## Host decision 3, the last attempt for C (2026-10-04, overnight run)

The young-tree change collapsed the tree before its look could be judged (relays born partly grown, sizes multiplying up the stem: the 12th module at 1.2e-4 of full size; `raw/v3b/chain.log`), so the trait was not judged on its merits; this is counted as an engine interaction with phase B, not the second rejection. One attempt, then C stops for the owner with no further exceptions: (1) a relay's draw decides the difference between the two continuations (stop and relay against continue), not the crown, so a near-certain relay is drawn whole; B's walks and A's oracle stay green; (2) leaves: foliage spacing set per short-shoot length so a short shoot carries 3 to 5 leaves in expectation; exact per-node leaves are a phase F contract. If the young beech then fails, or the tree collapses, C stops.

## Round 4, the host's last attempt: BLOCKED (the young beech still fails)

**Built (committed; all ten crate test binaries green, `raw/space-tests-r4.log`):**
- A stop's grow-in window is now scaled by what it decides: `stop_stake = 1 - relay` of the apex's expected wood. A relay carries on its axis's PA and growth units, so stopping and relaying is expected to grow what carrying on would. A near-certain relay is drawn whole.
- With no relay the stake is the whole wood, so every earlier tree is unchanged (the round-2 beech measures the same).
- A's oracle passes, and so do B's walks. The `relay` setting is walked on every PA up to 0.995, which covers the near-certain case.

**The module beech re-applied** (`raw/final3/module-beech-r4.patch`; reversed after the sheet, so the committed beech is round 2's):
- Short shoots of 3 to 5 nodes.
- Leaves only on the grown short shoots, spaced 6 mm, by the measured mean short-shoot length (19 to 33 mm, so 3 to 5 leaves a shoot in expectation).
- The tree no longer collapses: 9.3 to 9.6 m at 40 cycles, 17.9 to 19.0 m at 80.

**Sheet:** `raw/final3/sheet-beech.png`, in the same layout. I viewed every still.

| Trait | Reading |
|---|---|
| 10 years | Seed 1: a slightly crooked stem, with branches spreading from a few modules. Seed 7: a straight pole with rods. **Fails at seed 7.** |
| 20 years | Both seeds: a straight pole with near-horizontal rods, no visible module stacking and no horizontal spray. **Fails: the same trait as rounds 1 to 3.** |
| 40 and 80 years, bole | The bole is a stack of ring-shaped bulges, one per trunk module. Each relay is drawn as a codominant lateral beside its stopped head. **Regressed against round 2.** |
| 80 years, fork and limbs | No clear division at about 0.25 H any more. The crown is a broad, leaning fan of limbs 22 m wide. **Regressed against round 2.** |
| In leaf | 125k to 157k leaves on the short shoots alone: an open, sparse crown, not a dense mass. **Fails.** The sourced 3 to 5 leaves per short shoot give too few leaves for the shoots the engine keeps. |

**Measured costs:**
- At 80 cycles the module beech grows more than 5M phytomers and keeps 1.10M to 1.28M, against round 2's 0.61M to 0.90M.
- Growth takes 1.7 to 2.6 s.

By the host's rule this ends C for the night.

## Round 5, after the owner's review and host decisions (worker)

Starting point: round 2's beech, plus the relay-difference engine change (`57ab643f`).

**Changes, one per item:**

1. **A relay continues the axis** (`examples/space/tree.rs`). A relay from inside its parent now takes the parent's run on from its node: same girth path, stem role and terminal fate. The stopped module's head beyond the node turns aside as a lateral run of its own, so there is no codominant fork beside the tip. The engine is unchanged in this round, so its walks are as before: `relay`, `relay_at` and `epitony` are walked on every PA, and all crate tests are green (`raw/space-tests-r5.log`).
   - **The module beech is re-applied and tuned:** abortion 0.35, relay 1, `relay_at` 0.5, epitony 0.6, straightening 1, module tropism 0.6 toward 0.45 rad.
   - **Branch systems:** long shoots 0.8 on Z23. Z22 stays at 0.08, because 0.15 is supercritical and overran a 20M budget at 80 cycles.
   - **Branch rise:** branches rise to 0.35 rad and their wander is 0.8, because low branches otherwise reached the ground.
2. **Darker materials** (`presets/european-beech.values`, owner-approved).
   - **Bark:** 0.175/0.155/0.125, from fn-62's round-3 candidate (`.flow/evidence/fn-180-…/beech-candidate.json` on the fn-180 branch; the darker grey the owner remembered).
   - **Leaves:** back 0.1/0.195/0.055 from the same candidate, front 0.055/0.1/0.033.
   - Roughness stays 0.95 (owner, against plastic).
   - The preset identity pins in `tests/catalogue_identity.rs` are re-pinned for the changed rows.
3. **Leaf density.** Leaves stand on all wood under 0.012 of the root radius, every 6 mm, the short-shoot internode. The mean grown short shoot is 19 to 29 mm, so 3 to 5 leaves a short shoot (Dupré: 3 to 5 internodes; Letort: 3). Limb clumping and the preset's synthetic clusters are off. Result: 1.8M to 2.6M leaves at 80 cycles.

**Sheets:** `raw/final4/sheet-beech.png`, in the same layout; row 6 is today's beech with the darker preset. Close-ups are in `raw/final4/close-ups.png`: the trunk base and one limb at 80 cycles, beside S1 and S3. I viewed every still.

| Trait | Seed 1 | Seed 7 | Reading |
|---|---|---|---|
| 10 years | Crooked stacked stems, leaning and arching | Crooked, with layered laterals | **Reads as a young broadleaf** with Troll's crooked stem; seed 1 leans too far |
| 20 years | One bowed, arching stem with spray along it | A straight pole with tiers of rods | **Mixed**: seed 1 crooked but over-leaning; seed 7 still fails |
| 40 years | A low, leaning bole with a sprawling crown | Erect, forking, an open crown | Seed 7 reads; seed 1 does not |
| 80 years: fork, limbs, outline | The bole leans and kinks at a module junction; a broad two-lobed crown | An erect, stout bole dividing at about 0.3 H into ascending limbs; a domed crown | **Seed 7 keeps round 2's reading. Seed 1 lost it:** its young lean persists into the mature bole |
| In leaf (80) | Dense, darker green, two lobes | Dense, darker, domed | **Much closer to a mass**, but lighter and more textured than Nettleden's opaque dark crown |
| Bark (close-up) | Grey-brown, with white flecks and a dark seam where a module joins | Grey-brown, white flecks, faint horizontal banding at the base | **Darker, but not S3's smooth grey-green.** The flecks are the preset's mottle and lichen rows, and the seam is the module junction in the conversion. |
| Today's beech, darker preset | Leaves darker | | At whole-tree distance the bark still reads pale |

**Costs at 80 cycles:**
- Kept: 0.85M to 1.18M phytomers.
- Growth: 1.9 to 2.9 s.
- Dressing (wood and leaves): 1.0 to 1.3 s.

**Core tests:** `cargo test -p telperion-core` (`raw/core-tests-r5.log`) failed on three tests:
- **Two identity digests**, from the preset change. They are re-pinned and pass.
- **`fixed_beeches_pass_…`:** peak RSS of 4.38 GB against a 4.31 GB ceiling during the parallel run. Re-run alone, it passes in 14.7 s. I read it as load-dependent, not caused by the colour rows (a guess).

**For the host:**
- Seed 1's persistent lean, from the first module's tilt, is the remaining structural fault.
- The seam at module joins and the bark flecks are what the close-ups show.

## Round 6, after the host verdict on final4 (worker)

**Changes, one per fault:**

1. **Secondary erection** (engine). The new per-PA setting `erection` (neutral 0) straightens an axis's base towards the vertical by `1 - exp(-erection × the axis's age)`, on top of its straightening. It applies to the seed too, which limits the seedling's tilt. Beech trunk: 0.1 per cycle.
2. **Module hazard** (engine). The new per-PA setting `abortion_rise` (neutral 0) makes the abortion probability after an axis's kth unit `1 - (1 - abortion)^(k^rise)`, so modules end at a regular length. The closed form counts it. Beech trunk: abortion 0.1, rise 2, which gives 0.10, 0.34, 0.61 and 0.81 after units 1 to 4.
   - Both settings are refused by name and walked on every PA.
   - The closed-form mean test now runs with a rising hazard.
   - New tests: `erection_lifts_an_axis_base_with_its_years` and `the_abortion_hazard_rises_with_an_axis_units`.
   - All crate tests are green (`raw/space-tests-r6.log`).
3. **Seam at module joins** (conversion). A relay now leaves from the node below its span, so the stem runs through the relay's base without folding back from the span's top. The module head beyond becomes the lateral.
4. **Colours from the photographs** (preset; identity re-pinned):
   - **Bark:** S3's mean linear colour over every pixel greener than blue, 0.091/0.086/0.034.
   - **Leaves:** both faces at Nettleden's mean linear colour over a 500 px crop inside the crown, 0.062/0.069/0.032.
   - **Flecks turned down:** lichen coverage 0.5 to 0.1, lichen strength 1.0 to 0.3, lenticel strength 0.25 to 0.1, bark mottle 0.1 to 0.04.

**Sheets:** `raw/final5/sheet-beech.png` (same layout; row 6 is today's beech in the sampled colours) and `raw/final5/close-ups.png` (S1 and S3 beside the trunk base and a limb at 80 cycles). I viewed every still.

| Trait | Seed 1 | Seed 7 | Reading |
|---|---|---|---|
| Lean (fault 1) | **Fixed**: the trunk stands erect at 40 and 80 | Erect | Erection does what it should |
| 20 years (fault 2) | A young tree with a crooked crown on a short stem | **Still a straight pole with long rods** | The hazard regularised the modules, but seed 7's stem still reads as one pole |
| 80 years: fork, limbs, dome | A short bole, then a broad, rounded crown of many limbs | **Regressed**: a broad, flat-topped umbrella, not final4's erect fork under a dome | The hazard changed every draw's topology; seed 7's final4 tree is not kept |
| Seam (fault 3) | **Worse**: the trunk base carries stacked collar flanges, one per module join | Worse, the same | Each module head now leaves as a lateral nearly as thick as the stem, and the surface draws it as a flared socket. The fix made it worse. |
| Bark colour (fault 4) | Olive yellow-green under the renderer's sun | The same | The sampled photo colour is S3's bark lit by shade and sky; under the renderer's sun it reads olive, not S3's grey-green |
| Leaf colour | Bluish grey-green, a lighter mass than Nettleden | The same | Albedos this dark leave the sheen and sky term dominant. The colour sampled from the photograph is not the albedo the renderer needs. |
| Smooth limbs (close-up) | Smooth, sparse flecks | Smooth | The flecks are gone; the limbs are smooth, but olive |

**Costs at 80 cycles:**
- Kept: 0.74M phytomers (seed 1) and 1.83M (seed 7).
- Growth: 2.1 s and 4.1 s.
- Leaves: 1.3M and 4.2M.

**For the host:**
- The collar flanges come from the conversion's module heads. Final4's attachment had a seam; this one has flanges. A head as thick as the stem would want the codominant fork treatment, or to be drawn thinner than the pipe model gives it.
- A photo colour includes the photo's light. Setting albedo from it needs the renderer's own light divided out, or a still matched against the photo, which is what fn-40 did for the earlier rows.
- Seed 7's mature reading moved with the hazard: the topology changed, not just the geometry. The host decides whether to keep the hazard and retune, or to keep final4's flat abortion.

## Round 7: back to final4, with render-matched colours (worker)

**Base:** final4's values and relay attachment. `abortion_rise` is back to 0, and the setting stays in the engine at neutral. The conversion's relay-from-the-node-below is reverted. Seed 7 at 80 cycles has final4's exact topology (1,182,568 nodes).

**1. Erection on the beech: tried, set to 0.**
- `erection` 0.1 makes seed 1's 80-cycle trunk erect (`raw/match-m4`).
- But it also bends seed 7's trunk into a lean, and 0.03 does too (`raw/match-m5`). On the beech it acts only on the seed axis (the relays already straighten fully), and moving the seedling's direction moves every relay frame above it.
- Seed 7's final4 reading was the acceptance priority, so the beech ships with `erection` 0. The setting stays in the engine.
- **Seed 1's lean is not fixed this round.** Whether to apply erection only to some modules, or to limit the seedling's tilt another way, is the host's call.

**2. Seam at module joints** (`examples/space/tree.rs`; no branch moves):
- A module head that turns aside now parts as a codominant fork when it is nearly as thick as the stem, so it draws no socket flange.
- A relay's first internodes that still lie behind its joint along the stem merge into the joint, so the stem cannot fold back.
- **Result:** seed 7's trunk base is smooth. **Seed 1 still shows one dark slit** at a module joint on its leaning lower trunk. The fold there is not behind the joint, and the cause is not found.
- A span split at the relay's true base was also tried (`raw/match-m0` and `raw/match-m2`). It put a collar and then a flange on seed 7's base, so it was reverted.

**3. Colours by render matching** (fn-40's method), on the 80-cycle seed-7 stills under the stills' sun and sky:

| Target | Photograph's mean linear colour | Rendered, final rows | Preset rows |
|---|---|---|---|
| Bark: trunk base crop vs. S3's limb bark (pixels greener than blue) | 0.091 / 0.086 / 0.034 | 0.089 / 0.086 / 0.035 | 0.040 / 0.040 / 0.0165 |
| Leaves: crown crop vs. a crop inside Nettleden's crown | 0.062 / 0.069 / 0.032 | 0.064 / 0.075 / 0.042 | front 0.021 / 0.0158 / 0.0067, back 0.037 / 0.0307 / 0.0114 |

- Three rounds of per-channel scaling got there (`raw/match-m6` to `raw/match-m8`).
- The flecks stay turned down as in round 6.
- The preset identity is re-pinned, and all crate tests are green (`raw/space-tests-r7.log`).

**Sheet:** `raw/final6/sheet-beech.png` is in the same layout; row 6 is today's beech in the matched colours. `raw/final6/close-ups.png` holds the close-ups. I viewed every still.

| Trait | Seed 1 | Seed 7 | Acceptance |
|---|---|---|---|
| 80 cycles: fork, limbs, dome | Leaning, kinked lower trunk, two-lobed crown (final4's seed 1) | Erect stout trunk, division at about 0.3 H, rising limbs, dome (final4's seed 7) | Seed 7 **met**; seed 1 **not met** (erect) |
| Seams and collars | One dark slit at a module joint on the lower trunk | Smooth; faint horizontal bands only | Seed 7 **met**; seed 1 **not met** |
| Bark colour | Dark olive grey-green, smooth, sparse flecks | The same | **Met against S3's mean**. It reads browner-olive than S3 reads to the eye, because S3's own mean is olive. |
| Leaf colour | Dark green mass, close to Nettleden in tone | Dark green dome | **Met**. Seed 7 in leaf is the closest yet to Nettleden. |
| 20 cycles, seed 7 (diagnostic) | — | A straight pole with tiers, recorded and not chased | — |

## Round 8: an erect trunk at every seed, judged over five seeds (worker)

**1. Limited tilt and erection.**
- The trunk's module tip now bends towards 0.9 rad rather than 0.45, so each module tilts only part way to level.
- `erection` is 0.1 per cycle on the trunk, which straightens the seedling's base and each module's base over the years.
- Seed 7's topology is unchanged (1,182,568 nodes); its geometry changes.

**2. Seed 1's dark slit: found and fixed.**
- **Where:** a dump of the lower stem's segments (a temporary debug pass, since removed) located it at node 70, 2.09 m up. There the relay's first segment left the joint at 77° to the parent module's last segment (cos 0.224). The segment was 5 cm long on a trunk of 0.70 m radius, so the swept rings intersected and drew a dark fold.
- **Not the cause:** there was no zero-length segment and no radius jump; the start radius equalled the parent's 0.7035 m.
- **Fix:** with the module tips held steeper, no stem segment below 8 m turns more than 37° (cos < 0.8) at any of the five seeds.
- **Result:** seed 1's trunk base is smooth in the close-up.

**3. Five seeds at 80 cycles** (`raw/final7/five-seeds.png`, bare and in leaf, beside S1, S2 and S3). The extra seeds are 2, 3 and 4: the next ones in order, not chosen by look. I viewed every still.

| Seed | Bare | In leaf | Reads as an open-grown beech? |
|---|---|---|---|
| 1 | Erect trunk with a slight lean, forking at about 0.3 H into rising limbs; broad crown with one tall tuft | Dark green broad crown | **Yes** |
| 7 | Trunk leaning about 15°, forking; domed crown | Dark green dome | **Yes**. It leans more than in final6; it still reads as a beech. |
| 2 | Forked trunk with two rising limb systems; two-lobed crown | Two-lobed dark green crown | **Yes**, borderline: the two lobes are less domed than the photographs |
| 3 | A 6.5 m bare pole carrying a narrow broom | A narrow column | **No**. The crown above the fork is drawn at 0.231 of its size (the trunk's continuation, `raw/r8a`) |
| 4 | Erect trunk forking low into many rising limbs; broad dome | Broad dark green dome | **Yes**, the closest to S1 Entzia |

**Four of five seeds read as an open-grown beech.**

**Seed 3's failure is in the continuity mechanism, not the values:**
- Its first relay grew one long module of 11 units. One of that module's abortion draws fell within the 0.05 log-odds floor of its bound.
- So the module's persistence presence, and with it the fork and the whole crown carried on it, stands at 0.231.
- A stop that relays decides no wood (its stake is zero), yet the window floor still grows it in.
- Whether a zero stake should mean no grow-in (presence 1) is the host's design call. It would make a walk of abortion at relay 1 step, not slide.

**Sheet** `raw/final7/sheet-beech.png` (seeds 1 and 7, same layout) and `raw/final7/close-ups.png`:
- The trunk bases are smooth; seed 7 shows one faint horizontal band at the flare.
- The limbs are smooth olive grey-green with sparse flecks.
- The young ages are diagnostic only: seed 7 at 20 cycles is a pole with tiers, while seed 1 at 10 and 20 cycles is crooked, with layered spray.

**Tests:** all crate tests are green (`raw/space-tests-r8.log`).

## Round 9: the grow-in floor scales with the wood decided (worker)

**Engine change** (`presence.rs`, `grow.rs`).
- A stop's presence now takes the share of the apex's expected wood it decides (`1 - relay`). The window and its floor both scale with that share: `window = clamp(RATE × wood × decided, FLOOR × decided, SPAN)`, and a draw whose window is zero is whole.
- A stop that is always relayed therefore has no grow-in. With no relay (`decided` 1) every window is as before.
- All crate tests are green (`raw/space-tests-r9.log`): A's oracle, B's walks, and the closed-form and settings checks.

**The relay = 1 crossing, walked** (`raw/relay1-walk.log`; trunk abortion walked over 0.005 to 0.5 with relay 1, at seeds 1 to 3):
- **The walk shows a jump.** Seed 2 reaches slope 37 and seed 3 reaches 33, against the bound of 30.
- The steepest step does not shrink when split finer. At seed 2 it swaps 0.98 of the tree's wood, made or unmade, unchanged over three levels of refinement. At seed 3 it swaps 0.87; at seed 1 it is a 0.067 move.
- **The geometry it switches is not identical.** A relay is a new lineage, so its draws, and the crown it grows, differ from those of the apex it replaces. Its base also stands at `relay_at` with its own insertion, not at the tip on the parent's line.
- By the host's rule this jump is therefore not acceptable as it stands. I did not add the walk as a passing test.
- **Making it identical (host's call):** a relay that inherits its parent's lineage key, stands at the tip and has the parent's heading would switch nothing. The beech's relays (`relay_at` 0.5, insertion 0.3) are different trees by design.

**Five seeds at 80 cycles** (`raw/final8/five-seeds.png`). I viewed every still. Seeds 1, 7, 2 and 4 are byte-identical in structure to final7 (same node counts); only seed 3 changed.

| Seed | Reading |
|---|---|
| 1 | Erect trunk with a slight lean, fork, rising limbs, broad crown with a tuft: **a beech** |
| 7 | Leaning trunk, fork, dome: **a beech** |
| 2 | Fork, two-lobed crown: **a beech, borderline** |
| 3 | **Now full size**: 20.4 m, 23 × 25 m wide, a stout erect bole forking into rising limbs, a rounded crown with a few long low stragglers on one side. **A beech**, the bole thicker and the crown sparser at the sides than the photographs |
| 4 | Erect trunk, low fork into many limbs, broad dome: **a beech**, closest to Entzia |

**Five of five seeds read as an open-grown beech at 80 cycles; seed 2 is borderline.**

**Final8 sheet and close-ups** (`raw/final8/sheet-beech.png`, `raw/final8/close-ups.png`) are unchanged from final7 for seeds 1 and 7: smooth trunk bases, one faint band at seed 7's flare, and smooth olive grey-green limbs.

**Cost.** Seed 3 grows in 15.7 s at 80 cycles, and draws 1.89M nodes and 4.9M leaves, against 1.6 to 2.9 s for the others. It is the heaviest tree, because its whole crown now grows at full presence.

## Round 10: relays continue their axis, and the relay = 1 crossing walks by degree (worker)

**Engine changes:**
- **A relay inherits its parent axis's lineage** and the growth units it spent. Its growth units therefore draw what the apex's would have, and the crown above keeps its identities across a crossing.
  - The relay draw is keyed to the growth unit the stop happened in, not to the lineage, so successive relays of one axis draw independently.
  - A unit that failed its viability draw counts as spent (in the closed form too), so the relay's first unit draws anew.
- **A relay moves; it does not grow in.** Its `blend` (0 to 1, the stop's lead over the widest window, `SPAN`) places it between the continuation it replaces and its own bud:
  - position, heading and side are interpolated;
  - straightening and erection are scaled by the blend;
  - its section enters the parent at its node by the blend and at the tip by the rest.
- **The straightening fade runs over an axis's reach:** its own scaled length plus each relay's reach weighted by `1 - blend`. A relay that has barely left the line therefore leaves the axis's straightening as it was.
- **`relay_at` is now a share of the stopped axis's last growth unit**, the module's curvature zone. As a share of the whole axis, the next relay jumped whenever an earlier near-zero relay split the axis.
- **Epitony turns the bud through the smaller angle,** with the turn fading as the bud faces straight down, the case where the sign of the angle flips. This removes a jump the walk found at `states[1].epitony` = 1.
- **The walk measure** counts a relay with the axis it continues: one branch from the axis's base to the relay's tip.

**Walk test.** It adds "states[0].abortion, relay 1": the trunk's module-ending chance walked from 0.005 to 0.5 at seeds 1 to 3, with Troll's relay settings (`relay_at` 0.5, epitony 0.6, insertion 0.3, straightening 1).
- **It passes: worst slope 2.82** against the bound of 30, and every steep step shrinks when split.
- Before the blend window was set to `SPAN`, one step was steep but continuous: bisected to 5e-14 it still changed linearly. It came from the floor window on late stops.
- All crate tests are green (`raw/space-tests-r10.log`), the oracle among them. The relay-position test now checks the middle of the last growth unit.

**Five seeds at 80 cycles** (`raw/final9/five-seeds.png`). Every seed changed, because the relays' draws now continue their axis's. I viewed every still.

| Seed | Reading |
|---|---|
| 1 | A narrow tree (16.8 m wide, 21 m tall) on a leaning trunk; a small ovoid crown. **Not a beech**: it reads as a narrow broadleaf. |
| 7 | Erect stout trunk, fork, broad crown 18 to 21 m: **a beech**. A collar ring has come back at the trunk base (close-up). |
| 2 | Erect trunk, a fan of upright limbs, a two-lobed crown: **borderline**. It reads vase-shaped, with rings at the base. |
| 3 | Erect trunk under a broad spreading dome, 22 m: **a beech** |
| 4 | Erect trunk, a two-lobed V-shaped crown with a ring at the trunk: **borderline** |

**Two of five seeds read as an open-grown beech, two are borderline and one fails.** The continuity fix moved every tree, and the mature reading is worse than round 9's five of five.
- **Relays near the tip:** with `relay_at` now inside the last growth unit, the beech's relays stand near the module tip rather than halfway down the module.
- **Narrow crowns:** relays that continue their axis's draws make long modules, and long modules make narrow, upright crowns.
- The beech's values were not retuned this round.
- The collar rings are the conversion drawing a module head that turns aside next to a relay standing near the tip.

**Sheets:** `raw/final9/sheet-beech.png` (seeds 1 and 7, same layout) and `raw/final9/close-ups.png`. Seed 1's limb close-up frames sky, because its crown is narrow. The young trees are diagnostic only: seed 7 at 20 cycles is still a pole.

## Host decision 10 (2026-10-04)

The engine is continuous across relays (round 10; worst relay walk slope 2.82 against 30). The beech look regressed because relay_at now measures within the last growth unit. Round 11 is values only on this engine: shorter trunk modules, relays low in the curvature zone, limb spread retuned, collars at module joints blended by the conversion. Target: four of five seeds (1, 7, 2, 3, 4) read as an open-grown beech at 80 years, as in round 9; then Astra judges the same sheet.

## Round 11: values only on the continuous engine (worker)

**Beech values** (`beech.rs`; the engine is untouched):

| Trunk setting | Round 10 | Round 11 | Why |
|---|--:|--:|---|
| `relay_at` | 0.5 | 0.15 | The relay bud stands low in the curvature zone, the module's last growth unit ([M98]) |
| `abortion` | 0.35 | 0.5 | Shorter modules. A larger lead past the bound also moves each relay further from the line it continues (a larger `blend`) |
| module tip `elevation` | 0.9 rad | 1.4 rad | Seed 1's lean. A relay with a small `blend` keeps the tilted tip's heading and straightens only by its blend, so each module added tilt and the stem leaned. At 1.4 every seed stands erect |

- **Topology did not move.** At relay 1 a module's end only moves geometry, so the node counts match round 10 to within a few nodes merged by the conversion.
- **Limb spread** was tried in two steps and reverted:
  - Every reiterate elevation 0.1 rad lower changed almost nothing.
  - Lower again (leader 1.05, limb 0.85, bough 0.75, spur 0.6) flattened seeds 3 and 4 into umbrellas.
  - So the reiterates keep round 10's elevations.

**Collars at module joints** (conversion, `examples/space/tree.rs`). A dump of the lower stem found three causes, fixed one by one, each viewed at the trunk base of every seed:
1. **A relay's run starts at the parent node nearest the relay's own base**, not always at its bud's node. With a small `blend` the relay stands near the module tip, so the stem used to run up from the bud's node beside a co-linear head of 0.6 to 0.9 of its radius, drawn as a codominant fork. That drew the flanges.
2. **The stem from the bud's node to that joint is drawn as thick as the bud's node.** Those nodes' pipe radius leaves out the relay (0.70 m against 0.78 m at seed 2), which drew a neck and a step at every joint.
3. **The relay's first internodes still inside the wood of the joint merge into it**, and a head that lies wholly inside that wood is drawn at no more than `FORK_FROM` of the stem's radius. A 1.7 cm, 37° first segment on a 0.79 m trunk had folded into a ring at seed 7 (0.7 m up).

**Result:** no collar, flange or step at any of the five trunk bases (`raw/final10/bases.png`). A faint dark seam line remains at some joints (seeds 7, 2, 3, 4), and seed 3 shows a slight kink at one joint.

**Five seeds at 80 years** (`raw/final10/five-seeds.png`, bare and in leaf beside S1, S2 and S3). I viewed every still.

| Seed | Bare | In leaf | Reads as an open-grown beech? |
|---|---|---|---|
| 1 | Erect trunk, forking at about 0.35 H into two rising limb systems; a rounded crown set to one side, 17 m wide | Dark, rounded dome, lopsided | **Yes**. It is narrower and more one-sided than the photographs |
| 7 | Erect stout trunk; an irregular crown with a narrow, upright, broom-like limb top left and thin, zigzag laterals low at the sides | Ragged, with tufts at the top | **Borderline / no**. It reads as a broadleaf, not clearly a beech |
| 2 | Erect trunk, a fan of rising limbs, a broad dome | Dense, dark dome | **Yes**, the closest to Nettleden |
| 3 | Erect trunk, rising limbs, a broad dome with a lobe to the left; slightly flat-topped | Broad dark dome | **Yes** |
| 4 | Erect trunk, a low fork into many limbs; the bare crown has a shallow notch at the top | Broad dome | **Yes**. The bare notch is the weak point |

**Four of five read as an open-grown beech (1, 2, 3, 4); seed 7 is borderline.** Against round 10, the leans and the collar rings are gone at every seed.

**Sheet** `raw/final10/sheet-beech.png` (same layout; row 6 is today's beech from `raw/final9/today`, whose preset is unchanged) and `raw/final10/close-ups.png` (all five trunk bases and limbs). The young ages are diagnostic only. Seed 1 and seed 7 at 10 and 20 years are still straight poles with near-horizontal rods, and at 40 years a pole under a small crown.

**Tests and costs:**
- All crate tests are green (`raw/space-tests-r11.log`): the oracle, every walk including the relay walk, and the beech at every sheet age.
- At 80 years: growth takes 1.5 to 3.3 s and dressing 0.9 to 1.5 s, with 0.81M to 1.30M nodes, the same as round 10.

**For the host:**
- The module tips now barely bend (elevation 1.4), so Troll's module curvature shows little. That is the price of an erect stem while a relay of small `blend` keeps the tip's tilt. A relay whose straightening does not scale with its blend would let the tips bend again, but that is an engine design question.
- Seed 7's upright broom is a long, steep limb with dense shoots at its top. Of the two limb-elevation steps tried, neither removed it without flattening other seeds.
- `cargo clippy -p telperion-space -- -D warnings` fails on `geometry.rs:36` (`needless_range_loop`). It came in with round 10's engine change, and the engine is outside this round's scope.

## Gate, round 11 (2026-10-04)

Host: four of five seeds read as a beech (2, 3, 4 strongly; 1 narrower; 7 borderline). Astra, independently on the same sheet: FAIL (1 and 2 borderline, 7, 3, 4 no): "crowded ascending limbs produce fans and vases instead of a broad dome supported by substantial, outward-spreading branches" (`ASTRA-VERDICT-R11.md`). The host agrees on re-reading Rostock and Entzia: the host was too lenient. Host decision 11: fewer, stronger limbs (after the fork one to three reiterates dominate; Millet: the crown is a succession of reiterates, the most peripheral the shortest), limbs that run outward then up (elevation by physiological age, low near the limb base and rising toward its tip), and less fine brush so the large limbs show.

## Round 12: fewer, stronger limbs that run out then up, less brush (worker)

**Engine:** no new mechanism was needed. Each of the host's three items is expressible in the beech's values: physiological ages (PAs), their zones and `next`. The one engine edit is the clippy fix at `geometry.rs:36`: the placement loop now iterates the reaches. `cargo clippy -p telperion-space --all-targets -- -D warnings` is clean.

**Beech values** (`beech.rs`, `SOURCES.md` rows updated):
1. **Fewer, stronger limbs.**
   - The fork's top node bears a limb (0.9). The two nodes below bear a limb at 0.3 or a weaker, shorter bough at 0.5.
   - The leader bears limbs at only 0.06 a growth unit and boughs at 0.2. Round 11's leader bore 0.1 full limbs a unit, about seven 70-year limbs, which drew the crowded fan.
   - Limbs bear boughs (0.15), boughs bear spurs (0.15), and spurs bear none: the most peripheral the shortest.
2. **Limbs run out, then up**, by physiological age. A limb is two PAs:
   - **Out:** 15 growth units bending toward 0.55 rad (tropism 0.6, straightening 0.1).
   - **Up:** 55 units bending toward 1.15 rad.
   - A bough is likewise 8 units toward 0.5 rad, then 22 toward 1.0. A spur keeps one PA, toward 0.75.
   - That is two PAs more on the reference axis (11 in all).
3. **Less fine brush.**
   - Branch-system laterals on reiterates: 0.6 → 0.45.
   - Long shoots on a branch system: 0.8 → 0.65.
   - Branch internodes 4 → 3 cm; long-shoot internodes 3.5 → 2.5 cm.
   - A first, larger cut (also short shoots 0.6 → 0.5) left the crown near empty, and was undone.

**Tried and reverted:**
- Limb and leader lifespans of 40 and 55 changed nothing visible.
- Lower out-phase elevations (0.45 and 0.35) gave flat umbrellas.

**Five seeds at 80 years** (`raw/final11/five-seeds.png`; limbs and trunk bases in `raw/final11/limbs.png` and `raw/final11/bases.png`). I viewed every still. Each reading is against Astra's critique:

| Seed | Reading | Beech? |
|---|---|---|
| 1 | Erect bole, fork at about 0.3 H into a few substantial limbs that leave outward and rise; rounded crown, 19.5 × 13.4 m. In leaf, a rounded but open mass. Astra's "crown undersized above the long trunk" still holds | **Borderline** |
| 7 | Two or three heavy limbs run outward, then up, under a broad crown 20 m wide. The fan of crowded upright stems is gone, but one upright, flag-like limb top left and streaky foliage tiers remain | **Borderline**, much nearer than round 11 |
| 2 | Substantial spreading limbs carry a broad, rounded dome; the brush no longer hides the limbs | **Yes** |
| 3 | Two limbs in a V, with a thin leader between them and a long bare limb running out low to the left. The central hollow remains: the leader of this seed is weak and one limb dominates | **No** |
| 4 | A low fork into several substantial limbs that spread outward under a broad dome, close to Entzia. One comb-like flag of brush stands at the right | **Yes** |

**Two clear (2, 4), two borderline (1, 7), one no (3).** The large limbs read at every seed. Astra's "fans and vases" reading is gone at 2, 4 and 7, and persists at 3.

**The sheet** `raw/final11/sheet-beech.png` keeps the same layout; row 6 is today's beech from `raw/final9/today`. Trunk bases are smooth, with faint seams. The boles are slimmer than round 11's, because less wood is kept; breast-height diameter was not measured this round. The young ages are diagnostic only: 10 and 20 years are still poles with rods.

**Costs at 80 years:**
- Growth: 0.4 to 1.4 s.
- Dressing: 0.3 to 0.6 s.
- 0.34M to 0.66M nodes and 0.52M to 1.09M leaves, about 40% less than round 11.

**Tests:** all crate tests are green (`raw/space-tests-r12.log`): the oracle, every walk including the relay walk, and the beech at every sheet age.

**For the host:**
- Seed 3's hollow comes from a weak leader. Its continuation from the fork is drawn at 1 mm, a near-bound draw at its birth, so one limb dominates. A share of vigour among the fork's sibling reiterates, the engine setting the host named, would govern this directly. It was not built, because the values reached the other seeds.
- The comb-like flags at seeds 4 and 7 are branch systems on one plane along a steep limb tip.

## Round 13: roll, the leader's survival, a denser leaf mass (worker)

**1. The fork's sibling vigour share: not built. The cause it was aimed at is not the one measured.**
- A temporary dump of the draws behind seed 3's leader shows a whole first growth unit (presence 1, birth 1). The apex then failed its viability draw (0.995 a unit) at the second unit, well past the bound. So the leader is a 6-node stub because its apex died in its second year. It is not a near-bound birth draw drawn small, so a share of vigour among the fork's siblings would not reach it.
- **Values fix, applied:** the leader's viability is 1, so the stem's own relay at the fork does not die. Seed 3 now carries a leader in the middle of its crown, and its hollow is gone. The other seeds' leaders were alive, so they move only through the other changes this round.
- **For the host:** whether the vigour-share setting is still wanted, for a different case, is the host's call.

**2. `form.roll` (engine; neutral 0, walked, refused by name).**
- Each lateral of a PA turns about its parent by up to `roll` radians either way, by a draw its lineage keys (`lineage::ROLL`).
- **Tests:**
  - `roll_turns_each_lateral_about_its_parent`: most limbs turn, the same draws give the same tree, and no other tree grows.
  - The walk test walks `form.roll` over 0 to π on every PA.
  - Refusal: `states[1].form.roll`.
- **Beech:** branch systems roll by up to 0.7 rad. The comb-like flags of seeds 4 and 7 are broken up.

**3. A denser leaf mass.**
- **Short shoots live 6 growth units at viability 0.95**, against 3 at 0.9 before (estimated: [DTT86] gives short shoots that persist; no lifespan is sourced). This doubled the leaves while the bare tree kept its limbs.
- **Short-shoot sections:** each phytomer's pipe is 0.6 mm, against 1.07 mm before, so the extra short-shoot wood does not swell the bole: the dbh is 1.09 m at seed 1. With the old pipe, a trial drew a bole well over 1.3 m.
- **Short-shoot probability:** 0.7 on branch systems (the top of [LET]'s A24 range) and 0.6 on shoots, as sourced. Short shoots on reiterates: 0.3 → 0.5 (estimated). A trial at 0.85, above the sourced range, was undone: it added only 15% of the leaves.
- **The stills' leaf-bearing radius** (`ROWS` in `space_beech.rs`) is 0.018 of the root radius, against 0.012 before. Round 12's slimmer root had cut the bearing wood.
- **Leaves at 80 years:** 1.6M to 2.2M, against 0.5M to 1.1M in round 12.
- **Not sourced:** the mean grown short shoot is now 55 to 60 mm, so about 9 leaves a shoot over its life, against the sourced 3 to 5 a year. The pipeline places leaves along all bearing wood, not on the youngest growth unit only.

**Five seeds at 80 years** (`raw/final12/five-seeds.png`; `bases.png`, `limbs.png`, `sheet-beech.png` beside it). I viewed every still.

| Seed | Reading | Beech? |
|---|---|---|
| 1 | Erect bole, a fork into a few spreading limbs, a rounded crown that is narrow in depth (20 × 14 m). In leaf a rounded, darker mass, dappled at the edge | **Borderline / yes** |
| 7 | Heavy limbs spreading broad and low. In leaf a broad mass, but the leader rises as a narrow dark spire at the top left | **Borderline**: the spire |
| 2 | Substantial limbs under a broad, rounded dome. In leaf the nearest yet to Nettleden's mass | **Yes** |
| 3 | The leader now fills the middle: three limb systems under a broad crown with a lobe to the right. The hollow is gone | **Yes** |
| 4 | A low fork into spreading limbs under a broad dome; the flag at the right is much reduced | **Yes**, near Entzia |

**Three clear (2, 3, 4), two borderline (1, 7).**
- The bare trees keep their visible limbs.
- In leaf, the crowns are a darker, denser mass than round 12's. They are still more textured and see-through at the edge than Nettleden's solid crown.
- The trunk bases are smooth, with faint seams. The young ages are unchanged, a pole with rods.

**Tests:** all crate tests are green (`raw/space-tests-r13.log`), the oracle and every walk among them. `cargo clippy -p telperion-space --all-targets -- -D warnings` is clean.

**Costs at 80 years:**
- Growth: 1.4 to 2.6 s.
- Dressing: 0.9 to 1.2 s.
- 1.05M to 1.41M nodes.
- `examples/beech` measures seed 7's dbh as 0.00 m. It finds no stem segment across 1.3 m at that seed. That is a measuring gap in the example, not looked into this round.

## Gate, round 13 (2026-10-04)

Host: 2, 3, 4 read as a beech, 1 shallower, 7 borderline (spire); sent to Astra. Astra: FAIL (2 yes, 1 and 4 borderline, 7 and 3 no): "crown integration: too many seeds terminate their main limbs in visibly separate fans or tufts, rather than building the broad, connected dome" (`ASTRA-VERDICT-R13.md`). Progress from round 11 (no yes). Host decision 13: first, branch systems along each limb's whole length (less acrotony on limbs) with more mid-crown fill and even lateral spread; if the crown still does not integrate, the next round brings phase E's light pass forward so shoots fill gaps toward light.

## Owner on round 13 seed 2 (2026-10-04)

> "structure looks really good. BUt the texture of the branches looks very tube like to regular. But that should be out of scope for the growing part. So judgement seems good."

Recorded: the structure direction holds; the tube-like, regular branch surface is a rendering follow-up outside phase C (a candidate spec on irregular branch cross-sections and surface for the tree space, to be raised in the report).

## Round 14: crown integration, values only (worker)

**Engine:** unchanged. Every change is a beech value, and `form.roll` (round 13) was the one setting needed beyond them.

**Changes, each viewed on the five-seed sheet before the next:**
1. **Wood along each limb's whole length, and mid-crown fill.**
   - Limbs bear boughs at 0.2 a growth unit (0.15 before), and boughs bear spurs at 0.2 (0.15).
   - Boughs live 8 + 30 units (8 + 22) and spurs 25 (15), so the peripheral reiterates fill the crown between the limbs rather than ending as tufts at their tips.
   - Spurs bend toward 0.6 rad (0.75), so they fill sideways.
2. **Even lateral spread.** Reiterates roll about their bearer by up to 1.2 rad, so boughs and spurs stand all round a limb, not in its plane.
3. **The leader no longer rises above the dome.** It lives 45 units (70) and bends toward 1.1 rad (1.25). This removed seed 7's spire.
4. **Girth held.** Every pipe is about 12% thinner (0.5 mm, short shoots 0.53 mm). The extra wood had drawn a dbh of 1.74 m; it is now 1.01 m at seed 1.
5. **Short shoots back to their sourced life:** 3 units at viability 0.9. Round 13's 6 units, with the extra wood of this round, overran the beech test's 10M phytomer budget. The crown's density now comes from the added reiterates.

**Tried and reverted:** limb tips toward 1.25 rad and spurs toward 0.7 changed nothing visible.

**Five seeds at 80 years** (`raw/final13/five-seeds.png`; `sheet-beech.png`, `bases.png`, `limbs.png` beside it). I viewed every still. Each reading is against Astra's "separate fans or tufts":

| Seed | Bare | In leaf | Beech? |
|---|---|---|---|
| 1 | Erect bole, a fork into rising limbs, one broad spreading crown, 23 m wide | One broad, connected dome; dappled at the edge, two shallow lobes at the top | **Yes** |
| 7 | Limbs spread to a broad, even crown; the spire is gone | A broad dome with a shallow notch right of centre | **Yes** |
| 2 | Spreading limbs under a full, rounded crown | A connected, rounded dome | **Yes** |
| 3 | Three limb systems spreading under one crown, 22 m wide | A broad mass; a shallow notch between two lobes remains, much less deep than round 13's gap | **Yes / borderline** |
| 4 | A low fork into spreading limbs under a broad dome | A broad dome; a slight tuft at the top right | **Yes** |

**Four to five of five read as an open-grown beech. No seed ends its limbs in a separate fan.**
- The crowns are now broader than they are tall (about 19 m tall, 21 to 24 m wide): Entzia's shape rather than Nettleden's taller dome.
- In leaf they are a connected mass, still more textured at the edge than Nettleden.
- Trunk bases are smooth with faint seams. Young ages are unchanged: a pole with rods at 10 and 20 years; at 40, a pole under a spreading crown.

**Tests:** all crate tests are green (`raw/space-tests-r14.log`): the oracle, every walk, and the beech at every sheet age within the test's budget. Clippy is clean.

**Costs at 80 years:**
- Growth: 1.5 to 2.4 s.
- Dressing: 1.2 to 1.5 s.
- 1.05M to 1.27M nodes and 2.2M to 2.7M leaves.

## Gate, round 14 (2026-10-04)

Host: all five connected domes, confident; sent to Astra. Astra: FAIL (4 yes; 1, 7, 2 borderline; 3 no): "the consistently shallow, top-heavy crown: branches and foliage spread into upper fans without enough intermediate and lower volume to form the photographs' deep, broad dome" (`ASTRA-VERDICT-R14.md`). The fans and tufts are gone; the fault moved to depth (crowns about 19 m tall and 21 to 24 m wide). Host decision 14: a deeper dome: crown height at least its width, lower limbs and boughs living longer and carrying more so the crown reaches down from the fork, and mid-crown fill.

## Round 15: a deeper dome, values only (worker)

**Engine:** unchanged.

**Beech values** (each step viewed on the five-seed sheet):
1. **Taller.**
   - A limb's rising phase has 5 cm internodes (4 before) and bends toward 1.25 rad with tropism 0.4 (1.15, 0.25).
   - The leader lives 55 units (45) toward 1.2 rad with tropism 0.4.
   - Boughs rise toward 1.15 rad (1.0) and live 8 + 32 (8 + 30).
   - Spurs stand at 0.8 rad (0.6).
2. **The crown reaches down from the fork.** A limb's outward phase is 8 units (15), and its top node bears a bough (0.2) where it bore a branch system. So the lower limbs carry boughs from near the fork.
3. **Budget.** The extra reiterates took the beech test's 80-year grow past its 10M phytomer budget. Fewer fine laterals on reiterates bring it back inside: short shoots 0.3 (0.5) and branch systems 0.4 (0.45). The test's budget is unchanged.

**Tried and reverted:**
- Leader at 1.35 rad and limb tips at 1.3 rad with 5.5 cm internodes reached 23 to 25 m, but drew spires above the dome.
- Outward phases of 5 units barely narrowed the crown.

**Heights and widths at 80 years** (`raw/final14/run.log`):
- The bole is clear to the fork at about 4.5 to 5 m, about 0.2 to 0.22 H (read off the stills, not measured).
- So crown height is the tree's height less about 4.7 m.

| Seed | Height | Width (x × z) | Crown height ≈ | Crown height ≥ width? |
|---|--:|--:|--:|---|
| 1 | 22.8 m | 21.2 × 21.6 | 18 m | no (0.84) |
| 7 | 21.8 m | 19.5 × 21.6 | 17 m | no (0.80) |
| 2 | 22.6 m | 20.6 × 20.6 | 18 m | no (0.87) |
| 3 | 21.3 m | 18.3 × 23.0 | 16.5 m | no (0.72 on the wide axis) |
| 4 | 21.5 m | 20.6 × 20.8 | 17 m | no (0.82) |

**Against round 14:** the trees are 2.5 to 4 m taller and 1 to 3 m narrower. Crown depth over width rose from about 0.6 to 0.72–0.87. **The target, crown height at least its width, is not met.**
- Every further step tried toward it either drew spires above the dome or did not narrow the crown.
- The width is set by the boughs' and spurs' spread more than by the limbs. Narrowing it further without thinning the crown again is a design question for the host.

**Per-seed reading, against Astra's "shallow, top-heavy crown"** (`raw/final14/five-seeds.png`). I viewed every still.

| Seed | Reading | Beech? |
|---|---|---|
| 1 | A deeper, rounded crown reaching down to the fork; one tall lobe at the top right | **Yes / borderline** (the lobe) |
| 7 | A deep, broad, connected dome reaching low; the most Nettleden-like so far | **Yes** |
| 2 | A deep, rounded dome with a full lower crown; a small tuft at the top left | **Yes** |
| 3 | A broad crown, deeper than round 14, with the lower crown filled; two upper shoulders with a shallow dip between them | **Borderline** |
| 4 | A deep crown with fill low down; two pointed tips stand above the top | **Borderline** (the tips) |

- The lower and middle crown is fuller at every seed. The "upper fans" are gone.
- What remains are a few pointed tips at the top (seeds 1, 3, 4).
- Trunk bases are smooth, with faint seams; limbs read in the close-ups.
- Young ages are unchanged: a pole with rods at 10 and 20; at 40 a pole under a spreading crown.

**Girth and costs:**
- dbh 1.18 m at seed 1 (`examples/beech`), within 1.3 m.
- At 80 years: growth 2.2 to 3.5 s, dressing 1.5 to 2.2 s, 1.2M to 1.9M nodes, 2.6M to 3.7M leaves.

**Tests:** all crate tests are green (`raw/space-tests-r15.log`): the oracle, every walk, and the beech at every sheet age within its unchanged 10M budget. Clippy is clean.

## Gate, round 15, and host decision 15 (2026-10-04)

Astra: FAIL (1, 7, 2, 3 borderline; 4 no): "the rising limbs consistently produce an upright fan with separate peaks instead of the broad, integrated dome" (`ASTRA-VERDICT-R15.md`). Rounds 13 to 15 swung between flat fans and upright fans, so the limb design is wrong, not its values. The source says how: in *Fagus* "trunk and limbs are stacks of plagiotropic modules that straighten at the base" (LITERATURE.md catalogue table; Millet; CIRAD). Each limb rises at its base and arches over at its tip, relaying from the bend, and the arching tips make the integrated dome. Host decision 15: Troll's module mechanism (built for the trunk) applies to limbs and boughs, replacing the out-then-up two-phase limb: erect base, arching plagiotropic tip, relay from the upper side of the bend.

## Round 16: limbs and boughs as Troll module stacks (worker)

**Engine:** unchanged. The trunk's module mechanism (abortion with relay, `relay_at`, epitony, straightening) applies to any physiological age (PA), so the change is in the beech's values only.

**Beech** (`beech.rs`, starting from round 14):
- **The two-phase limb (out, then up) is gone.** `LIMB_UP` and `BOUGH_UP` are removed, so the reference axis is back to 9 PAs.
- **A limb and a bough are each one PA of plagiotropic modules:**
  - abortion 0.3 (limb) and 0.35 (bough) a growth unit, relay 1;
  - the relay at 0.15 of the last growth unit, turned 0.6 to the upper side;
  - the base straightening 0.6;
  - the tip bending toward 0.2 rad (limb) and 0.1 rad (bough), tropism 0.6.
- **Each module rises at its base and arches over at its tip.** The next relays from the upper side of the bend, so the arching tips form the crown's surface.
- **Lifespans:** limb 70 units, bearing boughs at 0.16 at the module top; bough 30, bearing spurs at 0.14. The leader and spurs are as in round 14.
- **Budget and girth:**
  - Module stacks multiply boughs and spurs. A first try grew 19M phytomers in the beech test, against its 10M budget, with a dbh of 1.58 m.
  - Fewer laterals bring it inside: boughs and spurs as above, and short shoots 0.3 and branch systems 0.35 on reiterates. The beech test passes with its budget unchanged; the expected growth is 10.5M including shed wood.
  - dbh is 1.16 m at seed 1.

**Heights and widths at 80 years** (`raw/final15/run.log`; the bole is clear to about 4.5 m, 0.23 H, read off the stills):

| Seed | Height | Width (x × z) | Reading against Astra's "upright fan with separate peaks" | Beech? |
|---|--:|--:|---|---|
| 1 | 19.3 m | 20.7 × 21.6 | One rounded, integrated dome, reaching down near the fork; no separate peaks | **Yes** |
| 7 | 19.1 m | 21.0 × 18.5 | A rounded dome on an erect bole, the outline continuous; a few thin shoots at the lower right | **Yes** |
| 2 | 20.2 m | 18.7 × 19.1 | A rounded, slightly ovoid dome, deeper than wide on one axis | **Yes** |
| 3 | 19.0 m | 20.7 × 21.1 | A broad, rounded dome with a continuous outline; a few shoots stand out at the right edge | **Yes** |
| 4 | 18.0 m | 20.5 × 22.5 | A broad dome, slightly flat-topped, filled to the lower crown | **Yes / borderline** (flat top) |

**Five of five read as one integrated dome, with no upright fan and no separate peaks.**
- The limbs are no longer separately visible in the bare tree: they arch and branch into a fine, even crown. Bare, it reads closer to Rostock's dense crown than to Entzia's open scaffold.
- In leaf, each crown is a connected dark mass, the nearest yet to Nettleden.
- The crowns are still about as wide as they are tall: depth (height less the bole) over width is about 0.7 to 0.85.
- Trunk bases are smooth, with faint seams.
- Young ages are unchanged: a pole with rods at 10 and 20; at 40 a narrow, rounded crown on a tall bole.

**Tests:** all crate tests are green (`raw/space-tests-r16.log`): the oracle, every walk, and the beech at every age within its unchanged budget. Clippy is clean.

**Costs at 80 years:**
- Growth: 2.3 to 3.9 s.
- Dressing: 1.6 to 2.0 s.
- 1.37M to 1.60M nodes and 2.6M to 3.4M leaves.

## Gate, round 16, and host decision 16 (2026-10-04)

Host: five integrated domes, the best sheet; sent to Astra. Astra: FAIL (3 yes; 1, 7, 4 borderline; 2 no): "most seeds form angular, tiered masses with pointed tops instead of the photographs' broad domes" (`ASTRA-VERDICT-R16.md`). Single Astra samples are noisy (seed 2 went from its yes in round 13 to no, seed 3 from no to yes, with modest changes), so the gate now takes three independent Astra judgements per sheet and a seed counts as yes on a majority. Host decision 16: round 17 softens the horizontal foliage shelves (layered sprays stay, but the crown outline must be continuous, not stepped) and rounds the top (the leader's and the top limbs' last modules arch over instead of pointing up).

## Round 17: round tops, a continuous outline, calmer twigs (worker)

**Engine:** unchanged. Round 16's architecture is kept, with values only (`beech.rs`):
- **(b) Rounded top.**
  - The leader is now a module stack too: abortion 0.3, relay from the upper side of the bend, tip arching toward 0.8 rad.
  - Limb and bough tips arch toward 0.3 and 0.2 rad (0.2 and 0.1 before).
  - An arching leader toward 0.5 rad was tried first. It gave the same rounded top on a slightly lower tree, so 0.8 was kept.
- **(a) A continuous outline.**
  - Branch systems roll about their bearer by up to 1.2 rad (0.7) and wander 0.5 (0.8). The sprays stop stacking into level shelves.
  - Module timing already differs per limb, since each module's end is its own draw.
- **(c) Fewer tangled twigs.** Long shoots wander 1.0 (2.0) and branch systems 0.5. The bare fine crown is calmer, with fewer looping shoots.

**Heights and widths at 80 years** (`raw/final16/run.log`):

| Seed | Height | Width (x × z) | Reading against Astra's "angular, tiered masses with pointed tops" | Beech? |
|---|--:|--:|---|---|
| 1 | 18.1 m | 19.5 × 21.1 | A broad, rounded dome, no pointed top; the outline continuous, with layered texture inside it | **Yes** |
| 7 | 17.5 m | 19.9 × 18.9 | A rounded dome with a soft, continuous outline; no shelves at the edge | **Yes** |
| 2 | 16.9 m | 17.9 × 19.4 | A rounded crown, full to the top; a few sprays stand out at the lower right | **Yes** |
| 3 | 17.6 m | 19.4 × 20.6 | A broad dome, rounded top; the lower left edge is still layered | **Yes / borderline** |
| 4 | 17.1 m | 20.5 × 21.9 | A broad, rounded dome, flatter on top than the others | **Yes / borderline** |

**No seed has a pointed top any more, and the outlines are continuous.**
- The crowns are broader than tall, 1 to 2 m lower than round 16: Entzia's proportions rather than Nettleden's.
- **Bare:** a fine, even crown; the twigs are less tangled than round 16's.
- **Trunk bark:** the horizontal bands Astra flagged remain. They are the bark texture's rings plus faint module seams (`raw/final16/bases.png`). That is the preset's bark rows and the conversion, outside this round's values; not changed.
- Young ages are unchanged.

**Tests:** all crate tests are green (`raw/space-tests-r17.log`): the oracle, every walk, and the beech at every age within its unchanged budget. Clippy is clean.

**Girth and costs:**
- dbh 1.17 m at seed 1.
- At 80 years: growth 2.2 to 3.3 s, dressing 1.5 to 2.0 s, 1.39M to 1.62M nodes.

## Gate, round 17, and host decision 17 (2026-10-04)

Host: five rounded, continuous domes; confident. Astra, three independent samples: FAIL in all three (majority yes on at most one seed). All three name the same fault: "the branch hierarchy: all five transition too abruptly from trunk and fork into fine, crowded branches, missing the long, substantial rising and spreading limbs" (`ASTRA-VERDICT-R17-{1,2,3}.md`), and all three flag the banded bole. Host decision 17: limbs keep their girth across relays as the trunk does, so a limb reads as one long tapering limb; fine laterals start further out along each limb; progressive taper along the hierarchy; the bark preset's ring banding down.

## Round 18: a branch hierarchy that tapers, a smooth bole (worker)

**Measured first** (a temporary dump along one limb of seed 1, since removed):
- **(a) is already the case.** A limb's relays keep its girth: the conversion's relay rule applies to every physiological age (PA), and the pipe model gives a relay the section of all it carries. Along that limb, radius runs 0.33, 0.33, 0.31, 0.29, 0.26, 0.20, …, 0.05 m over 25 modules, with no step between modules.
- **What drew the "abrupt transition":** with an area-preserving pipe (exponent 2), each fork divides the section, so a limb's thickness fell into its many laterals within a few modules. By mid-crown every axis was centimetres thick.

**Engine: one new setting, `form.exponent`** (per PA, neutral 2). It is the pipe model's exponent: a section is the sum of the radii it carries raised to the bearer's exponent. Above 2, a bearer stays thicker beside what it carries, so the taper is progressive along the hierarchy.
- Neutral 2 leaves every structure as it was: the oracle, every walk and every earlier test pass unchanged.
- It is walked over 2 to 3 on every PA, and refused by name outside 1.5 to 4.
- Test: `a_larger_pipe_exponent_keeps_limbs_thicker_beside_their_bearer`.

**Beech values:**
- **(c) Progressive taper.**
  - Exponent 2.6 on the trunk and the reiterates (leader, limb, bough, spur); 2 on branch systems, shoots and short shoots.
  - Pipe per phytomer: 7.5 mm on the trunk and reiterates, so a limb keeps its own wood; 0.9 mm on branch and shoot, 0.95 mm on short shoots.
  - dbh 1.01 m at seed 1.
  - Tried: exponent 2.6 with the same pipe everywhere drew a 0.49 m dbh, and pipes scaled up to match drew a bare crown of dark, crowded fine wood. Reiterate pipes of 11 to 12 mm drew blunt, thick limb ends.
- **(b) Fine laterals further out.** Branch systems survive at 0.95 a year (0.97), so the inner, older sections of each limb carry fewer of them, and the limbs show inside the crown.
  - At 0.93 the crown thinned visibly in leaf, so 0.95 was kept.
- **(d) Bark banding.**
  - The horizontal bands are the bark's plates at a 4 cm axial scale. `material/plateScale` is now 1.0 in `presets/european-beech.values` (owner-approved for materials).
  - The preset identity digests in `crates/telperion-core/tests/catalogue_identity.rs` are re-pinned. All `telperion-core` tests are green (`raw/core-tests-r18.log`).

**Heights and widths at 80 years** (`raw/final17/run.log`), and the per-seed reading against Astra's "abrupt transition into fine, crowded branches":

| Seed | Height | Width (x × z) | Reading | Beech? |
|---|--:|--:|---|---|
| 1 | 18.1 m | 19.5 × 21.0 | Substantial limbs rise and spread from the fork and taper into the crown; a rounded dome in leaf | **Yes** |
| 7 | 17.5 m | 19.9 × 17.7 | Several thick limbs rise from the fork and run out into the dome | **Yes** |
| 2 | 16.9 m | 17.8 × 19.4 | Spreading limbs visible well into the crown; a broad dome | **Yes** |
| 3 | 17.6 m | 19.4 × 20.6 | Thick limbs from a low fork spread out; a rounded dome | **Yes** |
| 4 | 17.1 m | 20.1 × 21.9 | Long spreading limbs under a broad dome | **Yes** |

**Five of five show long, substantial limbs that rise and spread and taper into the crown, under a rounded dome.**
- **Remaining fault:** the outermost reiterate tips end bluntly. With 7.5 mm a phytomer, a limb's or bough's last nodes are about 8 mm thick, and the bare outline reads slightly spiky at the edge.
- **Bole** (`raw/final17/bases.png`): smoother; the fine horizontal lines are mostly gone. One faint seam per module join remains at seeds 4 and 7, and seed 4 shows a limb's shadow arc.
- **Young trees:** still a pole at 10 and 20 years, now with a slightly thicker leader.

**Tests:** all crate tests are green (`raw/space-tests-r18.log`), including the oracle, every walk and the beech within its unchanged budget. Clippy is clean.

**Costs at 80 years:**
- Growth: 2.0 to 2.9 s.
- 1.11M to 1.32M nodes and 1.8M to 2.2M leaves.

## Round 19: limb tips ripen into a fine haze (worker)

**Engine: one new setting, `form.ripening`** (per PA, neutral 0). It is the number of years over which a phytomer lays down its own pipe: a phytomer `y` years old (its first year counted as 1) adds `min(y / ripening, 1)` of its own section.
- The setting is continuous: as `ripening` falls to 0 every share tends to 1.
- At 0 it is exactly the old behaviour: the oracle, every walk and every earlier test pass unchanged.
- It is walked over 0 to 20 on every PA, and refused by name below 0.
- Test: `ripening_leaves_young_tips_fine`.
- The tree's topology does not change; only girth does.

**Beech:**
- The reiterates (leader, limb, bough, spur) ripen over 15 years.
- Their pipe is 8 mm (7.5 before), so the old, inner limb keeps its weight and dbh stays near round 18's: 0.97 m at seed 1.
- Everything else is round 18's.

**Effect** (`raw/final18/seed4-final17-vs-final18.png`, seed 4 bare, round 18 beside round 19):
- The spiky, blunt limb ends are gone. Each limb tapers into fine young tips, and the crown edge reads as a fine haze of twigs, closer to Entzia's.
- At the five-seed thumbnail scale the difference is slight. It is plain at full size.

**Heights and widths at 80 years** (`raw/final18/run.log`; unchanged from round 18, since only girth moved), and per-seed reading:

| Seed | Height | Width (x × z) | Reading | Beech? |
|---|--:|--:|---|---|
| 1 | 18.1 m | 19.5 × 21.0 | Heavy limbs from the fork, fine tips, a hazy crown edge; a rounded dome in leaf | **Yes** |
| 7 | 17.5 m | 19.9 × 17.7 | Thick rising limbs ending in fine tips; dome | **Yes** |
| 2 | 16.9 m | 17.8 × 19.4 | Spreading limbs, a fine edge; broad dome | **Yes** |
| 3 | 17.6 m | 19.4 × 20.6 | Limbs spread from a low fork, fine tips; rounded dome | **Yes** |
| 4 | 17.1 m | 20.1 × 21.9 | Long spreading limbs into a fine twig haze (the side-by-side); broad dome | **Yes** |

- The limb close-ups (`raw/final18/limbs.png`) show thinner terminal shoots than round 18.
- Trunk bases are as in round 18: smooth, with one faint seam at seeds 4 and 7.
- Young ages are unchanged.

**Tests:** all crate tests are green (`raw/space-tests-r19.log`), including the oracle, every walk and the beech within its budget. Clippy is clean.

**Costs at 80 years:** unchanged from round 18: 1.11M to 1.32M nodes, growth 2.0 to 2.9 s.

## Gate, round 19, and host decision 18 (2026-10-04)

Host: fine tips now, limbs heavy inside; sent to Astra. Three samples: FAIL in all, the same fault: "the trees repeatedly compress their main limbs into an abrupt, shallow fan above the clear bole, instead of developing substantial rising and spreading limbs throughout a deep, rounded crown" (`ASTRA-VERDICT-R19-{1,2,3}.md`). Cause: the trunk forks at about 4.5 m (0.23 H) and every limb radiates from that point. Millet (LITERATURE.md, "Troll's model in *Fagus*") and Rostock show the trunk continuing through the crown, bearing limbs at many heights, and forking late near the top at maturity (about 20 m), where height growth stops; the clear bole is where the lowest limbs start, not where the trunk ends. Host decision 18: the trunk persists through the crown as the leader, bearing limbs at heights distributed from the clear bole up, and forks late, near the top, at maturity.

## Round 20: the trunk persists through the crown and forks late (worker)

**Engine:** unchanged.

**Beech values** (`beech.rs`):
- **The reference axis reorders to trunk → leader → fork.** The fork now comes after the leader, where it came before it.
- **The leader is the trunk carried on through the crown.**
  - A 40-unit stack of erect Troll modules: abortion 0.3, relay 1, the relay from the upper side of the curvature zone, tips toward 1.35 rad, straightening 1, viability 1.
  - It bears a limb (0.3) or a bough (0.2) at each module top, so limbs leave the stem at many heights from the clear bole up.
- **The fork comes late, at the top, at maturity.** After the leader's 40 units, at about 52 years, the fork is a 2-unit PA bearing limbs at its top nodes. It has no `next`, so height growth ends there.
- **Kept from round 19:**
  - The 12-unit clear bole, about 4.5 m, about 0.24 H.
  - The module limbs and boughs, fine tips and ripening.
  - The colours and the bark plates.

**Heights and widths at 80 years** (`raw/final19/run.log`), and per-seed reading against Astra's "abrupt, shallow fan above the clear bole". Full-size bare stills of all five are in `raw/final19/bare-large.png`. I viewed every still.

| Seed | Height | Width (x × z) | Reading | Beech? |
|---|--:|--:|---|---|
| 1 | 18.8 m | 20.8 × 20.5 | A deep, rounded crown from about 5 m to the top. The limbs still leave from a short stretch just above the bole, so the fan is shorter but not gone | **Borderline** |
| 7 | 18.4 m | 18.4 × 15.0 | The stem runs on into the crown, with limbs at several heights; a deep, rounded dome in leaf | **Yes** |
| 2 | 19.1 m | 17.1 × 20.9 | The stem continues through the lower crown, bearing spreading limbs at several levels; a deep dome | **Yes** |
| 3 | 18.5 m | 17.9 × 18.9 | A deep, round crown on a continuing stem; limbs at several heights | **Yes** |
| 4 | 18.7 m | 15.6 × 17.7 | The stem is clearly visible through the crown, bearing limbs from about 6 m up: Rostock's build. The crown is narrower and lighter than the others, and one low branch loops near the bole | **Yes / borderline** (narrow) |

**The crowns are now deep and rounded**, about as tall as wide or taller, against round 19's shallow, wide fans. In leaf they are the closest yet to Nettleden's mass.
- At four of five seeds the limbs leave the stem at several heights.
- Seed 1 still bunches its limbs low.

**Other observations:**
- Young ages: at 40 years the trees now carry a crown on a continuing stem; 10 and 20 years are unchanged.
- Trunk bases: smooth, with faint module seams at seeds 7, 3 and 4.
- Limb close-ups: heavy limbs with fine tips.

**Girth and costs:**
- dbh 1.08 m at seed 1.
- At 80 years: growth 1.4 to 5.0 s, 1.05M to 2.93M nodes. Seed 3 is the heaviest, at 2.93M nodes and 4.3M leaves.
- The beech test's seeds 1 and 7 stay within its 10M budget (expected 9.9M grown including shed wood).

**Tests:** all crate tests are green (`raw/space-tests-r20.log`): the oracle, every walk, and the beech at every age. Clippy is clean.

## Gate, round 20, and host decision 19 (2026-10-04)

Host: deep rounded crowns on a continuing trunk; confident. Astra, three samples: FAIL in all (one yes for seed 7 in one sample; seed 2 "the strongest beech candidate"); all name the branch hierarchy: "trunks resolve too abruptly into crowded slender limbs, without enough substantial, tapering main branches" (`ASTRA-VERDICT-R20-{1,2,3}.md`). The leader puts out a limb at every module top, so the crown has many similar slender limbs. Host decision 19: a vigour hierarchy among sibling limbs (one continuous engine setting, neutral today's behaviour): a few major limbs take most of the vigour, grow thick and divide progressively; the rest stay boughs and self-prune; fewer limb-bearing modules on the leader.

## Round 21: a vigour hierarchy among limbs (worker)

**Engine: one new setting, `form.dominance`** (per PA, neutral 0).
- A lateral of the PA keeps `1 - dominance × (1 - u⁴)` of its size, where `u` is a draw its lineage keys (`lineage::DOMINANCE`). So a few laterals hold nearly all their vigour and most are made smaller.
- It scales the lateral's base size in placement, so everything it bears shrinks with it, and its pipe girth follows.
- It is continuous in the setting, and topology does not change.
- At 0 it is exactly the old behaviour: the oracle, every walk and every earlier test pass unchanged.
- It is walked over 0 to 1 on every PA, and refused by name outside 0 to 1.
- Test: `dominance_leaves_a_few_laterals_whole`.

**Beech values:**
- **Limbs:** dominance 0.3 and an internode of 6 cm (4 cm before), so the dominant limbs run long. Boughs keep dominance 0.
- **Tried:**
  - Limb dominance 0.7 with bough dominance 0.5 shrank the crowns to 7 to 10 m wide and the dbh to 0.56 m: too few limbs held their size.
  - Limb dominance 0.5 drew lopsided, narrow crowns.
  - 0.3 keeps the crown full while a few limbs dominate.
- **Fewer limb-bearing modules on the leader: tried and reverted.** Limb 0.2 with bough 0.3 a module top, with dominance on, emptied the crown at three seeds (one-sided at seeds 1 and 4). The leader keeps round 20's limb 0.3, bough 0.2; dominance alone makes the hierarchy.
- **Kept from round 20:** the continuing trunk and late fork, the module limbs, fine tips and ripening, the colours.

**Heights, widths and major limbs at 80 years** (`raw/final20/run.log`; major limbs counted by eye on `raw/final20/bare-large.png` as limbs visibly thicker than the rest and running to the crown edge). Per-seed reading against Astra's "trunks resolve too abruptly into crowded slender limbs". I viewed every still.

| Seed | Height | Width (x × z) | Major limbs | Reading | Beech? |
|---|--:|--:|--:|---|---|
| 1 | 20.2 m | 21.3 × 18.3 | 3 | One long limb rises left, two spread right, dividing outward. The crown is tall, rounded and lopsided to the left | **Yes / borderline** (lopsided) |
| 7 | 18.5 m | 19.7 × 16.0 | 4 | A long, heavy limb runs out far to the left and divides into spray; three more rise right. Entzia's spread | **Yes** |
| 2 | 20.1 m | 16.6 × 19.5 | 4 | Several thick limbs rise through a deep, rounded crown, dividing progressively | **Yes** |
| 3 | 18.5 m | 15.5 × 17.7 | 3 | Three heavy limbs from the trunk; a deep crown with a dense mass on the left and an open limb on the right | **Borderline** (dense mass) |
| 4 | 18.6 m | 15.6 × 18.1 | 4 | The trunk continues; limbs at several heights, two long ones spreading right; a narrower crown | **Yes / borderline** (narrow) |

**Three to four major limbs at every seed, within the target of 3 to 6.** The limbs visibly divide outward at seeds 7, 2 and 1.
- In leaf the crowns are more irregular than round 20's: seed 7 is a wide, flat-topped spread like Entzia; seeds 1 and 4 are lopsided.
- Trunk bases are smooth, with faint seams at seeds 3 and 4. The limb close-ups show heavier limbs dividing into fine tips.
- Young ages: 40 years shows a dominant limb at the top. 10 and 20 years are unchanged.

**Tests:** all crate tests are green (`raw/space-tests-r21.log`): the oracle, every walk, and the beech within its budget. Clippy is clean.

**Girth and costs:**
- dbh 0.91 m at seed 1.
- At 80 years: growth 1.5 to 4.5 s, 1.01M to 2.86M nodes.

## Gate, round 21 (2026-10-04)

Astra, three samples: FAIL in all, no yes; "branches form upward-flaring fans with angular tops rather than spreading into broad, rounded domes" (`ASTRA-VERDICT-R21-{1,2,3}.md`). The dominance setting made crowns lopsided; round 20 (one yes, seed 2 the strongest candidate) stays the best on Astra's scale. Rounds 13 to 21 oscillate between fans and domes. The host pauses new rounds pending the owner's call on the gate (owner judges vs host and Astra).

## Owner verdict: the beech passes (2026-10-04)

> owner: "i mean to me they look really good structurally."

The beech passes phase C on the owner's verdict, with round 20's values (raw/final19: continuing trunk, late fork, limbs at several heights, deep domes, fine tips, matched colours). Round 21's `form.dominance` engine setting stays at its neutral 0 for the beech. Known gaps, recorded for later work, not this phase: Astra's branch-hierarchy critique (fewer, heavier limbs dividing progressively; `ASTRA-VERDICT-R19/R20`), the tube-like regular branch surface (owner, round 13), and the young ages (10 and 20 years) still reading as a pole with rods.

## Round 22: round 20's beech, with foliage lower on the tree (worker)

**Restore.** The beech's values are back to round 20's (`903ade4f`), and `form.dominance` stays in the engine at its neutral 0 on every beech PA. The five 80-year renders reproduce `raw/final19` exactly. Node counts are in `raw/final21/restore-r20.log`:

| Seed | Nodes, round 20 | Nodes, restored |
|---|--:|--:|
| 1 | 2,024,933 | 2,024,933 |
| 7 | 1,708,438 | 1,708,438 |
| 2 | 1,526,680 | 1,526,680 |
| 3 | 2,925,135 | 2,925,135 |
| 4 | 1,046,764 | 1,046,764 |

**One change, aimed at the owner's note** ("they still lack some foliage lower on the tree"):
- **The clear bole is 9 growth units** (12 before), about 3.4 m, 0.18 H. So the leader, with its limbs and boughs, starts three years earlier and lower, and those lower limbs have three more years of leafy growth.
- **Budget:** the extra wood took the beech test past its 10M budget. Branch systems on reiterates are 0.27 (0.35), which brings it back inside: 9.6M expected.
- **Tried and reverted:** branch-system viability 0.97 (0.95), for longer-lived lower sprays, grew 14M and overran the budget.
- **Kept from round 20:** the continuing trunk, the late fork, the module limbs, fine tips and ripening, the colours.

**Sheet for the owner: `raw/final21/five-seeds.png`.** Also `raw/final21/leaf-large.png` (full-size in leaf), `sheet-beech.png`, `bases.png` and `limbs.png`. I viewed every still.

| Seed | Height | Width (x × z) | In leaf |
|---|--:|--:|---|
| 1 | 18.6 m | 22.5 × 21.2 | A deep, rounded crown with foliage down to about 3.5 m; a few loose sprays at the lower left |
| 7 | 17.7 m | 18.3 × 19.1 | A full dome reaching down toward the bole, the nearest to Nettleden |
| 2 | 18.4 m | 18.1 × 21.1 | A broad dome with leafy lower limbs spreading low on both sides |
| 3 | 18.1 m | 20.9 × 19.6 | A dense, round crown down to the bole on the left |
| 4 | 18.2 m | 18.9 × 16.8 | A dome with leafy limbs low left and right; more open in the middle |

- In leaf, every tree carries foliage well down toward the bole, against round 20's crown base at about 4.5 m.
- Bare, the limbs and the continuing trunk read as in round 20.
- The bases are smooth, with faint module seams at seeds 7 and 3.
- dbh is 1.09 m at seed 1.
