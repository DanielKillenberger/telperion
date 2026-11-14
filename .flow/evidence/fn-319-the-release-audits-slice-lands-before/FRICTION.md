# Friction, fn-319

## 28 Oct 2026, task .4 (parallel worker, worktree fn-319-t4)

- Doing: the baseline before the first edit (typecheck and the focused vitest files).
- Slowed by: a fresh worktree has neither the renderer glue (`src/browser/render/`, needed by `tsc`) nor `src/browser/telperion.wasm` (needed by `presets.test.ts` and `voxelize.test.ts`). The first vitest run took 113 s and failed three tests on ENOENT, which reads like a red baseline until the log is opened. Both had to be built through the heavy queue (`render:build` 33 s, `wasm:build` 17 s) before the baseline meant anything.
- Cost: about 4 minutes and one wasted 113 s suite run.
- Would remove it: the conductor building both generated modules once when it creates a task worktree, or the worker dispatch naming `npm run wasm:build && npm run render:build` (through `telperion-heavy`) as the first step for TypeScript tasks.

## 28 Oct 2026, task .2 (parallel worker, worktree fn-319-t1, branch fn-319-t2)

- Doing: the answers check against task 1's hashes after the refactor.
- Slowed by: an intermittent `silver-birch 36` hash mismatch in all-preset `field_answers` runs (3 of 14 on the final binary, 0 of 12 on the base). Telling a regression from latent nondeterminism took 4 further batches of alternated runs.
- Cost: about 12 minutes and 29 GPU runs of 18 s.
- Would remove it: a repeatability test over a whole-catalogue sequence on one generator, not only repeated grows of one preset (`the_device_field_answers_the_same_every_time`), so nondeterminism shows up on the base before a change is blamed.

## 28 Oct 2026, task .3 (parallel worker, worktree fn-319-t3)

- Doing: the baseline (`npx vitest run src/field`) and the first red run of the new Rust test.
- Slowed by: vitest's global setup (`harness/release-binaries.setup.ts`) builds release binaries with cargo (telperion-jev and others) outside the heavy queue and at full parallelism, so a focused three-file vitest run took 136 s, 23 s of it tests. Then the red cargo run waited in `telperion-heavy` for more than 10 minutes on memory (room 1 to 5 GB for 8) while other workers ran, and the foreground call timed out into the background.
- Cost: about 2 minutes of uncapped compile per vitest run and over 10 minutes of queue wait.
- Would remove it: a vitest global setup that builds only what the selected files need (or skips when no selected file uses the binaries), and a smaller memory ask for a single focused test (`telperion-heavy 4`) when the target is warm.
