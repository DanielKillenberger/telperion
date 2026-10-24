---
satisfies: [R1]
---
# fn-313-a-prerelease-of-telperion-on-npm.1 The package job's build steps become one shared composite action

## Description
Moves the package job's build steps, from the toolchain install to the artifact-size check, into a composite action so the prerelease workflow (task 2) builds exactly as CI does without a copy. The Tests workflow's package job calls it; its checkout, pack, test:dist and upload stay in the job.

**Size:** S
**Files:** `.github/actions/package/action.yml`, `.github/workflows/tests.yml`
**Touches:** [.github/actions/package/**, .github/workflows/tests.yml]

### Approach
- Follow the composite shape of `.github/actions/nextest/action.yml:1-40` (name, folded description, `shell: bash` on every run step) and nest `./.github/actions/wasm-bindgen` as `.github/actions/suite/action.yml` nests actions.
- Steps, in the job's order (`.github/workflows/tests.yml:318-342`): `rustup toolchain install`, `Swatinem/rust-cache@v2` with `cache-on-failure: true`, `./.github/actions/wasm-bindgen`, `actions/setup-node@v7` with `node-version-file: package.json` and `cache: npm`, `npm ci`, `npm run build`, `node scripts/artifact-budgets.mjs`.
- The size check reads `GH_TOKEN`, `PR_NUMBER` and `PR_HEAD_SHA` (`scripts/artifact-budgets.mjs:166-176`): the job passes them as `env:` on its `uses:` step, which the composite's run steps inherit. No inputs.
- The job keeps its id `package`, its permissions, the pack step id `pack`, test:dist and the upload; `release.yml:96-121` reads the job name and the artifact name.

### Investigation targets
**Required**:
- `.github/workflows/tests.yml:306-369` — the package job
- `.github/actions/nextest/action.yml` — composite pattern with toolchain and cache
- `scripts/artifact-budgets.mjs:160-199` — env the size check reads and its skip path

**Optional**:
- `.github/workflows/release.yml:80-130` — what reads the package job
- `scripts/ci-key.mjs:15-20` — SHARED lists actions the receipt suites use; the package action is used by no receipt suite, so it is not added

### Key context
A local action resolves only after checkout, so checkout stays in each caller. `ci-key.mjs` already hashes `tests.yml`, so this edit changes every suite's receipt key once; that is expected.

## Acceptance
- [ ] `.github/actions/package/action.yml` holds the seven build steps; `tests.yml`'s package job calls it after checkout and keeps pack, test:dist and upload unchanged
- [ ] The size check still receives `GH_TOKEN`, `PR_NUMBER`, `PR_HEAD_SHA` on a pull request run
- [ ] Job id and name `package`, step id `pack` and artifact name `package` unchanged
- [ ] actionlint (1.7.12 with shellcheck) is clean on all workflows
- [ ] `node --test scripts/ci-key.test.mjs` passes

## Done summary
The package job's seven build steps, from `rustup toolchain install` to `node scripts/artifact-budgets.mjs`, now live in `.github/actions/package/action.yml`. The Tests package job checks out, calls it with GH_TOKEN, PR_NUMBER and PR_HEAD_SHA as step env, and keeps pack, test:dist and the `package` upload unchanged. Job id, permissions, step id `pack` and artifact name are as before. ci-key.mjs SHARED is left alone because no receipt suite uses the action; the edit to tests.yml moves every receipt key once, which is expected.

Proof (measured): actionlint 1.7.12 with shellcheck 0.11.0 on .github/workflows/*.yml, rc 0, before and after. The AC's `node --test scripts/ci-key.test.mjs` fails on the base before any edit (the file imports vitest; @vitest/runner throws without a runner), so baseline: red (node --test scripts/ci-key.test.mjs failed pre-edit). The working form `npx vitest run scripts/ci-key.test.mjs` passes, 2/2, before and after. This is logged in FRICTION.md.

Tier: not given by the conductor (model: claude-opus-5-5)

stage: impl-review - ran (codex, one reviewer on correctness, SHIP, rid beec4b493b79417d9cdc4cef9b0c79f9)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 29f19a07b98c71ed60b144f00cd0a8428251b340
- Tests: actionlint -shellcheck=<shellcheck 0.11.0> .github/workflows/*.yml (rc 0), npx vitest run scripts/ci-key.test.mjs (2/2 pass; node --test form red on base, inherited)
- PRs: