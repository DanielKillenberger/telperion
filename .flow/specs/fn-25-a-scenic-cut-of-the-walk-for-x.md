# A scenic cut of the walk, for X

## Goal

The owner wanted to post the oak-to-spruce walk on X the same day. fn-24's transition was a receipt: a fast linear walk at a camera locked on the oak's hero pose. This spec turned the same walk into a clip that reads as one tree becoming another: slow, eased, the camera easing between the two trees' own hero poses while drifting round them, plus a leaf turning from blade to needle. Clay only; motion and framing carry it. Re-grown crown shimmer is reduced by the slow walk, not removed (a later spec).

## Decisions

- The headless target gains `--walk` (seconds, smoothstep ease), `--hold` and `--sweep` (azimuth); the existing `--frames` transition stays byte-identical. `--walk` with `--frames` is refused as a clash.
- One script (`scripts/scenic-cut.mjs`) assembles whole-tree and leaf sequences into one 1920x1080 H.264 clip with crossfades and an ffmpeg grade; sequences and clip stay on disk, ignored; the record is committed.
- Boundaries: no renderer, generator or preset change; no text overlay; one full-size render, proofs at 480x270.

## Acceptance Criteria

- **R1:** Eased walk between two presets with holds and a camera easing between both hero poses, whole and leaf view; `--frames` unchanged; invalid flags named — met (`examples/headless/walk.rs`, `tests/walk.rs`).
- **R2:** One script produces `scenic.mp4`, 1920x1080, H.264 yuv420p, 24 fps, under 24 s, with crossfades, grade and a record — met (20.5 s, 52 MB).
- **R3:** Owner views the clip and records whether it is fit to post — met.

## Result

Full render: 384 whole-tree frames (2 s hold, 12 s eased, 2 s hold, 35 degrees) and 120 leaf frames (5 s, 20 degrees) at 1920x1080; clip 20.5 s.

> _verdict (owner, 2026-09-10):_ The owner viewed the clip locally and asked for the pull request and the merge in the same breath ("alright let's make pr's and merge them"). No separate wording on the clip was given; the merge request is taken as acceptance for this spec.

## Open

None.

## Evidence

- `.flow/evidence/fn25/scenic.json` (clip record and ffmpeg invocation), `whole/transition.json`, `leaf/transition.json` (sequence records); frames and clip are ignored.
