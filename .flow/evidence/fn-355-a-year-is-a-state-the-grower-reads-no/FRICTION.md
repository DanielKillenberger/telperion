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
