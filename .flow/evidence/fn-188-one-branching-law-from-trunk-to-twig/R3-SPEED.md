# fn-188 R3 speed: round 3's trees, faster, 2026-10-03

## Setup

- **Code:** round 3's probe (`4c59e337`) plus a bench mode, committed as `1e36a438` and reverted in `32dfe6d3`. It carries R1's `FN188_NO_TWIGS` switch to time the scaffold alone.
- **Machine:** Ryzen 9 5950X (16 cores, 32 threads), release profile, no other load.
- **Timing:** warm median and p95 of 7 or 9 runs, plus cold (the first run in the process). Probe ms covers the marker fill and the growth.
- **Shipped skeleton and scaffold ms:** `skeleton_ms` of the full build and of the twig-free build, fresh in the same process.
- **Identity:**
  - Each run hashes every node's parent, position and radius (FNV-1a over the f64 bits).
  - The tree grows from the full build's scaffold, as round 3's did. The twig-free build has the same scaffold positions and parents but slightly different radii.
  - Rows are in `raw/speed/`.

## Same trees

- **Step 1 fixes the order in which each bud sums its marker directions** (by marker id). Round 3 summed in first-touch order, which depends on the grid's storage order.
- **What differs from round 3:** only low bits of positions. Node count, tip count, and every perceive, occupy and sight visit count are equal on all 16 preset-seeds. Every traced measure agrees to 4 decimals (`raw/speed/speed-s0all.jsonl` against `speed-s1all.jsonl`).
- **Raw order still reproduces round 3's digests,** recorded in `digest-r3.txt`.
- **Every later step reproduces step 1's digests:**
  - the final configuration on all 16 preset-seeds, at 1 and 16 threads;
  - steps 2 to 6 on beech, oak and laurelin s1, including 2, 4 and 8 threads.

## Cost of each stage (beech G2 s1, round 3 code)

| Stage | ms | Work |
|---|--:|---|
| perception | 55 | 122,417 bud evaluations; each scans the 27 grid cells around the bud, 6.0M marker visits (about 49 per bud) |
| extend | 37 | about 47k metamers; each runs an occupancy query (3.9M visits) and a sight query (1.6M visits) |
| basipetal and allocate | 11 + 2.4 | 40 full passes over all nodes (7.3M node visits) |
| marker fill | 13.5 | about 97k draws, each one `Envelope::contains` with lobes |

Oak and laurelin have the same shape: about 2.2 and 2.6 times the beech's work (13.2M and 15.8M perception visits).

## Gain of each step (warm median ms, s1: beech / oak / laurelin)

| Step | beech | oak | laurelin |
|---|--:|--:|--:|
| 0. round 3 | 127.7 | 288.3 | 336.7 |
| 1. canonical sum order | 129.3 | 310.9 (p95 379, noisy) | 336.8 |
| 2. skip the shed pass and the per-cycle count and demand sums when shedding is off | 123.3 | 277.1 | 325.0 |
| 3. grid cell = perception radius × 1 / 0.5 / 0.4 (sphere-culled) | 127 / 200 / 331 | – | – |
| 4. flat marker layout: one array in cell order, dead markers swapped behind the live | 108.8 | 239.2 | 291.2 |
| 5. exact cell ranges per query (occupancy visits 3.9M → 0.8M) | 94.9 | 202.2 | 250.3 |
| 5b. cell × 0.7 / 0.5 with exact ranges | 102.4 / 112.7 | 218.9 / 246.2 | 272.3 / 303.5 |
| 6. threaded perception scan and marker fill, 2 / 4 / 8 / 16 threads | 86.0 / 73.9 / 67.0 / 65.7 | 184.6 / 160.9 / 142.9 / 148.7 | 228.3 / 198.7 / 185.8 / 174.0 |
| 7. stop once a cycle grows nothing, 1 thread | 84.2 | 194.8 | 232.9 |
| 7. the same, 16 threads | **55.0** | **139.4** | **157.9** |

- **Step 3:** smaller cells cut visits but cost more in per-cell overhead. Markers are sparse (about 6 per 5-unit cell), so the original size stays.
- **Step 6 scales poorly.**
  - The parallel scan falls from 34.2 to 9.3 ms on the beech, including starting threads every cycle.
  - The rest of perception stays sequential, about 9 ms: the bud list, the merge in bud order and the sorted sums.
  - Capping threads at one per 256, 1,024 or 4,096 buds did not help.
- **Step 7 is exact.** With no growth and no shedding, the markers and wood do not change, so every later cycle would repeat the last one. Presets stop after 8 to 31 cycles instead of 40.
- **Overall:** 2.3x on the beech and 2.1x on oak and laurelin. Single-threaded, 1.5x, 1.5x and 1.4x.

## Every preset, seeds 1 and 7 (final: steps 1, 2, 4, 5, 7)

**Total** is scaffold plus probe at 16 threads. **Memory** is peak RSS of a process that builds the twig-free scaffold and grows the tree once, at 1 / 16 threads.

| preset s | nodes | cycles | round 3 ms | 1 thread: med / p95 / cold | 16 threads: med / p95 / cold | scaffold ms | total ms | shipped skeleton med / p95 | MB |
|---|--:|--:|--:|---|---|--:|--:|---|---|
| ordinary 1 | 30,210 | 18 | 67.4 | 43.3 / 44.0 / 44.4 | 33.0 / 35.6 / 34.8 | 8.4 | 41.4 | 37.9 / 41.5 | 19 / 19 |
| ordinary 7 | 29,536 | 17 | 65.1 | 41.5 / 42.5 / 44.6 | 30.7 / 32.7 / 36.2 | 8.2 | 38.9 | 41.2 / 41.8 | 19 / 19 |
| oak 1 | 111,764 | 31 | 281.6 | 194.8 / 198.5 / 197.6 | 139.4 / 146.7 / 136.0 | 3.1 | 142.5 | 27.7 / 35.6 | 69 / 71 |
| oak 7 | 110,838 | 22 | 269.9 | 183.4 / 194.2 / 195.1 | 119.8 / 124.4 / 131.2 | 4.1 | 123.9 | 36.8 / 37.7 | 59 / 63 |
| spruce 1 | 31,269 | 11 | 53.3 | 30.7 / 30.9 / 31.5 | 22.0 / 23.3 / 23.8 | 14.0 | 36.0 | 33.0 / 37.1 | 25 / 24 |
| spruce 7 | 30,364 | 11 | 53.2 | 30.5 / 30.6 / 30.5 | 24.0 / 26.0 / 23.7 | 13.8 | 37.8 | 31.1 / 33.0 | 21 / 23 |
| birch 1 | 40,380 | 18 | 101.8 | 66.6 / 67.8 / 67.2 | 44.8 / 46.4 / 44.7 | 3.7 | 48.5 | 21.2 / 26.9 | 30 / 29 |
| birch 7 | 41,471 | 20 | 102.8 | 68.5 / 68.7 / 70.0 | 46.4 / 47.4 / 46.0 | 3.9 | 50.3 | 23.2 / 27.9 | 31 / 30 |
| date palm 1 | 249 | 9 | 0.8 | 0.6 / 0.6 / 0.7 | 2.3 / 2.5 / 2.7 | 1.5 | 3.8 | 1.6 / 1.8 | 11 / 12 |
| date palm 7 | 230 | 8 | 0.7 | 0.5 / 0.6 / 0.7 | 2.2 / 2.3 / 2.5 | 1.6 | 3.8 | 1.6 / 1.9 | 11 / 12 |
| telperion 1 | 12,534 | 18 | 28.1 | 19.1 / 19.3 / 19.3 | 16.5 / 17.9 / 17.0 | 6.3 | 22.8 | 333.6 / 341.0 | 12 / 12 |
| telperion 7 | 11,875 | 20 | 26.7 | 18.3 / 18.3 / 18.6 | 16.4 / 17.4 / 18.0 | 4.6 | 21.0 | 48.6 / 51.1 | 12 / 11 |
| laurelin 1 | 132,346 | 28 | 346.3 | 232.9 / 255.5 / 241.8 | 157.9 / 163.5 / 158.4 | 11.8 | 169.7 | 91.4 / 96.0 | 72 / 68 |
| laurelin 7 | 131,831 | 30 | 331.0 | 230.4 / 240.1 / 236.8 | 158.9 / 162.3 / 168.2 | 12.5 | 171.4 | 167.5 / 171.9 | 71 / 70 |
| beech G2 1 | 51,781 | 20 | 129.3 | 84.2 / 85.2 / 87.8 | 55.0 / 57.0 / 57.4 | 3.5 | 58.5 | 18.0 / 24.0 | 34 / 35 |
| beech G2 7 | 49,846 | 20 | 123.8 | 81.4 / 83.2 / 81.1 | 52.0 / 57.1 / 51.3 | 2.7 | 54.7 | 19.4 / 25.1 | 29 / 30 |

- **The round 3 ms column** is a single warm run.
- **Total against the shipped skeleton:**
  - Telperion is 15x and 2.3x faster.
  - Laurelin s7 is about equal (171 against 168 ms).
  - Ordinary is within 10% (0.94x and 1.09x).
  - Spruce is 1.1 to 1.2x slower.
  - Birch is 2.2 to 2.3x, beech 2.8 to 3.3x, laurelin s1 1.9x and oak 3.4 to 5.1x slower.
- **The date palm runs slower on 16 threads** (2.3 against 0.6 ms): the cost of starting the threads.

## The remaining floor (beech s1, 16 threads, 55 ms)

**By stage (ms):**

| Stage | ms |
|---|--:|
| extend | 24.1 |
| perception | 18.2 (9.3 threaded scan, about 9 sequential) |
| basipetal | 3.9 |
| marker fill | 2.7 |
| allocate | 2.0 |
| pipe | 1.9 |
| setup | 0.9 |

**Extend is sequential and now the largest stage.** One shoot's occupancy removes markers that a later bud in the same cycle would see, so growing shoots in parallel changes the tree.

**What would move it further.** These are estimates, not measured:

- **A persistent thread pool** instead of starting threads each cycle: a few ms per tree.
- **Parallel extension with markers frozen during a cycle and conflicts resolved after it:** extend up to about 4x faster, but a different tree.
- **Incremental passes** that touch only the ancestors of buds that changed: up to the 6 ms that basipetal and allocate cost on the beech (17 ms on oak).
- **Fewer markers or fewer metamers change the tree.** Round 3 measured density 0.025 at 65 ms against 128 ms at 0.05, at 33k against 52k nodes. Time follows nodes and bud evaluations roughly linearly.
- **A GPU perception and occupancy pass:** the scan is data-parallel (buds × nearby markers). The sequential merge and extend would remain, so the floor would be about 30 ms on the beech unless extension changes too.

## Files

- `raw/speed/`: `speed-<step>.jsonl` for every step, with digests, stage times and visit counters, and `digest-r3.txt` (round 3's digests).
- No stills: the tree did not change beyond the summation order's low bits.
