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
TBD

## Evidence
- Commits:
- Tests:
- PRs:
