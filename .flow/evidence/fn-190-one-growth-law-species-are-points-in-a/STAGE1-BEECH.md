# fn-190 stage 1: the European beech as Troll's model, 2026-10-03

## Summary

The module rule is built on the R1a engine (`organ.rs`, `organ/module.rs`): a tilting seedling, modules with an erect base and a plagiotropic tip, the relay from the curvature zone, and the fork near maturity. Plagiotropic distichous branch systems and a crown of reiterates that get older at each fork are built too. Every rule is one continuous setting with a neutral value.

- **Young tree:** reads as the source describes up to about 10 years, at both seeds.
  - The seedling's tip tilts and its first laterals are plagiotropic.
  - At seed 1 a crooked module head arches away while the relay carries the stem up.
- **From 20 years:** does **not** read as a beech.
  - The tree is a straight pole with stiff, straight, near-horizontal branch rods at every height, which reads as a conifer.
  - The fork at maturity happens. At seed 1 the trunk divides at about 0.35 H into a few ascending stems. The crown above is a flat-topped broom of straight whiskers.
  - The photographs show a short bole, many sinuous ascending limbs and a domed outline. At 75 years neither seed reads like them.
- **Cost at 75 years:** 132 to 137 ms warm, against today's 62 to 65 ms (2.1 to 2.2 times). Fine wood is 26.6 to 27.5 km, against today's 46.7 to 47.1 km (0.56 to 0.59). 58 to 60% of grown wood is pruned.
- **The open loop sits on a knife-edge.** Size either explodes past 4M grown nodes or the crown collapses after the fork. Most of the session went to that, not to architecture (`FRICTION.md`). Several rules below are mine and do not come from the sources. They are marked for the host.

## The rules and their sources

Quotes are from Millet, Bouchard & Édelin 1998, *Can. J. Bot.* 76:2100–2118 ([M98], *Fagus grandifolia*, `millet.md`), unless another source is named.

