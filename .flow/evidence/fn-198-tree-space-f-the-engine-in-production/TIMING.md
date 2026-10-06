# fn-198 task 2: parameters to a prepared tree

`crates/telperion-render/examples/prepare_timing.rs`: one process a tree, `Generator::new` then `Generator::prepare(&family, Delivery::Resident)` timed from the family to the prepared, resident tree (the engine's growth, conversion, the expansion's CPU steps, the uploads and GPU placement); seed 1; `--profile ci`; all cores (32 threads) or `RAYON_NUM_THREADS=1` (the engine's pool). Peak memory is the process's `VmHWM`. RTX desk GPU under the shared lock.

**Load:** other sessions held the 1-minute load at 52 to 127 on 32 threads throughout (column `load`, read before each tree), so these times are indicative only and all-core is not faster than one core here: the cores were taken. A quiet rerun is R2's (task 4).

| preset | age | cores | total ms | skeleton ms | descriptors ms | wood ms | leaves | peak MB | load |
|---|--:|---|--:|--:|--:|--:|--:|--:|--:|
| european-beech | 40 | all | 463 | 229 | 74 | 145 | 65,665 | 308 | 64.67 |
| european-beech | 80 | all | 6,701 | 2,753 | 865 | 2,637 | 2,474,647 | 1,267 | 83.22 |
| norway-spruce | 40 | all | 7,721 | 3,296 | 1,143 | 2,724 | 4,610,729 | 1,196 | 88.97 |
| norway-spruce | 80 | all | 45,213 | 11,683 | 9,444 | 22,334 | 16,912,892 | 5,332 | 97.26 |
| oregon-white-oak | 40 | all | 1,487 | 1,220 | 80 | 175 | 166,264 | 381 | 127.42 |
| oregon-white-oak | 80 | all | 18,109 | 15,414 | 697 | 1,769 | 951,156 | 1,918 | 127.54 |
| date-palm | 40 | all | 10 | 4 | 0 | 0 | 6,554 | 136 | 110.73 |
| date-palm | 80 | all | 15 | 5 | 0 | 0 | 6,554 | 137 | 110.73 |
| european-beech | 40 | 1 | 649 | 365 | 74 | 196 | 65,665 | 257 | 110.73 |
| european-beech | 80 | 1 | 11,217 | 5,736 | 1,280 | 3,635 | 2,474,647 | 1,259 | 110.73 |
| norway-spruce | 40 | 1 | 13,910 | 8,091 | 1,348 | 3,967 | 4,610,729 | 1,155 | 98.51 |
| norway-spruce | 80 | 1 | 45,334 | 23,678 | 5,503 | 14,798 | 16,912,892 | 5,232 | 87.95 |
| oregon-white-oak | 40 | 1 | 1,223 | 1,054 | 51 | 111 | 166,264 | 366 | 59.66 |
| oregon-white-oak | 80 | 1 | 14,553 | 13,331 | 350 | 782 | 951,156 | 1,770 | 59.66 |
| date-palm | 40 | 1 | 7 | 1 | 0 | 0 | 6,554 | 136 | 51.54 |
| date-palm | 80 | 1 | 10 | 1 | 0 | 0 | 6,554 | 137 | 51.54 |

Reading:

- The skeleton (the engine's growth, the canonical sort and the conversion) is 41 to 92 percent of the total for the beech, spruce and oak; the oak's at 80 is 13 to 15 s of its 15 to 18 s, its light lattice (GPU-PATH.md).
- The spruce at 80 peaks at 5.2 GB natively through `prepare`, over the 4 GB Wasm ceiling (host decision 5 refuses it in the browser; task 3).
- The palm grows the same 6,554 fronds at 40 and 80; its stem lengthens.
