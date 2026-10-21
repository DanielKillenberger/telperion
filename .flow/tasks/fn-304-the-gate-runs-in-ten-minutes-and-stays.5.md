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
TBD

## Evidence
- Commits:
- Tests:
- PRs:
