# fn-206 task 1: the shared chain and the continuous forms (worker, 2026-10-05)

A dispatched worker wrote this. It records what was built and stops at one design question (section 4). Nothing in section 4 is decided.

## 1. What is built

Each item is its own commit on `fn-196-tree-space-d-the-date-palm-as-a-point`, in this order. The renumbering stands apart from the continuous forms, for the merge with phase E's work on `grow.rs` and `species.rs`.

| Commit | What |
|---|---|
| `29d416ad` | The oak's round-4 values from fn-195 brought in (`oak.rs`, `space_oak.rs`). Its preset's material rows stay as this branch has them. |
| `a00ea2ce` | An age of lifespan 0 is passed through. The seed, laterals, continuations, woken buds, the closed form and `Living` all resolve to the next age that has a lifespan (`tests/chain.rs`, red on the base). |
| `56ce0d3b` | **The renumbering:** the beech, spruce, oak and palm moved onto the 15-age chain (`chain.rs`), as PROPOSAL.md section 2 laid out. Every phytomer's tip, radius and scale is bit-identical to the base at ages 10, 40 and 80, seeds 1 and 7, for all four. |
| `eee9664b` | **The continuation share** (host decision 2) replaces `next`. Continuations are keyed by their age on the chain, `Key::onto` (decision 1). |
| `50b0db8b` | **Zone roles** (decision 3): 0 bare, 1 medial, 2 top, 3 spare. The spare stands between the medial nodes and the top. |
| `e537e827` | **Buds per node** as a real number. |
| `8253402a` | **Uniform node bounds** as real numbers. |
| `06cb9a0b`, `3473cdf2` | **The shedding delay** as a real number of cycles. |

### Decision 2: why a `continuation` share, not abortion

The abortion draw is skipped on a lifespan's last unit (`grow.rs:252`, `if apex.units < state.lifespan`). Its hazard also rises with the units grown (`species.rs:403`, `abortion_at`). Stating an exact stop at the end of life through it would need a special case inside it. So the end of a lifespan is one draw against the age's continuation:
- **Move on:** the continuation is grown in by its lead, in the same way abortion's persistence is.
- **Stop:** the apex stops, and its relay may take over, as `next: None` did before.

Ages passed through scale whatever reaches the next age by their own continuations. In the closed form and `Living`, the move is weighted by the share and the stop and relay by its complement.

### The oracle

A's oracle passes, counts and signatures matching the simulator exactly. The chain has no loops, so the simulator's self-renewing PAs (a PA whose transition is itself, in `alternate-four` and `alternate-fixed-loop`) are unrolled in the test adapter into consecutive ages. Counts and signatures fold back onto the simulator's PAs (`tests/common/mod.rs`, `ages` and `fold`).

### R2: walk tests, all within the bound of 30

| Setting | Steepest step per unit |
|---|---|
| `continuation`, every PA | 0.81 |
| `zones[z].buds`, 1 to 3 | 14.6 |
| `zones[z].nodes.max` and `.min`, across two nodes | 14.6 |
| `states[2].shedding`, 0 to 3 cycles | 0.12 |

The jump checks pass for each. `cargo test --profile ci -p telperion-space`: 69 passed.

## 2. R1: look before and after, seeds 1 and 7

Before is the base, before any fn-206 change; after is HEAD. The stills are in `raw/before/` and `raw/after/`, and `raw/cmp-<species>-<age>-<seed>.png` puts them side by side (before left; whole above bare). Every comparison was viewed.

| Still | RMSE, whole | RMSE, bare |
|---|---|---|
| beech, 80 years, seed 1 | 0.122 | 0.105 |
| beech, 80, seed 7 | 0.116 | 0.103 |
| spruce, 80, seed 1 | 0.094 | 0.077 |
| spruce, 80, seed 7 | 0.084 | 0.069 |
| oak, 80, seed 1 | 0.150 | 0.093 |
| oak, 80, seed 7 | 0.161 | 0.108 |
| palm, 50, seeds 1 and 7 | 0 | 0 |

**What the stills show:**
- **Palm:** identical. It has no continuation and no lateral.
- **Spruce:** reads unchanged: the same narrow cone, tiers and skirt at both seeds, with the boughs in slightly different places.
- **Beech and oak:** each reads as a different individual of the same species. The beech at seed 1 is a rounder, narrower dome (18 m across against 22 m); at seed 7 it is the same spreading dome with a low limb out to the left. The oak at seed 1 is now spreading and flat-topped where it was narrow and tall; at seed 7 it has a dense crown and a long level limb. Both trees' main axes move from trunk to fork to leader to limb through continuations, whose draws decision 1 re-keys.

**The law is unchanged.** Twelve seeds at 40 years, on the base and on HEAD, give the same means within their spread:

| Species | Phytomers, mean (sd), base → HEAD | Width, m | Height, m |
|---|---|---|---|
| Beech | 133,450 (60,971) → 137,953 (40,250) | 7.24 → 7.21 | 10.00 → 10.05 |
| Spruce | 1,135,996 (44,338) → 1,150,965 (55,137) | 7.41 → 7.26 | 11.87 → 11.84 |
| Oak | 369,111 (119,816) → 399,086 (87,622) | 8.59 → 8.47 | 13.12 → 13.11 |

So the changes in look are new individuals drawn from the same law, as decision 1 expected. The host judges whether that counts as unchanged in look.

One knock-on: at 80 years, seed 7, the beech grows 10 to 20 million phytomers counting the shed ones (5 to 10 million before; 2.17 million kept against 1.69 million). `tests/form.rs`'s budget for the beech's sheet was raised from 10 to 20 million, the stills' own budget.

## 3. Not done: R3, and fractional lifespans

R3's strips walk every setting between two species. The four species' lifespans differ age by age: the beech's trunk is 9, the spruce's 35, the oak's 11. A whole-number lifespan steps, and every step moves an age's end, and with it everything after it, by a whole cycle. That would be a pop at every step of the strip. The host's design gives a lifespan a fractional last unit, "the share machinery of fn-202". Building that raised the question below, so I stopped before it.

## 4. The design question: a fractional lifespan's last unit

All checked in the code:
- Each growth unit's draws are keyed by the count of units its axis has grown in its age: `Key(lineage).child(units + 1)` (`grow.rs:204`).
- A continuation is a new axis that grows its first unit in the next cycle (`grow.rs:282`, `sprout`; `:293`).
- The abortion hazard counts units grown (`species.rs:403`).
- The closed form counts whole units: `units_left` (`closed_form.rs:39`).

Take a lifespan L = n + f, its last unit grown at the share f. That alone does not stay continuous. Whichever way the next age starts, one end of the fraction disagrees with a whole lifespan:

1. **The next age starts the cycle after the partial unit.** As f → 0 it starts a cycle later than at L = n, so everything after the age jumps by a cycle.
2. **The next age starts in the same cycle, its first unit at the share 1 − f,** as a woken bud's partial first unit does (`grow/wake.rs`, `dormant.rs`, `First`).
   - As f → 0 this agrees with L = n.
   - As f → 1 the next age's first unit is vanishing, but it is still counted. Its later units are numbered one higher than at L = n + 1, so their keys `child(units + 1)` all move: every draw of the age, and of all it bears, changes at once. That is a pop.

I see one design that is continuous at both ends. It is not chosen:

- (a) Each age's last unit is grown at the share f, and the next age's first unit in the same cycle at 1 − f, as a woken bud's first unit is.
- (b) A growth unit's draws are keyed by the cycle it grows in, from its lineage, not by how many units its axis has grown. A unit that appears at a vanishing share then renumbers nothing.
  - A relay keeps drawing what its apex would have, because it carries the lineage and the cycles run on.
  - This changes every species' draws again, so each is re-judged on look as under decision 1.
- (c) The abortion hazard counts the real time spent in the age, not the units, so it does not step when a unit appears.
- (d) The closed form and `Living` give a continuation a partial first unit, through `First` with a fixed share, as they do for a woken bud. A dormant bud ages through real lifespans.

Questions for the host:

1. Is (a) to (d) the design, in particular (b), re-keying growth units by cycle?
2. If not, which other way should a lifespan between whole cycles end?

The closed form counts a partial unit's phytomers whole, as it does for a woken bud. The presence windows it feeds therefore move by a unit's expected wood where a fraction appears. That small step is already accepted for woken buds.

## Host decisions 6 and 7 (2026-10-05)

6. **R1: "unchanged in look" means the same species law, not the same individual.** The host viewed `cmp-beech-80-1.png`: the same habit, dome and trunk, in a different individual. The 12-seed statistics match within their spread, so the change is accepted under AGENTS.md, "Generator evolution". The beech's and the oak's seed sheets change individual; the beech's pass stands, as it still reads as itself. The host checks the full five-seed beech sheet once fn-206 is done.
7. **A lifespan between whole cycles: (a) to (d) of section 4, together.** (b), re-keying growth units by the cycle they grow in, is the change that makes every draw independent of lifespan, so a fractional lifespan renumbers nothing; taking it now means the species are re-judged once, not twice. Then a walk test of real-valued lifespan, red first on the count-keyed form, and A's oracle still exact.

## Built after decisions 6 and 7 (worker, 2026-10-05)

Each item is its own commit on this branch, in this order.

| Commit | What |
|---|---|
| `f1086066` | A fractional bud place grows at its share. A uniform node's lead is ln u - ln bound. The log-odds leads were steep where a draw lies near 0 or 1. |
| `9480b8ac` | (b) Growth units are keyed by the cycle they grow in. Draw-bound fixtures were re-fitted: the lever reference was recomputed at fn-200's commit on a key-independent tree; the sag fixture moved to a new seed. |
| `44168c44` | (a), (c), (d): the schedule (`schedule.rs`). |
| `b09f2034` | `blend(a, b, t)`, the point between two species. Every axis is keyed by its age from its bud's key; without that the walk jumped as the spruce's seedling ages appeared under the beech's seed. |
| `5f6dbc1c` | Two continuity fixes. A relay carries on its axis's running direction and pull, by 1 - blend. Epitony fades near upright. |
| `81af1d93` | Tests: fixtures re-fitted to the new keys, the species walk, and a jump check that splits one level deeper. |
| `50b0bd6b` | The walk example (`space_walk`). |

### What (a) to (d) are, as built

- **(a)** An age's last unit grows at the share its lifespan leaves. What carries the axis on grows its first unit in the same cycle, at the rest of it.
- **(c)** The abortion hazard counts real time. It runs over the lesser of a unit's share and the next unit's share, so it vanishes with a unit that is all but gone, whether that unit is the first or the last.
- **(d)** The closed form and `Living` follow the same schedule. A partial unit counts its phytomers whole, at its share of their length. Dormant buds age through real lifespans.

The walks found three more steps, now made continuous:
- A relay's first unit is whole: every bud grows at least one unit, and without this a relay of an age just short of spent grew differently from one of an age spent.
- An apex alive at the tree's age lives on by the share of its age left.
- Shedding drops an axis once its time-weighted fade reaches nothing, not on whole cycles.

### R2

The lifespan walk is red on growth units keyed by count, with jumps at 1.015, 3.02 and 5.2. On keys by cycle it passes, steepest 3.4 per unit. A's oracle is exact. 70 tests pass. The jump check now splits to a 4096th of a step. The three steps it flagged at 512 were bisected to 1e-13 and are continuous: a draw near 0 or 1 grows its element in over about a 2000th of a step.

## R1 after (a) to (d): look and law

`raw/after2/` holds the stills and `raw/cmp2-*.png` sets them against the base; every comparison was viewed. Spruce seed 1 is missing: it failed four times on wgpu out-of-memory with other workers on the GPU.

| Still | RMSE whole | RMSE bare |
|---|---|---|
| beech 80 years, seed 1 | 0.123 | 0.107 |
| beech 80, seed 7 | 0.118 | 0.107 |
| spruce 80, seed 7 | 0.108 | 0.086 |
| oak 80, seed 1 | 0.154 | 0.085 |
| oak 80, seed 7 | 0.168 | 0.109 |
| palm 50, seed 1 | 0.100 | 0.029 |
| palm 50, seed 7 | 0.106 | 0.028 |

- **Beech:** a domed beech at both seeds, new individuals. Seed 1 is narrower (18 m against 22 m).
- **Spruce:** the same cone.
- **Oak:** reads as an oak, more spreading. Seed 1 has a long low limb.
- **Palm:** the same; its stem's own draws are re-keyed.

Twelve seeds at 40 years, base against HEAD:

| Species | Phytomers | Width, m | Height, m |
|---|---|---|---|
| Beech | 133k → 148k | 7.24 → 7.15 | 10.00 → 9.91 |
| Spruce | 1,136k → 1,177k | 7.41 → 7.31 | 11.87 → 11.73 |
| Oak | 369k → 377k | 8.59 → 8.76 | 13.12 → 13.27 |

