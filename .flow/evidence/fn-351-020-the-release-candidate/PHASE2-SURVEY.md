# Phase 2 survey: girth, sag and the botany of wood, 11 Nov 2026

By a Fable agent with literature sub-agents, at `3de1eda55`; condensed by the host. Marks: [code] read in this worktree; [full text], [abstract], [secondary] how far a source was read by a sub-agent (the surveyor did not re-read papers itself); [inferred] arithmetic or reasoning. Nothing in the repository was changed.

## A. The code as it is

**When a radius exists [code].** Radii are computed once per settle (`grow/settle.rs:142`), after shed and scale, before place and sag. During growth every phytomer's `radius` (an f64) is 0 and unused (`grow/unit.rs:75`). Nothing per phytomer records wood laid; a retired subtree keeps 12 bytes per axis.

**The formula (`girth.rs`) [code].** Own pipe = `pipe × scale × min((age+1)/ripening, 1) × leaf factor`, added by every held phytomer whether or not it bears a leaf. Sections add along an axis tip to base and across children, `section = r^exponent`; radius = `secondary × section^(1/e) + (1 − secondary) × established`. A shed branch leaves `retained × r_as_grown^e` at the one node where it stood (`girth.rs:245-246`); a shed relay has no node and enters at the bearer's tip (`girth.rs:247`, `disused.rs:261-264`).

**Readers of a radius [code].** Sag load (`sag.rs:237-240`: mass `(r² + own²) × length`), sag turn (`sag/turn.rs:96-143`: curvature `sag × moment / r⁴`), ground contact, the drawn nodes (`pipeline/space.rs:417`: wood under 0.2 mm ends an axis; `486-491`; structural from 0.05 of the root; codominant fork from 0.4 of the parent), relay joins (`pipeline/space/join.rs:71`), the surface (`surface/samples.rs:24-27`: root flare up to `flareRadius` 2.1 at the ground), foliage stations, the field splat, the year's presentation (`unfold.rs:418`). The light lattice and the carbon balance read no radius.

**Sag [code, inferred].** `dfall/ds = −(sag / r⁴) × min(reach × sin(fall), cap)`, on the unbent tree with final radii, once at the settle; no growth history, no reaction wood. The mass has no π and no density, so an elastic beam is `sag = 4ρg / E`: 5.0e-6 per metre for green oak (1,060 kg m⁻³, 8.3 GPa). The oak's limb row is 3e-4 and its bough row 5e-5, both estimates (fn-195 SOURCES.md:28): 60 and 10 times softer than oak wood. The trunk, fork and leader rows have sag 0; after 20 years the stem line carries on as limb wood at 3e-4, so the stem that lies down is limb wood.

**Why kept pipes draw steps [code].** The kept section enters at one node and jumps the bearer's section between two neighbouring phytomers 4.5 cm apart; a drawn node starts at its parent's radius, so the surface spans the jump over one internode. The flared disc is the root flare's 2.1 times a 5 m trunk. The open cylinder's exact cause was not found; the candidate is a shed relay's pipe entering at its bearer's tip, which ends a kept axis at full radius under a flat cap.

**No girth row has a source [code, SOURCES files].** Every preset's `pipe`, `exponent`, `ripening` and `sag` is "estimated from the stills" or fitted; the oak's `retained` 0 was fitted ("at 0.5 the stem stood two to four times Silvics' DBH", `oak.rs:110-111`). A pipe is laid per held phytomer, not per leaf: the oak at 120 holds 2.37 M phytomers for 305,000 leaves.

**fn-341's ring rule [code, its `probes/earned.patch`].** Each year every phytomer laid `leaf above / (R × N)`; radius = `sqrt(laid/π + established²)`. It replaced `pipe`, `exponent`, `ripening`, `retained`, `leaf_girth` and `secondary`. It cost 2.2 to 2.3 times the grow, nearly all of it a yearly settle of a copy and a hash map per phytomer; the one upward sum a year it needs was never built. It was judged only together with a posture rule; alone it was measured at one age (80, seed 1001): trunk 0.588 m, fork limbs 0.24 to 0.27 m against 0.39 to 0.71, the limb's chord at 50° against 41° under the unchanged sag.

## B. The botany

**Leaf area per sapwood area.**

| Species | m² leaf per cm² sapwood | Source | Read |
|---|---|---|---|
| *Quercus garryana*, 25 m, 250 yr | 0.37 ± 0.01 (sapwood 4.2 cm deep, DBH 0.56 m) | Phillips, Bond, McDowell, Ryan & Schauer 2003, Funct. Ecol. 17:832-840, Table 1 | full text |
| *Q. garryana*, 10 m, 40 yr | 0.23 ± 0.02 (sapwood 4.3 cm, DBH 0.23 m) | same | full text |
| *Fagus sylvatica* | 0.331 (38 trees, 8 to 59 yr) | Bartelink 1997, Ann. Sci. For. 54:39-50 | full text |
| *F. sylvatica* | 0.54 (6 trees) | McDowell et al. 2002, Oecologia 132:12-20, Table 1 | full text |
| *Picea abies* | 0.36 (28 trees, 7 to 31 m) | McDowell et al. 2002, Table 1 | full text |
| *Betula pendula* | not found | | |

McDowell 2002 holds no *Q. garryana*. The two beech values disagree and were not reconciled. With height the ratio rises in oak (1.6 times from 10 to 25 m) and spruce, falls in beech; across 13 studies the mean is a slight fall.

**Sapwood rings.** European oak 16.0, 20.4 and 25.9 rings under 100, 100 to 200 and over 200 years (Hollstein, in Haneca, Čufar & Beeckman 2009, J. Archaeol. Sci. 36:1-11, Table 1; secondary). British oak 22.4 under 100, 31.7 from 100 (Hillam et al. 1987, same table). Beech 25 to 73 conducting rings, sapwood 0.73 to 0.75 of basal area at about 105 years (Gebauer, Horna & Leuschner 2008, Tree Physiol. 28:1821-1830; full text). Spruce ring count not obtained; birch not found.

**The pipe model.** Shinozaki et al. 1964a,b [full text]: leaf follows the trunk's section just below the lowest living branch; below it the section keeps growing, and disused pipes are offered as a hypothesis with no rule or rate. Lehnebach et al. 2018, Ann. Bot. 121:773-795 [full text]: only active pipes (sapwood) track leaf; heartwood formation is its own process. LIGNUM (Perttunen et al. 1996) turns a fixed 0.07 of sapwood a year to heartwood.

**Open-grown oak, trunk diameter by age.** White 1998, Forestry Commission Information Note 12 [full text]: even rings to a core age, then rings of constant area. Derived from his tables [inferred; reproduces his worked example]:

| Age | Champion | Good, open | Average, parkland |
|---|---|---|---|
| 40 | 40 cm | 32 | 28 |
| 80 | 79 | 64 | 56 |
| 120 | 109 | 90 | 83 |
| 200 | 152 | 128 | 121 |
| 300 | 192 | 163 | 156 |

Independent: Drobyshev & Niklasson 2010 (69 *Q. robur*, not purely open-grown): 72 cm at 200, 105 cm at 300.

**Branch burial.** The trunk lays a collar round a branch each season (Shigo 1985, abstract). Occlusion takes 2 to 10 years in birch, longer for larger branches and slower stems; no depth in centimetres found.

**Stiffness.** Green E and density: oak 8.3 GPa, 1,060 kg m⁻³; beech 9.8, 1,030; birch 9.9, 930; Norway spruce 6.2, 960 (Lavers 1983 through Niklas & Spatz 2010, Am. J. Bot. 97:1587-1594, Table 1; Hamilton 1975). Alméras & Fournier 2009, J. Theor. Biol. 256:370-381 [full text]: wood supports only from when it is laid, so a passive limb bends several times more than the final moment over the final r⁴ gives (5.2 times at their poplars' exponent), and reaction wood cancels most of it (poplars: +0.040 against −0.044 m⁻²).

## C. Arithmetic on the oak, seed 1001 [inferred]

From fn-341's yearly leaf trace on the cycle before the budget fix (leaf above 1.3 m: 102, 441, 610, 570, 465 m² at 40, 80, 120, 200, 300). No yearly leaf is recorded for the fix branch or other seeds.

| Rule | 40 | 80 | 120 | 200 | 300 |
|---|---|---|---|---|---|
| Today's trunk in that trace | 54 cm | 111 | 136 | 127 | 120 |
| Sapwood alone, `2√(A/(Rπ))` | 23 | 43 | 46 | 44 | 40 |
| (i) Kept pipe resized to the sapwood ratio | 26 to 34 | 72 to 94 | 111 to 144 | 124 to 161 | 134 to 174 |
| (ii) A ring a year, `A/(R×N)`, never lost | 20 | 59 | 91 | 127 | 154 |
| (iii) Sapwood `A/R`, one Nth retiring to heartwood a year, the tree lays what is missing | 30 | 72 | 103 | 144 | 175 |
| White, three open classes | 28 to 40 | 56 to 79 | 83 to 109 | 121 to 152 | 156 to 192 |

Rule (i) overshoots at 80 and 120 (up to 1.9 times on seed 1007), because a pipe is per phytomer and not per leaf. Rule (ii) lays too little under 100 years, when the stem has fewer than N rings. Rule (iii) is the surveyor's construction with no source for its form; it sits inside White at every age and lays no ring in a year the crown shrinks faster than one Nth.

## What only a build can answer

Whether limbs of ring-rule girth stand at 40 to 120 under a sag near the elastic value; how far leaf summed during growth differs from the settled tree's; the trunk on the fix branch, whose crown falls to a fifth after 120; the spread across seeds; the beech, whose leaf is a fifth of a real beech's, so its trunk from leaf is a wire until its foliage is fixed.
