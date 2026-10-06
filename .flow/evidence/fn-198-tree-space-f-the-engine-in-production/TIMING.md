# fn-198 task 2: parameters to a prepared tree

`crates/telperion-render/examples/prepare_timing.rs`: one process a tree, `Generator::prepare(&family, Delivery::Resident)` timed from the family to the prepared, resident tree (the engine's growth, the canonical sort, the conversion, the expansion's CPU steps, the uploads and GPU placement); seed 1; `--profile ci`; all cores (32 threads) or `RAYON_NUM_THREADS=1` (the engine's pool; the expansion's own threads are unchanged). Peak memory is the process's `VmHWM`. Desk GPU under the shared lock. `skeleton` is stage 2: growth, sort and conversion.

## Quiet run (2026-10-07 01:30, load 4.5 to 8.3 on 32 threads)

| preset | age | cores | total ms | skeleton ms | descriptors ms | wood ms | leaves | peak MB |
|---|--:|---|--:|--:|--:|--:|--:|--:|
| european-beech | 40 | all | 110 | 55 | 14 | 39 | 65,665 | 258 |
| european-beech | 80 | all | 1,842 | 668 | 309 | 766 | 2,474,647 | 1,266 |
| norway-spruce | 40 | all | 2,050 | 804 | 340 | 803 | 4,610,729 | 1,175 |
| norway-spruce | 80 | all | 14,376 | 3,056 | 2,969 | 7,444 | 16,912,892 | 5,325 |
| oregon-white-oak | 40 | all | 424 | 329 | 33 | 57 | 166,264 | 386 |
| oregon-white-oak | 80 | all | 4,302 | 3,585 | 207 | 429 | 951,156 | 1,925 |
| date-palm | 40 | all | 5 | 2 | 0 | 0 | 6,554 | 136 |
| date-palm | 80 | all | 6 | 2 | 0 | 0 | 6,554 | 137 |
| european-beech | 40 | 1 | 129 | 73 | 14 | 39 | 65,665 | 256 |
| european-beech | 80 | 1 | 2,497 | 1,344 | 307 | 749 | 2,474,647 | 1,273 |
| norway-spruce | 40 | 1 | 3,048 | 1,794 | 345 | 802 | 4,610,729 | 1,157 |
| norway-spruce | 80 | 1 | 19,150 | 7,772 | 3,065 | 7,484 | 16,912,892 | 5,233 |
| oregon-white-oak | 40 | 1 | 647 | 554 | 32 | 57 | 166,264 | 368 |
| oregon-white-oak | 80 | 1 | 8,139 | 7,427 | 215 | 424 | 951,156 | 1,774 |
| date-palm | 40 | 1 | 4 | 0 | 0 | 0 | 6,554 | 136 |
| date-palm | 80 | 1 | 5 | 1 | 0 | 0 | 6,554 | 136 |

Reading:

- All cores against one: the skeleton runs 1.3x (beech 40) to 2.5x (spruce 80) faster; the oak's light-reading growth gains 2.1x at 80. The expansion's steps do not use the engine's pool, so totals gain less (1.2x to 1.9x).
- The oak at 80 spends 83 percent of its time in growth (its light lattice, GPU-PATH.md); the spruce at 80 spends 79 percent past it, in descriptors and wood for 4.6M nodes and 16.9M needles.
- The spruce at 80 peaks at 5.2 GB natively through `prepare`, over the 4 GB Wasm ceiling (host decision 5 refuses it in the browser; task 3). Every other tree stays under 2 GB.
- The palm grows the same 6,554 fronds at 40 and 80; its stem lengthens.

## Loaded run (01:00, load 52 to 127), kept for the record

| preset | age | cores | total ms | skeleton ms | load |
|---|--:|---|--:|--:|--:|
| european-beech | 40 | all | 463 | 229 | 64.67 |
| european-beech | 80 | all | 6,701 | 2,753 | 83.22 |
| norway-spruce | 40 | all | 7,721 | 3,296 | 88.97 |
| norway-spruce | 80 | all | 45,213 | 11,683 | 97.26 |
| oregon-white-oak | 40 | all | 1,487 | 1,220 | 127.42 |
| oregon-white-oak | 80 | all | 18,109 | 15,414 | 127.54 |
| date-palm | 40 | all | 10 | 4 | 110.73 |
| date-palm | 80 | all | 15 | 5 | 110.73 |
| european-beech | 40 | 1 | 649 | 365 | 110.73 |
| european-beech | 80 | 1 | 11,217 | 5,736 | 110.73 |
| norway-spruce | 40 | 1 | 13,910 | 8,091 | 98.51 |
| norway-spruce | 80 | 1 | 45,334 | 23,678 | 87.95 |
| oregon-white-oak | 40 | 1 | 1,223 | 1,054 | 59.66 |
| oregon-white-oak | 80 | 1 | 14,553 | 13,331 | 59.66 |
| date-palm | 40 | 1 | 7 | 1 | 51.54 |
| date-palm | 80 | 1 | 10 | 1 | 51.54 |
