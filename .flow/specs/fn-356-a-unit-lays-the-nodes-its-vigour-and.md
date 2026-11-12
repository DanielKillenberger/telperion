# A unit lays the nodes its vigour and presence give

Child of fn-351 (P1, P2, the speed aim). Host-written, 12 Nov 2026. Builds on the phase's top branch (`fn-355-a-year-is-a-state-the-grower-reads-no`) and lands with fn-354 and fn-355.

## Problem

Measured (`.flow/evidence/fn-356-a-unit-lays-the-nodes-its-vigour-and/SURVEY.md`): once the oak no longer collapses with age, its count of shoots has no ceiling. The oak at 200 on seed 1002 holds 29.4 million phytomers and peaks at 29.6 GiB (base 1.78 million, 5.9 GiB); the scoreboard's panel dies on it. On the oak at 120, 47% of held phytomers stand at a presence under 0.05 and carry 0.8% of the leaf; half the living tips are under 8% present. Light holds the crown's presence-weighted leaf at a ceiling (the summed presence of tips is flat from year 100), presence only ever multiplies downward along a lineage, every unit lays its full count of nodes whatever its size or presence (4.1 to 4.4 in every bin), and no rule ends a tip for being small or faint. So the same leaf is carried by ever more, ever fainter complete shoots.

## The design (host)

A growth unit lays a share of its nodes: `min(1, size x presence)` of them, counted from the unit's base, a whole number of nodes with the last one growing in by degree, as `lineage::laid` already lays a zone's quota.

- `size` is the unit's shade size (the vigour its light gives it against its kind's mean) and `presence` the presence its unit is grown at today. A unit at or above 1 lays every node, as now.
- Nothing else about a node changes: its scale, its leaf, its buds are as today. A unit that lays no node bears no bud; its apex lives on by the rules it lives by today.
- Source: Pałubicki et al. 2009, section 4.2: "The integer part of the amount v of the resource reaching a bud determines the number of metamers produced by this bud: n = ⌊v⌋", read in the paper's text. Departures, named: their v is an absolute amount and ours is relative to the kind's mean, capped at a whole unit; our last node grows in by degree where theirs is cut off.
- No new value.

