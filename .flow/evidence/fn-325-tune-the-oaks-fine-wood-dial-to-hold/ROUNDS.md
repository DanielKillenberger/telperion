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
