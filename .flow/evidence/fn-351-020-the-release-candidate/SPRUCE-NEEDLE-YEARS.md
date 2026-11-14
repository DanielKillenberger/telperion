# Spruce `leafYears` against its sources, 14 Nov 2026

By a dispatched Fable agent on branch `fn-351-spruce-needle-years` (from `fn-351-candidate` at 7685ee85b). **Stopped at step 1: no row was changed, nothing was built, measured or re-pinned, and no sheet was drawn.** The sources for the one number the grower needs are more than a quarter apart, and the preset's 5.5 lies inside their spread. The dispatch's rule for that case is to take none and report.

**Tags.** [code] read in this worktree. [source, full text] the paper's text read here from a scraped copy whose words run together and whose tables came through as digit strings; every quote below was found in that text, and the tables are marked where I rebuilt them. [source, methods and abstract] only those parts were reached. [arithmetic] computed here from tagged inputs; not a measurement and not a figure the source prints.

## 1. What the survey claimed, and what holds

`PHASE4-SURVEY.md` section 0 item 2 says `leafYears` 5.5 is a half-life used as a full life, and that the same abstract's turnover of 0.10 a year makes the row 1.8 times over (10 years against 5.5).

- The first half holds: 5.5 is the age at which half a cohort is shed, and the grower holds a cohort whole to that age (section 3).
- The second half does not hold. The 0.10 is a turnover of needle **mass**, weighted for older needles being heavier and for the mass a needle loses as it yellows. The same paper prints the unweighted figure beside it, 0.14, and its own survival table gives about 0.165 [arithmetic]. None of them is 0.10 in needle number or area.
- Reich's "6.4 to 7.2 needle age cohorts" is the count of cohorts that keep more than half their needles, the current year's included. It is the same kind of measure as Muukkonen's 5.5, not a longer one.

In the grower's terms the sources give 5.1 to 7.2 years. The row's 5.5 is inside that range. The error the survey set out to fix is at most a factor of 1.3, and may be nothing.

## 2. Sources, with passages

### Muukkonen and Lehtonen 2004, Can. J. For. Res. 34: 2517 to 2527 [source, full text]

The preset's and the ledger's source for 5.5 (`norway-spruce.values:148`, `spruce.rs:105`, fn-194 `SOURCES.md` row "Shoots grow 2 years and keep their needles 4 more", which records "abstract; full text not reached").

Population and method: 80 felled trees of the dominant layer from sample plots "in southern Finland (south of 62°4′N) during 1988–1990". "first-order needle cohorts (Fig. 1) were estimated visually from two branches in the 15th whorl from the top of the tree". "The percent survival of needles in each of the needle cohorts was estimated visually and classified into one of six classes: (1) 0%–5%, (2) 6%–25%, (3) 26%–50%, (4) 51%–75%, (5) 76%–95%, and (6) 96%–100%." "we converted those survival classes to percent survival using random numbers within the class boundaries. We made 1000 simulations."

Results: "with a probability of 71%, the youngest observed needle cohort (1 year old) belongs to class 5". "At the age of 5.5 years, 50% of the needles of that needle cohort have been shed. In addition, when the needles are 12 years old, all needles of the needle cohort are shed. Needles are shed most rapidly when they are 6–8 years old (Fig. 5)."

Turnover: "The dry mass of living needle increases during the first 4 years (Viro 1955). The mass of second-year needles, third-year needles, and needles older than that is 36%, 30%, and 40%, respectively, higher than that of first-year needles." "In this process spruce needles lose 13%–39% of their mass, depending on the age of the needle cohort (Viro 1955)." "The biomass turnover rates of the random simulations vary from 0.07 to 0.13 when the weighting and yellowing effects of needles are taken into account. Both the arithmetic mean and the median show that the turnover rate of needle biomass for Norway spruces in southern Finland is 0.10." Table 5, row Needles, modelled, southern Finland: "0.10 (0.0126)" and "0.14ᵃ", footnote a "Without weighting and yellowing effects"; northern Sweden "0.05ᵇ (0.0036)" and "0.08ᵃ,ᵇ"; estimated from measured litterfall and biomass "0.11ᶜ".

Equation 7 came through broken ("NL = −×× × + = − = − ∑ ∑ () () bbmd bm iiii i n ii i n 1 0 1 0 1"); its text says "b is the percent survival of the needle cohort, m is a mass factor ... d indicates loss of mass during yellowing ... The numerator indicates the total amount of needles removed annually, and the denominator indicates the total amount of needles on a tree or single branch. In this study n is 12". Its exact indices are [unknown].

Table 1, observations by survival class and cohort age (80 a column). The digit strings do not add up in the column for age 4 (they sum to 88) and three row totals miss; the cells below are the only set that meets all six printed row totals and twelve column totals of 80, and differ from the digits in four cells of that column [rebuilt; spot-check against the PDF]:

| Class | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 | 12 | Total |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 96–100% | 49 | 47 | 29 | 20 | 10 | 4 | 0 | 0 | 0 | 0 | 0 | 0 | 159 |
| 76–95% | 18 | 20 | 29 | 29 | 31 | 17 | 9 | 2 | 1 | 0 | 0 | 0 | 156 |
| 51–75% | 4 | 5 | 12 | 14 | 13 | 18 | 16 | 12 | 4 | 0 | 0 | 0 | 98 |
| 26–50% | 4 | 1 | 0 | 7 | 8 | 15 | 22 | 16 | 9 | 2 | 1 | 1 | 86 |
| 6–25% | 2 | 1 | 4 | 4 | 8 | 10 | 10 | 15 | 12 | 14 | 3 | 1 | 84 |
| 0–5% | 3 | 6 | 6 | 6 | 10 | 16 | 23 | 35 | 54 | 64 | 76 | 78 | 377 |

Survival by cohort age at class mid-points (98, 85.5, 63, 38, 15.5, 2.5%) [arithmetic]: 0.848, 0.837, 0.769, 0.708, 0.613, 0.468, 0.353, 0.232, 0.125, 0.057, 0.034, 0.031. Sum over ages 1 to 12: **5.08 cohort-years** (5.02 on the raw digits; 5.12 from Table 4's modelled cumulative probabilities). Half is crossed at 5.8 years, against the paper's 5.5 from its simulated model.

Not in the text [unknown]: whether the "1 year old" cohort is the current year's flush or the year before's. If a whole current flush stands on top of the twelve, the sum is 6.08.

### Reich, Oleksyn, Modrzynski and Tjoelker 1996, Tree Physiol. 16: 643 to 647 [source, methods and abstract]

Quoted by the preset. Abstract: "all of the Norway spruce populations had between 6.4 and 7.2 needle age cohorts". Methods: needle longevity was "assessed by counting the number of annual cohorts that retained a majority (> 50%) of their needles"; for the pines, "retained needles for between 1.5 to 2 years (and thus carried 2.5 to 3 needle age classes, including the current-year foliage)". Table caption: "Mean number of needle age cohorts (annual age classes) in a common garden plantation (at 180 m elevation) in Morawina, Poland for 18 Picea abies (Norway spruce) populations". Tree age and spacing were not reached [unknown].

### Reich et al. 2014, PNAS 111: 13703 [source, full text, methods only]

Not a Norway spruce measurement; it states what the count means. "needle life span was calculated by counting all living annual cohorts ... up to and including the youngest cohort with >50% but <90% of needles remaining". "Because cohorts younger than this one tended to retain nearly all of their foliage, and cohorts older than this tended to retain few, this technique provides a reasonable estimate of both mean cohort and mean needle longevity."

### Niinemets and Lukjanova 2003, New Phytol. 158: 75 [source, full text]

*Picea abies* at Järvselja, Estonia and the Fichtelgebirge. Its survival curve for spruce is a figure only (Fig. 3b, Fig. 4); no number is in the text. "An age of 1.0 yr. corresponds to current-year shoots." "Often, average leaf longevity is defined as the number of leaf cohorts retaining a certain percentage of needles (Reich et al., 1996; Balster & Marshall, 2000). As our study demonstrates, this concept is highly arbitrary." Muukkonen calls its own curve "similar to that reported by Niinemets and Lukjanova (2003) for an Estonia stand (Fig. 5)".

### Not reached

Schulze, Fuchs and Fuchs 1977 past its abstract ("Only about 18% of the total biomass of the tree was current year's growth", a mass share); Kayama, Sasa and Koike 2002 (abstract page only); Flower-Ellis and Mao-Sheng 1987; Muukkonen's Fig. 5 as an image.

## 3. How the grower uses the row [code]

- `crates/telperion-space/src/girth.rs:50-53`: a phytomer's leaf is `leaf_area × scale × size × clamp(leaf_years − age, 0, 1)`, age in whole years, 0 the year it was laid.
- `crates/telperion-space/src/girth.rs:58-60`: an axis's bearing phytomers are those with age under `leaf_years`.
- `crates/telperion-space/src/grow/deposit.rs:16`: the light's deposit reads the same `leaf`; `species.rs:167-174` says the wood reads it too.
- `crates/telperion-core/src/tree/leafing.rs:125-141`: the drawing keeps every station below the floor age, the fractional share at the floor age, none after.
- `crates/telperion-space/src/girth/tests.rs:198-213` pins it: at 5.5 the kept shares by year are 1, 1, 1, 1, 1, 0.5, 0, 0.

So a cohort is held whole, then dropped; it does not decline. On living wood a cohort contributes exactly `leaf_years` cohort-years of leaf, and standing leaf is `leaf_years` times one year's flush. The figure in a source that matches is the mean needle lifespan, the area under the survival curve with the current year counted, which is also one whole cohort over the unweighted turnover. Wood that dies takes its needle with it on top of this; the last record has 82 of 441.6 m² as the year's flush, 5.4 cohort-years standing against the row's 5.5 [record, survey 2b], so wood death trims little.

## 4. The value, source by source [arithmetic]

| Reading | Mean needle lifespan, years | From |
|---|---|---|
| Muukkonen, sum of survival over cohort ages 1 to 12 | 5.1 (5.0 to 5.1) | Table 1 at mid-points; Table 4 gives 5.1 |
| The same with a whole current flush added | 6.1 | if "1 year old" is last year's cohort [unknown] |
| Muukkonen, unweighted turnover | 1 / 0.14 = 7.1 | Table 5, footnote a |
| Reich 1996, cohorts keeping over half | 6.4 to 7.2 | abstract and methods; a count Reich 2014 calls "a reasonable estimate of ... mean needle longevity" |
| Muukkonen, the same count | 5.5 to 5.8 | "At the age of 5.5 years, 50% ..."; 5.8 from Table 1 |
| Muukkonen, mass turnover | 1 / 0.10 = 10 | **not comparable**: mass, weighted by needle age and yellowing |

Leaving the mass figure out, the readings run from 5.1 to 7.2, a spread of 41%. Like against like, Reich's count is 1.16 to 1.31 times Muukkonen's (6.4 to 7.2 over 5.5). Muukkonen's own two figures disagree with each other: its table gives a turnover of 0.165 to 0.167 on either index reading, its Table 5 prints 0.14, and I could not rebuild the 0.14 from the table.

The populations differ too. Muukkonen: branches of the 15th whorl of dominant trees in closed stands in southern Finland, so low in the crown. Reich: a lowland plantation in Poland. Niinemets finds needle age falls as branch light rises [survey 2b]. Neither is an open-grown tree.

**No value is taken.** By the dispatch's rule the sources disagree by more than a quarter. 5.5 is 8% over the lowest reading and 24% under the highest.

## 5. What a change would do, first order only [arithmetic]

Not an expectation for a decided value, since none was decided. For a row of L against today's 5.5, with the year's flush held fixed, standing needle scales by L / 5.5: 6.1 gives 1.11, 7.1 gives 1.29. With standing needle held fixed instead, new shoots a year scale by 5.5 / L: 0.90 and 0.77. Which of the two the grower does, or what mix, depends on light and sapwood feeding back and was not worked out here. The survey's 1.8 falls to 1.3 at most.

## 6. Not done

- Step 2's expectation for a chosen value, steps 3 to 6: no value to apply.
- No "before" measurement of the current build was taken, so the survey's unknown stands: standing needle and shoots a year on the current build.

## 7. Unknown

- Whether Muukkonen's "1 year old" cohort is the current flush (5.1 or 6.1).
- Equation 7's indices, and why the printed 0.14 is not the table's 0.165.
- Table 1's column for age 4 as printed.
- A numeric survival curve from Niinemets and Lukjanova, or from any second site.
- Needle survival on an open-grown spruce, or by crown position, as a number.
- Reich 1996's tree age and spacing.

## 8. For the host

Three ways on, none taken here:
1. Leave 5.5 and rewrite its comment: it is Muukkonen's half-shed age, which Reich 2014 calls a reasonable estimate of mean needle longevity, and the survival table's own sum is 5.1. The row then stands on its source as read.
2. Read the PDF of Muukkonen (eq. 7, Fig. 5, Table 1) by eye to settle 5.1 against 6.1 against 7.1, then decide.
3. Take Reich's 6.4 to 7.2 as the like measure for a lowland central European tree, which is a choice between two regions, not a correction.

The comments at `norway-spruce.values:148-155` and `spruce.rs:105-108` and the fn-194 ledger row still say "abstract"; whichever way is taken, they can now cite the text.
