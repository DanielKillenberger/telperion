---
satisfies: [R1]
---
# fn-22-hero-tree-through-a-rust-wgpu-renderer.5 Browser target and panel wiring

## Description
Compile the renderer to a wasm-bindgen module, build it from a script that checks the wasm-bindgen CLI version, and cut the harness canvas over to it: one canvas, drawn by the Rust renderer, every dial live, the three views, orbit input, the timing session in place of the pixel-ratio sweep, and every error in the panel's alert (R1). The Three.js files stay on disk untouched until task 7 deletes them; after this task nothing on the page imports them.

**Size:** M
**Files:** `crates/telperion-render/src/web.rs`, `crates/telperion-render/Cargo.toml`, `scripts/build-render.mjs` (new), `package.json`, `.gitignore`, `src/browser/render.ts` (new, typed adapter), `harness/rust-stage.ts` (new), `harness/family.ts` (new, the family composition lifted from `skeleton-view.ts`), `harness/orbit.ts` + `harness/orbit.test.ts` (new), `harness/GrowerDev.tsx`, `harness/grower-dev.css`, `vite.config.ts` (only if the glue needs `optimizeDeps.exclude`)
**Touches:** [crates/telperion-render/**, scripts/build-render.mjs, package.json, .gitignore, src/browser/**, harness/**, vite.config.ts]

### Approach
- `web.rs` (wasm32 only): `wasm-bindgen = "=0.2.128"` (exact, the CLI must match), `wasm-bindgen-futures`, `web-sys` with `HtmlCanvasElement`; `#[wasm_bindgen] WebRenderer::new(canvas) -> Promise<WebRenderer>` via `future_to_promise` around `SurfaceTarget::Canvas`, `set_tree(json: &str) -> Result<JsValue, JsError>` that parses with `params::parse`, generates with `mesh::build`, submits and returns the `Submitted` counts plus build ms; `set_view`, `set_camera`, `resize(w, h)` reconfiguring only between frames and clamped to the device texture limit, `frame() -> stats`, `timing() -> Promise<Report>`, `dispose()`. Every error converts through the `RenderError`/`Error` `Display` text. Alpha mode `Opaque`.
- `scripts/build-render.mjs`: `cargo build --release --target wasm32-unknown-unknown -p telperion-render`, read the wasm-bindgen version from `Cargo.lock`, compare with `wasm-bindgen --version`, on absence or mismatch print `cargo install wasm-bindgen-cli --version <x>` and exit 1, else `wasm-bindgen --target web --out-dir src/browser/render`; gitignore `src/browser/render/`. npm: `render:build`, and `dev`/`pretest` run it after `wasm:build`.
- `src/browser/render.ts`: typed loader (`import init, { WebRenderer } from "./render/telperion_render.js"` with the wasm URL via `?url`), a `ResizeObserver` with the pixel-ratio cap of two mirrored from `harness/stage.ts`, and a live-device counter exposed for the soak test as a typed global.
- `harness/family.ts`: the dial-to-family composition (`toSkeletonParams`, `toRadiusParams`, `toSurfaceParams`, `toCanopyParams`, `presetToParams` from `harness/skeleton-view.ts:38-165,285`) moved into a Three.js-free module with its tests; `skeleton-view.ts` keeps only what the old stage still needs until task 7.
- `harness/orbit.ts`: pure yaw, pitch, distance about a target from pointer deltas and wheel, with clamps; unit-tested like `harness/stage.test.ts`.
- `harness/rust-stage.ts`: `setTree(familyJson)`, `setView`, `frame`, `frameIfWaiting`, `frameNext`, `stats`, `timing`, `dispose`, with the frame latch semantics at `harness/stage.ts:228-262`.
- `harness/GrowerDev.tsx`: replace `createStage` with the Rust stage on the one canvas (the `logDepth` key and toggle go, WebGPU has no logarithmic depth flag); the build effect at `harness/GrowerDev.tsx:157-232` composes the family JSON and calls `setTree`, reusing `BUILD_LIVE_MS`/`BUILD_SETTLE_MS`; the view selector calls `setView`; the sweep button and `SweepResult` UI become a timing button showing the session's p50, p95 and verdict; on a generator or renderer error, `setBuildError` with the message and the previous tree stays. Keep the additions small; new logic lives in `rust-stage.ts`.
- Dispose on effect cleanup so React's development double-mount leaves one live device.

### Investigation targets
**Required** (read before coding):
- `harness/GrowerDev.tsx:78-155,157-232,300-341` — refs, stage effect, build effect, sweep, canvas, alert
- `harness/stage.ts:220-262` — the stage contract the Rust stage replaces
- `harness/skeleton-view.ts:38-165,285,373-390,498-520` — family composition and views
- `src/browser/core.ts:109-149` — the copying pattern this path must not repeat
- `scripts/build-wasm.mjs` — build script shape and where generated files go

**Optional** (reference as needed):
- `harness/stage.test.ts`, `harness/skeleton-view.test.ts` — test style for harness modules
- wasm-bindgen guide, deployment `--target web`

### Key context
- wasm-bindgen-cli is not installed on this machine; the script's version check is the install path.
- A JS view over wasm memory detaches when memory grows; the adapter never holds one, all uploads happen inside Rust.
- Only `Opaque` alpha mode works on the web backend; `Device::poll` is a no-op there.
- Vite 6 serves the harness; no COOP/COEP headers are needed because there is no shared memory.
- The compare mode (two presets side by side, `harness/GrowerDev.tsx:117`) is dropped from the page with the old stage; the memory note on two subjects on stage explains why it is not carried over.
## Acceptance
- [ ] `npm run render:build` emits the glue and wasm under `src/browser/render/`; with the CLI missing or mismatched it exits 1 naming the exact install command
- [ ] `npm run dev` shows oak and spruce through the Rust renderer on the one canvas; every dial and preset load regenerates and redraws; whole, bare and leaf views switch; the timing button reports p50, p95 and a verdict
- [ ] With WebGPU absent (Chromium `--disable-gpu`) the alert names the condition; a rejected parameter set shows the generator's message and leaves the previous tree on screen
- [ ] `harness/GrowerDev.tsx` and `harness/main.tsx` import nothing from `three` or `harness/stage.ts`; the dial-to-family tests pass from `harness/family.ts`
- [ ] Orbit unit tests pass; `npm run typecheck` and `npm test` pass; no authored untyped JavaScript under `src` or `harness`
- [ ] After React's development double-mount the live-device counter reads one; `GrowerDev.tsx` ends shorter than it started
## Done summary
The renderer now compiles to a wasm-bindgen module and draws the harness
canvas: `web.rs` exports a `WebRenderer` over a canvas surface with
`setTree`, `setView`, `heroCamera`, `setCamera`, `resize`, `frame`, `stats`,
`timing` and `dispose`, generation runs inside that module so only parameter
text goes in and counts come back, and every failure arrives in JavaScript as
the Rust error's own words. `scripts/build-render.mjs` builds the module and
generates its gitignored glue, refusing with the exact `cargo install
wasm-bindgen-cli --version 0.2.128` line when the CLI is missing or
mismatched. The page is cut over: the dial-to-family composition moved to
`harness/family.ts` with its own tests, `harness/orbit.ts` carries the orbit
arithmetic under unit test, `harness/rust-stage.ts` owns the frame loop, the
pointer and the frame latch, and `GrowerDev.tsx` drives all of it in 204
fewer lines with the timing session in place of the pixel-ratio sweep.
Nothing the page imports reaches `three` or the old stage.

Verified in Chromium on a hardware adapter: oak and spruce render in clay,
presets and dials regenerate and redraw, whole, bare and leaf switch, the
timing button reported a valid p50 of 0.327 ms and p95 of 0.331 ms over 120
frames, a parameter set the generator refuses shows its message with the
previous tree still on screen, a software-only adapter names that condition,
and the live device count reads one after React's development double-mount.

Follow-ups for task 7, not built here: `tsconfig.build.json` still sweeps
`src/browser/render.ts` into the library declarations, so `dist` carries an
unreferenced `browser/render.d.ts` pointing at glue that is not shipped; the
file is outside this task's Touches, and `npm run build` was wired to
generate the glue first so it passes either way.

stage: impl-review - skipped(policy: parallel-wave - the conductor reviews after it integrates)

Conductor decisions: GrowerDev.tsx ends at 412 lines, down from 616, within the about-400 rule; wasm-bindgen-cli 0.2.128 was installed on this machine by the worker; the tsconfig.build.json declaration sweep is carried to task 7.

stage: plan-sync - skipped(config: planSync.enabled != true)
stage: wave-join - ran (commit 458fe01 already on target; no integration needed)
## Evidence
- Commits: 458fe0157c3eda9df8c9083892d65f174ddab129
- Tests: cargo test --release --workspace, npm test, npm run typecheck, npm run build, npm run render:build (glue emitted; both refusal paths exercised with a missing and a mismatched CLI), browser smoke on hardware WebGPU (Chromium --enable-unsafe-webgpu --enable-features=Vulkan,VulkanFromANGLE --use-angle=vulkan): oak and spruce render, presets and dials regenerate, whole/bare/leaf switch, timing valid p50 0.327 ms p95 0.331 ms over 120 frames, generator rejection keeps the previous tree, software-only adapter names the condition, live devices = 1 after StrictMode double-mount
- PRs: