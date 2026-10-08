# fn-236 friction

## 2026-10-09: a redirect to a path held in a variable is blocked

- **Doing:** rendering round 0's stills into the evidence folder.
- **Slowed by:** the shell guard refused `> $E/sheet0/render.log` (a dynamic redirect target), as fn-213 recorded twice; the run became a script with its paths inside (`render_round.sh`).
- **Cost:** one blocked command, about 3 minutes.
- **Would remove it:** nothing in the repository; a local guard setting (reported, not specced). fn-213's entry names the same.

## 2026-10-09: no still tool for the 0.1.4 oak, and no value-row writer on this branch

- **Doing:** the 0.1.4 oak's stills beside the engine's, and planning how each round's values reach the preset.
- **Slowed by:** the 0.1.4 tree needs its own source tree and build (`git archive v0.1.4` into the scratchpad, fn-213's `old_birch.rs` adapted as `old_oak.rs`, a cold `--profile ci` build of the render crate). The `space_rows` example fn-213 used to write `/space` rows from `birch()` is on the birch branch only, so this branch's rows are edited by hand and held equal to `oak()` by the test.
- **Cost:** about 10 minutes of build and setup.
- **Would remove it:** stills of each shipped release's trees kept once in `.refs/` (rendered with the engine stills' shots), and `space_rows` merged to the engine branch.

## 2026-10-09: no measure of a closed crown that the rounds could steer by

- **Doing:** looking for a number that says whether a crown reads closed, to steer round 1's probes.
- **Slowed by:** two tries failed. A gap count on the stills' dark pixels was confounded by leaf brightness: O-WHOLE's sunlit leaves and 0.1.4's bright green leaves count as gaps (0.29 and 0.31 to 0.39 against the engine's dark crowns at 0.13 to 0.23), so it was dropped. The node-grid `fill` in `oak_measure.rs` barely moves across 21 probes (0.82 to 0.90 at 80). The probes were therefore steered by viewing compare sheets, one render of six trees each.
- **Cost:** about 20 minutes, and five extra render rounds.
- **Would remove it:** a still with a flat background and an unlit leaf mask (a silhouette shot in the still runner), so a gap is a background pixel whatever the leaf colour.

## 2026-10-09: other sessions' load

- **Doing:** the probes and renders.
- **Slowed by:** one-minute loads of 18 to 118 on 32 threads from other sessions; a 40-year probe that takes 0.7 s took 11 s.
- **Cost:** a few minutes, and build times in this round that are noise.
- **Would remove it:** nothing in the repository (reported, not specced).

## 2026-10-09: a leaf probe needs a rebuild, and the first after the merge took over ten minutes

- **Doing:** the leaf probes L1 to L4, each a preset edit and a render.
- **Slowed by:** the still runner reads the compiled preset, so each probe is a rebuild (about 45 s), and the first one after merging F4 rebuilt the workspace and ran past the 10-minute foreground limit. Restoring the preset between probes needed a scratch copy, because the shell guard refuses `git checkout -- <file>`.
- **Cost:** about 12 minutes.
- **Would remove it:** the still runner reading a value file from disk (as `oak_measure.rs` does) behind a flag, so a probe needs no rebuild.
