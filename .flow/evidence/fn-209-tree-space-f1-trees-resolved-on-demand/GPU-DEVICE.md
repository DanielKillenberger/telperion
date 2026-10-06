# fn-209 F1 task 5: fine growth stays on the device (worker, 2026-10-06)

Task 5 carried the host's three decisions after task 4. The wave loop now runs on the device, reading back only totals (host decision 6, FINE-CONTRACT.md). The sag lay's frame correction was checked and recorded (FRAME-CORRECTION.md). Ground-reaching roots are routed to the CPU (host decision 3).

## Answer in brief

- **No per-wave or per-chain readback** (`roots.rs`, `steps.rs`; `record`, `scan`, `gather`, `emit.wgsl`).
  - Per run, one read of four totals: packed axes, packed phytomers, scratch words, and queued chains.
  - Per wave, one read of three totals: kept axes, their phytomers, and borne buds.
  - Everything per chain or per node stays on the device: records, places, packing, overflow queues, gathered buds, geometry items, laid buffers, and the next wave's buds.
  - `Cost::trips` counts the reads where they happen. The tests hold that it equals runs plus waves: **10 to 21 round trips per tree** at 40 and 80.
  - `readback.rs` rebuilds the topology, for the tests only.
- **Topology identical by lineage; geometry inside the screen.**
  - At age 25 (beech, spruce, oak with light off) the results match task 4's.
  - At 40 and 80, tips are at most 34.2 µm (beech 80, seed 7) and RMS at most 2.3 µm.
  - The laid floats are identical, by digest, forward against reversed batches (at 40 and 80), and against batches of 64 at 25. Batches of 64 split every overflow level's queue across runs, and they caught a bug: a deeper level read its queue from the start in every run.
- **Ground routing** (`fine.rs`, `grounded`; `fine/tests/ground.rs`). A root whose reach bound, from the lower of its node and the point below, comes within the landing zone is grown on the CPU only.
  - The bound is the closed form's reach × base scale; the zone takes twice the committed base radius and the longest internode at that scale.
  - Counts at 40: 20 to 29 of the beech's 553 to 975 roots, and 1,301 to 1,658 of the spruce's 20 to 21k.
  - No other root's wood enters the zone. The GPU tests fail if any chain they grew sets the below-ground status bit (D19).
  - At 80 the GPU grows 4.46 to 4.48M of the spruce's 4.7M phytomers and all of the beech's.

## Timing (`raw/gpu-geometry/run4-device.txt`; run3 agrees on the GPU columns)

`FINE_GPU_LOCK=<lock> cargo test --profile ci -p telperion-render fine_geometry_at_40_and_80 -- --ignored --nocapture`. Ryzen 9 5950X (32 threads), RTX 3080.

**Load:** 12 to 20 (one-minute), from other sessions' gates and stills. The CPU columns are inflated by that, the worst item's single-core figure most of all. The GPU columns barely move between runs.

- **GPU:** wall time from table upload to the last wave laid. Grow covers growth, recording, packing and the totals' round trips. Lay covers describing, laying and handing on.
- **CPU:** `grow_fine`'s items, summed on 1 core and as wall time on all cores. These include the roots routed to the CPU.
- **Worst item:** the GPU-grown root slowest on one CPU core, expanded alone.

| Tree | GPU phytomers | Tips most / RMS µm | GPU cold ms (grow / lay; trips) | GPU warm ms (grow / lay; trips) | Worst item: CPU 1 core / GPU ms | CPU Σ items 1 core ms | CPU all cores ms |
|---|--:|--:|--:|--:|--:|--:|--:|
| beech 80 1 | 847,545 | 23.0 / 2.29 | 321 (256 / 41; 18) | 106 (93 / 13; 18) | 6.0 / 8.7 | 1,473 | 115 |
| beech 80 7 | 1,485,640 | 34.2 / 2.26 | 354 (296 / 55; 21) | 164 (146 / 17; 21) | 9.9 / 16.3 | 2,422 | 190 |
| spruce 80 1 | 4,477,634 | 5.6 / 0.33 | 593 (523 / 55; 21) | 435 (374 / 54; 21) | 11.2 / 16.7 | 25,100 | 1,092 |
| spruce 80 7 | 4,460,688 | 7.8 / 0.33 | 912 (844 / 53; 21) | 434 (374 / 53; 21) | 10.4 / 10.9 | 19,361 | 1,111 |

The beech and spruce at 40 are in the raw output: 31 to 42 ms warm on the beech, and 118 to 138 ms on the spruce.

What the table says (the worker's readings):

- **Warm, the spruce at 80 takes 0.43 s on the GPU**, against 1.1 s on 32 CPU cores and 19 to 25 s on one. Task 4's host-driven loop took 1.0 s; the readback it no longer does was 0.36 s of that.
- **Warm, the beech at 80 takes 0.11 to 0.16 s**, against 0.12 to 0.19 s on 32 cores. Its chains are long and few, so a wave is a handful of runs and the round trips (18 to 21) dominate.
- **Cold runs are 1.4 to 3 times warm.** First-touch buffer allocation is the visible part.
  - The first run after a shader change also compiled pipelines for 15 s (`run3`: "pipelines 15210 ms"). Later runs read them back from the driver's cache in 29 ms.
  - A production executor would build these pipelines once per session.
- **The worst item alone takes 9 to 17 ms on the GPU**: 3 to 8 waves of serial dispatches, each with its totals' round trip. Its CPU single-core figure (6 to 11 ms here, 2 to 4 ms at load 1.5 in task 4) is mostly the desk's load.

## Open for the host

- **What the realized-totals change (fn-211's decision) would touch in the GPU executor:**
  - **Girth and load.** `resolve.wgsl`'s fit to the committed base (`girth_all`, `handed`, `delta`, the rows in `gtab`) and `levers_all`'s committed masses would give way to realized sections, retained pipes, masses and moments. These would be summed from the deepest wave up: a new upward pass over the waves before any lay, reusing the wave buffers this task keeps.
  - **A report per root.** Each root's realized totals (section, retained pipes, mass, moment, survival, leaf-girth terms) would be written to a per-root record, the only thing besides totals the CPU then reads. That is a new crossing to name under Astra change 3: a record per root, not per node.
  - **Laying after the settle.** The lay waits for the CPU's settle, and the stubs (`laid::stubs`) carry the settled bearer frames. The bent pass passes its first-lay frame into `step` (`was_heading` and `was_side`, the laid fields 12 to 17), as whole-tree growth's second lay does.
  - **Shedding.** If survival becomes realized rather than read from the committed extinction curves, `shed.wgsl`'s `dead_by` and the curves in the tables change too.
- **The ground bound's tail.** The reach bound is a quantile bound (`TAIL` 1e-9 a zone), not an absolute one. `fine/tests/ground.rs` holds that no GPU root came near the ground at 40.
