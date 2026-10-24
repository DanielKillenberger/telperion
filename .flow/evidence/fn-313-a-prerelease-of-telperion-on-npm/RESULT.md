# fn-313 result: a prerelease of telperion on npm, published when the owner asks

## What was built

- `.github/actions/package/action.yml`: the package job's build, from the pinned toolchain to the artifact-size check, as one composite action. The Tests workflow's package job calls it; checkout, pack, test:dist and the `package` upload stay in the job.
- `.github/workflows/prerelease.yml`: `workflow_dispatch` only, one input `dry_run`. It builds the dispatched commit (`gh workflow run prerelease.yml -r <branch>`) through the shared action, sets the version `<package.json version>-next.<YYYYMMDD>.<short sha>` in the job only, refuses a version without `-next.`, a tag other than `next`, a version already on npm and a real publish without `NPM_TOKEN`, packs, and runs `npm publish --tag next --provenance --access public` on the tarball, with `--dry-run` when asked.
- `docs/prerelease.md`: triggering, installing `telperion@next`, going back to the released version, and the token setup.

## Host rulings (2026-10-24)

- The `ref` input is dropped: the run builds the commit it was dispatched on, so the provenance always names what was built (task 2's codex review found a `ref` other than the dispatched commit would be attested as the dispatched one).
- The `dist/wasm-source.js` budget entry is fn-314's, not this spec's.
- Getting `prerelease.yml` onto `master`, which GitHub needs before it can dispatch the workflow, is the host's to put to the owner.

## Proof

- actionlint 1.7.12 with shellcheck 0.11.0: clean on every workflow.
- Local dry run (`dry-run.md`): version `0.2.0-next.20261024.dc42612`; `npm publish --dry-run --tag next --provenance` rc 0; the tarball's 20 files equal the package job's, `dist/` byte-identical, `package.json` differing only in `version`; each refusal fails with its message.
- **A real dry run depends on fn-314.** The size check fails on the release branch before any version is set (`FAIL dist/wasm-source.js: not built by the recipe`; vite now emits the chunk as `dist/module.js`). It fails CI's package job the same way (Tests run 40087469094), so the CI dry run of R4 cannot pass until fn-314 lands, and it also waits on the workflow being on `master`.

## For the owner, to enable publishing

1. Create a granular access token on npmjs.com with read and write on `telperion`, with "bypass two-factor authentication" set if publishing requires 2FA.
2. `gh secret set NPM_TOKEN --repo DanielKillenberger/telperion`.
