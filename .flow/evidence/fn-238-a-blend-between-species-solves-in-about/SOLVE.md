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

## Host decision (2026-10-09)

1. **Build route 2, plus exact caching of the ends.**
   - The grown count becomes a running total, built the way `expected_length` already is, so the solve no longer pays for count tables it only sums.
   - A walk's two ends are fixed for the whole walk, so their predictions are computed once per walk and reused for every frame. This is exact, not a new first guess.
   - Route 1 (faster tables, same bits) is not built: route 2 makes it unnecessary for the solve.
2. **Re-pinning the walk digest is allowed** when the solved factors move only within the solve's own tolerance. This follows the generator-evolution policy in AGENTS.md. The evidence is:
   - every factor's change listed against its tolerance;
   - fn-230's walk strips at 40 and 80, before and after, viewed and showing no perceptible change;
   - shipped species byte-identical.
3. **The bar.**
   - At 40, every frame of fn-230's strips stays at 1 s or under.
   - At 80, frames without the ordinary tree as an end also stay at 1 s or under.
   - At 80, frames with the ordinary tree as an end are measured and reported. They are not held to 1 s, because a single kept prediction of the ordinary tree already costs 0.88 s.
4. **Route 4 becomes its own post-release spec.** It is a faster kept predictor for species with fractional rows. The host captures it.
5. **R3 is amended to match.** Then finish: the gate, npm test, impl-review and `flowctl done`.

## Route 2 built (2026-10-09)

**The grown count is a running total.** `telperion_space::expected_grown` adds up the grown phytomers as it goes (`closed_form/grown.rs`). It recurses the way `expected_length` does: the bud's first unit, then the rest of its axis as the same bud one cycle younger, with canonical carried keys, under the tree's windows. Each substructure is one number. Counts ignore a passed-through age's reach (counted whole), as `expected_counts` does. The size solve reads `expected_grown` in place of summing `expected_counts`' table. `expected_counts` stays for its other readers, the tests and the probes.
- **Test:** `the_running_total_is_the_count_tables_sum` (`tests/grown_total.rs`). The running total matches the table's sum within 1e-12 in the log, for the four branching presets and two raw spruce → ordinary walks, at ages 1, 2, 7 and 40. It was red on a stub before the build (`7b593eff`).
- **At 80** (`grown80`, a local probe): it is within 7.1e-15 in the log for every case. On the ordinary tree it takes 1.3 s against the table's 13.7 s. On the oak it takes 0.03 s against 1.8 s.

**The ends are cached exactly.** `blend/size.rs`, `Count::of_end`, keeps the predictions of the 8 most recent ends.
- The key is the count, the age and the end species' `Debug` text, which writes every number to round trip, so a different bit is a different key.
- A walk's frames share their ends, so only the first frame of a walk predicts them.
- The shape's own predictions are never cached.

## R3: factors

`walk_factors.rs.txt` dumps every frame's lateral factor, as bits, from the base (`8b64433c`, a separate worktree with its own `target/`) and from this build. It covers the walk digest's 165 frames and fn-230's strips, 18 frames each at 40 and 80. Raw output: `raw/factors-{before,after}-{digest,40,80}.txt`.

| Set | Frames | Bit-identical | Moved | Largest move in ln(factor) |
|---|---|---|---|---|
| walk digest | 165 | 161 | 4 | 3.9e-16 |
| strips at 40 | 18 | 14 | 4 | 5.0e-16 |
| strips at 80 | 18 | 13 | 5 | 2.0e-15 |

- **Which frames moved:** only grown-bound ones. At 40: beech → oak 0.25, beech → spruce 0.25 and 0.5, and spruce → ordinary 0.75. At 80: beech → spruce 0.5 and 0.75, and spruce → ordinary 0.25, 0.5 and 0.75.
- **Every move is within the solve's tolerance.** The tolerance is 1e-12 in the log of the prediction, and each move is a few ulps of the factor.
- **Refusals and unsized frames are unchanged:** 84 refusals and 18 frames without a factor, the same lines before and after.

## R3: the strips

The nine strip frames whose factor moved were drawn before and after (`strips.sh.txt`: `space_walk`, seed 1, under the GPU lock). Every PNG is byte-identical to its base (`cmp`). I viewed spruce → ordinary at 80, 0.5, before and after, and the sheet of all nine after frames (`raw/strips/sheet-after.png`). Each is a whole tree, drawn the same. The walk digest is re-pinned in `catalogue_identity.rs`. The family digest is unchanged, so every shipped species is byte-identical.

## R3: times

`walk_times.rs.txt` runs each walk's frames in order (0.25, 0.5, 0.75) in one process, as a harness walks them, so the first frame of a walk pays for its ends. There were three runs of each build, each a fresh process pinned to its own core, before and after side by side, `RAYON_NUM_THREADS=1`. The table gives the best of 3 (`raw/times-{before,after}-{1,2,3}.txt`). Load average was 3.6 at the start and 9.0 at the end, on 32 cores, with other sessions' gates running.

| Walk | Age | 0.25 | 0.5 | 0.75 |
|---|---|---|---|---|
| beech → oak | 40 | 1.65 → **0.34** | 0.33 → **0.05** | 0.29 → **0.04** |
| beech → spruce | 40 | 0.80 → **0.51** | 0.82 → **0.61** | 0.61 → **0.50** |
| beech → ordinary | 40 | 1.33 → **0.77** | 1.39 → **0.63** | 1.51 → **0.69** |
| oak → spruce | 40 | 0.43 → **0.10** | 0.47 → **0.08** | 0.51 → **0.12** |
| oak → ordinary | 40 | 0.80 → **0.12** | 0.89 → **0.13** | 1.01 → **0.12** |
| spruce → ordinary | 40 | 1.91 → **0.96** | 3.61 → 1.38 | 4.03 → 1.30 |
| beech → oak | 80 | 7.75 → 1.11 | 7.29 → **0.74** | 5.85 → **0.14** |
| beech → spruce | 80 | 2.72 → 1.81 | 5.32 → 3.48 | 5.45 → 2.73 |
| oak → spruce | 80 | 4.32 → **0.51** | 7.64 → **0.78** | 9.16 → **0.62** |
| beech → ordinary (reported) | 80 | 18.6 → 6.5 | 19.8 → 7.5 | 19.4 → 7.3 |
| oak → ordinary (reported) | 80 | 13.1 → 0.40 | 14.5 → 0.52 | 14.7 → 0.91 |
| spruce → ordinary (reported) | 80 | 74.6 → 9.8 | 66.4 → 10.1 | 58.6 → 6.8 |

Bold marks a frame within the bar. **Six held frames are over 1 s:** spruce → ordinary at 40, 0.5 and 0.75; beech → oak at 80, 0.25; and beech → spruce at 80, all three.

**Where their time goes now** (one run each, `one_frame`, a local probe that timed each prediction):

| Frame | Kept predictions | Grown predictions |
|---|---|---|
| spruce → ordinary 40, 0.5 | 7, 0.68 s | 6, 0.52 s |
| spruce → ordinary 40, 0.75 | 7, 0.60 s | 7, 0.51 s |
| beech → oak 80, 0.25 (ends included) | 9, 0.91 s | 3, 0.11 s |
| beech → spruce 80, 0.25 (ends included) | 9, 1.44 s | 3, 0.24 s |
| beech → spruce 80, 0.5 | 9, 1.92 s | 6, 1.07 s |
| beech → spruce 80, 0.75 | 7, 1.29 s | 7, 1.09 s |

- **The frames that remain are the solve's iterations on the shape**, about 0.1 to 0.2 s each for both counts. Each count makes one prediction at the ceiling, one or two for the bracket, and four to five regula falsi steps.
- **A better first guess (route 3) would cut the steps**, but it moves kept-bound factors within the tolerance. The host decision kept it out ("exact, not a new first guess").
- **Even with no regula falsi steps**, beech → spruce at 80, 0.5 needs about 2 × 0.21 s kept and 2 × 0.18 s grown, which is 0.8 s. With three of each it is 1.2 s (inferred from the per-call times). The cost of each shape prediction is route 4's.

## Host decision (2026-10-09, second): option (c), with the frames reported

Route 2 and the cached ends do most of the work. The ordinary tree's frames at 80 dropped from 13-75 s to 0.4-10 s. Frames at 40 now take 0.04-1.38 s, and frames at 80 without the ordinary tree 0.14-3.48 s. Every walk factor stays within 4e-16 of before, and every re-rendered PNG is byte-identical.

The six frames that remain over 1 s spend their time on the solve's own steps. A perfect first guess would still leave beech → spruce at 80 near 1 s, so route 3 alone does not reach the bar. The remaining work goes to fn-240, alongside route 4: a cheaper kept predictor and fewer solve steps, under the same evidence rule.

R3 is amended to say that every frame is measured and reported before and after, with the walk factors and stills unchanged as shown. The 1 s bar moves to fn-240.

Finish with the gate, npm test, impl-review and `flowctl done`. Remove the base worktree registered at scratchpad/fn238-base with `git worktree remove`.
