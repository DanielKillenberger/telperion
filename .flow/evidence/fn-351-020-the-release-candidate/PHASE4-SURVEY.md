# Phase 4 survey: species on sources, 14 Nov 2026

By a Fable agent, read-only, at the top of Phase 4. Nothing was built, run or measured. Findings only: what a preset becomes is the host's to decide.

**Tags.** [code] read in source on the `fn-364` worktree (`crates/telperion-space/src/*.rs`, `crates/telperion-core/presets/*.values`). [record] a file under `.flow/` or the goal log. [source, full text] I opened the paper's full text in this survey through a passage-quoting extractor (not a viewed PDF: spot-check a number before it reaches a preset). [source, abstract only] I read the abstract only. [unknown] not determined. "Arithmetic" marks a figure I computed on paper from tagged inputs; it is not a measurement.

**What I did not do.** I did not diff every zone row of the oak, beech and birch `.values` files against their `.rs` tables. I read the `.rs` tables in full and checked the species-wide rows in the `.values` (leaf area, inverse height, hazards, sapwood, `shadeShort`, coarse). The spruce I checked row by row: its `.values` and `spruce.rs` agree [code]. The log says `tests/space_presets.rs` holds the two equal [record].

## 0. The findings that change the phase

1. **The beech is compared against the wrong published measure.** The scoreboard sets one tree's leaf over its own crown projection (10.0, 12.6, 11.6 at 40, 60, 80 [record, `BEECH-LAI.md`]) beside a stand's leaf area index (5.6 to 9.5). Bartelink's own trees give the like-for-like figure: leaf = 3.38 × crown projection^1.028 [source, full text], which is 3.7 to 4.0 m² of leaf per m² of a tree's own crown projection, in closed stands whose stand index was 5.5 to 7.2 [source, full text]. Against that the head's beech carries 2.3 times the leaf at 40 and 3.0 times at 80 [arithmetic], not a quarter to a third over. Bartelink's trees are stand trees of 8 to 59 years; no open-grown beech value was found [unknown].
2. **The spruce's "shoots a year" follows from three rows, and one of them is misread.** New shoots a year = standing needle area ÷ `leafYears` ÷ needle area per shoot [arithmetic on code]. Needle per shoot matches Bellan (13.4 cm² against about 13 to 14) [code; source, full text]. `leafYears` 5.5 is the age at which half a cohort is shed, used as the age at which all of it is shed. The same abstract gives a needle turnover of 0.10 a year [source, abstract only]: one tenth of the needle is new each year, where the preset makes 1/5.5 = 18% new. That row alone puts the spruce's shoots per unit of needle 1.8 times over.
3. **Bellan's 691 to 2,372 cannot be the range for the preset's tree.** Those are stand trees of 15.3 cm, planted at 5,000 a hectare [source, full text], each making about 2.5 m² of new needle a year [arithmetic]. Two published equations give a spruce of the preset's size 110 to 310 m² of standing needle [source, full text; arithmetic, extrapolated to an open tree], hence about 8,000 to 42,000 shoots a year at 13.4 cm² a shoot. The generator's last recorded count is about 65,000 on 441.6 m² [record]. The excess is 1.5 to 8 times, not 27.
4. **The light response of every species is the oak's fitted values, scaled.** `tolerance.rs` sets beech, spruce and birch hazards as the oak's 1.0 and 2.0 times (5 − T) / 2.6 [code]. The oak's two numbers are "host values from fn-197's renders, not sourced" [record, fn-195 `SOURCES.md`]. T is sourced; the mapping and its anchor are not.
5. **The spruce's ledger no longer lists the spruce's values.** fn-194 `SOURCES.md` gives branchlets 25 years at 0.98, dominance 0.25, shoots keeping needles 4 more years; the code has 10 years at 0.99, a 40-year spur age the ledger never names, dominance 0.35 and `leafYears` 5.5 [code; record]. By the owner's rule those rows have neither a source nor a listed estimate today.

## 1. Every shoot and leaf row, by species

Status words, as asked: **checked** = I opened the source in this survey and the number is in its text; **cited, not reachable** = a source is cited and I could not reach its text; **cited, not opened** = cited, an earlier record marks it read, I did not reopen it; **listed estimate** = the code comment or the species' ledger says estimated or derived; **not listed** = no source and no entry that names today's value.

### 1.1 Norway spruce (`norway-spruce.values`, `spruce.rs`)

Preset ages: 0 seedling, 1 sapling, 2 trunk, 4 leader, 8 sprig, 9 branch, 10 branchlet, 11 spur, 12 shoot [code].

| Row | Value [code] | Source the comment cites | Status |
|---|---|---|---|
| `leafArea`, every age | 0.033565 m² per metre of shoot × the age's internode (0.000201 to 0.00168 m² a node) | mid-point of 18 to 31 needles a cm (Bellan et al. 2019) × 13.7 mm² a sun needle (Gebauer et al. 2019, Table 2) | **Listed estimate (derived).** The 18 to 31: **checked**, "The number of needles per cm of shoot length ranged from 18 to 31" [source, full text]. The 13.7 mm²: population **checked** ("~20-year-old Norway spruce trees that had been planted in a 1 × 1 m grid ... 10,000 trees ha-1", height 8.5 m, current-year needles), the number itself was not in the extract [source, full text; number unknown]. A product of two stands and a mid-point of a range. |
| `leafYears`, every age | 5.5 | Muukkonen & Lehtonen 2004; Reich et al. 1996 ("between 6.4 and 7.2 needle age cohorts") | **Checked**, abstract: "At the age of 5.5 years, 50% of the needles in the needle cohort have been shed. In addition, at the age of 12 years, all needles of the needle cohort have been shed." [source, abstract only]. The comment itself says "estimated in form". Modelled for southern Finland. Reich: cited, not opened. |
| `inverseHeight` | 1/61 | USDA FEIS, "heights of up to 203 feet (61 m) have been reported" | Cited, not opened. A reported maximum. |
| `shadeHazard` 0.2115, `balanceHazard` 0.4231 | oak's 1.0 and 2.0 × (5 − 4.45)/2.6 | T 4.45, Niinemets & Valladares 2006, App. A; the mapping is fn-264 host decision 4 | T: cited, not opened [record says checked]. Mapping and anchor: **listed estimate** ("no source gives the engine's response rows", `tolerance.rs`). |
| `form/sapwood` | 1/0.36 cm² per m² | McDowell et al. 2002, Oecologia 132, Table 1, 28 spruces of 7 to 31 m | Cited, not opened. |
| `form/sapwoodRings` | 30 | none ("no ring count was read") | **Listed estimate.** |
| `internode` | seedling 0.02, sapling 0.028, trunk 0.042, leader 0.03, sprig 0.05, branch 0.034, branchlet 0.016, spur 0.006, shoot 0.02 m | none | **Listed estimate** (ledger: "Pipes, exponents, internodes ... estimated from the stills"). Shoot 2 nodes × 0.02 = 4.0 cm against Bellan's mean 4.8 cm: "The mean LSHOOT was 4.8 cm" [source, full text]. |
| Trunk unit nodes | bare 2–3 / 4–5 / 5–6, medial 1–2, whorl 1 | M3 (one whorl a unit): Prusinkiewicz & Remphrey 2000 | Rule cited, not opened; counts **listed estimate**. |
| Whorl: `buds` 5, `lateral/8` 0.85 | about 4.25 branches a whorl | M14 | **Listed estimate** ("S4 shows 3 to 4 lateral buds"). No measured count of branches per whorl was reached [unknown]. |
| Medial `lateral/8` | 0.25 sapling, 0.3 trunk and leader | M9, CIRAD lecture | **Listed estimate** ("share estimated"). |
| Branch zones: `buds` 2, `lateral/10` 0.7 and 0.95 | 4.7 new branchlets a branch a year [arithmetic] | M5, M7 | **Listed estimate** ("counts estimated"). |
| Branch `dormant/10` 0.45, `delay` 1, `rate` 0.3 | 3.15 sleeping branchlet buds a unit [arithmetic] | M10; "The share asleep is tuned to S1's dark lower curtains" | **Not listed** in the ledger; the comment names it tuned to a reference image. |
| Branchlet zones: `lateral/12` 0.55 (2 nodes × 2 buds), 0.8 (1 × 3) | 4.6 new shoots a branchlet a year [arithmetic] | none | **Not listed.** |
| Spur zone: `lateral/12` 0.6, 2 buds | 1.2 shoots a year | none | **Listed estimate** ("for spray width"). |
| Shoot nodes | 2 a year | none | **Listed estimate.** |
| `lifespan` | seedling 2, sapling 8, trunk 35, leader 300, sprig 6, branch 300, branchlet 10, spur 40, shoot 2 | "for as long as a Norway spruce lives (300 years)", no source | Seedling, sapling, trunk, sprig: **listed estimate**. Branchlet 10 and spur 40: **not listed** (ledger says 25). Leader and branch 300: **not listed** (no source for 300). |
| `viability` | branch and sprig 0.998, branchlet and spur 0.99, shoot 0.9 | none | Branch: **listed estimate**. Branchlet 0.99: **not listed** (ledger says 0.98). Shoot 0.9: **not listed**. |
| `shedding` | branch 3, branchlet and spur 1, shoot 3 | M11, CIRAD | Branch: **listed estimate**. Others **not listed**. |
| `form/dominance` | 0.35 sprig and branch | none | **Not listed** (ledger says 0.25, "for variety between seeds"). |
| `rhythm` | 0.3 seedling, 0.7 sapling | M4 | **Listed estimate.** |
| `/space/coarse/*` (six rows) | first 8, factor 2.693, leaf 18.645, bud 0.982, hazard 3.759, shedding 0.051 | fn-328's fit on seeds 1001 to 1050 | Fitted per species. Phase 5 deletes them [record, plan]. |

### 1.2 European beech (`european-beech.values`, `beech.rs`)

| Row | Value [code] | Source the comment cites | Status |
|---|---|---|---|
| `leafArea`, every age | 0.0015723 m² | Hladká & Čaňová 2005, J. For. Sci. 51(4), Table 2 | **Checked.** Table 2, sunlit, average: 15.723 cm², length 5.58, width 3.785; shaded 21.648 [source, full text]. The tree: age 80, d1.3 50 cm, height 35 m, crown width 9.4 m, crown projection 69.36 m². One tree, one year (1998). |
| Short shoot `cluster_leaves` | 4 | Letort et al. ("usually 3 or 4"); the same Hladká paper | **Checked** for Hladká: "There were mainly 4 leaves on shoots, 7 to 9 leaves on terminal ones" [source, full text]. Letort: cited, not opened. |
| `shadeShort/14` on branch and shoot | 0.38 | Lemoine, Jacquemin & Granier 2002, Table II | **Checked.** Table I: PAR 1850 ± 50 (sun), 255 ± 55 (shade). Table II, percent long lateral shoots: 85% ± 3% sun, 60% ± 5% thinned, 40% ± 6% shade; apical 100%, 65%, 55%; n = 16 [source, full text]. "Five dominant 30-year-old Fagus sylvatica L. trees ... Trees were growing in a closed stand"; branches less than six years old. The exponent's form is a **listed estimate** ("a power law through the two settled classes"). |
| Full-light long share in medial zones | 85% long: branch SHOOT 0.53 / SHORT 0.17; reiterates BRANCH 0.485 / SHORT 0.085 | Lemoine Table II for the split | Split **checked** (as above). The zone totals it splits (0.7, 0.57) are **listed estimates** (fn-193: "A24 ∈ [0.5, 0.7] read as a probability"; reiterates "estimated"). |
| Branch spare zone `SHOOT` | 0.65 | Letort Table 2, A23 ∈ [0, 0.2] | **Listed estimate**: "raised to 0.65 (estimated)" [record]. Three times the top of the source's range. |
| Branch top `BRANCH` 0.08; shoot `SHORT` 0.6 | | Letort Table 2 (A22 ∈ [0, 0.2]; A34 ∈ [0.55, 0.6]) | Cited, not opened. Letort's trees are understorey trees aged 21 and 46 [record]. |
| Short shoot `lifespan` | 12 | Aas, LWF Wissen 86, "gut und gerne 10 - 15 Jahre" | **Listed estimate** ("an expert's statement, not a measurement; to be replaced by Roloff 1989 ... when read"). |
| Short shoot nodes 3–5, `viability` 0.9 | | Dupré et al. 1986 "3 à 5 entre-nœuds courts"; viability none | Nodes: cited, not opened. Viability: **listed estimate**. |
| `internode` | trunk 0.07, reiterates 0.04, branch 0.03, shoot 0.025, short 0.006 m | none ("internode lengths were not found in readable sources") | **Listed estimate.** |
| Unit nodes | trunk 6–8, reiterates 5–7, branch 5–8, shoot 3–4 | Letort Fig. 2; Dupré "6 à 10 entre-nœuds" (seedlings) | Cited, not opened. |
| `lifespan` | trunk 9, leader 40, fork 2, limb 70, bough 30, spur 25, branch 10, shoot 5 | none | **Listed estimate.** |
| `viability`, `shedding` | reiterates 0.995; branch and shoot 0.95, shed after 1 | Kint 2010 "die late but shed fast" (abstract) | **Listed estimate.** |
| Reiterate tops: LIMB, BOUGH, SPUR shares (0.3/0.2, 0.16, 0.14, 0.45, 0.9 ...) | | Millet et al. 1998 for the rule | Rule cited, not opened; every number **listed estimate**. |
| `inverseHeight` | 1/40 | waldwissen.net, "it may reach over 40 m" | Cited, not opened. The comment calls it a lower bound; a portrait page, not a measurement. |
| `shadeHazard` 0.1692, `balanceHazard` 0.3385 | oak's × (5 − 4.56)/2.6 | T 4.56, Niinemets & Valladares 2006 | T cited, not opened; mapping **listed estimate**. |
| `form/sapwood` | 1/0.331 | Bartelink 1997 | **Checked**: "area ratio of 0.331 m² cm⁻² (R² = 0.926); however, the relationship differed significantly between stands" [source, full text]. 38 trees, six even-aged stands, cutting ages 8 to 58/59, the Netherlands. |
| `form/sapwoodRings` | 50 | middle of Gebauer, Horna & Leuschner 2008's 25 to 73 | **Listed estimate.** |

No `/space/coarse` row in this preset [code].

**A stale comment in the product file [code].** `european-beech.values` lines 277 to 281 still say "The short shoots live four years (estimated, from three), which brings the crown's leaf area index to 5.2 at 40 and 7.3 at 80". Lines 595 to 599 of the same file say twelve. fn-193 `SOURCES.md` still lists "short 3 at 0.9".

### 1.3 Oregon white oak (`oregon-white-oak.values`, `oak.rs`)

The engine species is *Quercus robur*; the preset's name is *Q. garryana* [code].

| Row | Value [code] | Source the comment cites | Status |
|---|---|---|---|
| `leafArea` | 0.00181 m² | Visakorpi et al. 2020, PLoS ONE 15: e0228157, Table 1: 616 *Q. robur* leaves, ten Oxfordshire oaks | Cited, not opened [record says read]. A *Q. robur* leaf on a *Q. garryana* preset; no *Q. garryana* leaf area is cited. |
| `inverseHeight` | 1/34.8 | Gould, Harrington & Devine 2011 | Cited, not opened; fn-341 checked it against the text [record]. |
| `shadeHazard` 1.0, `balanceHazard` 2.0, `shadeSize` 1.5 / 0.5 (limb, bough) / 1.0 (branch, twig, shoot, short), `apical_control` 0.45 | | "Light as the oak's own values (fn-195 round 5 ...; host decision 29)" | **Listed estimate**: "Host values from fn-197's renders, not sourced" [record]. These anchor the other three species. |
| `form/sapwood` | 1/0.30 | Phillips et al. 2003, Funct. Ecol. 17, Table 1 (0.23 and 0.37 on trees of 10 and 25 m) | Cited, not opened. A mid-point of two values. |
| `form/sapwoodRings` | 20 | Hollstein in Haneca et al. 2009 (European oak) | **Listed estimate** ("read secondary ... Estimated for Q. garryana"). |
| Sprig `internode` 0.031 (5 nodes, 15.7 cm a year) | | Hibbs & Yoder 1993 via FEIS, "sapling growth averaged 6.2 inches (15.7 cm)/year" | Cited, not opened. A sapling's stem growth applied to a lateral. |
| Other `internode` | trunk and fork 0.045, leader 0.05, limb 0.035, bough 0.03, branch 0.017, twig 0.012, shoot 0.015, short 0.006 | none; limb "so a lit limb does not outgrow the dome" | **Listed estimate.** |
| Unit nodes, `buds` 3 at tops | | Barthélémy & Caraglio 2007 (acrotony) for the rule | Rule cited, not opened; counts **listed estimate**. |
| Lateral shares (fork LIMB 0.8; limb LIMB 0.007, BOUGH 0.08; SHORT 0.25 to 0.7; BRANCH, TWIG shares) | | Kędra 2019 for forking; fn-236 rounds | **Listed estimate.** Ledger rows are older than the code: it lists fork 0.55, the code has 0.8 "so no seed's crown stands on two limbs alone" [code; record]. |
| Sleeping buds: `dormant` 0.1, `delay` 6, `rate` 0.04 | | Tomlinson 1983; B&C 2007 for the rule | **Listed estimate** ("share, delay and hazard estimated"). `PHASE3-SURVEY.md` holds Fontaine's fates [record]. |
| `lifespan` | trunk 12, fork 2, leader 6, limb 500, bough 60, sprig 10, branch 12, twig 5, shoot 4, short 3 | none | **Listed estimate**; leader 6 and twig 5 are fn-236 round values. |
| `viability`, `shedding` | bough 0.999 / 4; sprig 0.95 / 3; branch 0.98 / 3; twig 0.98 / 2; shoot 0.93 / 2; short 0.9 / 2; limb shed 5 | "an arborists' account, no measured persistence found" | **Listed estimate.** |
| `/space/coarse/*` | first 9, factor 1.687, leaf 26.289, bud 0.82, hazard 2.385, shedding 0.243 | fn-328 | Fitted per species; Phase 5's. |

### 1.4 Silver birch (`silver-birch.values`, `birch.rs`)

| Row | Value [code] | Source the comment cites | Status |
|---|---|---|---|
| `leafArea` | 0.00131906 m² | Flora of China blade 3–7.5 × 1.5–6 cm, mid-points, × 0.67 (Schrader et al. 2021, Table 1) | **Listed estimate (derived)**: no measured leaf; a shape factor for a leaf class. |
| Short shoot `cluster_leaves` | 3 | Kull, Tulva & Vapaavuori 2003, "two or three leaves" | Cited, not opened [record says full text]. Nine-year-old trees in open-top chambers [source, abstract only]. |
| Medial shoot share 0.85 (SHORT 0.845 branch, 0.58 limb, 0.45 bough and sprig) | | Suzuki 2003, ">80 %" of current-year shoots short, on *B. maximowicziana* | **Listed estimate** ("the 0.85 estimated"); a congener. |
| `inverseHeight` | 1/30 | European Atlas of Forest Tree Species, "... 30 m" | Cited, not opened; "B. pendula inferred from the excerpt". |
| `shadeHazard` 1.1423, `balanceHazard` 2.2846 | oak's × (5 − 2.03)/2.6 | T 2.03 | T cited, not opened; mapping **listed estimate**. |
| `groundHazard` | 1.0 | owner's verdict of 14 Oct 2026 | **Listed estimate** ("a declared stand-in"). |
| `form/sapwood` | 4.6 | Repola 2008 over Hölttä et al. 2013 at Lintunen et al. 2011's SLA | **Listed estimate** ("derived, not measured"). |
| `form/sapwoodRings` | 40 | none | **Listed estimate.** |
| `internode`, unit nodes, every lateral share, `lifespan` (trunk 5, young 9, leader 26, top 100, limb 25, bough 100, sprig 14, branch 12, short 6), `viability`, `shedding`, sleeping buds (0.2, delay 4, rate 0.15) | | Millet et al. 1999 (grey birch) and Jones & Harper 1987 for the rules | Rules cited, not opened. Every number **listed estimate**: `MODEL-BIRCH.md` rounds 2 to 14 each say "estimated from the stills" [record]. |
| Short `form/primary` 0.0005 | | none | **Listed estimate**, fitted: "holds the stem's DBH to round 12's at 40 and 80" [code]. |
| `/space/coarse/*` | first 8, factor 1.503, leaf 10.045, bud 1.534, hazard 2.366, shedding 0.65 | fn-326 | Fitted per species; Phase 5's. |

### 1.5 Date palm and the rest, noted separately

- **Date palm** [code]: `leafArea` 2.81 m² a frond (Abdel-Sattar et al. 2024, Table 2; length × width × 0.8, so an outline: **listed estimate**, derived). 12 fronds a year: **listed estimate**. `internode` 0.0339: fitted "to keep the palm at 40 at today's 21.59 m", inside A1's "30-45 cm" a year. `lifespan` 150 (the ledger says 1000: **not listed**). `inverseHeight` 0: the palm takes no potential height. No coarse rows.
- **Ordinary, Telperion, Laurelin**: not real species. The Two Trees take the beech's rows and its inverse height [record], so every beech finding below reaches them.

### 1.6 Counts, by distinct row kind [arithmetic on the tables above]

| Preset | Checked in this survey | Cited, not opened | Listed estimate | Not listed |
|---|---|---|---|---|
| Spruce | 1 (`leafYears`, abstract) and half of `leafArea` | 4 | 12 | 7 |
| Beech | 4 (`leafArea`, cluster of 4, `shadeShort`, `sapwood`) | 5 | 9 | 0 (one stale comment) |
| Oak | 0 | 4 | 8 | 0 (ledger older than code) |
| Birch | 0 | 3 | 7, covering nearly every row | 0 |

A row kind covers several ages. The log's count of estimates per preset on 11 Nov was not found [unknown], so "lower than on 11 Nov" (P6) cannot be shown from the record I read.

## 2. The two named gaps, from primary literature

### 2a. European beech leaf

| Measure | Value | Paper and place | Population | Read |
|---|---|---|---|---|
| Leaf area from diameter | la = 8.560 + 0.0286 × dbh^2.623 (SE 0.0213, 0.218; R² 0.932), m² and cm | Bartelink 1997, Ann. Sci. For. 54: 39–50, Table II | 38 trees, six even-aged stands, cutting ages 8 to 58/59, mean stand diameters 3.1 to 27.9 cm, 360 to 9,920 stems a hectare, acid brown podsolic soils, centre of the Netherlands | [source, full text] |
| Leaf area from diameter and height | la = 0.2410 × dbh^2.899 × h^−0.984 (R² 0.942) | same, Table III | same | [source, full text] |
| Leaf area from crown projection | la = 3.38 × cpa^1.028 (R² 0.835) | same, eq. 4 | same | [source, full text] |
| Leaf area from sapwood | 0.331 m² per cm² at breast height (R² 0.926); la = 0.341 × sa − 1.674 × h_cb (R² 0.982), 20 trees | same, eq. 6 | same | [source, full text] |
| Specific leaf area | mean 172 cm² g⁻¹; 80 to 120 at the top, 300 to 340 at the crown base; a 2 m solitary tree 100 to 180 | same | same | [source, full text] |
| Stand leaf area index | "LAI in the closed-canopy stands generally varied between 5.5 and 7.2"; "LAI reached a maximum of seven; the leveling off was ascribed to self-thinning" | same | same | [source, full text] |
| Stand leaf area index | 7.4 (5.6 to 9.5), 23 closed mature stands, falling with stand age | Leuschner et al. 2006, Plant Ecol. 186: 247 | Germany, ages not in the abstract | [record: abstract only]; not reopened |
| One leaf, sunlit | 15.723 cm² (median 15.21, SD 4.2); shaded 21.648 | Hladká & Čaňová 2005, Table 2 | one model tree, 80 years, 35 m, d1.3 50 cm, crown projection 69.36 m², upper level 32 m, lower 19 m, Poľana | [source, full text] |
| One leaf, canopy top | 7.88 ± 0.56 and 8.52 ± 0.51 cm² (non-fruiting, fruiting shoots, ambient CO2); 9.18 and 10.30 under elevated CO2 | Han, Kabeya & Hoch 2011, Ann. Bot. 107: 1405, Table 3 | four trees at ambient, three at elevated; stand of 80 to 120 years, 30 to 35 m, 415 trees a hectare, near Basel; a mast year | [source, full text] |
| Leaves per shoot | "mainly 4 leaves on shoots, 7 to 9 leaves on terminal ones" | Hladká & Čaňová 2005 | as above | [source, full text] |
| Leaves per current-year shoot | 3.2 ± 0.2 non-fruiting, 4.7 ± 0.2 fruiting; fewer in mast years | Han et al. 2011, Table 3 | as above, canopy top | [source, full text] |
| Short and long units | "Buds with less than four and more than five leaf primordia formed short and long growth units, respectively" | Cochard et al. 2005, Tree Physiol. 25: 1545 | not in the abstract | [source, abstract only] |
| Share of long shoots | lateral 85 / 60 / 40%, apical 100 / 65 / 55% (sun / thinned / shade) | Lemoine et al. 2002, Table II | five dominant 30-year trees, closed stand, Hesse forest, France; branches under six years | [source, full text] |
| Long-shoot definition | "We classified long shoots as shoots with internodes longer than 5 mm" | same, Methods | same | [source, full text] |

**A disagreement with an earlier record.** `LEAF-SURVEY.md` gives Lemoine's threshold as "long-shoot internode at least 11 mm, short at most 2.5 mm" [record]. The text I read says 5 mm [source, full text]. One of the two extractions is wrong; which is [unknown] until the PDF is viewed.

**Arithmetic on the head's beech** (`BEECH-LAI.md` medians of five seeds [record]; equations above):

| Age | Drawn leaf | Leaf's ground hull | DBH | Bartelink from crown projection | Bartelink from diameter | Leaf by the preset's own sapwood row |
|---|---|---|---|---|---|---|
| 40 | 326 m² | 37.7 m² | 22.2 cm | 141 m² (2.3 times under) | 106 m² (3.1 times under) | 128 m² |
| 80 | 3,534 m² | 299 m² | 84.4 cm | 1,186 m² (3.0 times under) | outside the sample (largest stand mean 27.9 cm) | sapwood is not the whole section at 80; not computed |

- Bartelink's equation gives 3.38 × cpa^0.028 = 3.7 to 4.0 m² of leaf per m² of a tree's own crown projection across the hulls above [arithmetic]. Stand index is higher than that because crowns overlap.
- At 80 the crown projection is far past Bartelink's trees, so the 1,186 m² is an extrapolation.
- At 40 a 22.2 cm stem is 387 cm² of section; at 0.331 m² a cm² that is 128 m² of leaf, where 326 m² is drawn [arithmetic]. Why the drawn leaf is 2.5 times what the trunk's own rule accounts for is [unknown]; the log of 12 Nov recorded drawn blades at 2.1 times the grower's leaf at 40 and agreement at 80 [record].
- Hladká's 80-year tree, 50 cm and 69.36 m² of projection, would carry about 264 m² by eq. 4 [arithmetic, extrapolated]. The head's beech at 80 has four times that projection and 13 times that leaf.

**Not found [unknown]:** total leaf area, or leaf per crown projection, of an open-grown beech of any age. A study of solitary trees planted in 2001 (Sjöman et al. 2021, Arboric. Urban For. 47: 252) measured plant area index inside the crown projection for 64 taxa by a canopy imager, range 1.93 to 8.15 across taxa [source, full text]; whether *Fagus sylvatica* is among them and its value were not in the extract [unknown].

### 2b. Norway spruce shoots and needle

| Measure | Value | Paper and place | Population | Read |
|---|---|---|---|---|
| New shoots a tree a year | mean 1,707; "varied between 691 - 2372 (from sub-dominant trees to dominant trees)" | Bellan, Krejza & Marková 2019, Austrian J. For. Sci. 136(1), "Shoots" | 7 trees, DBH 15.3 ± 4.0 cm, height 15.5 ± 3.3 m; monoculture "established through reforestation (5000 trees per ha) after a clearcut ... in 1978"; Rájec, CZ | [source, full text] |
| Shoot length | "The mean LSHOOT was 4.8 cm"; longest shoots in the top fifth of the crown; sub-dominant trees made no new shoot in the lowest fifth | same | 48 sampled shoots | [source, full text] |
| Needles a tree a year | 188,453 (model) | same | same | [source, full text] |
| Needles a cm of shoot | 18 to 31; cited beside 6 to 63 (Sander & Eckstein 2001) and 13 to 23 (Stenberg et al. 1999) | same, Discussion | same | [source, full text] |
| Needle area from shoot length | 0.9917 × L^1.640 cm² (R² 0.91 confirmed; the coefficients were not in my extract) | same, Table 4 | same | coefficients [record, `LEAF-SURVEY.md`]; R² [source, full text] |
| Needle retention | half a cohort shed at 5.5 years, all by 12; needle biomass turnover 0.10 a year; branch biomass turnover 0.0125 | Muukkonen & Lehtonen 2004, Can. J. For. Res. 34: 2517 | modelled, southern Finland | [source, abstract only] |
| Maximum needle age | *Picea abies* up to 12 years; higher branch irradiance gives longer shoots, more frequent branching, lower maximum and mean needle age | Niinemets & Lukjanova 2003, New Phytol. 158: 75 | not in the abstract | [source, abstract only]; the bifurcation ratio is a keyword, its value is not in the abstract |
| Share of new growth | "Only about 18% of the total biomass of the tree was current year's growth, about 40% of the needles were 4 years and older reaching a maximal age of 12 years" | Schulze, Fuchs & Fuchs 1977, Oecologia | a mountain spruce forest, northern Germany | [source, abstract only] |
| One needle, projected | 13.7 mm² sun, 8.2 shade (control) | Gebauer et al. 2019, iForest 12: 27, Table 2 | ~20-year trees on a 1 × 1 m grid, 10,000 a hectare, 8.5 m, Ås, Norway; current-year needles | population [source, full text]; the numbers [record, fn-264 `SOURCES.md`] |
| Leaf area of a tree | ln LA = 1.024 + 0.631 ln CSA + 0.944 ln dbh − 0.840 ln h_dom (LA m² projected, crown surface area m², dbh cm, dominant height m) | Laubhann, Eckmüllner & Sterba 2010, For. Ecol. Manage. 260: 1498, eq. 14, Table 5 | 156 trees, eight stands of about 40, 80 and 125 years, thinned and unthinned, Austria | [source, full text] |
| Leaf area of a tree | LA = exp(−0.3385 + 0.5577 ln CSA + 1.0267 ln dbh − 1.3102 Stock) × 1.0341 (marginal R² 0.844) | Dirnberger et al. 2017, Ann. For. Sci. 74, eq. 2, Table 3 | 74 spruces, 12 plots of 40 to 175 years, 900 to 1,300 m, Styria; stocking 0.63 to 1.01 | [source, full text] |
| Measured leaf area of trees | mean 196 m², 56 to 435 m², one mature stand (DBH 34 to 61 cm, height about 36 m) | Laubhann et al. 2010, Table 1 | as above | [source, full text], the table came through broken: spot-check |
| Stand density effect | "Leaf area per crown surface area (crown density) increased with decreasing stocking degree" | Dirnberger et al. 2017, Results | as above | [source, full text] |
| Leaf per sapwood | 0.26 rising to 0.48 m² cm⁻² with stand age, 40 to 140 years | Köstner, Falge & Tenhunen 2002, Tree Physiol. 22: 567 | Fichtelgebirge | [record: abstract only] |
| Stand leaf area index | 10.5 projected, 28-year plantation | Bolstad & Gower 1990 | Wisconsin | [record: abstract only] |

**Arithmetic on the spruce [arithmetic; inputs tagged].**

- A shoot of the preset: 2 nodes × 0.02 m = 4.0 cm bearing 13.4 cm² [code]. Bellan's trees: 188,453 needles ÷ 1,707 shoots = 110 needles a shoot; at 13.0 mm² a needle [record] that is 14.4 cm² a shoot. Bellan's own equation at 4.0 cm gives 9.6 cm², at 4.8 cm 13.0 cm² [record coefficients]. So needle per shoot is right to within the sources' spread.
- Bellan's trees make about 2.5 m² of new needle a year each. At a turnover of 0.10 that is about 25 m² standing a tree.
- The preset's tree: the last recorded standing needle is 441.6 m², 82 m² of it the year's (18.6%) [record, log 11 Nov 18:53]; 484 m² at 80 [record, log 13 Nov]. 82 m² ÷ 13.4 cm² = 61,000 shoots, which is the recorded 64,650 [record, fn-339 R3]. So the count is the product of standing needle, retention and needle per shoot, with nothing left over.
- A spruce of the preset's size by the two equations, taking DBH 26 cm [record], a cone of 3.5 m radius and 11 m length (lateral surface about 127 m², my figure from the recorded 7 m spread and 12 m height): Laubhann with the tree's own height as dominant height, 159 m²; Dirnberger at stocking 0.78, 112 m², and at stocking 0 (outside its data), 311 m². These are stand equations stretched to an open tree.
- At a new share of 0.10 to 0.18 that is 11 to 56 m² of new needle and 8,000 to 42,000 shoots a year.
- Whether the recorded 441.6 m² is projected area is [unknown]: `NEEDLE_AREA` is built from a projected needle area [code], while `ranges.py` notes "the scoreboard's needle area is each needle's whole surface, not projected" [record].
- No current count exists: the 64,650 predates fn-354 (which the log says cut shoots 7%), fn-355, fn-363 and fn-364 [record].

**Not found [unknown]:** a measured number of branches per whorl or of inter-whorl branches for Norway spruce (Mäkinen et al. 2003, Forestry 76: 525, and Colin & Houllier 1991/1992 are the leads; neither text was reached); a bifurcation ratio for *Picea abies*; shoots per metre of branch on any tree; any shoot or needle count on an open-grown spruce; total needle area against diameter fitted on open-grown trees.

### 2c. Oak and birch rows with no checked source

No new primary text was opened for either in this survey. What the record already holds, and what my searches turned up:

| Species | Measure | Value | Source | Read |
|---|---|---|---|---|
| Oak | leaf per crown projection, open-grown analogue (*Q. douglasii*) | 2.5 to 7.7, mean 4.4, 14 harvested trees | Karlik 2002, USDA proceedings, Table 2 | [record: full text, extractor] |
| Oak | stand index | 3.9, thinned 45-year sessile oak | Bréda et al. 2003 | [record] |
| Oak | generator, cycle medians at 40 | leaf 182.7 m², 3.54 on its own columns | fn-339 R4 | [record], older than fn-354 to fn-364 |
| Birch | one leaf | 11.89 cm² mean, 15.0 upper crown | Konôpka & Pajtík 2026, Cent. Eur. For. J. 72: 159 | [record: full text, extractor] |
| Birch | leaf area of a tree | 2.37 m² at a base diameter of 50 mm (4.06 m tall), 8.54 m² at 100 mm (6.79 m) | Konôpka et al. 2023, Plants 12: 1607 | [source, abstract only]; 180 trees up to 15 years, natural regeneration, Western Carpathians |
| Birch | leaves per short shoot | "two or three leaves" | Kull et al. 2003 | [record: full text] |

Not found for either [unknown]: new shoots a year, shoot length by order, short-shoot lifespan, the share of long shoots by light, a leaf area equation for a mature tree, and for *Q. garryana* a leaf area of one leaf.

## 3. Which rows the measurements would move, and which way

Findings, not a design.

**Spruce.**
- `leafYears` 5.5 is a half-life. A cohort that is whole at first, half at 5.5 and gone at 12 holds more than 5.5 cohort-years [arithmetic]; the turnover of 0.10 a year and Reich's 6.4 to 7.2 cohorts (the preset's own quote) both point **up**. Moving it up lowers the new share of needle and so the shoots a year for the same standing needle.
- `leafArea` per metre is linear in shoot length (3.36 cm² a cm). Bellan's equation is not: 2.4 cm² a cm at 4 cm, 2.7 at 4.8, 9.7 at 35 cm [arithmetic on record coefficients]. Short shoots would move **down** by about a quarter, long leader units **up**. The 13.7 mm² needle comes from a 10,000-a-hectare thicket; Bellan's own trees give about 13.0 [record].
- Shoot `internode` × nodes (4.0 cm) sits under Bellan's 4.8 cm mean; **up** by a fifth on that one source.
- Standing needle: 441.6 m² against 110 to 310 by stand equations. The direction is **down**, by 1.4 to 4 times, if the record's figure is projected area. The rows that set it are the branchlet and spur shares and lifespans, none of which has a measurement to move to.
- Whorl: 4.25 branches against the ledger's own "3 to 4 lateral buds" on a reference image [record]: **down**, on a reference, not a measurement.

**Beech.**
- Total leaf: **down** by 2.3 to 3 times against Bartelink's stand trees, at 40 and 80. No open-grown figure exists to say how far an open tree may sit above that.
- `leafArea` 15.7 cm² is one tree's sunlit mean. Canopy-top leaves on mature trees near Basel are 7.9 to 10.3 cm² (**down** by a third to a half); the same Poľana tree's shaded leaves are 21.6 cm² (**up**). The preset gives every leaf the sun value. A crown mean is [unknown].
- Short shoot `lifespan` 12: no measurement; it went 3, then 4 "which brings the crown's leaf area index to 5.2", then 12 on an expert's sentence [code; record]. It multiplies the standing short shoots directly. Direction [unknown] until Roloff 1989 is read.
- Branch spare zone `SHOOT` 0.65 against Letort's 0 to 0.2: the source points **down**. Letort's trees are understorey, so the source's own range is for the wrong light.
- `shadeShort` 0.38 holds against the text. It rests on two light levels on branches under six years in the outer crown of 30-year stand trees.
- Leaves per short shoot (4) holds on two sources.

**Oak.** No row has a new measurement against it. The leaf was close on the last record (3.54 against 3.9 and 4.4), which is older than the current build.

**Birch.** `leafArea` 13.19 derived against a measured mean of 11.89 cm² [record]: **down** by a tenth. Nothing else has a measurement.

**What reads as a per-species knob, not a measured trait [code, record].**

| Preset | Row | Why |
|---|---|---|
| Oak, spruce, birch | `/space/coarse/*`, six rows each | fitted per species on tuning seeds (fn-325, fn-326, fn-328) |
| Oak, and through it all four | `shadeHazard` 1.0, `balanceHazard` 2.0 | host values from renders; `tolerance.rs` scales them into every species |
| Oak | `shadeSize` 1.5 / 0.5 / 1.0 by row | set in fn-236 rounds "so a lit limb does not outgrow the dome", "so the fine wood fills between the limbs' clumps" |
| Oak | fork `LIMB` 0.8; leader 6 years; short shares 0.4 to 0.7; twig 5 years at 0.98 | each moved in an fn-236 round against a look: "so no seed's crown stands on two limbs alone", "two decades stood it above the dome as a tuft", "to fill the crown" |
| Spruce | branch `dormant/10` 0.45 | "tuned to S1's dark lower curtains" |
| Spruce | sprig lifespan 6 and internode 0.05; branch dominance 0.35 and viability 0.998 | "so a young tree's tiers spread wide and the crown's top is a cone, not a spire"; "for variety between seeds" |
| Beech | short shoot lifespan | its history is a fit to a leaf area index |
| Beech | branch spare `SHOOT` 0.65 | "raised to 0.65 (estimated)" past the source's range |
| Beech | every reiterate share and lifespan | "set by the host-screened stills, never by a number alone" |
| Birch | nearly every row | fourteen rounds "estimated from the stills"; short `primary` 0.0005 fitted to hold DBH |
| Birch | `groundHazard` 1 | a stand-in on an owner verdict; the one text found says the opposite of a cultivar |
| Palm | `internode` 0.0339 | fitted to keep the height the old palm had |

## 4. Unknown

- The count of estimates per preset on 11 Nov, which P6 measures against.
- Total leaf area, leaf per crown projection, or any shoot count of an open-grown beech, spruce or birch. Every tree-level equation found was fitted on stand trees.
- Whether the spruce's recorded standing needle (441.6 m²) is projected area, and what it is on the current build.
- New shoots a year on the current build for any species; the spruce's last count is from 10 Nov.
- Branches per whorl, inter-whorl branches, and the bifurcation ratio of Norway spruce, as numbers.
- Whether Gebauer's Table 2 says 13.7 mm² (the population is confirmed; the number is from an earlier record).
- Bellan's needle-area coefficients, 0.9917 and 1.640, as printed (from an earlier record; the R² of 0.91 is confirmed).
- Lemoine's long-shoot threshold: 5 mm in the text I read, 11 mm and 2.5 mm in `LEAF-SURVEY.md`.
- Why the beech at 40 draws 326 m² of leaf on a trunk whose own sapwood row accounts for 128 m².
- A beech short shoot's measured lifespan (Roloff 1989, unread).
- A crown-mean beech leaf, between the sunlit 15.7 and shaded 21.6 cm² of one tree and the 7.9 to 10.3 cm² of another stand's canopy top.
- Any measured counterpart for a lateral probability, an axis lifespan, a viability, a shedding delay or a light-response coefficient, for any species. The earlier leaf survey said the same [record].
- A leaf area for one *Quercus garryana* leaf.
- The texts not reached: Mäkinen et al. 2003, Colin & Houllier, Niinemets & Lukjanova 2003 past its abstract, Muukkonen & Lehtonen 2004 past its abstract, Leuschner et al. 2006, Reich et al. 1996, McDowell et al. 2002, Phillips et al. 2003, Visakorpi et al. 2020, Letort et al., Niinemets & Valladares 2006. The last six are cited by presets and marked "cited, not opened" above.
- Whether Laubhann's Table 1 row (196 m², 56 to 435) is the mature thinned stand's: the table came through the extractor broken.
