---
satisfies: [R1, R2, R3, R4]
---
# fn-313-a-prerelease-of-telperion-on-npm.3 The prerelease moves into release.yml as its own job, published by trusted publishing (host ruling)

## Description
Host ruling, 2026-10-24: only `release.yml` is npm's trusted publisher, so a separate `prerelease.yml` could not publish. The prerelease becomes a `prerelease` job of `release.yml`, started by `workflow_dispatch` (input `dry_run`), publishing by OIDC with no token; the `publish` job's behaviour is unchanged.

**Size:** M
**Files:** `.github/workflows/release.yml`, `.github/actions/npm-publisher/action.yml`, `.github/workflows/prerelease.yml` (deleted), `docs/prerelease.md`, `scripts/release-workflow.test.mjs`
**Touches:** [.github/workflows/release.yml, .github/workflows/prerelease.yml, .github/actions/npm-publisher/**, docs/prerelease.md, scripts/release-workflow.test.mjs, .flow/evidence/fn-313-a-prerelease-of-telperion-on-npm/**]

### Approach
- The publish job's setup-node (Node 24, registry-url) and npm 11.5.1+ install move into `.github/actions/npm-publisher`, called by both jobs; the publish job keeps its `if:` on the step.
- The prerelease job carries task 2's version, refusal, pack and publish steps, minus the token.
- A vitest test reads `release.yml` and checks each job's guard admits only its own event and that no dist-tag is `latest`.

## Acceptance
- [ ] `release.yml` has `workflow_run` and `workflow_dispatch` (input `dry_run` only); `prerelease` runs only on a dispatch, `publish` only on a Tests run
- [ ] The publish job's steps are unchanged in effect; its npm setup is the shared action
- [ ] No `NPM_TOKEN`; `prerelease.yml` deleted; docs say `gh workflow run release.yml -r <branch> -f dry_run=true`, no secret
- [ ] `scripts/release-workflow.test.mjs` passes and fails when a guard is broken
- [ ] actionlint clean; local dry run of the job's steps passes

## Done summary
Host ruling 2026-10-24: the prerelease is a `prerelease` job of `release.yml` (trigger `workflow_dispatch`, input `dry_run` only), publishing `<version>-next.<YYYYMMDD>.<sha7>` with `npm publish --tag next --provenance --access public` by OIDC under the existing trusted publisher; no NPM_TOKEN; `prerelease.yml` deleted. The npm setup (Node 24, registry-url, npm 11.5.1+) moved into `.github/actions/npm-publisher`, called by both jobs; `publish` loads it from the workflow's own revision (sparse checkout at github.sha) since the tested commit may predate it. Dispatches use their own concurrency group. `scripts/release-workflow.test.mjs` (6 tests) holds triggers, the two jobs, each guard to its own event, the separate group, and no `latest`; three mutations turn it red. Local dry run of the job's extracted run blocks: rc 0, 20 files equal to the package job's. Docs and spec R1/R2 amended.

stage: impl-review - ran (codex, NEEDS_WORK on concurrency and the action's source, then SHIP)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 2d254167a274f8ed39881d63de603f472bae47de, 242d8e5edec7e0e3bb2fadb1a748119cd76e987d, f4fe0d44fdce7791b712ddeef70eb13c3a9c0b78, d345aa108eacad68d675e06a62afdb359f6b27e4
- Tests: actionlint 1.7.12 + shellcheck 0.11.0 (rc 0), npx vitest run scripts/release-workflow.test.mjs (6/6), local dry run of release.yml prerelease run blocks (rc 0, tarball list = package job)
- PRs: