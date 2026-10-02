# fn-183 friction

## 2026-10-02, task .1 (R1 ladder measurement)

- **A shell-safety hook refused the measurement driver.** I was writing the driver for 150 measured runs as a heredoc that reset its three output files with `: > "$R/..."`. The local `dcg` hook blocks truncating redirects to paths it cannot resolve. Cost: about 2 minutes and one rewrite with literal paths and append-only output. What would remove it: nothing in the repository. It is local setup and is reported here, not specced.
- **The lower-surface search was hidden inside "bisection" in the earlier count.** QUERY-COUNTS.md split the birch's queries by the call site that opened them, so 1.12M curtain-band queries showed up as stride and bisection queries. Cost: none to this task, since the exclusive purpose scope separates them. It is recorded because fn-174's scope was set from the earlier attribution.
- **Counting changes the time it counts.** The `query-count` build takes about 40% longer on the oak (63 to 66 ms against 47 ms), so timings come from a second binary built without the feature. Cost: one extra build of about 20 s per code change. What would remove it: nothing. This is the reason the counters compile out.
