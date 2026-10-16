---
satisfies: [R2]
---
# fn-211-tree-space-f2b-growth-years-cheap.3 A kept thread pool, scratch reuse and flat draw records

## Description
Host decisions on task 2's questions (2026-10-06): (1) a persistent pool for cores::spread, via rayon (the standard pool; a hand pool would need unsafe), respecting the process-wide core budget and working on one core; (2) no second index-order path for one core (docs/principles.md: no switch between ways of building); instead reuse scratch buffers across years so the depth-at-a-time passes cost no more than the loops they replaced on one core; (3) each axis's draw record and per-axis vectors become flat arrays (struct of arrays or one arena), removing the quarter second of freeing per big tree and cutting peak memory. Every tree identical to the bit.

## Acceptance
- The hash table identical at every commit (80 years, seeds 1 and 7, neutral and lit). - Per-pass timings (measures passes) before and after on 1 and all cores, ages 40 and 80, four species; one core no slower than before task 2. - Peak memory before and after. - The crate's unit tests compiled and run at every commit, not only the example. - Workspace gate green.

## Done summary
Three exact levers, every tree identical to the bit (measures hash 80, seeds 1 and 7, neutral and lit, all cores at each commit, one core at the first and last), crate tests run at each commit.

- a353489f: cores::spread runs on rayon's global pool, kept for the process (one thread a core is the budget); one core or wasm runs on the calling thread.
- e8c7996b: upward passes reuse one scratch per piece; sag's levers about -8% on one core, thicken unchanged. The by-depth passes' one-core cost is visiting order, not allocation.
- f55c91b1: Draws holds a Unit record and an optional shade pair (3 vectors where 5, 2 unlit); re-lays read draws by reference instead of cloning them.

All cores vs end of task 2: oak 80 -9 to -14%, spruce 80 -4 to -7%, beech 80 -12 to -13%. One core vs before task 2: no slower (oak 80 s1 repeated: means 7,351 to 7,036 ms). Peak memory: oak s7 3.95 to 3.72 GB, spruce 2.06 to 1.99 GB. Workspace gate 1,113 passed. Host questions in RESULT.md section 5 (cross-axis arena layout; one-core visiting order).

## Evidence
- Commits: a353489f, e8c7996b, f55c91b1, 784cbd6c
- Tests: measures hash 80 1,7 all both (all cores and taskset -c 0), cargo test --profile ci -p telperion-space, cargo test --profile ci --workspace --no-fail-fast, cargo build --release --target wasm32-unknown-unknown -p telperion-wasm
- PRs: