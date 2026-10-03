# fn-192 result: continuity by construction

The engine core in `crates/telperion-space` is now continuous in every setting. At a fixed seed, walking any continuous setting changes the tree by degree, and the fn-191 oracle tests still pass at their exact points. Everything below was measured on 2026-10-03 on the desk (AMD Ryzen 9 5950X).

## Lineage-keyed draws (R1)

- **Keys.** Every draw is a hash of the bud's path from the root and of what it decides. The path runs through the growth unit's rank on its axis, the zone, the node's draw index and the bud's slot. A continuation and a relay bud each have their own step. No draw is a position in a stream. Each axis carries its key as `Axis::lineage`, so a branch can be matched across settings.
- **Acrotony.** Within a zone, nodes stand in order of their draws rather than by the PA each drew. A bud that a setting makes or unmakes therefore moves no node. The ordering has the same distribution as the simulators' sort, because the youngest PA is the lowest draw.
- **Node counts.** A node count comes from the inverse of its law's distribution, applied to one draw.
- **Test.** `tests/lineage.rs` raises the trunk's lateral probability step by step at eight seeds. Every branch of the sparser tree, matched by structural path, keeps its PA and its growth. On the fn-191 engine the test was red: at seed 1, branch `s/6.1/4.0` vanished as soon as branches were added. It is committed red first, in `54c280a4`. The log is `raw/r1-red-on-base.log`.

## Birth and death at vanishing size (R2)

- **Presence.** Every element a draw makes carries a presence from 0 to 1, which scales its length and girth (`Phytomer::scale`, `Axis::vigour`). The presence is 0 where a setting crosses the draw. It grows to 1 once the setting is `GROW_IN` (0.1) of the draw's room past it.
- **Room.** The room is the narrower of two widths: the run of draws that makes the element, and the distance from the draw to certainty on the side the setting comes from. A certain element is therefore always whole. At most 2 × `GROW_IN` of the elements a setting makes are growing in at any setting.
- **What carries a presence.** Lateral buds, from either bound of their PA's run. Poisson nodes. Apex survival and abortion, which carry into every later unit and into the continuation. Relay buds. Shedding, which fades with the subtree's living time weighted by presence: a branch fades away before it is shed.
- **Shedding floor.** A shed fade never draws a living subtree smaller than its most present living apex, read from the apex's running presence at the tree's age. A relay counts the cycle between its parent's death and its own birth.
- **No size.** A tree whose every phytomer stands exactly at its draw, at no size, is `Error::Collapsed`.
- **Phyllotaxis.** It counts nodes by their presence, so a node growing in turns the nodes above it by degree.
- **Test.** `tests/walks.rs`, `branches_are_made_and_unmade_at_vanishing_size`. In every walk, the branch made or unmade with the most wood is bisected 48 times to its crossing, where its wood is below 1e-6 of the tree.

## Discrete botany as settings (R3)

Five new settings per PA, each refused by name outside 0 to 1:

| Setting | Neutral | Dormant when | What it does |
|---|---|---|---|
| `abortion` | 0 | always active | Sympodial growth: after each growth unit the apex aborts with this probability. |
| `relay` | 0 | no apex stops | Troll: a stopped apex is replaced by a relay bud of its PA, at its last node or at its base. |
| `readiness` | 1 | the axis has no laterals | Scales lateral probabilities; 0 is Corner's unbranched stem. |
| `rhythm` | 1 | one zone | Spreads the growth unit's laterals evenly over its nodes, turning tiers into continuous growth. |
| `straightening` | 0 | the axis is the seed or a continuation | Troll: a lateral's base is pulled towards the vertical. The pull vanishes for a branch hanging straight down, so the curl never has to pick a side. |

The closed form counts abortion, relays, readiness and rhythm. `tests/settings.rs` checks:
- dormancy, to the bit;
- the Corner stem;
- that the engine's mean over 4,000 seeds equals the closed form with all four settings away from neutral;
- straightening.

