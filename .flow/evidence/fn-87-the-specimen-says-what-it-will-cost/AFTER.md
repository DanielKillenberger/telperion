# After measurements

Same method as `BASELINE.md`: the whole `target/ci/deps/species-*` binary, `ci`
profile, peak RSS polled from `/proc/<pid>/status` field `VmHWM`, 32 cores, 32
GB. The binary now also reports its own figures on each heavy test, which is
where the ceiling, the charge limit and the peak charge below come from. Head
`33429197`, against the base `38e6c57f`.

Three runs were taken rather than one, because free memory on the desk drifted
between them, which is what makes them the evidence for R7's second clause: the
budget narrowed with it, on its own, without a number being changed anywhere.

| MemAvailable | Process ceiling | Charge limit | Peak charged | Peak VmHWM | Wall (s) |
| --- | --- | --- | --- | --- | --- |
| 19,933 MiB | 15,304 MiB | 10,203 MiB | 10,158 MiB | 12,539 MiB | 26.46 |
| 18,615 MiB | 13,960 MiB | 9,307 MiB | 9,258 MiB | 11,750 MiB | 39.80 |
| 17,803 MiB | 13,352 MiB | 8,901 MiB | 8,894 MiB | 11,222 MiB | 35.47 |

All twelve tests of the binary pass on each run.

## What the numbers say

**R6, the ceiling.** Every run's peak resident set stands under its own process
ceiling, with the charge limit saturated to within half a percent on all three:
the budget really is the thing deciding, not the core count. Resident stands at
1.23, 1.27 and 1.26 times what was charged, inside the 1.5 the headroom between
the charge limit and the ceiling allows. That ratio is the measured size of the
gap the prediction does not cover - the surface builder's scratch, the buffer
the digest serialises into, the metrics pass's adjacency - and it is the first
time it has been a measured number rather than an assumption.

**R7, the wall time.** 26.46 s, 39.80 s and 35.47 s against the 49.12 s the
baseline recorded, whose 15 percent ceiling is 56.49 s. The suite is faster on
every run, and the slowest of the three is the one that ran narrowest.

**Memory against the baseline.** The peak is roughly twice the 5,695 MB the
baseline recorded. The baseline's figure was a consequence of a constant - four
seeds a test, four tests, sixteen specimens whatever the machine had - and this
spec replaced the constant with a bound, not with a smaller constant. The suite
now spends what the machine can spare, which on this desk is more than sixteen
specimens' worth, and on a smaller machine is less; the three rows above are the
same binary deciding differently as the machine changed under it.

## The prediction against the specimen

`footprint::predict` and `footprint::measure` are asserted equal on every seed
of every species, so R3's 15 percent margin is never spent: the two counts come
from the same functions, and the only way they can differ is if one of them is
wrong. The margin remains the stated contract for a caller sizing a run; the
harness holds the implementation to exactness.

## A limitation, stated

The budget is process-wide, which is what the spec asks for and what the local
`cargo test` binary needs. CI runs the same tests under `cargo nextest`, which
gives each test a process of its own, so four heavy tests are four budgets, each
reading the same `MemAvailable`. On the four-core runner the seat count bounds
each process to four specimens, which is sixteen across the four - exactly what
`SEEDS_IN_FLIGHT = 4` gave before, so CI neither gains nor loses by this change.
On a many-core machine running `nextest` rather than `cargo test` the four
budgets would each spend their own two thirds. Whether the bound should be
shared across processes is a design question for the host, not one this task
answered.
