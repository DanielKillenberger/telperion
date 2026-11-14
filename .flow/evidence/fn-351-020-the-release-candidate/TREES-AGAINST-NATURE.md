# The candidate's trees against measured trees, 14 Nov 2026

By a Fable reading agent. Nothing was built, run or changed. Findings only.

**Tags.** [code] read in `.worktrees/cand`. [record] a file under `.flow/`. [source, full text] opened in this reading: White 1998 as the PDF's text, the others through a passage-quoting extractor (spot-check a number before it reaches a preset). [source, abstract only] a search excerpt or abstract. [arithmetic] computed here from tagged inputs. [unknown] not determined.

**The candidate's rows.** The candidate's grower equals fn-363's [record, goal log 14 Nov 06:26], so its figures are the "fn-363" rows of `fn-364-…/MEASURES.md` (seeds 1, 7 at 40 and 80; 1001, 1007, 1012 for the old oak), the host's reading of seeds 1001 and 1007 in the log entry of 14 Nov 06:26, `fn-370-…/R1.md` for the grower's and drawn leaf, and `fn-371-…/R3.md` for the ordinary tree. Base figures are `fn-352-…/SCOREBOARD.md` medians of twelve seeds and the same log entry. Every preset's own age is 40 [record, SCOREBOARD.md].

**Three leaves.** A tree here has three leaf areas [record, fn-370 R1]: the grower's (what the girth reads), the settled, and the drawn. The girth reads the grower's leaf on open wood [code, `girth.rs:50-53, 110-121`]. They differ by up to three times, so each row says which one it uses.

## 1. The table

Ratios are candidate over source. "Open" is open-grown, "stand" is forest-grown.

| Tree, age | Candidate: trunk at 1.3 m / height / leaf | Measured, open-grown | Measured, stand | Ratio | Verdict | Confidence |
|---|---|---|---|---|---|---|
| Norway spruce, 40 | 26 cm / 11.6 to 11.8 m / settled 165 to 170 m², grower 346 to 367, drawn 196 to 209 (whole needle surface) | UK amenity trees, n = 27: typical 19 m and 47 cm, height over trunk 40 (S2). No age, no equation | 15.3 ± 4.0 cm and 15.5 ± 3.3 m at about 36 years, 5,000 a hectare (S6). A stand spruce of 11.6 m has 12.6 cm; one of 26 cm stands 20.3 m (S3, Oinas and Sikanen) | Height over trunk 45 against 40 open and about 90 stand. Trunk 2.1 times a stand tree of its height. Leaf 0.95 to 1.9 of 87 to 174 m² at 26 cm (S3, Burger) | **Inside** for an open tree by slenderness. Its trunk is far thicker than a stand tree's of that height | Low. No open-grown spruce diameter by age was reached |
| European beech, 40 | 17 to 18 cm / 10.0 to 10.4 m / grower 61.5 to 72.1 m², drawn 182 to 204 | 32 to 48 cm at 40 (S1, even rings of 4 to 6 mm). UK amenity trees of 10 m: 43 cm, 29 to 62 within one standard error (S2) | 24 cm at 40 inside woodland (S1, 3 mm rings). Stand beeches of 17 to 18 cm carry 57 to 65 m² of leaf (S5, fitted on stand means 3.1 to 27.9 cm, ages 8 to 59: inside) | Trunk 0.35 to 0.56 of open, 0.71 to 0.75 of woodland. Grower's leaf 1.08 to 1.11 of a stand beech of its trunk; drawn leaf 3.1 times | **Slimmer** than an open-grown beech by about half; a stand beech in trunk and grown leaf, drawn with three times that leaf | Medium. White and the amenity trees agree. Height at 40 has no source [unknown] |
| Silver birch, 40 | 20 to 23 cm / 16.7 m / grower 49 to 63 m², drawn 81 to 116 | UK amenity trees of 16.7 m: 39 cm, 28 to 54 within one standard error (S2, R² 0.42). White gives birch no row | A stand birch of 16.7 m has 17.9 cm (S3, Oinas and Sikanen). Planted Finnish stands reach a mean of 26 cm by 50 only with two heavy thinnings (S7) | Trunk 0.51 to 0.59 of open, 1.17 to 1.28 of a stand tree of its height. Grower's leaf 0.8 of 61 to 80 m² at 20 to 23 cm (S3, Martin; a congener) | **Slimmer** than open-grown by 0.5 to 0.6; inside the stand range | Low to medium. The open equation is loose and carries no age |
| Oregon white oak, 40 | 27 to 33 cm / 10.7 to 11.1 m / settled 115 to 180 m² | *Q. robur* 28 to 40 cm (S1). *Q. garryana* at its fastest open growth 26 cm (S8). UK amenity *Q. robur* of 11 m: 52 cm, 34 to 80 (S2) | *Q. garryana* 23 ± 1 cm at 10 m and 40 years (S9). Woodland *Q. robur* 20 cm (S1) | 0.96 to 1.18 of White's low end; 1.04 to 1.27 of the *Q. garryana* ceiling | **Inside** White; up to 1.27 over *Q. garryana*'s fastest | Medium |
| Oak, 80 | 73 and 98 cm / 17.0 and 20.2 m / 562 and 1,016 m² | 56 to 79 cm (S1); *Q. garryana* ceiling 52 cm (S8) | Woodland 40 cm (S1) | Seed 1 inside; seed 7 1.24 times White's top | **Inside to 1.24 times thicker** | Medium |
| Oak, 120 | 1.15 / 1.65 / 1.34 m; 27.4 / 28.4 / 26.0 m; settled leaf 968 / 1,974 / 1,389 m² | 0.83 to 1.09 m (S1). *Q. garryana* ceiling 0.77 m (S8). UK amenity *Q. robur* of that height: 1.15 to 1.25 m, though 95% of those trees stand under 22 m (S2) | 48 cm and 24 m at 95 to 135 years, *Q. garryana* (S10). Woodland *Q. robur* 54 cm (S1) | 1.06 / 1.51 / 1.23 times White's top; 1.5 / 2.1 / 1.7 times the *Q. garryana* ceiling | **Thicker**, by 6% on seed 1001 and by a quarter to a half on the other two | Medium to high for the trunk |
| Oak, 300 | 2.20 / 3.13 / 2.90 m; 36.0 / 38.8 / 38.2 m; settled leaf 590 / 1,111 / 1,371 m² | 1.56 to 1.92 m (S1). *Q. garryana* ceiling 1.94 m (S8). Potential height 34.8 m, record 36.6 m (S10, S11) | 56 ± 5 cm at 25 m and 250 years, *Q. garryana* (S9). Woodland *Q. robur* 96 cm (S1) | 1.15 / 1.63 / 1.51 times White's top. Height 1.03 to 1.11 times the potential | **Thicker** by 15 to 63%, and taller than the species' potential | Medium to high |
| Ordinary tree, 40 | 14 to 17.5 cm / 12.4 to 13.2 m / grower 49 to 73 m² | No species. Amenity trees of 13 m: birch 32 cm, beech 55, oak 61 (S2) | A stand birch of 13 m: 12 cm (S3) | 0.23 to 0.54 of the amenity trees; 1.2 to 1.5 of the stand birch | Between a stand tree and an open one; **slimmer** than open-grown | Low: no species to compare |

**The base, by the same sources.** The base's oak stood 0.78 m at 40 and 1.99 m at 80 [record], which is 1.95 and 2.5 times White's top. The base's beech at 26 to 29 cm sat 0.8 to 0.9 of White's low end, its birch at 26 to 30 cm 0.7 to 0.8 of the amenity birch of its height, its spruce at 34 cm at a height over trunk of 34. On trunks the candidate's oak at 40 and 80 is closer to measured trees than the base's; its beech and birch are further from open-grown trees and closer to stand trees.

