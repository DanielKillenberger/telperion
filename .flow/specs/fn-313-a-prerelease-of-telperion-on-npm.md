# A prerelease of telperion on npm, published when the owner asks

## Goal & Context

Owner (2026-10-24): "can we do like a nightly release of telperion on npm or smth where i could test the package in killenberger.com before release?", and then: "Only when i ask we should release nightly i think?".

## Requirements

- **R1:** a GitHub Actions workflow with `workflow_dispatch` only, never on push or on a schedule.
  - **Inputs:** a ref, the release branch by default, and `dry_run`.
  - **Steps:** check out the ref, then build the package exactly as CI's package job does, on the pinned toolchain, with the same artifact-size check. Set the version to `<package.json version>-next.<YYYYMMDD>.<short sha>` in the job only, never committed. Then `npm publish --tag next --provenance`.
  - **Never `latest`:** it never publishes under the `latest` tag. The job fails if the version has no `-next.` suffix.
  - **Dry run:** with `dry_run` the job runs `npm publish --dry-run` and lists the tarball, with no token needed.
- **R2:** the token is a repository secret, `NPM_TOKEN`: an npm automation token the owner creates and adds. The workflow names the secret and fails with a clear message when it is missing. No token is ever printed.
- **R3 (docs):** a short section in `docs/` covers:
  - how the owner triggers a prerelease: `gh workflow run <file> -r <branch>`, or the Actions tab;
  - how killenberger.com installs it: `npm install telperion@next`, or the exact version;
  - how to go back to the released version.
- **R4 (proof):**
  - `actionlint` is clean.
  - A dry run on the release branch succeeds in CI, triggered by the host after the branch is pushed, with the owner's go for the push.
  - The tarball lists the same files as CI's package job.
  - A codex review, dispatched inside telperion-heavy.

## Boundaries

- No real publish until the owner adds the token and asks.
- No change to the release version or to the `latest` tag.
- Joins the release (fn-237).
