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
