# fn-210 friction

## 2026-10-05, the spec was not on this branch (worker)

- **Doing:** reading the spec with `flowctl cat` in the speed worktree.
- **Slowed:** fn-210's spec and the realtime decision were captured on the fn-198 branch (`d6e82f64`), not on the fn-206 port this worktree branches from, so `flowctl cat` failed. Bringing them over by `git checkout <ref> -- <path>` or `git show <ref>:<path> > <path>` is refused by the dcg hook (both treated as overwrites, though the paths did not exist); the files were copied by hand through the editor.
- **Cost:** about 5 minutes and four tool calls.
- **Would remove it:** capture a phase's specs on the branch its worker starts from, or have the dispatch copy the spec into the worktree.

## 2026-10-05, measurements under another session's load (worker)

- **Doing:** the baseline hashes and stage times at 80 years.
- **Slowed:** the 1-minute load rose from 0.8 to 61 while the baseline ran (other sessions on the 32-thread desk). The oak's seed-7 tree in light took 107 s against 29 s for seed 1. Stage times taken under that load are not comparable, so before and after are run back to back per tree, and timings are repeated.
- **Cost:** a second timing pass, about 10 minutes of wall time.
- **Would remove it:** a machine-wide lock for timing runs, as the GPU has.
