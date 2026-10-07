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

## Task 4: after the F1 merge (2026-10-07 06:23, load 5.6 to 10 on 32 threads)

The same `prepare_timing` at `fab45ab8` (F1 merged; stage 2 still the engine's CPU `grow`, which now runs the beech and the spruce through F1's partition on the CPU reference executor; the oak and the palm stay whole). Before is the quiet run above. Raw: `raw/task4/prepare-after.txt`.

| preset | age | cores | total ms before / after | skeleton ms before / after | peak MB after |
|---|--:|---|--:|--:|--:|
| european-beech | 40 | all | 110 / 102 | 55 / 35 | 257 |
| european-beech | 80 | all | 1,842 / 2,419 | 668 / 562 | 1,219 |
| norway-spruce | 40 | all | 2,050 / 3,016 | 804 / 881 | 1,205 |
| norway-spruce | 80 | all | 14,376 / 16,842 | 3,056 / 3,320 | 5,359 |
| oregon-white-oak | 40 | all | 424 / 592 | 329 / 492 | 385 |
| oregon-white-oak | 80 | all | 4,302 / 5,791 | 3,585 / 4,744 | 1,932 |
| date-palm | 80 | all | 6 / 6 | 2 / 2 | 138 |
| european-beech | 80 | 1 | 2,497 / 3,560 | 1,344 / 1,839 | 1,193 |
| norway-spruce | 80 | 1 | 19,150 / 31,007 | 7,772 / 12,338 | 5,271 |
| oregon-white-oak | 80 | 1 | 8,139 / 13,168 | 7,427 / 12,124 | 1,772 |

The descriptors and wood columns (not stage 2) also rose 30 to 70 percent between the runs, so the load, not the merge, explains much of the rise; the two runs are not comparable to a few percent. The clean comparison is the engine's `grow` alone, interleaved, before (`3d9d1175`, the base) and after (`8a1eafc1`), one tree a process, best of three warm, seed 1 (`raw/task4/onegrow-pre-vs-merged.txt`, probe `harness/onegrow.rs`):

| tree | one core (taskset), before / after ms | all cores, before / after ms |
|---|--:|--:|
| beech 80 | 994 / 1,109; 1,034 / 1,260 | 413 / 203; 392 / 184 |
| spruce 80 | 6,031 / 8,512; 9,318 / 10,181 | 2,118 / 1,115; 2,184 / 1,223 |
| oak 80 (lit) | 6,895 / 7,744; 7,074 / 8,158 | 3,435 / 3,424; 3,378 / 3,387 |
| oak 40 (lit) | 554 / 615; 526 / 638 | |

- **All cores:** the merge halves the beech's and the spruce's growth (F1's partition, REALIZED.md decision 7); the oak is unchanged.
- **One core, which is the browser's case:** the merge is 9 to 41 percent slower on every tree, the oak included, whose path did not change shape. F1's integer contract (Q32 presences, 128-bit products) is the likely cause; not profiled.
- **Stage 2 beyond growth** (one scratch split, spruce 80 all cores, load 7): growth 1.10 to 1.42 s, the canonical sort 0.14 s, conversion 0.41 to 0.42 s, the rest of `executor::grow` (inputs, envelope, freeing the structure) about 0.3 s. The sort re-orders a tree `grow` already returns in F1's lineage order, but into a different order (depth, then identity); adopting `lineage_order` would delete it and change the conversion's node numbering (host decision C of FINE-CONTRACT.md says fn-198 adopts F1's order). Not done: it moves bytes for 0.14 s.
