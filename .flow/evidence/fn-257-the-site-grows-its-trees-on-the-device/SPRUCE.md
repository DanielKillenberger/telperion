# fn-257 task 8: the spruce's cost that is not kernel arithmetic (worker, 2026-10-16)

The spruce at 40, seed 1, warm, RTX 3080:

- **Profile:** `growth_profile` (`raw/t8/`, `raw/t8/after/`), plus `CYCLE_TRACE` stage times.
- **Each remedy:** the base and changed binaries alternated, 16 warm `tree::drawn` builds each (`drawn_warm`, `raw/t8/warm-*.txt`).
- **Identity:** every preset's tree hashed at seeds 1, 7 and 42 against task 9's (`drawn_hashes`).

## The profile, before → after

| Stage | Before ms | After ms | Share (after) | Kernel arithmetic? |
|---|--:|--:|--:|---|
| CPU tables (capture, chain) | 19.7 (9.9 + 9.8, in turn) | about 11 (side by side) | 3 % | no: CPU; **taken** |
| grow: making 1.59 GB of tables (`Build::new`, 298 buffers) | 21.4 | 21.4 | 6 % | no: the driver making memory, about 14 ms a GB (a probe: 300 × 1 MB 3.3 ms, × 5 MB 21 ms, × 20 MB 89 ms) |
| grow: the cycles (device busy, census waits) | about 260 | about 260 | 67 % | yes: the census waits are the device working (no gap between cycles, 0.03 ms) |
| grow: of it, dispatch launch and order | about 26 | about 26 | 7 % | no: DISPATCH.md; task 4's fold measured slower |
| grow: the one compaction (after cycle 37) | 25–31 host, about 5 device | the same | about 1–2 % of wall | partly: see below |
| settle | 43.0 | about 37 | 10 % | about half: 21 ms device, 12 ms making its 426 MB of buffers, 7 readbacks (10 before), each on its own submission's result |
| plan, trunk | 11.0, 8.2 | 10.3, 7.8 | 5 % | yes: each one readback waits on the previous stage's device work (plan's own device time 0.4 ms) |
| whole `tree::drawn` | 395.8 | about 381 (medians, the two taken) | | |
| field on top: readied, stations, primitives, index | 56 | 56 | | readied 23.5, stations 16.6, primitives 9.6, index 6.4 |
| field query, 36 cells a side | 63–67 | 63–67 | | the query kernel |

## Each non-kernel cost

- **CPU tables: taken.** The capture's and the chain's tables are independent; natively they are now made side by side (`tree.rs`'s `side_by_side`, in turn in a browser's one thread). Warm `tree::drawn`: spruce 390.8 → 383.4 ms (sd 3.4, 3.6), Telperion 855.5 → 835.6 (sd 5.7, 3.9). 24 of 24 trees are byte-identical.
- **Growth's setup, 1.59 GB made a build: not taken, for the host.** Making memory costs the driver about 14 ms a GB, whatever the buffers' number (one error scope a buffer costs nothing measurable). The host's remedy, persistent buffers reused across builds of a capacity class, keeps growth's tables alive after growth:
  - Today growth's tables are let go before the dress so the dress's 665 MB never sits beside them. Pooled, they would sit beside it: the build's peak rises from 1,587 MB to about 2,250.
  - Between builds the page would also hold 1.6 GB.
  - It saves about 21 ms (growth) + 12 ms (settle) on the spruce. That is a memory-for-time trade the owner's memory direction weighs against, so it waits for the host's word. Smaller tables (task 9) make memory faster in proportion.
- **The compaction: not removed.** The spruce's tables are sized from the most rows a build holds when compacted (`ARENA`). So at cycle 37 the projected phytomers pass `TIGHT` (0.85) of their rows and the arena compacts in place, once. The host's pause has two parts:
  - **Before the read:** about 11 ms of recording, while the device still runs cycle 37.
  - **The read:** the census read after the compaction (`cut`), 14–27 ms waiting for cycle 37 and the compaction's 207 dispatches to finish.
  - **Dropping the read** (`PROBE_NOCUT`, the pre-compaction census used instead): the spruce went 6.0 ms *slower* (the next cycles' workgroups are bounded by the larger counts) and Telperion (two compactions) 10.0 ms faster (`raw/t8/warm-nocut.txt`). Without the `cut` the next move check reads counts the compaction already cut, so a needless move becomes likely.
  - **Sizing it in:** the tables would be sized to the uncompacted rows, about a fifth more phytomers' and axes' rows, near 100 MB of peak, to save about 5–10 ms.
  - Neither is a clear gain, so the compaction stays.
- **The settle's, plan's and trunk's readbacks: the reads of one submission batched, taken.**
  - **Batched:** the sweeps' tally, kept counts and longest lengths are now one read (`settle_run.rs`). Each lay's faults and first error are one read too (`settle_place.rs`'s `laid`), with the same refusal by name. That leaves 7 of the settle's 10 reads.
  - **Warm `tree::drawn`:** spruce 385.3 to 380.5 ms (medians 381.3 to 380.1; the base's spread 12.5 from one slow run), Telperion 837.5 to 833.2 (sd 4.0, 2.3; `raw/t8/warm-reads.txt`). 24 of 24 trees are byte-identical.
  - **The rest stay:** each of the remaining reads decides what the next submission records (the depths, the sweeps, whether to lay again). The plan's and trunk's single reads wait on the device work before them.
- **The spruce's 80.8 MB without a label: named, kept.** Charged under its own labels now (`buffer.rs`'s `allocate`), it is the foliage's level lists (60.4 MB) and selection scratch (20.4 MB). The selection writes both every frame and the draws read the lists, so both are read data:
  - **The scratch** is the selection's ranks and per-group counts.
  - **The lists** are one list per detail level, each with room for every leaf (`select.wgsl`: `level * stride + offset`), because the draws bind a level at a fixed offset. One list of all leaves, levels end to end, would hold 15 MB.
  - **What that needs:** the draws reading each level's start from the selection's prefix, either indirect draws with a first instance (not in WebGPU's core) or a per-level start word bound at a fixed offset. It is a renderer change behind the approved trees' look, proposed as its own spec rather than taken here.

## What remains

On the spruce, 67 % of the warm build is the cycles' device time and about 16 % the settle, plan and trunk's device time. Of what is not kernel arithmetic, the dispatch launches (about 26 ms) and the memory made (about 33 ms) remain: the first measured as no better folded (task 4), the second waiting on the host's trade-off.
