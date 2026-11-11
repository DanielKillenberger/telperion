# fn-346: the two rules of renewal on base Q, oak and presets

Drafted by the worker on 11 Nov 2026 at `374a2dc3c` plus the host's R3 answers, for the host. This is `R3-ANSWERS.md`, "The run", steps 1 to 4, with the host's later wording of rule B: a sleeping bud does not age while it sleeps, and it does not expire while its bearer lives. The probe is `probes/renew.patch`, and it was reverted; `git status --short crates` is empty. Nothing here proposes a design.

Marks: **measured** (printed by a run), **eye** (read off a still), **read** (from the code).

## Headline

- **The young oak holds under all four variants (measured, eye).** At 40 and 80 height and spread move under 6%.
  - **Q and QB** match today's leaf within 4% at 40 and 12% at 80.
  - **Rule A** (QA, QAB) adds 29 to 32% leaf at 40 and 19 to 24% at 80, with a denser skirt of fine shoots on the lower trunk.
- **At 120 every variant reads as an oak at least as good as today's (eye).** Each is a multi-limbed, clothed dome on 5 limbs, where today's stands on a two-limb V. Leaves are 23 to 42% above today's.
- **At 200 all four variants are wider and lower than today's pole (eye).** They spread 41 to 51 m on a short trunk with limbs from 4.7 m.
  - **Clothing:** the crown is clothed mostly at its limb ends, with long bare wood inside. QAB is the best clothed of the four.
- **At 300 all four are wreckage (eye).** Sprawling, snaking limbs lie out toward the ground with little leaf (34,000 to 91,000 leaves against today's 233,000).
- **No variant reaches a steady state on seed 1001 (measured).** Each tree peaks between 159 and 179 and then loses its framework every decade.
  - **The framework at its peak and at 299:** QAB's living branch tips go from 4,115 at 119 to 613 at 299, its bough tips from 84 to 6, and its limb-row tips from 9 to 0.
  - **Renewal does work,** about 1,000 buds woken a decade in QB. It does not hold the framework.
- **QAB on seed 1007 does not stop growing (measured).** Its census stopped it in year 186 at 51.3 million held phytomers and 19.05 GiB, eight times the most today's oak had held.
  - **Tips pile up in the dark.** Its living tips rise from 0.31 million at 99 to 1.48 million at 179 while its income falls from 128,000 (159) to 111,000 (179). The mean tip's own light falls to 0.075, so short shoots accumulate in the shade.
  - **No still exists at 300 on 1007.**
- **Other presets under QAB (eye, measured).**
  - **Palm and spruce:** the same as today's.
  - **The ordinary preset:** the same form, with twice the leaves.
  - **The beech:** at 40 a fuzz of fine shoots along the trunk; at 80 a denser crown with 52% more leaves and a trunk of 199 cm against 130.
  - **The birch:** worse, a thicket of hanging shoots down the whole white trunk to the ground.

## How it was built

**Base Q.** Q = P2 with `L_max` doubled: letters `PR`, `PIPES_LMAX` 34.8 times `PIPES_LMAX_FACTOR` 2. That is the charge by size × path / L_max, dead wood charging nothing, and continuations and relays tested as laterals are. Q's census equals the doubled instrument's row for row; both hold 3,901,146 phytomers at 159.

**Rule A** (letter `A`), all preset values, set in the example's `tuning.rs` `ablate`.
- **The twig moves on.** The twig row's (13) `continuation` goes from 0 to 1, so at the end of its life it moves on to the next lived row on the chain, which is the short shoot (14).
- **The short shoot lives on.** The short-shoot row's (14) `lifespan` is set to 1,000 years. The engine refuses infinity: "states[14].lifespan: a lifespan lies in 0 to 1000000 cycles". So 1,000 is used, longer than any run here.
- **Where it applies.** Each change applies only where the species lives in that row.

**Rule B** (letter `S`).
- **Preset part (`tuning.rs`).** Sleeping buds are put on the limb (6), bough (7) and branch (9) rows at the oak limb row's values: 0.1 per bud place, delay 6, rate 0.04.
  - **Which row a bud is laid as.** The limb row's own sleeping bud is a branch, which is the longest-lived lateral of the limb's medial zone. On each row a sleeping bud is therefore laid as the row of that row's medial zone's longest-lived lateral, taken from the preset's own lifespans before rule A: limb → branch, bough → branch, branch → twig (printed by the run: `RENEW S: row 7 sleeping … Some(9)`, `row 9 … Some(13)`).
  - **One row finer?** The host asked whether that is "one row finer than a lateral born at that node". It is the same row as the bearer's longest-lived medial lateral, not one finer.
  - **Existing buds kept.** A zone that already holds sleeping buds keeps its own.
- **Grower part (probe code).**
  - **No timetable.** `dormant::draw` holds the bud from its delay on, never ages it (`slept` 0), and drops its drawn waking time.
  - **A yearly draw.** Each year `grow/wake.rs` keeps the bud while something still carries its bearer on (`dormant::carried`). It wakes on a draw keyed on its lineage and the cycle, with probability equal to the bearer row's rate × max(0, 1 − cost / light).
  - **Light and cost.** Light is the lattice light at the bud's node, read with the field built at the end of the cycle before (`grow/cycle.rs` `read_light`). Cost is the node's path over `L_max`, or 0 where `L_max` is unset.
  - **The woken bud** is the row it was laid as (`dormant::aged` at 0 years slept).
  - **Light always read.** No bud went without a light read (`no_light_read` 0 in every census).
- **The census.** The census is RENEWAL.md's (`probes/renewal.patch`), with the `ASLEEP` line adding the buds woken by light and those let go when their bearer died.
- **Runs.** All runs were under `telperion-heavy`.
  - **Census:** `probes/renewal.sh` for the four variants to 300 on seed 1001 (logs in `raw/renewal/`).
  - **Stills:** `probes/pipes.sh` (logs and stills in `raw/stills/`).
  - **One-thread seconds:** QAB, pinned to one core.
  - **Not killed by the cap:** no run was killed by the memory cap. One batch was killed by the worker's own `timeout`, then redrawn (FRICTION.md).
- **Sheets and today's stills.** The sheets come from `probes/renew_sheets.sh`, and the curve tables from `probes/renew_curves.py <log> 20`. Today's stills are fn-345's (same crates), and today's oak at 80 was drawn in this spec's first run.

## 1. The census, oak seed 1001, every 20 years (measured)

- **Height and spread** are the rough layout's, before the settle.
- **Tips by row:** the living tips of the limb, bough and branch rows, the framework.
- **Limbs on stem:** the living limb-row laterals on the stem line.
- **Leaves:** the cycle's leaves the light reads.
- **B / D:** births over deaths in the decade to that year.
- **Sleeping held and woken:** the buds held for later cycles and the buds woken in the decade.

**Q**

| year | height m | spread m | held | living tips | limb / bough / branch tips | limbs on stem | lowest limb m | leaves | income | charge | B / D | sleeping held | woken |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 39 | 10.6 | 6.8 × 8.0 | 304,231 | 24,702 | 4 / 23 / 842 | 3 | 4.7 | 69,004 | 6,400 | 2,268 | 1.21 | 56 | 9 |
| 79 | 17.4 | 17.6 × 14.3 | 1,745,879 | 115,553 | 5 / 62 / 3,246 | 3 | 4.7 | 342,106 | 28,712 | 17,213 | 1.06 | 97 | 8 |
| 119 | 25.4 | 24.3 × 22.3 | 3,242,481 | 191,775 | 9 / 100 / 4,765 | 5 | 4.7 | 606,904 | 54,624 | 40,812 | 1.02 | 195 | 25 |
| 159 | 31.5 | 27.8 × 30.3 | 3,901,146 | 211,591 | 8 / 102 / 4,995 | 5 | 4.7 | 694,039 | 66,351 | 55,553 | 0.99 | 218 | 16 |
| 199 | 37.7 | 32.0 × 34.5 | 3,323,395 | 167,683 | 2 / 71 / 3,798 | 5 | 4.7 | 568,522 | 59,114 | 51,763 | 0.98 | 101 | 3 |
| 239 | 40.1 | 37.8 × 36.7 | 2,140,125 | 93,420 | 1 / 31 / 1,890 | 5 | 4.7 | 327,153 | 25,539 | 20,998 | 0.95 | 60 | 2 |
| 279 | 41.6 | 34.3 × 27.8 | 1,393,805 | 62,961 | 0 / 17 / 1,289 | 5 | 4.7 | 221,646 | 12,016 | 9,842 | 0.98 | 10 | 0 |
| 299 | 41.6 | 34.4 × 29.0 | 1,183,699 | 53,774 | 0 / 15 / 1,136 | 4 | 4.7 | 188,889 | 8,692 | 7,034 | 0.99 | 0 | 0 |

**QA**

| year | height m | spread m | held | living tips | limb / bough / branch tips | limbs on stem | lowest limb m | leaves | income | charge | B / D | sleeping held | woken |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 39 | 10.6 | 6.8 × 8.2 | 461,205 | 31,417 | 4 / 23 / 840 | 3 | 4.7 | 84,757 | 8,045 | 2,847 | 1.24 | 56 | 9 |
| 79 | 17.6 | 17.4 × 15.3 | 2,529,654 | 142,290 | 5 / 60 / 2,959 | 3 | 4.7 | 401,595 | 33,457 | 21,498 | 1.07 | 97 | 8 |
| 119 | 25.7 | 23.7 × 22.7 | 5,307,554 | 233,873 | 9 / 89 / 3,777 | 5 | 4.7 | 710,956 | 63,332 | 48,988 | 1.03 | 192 | 25 |
| 159 | 31.4 | 28.1 × 30.3 | 6,424,968 | 222,308 | 4 / 66 / 3,044 | 4 | 4.7 | 704,114 | 71,879 | 59,454 | 0.98 | 178 | 16 |
| 199 | 37.7 | 31.4 × 36.7 | 5,832,557 | 160,191 | 1 / 36 / 1,846 | 4 | 4.7 | 525,318 | 56,894 | 47,593 | 0.96 | 59 | 3 |
| 239 | 40.3 | 34.7 × 37.8 | 4,510,127 | 97,243 | 0 / 14 / 874 | 4 | 4.7 | 329,498 | 27,406 | 21,068 | 0.93 | 17 | 0 |
| 279 | 40.4 | 36.0 × 37.6 | 3,090,237 | 63,524 | 0 / 9 / 602 | 4 | 4.7 | 214,757 | 13,379 | 10,505 | 0.94 | 1 | 0 |
| 299 | 40.7 | 35.7 × 31.1 | 2,536,188 | 55,309 | 0 / 8 / 545 | 4 | 4.7 | 186,244 | 10,487 | 8,213 | 0.97 | 0 | 0 |

**QB**

| year | height m | spread m | held | living tips | limb / bough / branch tips | limbs on stem | lowest limb m | leaves | income | charge | B / D | sleeping held | woken |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 39 | 10.6 | 6.9 × 8.1 | 313,022 | 25,526 | 4 / 23 / 865 | 3 | 4.7 | 71,063 | 6,524 | 2,329 | 1.21 | 4,665 | 133 |
| 79 | 17.4 | 17.8 × 14.4 | 1,861,837 | 124,980 | 5 / 62 / 3,523 | 3 | 4.7 | 365,744 | 29,929 | 18,240 | 1.06 | 24,463 | 554 |
| 119 | 25.6 | 24.2 × 22.5 | 3,540,273 | 212,421 | 9 / 96 / 5,202 | 5 | 4.7 | 667,395 | 59,333 | 44,383 | 1.02 | 38,715 | 994 |
| 159 | 31.6 | 27.9 × 30.5 | 4,063,313 | 226,970 | 8 / 96 / 5,273 | 5 | 4.7 | 743,396 | 78,037 | 63,718 | 1.00 | 40,129 | 1,111 |
| 199 | 38.0 | 33.2 × 35.9 | 3,683,209 | 196,059 | 4 / 66 / 4,329 | 5 | 4.7 | 661,515 | 79,111 | 67,205 | 0.98 | 33,427 | 976 |
| 239 | 42.5 | 37.9 × 35.4 | 2,527,347 | 120,694 | 1 / 28 / 2,441 | 5 | 4.7 | 423,064 | 49,398 | 38,659 | 0.96 | 19,757 | 736 |
| 279 | 45.1 | 37.2 × 34.0 | 1,525,094 | 68,388 | 0 / 15 / 1,353 | 5 | 4.7 | 242,550 | 26,800 | 20,911 | 0.96 | 11,155 | 455 |
| 299 | 45.2 | 35.4 × 30.5 | 1,213,129 | 55,766 | 0 / 11 / 1,098 | 5 | 4.7 | 198,085 | 21,242 | 16,600 | 0.98 | 3,974 | 382 |

**QAB**

| year | height m | spread m | held | living tips | limb / bough / branch tips | limbs on stem | lowest limb m | leaves | income | charge | B / D | sleeping held | woken |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 39 | 10.6 | 6.8 × 8.2 | 468,393 | 32,260 | 4 / 23 / 866 | 3 | 4.7 | 86,645 | 8,156 | 2,904 | 1.24 | 4,670 | 117 |
| 79 | 17.6 | 17.4 × 15.1 | 2,607,417 | 149,319 | 5 / 60 / 3,146 | 3 | 4.7 | 418,310 | 34,476 | 22,263 | 1.07 | 22,247 | 385 |
| 119 | 25.7 | 23.8 × 23.0 | 5,502,534 | 251,218 | 9 / 84 / 4,115 | 5 | 4.7 | 759,638 | 67,044 | 51,455 | 1.03 | 30,264 | 596 |
| 159 | 31.7 | 28.0 × 30.4 | 6,899,082 | 260,246 | 4 / 67 / 3,544 | 5 | 4.7 | 819,911 | 84,527 | 69,420 | 0.99 | 27,970 | 622 |
| 179 | 34.8 | 29.3 × 34.4 | 7,064,530 | 249,726 | 3 / 52 / 3,092 | 5 | 4.7 | 801,464 | 87,425 | 73,780 | 0.98 | 24,344 | 578 |
| 199 | 37.7 | 31.2 × 36.1 | 6,966,194 | 216,768 | 2 / 37 / 2,454 | 5 | 4.7 | 710,544 | 77,722 | 62,842 | 0.96 | 19,234 | 547 |
| 239 | 40.1 | 34.6 × 36.9 | 6,082,849 | 151,405 | 1 / 17 / 1,364 | 5 | 4.7 | 516,348 | 47,033 | 35,080 | 0.94 | 10,645 | 385 |
| 279 | 42.1 | 35.9 × 35.5 | 4,715,019 | 100,005 | 0 / 7 / 770 | 5 | 4.7 | 348,220 | 20,950 | 14,636 | 0.91 | 6,149 | 297 |
| 299 | 42.1 | 35.9 × 32.1 | 3,800,336 | 82,359 | 0 / 6 / 613 | 5 | 4.7 | 285,554 | 13,586 | 9,436 | 0.96 | 2,212 | 261 |

Today's tree over the same years is in RESULT.md (held 2.51 million at 119, 1.60 million at 299, lowest limb 32.2 m at 299).

**QAB on seed 1007** (from its draw log, `raw/stills/oak-300-1007-QAB.log`; stopped by its census in year 186)

| year | height m | spread m | held | living tips | limbs on stem | lowest limb m | leaves | income | charge |
|---|---|---|---|---|---|---|---|---|---|
| 39 | 10.1 | 7.4 × 7.2 | 564,637 | 45,087 | 8 | 4.5 | 112,217 | 10,337 | 4,529 |
| 79 | 17.5 | 19.6 × 20.4 | 3,167,760 | 197,588 | 7 | 4.5 | 545,904 | 49,753 | 34,781 |
| 119 | 24.2 | 28.0 × 28.9 | 8,615,253 | 439,635 | 9 | 4.5 | 1,311,770 | 99,612 | 81,156 |
| 159 | 31.5 | 33.4 × 38.7 | 23,902,689 | 988,873 | 8 | 4.5 | 3,189,666 | 127,734 | 110,389 |
| 179 | 35.0 | 37.5 × 38.5 | 42,154,932 | 1,484,051 | 7 | 4.5 | 4,961,887 | 111,435 | 91,869 |
| 186 | | | 51,343,774 | | | | | | |

`STOP year=186 held=51343774 today=6283846 rss_gib=19.05`.

**What the curves show (measured, no proposal).**
- **No steady state on 1001.** Every variant's held phytomers peak (Q 3.90 million at 159, QA 6.42 million at 159, QB 4.06 million at 159, QAB 7.06 million at 179) and fall every decade after. Births over deaths stay at 0.91 to 0.99 from 159 on.
- **The framework is lost in every variant.** Limb-row tips reach 0 by 259 to 299. Bough tips fall from about 100 at 119 to 6 to 15, and branch tips from about 4,000 to 5,000 to 545 to 1,136.
- **Rule B wakes 40 to 300 times more buds than Q** (QB 976 to 1,111 a decade from 119 to 199, against Q's 3 to 25). Most held buds are let go when their bearer dies. QB's `ASLEEP` lines show 27,812 let go against 791 woken in the decade to 99, and 33,484 against 976 to 199.
- **Rule A raises the held phytomers 1.5 to 1.7 times.** It raises leaves by 17 to 23% from 39 to 119 and by 1% at 159.
- **On seed 1007, QAB runs away.** Its held phytomers grow 1.6 to 1.8 times a decade from 119 to 179 while income falls after 159. The mean own light of a tip falls from 0.24 at 99 to 0.075 at 179 (`BAL` lines).

## 2 and 3. Measures (measured)

- **Height and spread** are the mesh's. **Trunk** is the diameter at 1.3 m.
- **Lowest living limb and limbs** are the settled stem line's limb row.
- **Leaves** are the current year's. **Held** is the settled phytomers.
- **Peak GiB** is the drawn run's. **One-thread seconds** are the grow alone, measured for QAB only, as asked.

| tree | v | height m | spread m | trunk cm | lowest living limb m | limbs | leaves | held | peak GiB | 1-thread s |
|---|---|---|---|---|---|---|---|---|---|---|
| oak-40-1001 | A | 10.6 | 7.6 × 8.0 | 56 | 4.7 | 3 | 47,279 | 325,454 | 0.7 | 1.2 |
|  | Q | 10.6 | 7.6 × 8.0 | 56 | 4.7 | 3 | 47,432 | 328,331 | 0.7 | - |
|  | QA | 10.6 | 7.6 × 8.1 | 69 | 4.7 | 3 | 61,152 | 489,309 | 0.9 | - |
|  | QB | 10.6 | 7.7 × 8.0 | 57 | 4.7 | 3 | 48,860 | 337,825 | 0.7 | - |
|  | QAB | 10.6 | 7.6 × 8.2 | 70 | 4.7 | 3 | 62,628 | 498,322 | 0.8 | 1.7 |
| oak-80-1001 | A | 17.5 | 17.9 × 14.4 | 110 | 4.7 | 3 | 209,845 | 1,593,559 | 2.0 | 10.5 |
|  | Q | 17.5 | 18.8 × 14.5 | 109 | 4.7 | 3 | 217,924 | 1,719,507 | 2.2 | - |
|  | QA | 17.8 | 18.6 × 15.6 | 147 | 4.7 | 3 | 249,647 | 2,486,914 | 3.1 | - |
|  | QB | 17.5 | 18.9 × 14.6 | 113 | 4.7 | 3 | 235,603 | 1,842,406 | 2.3 | - |
|  | QAB | 17.8 | 18.9 × 15.4 | 149 | 4.7 | 3 | 260,488 | 2,572,677 | 3.2 | 14.8 |
| oak-120-1001 | A | 25.1 | 26.6 × 25.5 | 143 | 4.7 | 4 | 304,559 | 2,370,405 | 3.1 | 32.4 |
|  | Q | 24.3 | 27.0 × 25.8 | 131 | 4.7 | 5 | 375,802 | 3,120,543 | 3.9 | - |
|  | QA | 25.4 | 25.0 × 24.7 | 174 | 4.7 | 5 | 389,474 | 4,984,112 | 5.7 | - |
|  | QB | 24.6 | 26.9 × 25.3 | 136 | 4.7 | 5 | 419,275 | 3,479,360 | 4.2 | - |
|  | QAB | 25.4 | 25.6 × 24.6 | 180 | 4.7 | 5 | 433,109 | 5,394,826 | 6.0 | 45.5 |
| oak-200-1001 | A | 34.3 | 22.1 × 22.1 | 129 | 20.0 | 3 | 270,545 | 2,108,291 | 2.8 | 70.6 |
|  | Q | 29.3 | 50.0 × 50.7 | 128 | 4.7 | 5 | 344,012 | 2,845,517 | 3.8 | - |
|  | QA | 32.7 | 42.3 × 46.4 | 166 | 4.7 | 4 | 264,322 | 5,040,641 | 5.8 | - |
|  | QB | 29.6 | 48.7 × 47.4 | 134 | 4.7 | 5 | 412,365 | 3,503,927 | 4.5 | - |
|  | QAB | 33.5 | 41.2 × 44.4 | 172 | 4.7 | 4 | 346,017 | 6,431,892 | 7.1 | 117.6 |
| oak-300-1001 | A | 40.2 | 26.0 × 37.2 | 120 | 25.5 | 1 | 233,403 | 1,537,433 | 2.9 | 133.8 |
|  | Q | 23.1 | 40.3 × 24.8 | 64 | 4.7 | 4 | 33,737 | 1,030,396 | 2.5 | - |
|  | QA | 28.8 | 36.4 × 39.8 | 93 | 3.7 | 4 | 38,674 | 2,006,041 | 3.1 | - |
|  | QB | 22.2 | 59.3 × 30.0 | 86 | 4.7 | 5 | 90,693 | 1,069,081 | 2.6 | - |
|  | QAB | 35.1 | 49.4 × 45.5 | 101 | 4.7 | 5 | 51,052 | 3,480,672 | 3.5 | 177.5 |
| oak-120-1007 | A | 25.5 | 28.0 × 20.5 | 150 | 5.6 | 5 | 386,363 | 2,879,040 | 4.1 | 57.0 |
|  | QAB | 24.0 | 30.9 × 32.9 | 223 | 4.5 | 9 | 651,308 | 6,024,062 | 7.2 | 56.1 |
| oak-300-1007 | A | 31.9 | 34.9 × 24.0 | 106 | none | 0 | 94,815 | 681,125 | 4.0 | 193.6 |
|  | QAB | stopped by its census in year 186 (51.3 million held, 19.05 GiB) | | | | | | | 19.1 | not run |
| beech-40-1001 | A | 9.8 | 5.1 × 7.0 | 26 | n/a | n/a | 16,614 | 83,359 | 0.4 | 0.4 |
|  | QAB | 9.8 | 5.1 × 7.0 | 36 | n/a | n/a | 23,965 | 189,009 | 0.5 | 0.5 |
| beech-80-1001 | A | 17.4 | 19.6 × 20.2 | 130 | n/a | n/a | 463,750 | 1,887,629 | 2.6 | 7.6 |
|  | QAB | 17.4 | 19.6 × 20.2 | 199 | n/a | n/a | 703,231 | 5,316,138 | 6.1 | 12.9 |
| birch-40-1001 | A | 16.7 | 7.7 × 8.5 | 30 | n/a | n/a | 139,929 | 374,880 | 0.7 | 1.4 |
|  | QAB | 16.8 | 8.4 × 10.7 | 40 | n/a | n/a | 190,819 | 652,247 | 1.0 | 1.9 |
| spruce-40-1001 | A | 12.4 | 7.9 × 7.3 | 34 | n/a | n/a | 4,185,635 | 1,137,653 | 2.0 | 5.1 |
|  | QAB | 12.4 | 7.7 × 7.2 | 37 | n/a | n/a | 4,776,400 | 1,283,348 | 2.2 | 5.8 |
| palm-40-1001 | A | 21.8 | 19.3 × 18.7 | 59 | n/a | n/a | 6,554 | 480 | 0.3 | 0.3 |
|  | QAB | 21.8 | 19.3 × 18.7 | 59 | n/a | n/a | 6,554 | 480 | 0.3 | 0.3 |
| ordinary-40-1001 | A | 13.3 | 9.1 × 8.5 | 22 | n/a | n/a | 7,957 | 92,345 | 0.4 | 0.6 |
|  | QAB | 13.3 | 9.0 × 8.3 | 30 | n/a | n/a | 15,235 | 271,429 | 0.6 | 0.9 |

## 4. The read per tree and variant (eye, with the measures beside)

**Sheets.**
- **`renew-oak-1001-bare.jpg` and `renew-oak-1001-whole.jpg`:** rows 40, 80, 120, 200 and 300; columns today, Q, QA, QB and QAB.
- **`renew-others.jpg`:** today beside QAB, leafless then leafed, for the oak on 1007 at 120 and 300 and the other presets. The QAB oak at 300 on 1007 is a grey tile, since it was stopped.

**Oak 40, seed 1001.**
- **Q and QB: the same.** The same young tree, leader and low limbs, with leaf within 4%.
- **QA and QAB: the same tree, a little busier.** The lower trunk and the low limbs carry a denser skirt of fine shoots, with leaf up 29 to 32% and trunk 69 to 70 cm against 56. No knots show at this size. Height and spread are unmoved.

**Oak 80, seed 1001.**
- **Q and QB: the same.** Same form and crown, leaf +4% and +12%.
- **QA and QAB: the same to slightly worse.** The crown is a little denser (leaf +19 to +24%) and the trunk heavier (147 to 149 cm against 110).
  - **What is wrong:** a few long, thin shoots lie out low along the ground at the lower right of the tree. They show in both views and not in today's.

**Oak 120, seeds 1001 and 1007.**
- **On 1001 all four are better than today's.** Each is a fuller, multi-limbed dome on 5 limbs, clothed through its depth, with leaves in the crown's middle as well as at its rim. Today's stands on an open two-limb V. Height (24.3 to 25.4 m against 25.1) and spread hold.
- **QAB is the fullest; Q the most open.** QAB carries 433,109 leaves against today's 304,559. QA and QAB carry the heaviest trunk (174 to 180 cm against 143).
- **On 1007, QAB is better than today's lopsided tree.** It is a broad, dense, heavy-limbed dome (30.9 × 32.9 m, 24.0 m tall, 9 limbs, 223 cm trunk).
  - **What is wrong:** the trunk swells to 223 cm, against 150 today's.

**Oak 200, seed 1001.**
- **All four are better than today's pole in silhouette, and thin.** Each is a wide, low spread (41 to 51 m) on a short trunk, with limbs from 4.7 m. That is the open-grown oak's proportion, which today's tall pole with a tuft has lost.
- **What is wrong.**
  - **Clothing:** the crown is clothed mostly at the limb ends, with long bare snaking wood inside, most in Q and QB. QAB is the best clothed, and QA next.
  - **Flatness:** Q and QB are very flat (29 m tall and 50 m wide), their limbs reaching out nearly level.
- **No limb is lost whole at 200.** Nothing grows into the crown's inside either.

**Oak 300, seed 1001: all four worse than today's.**
- **What they are:** a sprawl of long snaking limbs, many lying out toward the ground, with little leaf (Q 33,737 and QA 38,674 leaves; QB 90,693; QAB 51,052, against today's 233,403). Limbs are lost whole from the crown's top and middle, and the gaps are not refilled.
- **The best of the four:** QAB, which holds the most framework, a 35.1 m tree 49 m wide. It is still a tangle rather than a crown.
- **What today's is:** a 40 m pole with a high tuft. Neither reads as an old oak.

**Oak 300, seed 1007: QAB not drawn.** It grew without bound and was stopped in year 186 (curve above).

**Beech 40: worse, slightly.** It is the same tree and height, with a fuzz of fine shoots along the whole trunk (leaf +44%, held +127%, trunk 36 cm against 26).

**Beech 80: the same to slightly worse.**
- **The same:** the dome and height.
- **What is wrong:** the crown is denser and darker (leaf +52%, held 2.8 times), and the bole is swollen at its foot (199 cm against 130).

**Birch 40: worse.**
- **What is wrong:** hanging shoots fill the whole trunk down to the ground, and the white stem is hidden behind them from 1 m up.
- **Measured:** wider (8.4 × 10.7 m against 7.7 × 8.5), with leaf +36%.

**Spruce 40: the same.** Leaf +14%, held +13%.

**Palm 40: the same,** to the bit in every measure.

**Ordinary 40: the same form, denser.** Leaf is 1.9 times today's, with a few more shoots along the trunk.

**The points the host named (eye).**
- **Clothed through its depth or only at its ends.** Through its depth at 120 in every variant. Only at its ends at 200 and 300 in every variant, least so in QAB at 200.
- **A thicket on the bole and lower limbs.** Under rule A, mild on the oak at 40 and 80. Strong on the birch, and a fuzz on the beech at 40.
- **Short shoots piling into knots.** No knots are visible on the 1001 oaks at any age. On 1007 the held phytomers run away (51 million at 186), the measured form of the pile-up. It could not be seen, since the tree was not drawn.
- **A lost limb's gap refilling.** No refill is visible at 200 or 300 in any variant.
- **The oak at 40 and 80.** It has not moved in height or spread. Its leaf is up a fifth to a third under rule A.

## Open for the host

- **Seed 1007 under QAB.** Held phytomers grow without bound while income falls and the mean tip's own light reaches 0.075. Rule A's short shoot lives until its survival draw fails or the balance sheds it. Whether that is what lets tips pile up in deep shade there is not measured here.
- **No steady state on 1001.** Every variant still loses its framework after about 170, with births over deaths at 0.91 to 0.99.
- **Buds lost with their bearers.** Rule B's buds are mostly let go with their bearer (33,484 let go against 976 woken in the decade to 199 in QB).
- **The oak at 300.** It is a sprawl near the ground in all four variants.

## Friction

One entry, in FRICTION.md: the worker's own `timeout` killed a draw batch.
