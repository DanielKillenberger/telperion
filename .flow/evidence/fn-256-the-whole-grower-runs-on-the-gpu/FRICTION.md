# fn-256 friction

## 2026-10-11, task 1: start
- **Doing:** the Phase-1 anchor in the `gpucycle` worktree.
- **Hindered:** `flowctl` is not on PATH in the worker's shell; found under the plugin cache by hand (fn-254 and fn-255 reported the same). The dcg hook also refused a heredoc into a file whose parent directory the same command created.
- **Cost:** about 2 minutes.
- **Would remove it:** `flowctl` on PATH for dispatched workers (local setup, not a repository change).

## 2026-10-11, task 1: replay runs
- **Doing:** writing each replay's output to `raw/` from a shell loop over the two captures.
- **Hindered:** the dcg hook refuses a `>` whose target holds a loop variable; each run had to be its own call with a literal path.
- **Cost:** about 1 minute.
- **Would remove it:** nothing in the repository; local hook policy.

## 2026-10-11, task 1: the capture's births
- **Doing:** counting each apex's births in the busy cycle.
- **Hindered:** an axis born this cycle can bear another the same cycle (a continuation carried on within it), so a bearer is not always a living apex; the first capture panicked on it.
- **Cost:** about 2 minutes and one 9 s recapture.
- **Would remove it:** nothing; the capture now credits such an axis to its first bearer.

## 2026-10-11, task 2: step 0 merges
- **Doing:** resolving the fn-255 merge's conflict in `grow/cycle.rs` by taking fn-255's side.
- **Hindered:** the dcg hook refuses `git show <ref>:<path> > <path>` (and so `git checkout <ref> -- <path>`); the conflict had to be resolved by a script editing the markers in place. `flowctl` again not on PATH.
- **Cost:** about 2 minutes.
- **Would remove it:** nothing in the repository; local hook policy and setup.

## 2026-10-11, task 2: the units kernel's first device run
- **Doing:** the first young-tree run of `generation/cycle` (units and wake, count and emit) on the GPU.
- **Hindered:** the test spent about 340 of its 348 s before its first line of output, which is the driver compiling the module's six entries. Every entry inlines `run`, and inside it `stop`, `sprout` and `grow_unit` are inlined at every call site (fn-235 met the same in the chain grower).
- **Cost:** about 6 minutes of GPU lock per run.
- **Would remove it:** one call site per heavy function (a small state machine of operations in `run`) and one entry for count, emit, units and wake, chosen by the uniform. Being done now.

## 2026-10-11, task 2: tracing the device's differences
- **Doing:** finding why node presences, and in the wake one living apex's slot, differed from the CPU's.
- **Hindered:** the agreement report gives outputs only. Each hypothesis needed a hand-written diagnostic or a swapped contract, then a 40 to 80 s GPU run under the lock. The f32 trace took about 8 runs; the live-list race, a stale write left by `carry_on`'s pop, took 5.
- **Cost:** about 20 minutes of GPU lock and the edits around them.
- **Would remove it:** a test switch that has the cycle kernel record each phytomer's draw, bound, window and lead, and each invocation's own appends beside its counts, for the host to recompute on the CPU. The count-against-emit check is cheap enough to keep permanently.

## 2026-10-11, task 2: the layout's sine on the device
- **Doing:** porting libm's `sincosf`, which evaluates in f64, as pairs of f32s (two-sum, two-product by `fma`).
- **Hindered:** the NVIDIA driver reorders f32 add and subtract chains as well as fusing products: the error-free transforms collapsed, and a 1M-input check found 18 to 28 % of results off by one to three ulps. `atan2f`'s `(z - PI_LO) - PI` is reordered the same way (113k of 124k differing results). Rewritten in integers of 2^-61, which no driver can reorder: 0 of 1M differ.
- **Cost:** about 15 minutes and three GPU runs.
- **Would remove it:** knowing up front that the device reorders sums, not only fuses products (UNITS.md named fusion only). Now in LAYOUT.md.

## 2026-10-11, task 2: WGSL names and constants
- **Doing:** first validation of the layout's shader.
- **Hindered:** `from` and `target` are reserved words in WGSL, and naga refuses `bitcast` in a module `const`; each was found by one full test build of `telperion-render` (about 1 minute each, three rounds).
- **Cost:** about 4 minutes.
- **Would remove it:** a naga check runnable on one shader source without building the crate's test binary.

## 2026-10-11, task 2: tracing the layout's last differences
- **Doing:** attributing the young layouts' remaining float differences under the exact experiment.
- **Hindered:** the comparison names columns, not the operation that rounded. Each of the last three causes needed a hand-written readback of the device's intermediates: dominance's `held` fused across a variable, `(tip + d len) - tip` simplified to `d len`, and the contract's `exp2c`. Each took a 1 to 2 minute rebuild and GPU run.
- **Cost:** about 15 minutes, five runs.
- **Would remove it:** UNITS.md's test switch (a kernel recording its intermediates beside its outputs), which applies to the layout as well.