## The walk test (R4)

- **The tree.** `tests/walk/` holds a three-PA tree that uses every setting: two trunk zones, Poisson and uniform nodes, limbs that abort, relay and straighten and age into twigs, and twigs that are shed.
- **The walks.** Each of its 36 continuous settings is walked over its range in 200 steps at seeds 1 to 3, on its own scale: log-odds for a probability, its own unit otherwise.
- **The stated multiple.** No step moves any measure by more than 3 / `GROW_IN` = 30 times the step, per unit of the setting's scale. The measures are total length, height, spread, every branch's base and tip by lineage, and the wood made or unmade, each as a share of the larger tree's length or height. The worst slope is 20.5, on trunk viability at seed 2. Every setting's worst is in `raw/walks.log`.
- **No jumps.** Each walk's three steepest steps are split 8 ways, three times over, and must shrink at least twentyfold. A lifespan switch fails this check (`the_jump_check_sees_a_switch`).
- **Phase A.** Its oracle tests pass unchanged. The stochastic worst difference is 2.64 standard errors against a bar of 4.5; it was 3.81 before.

The jump check found three real discontinuities, all fixed:
- phyllotaxis counted nodes by integer index;
- straightening flipped sides at the nadir;
- shedding's living time let a newborn bud count time its partial parent had not lived.

## Still strips (R5)

`cargo run --release -p telperion-space --example strips -- <dir>` draws eight walks, nine frames each, side view, with each internode as wide as its scale. The walks are readiness (towards Corner), a trunk lateral probability, limb abortion, limb straightening, trunk rhythm, a Poisson mean, twig viability and trunk viability. The stills are in `raw/strips/` (ignored).

The worker viewed six strips: readiness, lateral, abortion, straightening, Poisson mean and trunk viability. Rhythm and twig viability were not viewed. The six change by degree, with one exception. On trunk viability at seed 2, the whole crown grows in between the frames at 0.951 and 0.966. That is one draw crossed over about 0.004 of viability, continuous at the test's resolution but a jump at nine frames. **The host has not yet viewed the strips.**

## Review (Codex)

- **Round 1** (three reviewers): integration and contracts returned SHIP. Correctness returned NEEDS_WORK on two findings, both fixed with regressions in `b258cabf`: a living relay's parent faded to nothing, and a tree of no size passed the collapse check.
- **Round 2** returned NEEDS_WORK on one finding: the new fade floor jumped when an abortion was decided in the tree's last cycle. It is fixed in `6bda9e02`; its regression was red on the previous floor (1.375 to 1) and is green now.
- **Round 3** was refused by flowctl's round cap (2 of 2), so the last fix is unreviewed.

## Decisions for the host

- **Steepness near certainty is inherent.** With exact probabilities (the oracle) and certain elements drawn whole (the deterministic sets), an element whose draw lies a distance d from certainty must grow in within d. So the rate of change is bounded per unit of log-odds, not per unit of probability, and the walk test walks probabilities in log-odds between 0.005 and 0.995. A fixed absolute grow-in width would bound the steepness, but it would draw part of every certain structure partly grown.
- **`GROW_IN` = 0.1.** On the walk tree, 11% of axes are born partly grown and 19% of total length is lost to presence. At 0.05 those figures are 5.6% and 9.9%, and the walk slopes double. Phase C's calibration absorbs the size change.
- **Integer fields stay discrete:** lifespan, the uniform node bounds, buds per node, the shedding delay and the next PA. They are the reference axis's structure, and none is walked. Making them continuous (a fractional lifespan, for example) is open for phase D's walks between species.
- **Laterals take their frame from the parent axis's heading,** not from the parent's straightened local direction.

## Cost

The sheet example's time per structure is 20 to 51 µs, about 85 ns per phytomer, against fn-191's 50 ns. The extra comes from the keyed hashes, the presences and the shedding fades. The workspace gate's new walk tests take about 15 s.
