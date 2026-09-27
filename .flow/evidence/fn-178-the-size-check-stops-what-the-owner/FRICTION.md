# fn-178 friction

## 2026-09-27, fn-178.1 worker

- Doing: editing scripts/artifact-budgets.mjs and summing the gate logs from the shell.
- Hindered: the dcg pre-tool hook blocked three ordinary commands (a Python heredoc containing a JS template literal, and two `$VAR`-launched flowctl calls), each needing a rewrite through the Edit tool or literal paths.
- Cost: about 4 minutes and three extra tool calls.
- Would have removed it: a dcg allowlist entry for the flowctl plugin path and for Python heredocs that only rewrite repo files.

- Doing: codex impl-review of the size check.
- Hindered: all three reviewer draws reported they could not run vitest in their sandbox (no dependencies installed where they looked), so they verified with hand-written Node assertions instead.
- Cost: nothing extra to this task; the reviewers' own time.
- Would have removed it: the reviewer sandbox resolving node_modules from the worktree, or the review prompt naming `npx vitest run <file>`.
