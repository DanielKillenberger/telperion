# fn-346 friction

## 11 Nov 2026, worker (R1)

- **What:** writing the probe's build and census scripts, then reverting the probe from the crates.
- **What slowed it:** the local `dcg` hook refused a heredoc whose script redirected to `"$R/build.log"`, although the redirect was inside the script file being written, not run. It also refused `git checkout -- crates`, so the revert went through `git apply -R probes/census.patch`.
- **Cost:** about 3 minutes and two tool calls.
- **What would remove it:** the dispatch already says to put runs in script files. The fix is to write those files with the editor's Write tool, never with a shell heredoc, and to revert a probe with `git apply -R <patch>`. A line saying both in the dispatch template would have saved the two refusals.

## 11 Nov 2026, worker (rules A and B run)

- **What:** drawing the oak on seed 1001 for Q, QA, QB and QAB in one batch.
- **What slowed it:** the worker wrapped the batch in `timeout 595` to stay inside one tool call. The 300-year draws take 90 to 110 s each, so the batch passed 595 s and was killed during QB at 300. No partial still was used; QB at 300 and every QAB still were drawn again in a second batch.
- **Cost:** about 2 minutes of lost draw and one more batch.
- **What would remove it:** run a still batch in the background without a timeout and wait on its completion, never under a guard shorter than the batch.
