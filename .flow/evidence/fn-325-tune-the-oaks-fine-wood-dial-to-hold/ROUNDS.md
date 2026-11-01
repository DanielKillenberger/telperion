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
