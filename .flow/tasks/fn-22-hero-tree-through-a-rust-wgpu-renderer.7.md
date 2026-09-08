---
satisfies: [R8]
---
# fn-22-hero-tree-through-a-rust-wgpu-renderer.7 Retire the Three.js stage and move the rigs to the Rust renderer

## Description
Delete the Three.js stage, tree builder, library adapter and dependency, after moving the two rigs that still use them: the species QA capture onto the headless target and the browser integration test onto the Rust canvas. Update the docs the new crate and the deletion make stale. This is the last task because every rig must pass on the Rust renderer before the old one goes.

**Size:** M
**Files:** delete `harness/stage.ts`, `harness/stage.test.ts`, `harness/skeleton-view.ts` (remaining Three.js parts; family composition already lives in `harness/family.ts`), `harness/skeleton-view.test.ts` (Three.js parts), `src/browser/three.ts`, `tests/browser/geometry-benchmark*.mjs`, `tests/browser/geometry-compare.test.mjs`, `tests/browser/geometry-visibility.mjs`, `scripts/benchmarks/geometry-compare.mjs`; rewrite `tests/browser/species.mjs` and `tests/browser/integration.mjs`; edit `src/index.ts`, `package.json`, `vite.config.ts`, `README.md`, `tests/migration/README.md`, `docs/species-onboarding.md`, `scripts/species-chromium-gpu.sh` (delete if unused)
**Touches:** [harness/**, src/**, tests/**, scripts/**, package.json, package-lock.json, vite.config.ts, README.md, tests/migration/README.md, docs/species-onboarding.md]

### Approach
- Species QA: `tests/browser/species.mjs` becomes a Node script that runs the headless example per profile and seed (`--preset`, `--seed`, `--view whole|bare|leaf`, `--out`) and keeps its measurement half unchanged (`species:measure` already runs the core example); the capture directory layout and `docs/species-onboarding.md` describe the new source of stills. No Playwright in the species path.
- Integration test: `tests/browser/integration.mjs` loads the harness on the Rust canvas, drops the WebGL `finish` shim and the `BINDINGS_ONLY` WebGL branch, and asserts the binding fixtures plus a non-blank canvas; or fold its remaining assertions into `tests/browser/render.mjs` from task 6 and delete it.
- Library: `src/index.ts` stops exporting the Three.js adapter (`materializeTree`, `disposeTreeGeometry`); the package exports the wasm core, the presets and the renderer loader; `vite.config.ts` drops the `three` external; `package.json` removes `three`, `@types/three` and the `three` peer dependency, and its keywords swap `three`, `threejs`, `webgl` for `wgpu`, `webgpu`, `rust`.
- fn-19 geometry benchmark rig: delete the browser rig files and the compare script, add a short retirement note to `tests/migration/README.md:182-200` naming the last commit that ran it.
- Docs: `README.md` Architecture table gains `crates/telperion-render` with both targets and loses the Three.js adapter paragraph; Build and develop lists `render:build`, the wasm-bindgen-cli install line, the headless command and `species:qa`; Measurements and limits separates the rendering path from the rejected GPU query backend. `tests/migration/README.md` build sequence adds the render build.
- Final check: `grep -rn "from \"three" src harness tests scripts` and `grep -n three package.json` are empty; `npm ci` succeeds without three in the lockfile.

### Investigation targets
**Required** (read before coding):
- `tests/browser/species.mjs` — the capture half to replace and the measurement half to keep
- `tests/browser/integration.mjs` — what the binding fixtures still assert
- `src/index.ts`, `src/browser/three.ts` — the library surface being trimmed
- `README.md:45-100`, `tests/migration/README.md:1-30,182-200`, `docs/species-onboarding.md` — sections to update

**Optional** (reference as needed):
- `.flow/specs/fn-19-comparative-botanical-geometry-benchmark.md` — what the retired rig measured, for the note

### Key context
- The species evidence under `.flow/evidence/fn9` and the migration references stay as recorded history; nothing regenerates them.
- The library's npm consumers were never published to; the strategy allows the break.

## Acceptance
- [ ] `npm run species:qa` produces stills for every profile through the headless target and `npm run species:measure` is unchanged
- [ ] `npm run test:browser` (or the folded `test:render`) passes on the Rust canvas with the WebGL shim gone
- [ ] No file under `src`, `harness`, `tests` or `scripts` imports `three`; `package.json` and the lockfile carry no `three` or `@types/three`; `npm ci`, `npm run build`, `npm test` and `npm run typecheck` pass
- [ ] The fn-19 rig files are deleted and `tests/migration/README.md` records the retirement with the last commit that ran it
- [ ] `README.md`, `tests/migration/README.md` and `docs/species-onboarding.md` describe the render crate, both targets, the build step and the headless species stills; the Three.js adapter paragraph is gone

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
