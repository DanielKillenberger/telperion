---
satisfies: [R1, R5, R6, R7]
---
# fn-22-hero-tree-through-a-rust-wgpu-renderer.6 Browser conformance, soak, evidence and docs

## Description
Prove the browser path with Playwright under WebGPU flags: presets render, dials regenerate, views switch, the WebGPU-absent message appears, the timing session runs with a verdict, and the opt-in five minute soak holds one device. Record the browser evidence and finish the report for the owner's verdict. Doc updates wait for task 7, which changes the build story once more.

**Size:** M
**Files:** `tests/browser/render.mjs` (new), `scripts/test-render.mjs` (new), `package.json`, `.flow/evidence/fn22/oak-browser-timing.json`, `.flow/evidence/fn22/spruce-browser-timing.json`, `.flow/evidence/fn22/soak.json`, `.flow/evidence/fn22/REPORT.md`
**Touches:** [tests/browser/render.mjs, scripts/test-render.mjs, package.json, .flow/evidence/fn22/**]

### Approach
- `scripts/test-render.mjs` mirrors `scripts/test-wasm.mjs` (build, Vite server on a free port, run the browser test). `tests/browser/render.mjs` follows `tests/browser/integration.mjs:18-40`: Playwright chromium with `--no-sandbox --enable-unsafe-webgpu --enable-features=Vulkan --use-angle=vulkan --disable-vulkan-surface --ignore-gpu-blocklist`; if `navigator.gpu.requestAdapter()` returns null in the page, print the skip reason and exit 0. No WebGL `finish` shim; that belonged to the old stage.
- Scenarios: each of the five presets selected and the canvas read back non-blank with no alert; one dial move (height) changes the reported counts; bare and leaf views change the reported instances; a second launch with `--disable-gpu` asserts the alert names WebGPU or the adapter; the timing session for oak and spruce written as JSON with verdict, the flags used and a note that Chrome quantizes timestamps to 100 microseconds without developer features; `SOAK=1` runs five minutes idle polling every 30 s that the live-device counter is one, the canvas element identity is unchanged and the build counter is unchanged, then drags the orbit and asserts the pixels changed, writing `soak.json`.
- Evidence: add the browser rows to `REPORT.md` (species, flags, p50, p95, verdict) beside the native rows; keep the `## Owner verdict` slots empty for the owner. View no more than two images.

### Investigation targets
**Required** (read before coding):
- `tests/browser/integration.mjs:18-45` — launch, evidence dir and page setup conventions
- `scripts/test-wasm.mjs` — server-then-test wrapper
- `.flow/evidence/fn22/REPORT.md` — the report from task 4 to extend

**Optional** (reference as needed):
- `scripts/species-chromium-gpu.sh` — the Vulkan flag set already used for captures

### Key context
- Browser timestamps are coarse without developer features; the browser rows are honest about that and the native rows are the ones later specs compare against.
- The soak is opt-in and manual; nothing runs it on a schedule.
## Acceptance
- [ ] `npm run test:render` passes on this machine and exits 0 with a printed skip when the launched browser offers no WebGPU adapter
- [ ] Five presets render non-blank through the Rust renderer, a dial move regenerates, bare and leaf views change the reported instances, and the `--disable-gpu` launch shows the alert naming the condition
- [ ] `.flow/evidence/fn22/` gains both browser timing reports with verdicts and flags, and `soak.json` from one `SOAK=1` run showing one live device, one canvas and a responding orbit over five minutes
- [ ] `REPORT.md` lists native and browser numbers with verdicts and keeps one empty owner-verdict slot per species
- [ ] `npm test` and `npm run typecheck` pass
## Done summary
`npm run test:render` drives the page on the hardware adapter the way the owner
does: every shipped preset renders non-blank and no two of them draw the same
picture, eight notches of the height dial regenerate the tree, bare and leaf
change what is drawn, and a second browser launched with `--disable-gpu` shows
the renderer's own sentence about the software fallback. The timing sessions for
oak and spruce run on a bare canvas of their own at seed 7 and 1600x1000, so the
browser rows in `REPORT.md` are the same two trees the native rows measured - the
counts match the native table exactly - and the opt-in five minute soak held one
canvas, one live device and no rebuild the page asked for itself, with the orbit
answering a drag at the end.

Three things the run had to be honest about rather than assume. A headless
Chromium on this machine is offered SwiftShader and nothing else, which the
renderer refuses by design, so the suite needs a display and skips with the
renderer's own reason when there is no adapter behind it; the wrapper names `:0`
when the caller's environment has no DISPLAY and this session's X server is
there. Chrome did not quantize these timestamps - `--enable-unsafe-webgpu` lifts
the 100 microsecond grid - so each record works that out from its own percentiles
instead of stating the assumption. And the display belongs to a person: 344
keystrokes and pointer events arrived from the desktop during the soak, so the
soak drops keyboard focus before its idle window, counts what still arrives, and
starts again only when a rebuild has stray input behind it.

Two findings for the owner, neither of them this task's to fix. Browser spruce
measures 62.9 ms against 94.2 ms native on the same tree, same canvas and same
GPU, both sessions `valid` and both tight, while oak agrees to within a tenth of
a percent - something differs between the two pipelines on the instance-heavy
tree and nothing here establishes what; it is recorded in `REPORT.md` for the
fast-hero spec. And the no-GPU message reads `the software fallback ""`: WebGPU
tells the module nothing about the adapter, so the Rust record's adapter and
driver fields are empty in a browser and the page's own view of the adapter is
recorded beside them in the timing JSONs.

The owner verdict slots in `REPORT.md` are still empty, as asked.

baseline: green via handoff (green (verified at 458fe015 by fn-22-hero-tree-through-a-rust-wgpu-renderer.5))

Gates after the commit, all green: `cargo test --release --workspace`, `npm test`
(127 tests), `npm run typecheck`. Receipts:
`.flow/tmp/green-receipts/8863c885-unittest.json` and
`.flow/tmp/green-receipts/8863c885-npm-test.json`.

stage: impl-review - skipped(policy: parallel wave - the conductor reviews after integration)

Conductor decisions: the browser suite needs a display because headless Chromium offers only SwiftShader, accepted and recorded; the 62.9 ms versus 94.2 ms spruce discrepancy between browser and native is carried to the fast-hero spec via REPORT.md; the empty adapter name in the browser fallback message is recorded beside the page view of the adapter.

stage: plan-sync - skipped(config: planSync.enabled != true)
stage: wave-join - ran (commit 8863c88 already on target; no integration needed)
## Evidence
- Commits: 8863c88594b3adec5c7cef5125309920848fbf3c
- Tests: npm run test:render (hardware WebGPU, Chromium 153 headed on :0 with --enable-unsafe-webgpu --enable-features=Vulkan --use-angle=vulkan --disable-vulkan-surface --ignore-gpu-blocklist): 5 presets non-blank and all distinct, height dial regenerates, bare/leaf/whole change what is drawn, no-GPU launch names the software fallback, SOAK=1 npm run test:render: five minute soak, 10 polls, one canvas, one live device, 0 rebuilds, orbit responded (344 stray desktop events recorded), browser timing sessions: oak p50 17.860 ms p95 17.877 ms valid; spruce p50 62.910 ms p95 62.929 ms valid, both 120 frames at seed 7, 1600x1000, cargo test --release --workspace, npm test, npm run typecheck
- PRs: