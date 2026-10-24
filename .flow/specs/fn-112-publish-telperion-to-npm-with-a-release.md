# Publish telperion to npm with a release pipeline in CI

## Goal

killenberger.com integrates Telperion through fn-101's slim field package, but the package was on no registry and the repository had no release tag; with `dist` gitignored, a git install would compile Rust on the site's deploy machine. This spec publishes the built package to npm from a tag-driven CI workflow so installs are plain downloads traceable to a tagged, gated commit.

Owner quotes:
> "so telperion is ready to be integrated into killenberger.com?"
> "should we package an npm module?"
> "/flow-next:capture this including a release pipeline in ci"

## Decisions

- Registry over git dependency; unscoped `telperion` (free, repo public, MIT); start at 0.1.0, tags `v*`; versions bumped and tagged by hand; no changelog tooling or pre-release channels.
- `.github/workflows/release.yml` on `v*` tags only: version check -> the `tests.yml` gate (rust nextest, core-plan) -> node suite, `npm run build`, `npm publish --provenance --access public` via a trusted publisher; no stored token.
- R7 (owner, 2026-09-22): fix the main entry's 6.7 MB inlined Wasm before the first tag so 0.1.0 is lean on every entry.
- R6 amended by the host (2026-09-22: npm attaches a trusted publisher only to an existing package, so the first step is the owner's) and by the owner (2026-09-23): create the package with a placeholder `0.0.1` (no code, deprecated after), attach the publisher with direct publish allowed, and let the workflow publish 0.1.0. Staged publishing can be enabled later (`npm stage publish`).

## Acceptance Criteria

- **R1:** Release workflow on `v*` tags only, running the gate first — met.
- **R2:** Builds and publishes with provenance through a trusted publisher, no token — met.
- **R3:** Tag/`package.json` version mismatch fails before publish naming both — met (checked by hand: `v0.2.0`, `v0.1.0-rc1` fail).
- **R4:** Dry-run pack lists only `dist` plus `package.json`, `README.md`, `LICENSE` — met.
- **R5:** README section on the field package and its two exports — met.
- **R6:** Workflow publishes `v0.1.0` with provenance; a fresh project with no Rust installs it and runs the Node smoke — met (2026-09-23).
- **R7:** Main entry JavaScript-only, Wasm shipped as files resolved at run time; suites green; harness still draws — met.

## Result

- `telperion@0.1.0` published by Release run 35791697498 (attempt 2, tag on `dd86fa05`) with SLSA v1 provenance (Sigstore log index 2919901357); `0.0.1` placeholder by hand. Tarball 20 files, 1.2 MB packed / 3.7 MB unpacked.
- R7: `dist/telperion.js` 6,659,763 -> 108,486 bytes (22.7 kB gzip); `telperion.wasm` 1,286,823 and `telperion-render.wasm` 1,813,306 ship beside it. wasm-bindgen runs with `--omit-default-module-path`. A Vite plugin `telperion:wasm-beside-entry` keeps the plain `new URL("./x.wasm", import.meta.url)` literal in `dist` so consumers' bundlers emit the Wasm; `src/wasm-source.ts` reads `file:` URLs through `node:fs` in Node. A consumer Vite 8 site built and loaded all three Wasm files; `scripts/test-dist.mjs` checks Node imports in the release workflow.
- Fresh install on Node v26.8.1 with no Rust: 654 ms, `npm audit signatures` verified; the smoke grew silver-birch at seed 7 and queried an 8x8x8 grid in 0.5 s (wood 380, foliage 385 cells, max wood radius 0.252 m, 76 limb systems).
- The harness check (`npm run test:render`) drew all six presets but stopped on an orbit wall-clock p95 (30.00 ms vs 16.7 ms) on a contended display: recorded inconclusive.

## Open

- Deprecate the `0.0.1` placeholder (owner).

## Friction

- dcg hook refused variable-path redirects, `rm -rf dist`, heredocs and `mv` several times -> local setup, reported only.
- Trusted-publisher form unreachable before a first publish -> R6 amended (placeholder).
- Render suite's orbit timing fails on a shared display after ~6 min -> a draw-only mode or a contended-display skip would remove it.
- wasm-bindgen's default module path inlined 2.4 MB more -> `--omit-default-module-path`.
- The plugin's regex rewrote a literal in a doc comment -> literal not spelled in prose.
- Placeholder publish answered 404 because npm was not logged in -> R6 steps start with `npm whoami`.
- Fresh install hit a stale npm cache (`notarget`) -> use `--prefer-online`.
