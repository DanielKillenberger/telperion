# fn-328 result: the dial thins a coarse tree's laterals

Rounds, rules and every table: `ROUNDS.md`. Search: `probes/search.py`. Raw logs are in ignored `raw/`.

## What changed

- **R1, the row thins.** A coarse `bud` row under 1 now lowers a coarsened age's lateral probabilities by degree along the dial; at or above 1 it grows as before (the fn-327 digests at fidelity 0 held, and at fidelity 1 for every preset whose rows did not change). Not positive or not finite, it is refused by name. Test: `a_row_under_one_thins_the_laterals_by_degree`.
- **R2, frozen searches.** Every round's bounds, score, seeds and judgement were committed before it grew. Climbs tuned on seeds 1001 to 1012; candidates were read on 1001 to 1050 before any held-out read. Seeds 1 to 50 chose nothing.
- **R4, rows.** The oak and the spruce take new rows; the birch keeps fn-326's B3. The oak's row uses the thinning (bud 0.82), so the row stays.

## R3: the oak at its preset age

Cheapest found within each departure on twelve seeds, read again on fifty at five repeats, one pinned core:

| Within, on twelve seeds | Worst on fifty | Seconds | Against 50 ms | Against the old rows' 261 ms | Axes | Apex-cycles |
|---|---|---|---|---|---|---|
| 5% (ships) | 0.042 | 0.151 | 3.0 times | 58% | 39,438 | 55,678 |
| 7% | 0.090 | 0.118 | 2.4 times | 45% | 28,908 | 41,804 |
| 10% | 0.074 | 0.105 | 2.1 times | 40% | 24,146 | 37,381 |
| 15% | 0.098 | 0.100 | 2.0 times | 38% | 21,799 | 34,620 |
| 25% | 0.154 | 0.093 | 1.9 times | 36% | 18,864 | 29,056 |

The spec named the old rows' 245 ms (fn-327's read); on these fifty seeds and this protocol they read 261 ms, and 0.057 departure, on leaf area.

## The three presets

| Preset | Old rows on fifty | New rows on fifty | Held-out read of the new rows |
|---|---|---|---|
| Oak | 0.057, 261 ms | 0.042, 151 ms (42% less) | 0.081 on limbs, 156 ms; the old rows read 0.038 there |
| Spruce | 0.043, 855 ms | 0.049, 558 ms (35% less) | 0.049, 713 ms; the old rows read 0.035, 765 ms |
| Birch | unchanged | no row within 5% in two rounds; closest 0.056 | none |

## What it means for the goal

- The premise held for the oak: fewer laterals, each with more leaf, cut apex-cycles by 46% and held the medians. The spruce's gain came from a coarser factor instead; its rows barely thin.
- **No preset is near 50 ms.** On the oak, cost levels off near 90 ms however far the statistics are let go, so tuning these rows will not reach it. The spruce is at 558 ms. Something beyond the coarse rows has to cut the cost.
- The oak's held-out read is outside 5% while its fifty-seed read is inside, and its limbs moved 7 points between the two reads. Seeds 1 to 50 judge it at the end; it may prove less steady than the old rows.
- First age is the strongest lever: earlier than the preset's own loses the limbs (oak 6, birch 7), later keeps fine ages and costs more (oak 10 to 12, spruce 9).

## R5

Gate green on `d1c70252d`: every suite ran, the test run 1,381 s against its 1,560 s budget, the package check and the core without geometry passing. Codex review of the diff from `9ef28ea9a`, one reviewer on correctness: SHIP with no findings.

## Friction

- **The 8 GB claim held three climbs in the queue for 1 h 40 min.** Fixed here: `search.py` claims 2 GB by default, `SEARCH_CLAIM` for larger grows, and parallel work is checked in the queue straight after launch.
- **One slow climb held a round for five hours.** Fixed here in practice: from the birch's round 2 each climb ran as its own task and reported as it ended. Proposed for the next search: a frozen rule dropping a climb whose cheapest row within 5% already costs more than a finished climb's.
