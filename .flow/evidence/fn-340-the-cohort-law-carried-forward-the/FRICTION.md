# fn-340 friction

## 10 Nov 2026, R1: the probe's settle panics mid-growth on the oak, seed 1007

- **Doing:** reading the oak year by year to 120 (`probes/r1-trace.sh`), settling a copy of the grower every five years.
- **What hindered:** the coupled probe's settle panics at `geometry.rs:63` (a lateral whose bearer holds no phytomers) on seed 1007 at 95 in every probe mode, and at 55 with the split switch. The cycle never does. Inside the cores' worker threads the panic is not caught, so the second run of seed 1007 ended without its last two modes and the test harness reported nothing.
- **Cost:** about 6 minutes: a rerun for the lost modes, and the settle wrapped in `catch_unwind`, those years read as `nan`.
- **What would remove it:** the probe's own defect, not isolated here. `fn-339`'s reads settled only once, at the final age, and never met it. A probe that settles mid-growth needs the shed's kept bearers checked against the compaction's renames.
