# fn-238 friction

## 2026-10-09: no sampling profiler

- **Doing:** finding where a grown prediction's time goes (R1).
- **Slowed:** `samply record` refuses to run because `perf_event_paranoid` forbids it, and `perf` is not installed. I had to write counters into `closed_form.rs` and `predictor.rs` and remove them afterwards (`r1-instrumentation.diff.txt`).
- **Cost:** about 10 minutes and three rebuilds.
- **Would have removed it:** `kernel.perf_event_paranoid=1` on the desk. This is a local setup issue on the owner's machine: reported, not specced.
