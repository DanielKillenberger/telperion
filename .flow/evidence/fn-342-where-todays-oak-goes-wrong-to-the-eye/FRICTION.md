# fn-342 friction

## 10 Nov 2026, worker: yearly limb records keyed by axis index
- Doing: R3's yearly record of each limb's balance, printed from the grower's working set inside one grow to 300.
- Slowed by: the one cycle compacts its axes (`Grower::compact`), so an axis index names different limbs in different years; the first 300 run (4 minutes, two seeds) gave a table of limbs that "died" every few years. Rerun keyed by lineage.
- Cost: about 6 minutes of wall time and one wasted 300 run.
- Would have removed it: a note at `Grower::axes` (or in `compact`) that indices are not stable across cycles and lineage is the identity.

## 10 Nov 2026, worker: shell guard on redirects to variable paths
- Doing: a loop of neutral-setting runs writing `> $R/t-40-$n.log`.
- Slowed by: the dcg hook refuses a redirect whose target is a shell variable; rewrote the runs as `probes/runs.sh`.
- Cost: about 2 minutes.
- Would have removed it: nothing in the repository; a local hook on the owner's machine (reported, not specced).