The beech and oak match within their spread. **The spruce's height drops 0.14 m, 2.5 standard errors:** a law change, not a new draw. The likely cause is that a uniform node count's nodes now carry their draws' leads (`8253402a`, `f1086066`), so nodes just past their bound grow in partly, where before every node within whole bounds stood whole. Continuity in the bounds requires this.

**The beech's five seeds:** `raw/beech5-sheet.png`, final21 (top) against the new individuals (bottom), at seeds 1, 2, 3, 4 and 7, bare at 80 years. The same domed habit and forked crown on one trunk. The new trees carry more near-level low limbs at seeds 2 and 3. The lighting differs: final21 predates the sun moved behind the camera.

## R3: the strips, and two design questions

`raw/walk/strips.png` holds the three strips: beech to spruce (top), spruce to oak, and oak to palm. Each has 9 frames at t = 0 to 1 in eighths, seed 1, age 60, bare, on the beech's dressing, all from one camera. The 0.875 beech-to-spruce frame failed on GPU out-of-memory. The automated walk finds no jump between species. The look is another matter:

- **Spruce to oak:** a gradual turn from cone to spreading crown, with no pop.
- **Beech to spruce:** collapses. From 14.3 m at t = 0, the tree is a 2.6 m bush at 0.125 and stays a bush until 1.0, where it is a 16.9 m spruce.
- **Oak to palm:** from t = 0.125 on, an unbranched, thick, conical column 22 m tall.

Both failures come from what is mixed, checked by growing the midpoints and printing their ages.

**Q1. The settings of an age a species does not use.** `chain.rs` `passed()` gives an unused age lifespan 0, `Form::default()` (elevation 0, tropism 0), internode 0.04 and a zone of no nodes. The host's design set only the laterals at 0 and the lifespan at 0. A blend mixes these placeholders in as settings. At t = 0.125 between the beech and the spruce, ages 0 and 1 are at elevation 0.20 rad, from 0 and pi/2. The seed starts in them, growing almost level.

Options:
- (a) An unused age copies its nearest used age's settings: the next for ages before the species' first, the previous otherwise. Its lifespan stays 0 and its laterals 0, so it is still passed through, but grows in looking like the axis it joins.
- (b) A blend treats an age one side does not use as that side's neighbour.

**Q2. The scale a lifespan is mixed on.** The species write "never moves on" as lifespan 1000: the spruce's crown and branch, the palm's stem, the oak's limb. A linear mix sends any midpoint to about 1000 t. At t = 0.125 the oak's 11-year trunk lives 134.6 years, so it never forks, and the walk is a column at once. Between the beech and the spruce the leader lives 160 years and branch age 9 lives 134, so branches never die and grow 1.2 million phytomers in a bush.

Options:
- (a) Mix a lifespan as the shedding delay is mixed, through 1 / (1 + L), or on a log scale.
- (b) Write "never" as a continuation of 0 with a whole lifespan rather than as 1000.
- (c) Both.

Mixing the pipe (the palm's 0.297 m against the oak's millimetres) on a log scale would answer the column's girth the same way.

The gate, the Codex review and `done` wait on these.

## Host decisions 8 to 10 (2026-10-05)

8. **Unused ages, a canonical form.** An age a species does not use copies the settings of its nearest used age on the chain, with no lifespan and no laterals, so it is passed through and invisible. It is a normalisation applied when a species is built, a pure function, not a rule inside the blend.
9. **The scale settings mix on.** "Never moves on" is continuation 0 on a finite whole lifespan; the 1000 sentinel is removed everywhere. Every positive magnitude mixes on a log scale: lifespans (log1p or 1/(1+L), keeping 0 reachable), internodes, pipe radii, rates, delays, sag. Angles, probabilities and shares mix linearly. The scales stand in one table beside the settings' definitions, and the blend is one function over that table. A setting added later must declare its scale or the build refuses.
10. **The spruce's 0.14 m height change (2.5 standard errors) is accepted:** the price of continuity in the node bounds.

### As built

- `6974cf11`: the sentinel removed. The spruce's crown and branches live 300 years, the oak's limbs 500, the palm's stem 150, each about the tree's own life, then they stop.
- `e85504c0`: `Species::canonical`, applied as the chain builds a species. The nearest used age is the earlier one on a tie.
  - The table is `species/scale.rs`. A magnitude mixes as exp(lerp(ln(a + floor), ln(b + floor))) - floor, where the floor is the least magnitude that matters: 1 cycle for a lifespan, 1 mm for an internode, 0.01 mm for a pipe. With a floor, 0 stays reachable.
  - The shedding delay, which may be infinite, mixes by the share 1 / (1 + d) it keeps.
  - `settings` names every field of an age, so a new field does not compile until it is listed there, and a listed field with no scale is refused (tests in `scale.rs`).
- `b172c2d4`: one more step the species walk found. A relay stood at a node count along its unit, so a node growing in from nothing moved it by a node and turned its whole subtree (beech to spruce, t = 0.284). It now stands at its share of the unit's scaled length.

### R3 strips after decisions 8 and 9 (seed 1, age 60; `raw/walk2/strips.png`)

- **Spruce to oak: by degree.** Heights run 16.9, 16.9, 17.3, 17.9, 18.5, 18.7, 19.1, 18.5 and 17.6 m. The whorled cone thins into the oak's open crown with no pop or collapse.
- **Oak to palm: one pop at the start.** At t = 0.125 the oak's crown is gone and a bare 10 m pole stands. From there the pole thickens and rises (10.5, 11.4, 16.3, 21.6 and 22.6 m). At 0.875 it is thicker than the palm's own stem at 1.0, which then thins.
- **Beech to spruce: collapsed.** The frames at 0.125, 0.25, 0.5 and 0.875 exceed the 20M-phytomer budget. The frames that render (0.375, 0.625, 0.75) are 2.4 to 4.0 m bushes on the ground.

What grows at the collapsing midpoints (`grow` at age 30, seed 1):

- **The stem's scale compounds partial presences.** A draw near its bound grows its element at a presence below 1 (`presence.rs`), with a window up to 2 log-odds wide for wood the size of the crown. A relay's or continuation's base scale is its parent's base scale times its own vigour, and that vigour carries the draw's presence (`geometry::scale`). The two ends are whole because their relay and continuation are 0 or 1. Inside the walk those settings sit between (relay 0.5, abortion 0.25 on the beech-spruce trunk at t = 0.5), so every stem stop falls inside a window, and the stem's relays come in at vigour 0.05, 0.02 and 0.04. By age 30 the leader's phytomers stand at scale 0.000; the tree is 0.23 m tall.
  - Beech to spruce at t = 0.0625: one relay at vigour 0.10 on the trunk takes the 8 m tree to 1.2 m.
  - Oak to palm at t = 0.125: the trunk's move to LOW_FORK goes from certain to 0.88 (the palm never moves on), and the oak's limbs from relay 1 to 0.88. The crown's scaled length falls from 584,571 to 920.
