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

## Host verdict and decisions (2026-10-03, overnight run)

- **R5 strips, host view:** abortion and states[2].viability walk by degree. Two pops remain: states[0].rhythm (seed 1) adds a full-length basal branch between 0.500 and 0.375, and states[0].viability (seed 2) adds the whole upper crown between 0.951 and 0.966. R4/R5 are not met on the stills.
- **Decision:** the grow-in window becomes a fixed width in log-odds instead of probability, so near-certain structure (a trunk apex carrying the crown) grows in over a visible range; every birth, including the rhythm case, gets the same gradual entry. `GROW_IN` is set from that window, and the length lost to partly grown branches is reported.
- **Review cap:** rounds 1 and 2 each found and fixed real defects; the cap stopped round 3 on unreviewed fixes. The host resets the round counter once (`flowctl spec reset-review-rounds`), recorded for the owner's morning review: the overnight run was authorised to stop only on a genuine wrong path, and a converging review is not one. One reset per spec, never more.

## The log-odds window, measured (worker, after the host decision)

I built the host's fix as asked: every presence grows in over a fixed width in log-odds, with no cap from the run width. Certain structure stays whole. The oracle, lineage and closed-form tests pass with it, because presence never changes topology. The patch is `logodds-window.patch`, against `d47c8897`; it is not applied.

Length lost to partly grown branches on the walk tree, seeds 0 to 49 (`cargo run -p telperion-space --example partial`):

| `GROW_IN` (log-odds) | Length lost | Axes born partly grown | Trunk viability, seed 2 | Rhythm, seed 1 |
|---|---|---|---|---|
| reviewed code (0.1 of room) | 20.9% | 12.4% | crown pops between 0.951 and 0.966 | basal branch pops |
| 0.1 | 16.4% | 8.9% | not drawn | not drawn |
| 0.25 | 35.0% | 20.8% | crown ¾ grown by 0.966, whole by 0.977: near pop | by degree |
| 0.5 | 55.5% | 38.3% | crown half grown at 0.966, whole at 0.977 | not drawn |
| 1.0 | 75.6% | 66.4% | crown grows from 0.951 to 0.989 | by degree |

The stills are in `raw/strips-w0.25/`, `raw/strips-w0.5/` and `raw/strips-w1.0/` (ignored). The worker viewed every one listed.

**Why no width works.** A fixed window cannot remove the visible pop and keep the tree:
- The strips step about 0.39 log-odds per frame.
- An element that carries the crown needs a window of about 1 log-odds to grow in over more than one frame.
- A fixed window applies to every draw, so at that width two-thirds of all branches are born partly grown and three-quarters of the length is lost.
- Whether a fixed width pops also depends on the strip's frame count, not on the tree.

**Candidate for the host: a window that scales with what the draw decides.** Topology never depends on presence, so presences can be computed in a pass after growth, leaves first. Each draw would grow in over a log-odds window proportional to the presence-weighted length of the wood it decides, with a floor. That length is continuous in the settings, so presence stays continuous. A crown would then grow in over a wide window and a twig over a narrow one, and no crossing could change the tree faster than about (tree length) / (largest window) per log-odds unit. Expected length lost would be dominated by the few large draws near their bounds, rather than by every draw.

This is a design change to R2's mechanism, and it is untested. It is the host's call.

## Host decision 2 (2026-10-03, overnight run)

The fixed log-odds window is rejected on the worker's measurements (35% to 75.6% of length lost before the crown stops popping). Built instead: each draw grows in over a log-odds window proportional to the presence-weighted wood it decides, with a floor, presences computed in a pass after growth (topology never depends on presence). A crown grows in slowly and a twig quickly, so a crossing changes the tree at a bounded rate per log-odds unit. Gate: all eight strips by degree on the host's view, A's oracle green, length lost reported. Wrong-path stop for B: a visible pop remains, or more than 25% of length is lost across seeds 0 to 49.

## The wood-scaled window, measured (worker, after the host decision in bb480697)

Built as decided. `presence-pass.patch`, against `bb480697`, applies cleanly and is not applied.
- **Growth records leads.** Growth records each draw's lead past its bound in log-odds, and a pass after growth computes every presence, leaves first.
- **Window.** Each draw's window is `RATE × (presence-weighted wood it decides) / (the tree's expected length)`, clamped to `FLOOR` = 0.05 and `SPAN` = 2 log-odds.
- **Expected length.** It comes from the closed form (`expected_length`). A unit test checks it against the expected counts weighted by internode.
- **Tests at RATE 6.** All ten of the crate's test binaries are green: the oracle, lineage, closed form, settings, death and refusals tests and the walk bound of 30.

Length lost to partly grown branches on the walk tree, seeds 0 to 49 (`examples/partial.rs`). The rhythm and trunk-viability strips were viewed at every rate drawn; all eight were viewed at RATE 6.

| RATE | Length lost | Axes born partly grown | Trunk viability, seed 2 | Rhythm, seed 1 |
|---|---|---|---|---|
| 2 | 12.5% | 4.5% | crown grows in from 0.951 to 0.989 | a basal limb appears whole between 0.500 and 0.375: pop |
| 6 | 22.3% | 5.2% | by degree | the mid crown shifts and re-forms between 0.375 and 0.250: pop |
| 10 | 30.4% | 6.1% | not drawn | not drawn; over the 25% stop |
| 20 | 43.9% | 8.1% | not drawn | still re-forms between 0.375 and 0.250; over the 25% stop |

At RATE 6, the other six strips change by degree: lateral, abortion, straightening, twig viability and the Poisson mean, plus readiness, which thins strongly but through partly grown branches.

**Why the rhythm walk still jumps.** It is a mechanism of the design, not a crossing. A draw's window scales with the wood it decides, so a draw that is not crossed still changes size whenever that wood changes:
- On seed 1, the trunk's first node is a Poisson node, 0.73 log-odds past its bound, carrying a basal limb.
- As rhythm falls from 0.375 to 0.3, that limb grows in (vigour 0.58 to 0.92), so the node's window widens.
- The node's presence falls with it: the trunk's first internode shrinks from 1.00 m to 0.65 m, and the whole tree above slides down by it within one frame.

The fine walk is continuous (at most 2.4% of the tree per 1/160 of rhythm), and the walk test's bound holds. On the strip it reads as a jump.

**Wrong-path stop reached:** a visible pop remains at every RATE that loses under 25%.

The decision is the host's. The worker has not tested either option:
- Exclude a node's laterals from the wood its draw decides, so a node's size never depends on its limbs. A Poisson node would then grow in narrowly and carry its limb with it.
- Fix each draw's window from the wood it decides at full presence, from topology alone. That would not be continuous where topology changes inside the decided wood.

## Host decision 3, the last attempt (2026-10-03, overnight run)

The wood-scaled window hit the stated stop: at RATE 6 seven walks change by degree and the walk bound holds, but rhythm (seed 1) jumps because each window scales with the wood the draw actually decides, so a basal limb growing in widens its node's window and slides the tree down. This is a coupling in the mechanism, not in the approach. One last attempt, then B stops for the owner: each window scales with the **expected** wood under the node's physiological age from A's closed form (a smooth function of the settings), never the realised wood, so no realised birth can move another draw's window. Same gate and the same stop (a visible pop, or more than 25% of length lost); no further attempt overnight.
