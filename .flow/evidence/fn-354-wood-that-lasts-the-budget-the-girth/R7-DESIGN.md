# fn-354 R7: the beech's shoots, survey and design, 11 Nov 2026

The survey is a Fable agent's, read-only, condensed by the host; the design under it is the host's. [code] read in source, [source] checked in the paper's text, [inferred] arithmetic, [U] not determined.

## Why this is in Phase 2

On the base the beech's trunk at 40 is 0.32 m (0.26 to 0.51, twelve seeds), inside White's open-grown 0.32 to 0.48 m, because the old exponent was set to reach it. Under the wood rule the trunk follows the leaf, and the beech's leaf gives 9 to 12 cm. That is a worse tree on a published measure, so Phase 2 does not land without this.

## What the survey found

**The wood rule is consistent; the leaf is short.** 31.6 m² × 3.02 cm² = 95 cm², an 11.0 cm trunk, the probe's 9 to 12 cm. Bartelink's stand beech of 38 to 41 years carries about 85 m² at a mean dbh of 20.3 cm (Bartelink 1997, Tables I, II and IV [source]); 85 m² under the rule is 18.1 cm of wood under bark. His eq. 4 on the generator's own 7.0 m crown gives 145 m² and 23.6 cm [inferred].

**The branching rows are an understorey's, and nothing in the grower lets light change them.**

| Row [code] | Value | Source | What it measured |
|---|---|---|---|
| branch, medial zone (2 to 3 nodes) | SHORT 0.7 | Letort et al., A24 in 0.5 to 0.7 | two understorey beeches, 21 years (2.8 m, dbh 1.5 cm) and 46 years (8.5 m, dbh 6.3 cm) [source] |
| branch, spare zone (1 to 2 nodes) | SHOOT 0.65 | Letort A23 in 0 to 0.2, "raised to 0.65 (estimated)" | the same; the value is an estimate |
| branch, top node | BRANCH 0.08 | Letort A22 in 0 to 0.2, set by estimate | the same |
| shoot (2 to 3 nodes) | SHORT 0.6 | Letort A34 in 0.55 to 0.6 | the same |
| reiterates, medial zone | SHORT 0.3, BRANCH 0.27 | estimated from stills | none |
| lifespans and viabilities | branch 10 at 0.95, shoot 5 at 0.95, short 4 at 0.9 | estimated | none |

- Letort's numbers are slopes on supply over demand (eq. 5), fitted to biomass, not counted probabilities: "No quantitative topological information was recorded" [source]. "Our trees have grown in the understorey and were shaded by higher neighbor trees" [source].
- In the grower only survival answers light (viability to the power light^−hazard; the beech's hazard is 0.169) and the balance's shedding. `shade_size` shares vigour among a PA's buds around a mean of one; it changes no count. Every lateral probability is a fixed number [code].
- **Lemoine et al. 2002, Table II [source]:** long laterals are 85 ± 3% of laterals in sun (PAR 1,850 µmol), 60 ± 5% in a stand thinned the year before (1,550), 40 ± 6% in shade (255); dominant 30-year trees, branches under 6 years. A long shoot there has internodes over 5 mm (correcting `LEAF-SURVEY.md`, which gave 11 and 2.5 mm), and the 85% is of laterals only.
- The preset's branch bears 1.75 short, 0.975 shoot and 0.08 branch a unit: 38% long, Lemoine's shade class [inferred].
- Not found: an open-grown beech's leaf or dbh by age beyond White; shoots per metre; bifurcation by order [U].

**The birch** (17 to 19 cm at 40 against stand-grown 22 to 27 cm) is not the same case: its short share is from mature crowns of a canopy congener (Suzuki 2003). Its `sapwood` is the beech's by estimate and its grower's leaf has never been measured. It is measured at R8 before anything is changed.

## The design: a lateral's kind follows its light

One rule, in the grower, for every species:

- A new row on a PA, `shade_short`: a vector over the PAs, as the zones' `lateral` tables are, entry j the exponent with which a bud of this PA's kind turns into PA j when shade denies it. Neutral: every entry 0, the bud always keeps its kind. (Host, 12 Nov: the row is a vector; it replaces "two values: the PA a bud of this kind becomes, and an exponent e", since the tables name a PA only by its place in such a vector.)
- When a lateral bud of a PA forms at light L (the same light, 0 to 1 of open sky, that the shade hazard reads at its bearer's tip), with e the sum of its PA's `shade_short[j]`, it keeps its kind with probability L^e; if it turns, it becomes PA j with probability shade_short[j] / e. With one nonzero entry that is the rule as first written. It blends by degree like the lateral tables. (Host, 12 Nov.)
- The keep-or-turn decision draws off the bud's own key, only where e > 0, so a species at 0 takes no draw and grows byte-equal; its presence is by degree, as every other drawn row's is. An entry on a PA's own index is refused by name: a bud cannot turn into itself. (Host, 12 Nov.)
- The species' tables then hold what the species does in full light, and the understorey tree is the same table under less light.
- It changes by degree, is dormant at 0 and where no leaf shades, and adds no draw to a species that leaves it at 0.

Values for the beech:

- **e = 0.38** on SHOOT and BRANCH, both becoming SHORT (`shade_short[SHORT] = 0.38`, every other entry 0). From Lemoine's two settled classes: the long share falls from 0.85 to 0.40 as light falls from 1 to 255/1,850 = 0.138, and ln(0.40/0.85) / ln(0.138) = 0.38. The thinned class (0.60 at 0.84) is not used: it was measured the season after thinning. Estimate in form: a power law through two points.
- **The full-light tables:** every zone that mixes long and short laterals takes 85% long at its present total.
  - Branch, medial zone: `[(SHOOT, 0.53), (SHORT, 0.17)]` in place of `[(SHORT, 0.7)]`. With the spare's SHOOT 0.65 and the top's BRANCH 0.08 unchanged, a unit bears 2.8 laterals, 2.38 long (85%). At light 0.138 the long ones are 2.38 × 0.47 = 1.12, 40%, Lemoine's shade class and today's tree.
  - Reiterates, medial zone: `[(SHORT, 0.085), (BRANCH, 0.485)]` in place of `[(SHORT, 0.3), (BRANCH, 0.27)]`.
  - The shoot's own zone (SHORT 0.6) stays: by its definition it bears only short shoots.
- Other species: 0. Nothing of theirs moves.

**Expected on paper.** With every lateral in full light the survey's first-order count is about 108 m² of leaf and a 20 cm trunk at 40 (59 to 275 m², 15 to 33 cm across seeds). The rule's own shade takes some of that back inside the crown, so expect 60 to 100 m² and 15 to 19 cm at the median. The crown should read denser and more twiggy at its surface, and the understorey look should remain inside the crown.

**What would refute it:** leaf staying near 32 m² (the limb count or the hazards hold it, not the kind of lateral); leaf over a leaf area index of about 9 on its own ground; a beech that reads as a thicket; the walk over `shade_short` failing its bound.

**What it will not settle.** White's open-grown 0.32 m needs about 266 m² of leaf under the rule. The rows left with the most leverage are estimates with no source (the branch top's BRANCH 0.08, which keeps a branch from replacing itself: 0.64 daughters a life; the lifespans), and the beech stands 10 m tall at 40. If the trunk is still under White's range after this, that is recorded against the beech on the scoreboard and is Phase 4's (species on sources), with the measured gap as its start.

## After the twelve-seed scoreboard: two leaf values (host, 12 Nov 2026)

**The ordinary tree's leaf.** Seen on `phase2-2026-11-12/sheets/ordinary-whole.jpg` beside the base's: a 3 to 4 cm trunk under a collapsing wisp, plainly worse than the base's 22 to 25 cm. Its `leafArea` is the corners' point, 1.67 cm², between a needle and a blade; 23,810 leafy phytomers of that are 4 m² in the grower against 45 m² drawn.

- **Decision:** its `leafArea` is 15.7 cm² on every age, the mean of the oak's 18.1, the beech's 15.7 and the birch's 13.2 (estimated: the ordinary tree is a deciduous broadleaf, as its `leafYears` already says; the point between a needle, a blade and a frond is no tree's leaf).
- **Expected:** about 37 m² of leaf, a section of 113 cm², a 12 cm trunk at 40; it stands. Still under the base's trunk: it is a sparse tree (drawn leaf area index 0.8).
- **Refuted if** it still cannot hold itself up, or its crown changes shape by its own deeper shade.

**The beech's short shoot.** A Fable source check (web, read-only) found no measured lifespan of a *Fagus sylvatica* short shoot in anything retrievable; Roloff 1989 ("Kurztrieblebensdauer", Schweiz. Z. Forstwes. 140: 943-963) is the likely measured source and was not reachable. Read in text: Aas (LWF Wissen 86, the beech's morphology): short-shoot chains reach "gut und gerne 10 - 15 Jahre", a review statement without a measurement; Thiébaut & Puech 1984: a short unit has 3 to 5 internodes (isolated trees); Renard 1971 through Thiébaut & Puech: 77% of leaves on short units [secondary]. The preset's four years is "estimated, from three", set so a drawn leaf area index reached the literature's with blades drawn 2.14 times the measured leaf.

- **Decision:** the short shoot's `lifespan` is 12 (estimated: the middle of Aas's 10 to 15, an expert's statement, not a measurement; to be replaced by Roloff 1989 when read). Its yearly viability stays 0.9.
- **Expected:** a short shoot's expected life rises from 3.4 to 7.2 years where its bearer lasts (shoots live 5, branches 10), so leaf about 1.4 to 1.6 times, the gap to Bartelink's leaf per crown area (3.38 x cpa^1.028; the grower stands at about 2.3 per m²). Trunk about 1.2 times: the beech 19 to 20 cm at 40, the Two Trees about 1.3 m.
- **Refuted if** the leaf per crown projection passes about 5, or the drawn crown (still at 2.14 times blades until Phase 5) reads as a solid mass.
