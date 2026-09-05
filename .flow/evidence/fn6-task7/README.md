# FN-6 final CPU and GPU measurement

`results.json` contains the actual 2026-09-05 RTX 3080 measurements, including every CPU sample, raw GPU samples, renderer identity, browser flags, canvas dimensions, and source hashes. The final task 7 section of the FN-6 spec explains the budget misses.

Run the harness with `npm run dev -- --host 127.0.0.1 --port 5173`, then run:

```sh
node .flow/evidence/fn6-task7/measure.mjs
```

Prerequisites: the repo dependencies, Playwright available to Node, Chromium, an active X11/XWayland display, NVIDIA drivers with an RTX 3080, `nvidia-smi`, `lscpu`, and Hyprland's `hyprctl` for the physical display record. This is a Linux machine-specific measurement artifact, not a portable benchmark framework. The runner deliberately refuses to label another GPU or an absent timer as this measurement.

If Playwright is installed outside this repo, set `PLAYWRIGHT_MODULE` to its absolute `index.mjs` path. `CHROMIUM_EXECUTABLE`, `FN6_URL` and `FN6_OUTPUT` override the executable, harness URL and output directory. Defaults are `/usr/bin/chromium`, `http://127.0.0.1:5173` and `/tmp/fn6-task7-measurements`. The captured run used the isolated worktree's server on port 5182. The checked-in runner differs from the executed temporary runner only in these configurable dependency/executable/URL defaults.

The browser runs headed on X11 with a controlled 1600×1000 CSS viewport. Its reported `screen` is the controlled browser context; the separate physical-monitor record reports 3440×1440, scale 1. The actual browser ratio is 1; ratio 2 is a high-DPI diagnostic, not the monitor's native ratio. Every subject uses the existing stage framing and neutral sky, with foliage on. GPU queries time the whole room and subject. The runner requests vsync-off browser flags but does not disable the desktop compositor. These are GPU query times, never wall-clock frame times.

There is one excluded CPU warmup and five measured builds per subject. GPU points have eight warmup frames and twenty valid samples each. Sweeps use a fixed descending order; this single-camera run has no confidence interval or hardware-generalization claim. The unsupported timer case is deliberately simulated by hiding that extension on a separate context, and must return no timing number. Screenshots are generated alongside raw results for visual verification.
