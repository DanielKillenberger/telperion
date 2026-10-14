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