**What the sources cannot say.** Every leaf equation reached was fitted on stand trees. No source gives total leaf area of an open-grown spruce, beech, birch or oak of any age [unknown]. No source gives open-grown height at 40 for any of the four [unknown]. The amenity handbook records no ages, so it compares trunk with height only.

## 2. The oak's trunk, by the generator's own accounting

**The rule as coded [code, `girth.rs:62-68`].** Each year a phytomer needs `leaf above × sapwood × 1e-4` of sapwood, keeps `S × (1 − 1/rings)` of what it held, and lays the shortfall; the section T only grows. The oak's rows are `sapwood = 1/0.30` cm² per m² and `sapwoodRings = 20` at every age [code, `oregon-white-oak.values`].

**What that sums to [arithmetic].** While the leaf does not fall faster than one ring in twenty a year:

```text
T(now) = sapwood(now) + heartwood
sapwood(now) = leaf(now) × 3.333 cm²
heartwood = (3.333 / 20) × Σ leaf over every year lived = 0.1667 cm² per m² of leaf, per year
```

Kept heartwood is therefore the sum of every year's retired sapwood. It is not the largest sapwood ever reached. On seed 1001 at 300 the heartwood is 9.9 times the largest sapwood the stem ever held.

**The split, seed 1001 [arithmetic].** Leaf taken as the recorded 113.5, 514, 968, 1,041, 590 m² at 40, 80, 120, 200, 300 [record, log 13 Nov 18:24, V4], quadratic to 40 and straight lines between, run through the coded rule year by year:

| Age | Leaf m² | Sapwood cm² | Heartwood cm² | Sapwood's share | Trunk on paper | Trunk recorded |
|---|---|---|---|---|---|---|
| 40 | 113.5 | 378 | 243 | 61% | 28.1 cm | 27 (seed 1, leaf 115) |
| 80 | 514 | 1,713 | 2,301 | 43% | 71.5 | 73 (seed 1, leaf 562) |
| 120 | 968 | 3,227 | 7,203 | 31% | 115.2 | 115 |
| 200 | 1,041 | 3,470 | 20,591 | 14% | 175.0 | 180 |
| 300 | 590 | 1,967 | 34,220 | 5% | 214.6 | 220 |

The same run on seed 1007's leaf (1,974, 2,198, 1,111 at 120, 200, 300) gives 1.63, 2.51, 3.06 m against the recorded 1.65, 2.59, 3.13; on seed 1012's, 1.38, 2.21, 2.85 against 1.34, 2.27, 2.90. The yearly leaf between the five ages is not recorded, so the lines between them are mine. The rule accounts for every recorded trunk to within 4%, with nothing left over.

**Each input against a source.**

