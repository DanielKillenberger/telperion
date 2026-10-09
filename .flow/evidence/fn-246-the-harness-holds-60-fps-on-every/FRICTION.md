# fn-246 friction

## 2026-10-09: the Two Trees' baseline takes ten minutes

- **Doing:** R1's per-pass baseline, best of 3 processes per species.
- **Slowed by:** the frame probe measures every view with 140 untimed and 136 timed frames, and Telperion's whole view is 120 ms a frame, so one Telperion process is about 90 s and the two trees' three runs each took 9.5 minutes of the 10-minute foreground limit.
- **Cost:** about 10 minutes of wall time; a near miss on the command timeout.
- **Would have removed it:** a view filter on the probe (added in `examples/frames.rs` after this run, so the relief variants measured only the bare views).

## 2026-10-09: the command guard refuses a redirect into a variable path

- **Doing:** the first smoke run of the frame probe.
- **Slowed by:** the shell guard (dcg) blocks `> $R/file` because it cannot prove where `$R` points.
- **Cost:** one retry, under a minute.
- **Would have removed it:** nothing in the repository; written with literal paths since. Local setup, reported only.
