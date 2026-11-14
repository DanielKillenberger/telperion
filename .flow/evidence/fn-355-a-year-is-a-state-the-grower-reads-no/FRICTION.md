# fn-355 friction

## 12 Nov 2026, 01:10, builder (task 1): the shell hook refuses scratch redirects and heredocs that mention code

- **Doing:** writing the anchor bundle to a file, and multi-file edits through a short Python heredoc.
- **Slowed by:**
  - The hook refuses a redirect to the session scratchpad, since it lies outside a VCS worktree.
  - It also refuses a heredoc whose text contains backticks, which it reads as command substitution. Rust doc comments are full of backticks.
- **Cost:** about 6 minutes and five refused commands. The workarounds were the edit tool, `dcg create-new`, and redirects to new files under the evidence `raw/` directory.
- **What would have removed it:** nothing in the repository; it is a local hook, reported only.

## 12 Nov 2026, 01:15 to 02:40, builder (task 1): the narrow suites took 87 minutes under load

- **Doing:** the narrow space and core suites, once, inside `telperion-heavy 16 --threads 4` with `-j 3`.
- **Slowed by:**
  - The machine was at load 40 to 57, with the twelve-seed scoreboard and other sessions running.
  - The job waited 5 minutes for room in the queue.
  - `suite::sweep::every_wire_parameter_a_preset_moves_is_walked` ran past 28 minutes, and `every_walk_is_the_walk_it_was` took 1,194 s.
  - The run took 5,193 s, against 603 s in fn-354 task 4.
  - A first build of the windows filled the expected-length table to the longest stage chain of any age, not only the sleeping ones. That cost 2 to 6 s a tree and added to this run, but did not change its results.
- **Cost:** about 90 minutes of wall time. Other work went on in parallel.
- **What would have removed it:** a quieter machine, or the gate's receipts for the suites a change cannot reach.

## 12 Nov 2026, 17:50, blend-cost diagnosis: no sampling profiler, and the queue held a 15 s probe for 8 minutes

- **Doing:** profiling one oak-to-beech blend at the head of `gate/blend-cost`.
- **Slowed by:**
  - `perf` is not installed, and `samply record` fails with "mmap failed" both inside `telperion-heavy` and outside it (memlock limit 8,192 KB). The stages were timed instead with temporary counters, built and reverted by hand.
  - One probe build-and-run (about 40 s of work) waited about 8 minutes for admission at load 45 to 55, with six other heavy jobs queued.
- **Cost:** about 15 minutes of wall time and two extra probe rebuilds.
- **What would have removed it:** a raised memlock limit or `perf` on the machine (local setup, reported only).

## 13 Nov 2026, 00:20 to 00:50, blend-cost diagnosis: a narrow suite SIGTERMed mid-run, and no `time` or `bc` binary

- **Doing:** the core's narrow suites after the fix, and then CPU-time comparisons.
- **Slowed by:**
  - The first core narrow run ended after 9.6 s with 7 of 47 tests unrun and 4 aborted by SIGTERM. It ended when the machine's rules changed (the yield watcher arrived), not on a test result.
  - One of those four, `repeat_tests::the_largest_tree_and_the_palm...`, printed "a NaN in the tree" before its SIGTERM. Rerun alone, it passed on the fix and on the base.
  - `/usr/bin/time` and `bc` are not installed. A timing script recorded 0.0 s walls, and a later one exited 127, so the runs were redone with bash's `time` builtin.
  - The watcher paused the repeat test for 17 minutes, which took its wall time to 614 s against 111 s on the base. Its CPU time stands.
- **Cost:** about 25 minutes, and one rerun each of the repeat test and the narrow suite.
- **What would have removed it:** a note in AGENTS.md that CPU time is taken with bash's `time` (TIMEFORMAT), since neither GNU time nor bc is on the machine (local setup, reported only).
