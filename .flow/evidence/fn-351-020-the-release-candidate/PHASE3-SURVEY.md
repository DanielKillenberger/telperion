# Phase 3 survey: the old oak's crown, limbs and fork, 12 Nov 2026

By a Fable agent, read-only, at the top of the work; condensed by the host. [code] read in source (paths under `crates/telperion-space/src/` unless a crate is named), [evidence] a recorded file, [text] a quoted passage of the source (read through a passage-quoting tool, not a viewed PDF: spot-check any number before it reaches a preset), [abstract], [secondary], [inferred], [U] not determined.

## 1. Why the oak thins and shrinks after 200

**The course, measured** [evidence: `fn-354/probe/traces/G2-oregon-white-oak-300-*.csv`, `probe/VARIANTS.md`]:

| Seed | Leaf, m², at 120 / 200 / 300 | Grower's height at 120 / 200 / 300 | Drawn height at 300 |
|---|---|---|---|
| 1001 | 737 / 273 / 146 | 26.0 / 33.0 / 30.2 | 24.2 |
| 1007 | 1,172 / 407 / 254 | 25.8 / 31.7 / 31.2 | 18.0 |
| 1012 | 896 / 482 / 326 | 24.8 / 32.3 / 27.3 | 21.5 |

Leaf peaks at 100 to 120, falls 60 to 75% by 240, then holds. The grower's height is constant to the millimetre for up to a century: nothing grows above the old top. Why the drawn height sits 6 to 13 m under the grower's is [U].

**No table row ends a structural line** [code, `oak.rs`]: a limb lives 500 years with every stop relayed (abortion 0.15 a year, relay 1, viability 1, so shade does nothing to it); a bough 60 years and then relays without bound; a sprig 26, a branch 16, a twig 5, a shoot 4, a short 3.

**The balance is the only killer of limbs** [code, `grow/cycle/passes.rs`, `contract/shed.rs`]: a tip earns size x light and is charged size x (path / 34.8)², so it breaks even at light = (path / 34.8)². At the measured tip lights at 120 (10th, 50th, 90th percentile 0.057, 0.18, 0.46) the break-even path is 8.3, 14.8, 23.6 m. A balance death makes no relay; a shed axis takes every descendant.

**Nothing can rebuild from old wood** [code, `oak.rs:81-86, 275-283`, `dormant.rs`, `contract/decide.rs:245-266`]: sleeping buds exist only on limb zones, 0.1 a place, as branch buds, delay 6 years then a rate of 0.04; the wake time is fixed at the node's birth by a keyed draw and light does not enter it; a bud ages in the bark and is recorded only if it wakes within 16 years; a sleeper wakes only while its bearer's line lives, so when the balance kills a line its sleepers go with it, the opposite of release by lost dominance. About 0.26 sleepers a limb-year are recorded [inferred]. A new limb is born only at a living limb's top, 0.021 a year, a new bough 0.24 a year, both at the most charged place.

**The mechanism** [inferred]: leaf lives only on wood under about 26 years old, and such wood exists only behind an advancing limb or bough tip. Tips advance about 0.21 m a year until their path passes 34.8 x the square root of their light, then die unrelayed. Wood older than 16 years carries no bud, so the interior stays bare and the crown is a shell that retreats without replacement.

## 2. What real old oaks do

| Fact | Number | Source | Mark |
|---|---|---|---|
| Epicormic buds on *Q. petraea* trunks | 50 to 70 a metre young; about 25 a metre at 45 to 58 years | Meier, Saunders & Michler 2012, Tree Physiol. 32: 565; Morisset et al. 2012, Ann. Bot. 109: 365 | [text] |
| Two-year fate of buds | 74.5 to 90.5% stay buds, 8.5 to 18% die, 1 to 6% shoot | Fontaine et al. 2001, Ann. For. Sci. 58: 583, Tables III to V | [text] |
| Longevity | "can survive as buds for 40 years" on 40-year trees (a floor) | Fontaine et al. 1999, Trees 14: 83 | [abstract] |
| Longevity | "believed to be in the region of 100 years for oak" (a belief) | Read 2000, Veteran Trees, 3.1.3.1 | [text] |
| The reserve is finite | "persist for many decades but not for ever" | Lonsdale 2013, 4.3.2.4 | [text] |
| Release | light, thinning, pruning, defoliation, stress | Meier 2012; Morisset 2012 | [text] |
| Retrenchment | leaf area declines, "stag-headed ... typically in oak", over "decades or even centuries" | Read 2000, 3.2 | [text] |
| A lower crown | Raimbault stage 8: "a secondary, lower, crown sometimes starts to form and could eventually 'take over'" | Lonsdale 2013, Fig. 4.12 | [text] |
| Cause | a longer path from root to shoot, higher hydraulic resistance | Lonsdale 1.2.1, citing Rust & Roloff 2002 | [text] |
| Reiteration | delayed reiteration "from the development of a previously dormant bud" | Barthélémy & Caraglio 2007, Ann. Bot. 99: 375 | [text] |
| Crown size | optimum crown at 70 to 120 years | White 1998 | [text] |

From Fontaine's fates: a yearly release of 0.5 to 3% and a yearly death of 4 to 9% [inferred]. Not found: an age or girth at which retrenchment starts, a measured loss of height, a count of scaffold limbs or a limb-to-trunk ratio for oak, a height series by age [U].

## 3. Why many thin straight limbs of equal rank at 120

- The fork bears about 4.8 limbs, the leader 1.4 more and 1.8 boughs; every limb line then adds 0.021 limbs and 0.24 boughs a year, about 2 limbs and 24 boughs by 120; a bough is nearly a limb by its rows and lives without bound [code, inferred].
- Nothing lets a line that is ahead stay ahead: unit size is normalised to a mean of one within each kind and is not inherited; every unit lays its full count of nodes; the split's lambda is 0.45 on every row; `form.dominance` exists ("a few dominate and the rest stay small") and is 0 on every oak row [code].
- Girth is additive by sapwood area, so near-equal leaf gives near-equal limbs; the old exponent hid the count [evidence].
- No measured source on dominance among sibling limbs of oak was found [U].

## 4. The fork

A lateral between 0.4 and 0.7 of its parent's radius is marked codominant by weight; a run carries on into its widest child that is not codominant; every other child starts a closed tube sunk into a socket (`crates/telperion-core/src/pipeline/space.rs:47-49, 243-249`, `surface/paths.rs:47-66`, `surface/samples.rs:125-136`) [code]. With area-additive girth the codominant sibling can be the wider. On the oak at 300, seed 1001: 0.616² + 0.31² is about 0.69², so the wood is conserved and the rim is a drawing fault alone [inferred]. fn-279 names no mechanism and has no evidence; fn-280, fn-267 and fn-307 are specs only [evidence].

Options, smallest first [inferred]: (1) the run follows the widest child whatever its mark; (2) a codominant sibling's run starts flush at the fork with the parent's girth and eases to its own over the fork's diameter, blended from socket to flush by the codominant weight, moving by degree; (3) one fork patch blending both child rings into the parent's, a new mesh stage.

## 5. Options for a crown that renews

| | (a) Sleepers that persist and wake by light or lost dominance | (b) Reiteration at dieback | (c) A long trunk line, the balance alone |
|---|---|---|---|
| Rule | a bud does not age in the bark; it dies at a yearly hazard and wakes at one that rises with light at its node and when its bearer's tip is dead; it wakes as a bough or limb | a line the balance sheds leaves a relay of its own kind set back along the axis | the leader row lasts for life |
| New values | a bud's death rate and a light exponent (a first probe needs no code: the limb zones' sleeping row set to bough or limb) | a relay share and a set-back distance | none |
| Source | Fontaine 2001; Meier 2012; Lonsdale Fig. 4.12 | Barthélémy & Caraglio 2007, no number | none |
| Contracts | light rows are already outside the closed form; keyed yearly draws keep a year a state; a woken bud takes its bearer node's presence. Costs: every bearer of a sleeper stays in the working set; an axis is shed whole, so dead tops stand until a rule of partial dieback exists | continuous; reads the axis's own path and light | trivially |
| On paper at 200 and 300 | low wood at 5 to 10 m of path breaks even at light 0.02 to 0.08, so woken limbs live; leaf returns toward 700 to 1,200 m²; height holds near 28 to 32 m; limbs turn over | a stag head with a lower crown on the same limbs | the interior still bare, leaf unchanged |
| Refuted by | leaf at 200 under half its level at 120; woken axes shed within a decade; a broom of sprouts inside the closed crown at 120; a closed-form or independence test failing | the relay shed again within a few years; kinked stubs | the lower crown still bare at 200 (expected) |

## 6. Measurements that decide before building

1. A preset-only probe of (a): the limb zones' sleeping row moved to bough or limb with a mean wait of 25 to 50 years; the oak at 120 on three seeds and at 200 on one; leaf, height, woken axes by age and survival.
2. A census of what the balance sheds between 120 and 200: kind, path over 34.8, light, memory.
3. Bare wood at 120 and 200: length of structural wood older than 16 years with no living lateral; leaf by wood age.
4. Light at limb and trunk nodes under 12 m of path at 120, 160, 200.
5. Rank at 120: living limb and bough lines, the spread of their base diameters and lengths; then `dominance` at 0.5 on the limb row at 80 and 120.
6. Forks on the oak at 120 where a codominant child is wider than the run's choice, with the radius drop.
7. The gap between the grower's height and the drawn height at 300.
