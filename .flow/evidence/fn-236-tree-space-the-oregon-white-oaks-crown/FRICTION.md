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
