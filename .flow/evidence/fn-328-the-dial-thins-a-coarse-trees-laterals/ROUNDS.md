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

Fifty-seed read, 3 Nov 2026, seeds 1001 to 1050, five repeats, one pinned core, no other heavy work on the machine:

| Row: factor, leaf, pipe, bud, hazard, shedding | Worst | Seconds | Axes | Apex-cycles |
|---|---|---|---|---|
| Current rows: 2, 16, 2.5, 1.3, 2, 0.25 | 0.057 | 0.261 | 69,374 | 102,500 |
| **1.687, 26.289, 3.742, 0.82, 2.385, 0.243** | **0.042** | **0.151** | 39,438 | 55,678 |
| 1.37, 25.382, 4.145, 0.695, 2.756, 0.329 | 0.059 | 0.154 | 39,092 | 50,250 |
| 1.943, 22.175, 3.158, 0.841, 2.077, 0.202 | 0.062 | 0.163 | 41,337 | 62,638 |
| 1.723, 21.134, 3.326, 0.876, 2.131, 0.231 | 0.043 | 0.181 | 46,781 | 68,042 |
| 1.757, 19.531, 3.367, 1.019, 2.286, 0.177 | 0.066 | 0.216 | 59,140 | 81,498 |

The second row is the cheapest that holds, and costs 42% less than the current rows, which themselves read 0.057 here on leaf area. So thinning gives a row closer to the reference and cheaper.

Its one held-out read, seeds 1, 7, 42, 1234, 5678, five repeats: height 1.017, spread 0.991, trunk 1.058, limbs 0.919, shed 0.946, area 0.963, worst 0.081, 0.156 s, 33,476 axes, 48,093 apex-cycles. That is outside 5%, on limbs and trunk, and the rule replaces the rows with it whatever this says. The two rows disagree here: on these five seeds the current rows read 0.038 (fn-326), better than this row, while on fifty they read 0.057, worse. Fifty seeds are the stronger read, but this row's limbs moved 7 points between the two reads where the current rows' worst statistic moved about 3, so it may be less steady, and that is unsettled. Seeds 1 to 50 judge it at the end.

R3's table on the frozen protocol, the cheapest found within each departure on twelve seeds, read again on fifty:

| Within, on twelve seeds | Worst on fifty | Seconds | Against 50 ms | Against current 261 ms | Axes | Apex-cycles |
|---|---|---|---|---|---|---|
| 5% | 0.042 | 0.151 | 3.0 times | 58% | 39,438 | 55,678 |
| 7% | 0.090 | 0.118 | 2.4 times | 45% | 28,908 | 41,804 |
| 10% | 0.074 | 0.105 | 2.1 times | 40% | 24,146 | 37,381 |
| 15% | 0.098 | 0.100 | 2.0 times | 38% | 21,799 | 34,620 |
| 25% | 0.154 | 0.093 | 1.9 times | 36% | 18,864 | 29,056 |

The cost levels off near 90 ms however far the statistics are let go, so these rows alone will not reach 50 ms at any departure tried.

## Spruce and birch, round 1

Frozen 3 Nov 2026, before it grows. R4 of the spec sends them the same search. One climb per preset per first age, 8 and 9, each on its own core: a wide pass of 300 on four seeds, then 6 generations of 100 on twelve. Bounds and score are `probes/search.py`'s. The spruce's climbs claim 4 GB, since its grows are larger.

Judgement, as the oak's: the climbs' five cheapest rows within 5% go to tuning seeds 1001 to 1050 with the current rows as a control. The cheapest that holds there and costs less than the control gets one read on the held-out seeds, reported with whatever it says, and replaces the rows. Seeds 1 to 50 choose nothing.

Result, 3 Nov 2026, seeds 1001 to 1012 at one repeat. Spruce: 31 distinct rows within 5%, the cheapest at first age 8, 0.625 s against first age 9's 0.973 s; their laterals barely thin (bud 0.98 to 1.1). Birch: none within 5%. First age 8's cheapest within 7% reads 0.101 s and within 10% 0.098 s; first age 9's within 7% 0.226 s. Both birch climbs stopped improving after two generations.

Spruce fifty-seed read, frozen now: its five cheapest rows within 5%, in `raw/spruce-fifty.txt`, on seeds 1001 to 1050 at five repeats on one pinned core, with the current rows as a control. The rule above judges them.

## Birch, round 2

Frozen 3 Nov 2026, before it grows. As the oak's round 2: one climb per first age, 7, 8 and 9, each on its own core, no wide pass, 6 generations of 100 on twelve seeds, starting from every row the birch's round 1 read, moved to the climb's first age. Bounds, score, seeds and judgement are round 1's. Each climb reports as it ends.

Spruce fifty-seed read, 3 Nov 2026, seeds 1001 to 1050, five repeats:

| Row: factor, leaf, pipe, bud, hazard, shedding | Worst | Seconds | Axes | Apex-cycles |
|---|---|---|---|---|
| Current rows: 2.417, 13.725, 1.834, 1.022, 1.512, 0.067 | 0.043 | 0.855 | 139,278 | 323,782 |
| 2.819, 19.955, 2.237, 1.034, 2.632, 0.04 | 0.044 | 0.559 | 104,268 | 247,780 |
| **2.693, 18.645, 2.2, 0.982, 3.759, 0.051** | **0.049** | **0.558** | 103,022 | 242,590 |
| 2.721, 18.818, 2.242, 1.001, 2.932, 0.054 | 0.047 | 0.583 | 105,480 | 249,568 |
| 2.725, 18.807, 2.069, 1.032, 3.489, 0.051 | 0.035 | 0.568 | 108,897 | 254,348 |
| 2.911, 20.21, 2.112, 1.105, 2.019, 0.035 | 0.041 | 0.584 | 108,318 | 256,750 |

All five hold. The rule takes the cheapest, the bold row, 35% under the control; the next two are within 2% of its time and closer to the reference, so the choice among them is within the timing's noise. Its gain comes from a larger factor with more leaf and pipe, not from thinning: its bud row is 0.982.

Its one held-out read, seeds 1, 7, 42, 1234, 5678: height 1.000, spread 1.049, trunk 1.002, limbs 1.020, shed 0.957, area 0.995, worst 0.049, 0.713 s, against the current rows' 0.035 and 0.765 s there (fn-326). It replaces the rows.

Birch round 2, 3 Nov 2026: no row within 5%. The closest each first age reached on twelve seeds: 7, 0.230 at 0.099 s; 8, 0.068 at 0.101 s; 9, 0.066 at 0.096 s. Across both rounds the closest any birch row came is 0.056, and spread or trunk is the statistic that stays out. So the birch keeps fn-326's B3, and the thinning row ships in the oak's rows only.
