# fn-69 Actions runs, observed on GitHub 2026-09-18

| Run | Trigger | Head | Total | rust:test | npm test | typecheck | wasm-bindgen install | Result |
|---|---|---|---|---|---|---|---|---|
| 35349954061 | pull_request, PR #32 | eb876ef2 | 24 min 3 s (13:23:40Z to 13:47:43Z) | 19 min 2 s | 2 min 58 s | 2 s | 77 s | green |
| 35354444073 | push to master, merge dc835949 | dc835949 | 23 min 46 s (14:08:58Z to 14:32:44Z) | 18 min 44 s | 2 min 46 s | 1 s | 81 s | green |

A third run, 35349929437 on the superseded head e98a3308, was cancelled by the concurrency group 15 seconds after the next push, as designed.

R4, read honestly: the cargo and npm caches restore on the master run (the rust-cache step took 5 s, npm ci 6 s), and the run length does not move, because the release test run itself is about 19 minutes on the hosted runner and the cache holds dependency artifacts only; the workspace crates and every test binary link again under LTO with one codegen unit. The wasm-bindgen-cli install also repeats at about 80 s on both runs. A bound on the run length, and whether the test-bound step deserves a `test:quick` split (the fn-58 friction idea) or a different test profile, are the owner's call and were left out of this spec by its Edge Cases.

R6: the first run on master after the merge is green.
