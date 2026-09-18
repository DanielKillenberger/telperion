# fn-74 Actions runs, observed on GitHub 2026-09-18, PR #35

| Run | What | Total | receipts | rust job | node job |
|---|---|---|---|---|---|
| 35366126692 attempt 1 | first run on 60e47fcc, no receipt, cold caches, every suite executes | 10 min 59 s (16:03:06Z to 16:14:05Z) | 11 s | 10 min 42 s: core 8 min 13 s, render 67 s, wasm 14 s, jev 39 s | 5 min 36 s, of which wasm-bindgen install 92 s |
| 35366126692 attempt 2 | rerun of the same commit | 18 s (16:14:30Z to 16:14:48Z) | 17 s, five receipts found | skipped | skipped |
| 35367453289 | c265b101, a Cargo.toml comment moves every key, caches warm from attempt 1 | 9 min 56 s (16:16:07Z to 16:26:03Z) | 12 s | 9 min 34 s: core 8 min 11 s, render 30 s, wasm 15 s, jev 13 s | 3 min 36 s, wasm-bindgen install 0 s |

The commit that adds this file touches only `.flow/`; whether it starts a run is the R1 observation and is recorded in the close commit on master.

## What the numbers say

- **R5, rerun of an unchanged commit:** 18 s against the two-minute bound. Met.
- **R5, every suite executes:** 10 min 59 s cold and 9 min 56 s warm against the six-minute bound. Missed; the bound stays as written. The rust job is test-bound: its four `cargo test` steps run 74 binaries and doc-test runs one after another for about 8 minutes, `tests/species.rs` alone 214 s on the four-core runner, and compile is no longer the cost.
- **R6, warm compile:** the core crate's first test starts 33 s into the cold rust job and the warm render suite step, compile included, is 30 s; the whole rust job's compile is under two minutes cold and less warm, against the three-minute bound. Met without grouping the test binaries, so R6's grouping clause was not exercised.
- **R4:** the runner's results under the `ci` profile are green for every suite, as the desk comparison in LOCAL.md predicted.

## What would move the missed bound

The remaining cost is test execution, serial across binaries inside one job. Two ways under six minutes, neither in this spec: run the test binaries in parallel (cargo-nextest, or one job per crate for the core suite's slow binaries), which is bounded below by the 214 s species suite plus compile at roughly five minutes; or a larger runner, which scales the species suite with its cores. The owner decides.
