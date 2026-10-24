# A prerelease of telperion on npm, published when the owner asks

## Goal & Context

Owner (2026-10-24): "can we do like a nightly release of telperion on npm or smth where i could test the package in killenberger.com before release?", and then: "Only when i ask we should release nightly i think?".

The released package is published by `release.yml` from a green master Tests run, through npm trusted publishing bound to that workflow file. The prerelease is a second job of that same workflow, so it publishes through the same trusted publisher with no stored token (R2; host ruling, 2026-10-24).

## Acceptance Criteria

- **R1:** a `prerelease` job in `release.yml`, started only by a `workflow_dispatch` of that workflow, never on push or on a schedule (host ruling, 2026-10-24, replacing the separate `prerelease.yml`: only `release.yml` is the trusted publisher). The existing `publish` job keeps its behaviour: it needs a `workflow_run` event, so a dispatch never starts it, and a test proves each job's guard admits only its own event.
  - **Inputs:** `dry_run` only. The run builds the commit it was dispatched on (`gh workflow run release.yml -r <branch>`, the release branch as a rule), so the provenance always names what was built. Amended by the host, 2026-10-24: the original `ref` input was dropped, because a ref other than the dispatched commit would be built while the provenance named the dispatched one (task 2's codex review).
  - **Steps:** check out the dispatched commit, then build the package exactly as CI's package job does, on the pinned toolchain, with the same artifact-size check. Set the version to `<package.json version>-next.<YYYYMMDD>.<short sha>` in the job only, never committed. Then `npm publish --tag next --provenance --access public`, authenticated by OIDC as the `publish` job is (npm 11.5.1 or later, Node 22.14 or later), through the same setup steps, shared rather than copied.
  - **Never `latest`:** it never publishes under the `latest` tag. The job fails if the version has no `-next.` suffix.
  - **Dry run:** with `dry_run` the job runs `npm publish --dry-run` and lists the tarball.
  - Errors: a branch that does not exist fails at `gh workflow run`; a build or size-ceiling failure fails the job before any version is set; a version that is not valid semver (an all-digit short sha with a leading zero) fails at the version step, which compares the version npm wrote with the one it asked for (npm 11.19 drops the zero and exits 0, measured in task 2); a version without `-next.` or a tag other than `next` fails before publish; a version already on npm (a rerun at the same commit on the same day) fails with a message naming it.
- **R2:** no npm token (host ruling, 2026-10-24, replacing the `NPM_TOKEN` secret this criterion first named). The job authenticates by its OIDC token (`id-token: write`) under the trusted publisher configured on npmjs.com for this repository and `release.yml`; no secret is stored or read.
  - Errors: a run whose OIDC exchange npm refuses fails at `npm publish` with npm's message; nothing else reaches the registry.
- **R3 (docs):** a short section in `docs/` covers:
  - how the owner triggers a prerelease: `gh workflow run release.yml -r <branch>` (`-f dry_run=true` for a dry run), or the Actions tab;
  - how killenberger.com installs it: `npm install telperion@next`, or the exact version;
  - how to go back to the released version.
  - Errors: no error surface; the section also states that no secret is needed and that `next` can trail `latest` when the branch's version is already released.
- **R4 (proof):**
  - `actionlint` is clean.
  - A dry run on the release branch succeeds in CI, triggered by the host after the branch is pushed, with the owner's go for the push.
  - The tarball lists the same files as CI's package job.
  - A codex review, dispatched inside telperion-heavy.
  - Errors: the local proof is a dry run of the same steps on the owner's machine; the CI dry run is the host's step after the push and is recorded as pending until it ran.
  - **Prerequisite:** GitHub triggers `workflow_dispatch` only for a workflow file that exists on the default branch (`master`), whose copy today holds only `tests.yml` and `release.yml`. The CI dry run therefore waits until `release.yml` with its dispatch trigger is on `master`; the host puts the matching change on `master` (host ruling, 2026-10-24). This spec's build pushes nothing. A real dry run also depends on fn-314, which fixes the size check's `dist/wasm-source.js` entry.

## Early proof point

Task fn-313-a-prerelease-of-telperion-on-npm.1 validates the core approach (the package job's build steps live in one composite action that the Tests workflow calls unchanged in effect). If it fails, re-evaluate sharing by composite action (a reusable workflow, or the prerelease calling the package job through `workflow_call`) before continuing with fn-313-a-prerelease-of-telperion-on-npm.2.

## Boundaries

- No real publish until the owner asks. No push from this spec's build; the host pushes on the owner's word.
- No change to the release version, to the `latest` tag, or to the `publish` job's behaviour; `release.yml` gains only the dispatch trigger and the `prerelease` job, and its npm setup steps move into a shared action both jobs call.
- The package job keeps its job id, name and artifact name `package`, which `release.yml` reads.
- Joins the release (fn-237).

## Strategy Alignment

Active tracks served by this plan:
- **The core and integration** — consumers integrate the package; a prerelease lets the owner test it in a real consumer (killenberger.com) before the release.

## Decision Context

- **Sharing:** the build steps from the toolchain to the size check move into a composite action both workflows call, and the npm setup for trusted publishing into another that both `release.yml` jobs call; a reusable workflow was rejected because the prerelease needs the built tree in its own job to set the version and pack, and a reusable workflow hands back only artifacts.
- **Version set after the build:** the version is changed after the build and the size check, right before `npm pack`, so the built files are byte for byte what the package job builds and only `package.json` in the tarball differs.
- **Default ref:** the workflow checks out the commit it was dispatched on (`-r <branch>`), with no `ref` input (host ruling, 2026-10-24); dispatching on the release branch is what the docs give, so no branch name is written into the workflow.
- **Dry run:** the dry run packs and runs the same `npm publish` command with `--dry-run` added, against the packed tarball, so it shows the file list that would ship.
- **Trusted publishing, not a token:** npm's trusted publisher names `release.yml`, so the prerelease is a job of `release.yml` and publishes by OIDC with no stored token (host ruling, 2026-10-24; superseded the `NPM_TOKEN` design and the separate `prerelease.yml`).

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
| R1 | a `prerelease` job in `release.yml`, started only by a `workflow_dispatch` of that workflow, never on push or on a schedule (host ruling, 2026-10-24, replacing the separate `prerelease.yml`: only `release.yml` is the trusted publisher). The existing `publish` job keeps its behaviour: it needs a `workflow_run` event, so a dispatch never starts it, and a test proves each job's guard admits only its own event. | fn-313-a-prerelease-of-telperion-on-npm.1, fn-313-a-prerelease-of-telperion-on-npm.2, fn-313-a-prerelease-of-telperion-on-npm.3 | — |
| R2 | no npm token (host ruling, 2026-10-24, replacing the `NPM_TOKEN` secret this criterion first named). The job authenticates by its OIDC token (`id-token: write`) under the trusted publisher configured on npmjs.com for this repository and `release.yml`; no secret is stored or read. | fn-313-a-prerelease-of-telperion-on-npm.2, fn-313-a-prerelease-of-telperion-on-npm.3 | — |
| R3 | (docs): a short section in `docs/` covers: | fn-313-a-prerelease-of-telperion-on-npm.2, fn-313-a-prerelease-of-telperion-on-npm.3 | — |
| R4 | (proof): | fn-313-a-prerelease-of-telperion-on-npm.2, fn-313-a-prerelease-of-telperion-on-npm.3 | — |
