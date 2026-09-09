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
The oak holds 60 frames per second in the browser. A ten-second orbit at 1600
by 1000, native pixel ratio, on the RTX 3080 in Chrome keeps a frame-to-frame
wall **p50 of 10.00 ms, a p95 of 10.10 ms and a worst frame of 10.10 ms over
999 frames** on this machine's 100 Hz display, against R2's 16.7 ms tail and
33 ms ceiling. R2 is met with two thirds of the budget unspent. The session's
GPU numbers stand beside it: vegetation 2.251 ms, selection 0.063 ms, together
2.315 ms, valid verdict, unquantized. The spruce is recorded and not gated at a
wall p50 of 30.00 ms, which is its 29.4 ms vegetation pass and nothing else.

The suite was run twice end to end, once before and once after a note change;
both passed and the second run's records are the committed ones. Every verdict
came back valid and first-run, so nothing was repeated for contention.

`WebRenderer.orbit()` is the page's entry point. It runs the same protocol
`timing()` runs - conditioning, warmup, 120 measured frames, each awaited on its
own readback - with the camera advancing through one turn, and then draws ten
seconds of frames **the way the page draws them**: no timestamps, no readback,
nothing between one frame and the next but the frame itself, timed by the
`requestAnimationFrame` clock the browser hands any loop it paces.

That split is the one decision worth defending. A frame carrying a timestamp
readback is paced by the readback and not by the display - task 4 named this
about the native orbit, whose wall number is the loop's pace and not a frame
rate. So the GPU numbers and the cadence are measured over different frames on
purpose: the cadence claim is only worth what the frames it was read from cost
a viewer.

Three more decisions.

**The wall clock stands on its own account in the record.** `with_wall` no
longer defers to the GPU verdict. The host's clock is not the GPU's: a browser
without the timestamp feature still draws frames, and the cadence they kept is
what a page's frame rate means. So the record can say GPU time is unavailable
and report the wall numbers beside it - which is exactly R2's error case. The
series is still refused unless every wait is a duration. The record also carries
`wall_frames`, because a wall window is a length of time and how many frames it
came to is the display's answer, not a constant. This changed task 4's
assertion that an invalid verdict takes no wall numbers; that test now covers
selection and levels, and a new one pins the untimed-orbit record.

**A timed browser frame now writes both timestamp pairs.** The browser path
called `Renderer::draw` with the render pair alone while `resolve` read all four
queries, so two of them were never written - a latent hazard task 4 left when it
widened the query set. `Live::draw` takes both pairs or neither, and the browser
now reads the selection pass too: 0.063 ms of the oak's frame, 0.752 ms of the
spruce's.

**The rig's evidence folder defaults to this spec's.** It defaulted to
`.flow/evidence/fn22`, so a plain `npm run test:render` would have written over
a closed spec's committed records. The host flagged the default as fixable
inside this task, and it is fixed.

Chromium is launched with three more flags - background timer throttling,
occluded-window backgrounding and renderer backgrounding all disabled - and the
timing page is brought to the front. A browser slows its animation clock when it
decides nobody is looking, and this display belongs to somebody; the page being
measured is one a viewer is watching. The flags are in every record, as
fn-22's protocol requires. Both clocks are coarsened to Chrome's 100
microsecond step, which is why the wall numbers land on a 0.1 ms grid; the
record's note now says so rather than leaving a reader to wonder at a frame time
of exactly 10.00 ms.

Four deviations from the declared Touches, named rather than buried.
`crates/telperion-render/src/web/session.rs` is new - the task's own key context
asked for the session glue to move to a sibling module rather than push `web.rs`
past 400 lines, and `web.rs` ends at 341. `timing/report.rs` carries the wall
contract above, `timing.rs`'s wasm `sample_ms` returns both pairs like its
native twin, `tests/timing.rs` follows the contract change, and
`Cargo.toml` adds web-sys's `Window` feature for the animation frame. None of
them is reachable from inside the declared list.

AC5's software-flags case is covered where it can be. A browser launched with
`--disable-gpu` is offered no adapter at all, so the renderer refuses before a
session exists; the run prints that reason and continues rather than failing.
The record rule it stands for - wall numbers kept, GPU time unavailable - is
asserted directly in the Rust timing suite.

Not done here, by scope: no stills, no owner verdict, no report update - task 6
owns all three and the browser rows are on disk for it. `species:qa` was not run
for the same reason.

Follow-up worth a later spec, not built here: the browser's vegetation pass
reads 2.25 ms against the native target's 1.75 ms on the same tree and GPU, a
half-millisecond the fn-22 gap did not predict. It is inside the budget, so it
was not chased.

stage: impl-review - skipped(config: REVIEW_MODE=none)
## Evidence
- Commits: 1dc7829b229f7c7099d6916b66fcd601937a7e5e
- Tests: cargo test --release --workspace (29 suites, 0 failed), RENDER_EVIDENCE=.flow/evidence/fn23 npm run test:render (5 presets, dial, views, 2 timing sessions, 2 orbit sessions; run twice, both PASS), npm test (64 vitest cases), npm run typecheck, npm run rust:test:wasm, cargo clippy --release --workspace --all-targets, cargo clippy --release --target wasm32-unknown-unknown -p telperion-render, baseline: green via handoff (verified at 1e42b09 by fn-23-fast-hero-the-oak-inside-the-frame.4)
- PRs:
stage: plan-sync - skipped(config: planSync.enabled != true)
