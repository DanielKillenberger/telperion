# A prerelease of telperion on npm, published when the owner asks

## Goal & Context

Owner (2026-10-24): "can we do like a nightly release of telperion on npm or smth where i could test the package in killenberger.com before release?", and then: "Only when i ask we should release nightly i think?".

The released package is published by `release.yml` from a green master Tests run, through npm trusted publishing bound to that workflow file. A prerelease needs a second workflow, which cannot share that binding, so it authenticates with a token (R2).

## Acceptance Criteria

- **R1:** a GitHub Actions workflow with `workflow_dispatch` only, never on push or on a schedule.
  - **Inputs:** a ref, the release branch by default, and `dry_run`.
  - **Steps:** check out the ref, then build the package exactly as CI's package job does, on the pinned toolchain, with the same artifact-size check. Set the version to `<package.json version>-next.<YYYYMMDD>.<short sha>` in the job only, never committed. Then `npm publish --tag next --provenance`.
  - **Never `latest`:** it never publishes under the `latest` tag. The job fails if the version has no `-next.` suffix.
  - **Dry run:** with `dry_run` the job runs `npm publish --dry-run` and lists the tarball, with no token needed.
  - Errors: a ref that does not resolve fails the checkout; a build or size-ceiling failure fails the job before any version is set; a version that is not valid semver (an all-digit short sha with a leading zero) fails at `npm version` with npm's message; a version without `-next.` or a tag other than `next` fails before publish; a version already on npm (a rerun at the same commit on the same day) fails with a message naming it.
- **R2:** the token is a repository secret, `NPM_TOKEN`: an npm token the owner creates and adds. The workflow names the secret and fails with a clear message when it is missing. No token is ever printed.
  - npm removed classic automation tokens in November 2025 (plan review, 2026-10-24), so the token is a granular access token with read and write on `telperion`, with "bypass two-factor authentication" set when the package or account requires 2FA for publishing.
  - The workflow authenticates through `actions/setup-node` with `registry-url`, whose `.npmrc` reads `NODE_AUTH_TOKEN`; the secret reaches only the publish step's `NODE_AUTH_TOKEN`.
  - Errors: a missing or empty secret on a real publish fails before `npm publish` with a message naming `NPM_TOKEN` and the command that sets it; a dry run never reads it.
- **R3 (docs):** a short section in `docs/` covers:
  - how the owner triggers a prerelease: `gh workflow run <file> -r <branch>`, or the Actions tab;
  - how killenberger.com installs it: `npm install telperion@next`, or the exact version;
  - how to go back to the released version.
  - Errors: no error surface; the section also states the token setup and that `next` can trail `latest` when the branch's version is already released.
- **R4 (proof):**
  - `actionlint` is clean.
  - A dry run on the release branch succeeds in CI, triggered by the host after the branch is pushed, with the owner's go for the push.
  - The tarball lists the same files as CI's package job.
  - A codex review, dispatched inside telperion-heavy.
  - Errors: the local proof is a dry run of the same steps on the owner's machine; the CI dry run is the host's step after the push and is recorded as pending until it ran.
  - **Prerequisite:** GitHub triggers `workflow_dispatch` only for a workflow file that exists on the default branch (`master`), whose copy today holds only `tests.yml` and `release.yml`. The CI dry run therefore waits until `prerelease.yml` is on `master`: when the release merges, or earlier if the owner lands the workflow on `master` by itself. Which of the two is the owner's call, put to them by the host; this spec's build pushes nothing.

## Early proof point

Task fn-313-a-prerelease-of-telperion-on-npm.1 validates the core approach (the package job's build steps live in one composite action that the Tests workflow calls unchanged in effect). If it fails, re-evaluate sharing by composite action (a reusable workflow, or the prerelease calling the package job through `workflow_call`) before continuing with fn-313-a-prerelease-of-telperion-on-npm.2.

## Boundaries

- No real publish until the owner adds the token and asks. No push from this spec's build; the host pushes on the owner's word.
- No change to the release version, to `release.yml`, or to the `latest` tag.
- The package job keeps its job id, name and artifact name `package`, which `release.yml` reads.
- Joins the release (fn-237).

## Strategy Alignment

Active tracks served by this plan:
- **The core and integration** — consumers integrate the package; a prerelease lets the owner test it in a real consumer (killenberger.com) before the release.

## Decision Context

- **Sharing:** the build steps from the toolchain to the size check move into a composite action both workflows call; a reusable workflow was rejected because the prerelease needs the built tree in its own job to set the version and pack, and a reusable workflow hands back only artifacts.
- **Version set after the build:** the version is changed after the build and the size check, right before `npm pack`, so the built files are byte for byte what the package job builds and only `package.json` in the tarball differs.
- **Default ref:** the workflow checks out its `ref` input when given, else the ref it was dispatched on (`-r <branch>`); dispatching on the release branch is the default the docs give, so no branch name is written into the workflow.
- **Dry run:** the dry run packs and runs the same `npm publish` command with `--dry-run` added, against the packed tarball, so it shows the file list that would ship.
- **Token versus trusted publishing:** npm's trusted publisher names `release.yml`; whether npm accepts a second trusted publisher per package is unverified, so the prerelease uses the token the spec names.

## Memory findings

| Track | Category | Entry | Why relevant |
|---|---|---|---|
| bug | integration | a-ci-size-checks-skip-path-must-be-the-2026-09-27 | the size check's skip path on a non-PR run is the sanctioned one; the prerelease run relies on it |

## Quick commands

```bash
actionlint .github/workflows/*.yml
npm run build && node scripts/artifact-budgets.mjs && npm pack --dry-run --json | jq -r '.[0].files[].path'
```

## Requirement coverage

| Req | Description | Task(s) | Gap justification |
| --- | --- | --- | --- |
| R1 | a GitHub Actions workflow with `workflow_dispatch` only, never on push or on a schedule. | fn-313-a-prerelease-of-telperion-on-npm.1, fn-313-a-prerelease-of-telperion-on-npm.2 | — |
| R2 | the token is a repository secret, `NPM_TOKEN`: an npm token the owner creates and adds. The workflow names the secret and fails with a clear message when it is missing. No token is ever printed. | fn-313-a-prerelease-of-telperion-on-npm.2 | — |
| R3 | (docs): a short section in `docs/` covers: | fn-313-a-prerelease-of-telperion-on-npm.2 | — |
| R4 | (proof): | fn-313-a-prerelease-of-telperion-on-npm.2 | — |

