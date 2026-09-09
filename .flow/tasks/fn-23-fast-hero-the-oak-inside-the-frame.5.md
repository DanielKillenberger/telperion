---
satisfies: [R2]
---
# fn-23-fast-hero-the-oak-inside-the-frame.5 The browser at 60 frames per second: the orbit session on the page

## Description
Run the orbit session on the fn-22 page under hardware WebGPU and record R2. Mirror the native orbit protocol through the wasm surface, extend the TypeScript timing interface, and drive it from the Playwright rig with the flags and quantization recorded.

**Size:** M
**Files:** `crates/telperion-render/src/web.rs`, `src/browser/render.ts`, `tests/browser/render.mjs`, `scripts/test-render.mjs`, `.flow/evidence/fn23/oak-browser-timing.json`, `.flow/evidence/fn23/oak-browser-orbit.json`, `.flow/evidence/fn23/spruce-browser-timing.json`
**Touches:** [crates/telperion-render/src/web.rs, src/browser/render.ts, tests/browser/render.mjs, scripts/test-render.mjs, .flow/evidence/fn23/**]

### Approach
- `WebRenderer` gains `orbit()` beside `timing()` (`web.rs:259-267`), reusing the session from task 4 through the same per-frame callback path as `measure()` at `web.rs:296-320`; the measured frames advance the orbit parameter and the wall time is taken from `performance.now()` deltas passed in from the page loop, so the record's wall numbers are the page's own frame cadence.
- Ten seconds of orbit at the display's refresh rate is the measured window; conditioning and warmup precede it as in the plain session. The record carries the flags Chromium was launched with and whether timestamps were quantized, as fn-22's browser records do.
- `src/browser/render.ts`: add `orbit()` to the `Renderer` interface (:81-92) and the new optional fields to `TimingReport` (:44-58), matching the Rust JSON byte for byte.
- `tests/browser/render.mjs`: add an orbit scenario beside `session()` (:141-178) on the `/render-timing` route that writes the orbit record, asserts wall p95 under 16.7 ms and max under 33 ms for the oak, and records spruce without asserting. Keep the count parsers at :63-72 and the leaf-view check at :319 untouched; they must still pass.
- Run under `HARDWARE_FLAGS` on the RTX 3080 with the desktop otherwise idle; a contended GPU verdict is recorded and the run repeated once, then reported as is.

### Investigation targets
**Required** (read before coding):
- `crates/telperion-render/src/web.rs:151-274` — the wasm surface; `:296-320` the measure loop
- `src/browser/render.ts:44-92` — the TypeScript mirror of the record and the renderer interface
- `tests/browser/render.mjs:23-26` and `:141-178` — launch flags and the timing session driver

**Optional** (reference as needed):
- `harness/orbit.ts` — the pure orbit maths the panel uses for drags; the session's sweep is renderer-side and does not go through it
- `.flow/evidence/fn22/oak-browser-timing.json` — record shape with flags and quantization

### Key context
- Chrome quantizes WebGPU timestamps unless the unsafe flag is on; the wall-time numbers are the R2 evidence and stand without the timestamp feature.
- `web.rs` is at 337 lines; if `orbit()` pushes it over about 400, move the session glue to a sibling module rather than growing it.

## Acceptance
- [ ] The page's renderer exposes `orbit()` returning the orbit record with wall p50, p95 and maximum, GPU numbers when available, flags and quantization; `TimingReport` in TypeScript matches the Rust JSON fields
- [ ] `.flow/evidence/fn23/oak-browser-orbit.json` records a ten-second orbit at native pixel ratio on the RTX 3080 in Chrome with wall p95 under 16.7 ms and no frame above 33 ms; a failing frame is named in the test output
- [ ] `.flow/evidence/fn23/oak-browser-timing.json` and `spruce-browser-timing.json` are recorded with verdicts; spruce is not asserted
- [ ] `npm run test:render` passes under hardware WebGPU including the new orbit scenario, and the existing count and leaf-view checks pass unchanged
- [ ] Without the timestamp feature the orbit record carries wall numbers and reads GPU time as unavailable, covered by a software-flags run that skips with a printed reason rather than failing

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
