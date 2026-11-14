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