- **The budget counts faded wood.** At t = 0.0625 the beech-spruce tree already grows 277k phytomers against the beech's 32k, most of them faded to nothing: the spruce's branchlet and spur ages grow in at a small presence beside the beech's twigs. At age 60 the midpoints pass 20M.
- **A test of a stake-weighted presence (not kept).** A stop or persist presence of 1 - stake·(1 - p) instead of p brings t = 0.0625 and 0.125 back to 6.1 m and 7.3 m against the beech's 8.0 m. The middle still collapses (1.0 m at 0.5), because the stake reaches 0.5 there and the halves still compound down the stem.

Not decided here: how presence should work along a chain of stops (host question Q3 in the report).

## Host decision 11 (2026-10-05): a stop all but made grows both outcomes

As built (`25232b9c`, the stop and move code in `grow/stop.rs`):

- **Abortion.** An apex that all but aborted carries on at its own presence plus that of the relay it would have had: relay presence m + (1 - m)·p.
  - At the bound, the relay is laid as the apex's continuation (blend 0) and draws what the apex would, so the two outcomes are one piece of wood there.
  - A stop by abortion relays whole: its stop presence is 1.
- **Move.** An apex that all but stopped moving on grows its continuation at p and its relay at (1 - p)·m, both children of its last unit.
- **Viability.** A failed unit is unchanged: its relay misses that unit, so it still grows in from nothing past the bound.
- **Sleeping buds** wake on every axis that carries their bearer on, each at its presence.
- **The abortion hazard.** A relay carries the time its axis has grown into its hazard, in the grower and in both closed forms. Without this, a relay laid as the continuation restarted its hazard, and `abortion_rise` jumped at the crossing.
- **The walk measure** places a lineage that both outcomes grew where its chains stand, weighted by their lengths. A continuation that fades out at the bound and the relay that takes over carry laterals of one lineage.
- **Species exactness.** Relay and continuation are each 0 or 1 in all four species, so neither outcome is ever borderline. At age 30, seed 1, the four species are identical in height, phytomer count and scaled length before and after.
- **Red first.** In `a_stop_near_its_bound_fades_nothing_its_relay_carries`, the walk species' stem at relay 0.5, swept over abortion, dipped to 0.72 of its rim at seed 2 before the change. It now never dips (seeds 1 to 4).
- **Effect on beech to spruce at age 30, seed 1:**

  | t | Height before (m) | Height after (m) |
  |---|---|---|
  | 0.0625 | 1.2 | 6.9 |
  | 0.125 | 1.9 | 8.4 |
  | 0.25 | 1.8 | 3.6 |
  | 0.5 | 1.0 | 1.0 |

  The beech is 8.0 m at age 30.

### What still collapses at the middle of beech to spruce

The stem now loses wood only where it actually dies. The leader dies on a stop that its relay draw fails:

- The beech's trunk aborts at 0.5 a year and always relays.
- The spruce's never aborts.
- The mix walks both settings linearly, so a fatal stop comes at abortion × (1 - relay) = 0.5·t·(1 - t) a year.
- That rate is 0.125 a year at t = 0.5, so the leader's chance of surviving 30 years is about 2 %.
- It is 0.055 a year at t = 0.125, a survival of about 19 %.

The relay that does take over grows in over its draw's window, with presences of 0.12, 0.06 and 0.06 down the stem at t = 0.5. This is the metric (abortion and relay mixed as probabilities), not presence compounding. Whether the mix should keep the leader is a question for the host.

## fn-199's four relay-continuity findings, checked against the code (2026-10-05)

Probes: `raw/fn199-probes.rs`, walk species, seeds 1 to 3, the walk's refine (4 levels × 8 splits).

| Finding | Status | Evidence |
|---|---|---|
| R1: relay probability 1 crossing survival | **Open** | Limb viability walked with relay 1: jumps of 0.09 to 0.14 of the tree that do not shrink when split. A failed unit's relay misses the unit, and with relay 1 no window fades it in. Decision 11 left this path as it was. |
| R2: relay position in whole nodes | **Resolved** by `b172c2d4` | New test `a_node_growing_in_moves_no_relay`, red at `e85504c0` (a jump of 0.053 at seed 1) and green now. |
| R3: a relay that has barely left a bent lateral | **Open** | Limb abortion walked with relay 1, straightening 1, relay_at 0.5 and epitony 0.6: jumps of 0.007 to 0.05. The same at `546d4370`, before decision 11. At seed 1 the crossing is a relay whose first unit fails (an axis of no phytomers) and whose own relay is then laid from that empty axis's base and frame. |
| R4: relay girth on a discrete parent node | **Open** | The trunk's radii walked over relay_at (relay 1, abortion 0.4): steps of 0.06 to 0.35 of the largest radius that stay the same through 30 bisections. `girth.rs` puts a blended relay's section on one node. |

fn-199 is not covered by fn-206; R1, R3 and R4 stay for it.

## Species stills after decisions 9 to 11 (seeds 1 and 7; `raw/after3/`, `raw/after3-sheet.png`)

- **Oak, palm and spruce:** pixel-identical to R1's `after2` stills (RMSE 0), and their logged node, leaf and size figures are identical. Spruce at seed 1, missing from R1, now renders: 21.6 m, 4.64M nodes.
- **The 300, 500 and 150 lifespans change nothing at these ages.** Each is past the tree's age.
- **The beech's bare structure is identical at every commit from `f41ba376` to HEAD,** at seeds 1 and 7 and age 80. Axes, phytomers, scaled length and x extent were checked at each fn-206 commit since R1.
- **The beech's still nonetheless differs from `after2`.**
  - RMSE is 0.10 at seed 1 and 0.11 at seed 7.
  - Nodes rise by 9 and 19. The seed 7 crown measures 24.5 × 20.4 m against 21.9 × 21.9 m.
  - So `after2` was rendered from a build older than its commit.
  - Viewed side by side (`raw/cmp3-beech.png`), the tree is the same in character: the crown outline differs slightly and the trunk and form are unchanged.

## R3 strips after decision 11 (seed 1, age 60; `raw/walk3/strips.png`)

