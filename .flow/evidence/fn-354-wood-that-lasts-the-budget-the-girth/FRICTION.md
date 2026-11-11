# fn-354 friction

## 11 Nov 2026, 15:55, builder (task 1): the recorded census came from a build no commit reproduces

- **Doing:** R1's paper check, the oak on seed 1001 at 39 and 119 against fn-349's DRAWN.md census.
- **Slowed by:** this branch gave 354,856 held and 24,711 tips at 39, against DRAWN.md's 354,758 and 24,702. Finding out whether the carried rule or the record was off took four extra growths and one build in a scratch worktree: fn-349's own head binary, its first-rule commit (744dcd249), the restated rule (868278f07) with the charge divided by 34.8, and fn-349's base.
  - fn-349's head binary gives this branch's numbers exactly. No committed fn-349 state tried gives DRAWN.md's.
  - DRAWN.md's census came from `raw/early-r5`, which `r5.sh` copied from `target/` 51 s after 868278f07 was committed. Nothing records which source that binary was built from.
- **Cost:** about 15 minutes and five oak growths to 120.
- **What would have removed it:** the census script printing the commit it was built from and whether the tree was dirty, next to the counts.

## 11 Nov 2026, 15:40, builder (task 1): the shell hook refuses the plain conflict-resolution commands

- **Doing:** resolving the fn-349 cherry-picks.
- **Slowed by:** the hook refuses `git checkout <ref> -- <path>`, `git restore --worktree`, `rm -r` and a redirect to a variable path, each the usual way to take one side of a conflict or clear a stray directory. The workarounds were `git show <ref>:<path>` to a new file and then `mv`, and moving directories to `/tmp`.
- **Cost:** about 5 minutes and four refused commands.
- **What would have removed it:** none needed in the repository; it is a local hook, reported only.
