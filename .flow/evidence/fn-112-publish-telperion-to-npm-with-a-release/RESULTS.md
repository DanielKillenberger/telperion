# fn-112 results (2026-09-22)

Base: master `3fe67c92` (fn-101, PR #58), in its own worktree with its own
`target/`. Node v26.8.1, npm 11.19.0, wasm-bindgen-cli 0.2.128 at the crate's
pin. Raw logs sit in `raw/` (ignored): `build.log`, `pack-dry-run.json`,
`version-check.log`.

## R1 to R3: the release workflow

`.github/workflows/release.yml` starts on `push` of a tag matching `v*` and
on nothing else. Four jobs:

1. `version`: `${TAG#v}` against `package.json`'s `version`; on a mismatch
   the step prints `tag <tag> names version <x>, package.json names <y>;
   nothing is published` as an error annotation and exits 1. Every other job
   needs it, so a mismatch stops the run before any toolchain is installed.
2. `rust`: the five workspace crates under `.github/actions/nextest` on the
   ci profile, the gate `tests.yml` runs, without its receipts.
3. `core-plan`: the core built and unit-tested without its geometry feature,
   as `tests.yml`'s job of the same name.
4. `publish` (needs all three): the node suite (`npm test`, `npm run
   typecheck`), `npm run build`, `npm pack --dry-run` for the log, then
   `npm publish --provenance --access public` with `id-token: write`. No
   `NODE_AUTH_TOKEN`, no secret: the publish authenticates through the
   trusted publisher the owner configures on npmjs.com for this repository
   and this workflow file.

The version step was run by hand with the step's script extracted from the
YAML (`raw/version-check.log`): `v0.1.0` passes, `v0.2.0` and `v0.1.0-rc1`
fail with both versions named. `actionlint` is not installed on this
machine; the file was parsed with PyYAML (four jobs, the expected `needs`
and `permissions`) and read against `tests.yml`, whose steps the toolchain,
cache, wasm-bindgen and node setup copy.

## R4: the dry-run pack

`npm ci && npm run build` on the clean worktree, 57 s wall, then
`npm pack --dry-run --json`. `telperion-0.1.0.tgz`, 16 entries, 7,093,035
bytes unpacked:

| bytes | path |
|---:|---|
| 1,076 | LICENSE |
| 43,389 | README.md |
| 7,179 | dist/browser/core.d.ts |
| 2,361 | dist/browser/leaf.d.ts |
| 7,840 | dist/browser/presets.generated.d.ts |
| 6,721 | dist/browser/render.d.ts |
| 599 | dist/browser/specimen-wire.d.ts |
| 3,363 | dist/browser/specimen.d.ts |
| 2,690 | dist/field.js |
| 2,136 | dist/field/index.d.ts |
| 2,022 | dist/field/voxelize.d.ts |
| 787 | dist/index.d.ts |
| 345,645 | dist/telperion-field.wasm |
| 6,659,763 | dist/telperion.js |
| 4,526 | dist/voxelize.js |
| 2,938 | package.json |

`field.js`, `voxelize.js` and `telperion-field.wasm` are there with their
declarations `field/index.d.ts` and `field/voxelize.d.ts`; the main entry
`telperion.js` and its `index.d.ts` beside them. Nothing from `src`,
`crates`, `.flow`, `experiments`, `harness` or `tests` is in the tarball.
Three files outside `dist` are in every npm tarball whatever `files` says:
`package.json`, `README.md` and `LICENSE`, which npm always includes. The
README is the registry page and the licence is the MIT grant, so the
listing is `dist` plus the three npm cannot leave out.

## R5: the README

`README.md` gained `## The field package` before `## Architecture`: the
install line, the `telperion/field` and `telperion/field/voxelize` exports,
that `npm run build` puts them in `dist` with the slim Wasm beside
`field.js`, and a link to `docs/field-package.md`.

## R6: awaiting the owner

The first publish needs two things only the owner can do, in this order.
The npm form and its versions are from docs.npmjs.com/trusted-publishers,
read on 2026-09-22.

1. On npmjs.com, signed in as the account that will own `telperion`:
   Packages, the package, Settings, Trusted publishing, then GitHub Actions
   with these fields:
   - Organization or user: the GitHub owner of this repository (the
     `<owner>` in `github.com/<owner>/telperion`)
   - Repository: `telperion`
   - Workflow filename: `release.yml` (the file name only, not the path,
     with its extension)
   - Environment name: empty; the workflow declares no environment
   - Allowed actions: `npm publish`

   Unknown, to settle at the form: the docs open at "your package settings
   on npmjs.com", a page a name not yet on the registry does not have. If
   the form cannot be reached for an unpublished name, the owner publishes
   `0.1.0` once by hand from a built checkout (`npm run build && npm
   publish --access public` with the account's two-factor), configures the
   publisher on the page that then exists, and the workflow's first tag is
   the next version; R6's "first publish through the workflow" then reads
   as that tag. Nothing in this task attempted a publish.
2. On the master commit that carries this change, with `package.json` at
   the version to publish: `git tag v0.1.0 && git push origin v0.1.0`.
   The Release workflow runs the gate, builds, and publishes with
   provenance (the docs say provenance is automatic under a trusted
   publisher; the workflow passes `--provenance` as well, which is
   accepted). A tag on a commit whose gate is red publishes nothing.

The workflow pins Node 24 and upgrades npm to `^11.5.1`, the docs' floor
for trusted publishing (npm 11.5.1, Node 22.14.0).

After the run is green, the fresh-install half of R6: a directory with no
Rust toolchain, `npm init -y && npm install telperion`, then the Node smoke
from `docs/field-package.md` (grow a species, `grid` a bounds, query it,
read `flags`, `woodRadius`, `leaves` and `limbs`), recorded here with the
Node version and the installed package version.

R6 is not satisfied by this task and is not claimed.
