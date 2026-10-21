# fn-304 friction

## 2026-10-21 19:50, task R1 (measure)

- **Doing:** starting the build under `telperion-heavy` with its log redirected into the spec's `raw/`.
- **Hindered:** the dcg hook refused a redirect to a shell-variable path (`> $R/build.log`), the same refusal fn-274 recorded. Every heavy command has to spell the full evidence path out.
- **Cost:** about 2 minutes and one retry.
- **Would remove it:** the peak-measuring wrapper fn-274 proposed in `scripts/`, taking the log path as an argument, so no redirect is written by hand.

- **Doing:** finding fn-271's and fn-274's measurements to avoid redoing them.
- **Hindered:** both evidence directories went to the 2026-10-16 archive; their per-test tables are only in `git show 8dab10634:<path>`.
- **Cost:** about 3 minutes.
- **Would remove it:** nothing needed; the manifest's recovery line worked. Noted because a gate spec's per-test baseline is the evidence the next gate spec needs.

## 2026-10-21 20:06, task R1 (measure)

- **Doing:** waiting for `telperion-heavy 8` to admit the build (`--no-run`) that precedes the timed gate.
- **Hindered:** another session's workspace gate (`telperion-heavy-2232265`, `cargo test --workspace` at 2 threads) holds the desk; room reads -1 to 2 GB against the 8 asked. The build alone needs far less than 8 GB, but one cap is asked for the whole sequence.
- **Cost:** 21 minutes of queue so far, before a single binary compiled.
- **Would remove it:** this spec itself (a ten-minute gate frees the queue four times sooner); meanwhile a smaller cap for compile-only steps.

## 2026-10-21 21:58, task R1 (measure)

- **Doing:** the first nextest-with-groups run at 8 GB, with a per-test RSS runner.
- **Hindered:** the runner script lacked its executable bit; nextest failed at listing after a 10-minute queue wait. The rerun's log had to take a new name, because dcg refuses a truncating redirect onto an existing file.
- **Cost:** about 10 minutes of queue, my error.
- **Would remove it:** a checked-in measuring runner (fn-271 and fn-274 each wrote one by hand and lost it to `raw/`); R4 may supply it.
- **Also measured:** the timed gate waited 26 minutes in the queue and the build 53 minutes, behind other sessions' 40-minute gates.

## 2026-10-21 22:15, task R1 (measure)

- **Doing:** nextest with fn-271's groups at 8 GB, `-j 4`.
- **Hindered:** `ground_reach` (fn-301, one test) reached 7.65 GB anon by itself; beside a light render test it took the 8 GB scope over and systemd stopped the whole run (OOMPolicy stop) after 738 s, 50 tests short. One test that needs nearly the whole cap makes any wider runner fragile.
- **Cost:** 12 minutes of run plus a rerun with that binary alone (`threads-required`).
- **Would remove it:** a per-test memory budget beside the time budget in R4; ground_reach's builds dropped as it goes (to check: its peak may be accumulation, not one tree).