- **Spruce to oak:** unchanged and by degree, from 16.9 to 19.1 to 17.6 m.
- **Oak to palm:** unchanged.
  - The pop at t = 0.125 remains: the oak's crown goes and a bare 10 m pole stands.
  - The cause is outside decision 11: neither species relays the trunk (relay 0 in both), so no relay outcome exists to grow. The trunk's move to LOW_FORK walks from certain (oak) to never (palm), so it is 0.88 at t = 0.125. That move decides the whole crown, so its draw's window is the widest (2 log-odds), and seed 1's draw lies inside it.
  - From there the pole thickens up to t = 0.875, where it is thicker than the palm's own stem at 1.0.
- **Beech to spruce:** at age 60 the frames that render are unchanged: 2.4 m at 0.375, 2.5 m at 0.625, 4.0 m at 0.75.
  - At age 30, decision 11 restored the early midpoints (above). By age 60 the leader has died: the fatal-stop rate is abortion × (1 - relay), so its survival over 60 years is about 4 % at t = 0.125 and nil at 0.5.
  - The frames at 0.125, 0.25, 0.5 and 0.875 still exceed the 20M budget. That budget counts phytomers grown, the shed ones included. After shedding (budget raised for the count, age 60, seed 1):

    | t | Phytomers after shedding | Faded below scale 1e-3 | Where |
    |---|---|---|---|
    | 0.125 | 12.5M | 9.5M | Mostly ages 9 to 14, the branch-to-short twig system: 2.0M branch, 2.9M shoot, 6.5M short |
    | 0.5 | 6.6M | 5.9M | The leader, limbs and boughs (ages 4, 6, 7) all faded to nothing |
    | 0.875 | 7.4M | 2.6M | Mostly the spruce's branchlets, spurs and shoots |

    Nothing was weighted or dropped.

Remaining question for the host: the stop rate abortion × (1 - relay) that the linear mix gives at the midpoint (above).

## Codex review (task 1)

**Round 1, NEEDS_WORK, three findings, all fixed in `ddf13154`, each red first:**
1. A sleeping bud chose its stage from whole slept years and then added its waking fraction, so it could wake in a stage it had outlived, at a negative growth share. It now ages through its whole real sleep. The closed form splits each waking year at the stage boundaries inside it.
2. The closed form weighed a fractional bud place as an existence probability, while the grower grows it whole at its share of the size. Places are now counted whole and sized by their mean share.
3. `blend` read zones before validating its inputs. It now validates both species first.

The four species are unchanged at age 30, and the beech at age 80 (seed 1).

**Round 2, NEEDS_WORK, two findings:**
1. **A bud that wakes in a stage it enters within the year lost that stage's start offset in the closed form. Fixed.** The stage time at the year's start is now kept below zero, which runs the first unit's schedule and its hazard exactly as the grower's do. Test: `a_bud_waking_past_a_stage_boundary_grows_its_expected_counts`. Before the fix, PA 3 in cycle 3 grew 3.53 phytomers against a formula of 2.21.
2. **Open, a design question for the host.**
   - **What diverges.** Under decision 11, an apex that all but stopped moving on grows its continuation and also a separate relay axis at a presence below 1. The grower's counts count every phytomer whole. The closed form counts a move as either a continuation or a relay, never both.
   - **Repro.** Run standalone: two ages of lifespan 1, the first with continuation 0.5 and relay 0.5. At age 2 the first age averages 0.404 phytomers in cycle 2 over 20,000 seeds; the formula gives 0.25.
   - **The extra relay's existence depends on the presence windows.** Its chance is relay · (go − σ(logit go − w)), where w is the window of the wood the move decides, so it depends on the tree's age and the cycle.
   - **Options:**
     - (a) The closed form counts it through the windows. The count walker would then read the expected lengths, and `Living`'s chance that an axis is carried on would be keyed by the absolute cycle.
     - (b) Count tables weigh each element by its presence instead of counting it whole. This changes the counting convention, fractional bud places included.
     - (c) Something else the host prefers.
   - The review's round cap (2) is reached.

## Host decisions 12 and 13 (2026-10-05)

12. A stop's outcome is one three-way distribution: it carries on, relays or dies, as shares that sum to 1. The blend mixes the three shares linearly.
13. The closed form counts the extra relay that a move all but made grows, through its windows.

### As built

- **`7756d957`, decision 12.**
  - Each stop has its own relay share: `relay` after an abortion, the new `relay_ended` after an age that ends and does not move on, the new `relay_failed` after a failed unit.
  - Each pairs with its stop chance as one outcome: abortion, continuation and viability.
  - The scale table marks the six settings as `Outcome` (`OUTCOMES` in `species/scale.rs`). The blend mixes the stopping, relaying and dying shares by value and sets each relay to the relaying share over the stopping one. The species keep their readable form; each sets its three relay shares alike, so all four species are unchanged.
  - **Other categorical outcomes.** Continuation against `relay_ended` at the end of an age, and viability against `relay_failed`, are the same kind of outcome and are treated the same way. A zone's lateral and dormant probabilities, each a share of one bud's PA with the rest bare, already mix linearly as shares.
  - **One more change the species walk forced.** At a relay share of 1 nothing fades a stop, so the relay of an apex that aborted must be the apex's continuation exactly. It now grows the share of its first unit its lifespan leaves, as the apex would. A relay of an age that ended, or of a failed unit, keeps its whole first unit. Before this, beech to spruce jumped 0.53 of the tree at t = 0.62, seed 2.
  - **Red first:** `a_midpoint_between_two_undying_leaders_keeps_its_leader`. At beech to spruce t = 0.5, age 60, the leader stopped at age 30 before the change. It now grows at whole size at the top. A second test, `a_blend_mixes_a_stops_outcomes_as_shares`, checks that halfway the trunk aborts a quarter of its years and always relays.
- **`ae1b50e1`, decision 13.**
  - The extra relay's chance is relay_ended × (go − σ(logit go − w)), where w is the window of the wood the move decides, read from the tree's windows (`Windows::wood_left`, `Windows::borderline`).
  - `Living` now gives the expected number of axes carrying a sleeping bud's bearer on, the extra relay included. Where windows are read, it is keyed by the cycles of the tree's age left. The expected lengths that make the windows read none and keep their memo.
  - The change was about 60 lines across `dormant.rs`, `closed_form.rs` and `presence.rs`, not a deep one.
  - **Red first:** `the_closed_form_counts_the_relay_of_a_move_all_but_made`. Before: 0.40 phytomers grown against a formula of 0.25. Now the engine's mean over 20,000 seeds matches the formula in every cell at ages 2 and 4.

### Beech to spruce at age 60, seed 1, after decision 12

Every midpoint keeps its leader: an apex at whole size at the top.

| t | Height (m) | Phytomers after shedding | Faded below scale 1e-3 |
|---|---|---|---|
| 0.125 | 14.7 | 19.5M | 56 % |
| 0.25 | 15.4 | 12.4M | 39 % |
| 0.5 | 16.3 | 8.3M | 5 % |
| 0.75 | 17.1 | 8.2M | 2 % |
| 0.875 | 16.8 | 8.0M | 0 % |

The two species are 14.2 m and 16.9 m.

- **Every midpoint frame (0.125 to 0.875) now exceeds the strips' 20M budget,** which counts the phytomers grown, shed ones included. The full trees grow where the collapsed ones did not. Nothing was weighted or dropped.

### Strips after decisions 12 and 13 (`raw/walk4/strips.png`)

- **Beech to spruce:** every midpoint is over the budget (above).
- **Spruce to oak:** unchanged and by degree.
- **Oak to palm:** at t = 0.125 the tree is 13.8 m (was 10.2 m), a tall stem with a small crown at its top and a few short side limbs. At 0.25 it is 14.8 m and nearly bare. From 0.375 it is the palm's column.

### What the oak's trunk does at t = 0.0625 and 0.125

Decision 12 does not fix the oak-to-palm pop. The trunk's move to the fork age is not where the crown goes:

- **At 0.0625 the trunk moves on at whole vigour.** At 0.125 it moves on at 0.63: its continuation of 0.875 lies inside the move's window.
- **The crown goes earlier and over a wider stretch.** Its scaled length falls from 584,725 at t = 0 to 54 % by t = 0.01, 16 % by 0.031, 11 % by 0.0625 and 1.3 % by 0.125.
- **Limb-bearing stem nodes go too.** The stem nodes that bear limbs fall from 16 to 5 at 0.0625 (two of them faded, mean scale 0.90) and to 4 at 0.125 (all faded, mean scale 0.63).
- **Why.**
  - The palm has no laterals and no MEDIAL or TOP zones, so the oak's limb probabilities and its single certain whorl node (TOP min = max = 1) walk linearly to nothing.
  - Each draw that decides a limb decides much of the crown's wood, so it fades over a window of up to 2 log-odds (or 2 in ln u for a node). The whorl node, made with chance 1 − t, is faded or gone for about 7.4·t of the draws: 46 % at 0.0625 and 92 % at 0.125.
  - Down the crown's generations these fades multiply.
- **This is the presence windows' width on a crown that one of the two species does not have.** It is not the move-on decision.

## Codex rounds 3 and 4 (after the reset)

**Round 3, NEEDS_WORK, both findings fixed in `45d816cf`, each red first:**
1. A woken bud's first-unit survival was averaged over 1 − x even when its stage ends sooner. `First::of` now averages over the grower's own first unit at each waking time. Before the fix: 0.20 grown against a formula of 0.12.
2. A relay grown in its axis's cycle reused the move that failed there. Before the fix: 0.5 against 0.75.

**Round 4, NEEDS_WORK, both findings fixed in `8722083d`, each red first:**
1. **Round 3's same-cycle key switched draws as a lifespan crossed a whole number** (0.325 m at seed 3). Every relay of an age that ended now draws its stop decisions under its count of such relays, wherever it grows; its zones keep the cycle's key. Before the fix, lifespan 1 and 1 + 1e-9 grew 2.08 m against 2.40 m.
2. **The waking mean sampled a stage's excluded end, where the share is none.** It is now a midpoint rule. Before the fix, a bud woken into a stage at viability 0 counted 0.00013.

**Effect on the species.** The beech changes where relays of ended ages carry it on: at age 80, 89 fewer axes at seed 1 and 213 fewer at seed 7, out of 179,260 and 305,876. The stills differ from `after3` by an RMSE of 0.045 and 0.041. Viewed side by side (`raw/cmp4-beech.png`), the crowns are the same tree with a few twigs placed differently. Spruce, oak and palm are unchanged.

The workspace gate at `170f5ecc` (before rounds 3 and 4) passed: 1061 tests, 0 failed. The space crate passes in full after round 4 (86 tests).

## Host decisions 14 to 16 (2026-10-05)

14. The review counter was reset; Codex is to run until SHIP.
15. **Oak to palm's early crown loss is accepted as inherent, not a pop.**
    - A branching probability's effect compounds through the generations of a branching process. So the crown decays geometrically as the oak's limb probabilities walk to the palm's zero: 54 % of the crown's scaled length is left by t = 0.01, 16 % by 0.031, 11 % by 0.0625 and 1.3 % by 0.125.
    - The walk has no jump; it is steep near the oak end. That is a property of the space's metric for lateral probabilities, not a break in continuity.
    - **Open for calibration once more species exist:** a lateral probability could mix on a scale under which crown volume varies more evenly, for example the expected number of branches per generation. Not now.
    - No window is narrowed, since that would trade continuity for evenness.
16. **The beech-to-spruce strip renders at age 40,** where the midpoints fit the 20M safety bound. One age-60 midpoint is recorded at a 40M cap if it fits in memory.
    - Shedding before growth (F2, fn-210) is the proper remedy: 56 % of the t = 0.125 tree at age 60 is faded wood.

## Codex rounds 5 and 6 (after the second reset)

**Round 5, NEEDS_WORK, both findings fixed in `70dcb869`, each red first:**
1. A bud that reaches its age or stage through a continuation share grows whole at that share of its size. The closed form now counts it whole and sizes it by the share. Before the fix it counted half the phytomers.
2. A bearer whose age ended within a sleeping bud's waking cycle now bears the bud as far as it lived past the waking, and `Living` counts it. Before the fix, lifespan 3 ± 1e-9 grew 0 m against 0.48 m.

**Round 6, NEEDS_WORK, one finding fixed in `bb3dba5c`, red first:** the bearer-end limit was bypassed in the tree's last cycle. In that cycle a bearer alive at the tree's age now keeps the bud as far as it lives on past it (`Grower::living`). Before the fix, lifespan 2 ± 1e-9 at age 3 grew 0 m against 0.48 m.

The four species are unchanged by rounds 5 and 6 (beech, spruce and oak at age 80, seeds 1 and 7; palm).

### Beech to spruce at age 40, seed 1 (`raw/walk5-40/strip-beech-spruce-40.png`, `raw/walk5-40/mids40.png`)

