---
satisfies: [R1, R2, R3, R4]
---
# fn-313-a-prerelease-of-telperion-on-npm.2 A workflow_dispatch prerelease under the next tag, its docs and the local dry run

## Description
Adds the prerelease workflow on the shared build, the docs section, and the local proof: a dry run of the same steps and the tarball's file list against CI's package job.

**Size:** M
**Files:** `.github/workflows/prerelease.yml`, `docs/prerelease.md`, `.flow/evidence/fn-313-a-prerelease-of-telperion-on-npm/`
**Touches:** [.github/workflows/prerelease.yml, docs/prerelease.md, .flow/evidence/fn-313-a-prerelease-of-telperion-on-npm/**]

### Approach
- `on: workflow_dispatch` only, input `dry_run` (boolean) only; the run builds the dispatched commit (host ruling, 2026-10-24: the `ref` input was dropped). `permissions: contents: read, id-token: write` (provenance). `concurrency: {group: prerelease, cancel-in-progress: false}`, apart from `release`. Guard the job with `if: github.repository == 'DanielKillenberger/telperion'`.
- Steps: `actions/checkout@v7` at the dispatched commit; `./.github/actions/package`; then set the version: base from `package.json`, date `date -u +%Y%m%d`, `git rev-parse --short=7 HEAD`, `npm version --no-git-tag-version "$version"`; guard `-next.` in the version and that it is not on npm (`npm view telperion@$version version`); `npm pack --json`; publish the packed tarball with `npm publish --tag next --provenance --access public "./$tarball"`, adding `--dry-run` when `dry_run`, and listing the tarball's files.
- Untrusted input: pass the version and the tarball through `env:`, never `${{ }}` inside `run:`.
- Token (R2): a second `actions/setup-node@v7` step in the prerelease job with `node-version-file: package.json` and `registry-url: https://registry.npmjs.org`, as `release.yml:145-150` does; its `.npmrc` reads `NODE_AUTH_TOKEN`. Set `NODE_AUTH_TOKEN: ${{ secrets.NPM_TOKEN }}` on the publish step only; on a real publish fail first with `::error::` naming `NPM_TOKEN` and `gh secret set NPM_TOKEN` when it is empty. `--registry` alone is not authentication. Never `set -x`, never echo it.
- The tag is the literal `next` in one place; no step names `latest`.
- Docs: `docs/prerelease.md` per R3, plus the token setup (a granular access token with read and write on `telperion`, bypass 2FA set when publishing requires 2FA, then `gh secret set NPM_TOKEN`), the prerequisite that the workflow is on `master` before it can be dispatched, the version format, that `next` can trail `latest`, and going back (`npm install telperion@latest` or the exact version; `npm dist-tag rm telperion next` optional).
- Local proof, in a scratch copy of the worktree (never the worktree itself): `npm ci`, `npm run build` (through telperion-heavy), `node scripts/artifact-budgets.mjs`, the version step, `npm pack --json`, `npm publish --dry-run --tag next ./<tarball>`; compare the file list with `npm pack --dry-run --json` at the unchanged version (the package job's tarball). Summary in `.flow/evidence/fn-313-a-prerelease-of-telperion-on-npm/dry-run.md`; raw output under `raw/`.

### Investigation targets
**Required**:
- `.github/workflows/release.yml:1-60,150-171` — header style, setup-node with registry-url, publish line
- `.github/workflows/tests.yml:344-369` — pack step to mirror
- `docs/field-package.md` — sibling doc tone

**Optional**:
- `docs/evidence-retention.md` — where raw output goes

### Key context
npm tags a prerelease `latest` when no `--tag` is given, so `--tag next` is mandatory on every publish line. GitHub dispatches a workflow only when its file is on the default branch (spec R4, Prerequisite); the CI dry run is the host's after that, never this task's.

## Acceptance
- [ ] `.github/workflows/prerelease.yml` has `workflow_dispatch` as its only trigger, input `dry_run` only, builds the dispatched commit, and builds through `./.github/actions/package`
- [ ] The version is `<package.json version>-next.<YYYYMMDD>.<short sha>`, set in the job only; a version without `-next.` or already on npm fails before publish
- [ ] Every publish passes `--tag next`; no step names `latest`
- [ ] A real publish without `NPM_TOKEN` fails with a message naming it; the token is only in `NODE_AUTH_TOKEN` on the publish step
- [ ] `docs/prerelease.md` covers trigger, install, going back and token setup
- [ ] actionlint is clean
- [ ] Local dry run in a scratch copy succeeds; its tarball file list equals the package job's; recorded in the evidence summary

## Done summary
`.github/workflows/prerelease.yml`: workflow_dispatch only, one input `dry_run`; builds the dispatched commit through `./.github/actions/package`, a second setup-node with `registry-url` for auth, sets `<version>-next.<YYYYMMDD>.<sha7>` in the job (refusing when npm rewrites it), refuses no `-next.`, a tag other than `next`, a version already on npm and a real publish without NPM_TOKEN, packs and publishes the tarball with `--tag next --provenance --access public` (`--dry-run` when asked). `docs/prerelease.md` covers trigger, install, going back and token setup. Local dry run in a scratch copy: rc 0, 20 files equal to the package job's tarball (evidence dry-run.md). RESULT.md records that a real dry run depends on fn-314 (size-check entry dist/wasm-source.js) and on the workflow reaching master.

Round 1 review NEEDS_WORK (rid e2d5af07): the inherited size-check failure (host ruling: fn-314's) and the ref input versus provenance (host ruling: ref input dropped, R1 amended).

stage: impl-review - ran (codex, NEEDS_WORK then SHIP on re-review)

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 4674d7970bc1e8bb6119c5128fb2e3f4b7eb03ac, 00de90204aa74de6d2faf6f18c3960841f8ecead, 788b4c82685117eb91f80d0e5c6c79049b968dee
- Tests: actionlint 1.7.12 + shellcheck 0.11.0 (rc 0), local scratch dry run: npm publish --dry-run --tag next (rc 0), tarball list = package job
- PRs: