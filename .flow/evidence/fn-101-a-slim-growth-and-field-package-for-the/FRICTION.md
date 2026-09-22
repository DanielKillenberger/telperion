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
