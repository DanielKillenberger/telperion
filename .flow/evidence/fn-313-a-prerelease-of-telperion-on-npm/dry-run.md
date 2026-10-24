# fn-313 local dry run (task 2, 2026-10-24)

R4's local proof: the prerelease workflow's own step scripts, run on the
owner's machine at `dc4261238` in a scratch copy of the package, with a dry
run of the publish. The CI dry run is the host's after `prerelease.yml` is on
`master`, and is pending.

## How it ran

- Build in the worktree: `telperion-heavy 5 --threads 2 -- npm run build`,
  rc 0 in 53 s (`raw/build.log`).
- Size check: `node scripts/artifact-budgets.mjs`, **rc 1, inherited**
  (`raw/budgets.log`): every ceiling holds, but
  `FAIL dist/wasm-source.js: not built by the recipe`. Vite emits the shared
  chunk as `dist/module.js`; the budget file still names
  `dist/wasm-source.js`. The release branch's Tests run 40087469094 fails its
  package job on the same line, so this predates fn-313. In CI it fails the
  prerelease build before any version is set, so the CI dry run cannot pass
  until the budget entry is fixed. See FRICTION.md.
- The package job's tarball at the unchanged version: `npm pack --dry-run
  --json` in the worktree (`raw/pack-package-job.json`), 20 files.
- Scratch copy (session scratchpad `prerelease-dryrun/pkg`): `package.json`,
  `README.md`, `LICENSE` and `dist/` copied from the worktree. The four `run:`
  blocks of `prerelease.yml` were extracted from the YAML and run with
  `bash --noprofile --norc -eo pipefail`, as Actions runs them, with
  `DIST_TAG=next`, an empty user `.npmrc` (no token in scope) and a stub
  `npm` that refuses any `publish` without `--dry-run`.
- `npm` 11.19.0, Node 26.8.1. The worktree's `package.json` was never changed;
  the build left no tracked or new files.

## Results

| Step | Result |
|---|---|
| Set the version | `0.2.0-next.20261024.dc42612` |
| Guards | pass |
| Pack | `telperion-0.2.0-next.20261024.dc42612.tgz`, 20 files |
| `npm publish --tag next --provenance --access public --dry-run` | rc 0, "Publishing to https://registry.npmjs.org/ with tag next and public access (dry-run)" (`raw/dry-run-steps.log`) |

Provenance is not exercised outside Actions: the dry run accepted
`--provenance` and generated no attestation.

**File list:** the prerelease tarball's 20 paths equal the package job's
(`raw/files-package-job.txt`, `raw/files-prerelease.txt`, `diff` empty).
Every file's size is equal except `package.json` (3,146 to 3,168 bytes); the
unpacked `dist/` is byte-identical to the worktree's (`diff -r`), and
`package.json` differs only in `version`.

## Guards, run as a user meets them (`raw/guards.log`)

| Case | Outcome |
|---|---|
| package.json at `0.2.0` (no `-next.`) | guard step rc 1: `::error::version 0.2.0 has no -next. suffix; a prerelease never publishes it` |
| `DIST_TAG=latest` | guard step rc 1: `::error::dist-tag latest is not next; a prerelease publishes under next only` |
| version already on npm (versions list stubbed) | guard step rc 1: `::error::telperion@0.2.0-next.20261024.dc42612 is already on npm; …` |
| real publish, empty `NODE_AUTH_TOKEN` | publish step rc 1 before `npm publish`: `::error::the repository secret NPM_TOKEN is missing or empty; add an npm granular access token with gh secret set NPM_TOKEN` |
| short sha `0123456` (git stubbed) | version step rc 1: `::error::npm wrote version 0.2.0-next.20261024.123456 for 0.2.0-next.20261024.0123456, which is not valid semver; nothing is published` |

The last row is a measured correction to the spec: npm 11.19 does not refuse
`0.2.0-next.20261024.0123456`, it writes `…123456` and exits 0 (first entry
for that case in `raw/guards.log`), which would publish a version naming
another commit. The version step now compares what npm wrote with what it
asked for and fails on a difference.
