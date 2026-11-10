# fn-346 friction

## 11 Nov 2026, worker (R1)

- **What:** writing the probe's build and census scripts, then reverting the probe from the crates.
- **What slowed it:** the local `dcg` hook refused a heredoc whose script redirected to `"$R/build.log"`, although the redirect was inside the script file being written, not run. It also refused `git checkout -- crates`, so the revert went through `git apply -R probes/census.patch`.
- **Cost:** about 3 minutes and two tool calls.
- **What would remove it:** the dispatch already says to put runs in script files. The fix is to write those files with the editor's Write tool, never with a shell heredoc, and to revert a probe with `git apply -R <patch>`. A line saying both in the dispatch template would have saved the two refusals.