| # | rule as built | source passage |
|---|---|---|
| 1 | **Seedling.** The root bud is vertical (phi 0). Its tip tilts toward a seeded horizontal heading by `tilt × (1 − exp(−years ÷ tiltYears))` added to the lean. | "The seedling (Fig. 6a) is an orthotropic monopodium (A1) with rhythmic growth … The slightly older seedling (Fig. 6b) has the tip of its A1 axis tilted and has an alternate distichous phyllotaxy. The first lateral axis (A2) is plagiotropic." |
| 2 | **Module.** Every axis is a module. Its curvature zone is the first node whose direction falls below `bendElev`. Wood grown before it while still at or above `bendElev` is the erect base. Only the erect base straightens, by `straighten × (1 − phi)` radians a year, up to the share `(1 − phi)^straightenShape` of its gap to vertical. This is one end pass equal to the yearly sum (host decision after R1a round 1, item 3). | "Its modules are large monopodial axes with rhythmic growth, whose proximal part is erect and whose distal part is plagiotropic." "The first module (AU1) has a secondary straightening of its stem up to the point of insertion of the lateral axis, which takes on the relay of the apical dominance." CIRAD: "a trunk made by the stacking of the plagiotropic portions that are sort of straightened up." |
| 3 | **Relay.** Once a module has a curvature zone, each year it relays with probability `relay × (1 − phi)^relayShape`. The relay is a new axis at the curvature-zone node, `relayUp` degrees above that node's elevation (the upper side). Its heading turns by up to `relayTurn` of a random full turn about the vertical, and it continues the line (the stem follows relays). | "The modules are derived from one another by branching in the curvature area between these two parts." "The plagiotropic relay straightens secondarily and reproduces the structure of the AU1." Epitony, the upper side, is "typical of the Champagnat, Troll and Mangenot models" ([BC07], `bc2007.md`). "the orientation of the branching plane may change from one module to the other." |
| 4 | **The head becomes a branch.** At the relay, the head's phi jumps by `headJump` and it lives `headLife × E(t)` more years. E is the establishment curve, so the head dies soon in the seedling years and later lives long. | "The plagiotropic extremity of the module has not, at this stage, a great life expectancy. It can continue its development horizontally, but it most often aborts rapidly … During the growth succession, the AU1s gain in number of branching orders and the life expectancy of their plagiotropic extremity increases, creating a branch." |
| 5 | **Branch axes (A2 to A5).** Laterals take phi from the R1a birth jump (`phiStep`, `zone` for acrotony, `vigourJump`). They lean by `lean1 × phi`, capped at `plagio`. They are born two-ranked beside the shoot (`distich` 1). They branch by `branching × (1 − phi)^fate`. Their shoots weaken by `phi^vigourShape` toward `n1` metamers and `short` internodes. | "All the other categories of axes (A2 to A5) are plagiotropic monopodia with distichous phyllotaxy … All of the A2 branches are systems of four categories of axes arranged in a single horizontal plane, which gives them a dorsiventral bilateral symmetry. The ultimate axes (A5) are short ageotropic shoots with very closely spaced leaves." Dupré et al. 1986 (`dupre.md`): short shoots have "3 à 5 entre-nœuds courts" and buds "qui ne se développement jamais"; long shoots "6 à 10 entre-nœuds". Nicolini 1998 (`nicolini98.md`, abstract): "acrotonie primaire … qui favorise la formation et le développement des axes latéraux les plus hauts sur la pousse annuelle". |
| 6 | **Establishment and lower-branch decline.** These are R1a's establishment curve and lifespans by phi, unchanged in form. | Nicolini 1998: an "augmentation progressive du taux de croissance annuel", and a "réduction graduelle … des axes les plus bas dans l'arbre [qui] annonce leur déclin et leur disparition prochaine par élagage naturel". |
| 7 | **Fork near maturity.** A relay event makes `1 + fork × m(t)` relays in expectation. m is a logistic of age about `forkAge` with width `forkWidth`. Forked relays leave at `forkAngle` off vertical, spread evenly, and are alike. Each fork moves their phi by `reiterStep × (relays − 1)`, so later reiterates are weaker and shorter. | "At the top of the stem the branches become large and oblique, and a fork is created from subterminal buds. The relays do not differentiate themselves from one another, so that the growth of the stem is definitely stopped. Each reiterate forks and reiterates in its turn, producing reiterates that are increasingly smaller and less branched." "The crown of the tree is made up of a succession of such reiterates, the most peripheral being the shortest." |
| 8 | **Sequential reiteration of upper laterals.** Near maturity a lateral's phi falls back toward its carrier's by `reiterate × m(t) × u × vigour`, where u is its place along the unit. | "The mature tree … has main branches in the upper part of its stem … These branches are oblique, have a large diameter, and reproduce the structure of the young tree." [BC07]: "Mature broadleaf crowns are mostly built of reiterated complexes." |

### Mine, not the sources' (for the host)

- **`plagio`, the lean cap.** Without it every plagiotropic axis ends horizontal and the young tree reads as a fir with flat plates (v1 to v4). Millet says only that higher branches are "oblique". The cap value (0.75, about 18° above horizontal) is my choice.
- **A reiterate renews its carriers' lives.** When an axis relays, every death record carrying it is extended to the relay's lifespan. Without this the mature crown collapsed between 55 and 75 years, because reiterates born on mortal branches died with them (v5 to v11). Millet's "great aptitude to reiterate, allowing for a wide crown and a prolonged lifespan" points this way, but the mechanism is mine.
- **Death records.** A relayed head gets its own record, so it can die without its base. A bud stops growing when any carrier's record is dead. This is bookkeeping for rules 4 and 6.
- **`tipRadius`.** Radii come from the pipe model with a fixed tip radius (8 mm), not the preset's root, so a young tree has a young tree's girth. Neutral 0 keeps R1a's rule.
- **Removed:** R1a's `persist0`, `persist1`, `persistShape`, `reiteration` and `reiterShape` (the persistent leader by phi, with relay only on abortion). R1a's points in `raw/r1a/pts` no longer load.

## Settings (all continuous; neutral is a monopodium with no tilt, relay or fork)

