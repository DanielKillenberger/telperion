# Leaf and shoot survey: oak, beech, birch, spruce, 11 Nov 2026

By a Fable agent with literature sub-agents, read-only; condensed by the host. [code] read in source; [evidence] a recorded result; [full text, extractor] a sub-agent opened the full text through an extraction step, to be spot-checked before a value reaches a preset; [verified] re-queried by the surveyor; [abstract]; [inferred] arithmetic; [U] not determined. Generator figures are cycle medians at 40, seeds 1001 to 1012, from fn-339's `R4-TABLES.md`.

## The findings

1. **Three leaf accounts exist and nothing ties them together [code].**
   - The grower's: each phytomer laid this cycle deposits `leaf_area × scale × size` into the light lattice (`grow/draw.rs:224-229`). Only the year's nodes deposit; `cluster_leaves`, `leaf_years` and `leaf_life` are not read.
   - The drawn: stations = `ceil(run length / twig internodeLength) × stationsPerInternode`, gated by `leaf_years`, clusters at tips, leaf size from the mesh element × `canopy.size` (`telperion-core/src/pipeline/foliage/plan/stations.rs:42-51`, `plan.rs:466-476`). No drawing code reads the grower's `leaf_area`, and no test compares the two.
   - The measure fn-339 used: standing leaf over the tree's own occupied 0.5 m ground columns, smaller than its crown projection.
   - They disagree by 1.2 to 2.4 times in leaf size and count: the beech draws an 8.8 × 5.5 cm blade against a grower's 15.7 cm²; the oak draws a leaf every 8 mm, 10 to 11 on a 5-node unit.
2. **The beech's deficit is real and is in how many shoots it makes.** Leaf 31.6 m² (17.3 to 80.1) on 4,936 shoots. Leaf per leaf is a measured sun leaf; leaves per shoot is 4.07, as published. By Bartelink's leaf per crown projection (3.38 × cpa^1.028) this crown should carry 109 to 145 m²: 3.4 to 9 times today's shoots. The branching shares come from understorey trees aged 21 and 46 (fn-193 SOURCES.md), and the preset reproduces an understorey beech's leaf (about 30 m² on a 46-year, 8.5 m tree). Its short shoot's life was fitted ("lives four years (estimated, from three), so the crown's leaf area index stands at 5.2 at 40", `beech.rs:320-322`) on the drawn leaf, a measure that no longer holds. Which row carries most of the gap is [U].
3. **The spruce's "27 times too many shoots" is mostly a mismatch of tree size.** It compared an open tree of DBH 36 cm and 7 m spread with stand trees of DBH 8.6 to 21 cm. Split [inferred]: crown projection 5.4 times, leaf per crown projection 2.1, shoots per standing leaf 3.3, the last resting on one source's 6% new needles that others contradict (18.6% in the generator; 23 to 39% and a turnover of 0.10 a year elsewhere). Leaf per shoot (12.7 cm² against 13.5 to 13.8) and shoot length (4.0 cm against 4.8) match Bellan et al. 2019. Shoots per metre of branch look high but no source read gives the count on open trees.
4. **The oak is close:** leaf 182.7 m², index 3.54 on its own columns, against a thinned sessile oak stand's 3.9 and an open-grown blue oak's 4.4 per crown projection.
5. **The birch has never been measured** in the generator, and no mature value was found.

## Measured values found

| Species | Measure | Value | Source and read |
|---|---|---|---|
| Beech | leaf per crown projection | la = 3.38 × cpa^1.028 (R² 0.835); 38 trees, 8 to 59 yr | Bartelink 1997, Ann. Sci. For. 54:39-50 [verified] |
| Beech | leaf per tree | 8.560 + 0.0286 × dbh^2.623 m² (dbh in cm) | same, Table II [full text, extractor] |
| Beech | stand LAI | 5.6 to 9.5, mean 7.4 | Leuschner et al. 2006, Plant Ecol. 186:247 [abstract] |
| Beech | long shoots | sun branches 85 to 100% long shoots, shade 40 to 55%; long-shoot internode at least 11 mm, short at most 2.5 mm | Lemoine, Jacquemin & Granier 2002, Ann. For. Sci. 59:761-766, Table II [full text, extractor] |
| Oak | open-grown analogue (*Q. douglasii*) | leaf per crown projection 2.5 to 7.7, mean 4.4; 14 harvested trees | Karlik 2002, USDA proceedings, Table 2 [full text, extractor] |
| Oak | stand LAI | 3.9, thinned 45-yr sessile oak | Bréda et al. 2003, J. Exp. Bot. 54:2403 [extractor] |
| Birch | single leaf | 11.89 cm² mean; 15.0 upper crown | Konôpka & Pajtík 2026, Cent. Eur. For. J. 72:159-169 [full text, extractor] |
| Spruce | new shoots per tree | 691 to 2,372 on 36-yr stand trees | Bellan, Krejza & Marková 2019, Austrian J. For. Sci. 136(1), Tables 1-6 [full text] |
| Spruce | shoot and needle | shoot 4.8 cm; 23.6 needles/cm; 13.0 mm² a needle; needle area = 0.9917 × L^1.640 cm² | same, Tables 3-4 [full text] |
| Spruce | needle life | half a cohort shed at 5.5 yr, all by 12 | Muukkonen & Lehtonen 2004, Can. J. For. Res. 34:2517-2527 [passages] |

No measured counterpart exists for any lateral probability, lifespan, viability, shedding delay, the spruce's whorl and sleeping-bud shares, or any light-response row.

## What follows for the plan (the surveyor's options; the host decides)

- Compare leaf per crown projection and shoots per standing leaf, never per-tree counts against stand trees of a fifth the crown.
- One leaf account: the drawn leaf is the grower's leaf. Sapwood from "leaf above" needs standing leaf, which for an evergreen needs needle retention in the grower (fn-267 names this).
- Beech first: its branching comes from understorey trees. A mechanism also looks wrong: whether a bud makes a long or a short shoot is one fixed table, where Lemoine measures 85 to 100% long shoots in sun and 40 to 55% in shade.
- Spruce: Bellan's needle equation; then shoots per metre of branch.
- Oak: leave the leaf. Birch: measure first.

## Only a build can answer

The birch's leaf and shoots at 40; the spruce's standing leaf by wood age; which beech row carries the gap and whether its weak shade hazard caps the crown once shoots multiply; cost of a beech with 4 to 8 times the shoots.
