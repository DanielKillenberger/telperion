# Where the test suite's time goes

## Goal

A recon spec: measure where the local gate's and CI's time goes and which tests earn it, and end in a ranked list of follow-up specs, changing no test. CI had grown from fn-76's 2 min 55 s (2026-09-18) to 7 to 8.5 minutes and the local gate took 5 to 10 minutes, with friction entries across `.flow/evidence/` recording repeated gate runs, failures found one per run, pins in many places, a replay keyed on rendered images and a race. It follows `docs/principles.md`, "Question, delete, then optimise".

## Decisions

- user (2026-10-02): "are we using significant amount of time running test suites?"
- user (2026-10-02, on measuring first, then a spec): "or just capture a recon spec that i could take over in another session for the test suite optimization"
- Recon only: building any follow-up is its own spec. The objectives race (`objectives.rs:28`) was fixed on the fn-183 branch (host, 2026-10-02).
- Prior art and baseline: fn-76 (one generation per specimen, digests, nextest, shards); the owner's CI bound, "i like ci being below 5min. Ideally much below" (fn-76).

## Acceptance Criteria

- **R1:** Slowest 30 tests and time per binary and module from a local gate run and a CI run at the same commit, compile vs test, cold and warm — partial: local measured at master `ebd2e37b` (cold build 47 s, incremental 34 s, gate 427 s with fresh specimens and 342 s cached, nextest 193 s); no CI run at the same commit (the `Tests` path filter excludes `.flow/**`), so CI numbers come from the last master runs, 11 commits back.
- **R2:** For the slowest 30 tests and every pin family: what each catches, what else catches it, the last real defect, named requirement, and delete/merge/keep/speed-up — met. No pin family caught a defect in 30 days except the species digests (fn-177's empty `Tree::pipe`).
- **R3:** The gap between 2 min 55 s and 7 to 8.5 minutes explained — met: since #126 (09-26) the `core without geometry` job reruns all 515 lib tests (333 s of a 400 s job, the critical path), because the self dev-dependency does not set `default-features = false`, so `geometry` unifies back on.
- **R4:** A ranked list of follow-up specs with savings, obvious fixes marked — met (below).

## Result

Ranked follow-ups (est. unless measured):
1. The no-geometry job tests the no-geometry build (`default-features = false` on the self dev-dependency): CI ~8 to ~5 min. Obvious. (Captured as fn-186.)
2. The local gate runs under nextest as CI does: 427 to 193 s measured, ~120 s after fn-183; ends the shared-process memory-ceiling flake. Needs the owner's word (changes the AGENTS.md gate).
3. One recorder rewrites every pin from one run: 10 to 25 min per intended tree change.
4. The replay tape keyed by request shape, not dial-table text: four re-records in four days, ~50 Jev calls.
5. Delete what another test already asserts (#29 obvious; pin deletions to the owner): 100 to 150 CPU-s.
6. The growth path stops gating: covered by fn-181 (344 CPU-s, 128 tests).
7. The slow suites build each tree once via the specimen cache: 200 to 300 CPU-s.
8. The generation-limits inventory keyed to survive a rewrite, or deleted (owner call; most churn, no recorded catch).
9. Surface every failing assertion in one run (collecting assertions); weighs a rule that exploratory steps touching generator code run the suite once `--no-fail-fast` (owner decides).
10. The objectives race: already fixed.

The longest local test was jev's GPU beech replay (152 s), removed by fn-183.

## Open

Follow-ups 2 to 9 are for the owner to spec.

## Friction

- Measurements waited on another session's builds (load ~30); `bc` missing -> local setup, reported.
- No CI run at the measured commit (path filter excludes `.flow/**`) -> proposal: a `workflow_dispatch` trigger at a named commit.

## Evidence

- `.flow/evidence/fn-185/REPORT.md` (the recon report: per-test timings, CI job split, R2 verdicts per test and pin family, R4 list).
