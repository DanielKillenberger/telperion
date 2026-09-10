---
satisfies: [R1, R2, R3]
---
# fn-25-a-scenic-cut-of-the-walk-for-x.1 The eased walk, the camera, and the graded clip

## Description
Build the scenic walk on the headless target and the assembly script, prove them at proof size, then render the one full-size clip for the owner (R1, R2; R3 is the owner's). Single task; keep the diff lean and the existing fn-24 transition path untouched.

**Size:** M
**Files:** `crates/telperion-render/examples/headless.rs` (walk flags, ease, hold, camera walk; split a `walk.rs` sibling module under `examples/headless/` if the file passes 400 lines), `crates/telperion-render/src/camera.rs` only if a pose-interpolation helper belongs there, `scripts/scenic-cut.mjs`, `README.md` (headless block), `.flow/evidence/fn25/` (record and clip; frames ignored)
**Touches:** [crates/telperion-render/examples/**, crates/telperion-render/src/camera.rs, scripts/scenic-cut.mjs, README.md, .flow/evidence/fn25/**, .gitignore]

### Approach
- Flags on the headless example beside the existing `--to`/`--frames`: `--walk <seconds>` (frames = seconds × 24, blend eased with smoothstep 3t²−2t³), `--hold <seconds>` (that many seconds of frames at t=0 before and t=1 after the walk; the camera keeps sweeping through the holds), `--sweep <degrees>` (total azimuth change across the whole sequence, linear in time). With `--walk`, the camera is the hero pose of the first family eased to the hero pose of the last family (compute both once from each endpoint's bounds via the existing `hero_pose`; interpolate target, distance and elevation with the same smoothstep as the blend; azimuth from the sweep). The leaf view (`--view leaf`) uses the same rule on the leaf bounds. `--frames` without `--walk` keeps today's behaviour exactly; add a test that the flag parser and the frame schedule for a `--frames` run are unchanged.
- The record `transition.json` gains `walk`, `hold`, `sweep`, `ease` fields when a walk ran.
- `scripts/scenic-cut.mjs`: renders (or takes as input) two sequences, whole and leaf, then runs ffmpeg once with a filter graph: the whole sequence (hold 2 s, walk 12 s, hold 2 s, sweep about 35°) then xfade 0.5 s into the leaf sequence (walk 5 s, no hold, sweep 20°), then `eq=contrast=1.06:saturation=1.05`, `colorbalance` a touch warm in the mids, `vignette` soft, `-c:v libx264 -crf 18 -pix_fmt yuv420p -r 24`, 1920x1080. Total under 24 s. Writes `scenic.json` beside the clip with the inputs and the exact ffmpeg command. Missing ffmpeg: one line, exit 0. Missing sequence: error naming the path.
- Proof first at 480x270 with `--walk 2 --hold 0.5`; check the record and the frame count, never open the frames. Then the one full-size render into `.flow/evidence/fn25/` (frames ignored by a `.gitignore` in that directory, the clip ignored too; only `scenic.json` and `transition.json` records are committed). Do not open the clip; the owner views it.

### Investigation targets
**Required** (read before coding):
- `crates/telperion-render/examples/headless.rs` — the fn-24 walk (`--to`, `--frames`), the frame loop, the record
- `crates/telperion-render/src/camera.rs:180-260` — `hero_pose` and the pose fields to interpolate
- `.flow/evidence/fn24/transition/transition.json` — the record shape to extend

**Optional** (reference as needed):
- `scripts/build-wasm.mjs` — the repo's script conventions (ESM, no deps)
- `.flow/evidence/fn24/REPORT.md` — the transition timings (about 1.5 s per whole-tree frame on the 3080)

### Key context
- ffmpeg 9.0.1 is at /usr/bin/ffmpeg. X wants H.264 yuv420p, 1920x1080, under 140 s.
- Budget: one full render, about 12 minutes; proofs at 480x270 only. Never open frames or the clip (project evidence rule).
- Nothing in `crates/telperion-render/src/` beyond a camera helper; no lighting or colour change.

### Acceptance
- [ ] `--walk`, `--hold`, `--sweep` on the headless target; eased blend and eased camera between the two hero poses; leaf view walks too; invalid values refused naming the flag
- [ ] `--frames` path unchanged, asserted by a test on the frame schedule
- [ ] `scripts/scenic-cut.mjs` produces a 1920x1080 H.264 yuv420p 24 fps clip under 24 s with crossfades and grade, plus `scenic.json`; missing ffmpeg is one line; missing sequence errors naming the path
- [ ] Proof render at 480x270 ran; full render into `.flow/evidence/fn25/` ran once; frames and clip ignored, records committed
- [ ] README headless block documents the three flags
- [ ] `cargo test --release -p telperion-render`, fmt and clippy pass

## Acceptance
- [ ] `--walk`, `--hold`, `--sweep` on the headless target; eased blend and eased camera between the two hero poses; leaf view walks too; invalid values refused naming the flag
- [ ] `--frames` path unchanged, asserted by a test on the frame schedule
- [ ] `scripts/scenic-cut.mjs` produces a 1920x1080 H.264 yuv420p 24 fps clip under 24 s with crossfades and grade, plus `scenic.json`; missing ffmpeg is one line; missing sequence errors naming the path
- [ ] Proof render at 480x270 ran; full render into `.flow/evidence/fn25/` ran once; frames and clip ignored, records committed
- [ ] README headless block documents the three flags
- [ ] `cargo test --release -p telperion-render`, fmt and clippy pass


## Done summary
The headless target now walks in seconds rather than frames: `--walk` eases the blend with a smoothstep over a stated duration, `--hold` stands still at each end while the camera keeps moving, and `--sweep` turns a stated azimuth across the whole sequence. A walk no longer holds the first frame's pose - `walk_pose` in `camera.rs` eases between the two ends' own hero poses (what the camera looks at, how far back it stands, how high) so neither tree is framed for the other, and `--view leaf` walks the same way on each end's leaf. The command line and the schedule moved to `examples/headless/walk.rs`, which `tests/walk.rs` compiles in so the plan can be asserted without a device; the `--frames` path keeps its own branch, its own record and its own formula, pinned by a test that spells the old formula out. `scripts/scenic-cut.mjs` reads the two sequences off disk, crossfades and grades them and encodes once through ffmpeg, writing `scenic.json` with every input and the exact invocation and re-reading the finished file rather than assuming it.

One full-size render was taken: 384 whole-tree frames (2 s held, 12 s eased, 2 s held, 35 degrees swept) and 120 leaf frames (5 s, 20 degrees), both 1920x1080. The cut is `.flow/evidence/fn25/scenic.mp4`, h264 yuv420p 1920x1080 at 24 fps, 20.5 s, 52 MB. No frame and no clip was opened; the records beside them are the evidence. R3 is the owner's: the clip is waiting to be viewed and judged.

Two notes for the reviewer. `crates/telperion-render/tests/walk.rs` is new and sits outside the task's declared Touches: an example target's `#[cfg(test)]` module never runs under `cargo test`, so the acceptance criterion that the `--frames` schedule be asserted by a test could not be met inside `examples/**` without a `[[example]] test = true` stanza in `Cargo.toml`, which is equally outside Touches and is an implementation edit rather than a test one. The new file adds no product behaviour. And `--walk` with `--frames` is refused as a clash: the two state one length two ways, and silently preferring either would have been a guess.

stage: impl-review - skipped(config: REVIEW_MODE=none)
## Evidence
- Commits: e64e7b23daea90b7640cb2d780be8f795a744675, a0a35bacebc0eff1781f04783e963f5cbfa9c751, 6b2e8ad7b29ad17a12da8583e8316e82ce74cf92
- Tests: cargo test --release -p telperion-render (55 tests, green), cargo fmt --check -p telperion-render, cargo clippy --release -p telperion-render --all-targets, proof render 480x270 --walk 2 --hold 0.5 --sweep 30 (72 frames), leaf proof --walk 0.5 --sweep 20 (12 frames), --frames 3 at 64x64 (record unchanged), node scripts/scenic-cut.mjs: full cut 1920x1080 h264 yuv420p 24 fps 20.5 s; missing sequence, empty directory, bad --fade, over-long fade, bad --size, unknown flag and missing-ffmpeg paths each exercised
- PRs: