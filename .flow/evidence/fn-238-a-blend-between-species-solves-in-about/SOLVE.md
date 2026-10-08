# fn-238: where a blend's solve spends its time (R1), and the fix that needs a host decision

## How this was measured

- **Tools.** `solve_probe.rs.txt` times `blend::families` for every walk frame of fn-230's strips: six pairs, t = 0.25, 0.5 and 0.75, at 40 and 80. It also logs each prediction the solve makes, through a timing wrapper around `kept` and `grown` in `blend/size.rs` (`r1-instrumentation.diff.txt`). `grown_probe.rs.txt` times one prediction of each end and counts the closed form's work, through counters in `closed_form.rs` and `closed_form/predictor.rs` (same diff). The instrumentation is reverted. Nothing in this file is committed code.
- **Run.** One run each, `target/ci/examples/solve_probe 40 80`, pinned to one core (`taskset -c 5`, `RAYON_NUM_THREADS=1`). Load average was 3.2 at the start and 10.1 at the end, on 32 cores. The raw output is `raw/r1-probe.txt` (gitignored).
- **Profiler.** `samply` could not record (`perf_event_paranoid`), so the breakdown below comes from counters, not samples.

## Result: the predictions are the cost, and one prediction already passes 1 s

**The solve itself costs nothing.** All of a frame's time is spent in its predictions. A count's solve makes two predictions of the ends (for the target), one at the ceiling, then the bracket and the regula falsi steps. A row with **3** predictions did no solving at all: the shape was already at or below the target.

**Grown predictions dominate wherever the ordinary tree is an end.** At 80 they take 64 to 97 % of the frame. Six of the 18 frames at 80 do no grown solving (3 grown predictions), yet still take 12 to 14 s, because of what those three predictions cost.

**At 80** (seconds, one core):

| Frame | Total | Kept: calls, s | Grown: calls, s, slowest |
|---|---|---|---|
| beech → oak 0.25 | 7.5 | 9, 0.98 | 3, 6.51, 4.89 |
| beech → spruce 0.5 | 5.0 | 11, 2.18 | 8, 2.80, 0.56 |
| beech → ordinary 0.5 | 20.3 | 11, 7.34 | 3, 12.96, 6.46 |
| oak → spruce 0.75 | 8.8 | 3, 0.38 | 3, 8.47, 7.48 |
| oak → ordinary 0.75 | 15.3 | 3, 1.53 | 3, 13.80, 6.57 |
| spruce → ordinary 0.25 | 80.8 | 9, 5.58 | 9, 75.27, 10.55 |
| spruce → ordinary 0.5 | 85.7 | 9, 7.36 | 9, 78.30, 12.51 |
| spruce → ordinary 0.75 | 66.6 | 3, 3.01 | 9, 63.56, 10.21 |

**At 40**, every frame takes 0.25 to 3.6 s. Nine of the 18 take more than 0.8 s. The slowest are spruce → ordinary at 0.5 and 0.75 (3.3 and 3.6 s; 8 and 9 grown predictions, 2.5 and 2.9 s of them). All 36 rows are in `raw/r1-probe.txt`.

### One prediction of each end, at 80

| End | Kept | Grown | Wood (`expected_length`) | Closed form: subs, events, table cells added |
|---|---|---|---|---|
| beech | 0.035 s | 0.68 s | 0.020 s | 9,557 / 2.2M / 0.39G |
| oak | 0.032 s | 1.05 s | 0.015 s | 8,573 / 2.8M / 0.49G |
| spruce | 0.061 s | 0.20 s | 0.027 s | 2,871 / 0.38M / 0.08G |
| ordinary | 0.88 s | 7.4 s | 0.49 s | 17,913 / 14.3M / 3.72G |

- **What makes the ordinary tree, and every blend, expensive.** Its rows hold every end's features at once: fractional lifespans, continuations between 0 and 1, abortion and viability on most ages, and sleeping buds on ages 6, 8 and 9. The predictors' state space and tables grow with them. The kept predictor builds 21,042 substructures with 338,853 table cells, against 4,417 to 7,405 and 22,000 to 30,000 for the corners. A blend's shape inherits these rows. The shape of spruce → ordinary at 80 costs about 0.67 s a kept prediction and 8 to 10 s a grown one.
- **Where a grown prediction's time goes** (ordinary at 80, 7.4 s). `expected_counts` recurses over each substructure's whole axis, every unit of it, and builds a per-cycle table (PAs × cycles) for each substructure. The breakdown:
  - 2.6 s adding a child's table into its bearer's (`add_shifted`, 3.7 G cells);
  - 2.1 s building events, including each event's stand and wood moments, which the counts never read;
  - 0.9 s cloning a memoised table on each of 13 M memo hits;
  - the rest in hashing and allocation.
- **The two cheaper recursions.** The kept predictor and `expected_length` recurse on a bud's first unit plus "the rest of its axis" (fn-209 P1), with canonical carried keys. That is why wood costs 0.49 s on the ordinary tree where the grown count costs 7.4 s.

## What R3's bar needs

R3 asks for at most 1 s a frame. **No change to the solve alone reaches it at 80, on the frames with the ordinary tree as an end.**

- **Ends.** The target alone needs the ends' kept and grown counts. On the ordinary tree that is 0.88 s kept plus the grown count, before the shape is predicted even once.
- **Shape.** The shape needs at least one kept prediction (about 0.67 s for spruce → ordinary) and one grown prediction.
- **Even with a free grown count**, spruce → ordinary at 80 needs about 0.06 + 0.88 + 0.67 = 1.6 s of kept predictions with no solve steps at all. Today it takes 9 (inferred from the per-call times above).

## The candidate fixes and what each costs R3

1. **A cheaper grown count with the same bits.** Share memoised tables (`Rc`) instead of cloning, build the counts' events without stand and wood, and vectorise `add_shifted`. Every grown count stays bit-identical, so every factor and both identity digests stay unchanged.
   - Estimated (inferred, not built): 2 to 4 times faster on a grown prediction, with nothing gained on kept.
   - Spruce → ordinary at 80 would come down from 67 to 86 s to roughly 20 to 40 s. **R3 is not met at 80; at 40 it is probably met** (the slowest frame, 3.6 s, has 2.9 s of grown predictions).
2. **A grown count on the first-unit recursion, as total phytomers.** Build it like `expected_length`, with a scalar total and canonical keys instead of per-cycle tables.
   - Estimated (inferred from `expected_length`'s cost): 10 to 25 times faster on grown.
   - The total is the same expectation summed in a different order, so a grown-bound factor moves within the solve's tolerance (about 1e-13 relative). The tree moves in its last bits too.
   - **The walk identity digest moves.** Its blends at 40 include spruce and ordinary walks whose grown count binds. This conflicts with R3's "the blend identity digests unchanged".
   - **Frames with the ordinary tree at 80 still take an estimated 3 to 8 s**, on kept predictions.
3. **Fewer predictions** (a better first guess, cached end counts across frames).
   - A different path through the regula falsi lands on a different factor inside the tolerance, so kept-bound factors move in their last bits, and so does the walk digest.
   - Caching the ends across calls is shared state: a memo of predictions keyed by the end species and age, kept between calls.
   - **This alone does not reach 1 s** (see the floor above).
4. **A faster kept predictor on species with fractional rows.** This is engine work on fn-209's predictor, which is pinned to the closed form "to the bit". It is the only route under 1 s at 80 for the ordinary-tree frames. Its design is open: no cause narrower than "the rows make the state space larger" was measured.

## Where this stops

The brief stops the work where the fix needs a choice between ways of building, or where it moves a factor or digest. **Every route that gets near R3's bar does one or the other.**
- Route 2 moves grown-bound factors within tolerance and so moves the walk digest.
- Route 3 moves every solved factor within tolerance.
- Route 4 is an open engine design.
- Route 1 changes nothing, and leaves the ordinary-tree frames at 80 well over 1 s.

**For the host**, three questions:
- Which route or routes to build.
- Whether the walk digest may be re-pinned for factors that move within the solve's tolerance.
- Whether R3's 1 s at 80 holds for frames with the ordinary tree as an end, or is relaxed, or waits for route 4 as its own spec.
