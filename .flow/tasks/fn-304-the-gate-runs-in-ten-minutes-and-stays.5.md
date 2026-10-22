---
satisfies: [R8, R9]
---
# fn-304-the-gate-runs-in-ten-minutes-and-stays.5 Budgets: 60 s and 4 GB per test, a shrink-only allowlist, the ten-minute total

## Description
R8 and the close-out R9: the checks that keep the gate where task .4 put it.

**Size:** M
**Files:** `.config/nextest.toml`, a gate script and its allowlist under `scripts/` (or `.config/`), `AGENTS.md`, RESULT.md
**Touches:** [.config/nextest.toml, scripts/**, AGENTS.md, .flow/evidence/fn-304-the-gate-runs-in-ten-minutes-and-stays/**]

### Approach
- The declared runner is nextest (task .4 stops for the host if `cargo test` were faster).
- Time: the gate profile sets `slow-timeout = { period = "60s", terminate-after = 1 }`; each allowlisted test gets a per-test override with its own figure. A timed-out test reports TIMEOUT; the gate script names the budget and fn-304 beside it.
- Memory: a nextest wrapper script (or the gate's target runner) runs each test and reads its peak RSS from `getrusage(RUSAGE_CHILDREN)`, the R1 mechanism (`raw/rss_runner.py`), and fails a test past 4 GB or past its allowlist figure. Verify wrapper-script support in nextest 0.9.145 before choosing it over the target runner. Python fits `scripts/` (other `.py` scripts exist); nothing under `src` or `crates`.
- Allowlist: one file listing each over-budget test's full name, its seconds and MiB on the declared invocation; it is the one source of the figures, and a check fails when the gate profile's per-test timeout overrides differ from it (same names, same seconds). The check compares the file to the base branch's copy: when the base ref resolves and holds no such file, this branch establishes the baseline and the check says so; when the base ref cannot be resolved, the check fails with that error; when the base holds the file, a new entry or a raised figure fails. A listed test that no longer exists fails as stale.
- Total: the gate script runs the declared invocation, prints the total wall time against 600 s and exits non-zero over it; past ten minutes the best figure goes to the host.
- Close-out: the gate once through the script, `npm test`, RESULT.md with every count change named (moves to ignored, merges), friction read.

### Investigation targets
**Required:**
- `.flow/evidence/fn-304-the-gate-runs-in-ten-minutes-and-stays/raw/rss_runner.py`
- `.config/nextest.toml`
- `scripts/artifact-budgets.mjs` and `scripts/artifact-budgets.json` (an existing budget-with-figures pattern)
- `AGENTS.md:37-41`
## Acceptance
- [ ] A test past 60 s is ended and fails by name; shown on a temporary sleeping test.
- [ ] A test past 4 GB fails by name with its figure; shown on a temporary allocating test.
- [ ] The allowlist holds every over-budget test with its figure; adding an entry or raising a figure fails the check; a stale entry fails.
- [ ] The gate script reports total wall against ten minutes; the result recorded.
- [ ] Gate green (fn-302's two tests named if still red), `npm test` green, codex impl review.
## Done summary
R8 and R9 as ruled. scripts/gate.py runs the gate profile with scripts/gate_test.py as runner: 120 s wall cut-off, 4 GiB per test, nine tests allowlisted at twice their worst time and a tenth over their worst peak; the allowlist only shrinks against the base and stale entries fail; a doc-test the gate would not run fails before the run; the test run fails past 1.25 x the median (1,560 s); cargo-nextest 0.9.145 checked first with its install command. Demos: a 125 s sleeper cut at 120 s and passing under a 130 s override; a 4.5 GiB test failed by name; new, raised and stale allowlist entries refused; nextest missing or 0.9.100 refused. Declared gate green: 1,473 passed, 0 failed, 1,137 s test run (median 1,241 s over three runs, 1,137-1,348), peak 5.2 GiB. npm test green (177 tests).
stage: impl-review - ran (codex, three lenses NEEDS_WORK, two fixes, re-review NEEDS_WORK on one finding) (model: gpt-6.1-sol)
OVERRIDDEN: codex finding that the total budget excludes the build and listing. Host (2026-10-22): the budget is there to catch tests growing; build time depends on cache state, not on the change, so counting it would fail a cold build for no defect. Reviewer: the declared command's whole wall time should be held to the budget.

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: af1a97f99028faf34361e955c64523eb9a8dd08f, 438e30b82d578bd5edd4dc4e1233e8dd992b1cb1, fb0c80de2501cbacbd42c9b76ffa7a5b233f7a31, cbc7fd632510911ef22fb5137ee920328a1a8c5f, 17c1a3c070c6cf8bfbceddb91046cc4e16d3f12b, ef44834128172bf8a7c758f62e53bcd27d8e4bf5, 77888b916d711b537357790985650dbeb51de592, 90a24fcdf8faf2f834c4505e67f3eb286f1c7c06, 1ca40a4d53e781820a45e9c50d36ac36531559cf
- Tests: telperion-heavy 8 --threads 4 -- python3 scripts/gate.py (1473 passed, 0 failed, 1,137 s), npm test (177 passed), demo.sh, allowlist_demo.py, nextest version demo
- PRs: