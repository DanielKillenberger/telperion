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
The repository has one renderer. The clay stage, the tree builder, the Three.js
adapter, the `three` and `@types/three` dependencies and the peer dependency are
deleted, and the two rigs that still needed them run on the Rust renderer: species
QA renders its stills through the headless example, and the browser binding
contract runs where it always belonged, under `npm run rust:test:wasm`.

Species QA is no longer a browser rig. `tests/species.mjs` keeps the measurement
half exactly as it was and replaces the capture half with one headless process per
still: 48 numeric cases pass and 48 stills come back, whole, bare and leaf for
oak and spruce at fixed 1/2/3, the first three fresh seeds, the retained spruce
width counterexample, and Ordinary, Telperion and Laurelin. The 16 whole and 16
bare stills are all distinct; the leaf stills collapse to three, because one placed
element at generated scale is a property of the preset and not of the seed. The
supplementary presets are captured at the protocol's first fixed seed rather than
at a preset's own, because the headless target names its seed instead of inheriting
one. The whole run takes about eight minutes on this machine and no browser, page,
Vite server or Playwright appears anywhere in it.

Three judgements the task had to make rather than mechanically apply.

The old capture rig framed exterior, peg, socket, junction and branch-curtain
cameras to inspect attachment; those were diagnostics of a Three.js scene and have
no equivalent on a renderer that draws three views. They are retired with the rig
rather than reinvented, and the migration guide says so. Second, `test:browser` and
`rust:test:wasm` were two entry points to the same binding rig with different
environment variables. The rig is now `tests/browser/bindings.mjs` - binding
contract only, no adapter needed - and `test:browser` is gone; the page on a real
GPU is `tests/browser/render.mjs` under `test:render`, and the two suites no longer
overlap. Third, three further rigs drew through the stage and could not survive it:
`tests/browser/migration.mjs` (the FN6 browser comparison) and
`scripts/benchmarks/{measure,memory}.mjs` (the stage's frame and heap costs). They
are deleted with a retirement note beside the fn19 one, naming `43b5881` as the
last commit that could run them; their evidence and their code are untouched in
Git history.

The library now exports `createRenderer` beside the core and the presets, which is
what the spec's library surface asked for and what resolves the loose end task 5
left: `src/browser/render.ts` was being swept into `dist` as an unreferenced
declaration, and it is now the referenced renderer loader. The bundle carries both
wasm modules inline, at 2.6 MB and 890 kB gzipped, and a consumer that never
imports the renderer can still tree-shake it: the package has no runtime
dependencies and `sideEffects` is false.

One inherited red, recorded rather than fixed: `cargo clippy --workspace
--all-targets -D warnings` fails on `clippy::assign_op_pattern` in
`crates/telperion-core/examples/geometry_benchmark/metrics.rs:238`. That file has a
zero-line diff from the base commit and `crates/**` is outside this task's declared
Touches, so it is not this task's to fix and not this task's to hide. Clippy is not
among the spec's Quick commands; every gate that is, is green.

Two things a follow-up may want. The old integration rig also drove the panel's
load-failure retry, its build-failure alert and the unknown-`?species=` link error;
those flows still exist on the Rust page and now have no automated coverage,
because the spec's approach for that rig was the binding fixtures and a non-blank
canvas, and `render.mjs` already asserts the canvas for every preset. And the
species stills are now framed in the renderer's own clay scene, ground and 1.8 m
figure included, where the old diagnostic rig omitted them; the owner may want the
figure out of a QA still.

baseline: green via handoff (green (verified at 8863c885 by fn-22-hero-tree-through-a-rust-wgpu-renderer.6))

Gates after the commit: `cargo test --release --workspace`, `npm test` (64 tests),
`npm run typecheck`, `npm run build`, `npm ci`, `npm run rust:test:wasm`,
`npm run test:render`, `npm run species:qa`, `cargo fmt --all -- --check` all green;
`cargo clippy` red as recorded above. Receipts:
`.flow/tmp/green-receipts/2fd76c2f-unittest.json` and
`.flow/tmp/green-receipts/2fd76c2f-npm-test.json`.

stage: impl-review - skipped(policy: parallel wave - the conductor reviews after integration)

Conductor decisions: the three extra rig deletions (FN6 browser comparison, stage frame and heap benchmarks) accepted with their retirement note; test:browser folded into rust:test:wasm accepted; the two follow-ups (panel error-flow coverage, scale figure in QA stills) go to the successor list. Inherited clippy red fixed by the conductor in Phase 4.

stage: plan-sync - skipped(config: planSync.enabled != true)
stage: wave-join - ran (commit 2fd76c2 already on target; no integration needed)
## Evidence
- Commits: 2fd76c2fc867f55b156da6ace0d8611a1cc2d25c
- Tests: npm run species:qa -- --output /tmp/claude-1000/species-run: 48/48 numeric pass, 48/48 stills pass through target/release/examples/headless (oak and spruce, fixed 1/2/3 + first three fresh + the retained spruce counterexample, plus ordinary/telperion/laurelin at fixed seed 1), whole/bare/leaf each; 16 whole and 16 bare stills all distinct; runner exits 1 by design while visual inspection is unassessed, npm run test:render (hardware WebGPU, Chromium headed on :0): PASS - 5 presets non-blank and distinct, height dial regenerates, bare/leaf/whole change what is drawn, no-GPU launch names the software fallback, oak p50 17.948 ms p95 17.960 ms valid, spruce p50 62.874 ms p95 62.930 ms valid, npm run rust:test:wasm (the binding suite, now tests/browser/bindings.mjs, plain headless Chromium, no adapter): all 13 binding contract groups pass, cargo test --release --workspace: pass (receipt .flow/tmp/green-receipts/2fd76c2f-unittest.json), npm test: 3 files, 64 tests pass (receipt .flow/tmp/green-receipts/2fd76c2f-npm-test.json), npm run typecheck: pass, npm run build: pass, dist/telperion.js 2,648.82 kB with both wasm modules inlined, npm ci: pass, 97 packages, no three and no @types/three in the lockfile or node_modules, grep -rn 'from "three' src harness tests scripts: empty; grep -n three package.json: empty, cargo fmt --all -- --check: pass, cargo clippy --workspace --all-targets -- -D warnings: RED, inherited - clippy::assign_op_pattern at crates/telperion-core/examples/geometry_benchmark/metrics.rs:238; crates/ has a zero-line diff from the base commit and is outside this task's Touches, so the failure predates this task and was not touched by it, baseline: green via handoff (green (verified at 8863c885 by fn-22-hero-tree-through-a-rust-wgpu-renderer.6))
- PRs: