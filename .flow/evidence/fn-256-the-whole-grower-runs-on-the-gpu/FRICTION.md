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
