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
TBD

## Evidence
- Commits:
- Tests:
- PRs:
