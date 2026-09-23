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

## R7: the main entry's Wasm as files

Task 2, on this branch at `2643188d`. `dist/telperion.js` was 6,659,763
bytes because Vite's library build inlines every asset a `?url` import
names as a base64 data URL, whatever `assetsInlineLimit` says: the full
generator Wasm and the renderer's Wasm both went in as text. The two
entries now hold the file name in a constant and resolve `new URL(WASM,
import.meta.url)` at run time, the way `src/field/index.ts` resolves
`telperion-field.wasm`, and the build scripts copy each module beside its
entry: `telperion.wasm` and `telperion-render.wasm` in `src/browser/` for
the dev server and in `dist/` for the package. The wasm-bindgen glue is
generated with `--omit-default-module-path`, because its own default
`new URL('telperion_render_bg.wasm', import.meta.url)` is a string literal
Vite inlined too (2.4 MB of base64 in a branch the entry never takes).
The `dist/browser/*.d.ts`, `index.d.ts` and `field/*.d.ts` declarations are
byte-identical before and after (md5 checked).

`npm pack --dry-run --json` after `npm run build` on a fresh `dist`
(`raw/r8-pack-dry-run.json`, the second round; the first round's listing
is `raw/r7-pack-dry-run.json`): `telperion-0.1.0.tgz`, 20 entries,
3,643,702 bytes unpacked, 1,152,721 packed, against 16 entries and
7,093,035 bytes before R7.

| bytes | path | before R7 |
|---:|---|---:|
| 108,486 | dist/telperion.js | 6,659,763 |
| 1,286,823 | dist/telperion.wasm | inlined |
| 1,813,306 | dist/telperion-render.wasm | inlined |
| 345,645 | dist/telperion-field.wasm | 345,645 |
| 2,720 | dist/field.js | 2,690 |
| 4,526 | dist/voxelize.js | 4,526 |
| 235 | dist/wasm-source.js | new, the loader both entries share |
| 78 | dist/wasm-source.d.ts | new |
| 44,530 | README.md | 43,389 |
| 3,269 | package.json | 2,938 |

The declarations, `LICENSE` and `package.json` are the same bytes as the
R4 listing. The main entry's JavaScript gzips to 22.7 kB; the two modules
are served beside it and fetched once, cacheable as files.

Checks run, logs in `raw/r7-*.log`:

- `npm run typecheck`, `npx vitest run`: 11 files, 123 tests, green before
  and after the change.
- `npm run rust:test:wasm`, the binding and field browser suites on the
  dev server: green; `bindings.mjs` calls `TreeEngine.create()` with no
  source, which is the dev-mode resolution of `telperion.wasm` beside
  `core.ts`.
- A static server over `dist/` alone, loaded in headless Chromium
  (`raw/r7-dist-check.log`): `telperion.js` fetched `/telperion.wasm` and
  `/telperion-render.wasm` beside itself, both 200, built Ordinary to 9,240
  nodes, and `createRenderer` failed only with the renderer's own "no
  hardware GPU adapter" words, after its module had loaded; headless
  Chromium offers no hardware adapter, so the draw is judged below.
- The harness, `npm run test:render` on the dev server with the RTX 3080
  (`raw/r7-render-suite.log`): the page built and drew all six presets,
  the height dial, the three views and the timing sessions, so the harness
  still draws a tree. The run stopped at the Oregon white oak orbit's
  wall-clock p95, 30.00 ms against the 16.7 ms threshold, with the GPU
  percentiles valid (p50 12.7 ms, p95 14.5 ms). Another harness was on
  port 5173 and the display was in use during the run; a page's animation
  clock on a contended display is what that assertion measures, and Wasm
  loading does not run in the frame loop. Recorded as inconclusive on that
  one assertion, not as green.

### R7, second round: the literal survives the build and Node reads a file

The review found two defects in the first round (`merged.md`, findings 1
and 2). The filename held in a constant kept our own Vite library build from
inlining the Wasm, but a consumer's bundler recognises an asset only as the
literal `new URL("./name.wasm", import.meta.url)`, so a site that bundled
`telperion.js` would have moved the script and left the Wasm behind. And in
Node `import.meta.url` is a `file:` URL that `fetch` refuses, so the
README's `TreeEngine.create()` with no source failed.

The fix keeps Vite for the bundle and adds one plugin in `vite.config.ts`,
`telperion:wasm-beside-entry`. Vite 6's `shouldInline` returns true for
every asset in library mode before `assetsInlineLimit` is read (checked in
`node_modules/vite/dist/node/chunks`, `shouldInline`), and its own rewrite
of a `new URL` literal emits `/* @vite-ignore */` into the chunk, which
would blind a consumer's Vite. The plugin marks the literal ignored before
Vite's asset pass and unmarks it in the emitted chunk, so `dist` carries the
plain literal: `dist/telperion.js` lines 1734 and 3361 and `dist/field.js`
line 8 hold `new URL("./telperion.wasm", import.meta.url)`,
`"./telperion-render.wasm"` and `"./telperion-field.wasm"`. `package.json`
exports the three files as `telperion/telperion.wasm`,
`telperion/telperion-render.wasm` and `telperion/telperion-field.wasm` for a
bundler's `?url` import. `src/wasm-source.ts` is the loader the main and the
field entries share (emitted as `dist/wasm-source.js`): a `file:` URL is
read through `node:fs/promises` behind a computed `import()` marked
`@vite-ignore` and `webpackIgnore`, anything else is fetched. The renderer
needs a browser and keeps the plain URL.

Checks, logs in `raw/r8-*.log` and `raw/consumer/`:

- `node scripts/test-dist.mjs`, new, run by the release workflow after the
  build: `dist/telperion.js` and `dist/field.js` imported in Node v26.8.1
  with no source given built Telperion to 16,993 nodes and grew silver-birch
  (wood radius 0.252 m), both from the Wasm beside them. `src/field/field.test.ts`
  covers the same path from the source in vitest (124 tests, was 123).
- The consumer proof, `raw/consumer/site` (ignored): `npm pack` of this
  checkout, `npm init -y`, `npm install ../telperion-0.1.0.tgz vite` (Vite
  8.3.0 resolved), an `index.html` and a `main.js` importing `telperion`,
  `telperion/field` and `telperion/telperion-field.wasm?url`, then `vite
  build` (`raw/consumer/consumer-build.log`):

  ```
  dist/index.html                                 0.16 kB
  dist/assets/telperion-field-C452MQ6d.wasm     345.64 kB
  dist/assets/telperion-DAWlg5Qe.wasm         1,286.82 kB
  dist/assets/telperion-render-BMqsBoOW.wasm  1,813.30 kB
  dist/assets/index-BJkd4fLc.js                  75.30 kB
  ```

  The consumer's `dist` served by `vite preview` and loaded in headless
  Chromium (`raw/consumer/consumer-check.log`): fetched `200
  /assets/telperion-DAWlg5Qe.wasm`, `200 /assets/telperion-field-C452MQ6d.wasm`
  (twice: once from beside the script, once through the `?url` import
  passed as `source`), `200 /assets/telperion-render-BMqsBoOW.wasm`; the
  page's console: `REPORT {"nodes":16993,"field":[3],"explicitUrl":
  "/assets/telperion-field-C452MQ6d.wasm","explicitBounds":true,"render":
  "Error: WebGPU is unavailable: No suitable graphics adapter found ..."}`.
  Both entries built a tree from the emitted assets; the renderer's module
  was fetched and failed only at the adapter, which headless Chromium does
  not offer.
- `npm run typecheck` and `npx vitest run` (11 files, 124 tests) green;
  `node scripts/test-wasm.mjs`, the binding and field suites on the dev
  server, green (`raw/r8-bindings.log`); the gate `cargo test --profile ci
  --workspace --no-fail-fast`, 122 suites, every one `ok`, exit 0
  (`raw/r8-cargo-ci.log`).

## R5: the README

`README.md` gained `## The field package` before `## Architecture`: the
install line, the `telperion/field` and `telperion/field/voxelize` exports,
that `npm run build` puts them in `dist` with the slim Wasm beside
`field.js`, and a link to `docs/field-package.md`.

## R6: published by the workflow (2026-09-23)

npm attaches a trusted publisher only to a package that already exists. The
owner chose a placeholder over a hand publish of 0.1.0 (spec, R6 as amended
2026-09-23), so the workflow published 0.1.0 itself. In order:

1. The owner published `telperion@0.0.1` by hand to create the package. It
   holds only a `package.json`, 256 B, and is to be deprecated.
2. The owner attached the trusted publisher on npmjs.com for
   `DanielKillenberger/telperion`, `release.yml`, no environment, with
   direct `npm publish` allowed.
3. The Release run on tag `v0.1.0` (`dd86fa05`), run 35791697498, had
   failed only at its publish step on 2026-09-22, because the package did
   not exist. `gh run rerun --failed` re-ran the publish job (attempt 2).
   Its gate, build and `npm publish --provenance --access public` passed
   with no token. npm signed the provenance statement and logged it to
   Sigstore at log index 2919901357. The registry lists `0.0.1` and
   `0.1.0`, `latest` is `0.1.0`, and 0.1.0 carries an SLSA v1 provenance
   attestation. The tarball has 20 files, 1.2 MB packed and 3.7 MB
   unpacked. Its main entry is 109.9 kB of JavaScript (R7), with
   `telperion.wasm` at 1.3 MB and `telperion-render.wasm` at 1.8 MB beside
   it.
4. Fresh install, run in an empty directory with Node v26.8.1. `PATH` held
   only Node and `/usr/bin:/bin`, with no `cargo` or `rustc`. `npm install
   telperion@0.1.0` added 1 package in 654 ms, and `npm audit signatures`
   reports 1 verified attestation. The smoke, `r6-smoke.mjs` beside this
   file, grows `silver-birch` at seed 7, queries an 8×8×8 grid of cubes
   over its bounds and reads the four answers. It took 0.5 s wall:

   ```
   bounds  min [-6.62, -0.25, -7.64]  max [7.50, 14.74, 5.98]
   cells 512  wood 380  foliage 385  maxWoodRadius 0.252
   leaves 341182.5  limbSystems 76
   ```

   Each cube's half extent is half the largest axis step, so neighbouring
   cubes overlap. The counts show that every answer is populated; they are
   not a density.

The first `npm install` answered `notarget`, because the local npm cache
still held the package listing from before 0.1.0. `--prefer-online`
fetched the fresh listing.

`package.json` carries `repository.url`
`git+https://github.com/DanielKillenberger/telperion.git`, which npm's
trusted publishing requires to match the workflow's repository. The
workflow pins Node 24 and upgrades npm to `^11.5.1`, the docs' floor for
trusted publishing (npm 11.5.1, Node 22.14.0).