| Input | The rule's value | Source | Factor |
|---|---|---|---|
| Sapwood per leaf | 1/0.30, every age | *Q. garryana*: 0.23 ± 0.02 m² per cm² at 10 m and 40 years, 0.37 ± 0.01 at 25 m and 250 years (S9) | Sapwood 0.77 of the source at 40; 1.23 times it once the tree is 25 m |
| Sapwood rings | 20, every age | European oak 16.0, 20.4, 25.9 rings under 100, 100 to 200, over 200 years; British oak 22.4 and 31.7 (S12, read secondary) | Heartwood laid 0.8 of the source's rate under 100 years; 1.3 to 1.6 times it over 200 |
| Ring area a year, mature tree | 153 / 309 / 289 cm² on the three seeds, 120 to 300 [arithmetic from the recorded trunks] | 76.6 to 109.2 cm², constant: White's Table 2 rows 100/3.5 and 70/5 (S1) | 1.4 to 2.0 / 2.8 to 4.0 / 2.6 to 3.8 |
| Leaf, 120 to 300 | mean about 920 / 1,850 / 1,730 m² [arithmetic: ring area over 0.1667] | No direct source. White's ring area at the sources' old-tree rate (0.37 and 25.9 rings, 0.104 cm² per m² a year) implies 734 to 1,046 m² [arithmetic] | Inside / 1.8 / 1.7 times the top |
| Crown spread | 25.6 / 36.2 / 30.7 m at 120; 34.5 / 50.2 / 53.9 m at 300 [record] | Open-grown *Q. robur*, 95% quantile at White's trunk: 17.4 to 21.8 m at 120, 29.3 to 34.6 m at 300 (S13) | 1.17 / 1.66 / 1.41 at 120; 1.0 / 1.45 / 1.56 at 300 |
| Leaf per own crown area | 1.9 m² per m² at 120, 1.0 at 200, 0.6 at 300 [arithmetic] | 2.5 to 7.7, mean 4.4, open *Q. douglasii* (S14); 3.9, a thinned sessile oak stand (S15) | 0.2 to 0.8 of the low end: thinner than measured, over a wider crown |

**Which input accounts for the excess.** No single input does on every seed and age. Two do, and the arithmetic separates them.

- **At 120 it is the leaf.** The trunk's section follows the summed leaf. Seeds 1007 and 1012 carry 2.04 and 1.43 times seed 1001's leaf at 120, and their sections are 2.06 and 1.36 times its section. Seed 1001, with 968 m², stands at 1.06 times White's champion. Running seed 1001 with the sources' own rows by age class (0.23, 0.30, 0.37; 16.0, 20.4, 25.9 rings) moves its trunk at 120 from 1.152 to 1.126 m [arithmetic]: the rows are not the cause at 120.
- **From 120 to 300 the fixed rows add 1.6 times.** The rule lays 0.1667 cm² of ring per m² of leaf a year at every age. The sources' values for a tree over 200 years and 25 m give 1 / (0.37 × 25.9) = 0.104, so the rule lays 1.60 times as much. With the sources' rows by age class seed 1001 ends at 1.89 m, inside White's 1.56 to 1.92; seed 1007 ends at 2.69 m, still 1.4 times the top [arithmetic].
- **On seeds 1007 and 1012 both multiply.** Ring area 2.8 and 2.6 times White's top = 1.60 (rows) × 1.8 and 1.7 (leaf).
- **What the leaf itself disagrees with.** Total leaf has no direct source. The measured thing it disagrees with is the crown: spread 1.2 to 1.7 times the open-grown 95% quantile at 120, so ground area 1.4 to 2.8 times, beside sections 1.1 to 2.3 times White's top. At 300 on seed 1001 the spread is inside the quantile and the trunk is still 1.15 times over, which is the rows' 1.6 times acting alone.

**What real oaks do that the rule does not.**

- **Sapwood rings rise with age**, 16 to 26 in Hollstein's classes (S12); the row is 20 throughout.
- **Leaf per sapwood rises with height in this species**, 0.23 to 0.37 (S9: "a species in which leaf to sapwood area ratio increases with tree size"); the row is 0.30 throughout.
- **Ring area holds, then falls with the crown.** White: "annual rings in the stem which are of the same cross-sectional area but they progressively decline in width"; after crown damage "the CAI might be reduced to almost nothing" (S1, paragraphs 4 and 14). The rule does lose conducting need when the crown shrinks: need follows the year's leaf, and a year whose leaf falls faster than one ring in twenty lays nothing [code]. A slower fall still lays nearly a full ring. Seed 1001 loses 43% of its leaf from 200 to 300 and lays 121 cm² a year over that century [arithmetic], above White's mature 76.6 to 109.2.
- **Is the recorded peak leaf plausible?** Peaks are 1,041 / 2,198 / 1,887 m² at 200 [record]. The open-grown crown quantile at White's trunk at 200 (23.8 to 28.6 m across) times 4.4 m² per m² gives 1,950 to 2,830 m², and 1,110 to 4,950 across Karlik's range [arithmetic on S13, S14; two species, neither *Q. garryana*]. The peak is plausible as a total. It is held at about 1.0 m² per m² over a crown 36 to 53 m across.

**A cross-check that cuts the other way [arithmetic, weak].** Phillips's two trees, taking even rings and no bark: the 40-year tree (23 cm, sapwood 4.3 cm deep) has about 253 cm² of sapwood, 58 m² of leaf and 12 sapwood rings; the 250-year tree (56 cm, 4.2 cm deep) about 683 cm², 253 m² and 35 rings. The rule's 20 rings sits between them. Those are grove trees, slower than White's.

## 3. The beech at 40

**The split [arithmetic].** Rows: `sapwood = 1/0.331`, `sapwoodRings = 50` [code]. Grower's leaf 61.5 and 72.1 m² on seeds 1001 and 1007 [record, fn-370 R1], grown quadratically over 40 years:

| Seed | Grower's leaf | Sapwood cm² | Heartwood cm² | Sapwood's share | Trunk on paper | Trunk recorded |
|---|---|---|---|---|---|---|
| 1001 | 61.5 m² | 186 | 48 | 80% | 17.2 cm | 17 |
| 1007 | 72.1 m² | 218 | 56 | 80% | 18.7 cm | 18 |

- **The sapwood row is at its source.** Bartelink's 0.331 m² per cm² was checked against the text [record, PHASE4-SURVEY.md]. His own equation gives a stand beech of 17 and 18 cm 56.8 and 64.7 m² of leaf (S5); the grower's leaf is 1.08 and 1.11 times that. Trunk and grown leaf agree with each other and with a stand beech.
- **The ring count barely matters at 40.** At 25 rings, the low end of Gebauer's 25 to 73 [record], seed 1001's trunk is 18.9 cm instead of 17.2 [arithmetic].
- **The leaf is low.** An open-grown beech of 40 stands on 32 to 48 cm (S1). Bartelink's equation, stretched past its largest stand mean of 27.9 cm, puts 262 to 744 m² on such a trunk [arithmetic, extrapolated]; the grower holds 61.5 to 72.1.
- **The drawn crown is three times the grown one.** The tree is drawn with 182 and 204 m² [record]. Had the grower held the drawn leaf, the same rows would lay a trunk of 29.7 and 31.6 cm [arithmetic], beside the base's 26 and 29 cm and White's 32 to 48.

So the beech is slim because the grower's leaf is low. Its sapwood row is not low. The trunk looks slimmer still because the drawing hangs 2.8 to 3.0 times the grown leaf on it.

**The other young trees by the same run [arithmetic].** Birch (4.6 cm² per m², 40 rings; grower's leaf 63.1 and 48.8 m²): 22.1 and 19.4 cm on paper, 23 and 20 recorded. Spruce (1/0.36, 30 rings): the settled 165 to 170 m² gives 29 cm against 26 recorded; the probe's grower figure of 346 to 367 m² would give 42 to 43 cm, so the girth does not read that figure. The probe's note says it counts wood that closed with its leaf on [record].

## 4. Unknown

- Total leaf area of an open-grown tree of any of the four species, at any age. Every equation used was fitted on stand trees.
- Open-grown height at 40 for spruce, beech and birch, and open-grown diameter by age for spruce and birch. White's table has no spruce or birch row. Hasenauer 1997 (open-grown trees in Austria) did not load, and a thesis quoting its parameters returned nothing through the extractor.
- A yield table's height and mean diameter at 40 for beech, spruce or birch: none was opened. The stand figures above are one Czech spruce stand, White's woodland rows and a Finnish height curve quoted in a compendium.
- Whether the amenity handbook's equations hold at the candidate's sizes: it states no fitted range past its filters (height 3 to 50 m, trunk 10 to 400 cm), and the old oak's 26 to 39 m is above the 22 m that 95% of its oaks stand under.
- The species behind the birch leaf equation (Martin et al. 1998), quoted in the JRC compendium among birch equations and not opened; most likely an American birch.
- The yearly leaf of the oak between 40, 80, 120, 200 and 300 on any seed; the leaf of seeds 1007 and 1012 at 40 and 80.
- Which leaf the spruce's girth sums. The settled figure fits the trunk within 3 cm; the cause of the rest was not traced.
- Sapwood ring counts for *Q. garryana*. Hollstein's and Hillam's are European and British oak, read secondary.
- Whether Karlik's blue oak and a thinned sessile oak stand bound the leaf per crown area of an open *Q. garryana*.
- Why the grower's beech holds a stand tree's leaf when it grows alone.

## Sources

- **S1** White 1998, Forestry Commission Information Note 12, Tables 1a and 2 [source, full text]. Oak rows "70/5", "80/4", "100/3.5", inside woodland "70/2.5"; beech "80/6", "70/5", "70/4", inside woodland "120/3"; ring areas 109.2, 79.9, 76.6, 27.3 cm². Diameters by age are arithmetic on those rows and equal `PHASE2-SURVEY.md`'s.
- **S2** Handbook of UK Urban Tree Allometric Equations and Size Characteristics, v1.4 [source, full text, extractor]. "Trees grown in woodland conditions are not likely to be well represented". DBH from height: birch `exp(1.45866) × H^0.76676 × exp(0.05237)` (R² 0.42, SE 0.324, n = 231); beech `exp(1.36669) × H^1.00453 × exp(0.07088)` (0.54, 0.377, n = 149); *Q. robur* `exp(1.68021) × H^0.91195 × exp(0.09454)` (0.41, 0.435, n = 1,008). *Picea abies* n = 27, typical 19.0 m and 47.0 cm, no equation.
- **S3** Widlowski et al. 2003, Allometric Relationships of Selected European Tree Species, EUR 20855 EN [source, full text, extractor; a compendium, each equation secondary]. Oinas and Sikanen 2000 height curves; Burger 1941 spruce leaf, "empirically verified up to DBH ≈ 50 cm"; Martin et al. 1998 `0.148252 DBH^2.008`, "up to about 40 cm"; Gower et al. 1993 gives 154 m² at 26 cm.
- **S5** Bartelink 1997, Ann. Sci. For. 54: 39-50: `la = 8.560 + 0.0286 × dbh^2.623` [record: full text, PHASE4-SURVEY.md].
- **S6** Bellan, Krejza and Marková 2019 [record: full text].
- **S7** Thinning silver birch stands in Finland, Trees, Forests and People 2025 [source, abstract only: search excerpt]: "the highest mean diameter of 26 cm and dominant diameter of 29 cm by the age of 50 were attained exclusively through a thinning program that comprised two intensive thinnings".
- **S8** Gould, Marshall and Harrington 2008: "The maximum predicted diameter growth of an open-grown oak (SI, 100) is 1.27 in./5years" [record: fn-341 R1-MEASURES.md].
- **S9** Phillips et al. 2003, Funct. Ecol. 17: 832-840, Table 1 [record: full text, PHASE2-SURVEY.md]; the quoted sentence is its abstract, read here.
- **S10** Stein 1990, Silvics of North America. **S11** Gould, Harrington and Devine 2011 [record: fn-341 R1-MEASURES.md].
- **S12** Hollstein and Hillam in Haneca, Čufar and Beeckman 2009, Table 1 [record: secondary].
- **S13** Pretzsch et al. 2015, Table 2, `ln(cr) = −1.4370 + 0.8157 ln(d)` [record: fn-341 R1-MEASURES.md].
- **S14** Karlik 2002. **S15** Bréda et al. 2003 [record: PHASE4-SURVEY.md].
