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
