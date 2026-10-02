# fn-185 friction

## 2026-10-02: measurements waited on another session's builds

- **Doing:** the local gate timings (R1).
- **Slowed by:** the fn-183 session was building and running `npm test` on the same machine (load about 30). Timing under that load would have inflated every number, so the run waited for idle. My first try was also lost: `bc` is not installed and the step timer printed zero.
- **Cost:** about 10 minutes of waiting, and one restarted script.
- **Would have removed it:** a shared "machine busy" marker that sessions check before heavy runs. This is local setup on the owner's machine, so it is reported, not specced.

## 2026-10-02: no CI run at the measured commit

- **Doing:** R1's same-commit CI run.
- **Slowed by:** the `Tests` workflow's path filter excludes `.flow/**`, so a branch that changes only `.flow/` starts no CI run. The CI numbers come from the last master runs, 11 commits back.
- **Cost:** none in time; R1 is partly met.
- **Would have removed it:** a `workflow_dispatch` trigger that runs the suite at a named commit.
