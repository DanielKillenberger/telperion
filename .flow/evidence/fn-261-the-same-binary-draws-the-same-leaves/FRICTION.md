# fn-261 friction

## 2026-10-11, task 1

- **The first R1 run deadlocked for 10 minutes holding the GPU lock.** The test took `generation::whole_tree()` itself, as `florets_tests` does, and `roots::kernels` takes the same non-reentrant mutex in test builds (`_one`), so the build waited on its own test forever at 0 % CPU. Cost: 10 minutes of wall time with the GPU lock held, one killed run. What would remove it: `whole_tree()` documented at `kernels` as already held, or a reentrant guard that panics on a second take in one thread instead of waiting.
- **The dcg hook blocked every way of laying the base files back to time the pre-fix binary**: `git checkout --`, `git restore`, and `mv` or `>` to a path held in a variable. A detached `git checkout <commit>` of a clean tree was the way through, after a WIP commit. Cost: about 5 minutes and four refused commands. What would remove it: a documented before/after recipe (a probe binary built at a named commit into a scratch directory), or a flag on the probes that names the commit they were built from.
- **ImageMagick 7's `compare -metric AE` again reported an intensity-weighted figure** (226 for 31,574 differing pixels), as fn-258 found. Counted instead with a difference image, its channel maximum, then a threshold. Cost: 2 minutes. What would remove it: the pixel-difference helper fn-258 asked for.