- **Every frame renders within the 20M bound.**
- **Heights:** 10.1, 10.9, 11.4, 12.0, 12.2, 12.5, 12.7, 12.5 and 11.8 m.
- **Read, no pops:**
  - t = 0: the beech, a forked broadleaf.
  - t = 0.25: a single-stemmed open tree with spreading branches.
  - t = 0.5: a taller stem with tiered, more level branches.
  - t = 0.75: a narrow, spruce-like spire.
  - t = 1: the spruce.
- **The leader is kept throughout.**

### One age-60 midpoint at a 40M cap (`raw/walk60-40M/`)

Beech to spruce at t = 0.5, age 60, seed 1, rendered within 40M in memory: a 16.5 m single-stemmed tree with its leader at the top and tiered branches, between the beech's 14.2 m and the spruce's 16.9 m.

## Codex rounds 7 and 8 (after the third reset)

**Round 7, NEEDS_WORK, both findings fixed in `c7548eb9`, each red first:**
1. **An age's end was drawn under the key of the cycle it fell in,** so a lifespan crossing a whole cycle changed the move draw: at continuation 0.5, lifespan 1 ± 1e-9 grew 1 m against 1.63 m.
   - The end is now drawn once for the age (key `END`), apart for each relay of an age that ended.
   - Its windows read the wood from the moment the age ends: between the cycle's and the next's, by log, in the grower and the closed form alike.
2. **An age of no lifespan with continuation below 1 passed its buds on at that share of size,** while an age all but as short decides by a draw. Such a species is now refused: an age of no lifespan is passed through and moves on, as the canonical form makes every unused age.
   - Also: the walk measure places a lineage whose copies have no length yet by how present each stands.

The four species are unchanged by round 7.

**Round 8, NEEDS_WORK, one finding fixed in `3fecc5a1`, red first:**
- **A continuation took its parent's last phytomer's heading,** pulled at that phytomer's middle, so a straightened lateral whose last unit all but vanished turned its next age by the whole pull: 0.92 m at the tip.
- It now runs on in the running direction and side its parent ends on, where the pull has faded to none, as a relay at the bound does.
- **This changes how every age change is laid** (`raw/after5/`, `raw/cmp5.png`):
  - Palm: identical.
  - Oak: RMSE 0.0006.
  - Beech: 0.021 and 0.038.
  - Spruce: 0.040.
  - Viewed side by side: the same trees. The beech's crown outline differs slightly, and the spruce's lower boughs sit a little differently; character, height and form are unchanged.

## Host decision 17 (2026-10-05): triage of the remaining Codex rounds

- **Blocks fn-206:** a finding that changes a species' look, a strip, the oracle, or a count the closed form owns.
- **Deferred:** a continuity defect only at an exact boundary that changes no species or strip. It is recorded here and folded into a follow-up beside fn-199.

## Deferred to continuity hardening

From Codex round 9 (`raw/review-10.json`, also in `.flow/review-deferred/`). Both are introduced, both sit only at a whole-number lifespan ± 1e-9, and neither changes a species (all four use relay_ended 0 or 1 at ages whose relays never vanish at such a bound) or a strip.

1. **A vanishing end-of-age relay applies its full insertion angle** (`grow/stop.rs:113`).
   - Repro: one state, one certain node per unit, internode 1, viability 1, continuation 0, relay_ended 1, relay_at 1, insertion 0.7, divergence 0, shedding infinite, at age 2.
   - Lifespan 1 − 1e-9 inserts a tiny relay that turns the next whole unit twice; 1 + 1e-9 turns it once. Tips are about (0.985, 0, 1.170) against (0.644, 0, 1.765), a 0.686 m jump.
   - Codex's suggestion: let a relay's departure vanish with its growth share.
2. **The same vanishing relay counts towards `ended_relays`** (`grow.rs:223`), so the next whole unit's survival draw changes.
   - Repro: the species above with insertion 0, viability 0.5, relay_failed 0, seed 244. Below lifespan 1, cycle 2 uses relay count 2 and draws 0.7597 (it fails); above, count 1 and 0.0176 (it survives). Scaled length jumps from 1 m to 2 m.
   - Codex's suggestion: keep later decision keys when a relay's share vanishes.

## Close (2026-10-05)

- **Codex: SHIP** (`raw/review-11.json`), after rounds 1 to 9 above. The two boundary findings are deferred under decision 17.
- **Workspace gate** at `fd7d9ffe`: 1071 passed, 0 failed (`raw/gate3.txt`).

### Final strips (`raw/walk6/strips.png`), seed 1

Viewed; no pops in any of the three.

- **Beech to spruce, age 40:** 10.1, 10.9, 11.4, 12.0, 12.2, 12.5, 12.7, 12.5, 11.8 m. Unchanged in character from `walk5-40`, which the host passed: the forked beech by degree to the spruce's spire, the leader kept throughout.
- **Spruce to oak, age 60:** 16.9 to 19.2 to 17.6 m. The cone opens into the oak's crown by degree.
- **Oak to palm, age 60:** 17.6, 17.9, 20.0, 21.1, 22.1, 22.6, 22.6, 22.6, 22.4 m.
  - The crown's early loss is decision 15's accepted steepness: 54 % of its scaled length left by t = 0.01, 1.3 % by 0.125.
  - Since round 8 the oak's stem runs on in its own direction through each age change, so from t = 0.125 the stem now carries on as a tall leader with a few short limbs and a turned tip, rather than stopping at 13.8 m.
  - It thickens into the palm's column from t = 0.5.

## Host decision 18 (2026-10-05): oak to palm's stem girth, a calibration note

On walk6's oak-to-palm strip the stem swells into a tapered cone thicker than the palm's column at t = 0.75 to 0.875, then narrows at t = 1. The change is continuous but steep: it comes from mixing pipe-model girth with secondary 0.

Recorded beside decision 15's crown as calibration for later, once more species exist. Not a defect in continuity.

## The port onto master (2026-10-05)

