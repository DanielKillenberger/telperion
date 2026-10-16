# fn-257: where a tree's device build spends its time (worker, 2026-10-16)

Measurement only; no production path changed. The owner asked whether the 0.25–0.45 s of device growth (`tree::drawn`, PROBE.md's "Device drawn") is data movement or waits, dispatch overhead and setup. Spruce, birch and oak at age 40, seeds 1 and 1234, warm (the third build of each), RTX 3080, one GPU process under `telperion-heavy`.

- **The test:** `growth_profile` (`crates/telperion-render/src/generation/cycle/growth_profile_tests.rs`), `cargo test --profile ci -p telperion-render --lib growth_profile -- --ignored --nocapture` with `PROFILE_OUT` set. Raw reports in `raw/profile/` (ignored); `profile.py raw/profile` adds the byte estimates.
- **What it does per case:** times `tree::drawn` whole; then runs the same steps one by one, counting each stage's submissions, dispatches, host waits and buffers made, and each growth cycle's host recording, census wait and readying (moves and compactions); then a third build with every pass timestamped.
- **Its instrumentation, test builds only:** counters in `trace.rs` (`COUNTED`), called from `io::buffer`, `io::Pass::run`, the census wait (`build.rs::mapped`) and the existing wait and submit hooks; a `cfg(test)` per-cycle host field on `Grew` (`build_grow.rs`).
- **(m)** measured, **(c)** computed. Bytes are fn-256's `roofline.py` `records` from each cycle's census: source-level words per record, an upper estimate within about 2x, the light counted lit. Compaction's bytes are not counted.
- **Stamped times overstate small dispatches:** with a profile on, every dispatch runs in a timestamped compute pass of its own (`io::Pass::run_all`), so the stamped build's growth took 302–533 ms against 176–358 ms unstamped. Kernel times are read from it; wall times from the unstamped build.

## The whole build (m)

| Tree | `tree::drawn` warm | tables (CPU) | grow | settle | plan | dress + trunk |
|---|--:|--:|--:|--:|--:|--:|
| spruce s1 | 414 | 19 | 328 | 33 | 10 | 8 |
| spruce s1234 | 410 | 19 | 331 | 37 | 10 | 8 |
| birch s1 | 383 | 10 | 333 | 24 | 1 | 10 |
| birch s1234 | 415 | 11 | 358 | 25 | 3 | 13 |
| oak s1 | 227 | 9 | 176 | 26 | 1 | 16 |
| oak s1234 | 265 | 9 | 197 | 35 | 1 | 19 |

Growth is 77–86% of every build.

## Growth (m)

| Tree | grow ms | host recording | host waiting on the census | host moving and compacting | moves / compactions | dispatches | of them in a pass of their own | submits | buffers made / MB | GPU kernel ms (stamped) | of it, dispatches under 20 µs |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| spruce s1 | 328 | 44 | 166 | 52 | 1 / 2 | 15,765 | 1,121 | 52 | 557 / 1,691 | 234 | 13,452: 74 ms |
| spruce s1234 | 331 | 44 | 174 | 51 | 1 / 2 | 15,872 | 1,121 | 52 | 557 / 1,691 | 243 | 13,488: 74 ms |
| birch s1 | 333 | 41 | 16 | 252 | 23 / 15 | 15,975 | 3,812 | 165 | 4,693 / 1,421 | 129 | 14,663: 70 ms |
| birch s1234 | 358 | 42 | 22 | 266 | 23 / 15 | 16,195 | 3,812 | 168 | 4,656 / 1,492 | 143 | 14,672: 71 ms |
| oak s1 | 176 | 41 | 100 | 0 | 0 / 0 | 13,883 | 707 | 42 | 95 / 838 | 141 | 12,092: 60 ms |
| oak s1234 | 197 | 41 | 121 | 0 | 0 / 0 | 14,057 | 707 | 42 | 95 / 838 | 162 | 12,066: 61 ms |

- **Host waits:** one census read a cycle (40 a tree), one cycle late: the host records and submits cycle n, then waits for cycle n−1's census. Recording is about 1 ms a cycle and is hidden behind the GPU wherever the host waits. The birch adds 15 readbacks and 116 waits for the queue to empty, all inside its moves.
- **Where the GPU sits idle between cycles (stamped build):**
  - Oak and spruce: under 0.05 ms between consecutive cycles. Their per-cycle census read does not starve the GPU; the host waits because the GPU is the slower side.
  - Birch: 17 gaps of 6.7–11.3 ms, about 104 ms in all, at the cycles after which it moved or compacted.
  - All three show a gap of 12–32 ms before cycle 2 in the stamped build only (query sets made on first use); the unstamped host recorded cycles 1 and 2 in 1.8 and 3.3 ms.
- **Arena sizing:** the birch's arena starts small and moves 23 times; each move makes new buffers, copies the tables across, waits for the queue and replans. That is 252–266 ms, 75% of its growth. The spruce moves once and compacts twice (52 ms). The oak's hint holds it whole.
- **Over-dispatch:** bounded dispatches were given 2.3–12.6 million workgroups for 0.66–4.6 million needed, about 3.5x. On this card an empty workgroup costs almost nothing (the floor below: 1,024 workgroups against 1 adds 0.3–0.5 µs a dispatch).

## Kernel time by phase, and the bytes it moves

GPU ms (m) are stamped; GB (c) is an upper estimate; the last column is the time those bytes take at the card's measured 705 GB/s copy rate (ROOFLINE.md).

| Tree | units and wake | balance and allotment | layout | sizing | retirement | light | compaction | GB moved (c) | at 705 GB/s |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| spruce s1 | 81.6 | 62.2 | 52.3 | 12.3 | 10.6 | 7.5 | 7.7 | 18.6 | 26 ms |
| spruce s1234 | 85.7 | 62.1 | 56.3 | 12.3 | 10.8 | 8.0 | 7.6 | 19.9 | 28 ms |
| birch s1 | 36.2 | 43.0 | 12.0 | 11.6 | 10.7 | 4.0 | 11.3 | 3.7 | 5 ms |
| birch s1234 | 41.3 | 46.7 | 15.0 | 11.8 | 11.5 | 4.2 | 12.2 | 4.9 | 7 ms |
| oak s1 | 43.5 | 51.5 | 17.0 | 12.2 | 10.9 | 5.2 | 0 | 4.2 | 6 ms |
| oak s1234 | 52.3 | 56.0 | 23.9 | 12.2 | 12.2 | 5.6 | 0 | 7.1 | 10 ms |

Per phase, the spruce's units and wake phase moves the most: about 11–12 GB, 16–17 ms at the copy rate, inside 82–86 ms of kernel time.

## The dispatch floor (m)

10,000 dependent dispatches of a kernel that bumps one word, warm, the best of five (`raw/profile/floor.md`):

| Passes | Workgroups | Submits | ms | of it recording and submitting | µs a dispatch |
|---|--:|--:|--:|--:|--:|
| one a dispatch | 1 | 1 | 131.3 | 68.5 | 13.1 |
| one for all | 1 | 1 | 17.2 | 6.4 | 1.7 |
| one a dispatch | 1,024 | 1 | 128.7 | 66.0 | 12.9 |
| one for all | 1,024 | 1 | 21.8 | 6.7 | 2.2 |
| one a dispatch | 1 | 40 | 53.0 | 49.5 | 5.3 |
| one for all | 1 | 40 | 10.6 | 9.4 | 1.1 |

So a tree's 14–16 thousand dispatches cost about 1–2 µs each to launch in shared passes and 5–13 µs in passes of their own. That is roughly 20–35 ms a tree, more for the birch's 3,812 passes of their own.

## The drawn records (m)

| Tree | axis records (48 B) | phytomer records (152 B) | used | buffers |
|---|--:|--:|--:|--:|
| spruce s1 | 287,564 | 1,369,978 | 222.0 MB | 222.0 MB |
| spruce s1234 | 289,897 | 1,396,926 | 226.2 MB | 226.2 MB |
| birch s1 | 36,968 | 298,775 | 47.2 MB | 47.2 MB |
| oak s1 | 52,685 | 401,189 | 63.5 MB | 63.5 MB |
| oak s1234 | 111,570 | 721,697 | 115.1 MB | 115.1 MB |

- **The buffers are full.** PROBE.md's 217–226 MB for the spruce is data, not headroom: about 1.4 million phytomer records of 38 words each.
- **What a phytomer record holds** (`dress.wgsl`, "A drawn phytomer"), 38 words of 4 bytes:
  - its node's position (3), radius, start radius and base radius (3);
  - flags, the node it is drawn as, the node's run, its laid axis (4);
  - the parent node's position (3), codominance, id, distance along the wood, parent's id (4);
  - its wood's frame as laid, heading and side (6);
  - branch, key, wood as grown (3), the phytomer's key (2), flowering share (1), cluster spread (1);
  - its leaf retention: metres before the boundary and the two retentions (3).

  That is 37 of the 38 words; the layout comment does not name the last.
- **Axis records** are 12 words (`resident/fold.rs`, `CA`).
- **The build's own buffers are larger still.** Growth makes 0.84–1.69 GB of buffers for each tree and holds up to 1.59 GB at its peak (the spruce: pencils 275 MB, axes 227 MB, counts 211 MB, units 153 MB). Settle makes another 650–890 buffers (72–437 MB) with about 600–820 initial writes.

## Verdict

The growth is not limited by data movement. Even the upper estimate of every byte the spruce's cycles touch takes 26–28 ms at the card's copy rate, against 328–331 ms of growth; the oak's and birch's take 5–10 ms against 176–358 ms.

Where the wall time goes, by tree:

| Share of growth | Spruce | Birch | Oak |
|---|--:|--:|--:|
| Arena moves and compactions (host, GPU idle between cycles) | 16% (52 ms) | 75% (252–266 ms) | 0% |
| GPU kernels at or over 20 µs | about 49% (160–169 ms stamped) | about 18% (59–72 ms) | about 45% (80–101 ms) |
| GPU dispatches under 20 µs (launch and latency) | about 22% (74 ms) | about 20% (70 ms) | about 33% (60 ms) |
| Data movement at the copy rate (c, upper) | at most 8% (26–28 ms) | at most 2% (5–7 ms) | at most 5% (6–10 ms) |
| Host recording (hidden wherever the host waits) | 44 ms, hidden | 41 ms, partly exposed | 41 ms, hidden |
| Census round trips idling the GPU | under 1 ms | inside the moves | under 1 ms |

The stamped columns overstate the unstamped build. Their ratios are the measure: the GPU-bound oak's unstamped growth (176–197 ms) is 1.2x its stamped kernel sum (141–162 ms).

### The levers this implies, each bounded by the measurement

1. **Size the arena from the parameters before growing, so no build moves** (fn-270 R4's second lever).
   - Birch: saves most of its 252–266 ms of moving. Its growth would then be held by its kernels (129–143 ms stamped), so about 150–200 ms saved, bringing the birch's whole build from about 383–415 ms to about 200–250 ms.
   - Spruce: at most 52 ms. Oak: nothing.
   - Bounded above by the moving time.
2. **Fewer small dispatches: fuse the per-level and single-workgroup steps.** 12–14.7 thousand dispatches a tree run under 20 µs and take 60–74 ms stamped. At the floor's 1–2 µs a launch, merging them would save roughly half, about 30–40 ms a tree, bounded by the 60–74 ms.
3. **Real kernel work** (80–169 ms a tree) is the remaining floor. The spruce's units and wake phase (82–86 ms) and every tree's balance and allotment phase (43–62 ms) are the largest. ROOFLINE.md found these run 14–270x over their roofline time, mostly on latency, so speeding the kernels themselves is a separate design question.
4. **Reuse buffers across builds.** Growth makes 95–4,700 buffers (0.84–1.69 GB) and settle about 650–890 per tree. Their cost is not measured apart: inside the moves for the birch, and bounded by the 24–37 ms of settle elsewhere.
5. **Cache the CPU's tables per family:** 9–19 ms a tree, bounded by that.
6. **Compact records:** at most the data-movement share, about 26–28 ms on the spruce and under 10 ms on the oak and birch even at the upper estimate. The real gain is memory, not time: 222 MB of drawn records and up to 1.6 GB of build buffers for one spruce.
7. **Removing the per-cycle census read** saves under 1 ms on the oak and spruce, because the GPU never waits for it. On the birch it only matters as part of the moves (lever 1).

### What would bring the site's three species near 0.1.4, from these numbers alone

| Tree | 0.1.4 | `tree::drawn` today | with levers 1 and 2 (estimate) |
|---|--:|--:|--:|
| oak | about 0.29 s | 0.23–0.26 s | about 0.19–0.24 s |
| birch | about 0.35 s | 0.38–0.42 s | about 0.17–0.24 s |
| spruce | about 0.24 s | 0.41 s | about 0.33–0.36 s |

These estimates add bounded savings and were not measured together. The spruce stays over 0.1.4 on the levers here: its floor is about 160–170 ms of real kernel work plus 19 ms of tables, 33–37 ms of settle and 8–10 ms of plan.
