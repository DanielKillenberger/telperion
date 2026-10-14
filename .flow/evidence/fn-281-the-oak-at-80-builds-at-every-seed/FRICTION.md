# fn-281 friction

## 2026-10-14, task 1: the probe's first build waits for room

- **Doing:** building the sweep probe (`radii_sweep`) in a fresh worktree with no `target/`, under `telperion-heavy 6`.
- **Slowed by:** the build itself is admitted as a heavy job; the desk had 3 GB of room for 10+ minutes while other workers ran, and a cold workspace build cannot share the other checkouts' `target/` (AGENTS.md forbids it).
- **Cost:** 10+ minutes waiting before the first compile.
- **Would remove it:** a smaller admission for a compile (a cold `ci` build of one example peaks well under 6 GB), or a shared sccache so a new worktree does not rebuild the dependency graph cold.
