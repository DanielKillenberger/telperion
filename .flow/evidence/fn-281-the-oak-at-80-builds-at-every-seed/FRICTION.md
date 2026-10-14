# fn-281 friction

## 2026-10-14, task 1: the probe's first build waits for room

- **Doing:** building the sweep probe (`radii_sweep`) in a fresh worktree with no `target/`, under `telperion-heavy 6`.
- **Slowed by:** the build itself is admitted as a heavy job; the desk had 3 GB of room for 10+ minutes while other workers ran, and a cold workspace build cannot share the other checkouts' `target/` (AGENTS.md forbids it).
- **Cost:** 10+ minutes waiting before the first compile.
- **Would remove it:** a smaller admission for a compile (a cold `ci` build of one example peaks well under 6 GB), or a shared sccache so a new worktree does not rebuild the dependency graph cold.

## 2026-10-14, task 1: the device check queues behind the sweep

- **Doing:** building the oak at 80, seed 7, on the device (`every_tree_builds_on_the_device` filtered to the one case) while the CPU sweep ran.
- **Slowed by:** the sweep's shard holds a 5-6 GB claim, and a guessed 8 GB device ask never found room; there is no stated host-RAM figure for one device build of an 80-year oak to size the ask from.
- **Cost:** about 20 minutes waiting before the ask was cut to 5 GB.
- **Would remove it:** a printed host peak per device build in fn-276's sweep log, so a later ask is sized from a measurement.

## 2026-10-14, task 2: the device check refused for memory, then the shader swaps

- **Doing:** holding the device's dress of the oak at 80 to "no radius-zero node", red on the base shader and green on the fix.
- **Slowed by:** `tree::drawn` called outside a build is not held to the device budget, so the first run was refused with "Out of Memory" at the cycle inputs; only `build::crowned` takes `ledger.capped`. The red run then needed the base shader, and the hook (rightly) refuses `git checkout --` and `git show > path`, so each side was a file swap and its own rebuild.
- **Cost:** about 25 minutes and three render-test rebuilds.
- **Would remove it:** a test helper that draws a family's tree on the device under the budget, and a device-node readback the CPU sweep's probe can call, so one binary reports both executors.

## 2026-10-14, task 2: focused tests starve behind my own two sweeps

- **Doing:** running the core's `pipeline::space` tests while the base and fixed digest sweeps ran side by side.
- **Slowed by:** each sweep shard holds a 5-6 GB claim, and the two of them leave 1-2 GB of room, so a 6 GB test ask waited 30+ minutes. The ask was a guess; the space tests' peak is not printed anywhere.
- **Cost:** 30+ minutes of waiting.
- **Would remove it:** one sweep binary that builds both the base and the fixed conversion of a tree in one process (a test-only switch is not allowed, so a stored per-tree digest table on the branch tip would do), and measured peaks for the focused suites.
