# fn-353 friction

## 11 Nov 2026, task 1: the space and core tests killed under a 16 GB cap

Doing: running `cargo nextest run --cargo-profile ci -p telperion-space -p telperion-core` through `telperion-heavy 16 --threads 12` after the species-ownership change.
Slowed: after 59 s the scope hit its memory cap and every running test was killed (32 SIGTERM/SIGKILL, 148 not run). None of them was a real failure. Nothing says what cap or test-thread count these two crates need under nextest's default parallelism.
Cost: about 2 minutes and one wasted run.
Would have removed it: a stated memory figure and `--test-threads` for running the narrow suites through `telperion-heavy` (in `docs/gpu-tests.md` or `.config/nextest.toml`).
