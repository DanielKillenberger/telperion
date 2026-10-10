# fn-261 friction

## 2026-10-11, task 1

- **The first R1 run deadlocked for 10 minutes holding the GPU lock.** The test took `generation::whole_tree()` itself, as `florets_tests` does, and `roots::kernels` takes the same non-reentrant mutex in test builds (`_one`), so the build waited on its own test forever at 0 % CPU. Cost: 10 minutes of wall time with the GPU lock held, one killed run. What would remove it: `whole_tree()` documented at `kernels` as already held, or a reentrant guard that panics on a second take in one thread instead of waiting.