fn-206 brought onto master after phase E (#151), bend_length (#152) and the oak at round 10 (#153), on branch `fn-206-tree-space-one-reference-axis-every-pr`, as one port of the reviewed work (`a003d8fe`). The palm's earlier commits are already on master and are left out.

### What the port added for E

- **The scale table** declares phase E's settings and bend_length.
  - A magnitude on a log above a floor: `leaf_area` (floor 1e-4 m²), `upkeep` (1e-3), `balance_hazard` (0.01) and `form.bend_length` (0.1 m).
  - By value: a response's exponent (`shade_hazard`, `shade_size`, `leaf_girth`), a share (`apical_control`, `retained`) and a balance (`tolerance`).
  - The canonical form copies them with the rest.
- **Allocation (decision 23)** groups each bud by its PA, which is now its age on the chain. Each species age is still one group, so each order keeps its own mean. The groups an unused age would form stay empty.
- **The rough layout** reads each unit's share of its cycle as the final sizes do. It frames relays and continuations from its bearer's running frame and pulls through geometry's `Layer` (`Lift`, `ends`, `handed`), as the final lay does. The full re-lay marks its living apexes as living whole.
- **The dressing's trunk-level ages are renumbered onto the chain:**
  - beech 2, 4, 5
  - spruce 0, 1, 2, 4
  - oak 2, 3, 4
  - palm 2
- **A walk's stem** is the union of either end's trunk-level ages.
- **The oak is master's round 10, light values kept, on the chain.** Its limb's 1000 is the decision-9 500.
- **Master-only tests** (girth, light, bend) are written in the chain's real-valued forms.

### Species against master: seeds 1 and 7, stills (`raw/port-cmp.png`)

| Still, whole | RMSE, seed 1 | RMSE, seed 7 |
|---|---|---|
| beech, 80 | 0.129 | 0.120 |
| spruce, 80 | 0.092 | 0.108 |
| oak, 80 | 0.156 | 0.172 |
| palm, 50 | 0.100 | 0.106 |

Viewed: each species reads as itself, a new individual of the same law as decision 6 allows. The beech is the same spreading dome, the spruce the same narrow cone, the oak the same open, forking crown and the palm the same crown on its stem. The palm moves because its stem is re-keyed by its age on the chain: identical length and height, with the stem's wander drawing a different path.

### The law, 8 seeds at age 60 (`raw/stats-*.txt`)

The port's beech, spruce and palm are identical to fn-206's own branch (every statistic to the digit). The port changes nothing of them; what follows is fn-206's own difference from master.

| Species | Measure | Master, mean (se) | Port, mean (se) | Difference |
|---|---|---|---|---|
| Oak, neutral | axes | 173,123 (20,602) | 171,661 (21,353) | within 0.1 se |
| Oak, neutral | length | 1,033k (126k) | 974k (111k) | 0.4 se |
| Oak, neutral | height | 16.76 (0.22) m | 16.38 (0.39) m | 0.8 se |
| Oak, lit (extinction 0.5) | axes | 129,208 | 129,063 | within 0.1 se |
| Oak, lit | height | 18.32 (0.76) m | 18.12 (0.46) m | 0.2 se |
| Spruce | axes | 591k (5.6k) | 606k (5.1k) | 2.0 se |
| Spruce | height | 17.19 (0.04) m | 16.96 (0.06) m | 3.3 se |
| Beech | axes | 77.9k (9.2k) | 96.7k (6.1k) | 1.7 se |
| Beech | length | 434k (56k) | 575k (49k) | 1.9 se |
| Beech | base radius | 0.310 (0.015) | 0.353 (0.014) | 2.1 se |
| Beech | height | 14.27 (0.15) m | 14.08 (0.25) m | 0.7 se |
| Palm | height, length | 18.60 m, 600 | 18.60 m, 600 | identical |
| Palm | stem spread | 0.041 (0.006) | 0.031 (0.004) | 1.4 se |

- **The spruce's height** is decision 10's accepted change (0.14 m at age 40, 0.23 m at age 60).
- **The beech at age 60** carries about a third more wood than master's, at about 2 se. At age 40 its law was unchanged (R1). This difference was already fn-206's and is reported here for the host.

### Strips on the port (`raw/walk7/strips.png`), seed 1

Viewed; no pops in any of the three.

- **Beech to spruce, age 40:** identical in height to walk6 (10.1 to 12.7 to 11.8 m), the leader kept.
- **Spruce to oak, age 60:** 16.9, 16.9, 17.3, 17.9, 18.5, 18.6, 19.0, 17.8, 15.8 m. The spruce's cone opens by degree into the round-10 oak's leaning, forked crown.
- **Oak to palm, age 60:** 15.8, 17.5, 20.0, 21.0, 22.2, 22.6, 22.6, 22.6, 22.4 m. The oak's crown goes early and steeply (decision 15), the stem runs on as a leader, and it thickens into the palm's column (decision 18).

### Workspace gate and Codex on the port

- **Workspace gate** at `019a98b1`: 1093 passed, 0 failed (`raw/gate4.txt`).
- **Codex round 1 on the port, NEEDS_WORK, both fixed in `1701baf6`:**
  - The full lay grown with the tree chose its standing axes by whole cycles, so it dropped dead wood a fractional shedding delay still keeps, and the sag it laid jumped. Red first: a twig's shedding delay walked in leaf on sagging limbs jumped at 1.005.
  - A relay chain's pull history grew with its length. A relay that has left the axis's line now carries no pull, and a spent pull is dropped.
  - Unlit, the species are unchanged. The lit oak moves within a thousandth of its axes.
- **Codex round 2 on the port, NEEDS_WORK, fixed in the next commit:** shedding counted a parent's share of its first cycle twice where its continuation began in the same cycle, keeping a branch of quarter-cycle ages at 25 % a cycle after it should go. This was fn-206's own code, found now.
  - Red first: `a_branch_of_ages_within_one_cycle_is_shed_on_time`.
  - The four species are unchanged.
  - The round cap (2) is reached.

### Host decisions 19 to 21 (2026-10-05)

19. **Response exponents and shares mix linearly** (`shade_hazard`, `shade_size`, `leaf_girth`, `apical_control`, `retained`). An exponent acts as a power, so its effect is already geometric. This is recorded in the scale table's doc comment.
20. **The beech's extra wood at 60 years is accepted as fn-206's change of law.** Over 8 seeds against master:
    - length 434k to 575k (+32 %)
    - axes 77.9k to 96.7k (+24 %)
    - base radius 0.310 to 0.353 m (+14 %)
    - each about 2 standard errors; height unchanged (14.27 to 14.08 m)

    At 80 years it reads as the same rounded beech dome. The beech's pass stands; this is a candidate for the beech's calibration later.
21. The scratch worktree `.worktrees/palm-master` is removed once done.

### Port close

- **Codex: SHIP** on the port against origin/master (`raw/review-port3.json`). The two boundary findings stay deferred under decision 17.
- **Workspace gate** at `bc832d12`: 1095 passed, 0 failed (`raw/gate5.txt`).
