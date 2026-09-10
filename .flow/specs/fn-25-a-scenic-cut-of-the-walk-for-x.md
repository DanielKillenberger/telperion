# A scenic cut of the walk, for X

## Goal & Context

The owner wants to post the oak-to-spruce walk on X today. fn-24's transition is a receipt: a ten-second linear walk at a camera locked on the oak's hero pose, which reads as fast and noisy and shrinks the spruce into the middle of the frame. This spec turns the same walk into a clip that reads as one tree becoming another: slow, eased, with the camera easing between the two trees' own hero poses while drifting round them, and a close-up of one leaf turning from blade to needle. Clay, no appearance work; motion and framing carry it. The frame-to-frame shimmer of a re-grown crown is reduced by the slow eased walk, not removed; that stays a later spec, as the owner said.

## Approach

- The headless target gains a walk: a duration in seconds instead of a frame count, hold seconds at both ends, a smoothstep ease on the blend parameter, and a camera that eases between the hero pose of the first family and the hero pose of the last while sweeping a stated number of degrees of azimuth over the whole sequence. The existing `--to` and `--frames` behaviour is untouched, so fn-24's evidence render still reproduces.
- The leaf view takes the same walk, framed on the leaf of each endpoint.
- One script assembles the whole-tree sequence and the leaf sequence into a single 1920 by 1080 H.264 clip at 24 frames per second, under 24 seconds, with half-second crossfades and a light grade (a little contrast, a little warmth, a soft vignette), via ffmpeg. The frame sequences stay on disk, ignored; the clip and its record are the deliverable.

## Quick commands

```bash
cargo test --release -p telperion-render
cargo run --release -p telperion-render --example headless -- --preset oregon-white-oak --to norway-spruce --seed 7 --walk 2 --hold 0.5 --sweep 30 --size 480x270 --out /tmp/fn25-proof/frame.png
node scripts/scenic-cut.mjs --out .flow/evidence/fn25/scenic.mp4
```

## Acceptance Criteria

- **R1:** The headless target renders an eased walk between two presets with hold seconds at both ends and a camera that eases between the two endpoints' hero poses while sweeping a stated azimuth, for the whole view and the leaf view, and the existing `--frames` transition is byte-identical to before. Errors: a walk of zero seconds, a negative hold or a sweep outside minus 360 to 360 degrees is an invalid input naming the flag.
- **R2:** One script produces `scenic.mp4`, 1920 by 1080, H.264 yuv420p, 24 frames per second, under 24 seconds, from the two sequences with crossfades and a grade, and writes a record beside it naming every input and the ffmpeg invocation. Errors: a missing ffmpeg is one line and the sequences stay; a missing sequence is an error naming the path.
- **R3:** The owner views the clip on the tailnet page and records whether it is fit to post; the spec closes on that verdict. Errors: a rejecting verdict stops the spec with the owner's words.

## Boundaries

- No renderer change: no lighting, colour, material or background change in the render crate. The grade is ffmpeg's.
- No generator or preset change; the flicker of a re-grown crown is a later spec.
- No text overlay; the owner captions on X.
- One full-size render for the final clip; proofs at 480 by 270.

## Requirement coverage

| Req | Description | Task(s) |
|-----|-------------|---------|
| R1 | Eased walk with camera on the headless target | .1 |
| R2 | Assembly script and the graded clip | .1 |
| R3 | Owner verdict on the clip | .1, verdict by the owner |

## Owner verdict

R3 is the owner's judgment of the clip at `.flow/evidence/fn25/scenic.mp4`, 20.5 seconds, 1920 by 1080, kept on disk and out of git.

> _verdict (owner, 2026-09-10):_ The owner viewed the clip locally and asked for the pull request and the merge in the same breath ("alright let's make pr's and merge them"). No separate wording on the clip was given; the merge request is taken as acceptance for this spec.
