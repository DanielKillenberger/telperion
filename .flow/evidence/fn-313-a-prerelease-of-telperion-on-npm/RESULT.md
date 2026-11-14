# fn-313 result: a prerelease of telperion on npm, published when the owner asks

## What was built

- `.github/actions/package/action.yml`: the package job's build, from the pinned toolchain to the artifact-size check, as one composite action. The Tests workflow's package job calls it; checkout, pack, test:dist and the `package` upload stay in the job.
- `.github/actions/npm-publisher/action.yml`: Node 24 against the npm registry and npm 11.5.1 or later, the trusted-publishing setup the release's `publish` job ran inline; both `release.yml` jobs call it.
- `.github/workflows/release.yml`: a `workflow_dispatch` trigger (input `dry_run` only) and a `prerelease` job guarded by `github.event_name == 'workflow_dispatch'`. It builds the dispatched commit through the shared package action, sets `<package.json version>-next.<YYYYMMDD>.<short sha>` in the job only, refuses a version npm rewrote, a version without `-next.`, a tag other than `next` and a version already on npm, packs, and runs `npm publish --tag next --provenance --access public` on the tarball, with `--dry-run` when asked. It authenticates by OIDC under the trusted publisher, with no token. The `publish` job's behaviour is unchanged: it still needs a successful `workflow_run` from a push.
- `scripts/release-workflow.test.mjs`: checks that `release.yml` runs on those two triggers only, holds only those two jobs, starts `publish` only from a Tests run and `prerelease` only from a dispatch, and never names `latest` as a dist-tag; each of three mutations (guard removed, `||` added, `--tag latest`) turns it red.
- `docs/prerelease.md`: `gh workflow run release.yml -r <branch> -f dry_run=true`, installing `telperion@next`, going back, and that no secret is needed.

## Host rulings (2026-10-24)

- The `ref` input is dropped: the run builds the commit it was dispatched on, so the provenance always names what was built (task 2's codex review found a `ref` other than the dispatched commit would be attested as the dispatched one).
- The `dist/wasm-source.js` budget entry is fn-314's, not this spec's.
- Getting the dispatch trigger onto `master`, which GitHub needs before it can dispatch the workflow, is the host's: the host puts the matching `release.yml` change on `master`.
- No `NPM_TOKEN` (replacing R2's first design): only `release.yml` is npm's trusted publisher, so the prerelease is a job of `release.yml` publishing by OIDC, and the separate `prerelease.yml` is deleted.

## Proof

- actionlint 1.7.12 with shellcheck 0.11.0: clean on every workflow.
- Local dry run (`dry-run.md`): the `prerelease` job's run blocks, extracted from `release.yml`, gave `0.2.0-next.20261024.2d25416` and `npm publish --dry-run --tag next --provenance --access public` rc 0; the tarball's 20 files equal the package job's (`dist/` byte-identical in task 2's run, `package.json` differing only in `version`); each refusal fails with its message.
- **A real dry run depends on fn-314.** The size check fails on the release branch before any version is set (`FAIL dist/wasm-source.js: not built by the recipe`; vite now emits the chunk as `dist/module.js`). It fails CI's package job the same way (Tests run 40087469094), so the CI dry run of R4 cannot pass until fn-314 lands, and it also waits on the dispatch trigger being on `master`.

## For the owner

Nothing to set up: no token or secret. The trusted publisher on npmjs.com already names `release.yml`. Once the dispatch trigger is on `master` and fn-314 has landed, `gh workflow run release.yml -r <branch> -f dry_run=true` is the first run.
