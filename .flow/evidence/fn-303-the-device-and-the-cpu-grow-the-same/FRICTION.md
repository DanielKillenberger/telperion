# fn-303 friction

## 2026-10-21: the bisect queue

- **Doing:** the four bisect builds, one per checkout, each from a cold `target/`.
- **Slowed by:** `telperion-heavy` held two of the four with "room 1 GB for 8" while the other two compiled. Each cold `ci` compile at `--threads 2` takes about 100 s.
- **Cost:** about 3 minutes of queueing.
- **Would remove it:** nothing in the repository. The cold `target/` per checkout is the rule, and the queue is shared.

## 2026-10-21: no per-cycle comparison for a walk frame

- **Doing:** finding the first cycle where the device and the CPU part on a blended walk frame.
- **Slowed by:** `grow_capture_grown_at` exists ("where a build first parts from the CPU's") but is private and unused. The device census tap has no drain outside `Survey::take`. `film_walk_tests::frame` is private. A probe had to open all three.
- **Cost:** about 10 minutes of reading, plus one more compile.
- **Would remove it:** a test-only "first cycle apart" helper that takes any `Family`, next to `grown_tests::held_to_cpu`.

## 2026-10-21: a hook blocked a log redirect

- **Doing:** a loop over the four frames that wrote each frame's log as `raw/r1-nodes-$n.log`.
- **Slowed by:** the dcg hook refuses a redirect to a path holding a shell variable.
- **Cost:** about 2 minutes, to move the loop into a script file.
- **Would remove it:** nothing in the repository. This is local tooling: put such loops in a script from the start.

## 2026-10-22: tracing 3 leaves took nine probe iterations

- **Doing:** tracing oak to palm frame 36's 3 extra CPU leaves to their placement and the branch.
- **Slowed by:**
  - No comparator ties a leaf, a run or a node to its source on both executors for a rosette canopy. `compact_stations` returns None there (`supports_stations`), so `agree_tests::stations` cannot run.
  - Each step needed a probe in a different crate's private scope (telperion-core `foliage`, telperion-render `resident` and `cycle`). Each probe cost a compile through the shared queue.
- **Cost:** about 60 minutes and 14 heavy runs.
- **Would remove it:** a test-only "explain a count gap" helper for any `Family`. It would list unmatched leaves with their run key, and unmatched nodes with their engine axis and phytomer or join, on both sides.

## 2026-10-22: the task 2 gate queued 25 minutes

- **Doing:** task 2's workspace gate, `telperion-heavy 12 --threads 2`.
- **Slowed by:** the shared queue held it from 01:59 to 02:24 ("room 10 GB for 12") behind another worktree's heavy job.
- **Cost:** 25 minutes idle, on top of the gate's own run of about 35 minutes.
- **Would remove it:** nothing in the repository. The gate's 12 GB declaration and the shared desk are by design. A queue position or ETA in the wrapper's waiting line would let an agent plan around it.

## 2026-10-22: the worktree has no node_modules for npm test

- **Doing:** preparing `npm test`, whose `pretest` builds the Wasm and the render package.
- **Slowed by:** a fresh worktree has no `node_modules`, so `npm ci` runs first.
- **Cost:** `npm ci` took about a second from the cache. It skipped esbuild's install script under the local install-scripts policy, and `npm test` passed regardless.
- **Would remove it:** a local setup step for new worktrees, for example the worktree kit copying or linking `node_modules`. This is local setup, not a repository change.
