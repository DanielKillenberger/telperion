# fn-254 friction

## 2026-10-09 22:03, task 1: start
- Doing: the Phase-1 anchor.
- Hindered: `flowctl` is not on PATH in the worker's shell; found it under the plugin cache by hand.
- Cost: about 1 minute.
- Would remove it: `flowctl` on PATH for dispatched workers (local setup, not a repository change).

## 2026-10-09 22:20, task 1: the before timings
- Doing: the cold `grow` compile before any change (`hang_tests::browser_route`, driver cache off), three runs under the GPU lock.
- Hindered: load average 22 to 34 from other sessions; `grow` made in 28 to 33 s (fn-235 measured 13 to 17 s) and `girths` in 2.1 to 7.3 s, so one run fails the existing 20 s and 2.5 s bounds by load alone. `cargo nextest` is not installed here, so the gate runs as `cargo test --profile ci`.
- Cost: about 10 minutes; the before and after are measured again side by side at the end.
- Would remove it: a quiet window for compile timings, or a probe that reports every pipeline's make time without asserting (written here as `cold.sh` in the scratchpad).
