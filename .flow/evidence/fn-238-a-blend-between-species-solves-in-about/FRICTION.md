# fn-238 friction

## 2026-10-09: no sampling profiler

- **Doing:** finding where a grown prediction's time goes (R1).
- **Slowed:** `samply record` refuses to run because `perf_event_paranoid` forbids it, and `perf` is not installed. I had to write counters into `closed_form.rs` and `predictor.rs` and remove them afterwards (`r1-instrumentation.diff.txt`).
- **Cost:** about 10 minutes and three rebuilds.
- **Would have removed it:** `kernel.perf_event_paranoid=1` on the desk. This is a local setup issue on the owner's machine: reported, not specced.

## 2026-10-09: the shell guard refuses a redirect to a path held in a variable

- **Doing:** writing the before and after factor dumps and the timing runs into `raw/`.
- **Slowed:** the shell guard (dcg) refuses `> $R/file` whenever `$R` is a variable, and `git checkout -- <path>` even after the diff is saved. Every multi-file run had to become a script in the scratchpad.
- **Cost:** about 5 minutes and three retries.
- **Would have removed it:** a guard that resolves a variable path before judging it. This is local tooling: reported, not specced.
