# fn-325 search rounds

Each round is frozen here before it grows a tree. Tuning seeds 1001-1005 at age 40, the oak's preset age. Objective: the largest departure from 1 over height, spread, trunk, limbs, shed share and leaf area, each a median ratio to the reference on the same seeds. Cost: the median first-grow seconds on one thread, enough to rank.

## Round 1

A coarse grid, first on two tuning seeds to rank, then the best at each cost level on all five.

| Value | Levels |
| --- | --- |
| First age scaled | 6 (limb), 9 (branch), 12 (shoot), 13 (twig) |
| Factor `s` | 2, 3, 4, 6, 8 |
| Leaf area per node, times `s` | 1, 2, 4, 8, 16 |
| Pipe per phytomer, times `s^(1/exponent)` | 1, 1.5, 2, 3 |

That is 400 candidates.

## Round 1's result

On seeds 1001 and 1002, first age 9 at factor 3, leaf 8, pipe 2 costs 133 ms with height 1.03, spread 1.00, trunk 0.99 and leaf area 1.06, and limbs 0.79 and shed share 0.85 short. Scopes from the limbs (6) lose limbs, and later scopes (12, 13) cost over 0.6 s. Under 50 ms, the best has limbs 0.22 and leaf area 0.29.

## Round 2

Scope fixed at first age 9. Two new values, for the two metrics short: lateral bud probability on the scaled ages, times b, capped so a zone's probabilities sum to at most 1, for limbs; and shade_hazard and balance_hazard on the scaled ages, times h, for the shed share.

| Value | Levels |
| --- | --- |
| Factor s | 2, 3, 4, 6 |
| Leaf area, times s | 4, 8, 12 |
| Pipe, times s^(1/exponent) | 1.5, 2, 2.5 |
| b | 1, 1.3, 1.6, 2 |
| h | 1, 2, 4 |

432 candidates, on seeds 1001 and 1002.

## Round 2's result

On seeds 1001 and 1002: s 3, leaf 4, pipe 1.5, b 2, h 1 costs 248 ms, worst 0.110: height 1.01, spread 0.98, trunk 0.89, limbs 0.89, shed share 0.90, leaf area 0.95. Trunk moves with pipe alone (pipe 2 gives 1.13, limbs 0.99), so it can be set last. Limbs answer to b. Shed share stays near 0.90, and raising the hazards moved it little. No candidate is under 50 ms, and the fastest fits cost 90 to 100 ms with worst 0.27.

## Round 3

Scope first age 9. Two new values. The shedding delay on the scaled ages, times x, since dead laterals shed sooner lower the kept share. A second factor s2 on shoots and twigs and later (age 12 and up), on top of s, with its internode, nodes, leaf area and pipe scaled as s scales them, since most apex-cycles are at the deepest orders.

| Value | Levels |
| --- | --- |
| s | 2, 3, 4 |
| s2 | 1, 1.5, 2, 3 |
| Leaf area, times s | 4, 8 |
| Pipe, times s^(1/exponent) | 1.5, 2.5 |
| b | 1.3, 2 |
| h | 1, 2 |
| x | 1, 0.5, 0.25, 0.1 |

768 candidates, on seeds 1001 and 1002.

## Round 3's result

On seeds 1001 and 1002: s 2, leaf 8, pipe 2.5, b 1.3, h 2, s2 1, x 0.1 costs 287 ms, worst 0.038: height 1.03, spread 0.96, trunk 1.00, limbs 0.99, shed share 0.99, leaf area 0.99. The shedding delay closed the shed share. A second factor on twigs (s2 above 1) loses shed share and leaf area quickly. The fastest fits, 93 to 97 ms, have worst 0.32.

## Round 4

A random search around that candidate, for the cheapest with worst at or under 0.05, seeded at 325, 1,000 candidates on seeds 1001, 1002 and 1003. Uniform in log space: s 1.5 to 5, s2 1 to 2, leaf 3 to 24, pipe 1.2 to 5, b 1 to 3, h 1 to 6, x 0.05 to 1. Scope first age 9. The generator is probes/round4.py.

## Round 4's result

On seeds 1001 to 1003: s 3.09, leaf 6.0, pipe 2.05, b 1.49, h 1.21, s2 1.07, x 0.29 costs 111 ms, worst 0.099: height 1.01, spread 0.92, trunk 0.92, limbs 0.90, shed share 0.96, leaf area 0.91. No candidate is within 5%, and none is under 50 ms; the cheapest reach 89 ms. 84 of 1,000 candidates could not be read.

## Round 5

The hill-climb in probes/climb.py on seeds 1001 to 1005, starting from round 3's and round 4's readings: 4 generations of 200, sigma 0.15, seed 3250. Its score puts the statistics first and cost second, as the file states. The held-out seeds still choose nothing.

## Round 5's result

On seeds 1001 to 1005, the climb levelled off: worst 0.068 at 154 ms, 0.097 at 107 ms, 0.145 at 87 ms. Generation 3 did not improve on generation 2.

## Judgement 1, on the held-out seeds

Four candidates, chosen before any held-out grow and named here. Seeds 1, 7, 42, 1234 and 5678, ages 40 and 80, fn-323's timing protocol.

| Name | first s leaf pipe b h s2 x |
| --- | --- |
| A, round 3's | 9 2 8 2.5 1.3 2 1 0.25 |
| B, about 7% | 9 2.336 16.648 3.712 1.649 2.447 1.332 0.202 |
| C, about 10% | 9 2.84 18.583 4.135 1.602 2.343 1.265 0.222 |
| D, about 15% | 9 2.729 20.172 4.758 1 2.452 1.773 0.213 |

Alongside it, on seed 1 at age 40, one census of C and D, to read where their time goes.

## Judgement 1's result

Held-out seeds, medians, fn-323's protocol. Reference at age 40: 1,722 ms. At age 80: 17,669 ms.

| Name | Age | Height | Spread | Trunk | Limbs | Shed | Leaf area | Worst | ms |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| A | 40 | 1.019 | 1.010 | 1.038 | 1.017 | 0.983 | 1.028 | 0.038 | 274 |
| B | 40 | 1.017 | 1.053 | 1.030 | 0.960 | 0.905 | 1.094 | 0.095 | 135 |
| C | 40 | 1.015 | 0.934 | 1.045 | 0.868 | 0.911 | 0.873 | 0.132 | 109 |
| D | 40 | 1.020 | 0.958 | 1.080 | 0.845 | 0.864 | 0.839 | 0.161 | 80 |
| A | 80 | 1.016 | 0.948 | 0.863 | 0.855 | 1.012 | 0.811 | 0.189 | 2,079 |
| B | 80 | 0.948 | 0.911 | 0.674 | 0.699 | 1.083 | 0.461 | 0.539 | 812 |
| C | 80 | 0.956 | 0.698 | 0.589 | 0.563 | 1.093 | 0.273 | 0.727 | 520 |
| D | 80 | 0.967 | 0.578 | 0.600 | 0.532 | 1.107 | 0.244 | 0.756 | 347 |

A holds all six medians within 5% at the preset age, 6.3 times faster than the full cycle. B to D missed by more on these seeds than on the tuning seeds, so two to five tuning seeds overfit. Values tuned at age 40 do not carry to age 80.

The census shows cost following apex-cycles: on seed 1 at age 40 the reference takes 1.137 s for 337,970 of them, A 0.170 s for 76,634, D 0.068 s for 26,212, or 3.4, 2.2 and 2.6 us each, and light is 6 to 12%. D's apex-cycles sit at orders 1 to 3, under the ages this scope scales. Cycle time past the scaled ages is in the passes that visit every apex.

## Round 6, the climb from a wider scope

probes/climb.py, on seeds 1001 to 1008, 4 generations of 150, sigma 0.15, seed 3260, starting from round 3 and round 5. It may now move the first scaled age among 7, 8 and 9. Weight 30 on the statistics, not 10, so a candidate that holds them outranks one that is only cheap. The held-out seeds and judgement logs are not in its pool.

## Round 6's result

On seeds 1001 to 1008: A holds at worst 0.039, 256 ms. E (s 2.099, leaf 8.082, pipe 2.667, b 1.241, h 1.944, s2 1, x 0.253) has worst 0.045 at 230 ms. F (s 2.773, leaf 19.606, pipe 4.515, b 1.821, h 2.429, s2 1, x 0.203) has worst 0.069 at 128 ms. Scaling from age 7 or 8 is worse (best 0.116 and 0.079). The search has levelled off: three climbs moved the 5% line little.

## Judgement 2 and the reach

E and F on the held-out seeds at age 40, fn-323's protocol. Then the reach of A, F and D on the oak at seed 1: ages 80, 120, 160, 200, 300, one grow each, its seconds, shape, grown phytomers, or the refusal by name.

## Judgement 2 and the reach: results

In RESULT.md. E and F missed on the held-out seeds, by trunk and limbs.
