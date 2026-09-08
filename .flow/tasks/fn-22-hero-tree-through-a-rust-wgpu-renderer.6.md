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
TBD

## Evidence
- Commits:
- Tests:
- PRs:
