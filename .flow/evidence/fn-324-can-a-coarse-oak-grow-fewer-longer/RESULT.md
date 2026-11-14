# fn-324 result: no-go for this transformation

**Verdict, by the rule frozen in this child's spec at `78ba02e82`: no-go.** Only one candidate meets 50 ms at age 80, and it is not the same tree: all ages at factor 4 takes 43 ms with trunk diameter at 0.13 of the reference and leaf area under 0.01. So "worth tuning" does not apply either.

Measured 1 Nov 2026 at `78ba02e82` plus the probe, native, `--profile ci`, one thread. Oak, seeds 1, 7, 42, 1234 and 5678; medians. Raw logs are in `raw/`. The probe is in `probes/`, and it is not in the crates.

## The numbers

Each metric is the candidate's median over the reference's. The limit is 0.95 to 1.05.

**Age 40, the oak's preset age.** Reference: height 10.5 m, spread 7.7 m, trunk 0.68 m, 471 limbs, shed share 0.63, leaf area 205 m², 1.48 s.

| Candidate | Height | Spread | Trunk | Limbs | Shed | Leaf area | Seconds |
| --- | --- | --- | --- | --- | --- | --- | --- |
| all, 4 | 0.79 | 0.42 | 0.19 | 0.05 | 1.05 | under 0.01 | 0.011 |
| all, 3 | 0.83 | 0.44 | 0.20 | 0.08 | 1.13 | under 0.01 | 0.017 |
| all, 2 | 0.90 | 0.67 | 0.26 | 0.30 | 1.07 | 0.03 | 0.066 |
| all, 1.5 | 1.00 | 0.70 | 0.37 | 0.49 | 1.02 | 0.10 | 0.152 |
| fine, 4 | 0.97 | 0.91 | 0.40 | 0.77 | 0.84 | 0.11 | 0.111 |
| fine, 3 | 0.97 | 0.90 | 0.44 | 0.82 | 0.88 | 0.15 | 0.149 |
| fine, 2 | 0.98 | 0.92 | 0.58 | 0.97 | 0.95 | 0.30 | 0.382 |
| fine, 1.5 | 0.99 | 0.95 | 0.68 | 0.96 | 0.97 | 0.43 | 0.596 |

**Age 80.** Reference: height 17.4 m, spread 19.8 m, trunk 1.30 m, 780 limbs, shed share 0.82, leaf area 776 m², 17.6 s.

| Candidate | Height | Spread | Trunk | Limbs | Shed | Leaf area | Seconds |
| --- | --- | --- | --- | --- | --- | --- | --- |
| all, 4 | 0.61 | 0.38 | 0.13 | 0.05 | 0.97 | under 0.01 | 0.043 |
| all, 3 | 0.67 | 0.38 | 0.13 | 0.08 | 0.99 | under 0.01 | 0.077 |
| all, 2 | 0.90 | 0.55 | 0.21 | 0.35 | 1.00 | 0.03 | 0.692 |
| all, 1.5 | 0.91 | 0.60 | 0.29 | 0.58 | 0.98 | 0.09 | 2.657 |
| fine, 4 | 0.85 | 1.06 | 0.35 | 0.54 | 0.89 | 0.10 | 0.902 |
| fine, 3 | 0.87 | 1.05 | 0.38 | 0.64 | 0.93 | 0.13 | 1.401 |
| fine, 2 | 0.91 | 1.00 | 0.51 | 0.87 | 0.97 | 0.28 | 3.792 |
| fine, 1.5 | 0.93 | 0.96 | 0.61 | 0.94 | 0.98 | 0.44 | 6.357 |

The verdict does not rest on age 80. At the preset age, every candidate at or under 50 ms has trunk 0.20 or less and leaf area under 0.01.

## What it shows

- **The trade is steep.** Every candidate that keeps height, spread and limbs near the reference costs 0.1 to 0.6 s at age 40 and 0.9 to 6.4 s at age 80.
- **The compensation is too small.** The rule keeps a unit's leaf area, but fewer laterals per unit compound over orders. On seed 1 at age 40, fine at factor 4 keeps 17,193 phytomers against 348,504, 5% of them, so four times the area per node gives at most 20%. It reads 11%.
- **Trunk falls with leaf area** in both tables (inferred from these rows). The pipe rule also keeps a unit's area, not the tree's.
- **The limbs can be kept.** Fine scope holds height and spread much better than all ages at the same factor. It cuts the cycle about 13 times at age 40 and 20 times at age 80 at factor 4.

## What it cannot show

- One transformation, untuned, on one species. Raising leaf area and pipe further could close those two gaps, and whether light, shedding and cost then hold is **unknown**. No such run was made, since the rule does not allow tuning after a result.
- Cycle cost only. Field output, Wasm and browser cost are unmeasured. 0.1.4's times are from a browser, so these native seconds do not compare directly with them.
- Five seeds do not read R4's interquartile limit. No view was rendered.

## Departures

- A candidate's first grow over 0.5 s was not repeated, since no repeat could bring it to 50 ms. So fn-323's five repeats apply only to grows under that, and the slower rows are one grow per seed. That includes the age 80 reference.
- Option A named 3 hours, and the goal's 19:00 stop allowed less. The probe took about 8 minutes of run time, so the time limit did not decide anything.
- The review was skipped: the commit holds only `.flow/` files, and the probe's code is not in the crates.

## Friction

None slowed this child. Reusing `grow_census` and the family's species table made the probe one file.
