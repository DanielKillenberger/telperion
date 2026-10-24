---
satisfies: [R4]
---
# fn-314-the-release-branchs-ci-is-green-and-the.3 Every tests.yml job green locally; the gate and npm test

## Description
The end-of-spec proof: every `tests.yml` job's command run locally on the fixed branch, one heavy thing at a time, and the gate once.

**Size:** S
**Files:** `.flow/evidence/fn-314-the-release-branchs-ci-is-green-and-the/RESULT.md`
**Touches:** [.flow/evidence/fn-314-the-release-branchs-ci-is-green-and-the/**]

### Approach
- Jobs and commands (`tests.yml`): `fmt` (`cargo fmt --all --check`); `receipts` (`node scripts/ci-key.mjs <suite>` for the five suites; the cache lookup is CI-only); `core 1-4/4`, `render`, `wasm, field and jev` each through the nextest action's own command (`.github/actions/nextest/action.yml`: `cargo nextest run --cargo-profile ci -p <packages> --no-fail-fast`, the core once per `--partition count:N/4`, or once unpartitioned, which runs the union of the four; render alone; `-p telperion-wasm -p telperion-space -p telperion-jev`), not the gate, since the gate runs another nextest profile and target runner (plan review, codex round 1); `core without geometry` (its two commands); `rust-receipts` (CI cache writes only: not runnable locally, say why); `node` (`npm ci`, `npm test`, `npm run typecheck`); `bindings` (`npx playwright install chromium` if absent, `npm run test:bindings`); `package` (`npm run build`, `node scripts/artifact-budgets.mjs`, `npm pack`, then remove dist, unpack the tarball's `package/dist`, `npm run test:dist`; remove the tarball after).
- The gate: `~/.local/bin/telperion-heavy 8 --threads 4 -- python3 scripts/gate.py`, once. Every other heavy command through `telperion-heavy 5 --threads 2`. Check `ps -eo pid,args | grep target/ci/deps` before each.
- Save each command's full output under the evidence `raw/` directory (gitignored) and write one row per job to RESULT.md: command, result, wall time.

### Investigation targets
**Required**:
- `.github/workflows/tests.yml`
- `.github/actions/nextest/action.yml`
- `docs/evidence-retention.md`
## Acceptance
- [ ] RESULT.md has one row per `tests.yml` job: its local command and result, or why it cannot run locally
- [ ] The Rust jobs ran through the nextest action's command, not through the gate
- [ ] The gate green once, at the end
- [ ] `npm test` green
- [ ] The working tree carries no build output (tarball, unpacked dist) after the run
## Done summary
Every `tests.yml` job's command was run locally on the fixed branch. Each one is green or named as CI-only, with one row per job in `.flow/evidence/fn-314-the-release-branchs-ci-is-green-and-the/RESULT.md`. The gate ran once, last of the heavy runs, and was green in 1,455 s: build and listing took 141 s, the package check 2 s, the test run 1,200 s against its 1,560 s budget, and the no-geometry check 111 s. `npm test` passed all 177 tests in 22 files.

The Rust jobs ran the nextest action's command, not the gate's. `NEXTEST_TEST_THREADS` was set because nextest ignores the `RUST_TEST_THREADS` that `telperion-heavy` exports. At the dispatch's 5 GB cap, four runs were ended by the OOM killer, and FRICTION.md has one entry for each:

- render and wasm/field/jev: each finished across two runs, the second taking an `-E` filterset of exactly the tests the first had not passed.
- wasm/field/jev: the second run used `telperion-heavy 8 --threads 2`, because `walks every_setting_changes_the_tree_by_degree...` alone peaks at 5,396 MiB. This departs from the 5 GB rule, and the run's own measurement forced it.

`rust-receipts` writes only CI's caches, so it was not run locally. In the first OOM-ended core attempt, one test panicked with `a NaN in the tree`. That test passed in three later runs, and the cause is unknown (RESULT.md, Notes).

Follow-up: `telperion-heavy` could export `NEXTEST_TEST_THREADS` beside `RUST_TEST_THREADS`. It is a desk script outside the repository.

Tier: session (jev long_running 0.67) at default effort (model: claude-opus-5-5)

stage: impl-review - ran (codex, one correctness reviewer, round 1 SHIP; one P3 wording finding fixed in 190a513bb)
GATE_SKIPPED:quick:docs-only - cumulative diff classified tier-B (no executable paths touched); the full gate ran green regardless (raw/gate.log)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: bc201e4c780945180808aac91ae2b717e015bdbb, 190a513bb4c524b792bfe78c35f93e0029434771
- Tests: ~/.local/bin/telperion-heavy 8 --threads 4 -- python3 scripts/gate.py (green, 1455 s), npm test (177 passed), every tests.yml job command, see RESULT.md
- PRs: