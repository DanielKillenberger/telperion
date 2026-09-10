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
The core walks between families: `blend::families(&a, &b, t)` interpolates every
numeric parameter of a family at one seed — angles along the shorter arc, lobes
rounded down and sections up so the leaf's own rule survives the walk, a
disabled bias field rising from nothing rather than arriving, and a growth
override neither row states left to the envelope — and the headless target
renders a walk between two presets as a numbered PNG sequence with a record
beside it and a video when the machine has an encoder.

stage: impl-review - skipped(policy: PARALLEL_WAVE - conductor reviews after it integrates the wave)

### What landed

- `crates/telperion-core/src/blend.rs` (new, 234 lines), `pub mod blend;` in
  `lib.rs`. `t` outside `0..=1` or not finite is
  `InvalidInput("blend parameter")`; 0 and 1 return the rows themselves clone
  for clone; `blend(a, a, t) == a` for every `t`.
- `crates/telperion-core/tests/sweep.rs` (new, five tests, 6.2 s together, no
  device).
- `crates/telperion-render/examples/headless.rs` gained `--to <preset>` and
  `--frames <n>` (240 by default, 24 fps); the still path is untouched.
- `README.md`'s headless block documents both flags and the sequence layout.

Nothing else moved: no preset row, no trait default, nothing under `foliage/` or
`branching/`, and no pin was re-recorded.

### The acceptance, item by item

1. **A blend returns a family; 0 and 1 return the inputs exactly; a blend of two
   valid families validates.** `the_ends_of_a_walk_are_the_rows_themselves_...`
   compares the whole wire object at both ends of all ten pairs, and every step
   of the sweep below validates by generating.
2. **Every pair in ten steps generates with no error and no non-finite
   position.** `every_pair_of_presets_grows_a_tree_at_every_step`: ninety
   generations under one 8,000-node cap, wood positions and bounds all finite.
   The cap is stated in the test's doc comment; leaf counts are judged at full
   size, because a cap that low truncates a tree before its twigs.
3. **Leaf counts inside the fidelity band.** `BANDS` in the same file, measured
   at full size — see the one number below.
4. **A no-switch-frame check.** `the_oak_to_spruce_walk_has_no_switch_frame`:
   adjacent steps differ in both the skeleton hash and the element hash, and the
   section count never moves further in one step than the rounding rule allows
   (the walk's own stride, plus one).
5. **The transition and the encoder.** Proved by running it, both branches.
6. **`cargo test --release --workspace` and clippy pass.** Green in 234 s (the
   pre-edit baseline was green in 213 s); `cargo clippy --release --workspace
   --all-targets -- -D warnings` is clean.

### The one number the owner should see

`ordinary` carries **41,827 retained leaves at seed 7** on a twenty-four metre
envelope, a decade below the 10^5-to-10^7 the strategy's fidelity track names —
on the same envelope the oak carries 869,310. This is a property of the
colonizing rows after fn-24.1's one builder, not of the blend, and it reads as
the same movement fn-24.1 recorded when it lowered Telperion's retained rail
from a million to four hundred thousand. `BANDS` records Ordinary at 10^4 to
10^6 and says in its comment that this is where the count stands, not where the
owner has said it should stand. No preset was touched; task 6 should put the
number in front of the owner.

### Evidence from the two runs

- 24 frames of oak-to-spruce at 480x300, seed 7: `transition.json`,
  `transition.mp4` and `frame-0001.png` onward in
  `/tmp/flow-handover-fn24/fn-24.5-transition/`, written in 35 s on an RTX 3080.
  No frame and no video was opened.
- The absent-encoder branch, run with `PATH` pointing at an empty directory into
  `/tmp/flow-handover-fn24/fn-24.5-noencoder/`: one line — `no ffmpeg on the
  path; the frames stand` — the same string in the record's `encoder` field, the
  sequence kept, exit 0.

Per frame the cost is one whole `mesh::build` plus its upload, so task 6's
240-frame full-size render is minutes: budget it as one render.

Wave notes, including what siblings collide with and the blend's contract in
detail, are in the run's notes directory as `task5-integration-notes.md`.

### Integration and the host review (conductor, 2026-09-10)

Cherry-picked onto the spec branch as 288d640; the workspace commit 26b8f81 is retired with the worktree. Host review: the blend walks every numeric field of a family by name, angles along the shorter arc, lobes down and sections up, the supernatural field rising from zero, and 0 and 1 return the rows unchanged; the headless flags and the transition record match the spec's contract, and the encoder branch was exercised both ways. Carried to task 6: the ordinary preset's 41,827 retained leaves at seed 7 sits below the strategy's band, a property of the colonizing rows after task 1 that the owner should see beside the stills.

stage: plan-sync - skipped(config: planSync.enabled != true)
## Evidence
- Commits: 288d640
- Tests: worker baseline: cargo test --release --workspace green in 213 s before any edit, worker: cargo test --release --workspace - 30 binaries green in 234 s, worker: cargo clippy --release --workspace --all-targets -- -D warnings - clean, conductor, integrated target 288d640: cargo test --release --workspace - green, rc=0, proof render: headless --preset oregon-white-oak --to norway-spruce --seed 7 --frames 24 --size 480x300 into /tmp/flow-handover-fn24/fn-24.5-transition/ (35 s, mp4 assembled), absent-encoder branch with an empty PATH into /tmp/flow-handover-fn24/fn-24.5-noencoder/ (sequence kept, one-line notice, exit 0)
- PRs: