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
`scripts/gate.py` now runs two of CI's checks with `tests.yml`'s commands. The package check (`npm run build`, then `node scripts/artifact-budgets.mjs`, ceilings only locally) took 37 s after a core change and 2 s warm, so it runs before the test run. The core without geometry (`cargo build` and `cargo test --lib`, `--no-default-features`) took 220 s after a core change and 139 s warm, nearly all of it the lib tests (139-172 s), so it runs after the test run, outside `TOTAL`, and also when the test run failed. A failing or unstartable check fails the gate, naming the check and its command. The module docstring and AGENTS.md's gate line give both costs and placements.

Costs were measured 2026-10-24 through `telperion-heavy 8 --threads 4`, with another checkout's gate (f4-311) running on the desk. Timed logs are in `.flow/evidence/fn-314-the-release-branchs-ci-is-green-and-the/raw/` (`no-geometry-timed.log`, `package-timed.log`). The plan-time 49 s figure covered only the build and `--no-run`; the lib tests are what push this check past a minute.

How the checks were exercised without the nextest run (measured):
- Red: a scratch commit reverting 57203f92b, then `gate.ci_check(*gate.PACKAGE)` and `gate.ci_check(*gate.NO_GEOMETRY)` called directly (`python3 -c 'import sys; sys.path.insert(0,"scripts"); import gate; ...'` inside `telperion-heavy 5 --threads 2`). Both failed with their own output: `FAIL dist/wasm-source.js: not built by the recipe` and `error[E0425]: cannot find function bloom_element`. The error lines named each check (`raw/checks-red.log`). The scratch commit was dropped with `git reset --hard` back to a8ba24f9a.
- Green: the same call on a8ba24f9a returned no errors; the lib tests ran 142 passed (`raw/checks-green.log`).
- Missing npm: `ci_check(*PACKAGE)` with `PATH` emptied returned `package: npm run build cannot start: [Errno 2] ... 'npm'` and exited 1 (`raw/no-npm.log`).
- Placement and exit logic: `gate.main()` ran with the nextest run and both checks stubbed (a scratchpad script, not committed) through six cases (`raw/gate-flow.log`). The order was always package, then the test run, then no-geometry. No-geometry still runs when the test run fails. Any failure exits 1; all green exits 0.

The full gate was not run here; task .3 runs it.

baseline: green (both Quick commands rc=0 pre-edit, from the cost measurement runs)
Tier: session (jev moderate 0.57) at default effort (model: claude-opus-5-5)
stage: impl-review - ran [14:19..14:21] SHIP, codex, one correctness reviewer (model: gpt-6.1-sol)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: a8ba24f9af444315042035b330534ac0940248e4
- Tests: python3 -c 'import gate; gate.ci_check(*gate.PACKAGE)+gate.ci_check(*gate.NO_GEOMETRY)' (red on a revert of 57203f92b, green on a8ba24f9a), gate.main() control flow with nextest and checks stubbed (6 cases)
- PRs: