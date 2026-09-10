---
satisfies: [R4, R7]
---
# fn-24-continuous-tree-space-habit-and-leaf-as.5 The blend, the sweep, and the transition on the headless target

## Description
Add the family blend and the every-pair sweep test in the core, and teach the headless example to render a transition between two presets as a numbered frame sequence with a best-effort video (R4's mechanism, R7's no-switch-frame proof).

**Size:** M
**Files:** `crates/telperion-core/src/blend.rs` (new), `crates/telperion-core/tests/sweep.rs` (new), `crates/telperion-render/examples/headless.rs`, `crates/telperion-render/src/headless.rs` (frame loop helper if needed), `README.md` (headless flags)
**Touches:** [crates/telperion-core/src/blend.rs, crates/telperion-core/src/lib.rs, crates/telperion-core/tests/sweep.rs, crates/telperion-render/examples/headless.rs, crates/telperion-render/src/headless.rs, README.md]

### Approach
- Blend: per-field linear interpolation of every numeric family field at one seed; angles as shortest-path deltas; integer counts rounded last, lobe count down and section counts up; 0 and 1 return the inputs exactly. Validation of the result is the existing family validation.
- Sweep test: every pair of shipped presets, ten linear steps, each validates and generates a tree with no error and no non-finite position, and each shipped preset's leaf count sits inside its fidelity band. Runs without a device.
- Headless: add `--to <preset>` and `--frames <n>` (default 240) to the usage at `examples/headless.rs:15-16`; with both, blend per frame, fix the hero pose from the first frame's bounds, render through the existing `render()` at `src/headless.rs:63-160` and `write_png` at `src/headless.rs:162-176`, and write `frame-0001.png` onward beside a record naming both presets, seed, size, frame count and 24 frames per second.
- Video: if `ffmpeg` is on the path, assemble `transition.mp4` with libx264 and yuv420p at 24 fps from the sequence; if not, print one line and keep the sequence. Never a failure.
- Document the two flags in the README's headless block at lines 88-98.

### Investigation targets
**Required** (read before coding):
- `crates/telperion-render/examples/headless.rs:1-60,110-190` — flags, the single still write, the timing session
- `crates/telperion-render/src/headless.rs:63-176` — readback with row padding and the PNG writer to loop
- `crates/telperion-core/src/params.rs:9-40` — the family field table the blend walks

**Optional** (reference as needed):
- `crates/telperion-render/src/camera.rs:193-260` — hero pose from bounds

### Key context
- wgpu 30: `get_mapped_range()` returns a Result; the in-repo readback is the current pattern, not older examples online.
- Agents never open the frames or the video; the sequence and its record are the artefact, the owner is the viewer.
- The transition is one render; it is not a capture to iterate on. Prove the blend on the sweep test first.

## Acceptance
- [ ] A blend of two families at t returns a family; 0 and 1 return the inputs exactly; a blend of two valid families validates
- [ ] Sweep test: every pair of shipped presets in ten steps generates without error or non-finite position; leaf counts inside the fidelity band
- [ ] Headless `--to` and `--frames` render a numbered PNG sequence at the hero pose with a record naming presets, seed, size, frame count and rate; a video is assembled when the system encoder exists and a one-line notice is printed when it does not
- [ ] A no-switch-frame check in the sweep test: adjacent steps of the oak-to-spruce blend differ in element hash and skeleton hash, and no step's element section count jumps by more than the rounding rule allows
- [ ] README documents the flags; `cargo test --release --workspace` and clippy pass

## Done summary
TBD

## Evidence
- Commits:
- Tests:
- PRs:
