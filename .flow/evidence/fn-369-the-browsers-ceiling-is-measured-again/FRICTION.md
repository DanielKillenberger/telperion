# fn-369 friction

## 2026-11-14 02:47 to 03:10, the queue's admission

- **Doing:** the first measurement run (the `measure-ceiling` module build and one spruce case) through `TELPERION_HEAVY_RESERVE=8 telperion-heavy 8`.
- **What slowed it:** admission held it with 0 to 5 GB of room against the 8 GB asked while about 30 GB was available: the reserve of 8 plus what other running heavy jobs may still claim up to their caps.
- **Cost:** 23 minutes of waiting (admitted at 03:10:32 with 9 GB of room) before the first case, with nothing else to run (every step of this task grows a tree).
- **What would remove it:** an admission that counts other jobs' current use rather than their full caps, or a smaller cap for the module build alone (it compiles in about 20 s and needs well under 8 GB).

## 2026-11-14 03:50 to 04:56, the oak walked to 400 without a trap

- **Doing:** the full `scripts/wasm-ceiling.mjs` run.
- **What slowed it:** the oak's kept count peaks at 3.54 M near 140 and then falls (the light thins it), so stepping by ten years never meets a failure and walked all 37 ages to the script's 400 cap: 70 minutes of build time against 10 for the spruce and 22 for the beech.
- **Cost:** about 65 minutes of the run's 105.
- **What would remove it:** stop a species' walk once its kept count has fallen for two steps in a row (its peak is past), or walk by kept count rather than age.

## 2026-11-14 05:24, a queued test job killed in its compile

- **Doing:** step 2's narrow tests (`cargo test --profile ci`) and the clothed-tree hashes through `telperion-heavy 8 --threads 4`.
- **What slowed it:** the job was killed 50 s after admission with exit 143 (signal 15), which the wrapper reports as "likely the 8 GB cap", while it was still compiling; the yield watcher paused and resumed at 05:22:41 and 05:24:03. Which of the two stopped it is unknown.
- **Cost:** one admission and the compile restarted, a few minutes.
- **What would remove it:** the wrapper naming who sent the signal (the cap, the watcher or a user stop) instead of guessing.
- **Update 05:26:** `journalctl --user` names it: the kernel's OOM killer in the scope, at the 8 GB cap and again at 10 GB, about a minute in. The test binaries then compiled outside the queue in 27 s, so the compile was not it: the likely cause is `telperion-wasm`'s whole library suite (its `generate` tests build trees) running four at a time, which my filter did not narrow. Rerun with the wasm tests filtered to `live::`.
