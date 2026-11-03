# fn-328 search rounds

Frozen before each grows a tree. The rule is `probes/search.py`'s docstring.

## Oak, round 1

One climb per first age, 6, 7, 8 and 9, each on its own core: a wide pass of 300 on four seeds, then 4 generations of 100 on twelve. The bud row may now thin the laterals. The bar is R4's, all six medians within 5%.

Judgement, fixed now: the climbs' cheapest rows within 5% go to tuning seeds 1001 to 1050. The cheapest that holds there, and costs less than the current rows' 245 ms, then gets one read on the held-out seeds, reported with whatever it says, and replaces the rows. Seeds 1 to 50 choose nothing.

Result, 2 Nov 2026, on seeds 1001 to 1012 at one repeat per seed. **No row of any climb came within 5%, so the rule sends nothing to the fifty seeds and the rows stay for now.** The cheapest within each departure, across all four climbs:

| Within | First | Worst | Seconds | Axes | Apex-cycles | Row: factor, leaf, pipe, bud, hazard, shedding |
|---|---|---|---|---|---|---|
| 5% | none | | | | | |
| 7% | 9 | 0.059 | 0.124 | 30,326 | 42,660 | 1.667, 30.84, 3.796, 0.651, 2.263, 0.313 |
| 10% | 9 | 0.072 | 0.114 | 25,541 | 37,790 | 1.559, 28.714, 4.045, 0.534, 2.21, 0.296 |
| 15% | 9 | 0.149 | 0.102 | 25,138 | 33,475 | 2.696, 57.845, 4.104, 0.932, 2.695, 0.329 |
| 25% | 9 | 0.238 | 0.085 | 15,184 | 25,137 | 1.781, 38.301, 3.365, 0.361, 2.09, 0.345 |

Against 50 ms and the current rows' 245 ms. These times are not yet the frozen protocol: the machine was in other use, and the same 7% row read 0.141, 0.133 and 0.124 s in three generations, so taking the cheapest reading flatters a row by about 10%. R3's table still needs five-repeat reads.

What it shows: the 7% row is the first with fewer laterals (bud 0.651), and grows about half the current row's time. Its worst medians are trunk and limbs, both 0.941. Cost falls by departure but slowly: even 24% off reads 85 ms. First age 6 never got within 12%, at 1.15 s or more, and first 9 led from its first generation, so a later first age may do better; none was tried.

## Oak, round 2

Frozen 3 Nov 2026, before it grows. One climb per first age, 9, 10, 11 and 12, each on its own core: no wide pass, 6 generations of 100 on twelve seeds, starting from the rows round 1 read at first ages 8 and 9, moved to the climb's first age. Bounds, score and seeds are round 1's.

Judgement, as round 1's: the climbs' cheapest rows within 5% go to tuning seeds 1001 to 1050. The cheapest that holds there, and costs less than the current rows' 245 ms, then gets one read on the held-out seeds, reported with whatever it says, and replaces the rows. Seeds 1 to 50 choose nothing.

Result, 3 Nov 2026, on seeds 1001 to 1012 at one repeat per seed. Five rows came within 5%, all at first age 9. First ages 10, 11 and 12 did not: their closest read 0.085, 0.138 and 0.133 at 0.41 s or more, because they keep more fine ages. So round 1's guess that a later first age may do better is wrong, and first age 9 is the best tried.

| Within | First | Worst | Seconds | Axes | Apex-cycles | Row: factor, leaf, pipe, bud, hazard, shedding |
|---|---|---|---|---|---|---|
| 5% | 9 | 0.049 | 0.213 | 40,075 | 51,322 | 1.37, 25.382, 4.145, 0.695, 2.756, 0.329 |
| 5% | 9 | 0.041 | 0.213 | 41,134 | not recorded here | 1.687, 26.289, 3.742, 0.82, 2.385, 0.243 |
| 7% | 9 | 0.059 | 0.156 | 30,326 | 42,660 | 1.667, 30.84, 3.796, 0.651, 2.263, 0.313 |
| 10% | 9 | 0.082 | 0.141 | 24,976 | 37,638 | 1.833, 33.34, 3.877, 0.589, 2.113, 0.329 |
| 15% | 9 | 0.128 | 0.130 | 22,650 | 35,083 | 1.806, 31.496, 3.86, 0.513, 2.128, 0.333 |
| 25% | 9 | 0.154 | 0.123 | 19,518 | 29,346 | 1.597, 33.856, 4.301, 0.431, 2.27, 0.407 |

Fifty-seed read, frozen now, before it grows: all five rows within 5%, in `raw/oak2-fifty.txt`, on seeds 1001 to 1050, with the current rows beside them as a control, at five repeats on one pinned core. A row holds when all six medians are within 5% there. The cheapest that holds and costs less than the control takes the held-out read.