**Expected on paper** (the survey's first-order sums): held phytomers on the oak at 120, seed 1002, from 9.7 million to about 2.7 million; on the oak at 200 from 29.4 million to 4 to 6 million; leaf area down by a few percent at most (units under size 0.5 carry 6% of it, units under presence 0.2 carry 2.4%); time and memory down with the phytomers. Species with no size response change only through presence. The faint lineages should die out, since a unit that lays no distal node bears no lateral.

**What would refute it:** a census at 120 that still shows the tip count rising against a flat summed presence (then a tip that lays nothing must also end, and the host designs that from Borchert and Honda's F_min rule); the oak's leaf area falling by more than a tenth; a crown hollowed or a tree visibly barer than before on the sheets; a walk failing its bound.

### The first design is refuted; the second (host, 12 Nov 2026)

Built as above (`9bc970234`), the quota did what it was for (the oak at 120, seed 1002: 9.18 to 2.13 million phytomers, phytomers under 0.2 mm 3.89 million to 5 thousand, the living apices level off) and broke the tree: the drawn oak's leaf fell 78% at 40 and 42% at 120, and the tree at 120 shrank from 30.0 to 25.6 m. It also made one branch's node count read the other branches (three independence tests), and gave the closed form a rule it has no term for. The host's errors: the shade size is normalised to a mean of one, so a cap at one takes nodes from every unit below the mean and gives none back above it (Pałubicki's vigour is absolute and uncapped); and the presence the allotment carries holds dominance, which is a designed size, not faintness. The quota is withdrawn: its commit and its test are reverted.

**The second design: a tip whose unit is less than one node ends.**

- Each year a living tip's survival is multiplied by `min(1, presence x N)`, where `N` is the number of nodes its unit lays this cycle and `presence` is the faintness it is grown at: the product of the by-degree presences of the draws that made its lineage and of its own running presence. Where the axis's stored scale also holds dominance and the two cannot be told apart without a new field, the stored scale is used and the report says how many laterals of a whole draw fall under `1 / N` by dominance alone.
- The shade size is not read. Nothing reads another branch.
- A unit of one node or more (`presence x N >= 1`) is untouched: on a unit of four nodes that is every tip at a quarter present or more.
- It is a hazard like the others: by degree, drawn on the tip's own key, its presence handled as the shade hazard's is.
- No new value. Source: the same sentence of Pałubicki et al. 2009 (a bud whose vigour gives less than one metamer lays none), and Borchert & Honda 1984's rule that a terminal under its minimum flux dies after a few increments; read as a yearly survival where they have a cut-off and a delay. That reading is the host's.

**Expected on paper:** at 120, tips under a presence of 0.2 are about 55% of the living tips and their units hold 56% of the phytomers and 2.4% of the leaf; most end within a year or two of falling under a quarter of a node, and their wood closes and is shed. Held phytomers about halve (9.7 to 4 to 5 million); leaf, height, spread and trunk move by a few percent at most. At 200 the tree should fit a 16 GB scope.

**What would refute it:** leaf, height or trunk moving by more than a twentieth on the oak at 40 or 120; phytomers at 120 falling by less than a third; the tip count still climbing against a flat summed presence over the last two decades at 120; a closed-form oracle or an independence test failing (return with which and by how much; do not repin an oracle).

### The second design is refuted; the third (host, 12 Nov 2026)

Built (`90e3b3dda`), the survival by `min(1, presence x N)` failed the narrow suites before any tree was measured: nine closed-form oracles, three independence tests, two walks, and one simulator comparison refused as collapsed. The cause is in the design, not the build: the closed form counts a faint element at its share, linearly, and any rule that treats a faint element differently from a whole one is not linear in presence; and a unit's summed node presences read its laterals' and sleepers' expected wood, so the rule read other draws. Its commits are reverted. Two attempts on this fault are spent.

**What both attempts taught:** inside this grower, faint elements cannot be made to die by a rule of growth without giving up the closed form, the independence of draws, or the leaf. So they are not given a rule of growth.

**The third design: the grower does not grow what is under its own grain.** It rests on a measurement (the survey's census: 47% of the phytomers at 120 stand under a presence of 0.05 and carry 0.8% of the leaf) and on the contract the walks already hold (a branch under 1e-6 of the tree counts as vanished).

- A bud is not grown, and a living tip ends, when its presence times the share of the tree's expected wood that its window already reads (`Windows::wood` for its PA at its cycle, the same number that sizes its window, age-free since fn-355) is under `GRAIN`.
- `GRAIN` = 1e-7, a contract constant beside RATE, SPAN, FLOOR and LOOK. Estimate, named: a tenth of the walks' vanishing bound, so that what is dropped stays under that bound when its realised wood is several times its expected.
- It is no draw and reads nothing but the tip's own lineage and the species' tables, so it moves no other draw and adds nothing the closed form's fixtures can see: there every element is far over the grain. If a closed-form oracle or an independence test does move, that refutes it.
- It is not a limit on how much a tree may grow: nothing is counted and nothing is refused. It is the resolution the grower works at, and it is relative to the tree.

**Expected on paper:** a whole twig's share in the oak at 120 is of the order of 1e-6 to 1e-5, so tips under a presence of about 0.01 to 0.1 end: a third to a half of the phytomers at 120 and more at 200, with the leaf, height and trunk moving by about a hundredth. Young trees do not change at all (their elements are far over the grain); the tests' small trees do not change.

**What would refute it:** any closed-form oracle, independence test or walk failing; leaf, height or trunk moving by more than a twentieth on the oak at 40 or 120; phytomers at 120 falling by less than a quarter; the oak at 200 on seed 1002 still over 16 GB (then the count is Phase 5's to carry as counts, and this rule stays only if it costs nothing).

## Acceptance Criteria

- **R1.** The rule (the third design) is in the grower for every species, with no branch by species; a tree in which every tip stands over the grain is byte-equal to today's, and a tip under it ends (both tested on fixtures).
- **R2.** The walks hold: a branch that enters at a crossing enters at vanishing size and no step breaks its bound.
- **R3.** Paused and one-call growth agree; fn-355's year tests (a tree at y is the tree asked for at y) pass unchanged.
- **R4.** Census on the oak at 80 and 120, seed 1002, before and after: living tips, summed presence, held phytomers, phytomers under 0.2 mm, leaf area, by decade for the tip count.
- **R5.** Measured on seeds 1001, 1007, 1012 beside the branch's head: the oak at 40 and 120, the ordinary tree, beech, birch, spruce at 40, Telperion at 82: height, spread, trunk, drawn leaf, new shoots, held phytomers, seconds and peak memory in one load window; sheets whole and bare. Then the oak at 200 on seeds 1001, 1002 and 7 in a 16 GB scope: drawn or not, peak memory, seconds, sheets.
- **R6.** Docs say what a unit lays; every repinned digest is listed before and after.

## Stops (return to the host with the numbers)

- Any of the refutations above.
- The quota needing a definition the design does not give (which presence, at which moment; a zone order that is not base to tip).
- Any change to what ends a tip.

## Boundaries

- No new value, no per-species value, no cap on any count.
- What ends a tip, what wakes a bud and the balance are untouched.

## Rigor

Product code, Opus to this design; narrow tests while building; the full gate and one codex review at the landing with fn-354 and fn-355.
