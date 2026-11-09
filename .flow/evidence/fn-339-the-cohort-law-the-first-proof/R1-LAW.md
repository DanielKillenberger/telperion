# fn-339 R1: the cohort law on paper

Drafted by the worker on 10 Nov 2026 at `7194a3efa`, for the host. Nothing here is built or decided: it writes the outline in the spec as formulas over the species' existing settings, works them on the oak's, spruce's and beech's own tables, and lists where the outline is ambiguous, cannot be written, or leaves something out. The design questions at the end are the host's.

The arithmetic is `probes/law.py` (stdlib Python, deterministic). It transcribes the three tables by hand from `oak.rs`, `spruce.rs`, `beech.rs`, `chain.rs` and `tolerance.rs`, applies `Species::canonical`, and runs the law below on one bearing node, 20,000 repeats, seed 339. A linear recursion with no shedding (the closed form's own arithmetic) gives the expected units grown each year beside it. The two agree within 0.1 units on every case (columns "law / closed form" in the output). The output is `raw/law-run2-20000.txt` (ignored); the tables quoted below are copied from it. The transcription is the script's own risk: it reads no crate. Re-running it against `closed_form::expected_bud_counts` would catch a mis-copied row. That check needs a build, so it is left for R3's dry run.

Light levels are fn-335's birth-light quintile cuts, from the fit seeds at age 40 (`fn-335/raw/read/<preset>.json`). Medians: oak short and twig about 0.26, with a lower quintile of 0.125. Spruce shoot about 0.49. Beech short and shoot about 0.73 to 0.77. Each case is also run at 1.0.

## 1. The law, step by step

Notation: a cohort member is a lateral of age (PA) `j` with `t` cycles spent in that age, `a` cycles since its birth, and its nodes laid so far. `L` is the light of the cell holding its bearing node, from the leaves of the cycle before.

### Which laterals are cohorts

**Rule (as written for this note).** A lateral of age `j` is a cohort when its subtree's reach is under the cell:

`E(j) = r(j) + max over i borne by j's axis of E(i) < c`

- `r(j) = Σ_{t<lifespan_j} share_t · S_j(t) · nodes_j · internode_j + continuation_j · r(j+1)`
  - `nodes_j = Σ_zones NodeLaw::mean`
  - `S_j(t)` is survival so far. In fn-334's form it is `viability_j^(t+1)`. In the form used here, a failure or abortion that relays does not end the axis: `S` falls by `(1 - viability)(1 - relay_failed)` and by `viability · abortion · (1 - relay)` a cycle.
- The borne ages are the zones' `lateral` and `dormant` entries passed through `Species::lived`, over the axis and its continuations.
- A lateral age that bears its own age (the oak's branch and bough, every limb) has `E = ∞`.

Settings read: `lifespan`, `continuation`, `viability`, `relay_failed`, `abortion`, `relay`, every zone's `nodes`, `lateral` and `dormant`, and `internode`. Shade, size and wander are left out: `E` is a path length in expectation, not a straight-line bound.

| species | lateral age | r, fn-334 | r, relays carry | E (expected subtree) | potential subtree (max nodes, no death) | cohort at 0.25 / 0.5 / 1.0 m |
|---|---|---|---|---|---|---|
| oak | 6 limb | 105.0 | 105.0 | ∞ | ∞ | no / no / no |
| oak | 7 bough | 10.48 | 10.80 | ∞ | ∞ | no / no / no |
| oak | 8 sprig | 2.30 | 2.43 | ∞ | ∞ | no / no / no |
| oak | 9 branch | 1.12 | 1.25 | ∞ | ∞ | no / no / no |
| oak | 13 twig | 0.254 | 0.254 | 0.313 | 0.390 | no / **yes** / **yes** |
| oak | 14 short | 0.059 | 0.059 | 0.059 | 0.090 | **yes** / **yes** / **yes** |
| spruce | 8 sprig | 31.8 | 31.8 | 32.6 | 42.8 | no / no / no |
| spruce | 10 branchlet (with its spur) | 0.651 | 0.651 | 0.719 | 0.800 | no / no / **yes** |
| spruce | 12 shoot | 0.068 | 0.068 | 0.068 | 0.080 | **yes** / **yes** / **yes** |
| beech | 6 limb | 14.1 | 16.8 | ∞ | ∞ | no / no / no |
| beech | 7 bough | 6.67 | 7.20 | ∞ | ∞ | no / no / no |
| beech | 8 sprig (its "spur") | 5.63 | 5.63 | ∞ | ∞ | no / no / no |
| beech | 9 branch | 1.86 | 1.86 | ∞ | ∞ | no / no / no |
| beech | 12 shoot | 0.376 | 0.376 | 0.450 | 0.620 | no / **yes** / **yes** |
| beech | 14 short | 0.074 | 0.074 | 0.074 | 0.120 | **yes** / **yes** / **yes** |

The table lists only ages that arise as laterals. Each age that arises only by continuation belongs to its lateral's axis: the oak's shoot (12) is its branch's continuation, the spruce's spur (11) and branch (9) continue its branchlet and sprig, and the beech's shoot ages 10 and 11 continue its branch.

Read off the table:
- **Oak:** short at 0.25 m; twig and short at 0.5 and 1.0 m. The set does not change from 0.5 to 1.0, because the branch's reach (1.12 to 1.25 m) sits just past 1.0.
- **Spruce:** shoot at every cell; the branchlet only at 1.0 m.
- **Beech:** short at every cell; shoot at 0.5 and 1.0 m. Its expected subtree, 0.450 m, sits near the 0.5 m line, and its potential, 0.62 m, is over it.

**How this differs from fn-334's rule.** fn-334 coarsened a *bearer* age when the mean reach of the laterals it bears was under `f`, which gave fn-335's sets: oak 7, 8, 9, 13, 14; spruce 8, 10, 12; beech 9, 12, 14. That is a rule about which ages' units to merge. This one is about which laterals leave the bud-by-bud grower. The two agree on the tip ages and part on the bearers.

### Birth

At a structural node of zone `z` of a unit of age `k`, each of the zone's `buds` places draws one categorical bud. The bud carries PA `i` with probability `laterals_k[z][i]`, which is `lateral` spread by `rhythm` and scaled by `readiness` (`PaState::laterals`). It grows from the next cycle in `lived(i)`, at `reach` from the continuations passed through.

This node is grown by the cycle. Its draws are today's whole-number draws, keyed as today (`unit.rs`), so birth needs no expectation. **Sleeping buds are a second birth:** each place also holds a sleeping bud of `j` with probability `dormant[j]`, waking after `delay` years at a yearly hazard `rate`. The outline names only lateral probabilities. On the spruce at 1.0 m, sleeping branchlets are 3.15 of the 7.85 branchlets a branch unit bears (40%).

A member's own laterals are drawn the same way over its own nodes, from its unit's draws. They join the stretch's cohorts. Settings read: `buds`, `lateral`, `readiness`, `rhythm`, `dormant`, `delay`, `rate`, `nodes`.

### Ageing

A member that survives the year (below) lays one unit: `nodes` drawn from each zone's `NodeLaw`, then `t ← t + 1` and `a ← a + 1`. When `t ≥ lifespan_j`, it moves to `successor(j)` with probability `continuation_j ·` (the reach of the ages passed through), with `t ← 0`; otherwise it stops. A fractional lifespan grows its last unit at its share (`Carried`). Every cohort age of the three species has a whole lifespan.

Not in the outline, needed by the tables:
- `abortion` and `abortion_rise`.
- The three relays (`relay`, `relay_ended`, `relay_failed`), with `relay_at` and `epitony`.

All of these are 0 on every cohort age of the three species, so they are dormant in this proof. They are not dormant across the space: the birch, the palm and any walk can put a relay on a cohort age, and the law then has to carry "a member replaced by one of its own age carrying its time".

### Death

Each cycle, before it grows, a living member survives with

`s_j(L) = viability_j ^ (L ^ -shade_hazard_j)`, in `advance.rs`, with `L` read at the cell.

Its values here:
- Oak: `shade_hazard` 1. A short (`viability` 0.9) survives at 0.667 a year at `L` = 0.26; a twig (0.98) at 0.925.
- Spruce: 0.2115, from `tolerance.rs` with T = 4.45. A shoot survives at 0.887 at 0.49.
- Beech: 0.169, with T = 4.56. A short survives at 0.895 at 0.73.

**The outline leaves out the carbon balance's own hazard.** In the cycle (`balance.rs`, `contract/shed.rs`), every living lateral's subtree is shed with yearly probability `1 - e^-h`, where:
- `h = balance_hazard · max(0, tolerance - m)`
- `m ← (m + (q - c)/(q + c)) / 2`
- `q` = Σ over its living apexes of presence × the light at the tip
- `c = upkeep ·` the subtree's metres of wood

Every input exists for a cohort: its living count × `L`, and its wood. The outline only charges the cohort's upkeep to its bearer.

Worked here on the cohort as one subtree, the balance stays positive while anything in it grows, at every light read. The one exception is the oak at `L` = 0.125 in years 7 and 8 (balance −0.30 and −0.67, memory −0.01 and −0.34). There the growth is 0.11 and 0.03 units. So at these lights the term is nearly dormant on cohorts, but it is a hazard of the law and has to be written in or dropped by name.

`ground_hazard` is 0 on all three species.

### Shedding

`shed.rs` drops a lateral once its whole subtree has held no living apex for more than the `shedding` cycles of the age it ended in. It fades over the cycle past that, and it is gone with its bearer.

The outline's "what is dead drops after shedding" is per member. That cannot be written for a member whose descendants still live, so a flattened table by (age, years) cannot hold it. A twig whose shorts still grow is kept; a shoot is gone when its spur goes. The script keeps each member's lineage to do it.

The spruce shows the difference. A shoot's `shedding` is 3 and its spur's is 1. Under per-member timers the shoots would outlive the spur that bears them by two years.

### Totals

Each total, as the law has to state it for the bearer:

- **Leaf into the cell.** `Σ` over members' nodes laid this cycle of `scale · size · leaf_area`, deposited at the bearing node. Today the lattice takes only the phytomers laid this cycle (`grow/draw.rs`, `sketch_end`: `phytomers[from..to]`). Section 3 says what this does to each species.
- **Upkeep into the bearer's balance.** `c += upkeep · Σ members' wood`, and `q += living members · L · presence`. Both are as the cycle sums them over a lateral's subtree.
- **Section into the bearer's girth.** Pipe model: each member's base radius is `(Σ own pipe_j^e_j · leaf_term + Σ its children's r^e_j)^(1/e_j)`, then raised to the bearer's exponent `e_k` (`girth.rs`, the upward pass).
  - Where `e_j = e_k`, the flat sum `Σ pipe²` over every standing phytomer is exact. That holds for the oak's twig and short on a branch (all 2.0), the spruce's shoot on a branchlet or spur, and the beech's short and shoot on a branch.
  - Where they differ, flattening is wrong, and each member's radius has to be taken through its own lineage: shorts on the oak's bough (2.6) and limb (2.8), branchlets on the spruce's branch (2.2).
  - `leaf_term` is `leaf_girth` (1 on all three) times the light against the tree's mean. Neither `ripening` nor `retained` is set on any cohort age (both 0).
- **Wood.** `Σ nodes · internode · size · scale`.

**The vigour split.** A unit's internode, its leaf and its girth all scale by `size = (v / v̄_PA)^ψ`, where:
- `v` is the bud's Borchert–Honda share (`allocation.rs`);
- `ψ = shade_size` (the oak's twig and short 1.0, its sprig 1.5; the spruce and beech 0);
- the result is normalised to a mean of 1 over each PA's living buds.

A cohort has no bud in the split, and the outline gives it no size. On the oak the size is live. Taking `size = 1` keeps the mean but drops the light-size correlation, which the cycle has and fn-335 measured: light took the oak's leaf error from 0.114 to 0.049. Cohort and structural ages are disjoint, so dropping the cohorts from the split leaves the structural groups' normalisation alone. The split itself still sees them as laterals at their node, through `Q_l`.

A newborn lateral also inherits its bearer phytomer's `scale` (presence) times its `dominance` share (`draw.rs`). Dominance is 0 on every cohort age.

### Whole numbers

"Each step's count is drawn from its expectation by the bearer's lineage key" can be read two ways:
- **(a)** For each class, the step is a Binomial(n, p) (survivors, movers) or a sum of categorical bud draws (births), each keyed by the bearer's lineage, the class and the cycle. This is exact in distribution: per-member draws and per-class draws give the same law.
- **(b)** The expected count, rounded or drawn once. This loses the variance. An oak node expects 0.6 shorts; rounding always gives 1.

The script uses (a). Either way the count moves by a whole member at a threshold when a setting moves a little. The cycle avoids that with presences (`lead`, `placed`, `windows`, the `kept` share), so R10's "changes the tree by degree" holds on its parameters. The outline carries no presence.

## 2. One bearing node over ten years

The bearer lays the node in year 0, its laterals grow from year 1, and the cell's light is held fixed. All values are means over 20,000 repeats:
- standing: members not yet dropped;
- units: units grown that year;
- light leaf: leaf on phytomers laid that year (what the lattice receives today);
- drawn leaf: what the expansion draws, by `leaf_years` and clusters;
- wood: standing metres;
- section: `Σ pipe²`, in radius units squared.

The full year-by-year tables are in the output. Summaries follow.

**Oak, a medial node of a branch** (one bud: short 0.6, twig 0.35, bare 0.05; cohort ages twig and short, at 0.5 and 1.0 m):

| light | units over 10 y | peak standing (year) | leaf deposited over 10 y | peak year's leaf | peak wood | peak section (radius) | standing at year 10 | last growth |
|---|---|---|---|---|---|---|---|---|
| 0.26 | 7.42 | 3.44 (5) | 0.0550 m² (30 leaves) | 0.0101 m² (year 3) | 0.169 m | 9.65e-6 (3.1 mm) | 0.32 (shorts 0.19, twigs 0.14) | year 8 |
| 1.0 | 13.80 | 4.90 (5) | 0.1014 m² (56 leaves) | 0.0185 m² (year 4) | 0.293 m | 1.70e-5 (4.1 mm) | 0.90 | year 8 |
| 0.125 | 3.52 | 2.56 (3) | 0.0264 m² | 0.0053 m² (year 2) | 0.093 m | 5.21e-6 | 0.06 | year 8 |

Light leaf and drawn leaf are equal for the oak (`leaf_years` 1). By year 9 nothing on the node grows, because neither cohort age renews itself: the twig's top `TWIG` is 0.0 and the short bears nothing. So an oak node's foliage lives about eight years past its laying, and the oak's leaf stands within about eight years of the growing structural tips.

**Spruce, a medial node of a branchlet** (two buds, shoot 0.55; the cohort at 0.25 and 0.5 m): at 0.49, 1.83 units over two years, 0.0025 m² of needles deposited, and nothing standing after year 5. At 1.0, 1.88 units.

**Spruce, a medial node of a branch** (two buds, branchlet 0.7, sleeping branchlet 0.45 per place; at 1.0 m the branchlet, its spur and their shoots are cohorts):

| light | units, year 10 | units over 10 y | standing, year 10 | living | light leaf, year 10 | drawn needles, year 10 | wood | section |
|---|---|---|---|---|---|---|---|---|
| 0.49 | 17.7 | 140.9 | 54.8 | 19.7 | 0.0243 m² | 0.1223 m² | 3.64 m | 3.59e-5 |
| 1.0 | 18.4 | 145.1 | 55.9 | 20.2 | 0.0253 m² | 0.1263 m² | 3.76 m | 3.70e-5 |

At year 10 the standing members are shoots of 0 to 5 years (about 9.4 of each age to 3 years, then 8.1 and 7.0), 0.8 branchlets and 1.25 spurs. The steady state holds about 18 new shoots a year per branch medial node and keeps rising slowly as spurs accumulate (lifespan 40). Light barely moves it (`shade_hazard` 0.21). The drawn needles are 5.0 times the deposited ones, because `leaf_years` is infinite and needles are drawn on dead standing shoots too.

**Beech, a medial node of a branch** (short 0.7): at 0.73, 2.15 units over four years, 0.0135 m² of leaf, gone by year 6. **Beech, a spare node** (shoot 0.65): at 0.73, 15.6 units, 0.0957 m² of leaf (peak 0.0191 m² in year 5), 0.83 standing at year 10, last growth in year 9. Light and drawn leaf agree in expectation: the short's cluster of 4 equals its mean of 4 nodes a unit.

**What a law over classes steps, against one over members.** One whole bearer unit, ten years, at the median light. Sleeping buds are left out of this count.

| bearer unit, cohort ages | nodes | member-years standing | units grown | class-years per node | class-years per unit |
|---|---|---|---|---|---|
| oak branch, twig + short | 4.99 | 70.0 | 24.5 | 48.1 | 38.6 |
| oak bough, short | 5.99 | 4.2 | 1.8 | 4.2 | 3.7 |
| spruce branchlet, shoot | 3.00 | 21.7 | 7.8 | 14.5 | 7.4 |
| spruce branch, branchlet + spur + shoot | 4.00 | 925.8 | 333.5 | 221.4 | 96.1 |
| beech branch, shoot + short | 6.53 | 46.7 | 28.3 | 36.9 | 30.4 |

A class is (age, years in it, years since birth, living). Counting by class rather than by member saves 1.5 to 2 times on the oak and beech, and only when classes are pooled over a stretch, not a node. The spruce at 1.0 m saves about 10 times.

## 3. Where the outline is wrong, ambiguous, or cannot be written from the tables

1. **Correction, 10 Nov 2026, R3's dry run: this item's arithmetic is wrong by a factor of about ten, and its conclusion is reversed.** In steady state, a branch unit laid each year sets beside its own 1 unit the cohort units of every unit laid in the ten years before. That is 24.5 a year at 0.26, not 2.45: the sum over a unit's ten years was divided by ten. Measured on the replay's trees at 40, the cohorts at 0.5 m hold 94.4% of the oak's yearly units, 93.2% of the beech's and 81.9% of the spruce's. By unit count, that bounds the saving at 17.9, 14.7 and 5.5 times (`R3-DRY.md`). The text below is kept as it was written.

   **The speed the law can buy, by unit count (inferred from the tables).**
   - Per oak branch unit-year, the cohort ages grow 2.45 units at 0.26 (24.5 over ten years) and about 3.75 at full light. The branch itself grows 1. If cost follows units grown, taking the oak's twigs and shorts out saves at most 3.5 to 4.8 times on branch wood, at 0.5 m and at 1.0 m alike, since the set is the same.
   - On boughs and limbs the share is smaller, because their laterals are mostly branches, which stay structural.
   - The beech's branch unit: 2.8 cohort units a year against 1, so at most 3.8 times.
   - The spruce has room: at 0.5 m a branchlet unit carries 7.7 shoot units, and at 1.0 m a branch unit carries 33.
   - So on the oak and beech, the goal's "cheaper by five times" is not reached by unit count at these cells. It has to come from each step costing less (no geometry, lattice deposit or split per phytomer), or from a larger cell where the oak's branch becomes a cohort (over 1.25 m).
   - These are expectations from the tables, not a grown tree. R3's dry run should read the cohort ages' share of the cycle's units directly.
2. **"Closer to real trees" (inferred).** The law moves the same tables, and its expectation is the closed form's: the node tables above match the linear recursion. On shoots a year and leaf it reproduces the cycle in expectation. It can depart only where the cycle departs from its own expectation, through light feedback and the vigour split, or where the law counts differently: which leaf it deposits, and what size a cohort takes. The spruce's surplus of new shoots (about 18 a year per branch medial node) is in its tables, not in how they are run. Under the Boundaries ("no value fitted per species") the law cannot remove it.
3. **Which leaf enters the cell.** Today only the current year's leaf shades.
   - For the oak and beech that is also all the drawn leaf.
   - For the spruce the drawn needles are about 5 times the deposited ones (0.122 against 0.024 m² per branch node).
   - R2's leaf area index is standing leaf, so the law must say which it deposits. Depositing standing needles would deepen the spruce's shade by up to 5 times and change its survival and size, on tables tuned under current-year shade.
4. **Cell sizes 0.25 and 1.0 m are not the lattice's.** The lattice spacing is a constant 0.5 m (`light.rs`, `pub(crate) const CELL`). At 1.0 m, "cannot outgrow a cell" is a threshold beside a 0.5 m lattice. The spruce's branchlet subtree reaches 0.72 m, so its leaf, deposited at the bearing node, would sit more than a lattice spacing from where the cycle puts it. Whether the cell in R1 is the lattice spacing, or a threshold the lattice need not follow, is open.
5. **"On the phytomer that bears it" against "on the wood that bears them".** This spec counts a cohort on the bearing phytomer; the goal's R1 and the RETHINK say a bearing stretch. The step counts above favour a stretch (a unit, or the cell). A stretch mixes birth years and lights unless it is one unit.
6. **The boundary is a step, not a degree.** The cohort set changes by whole ages at the reach thresholds in the table. The goal's R1 asks the boundary to move by degree with the cell and the shoot lengths. That needs the blend the Decision Context leaves open.
7. **"Cannot outgrow" has no bound on the oak.** `size = (v/v̄)^ψ` is unbounded above, so a bright oak twig can outgrow any reach computed from the tables. The rule can only be an expectation, and promotion of a member that would leave its cell is the open question already named.
8. **Whole numbers against R10.** As in section 1 under "Whole numbers".
9. **Shedding needs lineage.** As in section 1 under "Shedding". A table by (age, years) alone cannot drop a member at the cycle's time.
10. **Light at the bearing node, not the tip.** The cycle reads each apex's light trilinearly at its tip. The law reads the cell. The shift is up to the subtree's reach: 0.06 to 0.45 m at 0.5 m, and 0.72 m for the spruce's branchlet at 1.0 m.
11. **Sleeping buds** are a birth the outline does not name. They are 40% of the spruce's branchlets at 1.0 m.
12. **The stop rule's reading.** "Stops if they are no closer and no cheaper by five times" can mean stop only when both fail, or stop when either does. With items 1 and 2, the likely outcome is about as close and under 5 times on the oak and beech, so the reading decides the verdict.

## 4. What the law leaves out (each 0 or dormant on these three species unless said)

- Abortion and its rise; the three relays, `relay_at`, `epitony` and `takeover`.
- Fractional lifespans and their carried shares.
- `ground_hazard`.
- Presences, leads and the `kept` share. These are live in the cycle on every species.
- The carbon balance's hazard on the cohort itself. It is live but nearly dormant at the lights read.
- Posture, wander, insertion, roll and phyllotaxis. These are geometry: they place leaf within the cell, which the law no longer resolves.
- Flowering (`flowering`) and leaf phenology (`elongation`, `expansion`, `leaf_life`, budburst and senescence). The expansion reads these, not growth.
- `ripening` and `retained` (0 on cohort ages).

## 5. Questions for the host

1. Which cell R1's cohort rule uses (the lattice's 0.5 m, or a threshold free of it), and whether the rule is the expected reach `E` above or the potential bound.
2. Whether the law includes the carbon balance's hazard on the cohort, and sleeping buds as a birth.
3. Which leaf a cohort deposits: current-year, as today, or standing.
4. What size a cohort's units take: 1, or a share from the split.
5. Whether cohort counts carry a presence for R10, and whether "whole numbers" means reading (a) or (b).
6. Whether a cohort is kept per node, per unit or per stretch, and whether it keeps lineage for shedding.
7. How the stop rule is read (section 3, item 12), given that the unit count above puts the oak and beech under 5 times at 0.5 to 1.0 m.
