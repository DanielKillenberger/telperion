# fn-124 friction

## 2026-09-24 - native sampling profiler unavailable

Doing: R1's native attribution. `samply record` refuses to run because `kernel.perf_event_paranoid` is 2 on this machine, and `perf` is not installed. Changing it needs sudo, which an agent does not take. Cost: about 5 minutes and a `cargo install samply` (1 minute). Worked around by attributing with V8's sampling profiler on the browser module (R2's target anyway) and with scratch-only counters natively. Would have removed it: `perf_event_paranoid` at 1 on the owner's machine. This is a local setup issue, not a repository one.

## 2026-09-24 - the R4 stage comparison is slow on spruce

Doing: R4's paired native `generation_stages` comparison, ten alternated rounds per fixture. The spruce's CPU foliage makes each process several seconds, so the run took about 25 minutes, most of the task's waiting, and exceeded the 10-minute foreground limit, which moved it to the background. The `rings` and `surface` stages sit within 1 to 3% of each other run to run while their code is unchanged, so a strict 3% line on them needs many rounds to read. Would have removed it: a stage selector on `generation_stages` (skeleton and rings without foliage) or a documented noise floor for the R4 comparison.
