# fn-340 friction

## 10 Nov 2026, R1: the probe's settle panics mid-growth on the oak, seed 1007

- **Doing:** reading the oak year by year to 120 (`probes/r1-trace.sh`), settling a copy of the grower every five years.
- **What hindered:** the coupled probe's settle panics at `geometry.rs:63` (a lateral whose bearer holds no phytomers) on seed 1007 at 95 in every probe mode, and at 55 with the split switch. The cycle never does. Inside the cores' worker threads the panic is not caught, so the second run of seed 1007 ended without its last two modes and the test harness reported nothing.
- **Cost:** about 6 minutes: a rerun for the lost modes, and the settle wrapped in `catch_unwind`, those years read as `nan`.
- **What would remove it:** the probe's own defect, not isolated here. `fn-339`'s reads settled only once, at the final age, and never met it. A probe that settles mid-growth needs the shed's kept bearers checked against the compaction's renames.

## 10 Nov 2026, R1b: the shell hook refused a scripted edit with a variable redirect

- **Doing:** adding the labelled diagnostic and writing each seed's yearly ratios to a file from a loop.
- **What hindered:** the hook refuses a redirect onto a path built from a loop variable, so the edit and the ratio files written in the same command were refused together.
- **Cost:** about 3 minutes, to move the edit to the Edit tool and the ratios into a script (`probes/diag-ratios.py`) that the run reads directly.
- **What would remove it:** nothing in the repository. The dispatch already names the limit; it was missed once.

## 10 Nov 2026, R1c: the open-loop replay does not fit 8 GB past 80

- **Doing:** fn-339's replay (`cohort_replay`, every axis open and none retired, every year's lattice kept) on the oak at 40, 80 and 120.
- **What hindered:** `telperion-heavy 8` killed it (exit 137, the 8 GB cap) at 120 on both seeds and at 80 on seed 1007. Only 40 on both seeds and 80 on seed 1001 were read.
- **Cost:** about 4 minutes of runs lost, and the read past 80 is missing.
- **What would remove it:** a replay that keeps each year's lattice only while a bearing unit reads it, or reads each unit's light as the tree grows instead of after. Or a larger cap, which the dispatch does not allow.

## 10 Nov 2026, R1c: the ledger's first year carried the last run's final year

- **Doing:** the structure's population ledger, the cycle and then the probe in one process.
- **What hindered:** the ledger was not cleared when a run began. The cycle's year-120 counts landed in the probe's year 1, and the first reading named a parting at year 1 and at 30 that did not exist.
- **Cost:** about 8 minutes and one rerun of both seeds.
- **What would remove it:** clear every per-run store at `trace(true)`, which the patch now does. The reader also leaves year 1 out.