| setting | neutral | beech | meaning |
|---|--:|--:|---|
| tilt / tiltYears | 0 / 3 | 0.5 / 1.5 | module tip lean added with module age |
| bendElev | 45 | 55 | curvature-zone elevation, degrees |
| relay / relayShape | 0 / 4 | 0.5 / 3 | yearly relay chance × (1 − phi)^shape |
| relayUp / relayTurn | 30 / 0 | 25 / 1 | relay elevation above the bend; heading turn share |
| headJump / headLife | 0 / 1000 | 0.5 / 50 | head's phi jump; years it lives × E(t) |
| fork / forkAge / forkWidth / forkAngle | 0 / 40 / 4 / 35 | 1 / 22 / 4 / 30 | relays per event at maturity, and their spread |
| reiterStep | 0 | 0.08 | phi added per extra relay of a fork |
| reiterate | 0 | 0.3 | upper laterals' phi falling back near maturity |
| straightenShape | 0 | 4 | how far an older erect base straightens |
| plagio | 1 | 0.75 | the most an axis leans |
| vigourShape | 1 | 0.5 | metamers and internodes blend by phi^shape |
| tipRadius | 0 | 0.008 | tip radius, metres (0: the preset's root) |

The full point, R1a settings included (`raw/stage1/beech.json`): cycles 75, unit 0.006, n0 8, n1 3, short 0.3, drift 0.03, est0 0.3, estYears 8, straighten 0.2, branching 1, fate 2, phiStep 0.45, zone 3, vigourJump 1, acrotony 1.5, lean1 1.2, eta 0.15, angle0 50, angle1 60, distich 1, life0 120, life1 20, lifeShape 4, ground 1, maxNodes 4M, and the table's beech column. Ages are grown with `beech.json@AGE`, and the same point is used at every age and seed.

## What each still shows

The sheet is `raw/stage1/sheet-stage1-beech.png`:

- Row 1: S1 Entzia, S1 Rostock, and the 75-year bare trees at seeds 1 and 7.
- Row 2: S2 Hrachoviště, S2 Nettleden, and the 75-year whole trees.
- Rows 3 and 4: bare trees at 5, 10, 20 and 40 years, seed 1 then seed 7.

The camera is fitted to each tree, so ages are not to one scale. The person is 1.8 m tall. I viewed every still.

| age | seed 1 | seed 7 | against the source and photos |
|---|---|---|---|
| 5 (2.2 / 2.5 m) | A thin stem whose tip bends over strongly; short plagiotropic laterals | The same, with a milder tilt | **Reads right.** Millet's Fig. 6b: "tip of its A1 axis tilted"; "first lateral … plagiotropic". |
| 10 (4.1 / 5.1 m) | A crooked module head arches out to the left; the relay carries the stem up from the bend | A straight stem with a tilted tip; laterals are straight rods | **Seed 1 reads right**: "a mixed sympodium, which is plagiotropic at its extremity and orthotropic at its base". Seed 7 relayed less and reads as a monopodium. |
| 20 (10.5 / 10.9 m) | A straight pole with radial, straight, near-horizontal rods to the ground; a tilted tip | The same | **Partly right**: Millet's young tree is "orthotropic along almost all its length" with plagiotropic A2. **Wrong** for a beech: it reads as a spruce sapling. The branches are stiff straight rods, and their flat systems show only as a haze. |
| 40 (16.8 / 20.4 m) | A tall narrow column, forked near the top into two stems; a fan of short flat reiterates; rods along the whole stem | One stem to the top; a flat broom of reiterates | **Wrong**: conifer- or Araucaria-like. There is no clear bole (rods reach the ground) and no ascending limbs. |
| 75 bare (22.3 / 24.4 m) | The trunk divides at about 0.35 H into three or four ascending stems under a flat-topped broom of straight whiskers. Thin horizontal rods on the lower bole | One stem to about 0.9 H under the same broom; it reads as a conifer | **Not a beech** at either seed. Seed 1's division is the closest to the photographs' "divides … into several ascending leaders", but its limbs are few, straight and thin, and the outline is an inverted cone with a flat top, not a dome. |
| 75 whole | Leaves sit along straight rods in parallel streaks: a stiff, cypress-like texture | The same | **Not a beech.** S2 Nettleden is a dense, rounded, opaque mass. This crown is a streaked block. |

## Measures at 75 years (reported, not voted)

| seed | warm ms (today) | grown / kept / pruned nodes | fine km (today) | wood km | H m (H ÷ authored) | w/h (authored) | crown base p5 | division (projected) | relays / forks |
|---|---|---|---|---|---|---|---|---|---|
| 1 | 132 (64.6) | 1.06M / 450k / 610k | 27.5 (46.7) | 30.8 | 22.3 (0.70) | 0.66 (0.72) | 0.51 | 0.25 | 1,355 / 1,328 |
| 7 | 137 (61.6) | 1.09M / 435k / 655k | 26.6 (47.1) | 29.8 | 24.4 (0.76) | 0.63 (0.72) | 0.43 | 0.56 | 1,370 / 1,328 |

- **Passes (cold ms):** at seed 1, sweep 118, prune 19, straighten 3.4 and pipe 3.2. The sweep grows the wood that is later pruned.
- **Fine wood is not like for like.** It is wood under 0.05 of the root's radius, and the root now comes from `tipRadius`. At 0.04 and 0.06 the figures are 24.7 and 28.9 km (seed 1).
- **The Troll trace reads a 0° rise.** Its first-order axes are laterals off the stem, which by rule 5 do not straighten. Only module bases do. The probe's Troll definition (host decisions after round 2, item 4) does not match the rule as built.

## Open questions for the host

1. **Should straightening carry the subtree?** The end pass turns each erect-base segment on its own, and wood born on it keeps its birth direction. In the tree, laterals ride up as the base straightens, and laterals born on a leaning module are computed against the leaning, unstraightened direction. This may be why the stem's A2 rods stand out at fixed angles and some laterals gain erect bases they should not have.
2. **Should a module's tip stay oblique or go horizontal?** Rule 1 drives every module tip toward its heading, capped by `plagio`. Millet's modules end "plagiotropic", which the sources do not quantify. The beech's ascending limbs in the photographs suggest reiterates stay oblique, so the tilt should depend on phi, or reiterates should tilt less.
3. **Should reiterates be told apart from A2 in phi?** Forked reiterates (phi 0.08 to 0.4) and A2 branches (about 0.36 and up) overlap, so one lean, lifespan and vigour curve serves both. Millet's mature crown needs reiterates that "reproduce the structure of the young tree" while A2 stay plagiotropic monopodia.
4. **Does the size problem belong to stage 5?** Each choice that brought the 75-year crown in reach was a lifespan or drift balance: the explosion and collapse in v3 to v11, and the renewal rule. That is the open loop's missing supply limit. Should stage 1 be judged before light and the carbon balance, or should a size bound (supply ÷ demand) come forward?
5. **Do Millet's ages hold for an open-grown beech?** The young tree is 13 m and the forked mature tree 20 m in forest-grown *F. grandifolia*. The photographs are open-grown *F. sylvatica*, which fork lower (about 0.2 H, S1 Entzia). CIRAD notes that Troll trees grown in the open tend toward "a shrub more than a tree". `forkAge` 22 was my compromise.

## Files

- `crates/telperion-render/examples/growth_law/organ.rs`: the engine, with the module rule's tilt, curvature zone, erect base and death records in the sweep.
- `crates/telperion-render/examples/growth_law/organ/module.rs`: the relay, fork, head, renewal and lifespans.
- `crates/telperion-render/examples/growth_law/params.rs`: the new settings.
- `crates/telperion-render/examples/growth_law/main.rs`: `SETTINGS.json@AGE` and fitted stills.
- Raw files (ignored, on disk):
  - `raw/stage1/beech.json`, the point;
  - `raw/stage1.sh POINT TAG SEED AGES…`, the age series;
  - `raw/stage1-try.sh TAG AGE KEY=VALUE…`, single-setting tries;
  - `raw/stage1/final-s{1,7}-a{5,10,20,40,75}.json`, the rows;
  - `raw/stage1/stills/`, every still including rounds v1 to v14;
  - `raw/stage1/sheet-stage1-beech.png`, the sheet.
