---
satisfies: [R3]
---
# fn-314-the-release-branchs-ci-is-green-and-the.2 The gate runs both CI checks, at their measured cost

## Description
`scripts/gate.py` runs CI's no-geometry commands and the package job's build-and-budget commands, so neither failure can reach CI again. Where each runs follows its measured cost.

**Size:** M
**Files:** `scripts/gate.py`, `AGENTS.md`
**Touches:** [scripts/gate.py, AGENTS.md]

### Approach
- Measure first, on this desk through telperion-heavy at the gate's four threads: each check after a one-line change to a core crate source (touch a `.rs` file under `crates/telperion-core/src`), and warm with nothing changed. Save the timings to `.flow/evidence/fn-314-the-release-branchs-ci-is-green-and-the/raw/`.
- Add to `gate.py` a list of the two checks as argv lists copied from `tests.yml` (no-geometry: the `cargo build` and `cargo test ... --lib` lines; package: `npm run build`, `node scripts/artifact-budgets.mjs`). Run each with `subprocess.run(..., cwd=ROOT)` and stream its output; a nonzero exit or an `OSError` (no npm) fails the gate naming the check. Print each check's wall time, as `gate: build and listing` does (`gate.py:150`).
- Placement by the R3 rule: a check measured under about a minute after a core change runs with the pre-run checks (beside `shrinks_only`/`stale`/`doc_tests`, `gate.py:147-152`); one over it runs after the timed test run, outside `TOTAL`, and still runs when the test run failed. The gate's exit is nonzero if any check failed.
- Update the module docstring and AGENTS.md's gate line (line 39) to name the two checks, where each runs, and its measured cost. The local budget check holds ceilings only (no PR base), and the line says so.
- No test file exists for `gate.py`; prove it by running the gate's checks red (temporarily revert task 1's two fixes in a scratch commit you drop, or run the guard function against the base) and green. Do not add a test harness for gate.py.

### Investigation targets
**Required**:
- `scripts/gate.py:1-25,138-170`
- `AGENTS.md:39`
- `.config/nextest.toml:28-38` (fn-306's guards, for the wording)
- `.github/workflows/tests.yml:161-176,317-360`

### Key context
- The gate's `TOTAL` times only the nextest run; keep it that way.
- Never share a `target/` with another checkout.

## Acceptance
- [ ] Each check's measured cost (after a core change, and warm) is recorded in evidence and in AGENTS.md's gate line
- [ ] A check under about a minute runs before the test run; one over it runs after, outside `TOTAL`, and AGENTS.md says so
- [ ] Each check failing (shown on the base's two breaks) fails the gate with its own output; a missing npm fails it, never skips it
- [ ] Both checks green on the fixed branch

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
