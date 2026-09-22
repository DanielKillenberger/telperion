# fn-101 friction

## 2026-09-22 — the accepted R8 sheets are not where the dispatch said

The dispatch names the main checkout's `.flow/evidence/fn-100-.../raw/` for
`sheet-64-candidate-field-order3.png` and the wood-only 2 cm sheet. That
directory holds only the task-1 sheets; the order-3 and wood-only sheets sit
in the fn-100 worktree's `raw/` (`.worktrees/fn-100-the-field-reads-the-plan-not-placed/.flow/evidence/.../raw/`),
where task 2 ran. Cost: three minutes of listing every worktree. What would
have removed it: `raw/` is ignored, so a sheet's location is the worktree it
was drawn in; a results note that names the sheet should name the checkout.

## 2026-09-22 — R3's timing ran on a loaded machine

The first R3 measurement (`raw/r3-timings.log`) ran while other sessions'
`rustc` jobs held the 32-core desk at a load average of 15.8: the oak at
seed 1 took 832 ms warm where fn-100 measured 288 ms for the same grid the
day before. The number says nothing about the slim build. Cost: one run of
the script and the note here; the measurement is repeated at the end of the
task with the load recorded beside it, and the full binding is measured in
the same minute so the comparison holds whatever the machine is doing. What
would have removed it: a measurement script that refuses to run, or labels
its rows, above a load threshold, and one checkout building at a time.

## 2026-09-22 — R3's 250 ms ceiling was never reachable on this code

R3 asks for 250 ms from species and seed to a queried 64-cell grid, and
cites the prototype's 93 to 205 ms from the structure export. fn-100's
RESULTS.md, on the commit this branch starts from, already measured the
field-only build plus the same query at 288, 330, 345, 579, 233 and 223 ms
for the six trees; the ceiling was inferred from a path that never ran the
plan query. Cost: two measurement runs and the escalation, where a checked
number would have set the ceiling at parity with the full binding or asked
for the query speed-up as its own spec first. What would have removed it:
the readiness pass reading the dependency's measured figures before writing
a ceiling on the same operation (CLAUDE.md, "Gates and checked claims").

## 2026-09-22 — host note on the R3 ceiling

The host marked this spec ready with R3's 250 ms figure unmeasured, against CLAUDE.md's rule that a claim not checked is written as unknown. It cost the worker one measurement pass and a stop. What would have removed it: the readiness check running fn-100's `measure.mjs` once on the same code before marking ready.
