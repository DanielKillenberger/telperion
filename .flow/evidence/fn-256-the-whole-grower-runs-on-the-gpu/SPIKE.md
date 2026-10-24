# fn-256 spike: one busy cycle replayed on the device (COUNCIL.md early test 3, native) (worker, 2026-10-11)

**Verdict on the stop condition: dependency overhead alone does not exhaust the warm build budget.** On the worst schedule and the slowest form of dispatch, Telperion's 82 cycles would spend 163 ms waiting on dependencies against F4's 1.2 s, about 14%. The beech's 40 cycles would spend about 84 ms against 0.8 s. No dispatch lost the device, and every integer phase output agreed with the CPU. Every real output stayed within its last places. Whether that meets R4's agreement is the host's call (see Design calls).

This is the council's early test 3 and nothing beyond it. Two parts are not here: the Chrome probe (run apart) and the full port.

## How it ran

- **Device:** NVIDIA GeForce RTX 3080 under Vulkan (driver 610.57.04), native wgpu 30.
- **Tools:**
  - `telperion_space::grow_capture` (`grow/cycle/capture.rs`) grows a preset through the one cycle with the quantised deposit. It writes the cycle before the last's light phase and the last cycle's hierarchy, normalisation and fate phases, inputs and the CPU's outputs both.
  - `crates/telperion-render/examples/gpucycle/` replays them in throwaway kernels wired into nothing.
- **Captures:** the oak at 80, seed 1, and Telperion at 82, seed 42. Each busy cycle is the tree's last.
  - Oak: 1,473,170 axes, 130,165 surviving apexes, 380,243 leaves, a 29,376-node lattice, 25 ancestors at most.
  - Telperion: 1,463,911 axes, 233,233 apexes, 823,805 leaves, a 61,600-node lattice, 62 ancestors at most. Telperion's site has no extinction, so its lattice is all ones. The device still sweeps it, so the timings show what a lit Telperion would pay.
- **Agreement:** each phase reads the CPU's own inputs, so its differences are its own and not inherited.
- **Timing:** the whole cycle chained on the device in one submission, each phase reading the one before. Each dispatch sits in its own compute pass with begin and end timestamps. Times are the median of 9 runs.
- **Captures and logs:** `raw/` (ignored). The final runs are `raw/final-oak.txt` and `raw/final-telperion.txt`.

The two hierarchy schedules:

- **A, ancestor walks:** a fixed dispatch count.
  - Bottom-up sums: each axis climbs its ancestors and adds its value in 2^-32 units to each one's 64-bit integer sum. That takes two atomic words, the carry going to the high one.
  - Top-down walks: each axis climbs into a path and replays root first. Shedding decides locally first. The split computes each child's share with a vigour of one, then multiplies the shares down the path.
- **B, bounded subtrees:** greedy connected blocks of at most 1,024 axes, one workgroup each. A barrier separates each local height (bottom-up) or depth (top-down), and the dispatch boundaries are block levels. Every axis does the CPU's own operation in the CPU's order.

The two sweeps both take `light.rs::swept`'s recurrence. It reads the layer above's depth bilinearly where the ray meets it, then adds the trapezoid of densities:

- **Wg:** one workgroup per direction loops the layers, with a barrier between layers.
- **Layer:** one dispatch per layer covers every direction's nodes.

Each sweep is followed by one dispatch that sums the directions in order.

## Agreement with the CPU phases

| Phase | Output | Oak 80 | Telperion 82 |
|---|---|---|---|
| Light | box, quanta (u32), densities (bits) | exact | exact |
| | deposit overflow | none | none |
| | lattice: device sweep (f32) vs CPU (f64, rounded once) | 41.6% bit-equal, max abs 3.6e-7 | exact (unlit) |
| | reads on the CPU's lattice | exact (see FMA below) | exact |
| | reads on the device's lattice | 42.6% bit-equal, max abs 2.4e-7 | exact |
| Balance totals | A: integer sums vs the host's integer sums | exact | exact |
| | A: vs the CPU's f32 (light; upkeep) | max rel 7.7e-6; 8.6e-3 | 9.5e-6; exact |
| | B: vs the CPU's f32 | exact | exact |
| | living | exact | exact |
| Shedding | draws (deciding laterals) | exact (142,465) | exact (258,917) |
| | shed flags | exact | exact |
| | kept share | exact | exact |
| | memory | 99.0% bit-equal, max abs 1.2e-7 | 99.999%, 6.0e-8 |
| Allocation totals | A: integer sums vs host's | exact | exact |
| | A: vs CPU f32 | max rel 7.7e-6 | 9.5e-6 |
| | B: vs CPU f32 | exact | exact |
| Vigour split | incoming, A / B | 90.9% / 91.0% bit-equal, max rel 1.8e-6 / 2.1e-6 | 83.4% / 83.7%, 2.3e-6 |
| | bud vigour, A / B | 91.8% / 91.8%, max rel 1.8e-6 / 2.2e-6 | 85.3% / 85.6%, 2.3e-6 |
| Normalisation | PA sums: integer words vs the host's | 5 of 30 words differ | exact |
| | PA sums vs CPU f32 | max rel 1.1e-4 | 2.2e-4 |
| | sizes | 91.2% bit-equal, max rel 1.2e-4 | 99.4%, 6.0e-8 |
| Fate | survival draws | exact (130,165) | exact (233,233) |
| | survival thresholds | differ by at most 2.1e-7 | at most 2.4e-8 |
| | survival decisions | exact | exact |
| Births | first index of each apex's births (exclusive scan) | exact (21,588 bearers) | exact (35,543) |

- **Every integer output agrees to the bit:** hashes, draws, shed flags, decisions, the deposit's quanta and the box, the densities, and the births' placement. So do the kept shares, the product of presences down each axis.
- **B reproduces the CPU's f32 sums to the bit at every block cap** (256, 1,024 and 4,096, on Telperion). The block size changes no result.
- **A's integer sums are exact and independent of order.** Against the CPU's f32 they differ by up to 0.86% (the oak's upkeep at a large subtree). That error is the CPU's sequential f32 sum's. The integer sum is the exact one.
- **The reals that are not bit-equal differ in their last places.** The largest relative difference outside the normalisation is 2.3e-6. The normalisation's 1.2e-4 comes from its PA sums, summed as integers on the device and in f32 order on the CPU.
- **Fused multiply-add explains the reads.** The weights agree to the bit: the deposit, which takes them exactly, agrees on every node. The read then sums `weight × light` in corner order:
  - First version: 71.5% of reads bit-equal.
  - The same kernel with one added branch that passes the product through its bits before the add: 100%, whether or not the branch is taken.
  - So the driver fuses the multiply into the add, and it decides by the code's shape. The split, memory and sizes also divide, and WGSL holds division to 2.5 ulp. Their causes were not traced one by one.
- **The thresholds differ because the CPU's survival takes viability in f64** (`decide::survival(viability: f64, ..)`), while the device holds the PA table's f32.
- **The oak's 5 integer PA-sum words** come from `(v/p)^ψ`, which differs from the host's contract `exp2`/`log2` in its last places for some buds. This was not traced further.

## Timing (busy cycle, median of 9)

| Tree | Schedule + sweep | Dispatches | Encoding ms | GPU first-to-last ms | In dispatches ms | Fenced wall ms | Longest dispatch ms |
|---|---|--:|--:|--:|--:|--:|--:|
| oak 80 | A + Wg | 22 | 0.02 | 5.94 | 5.82 | 6.26 | 1.81 (balance totals) |
| | A + Layer | 45 | 0.02 | 6.13 | 5.87 | 6.56 | 1.81 (balance totals) |
| | B + Wg | 58 | 0.03 | 4.46 | 4.14 | 4.96 | 0.29 (normalise) |
| | B + Layer | 81 | 0.08 | 4.64 | 4.17 | 5.57 | 0.29 (normalise) |
| Telperion 82 | A + Wg | 22 | 0.04 | 6.26 | 6.13 | 6.87 | 1.49 (shed) |
| | A + Layer | 49 | 0.05 | 6.43 | 6.14 | 7.33 | 1.49 (shed) |
| | B + Wg | 50 | 0.03 | 5.76 | 5.46 | 6.39 | 0.52 (normalise) |
| | B + Layer | 77 | 0.07 | 5.60 | 5.16 | 6.58 | 0.48 (normalise) |

### GPU time per phase, ms (dispatches)

| Phase | Oak A + Wg | Oak B + Wg | Telperion A + Wg | Telperion B + Wg |
|---|--:|--:|--:|--:|
| light: box | 0.063 (1) | 0.058 (1) | 0.125 (1) | 0.125 (1) |
| light: deposit and densities | 0.189 (2) | 0.178 (2) | 0.111 (2) | 0.112 (2) |
| light: sweep Wg and combine | 0.072 (2) | 0.065 (2) | 0.128 (2) | 0.127 (2) |
| light: sweep Layer and combine | 0.118 (25) | 0.107 (25) | 0.163 (29) | 0.150 (29) |
| light: read | 0.021 (1) | 0.020 (1) | 0.033 (1) | 0.034 (1) |
| balance: own values | 0.048 (1) | 0.049 (1) | 0.050 (1) | 0.051 (1) |
| balance totals | 1.879 (2) | 0.822 (11) | 0.936 (2) | 0.911 (9) |
| shedding | 1.317 (2) | 0.428 (11) | 1.581 (2) | 0.668 (9) |
| allocation: lit presence | 0.041 (1) | 0.041 (1) | 0.041 (1) | 0.043 (1) |
| allocation totals | 0.304 (2) | 0.639 (11) | 0.787 (2) | 0.718 (9) |
| vigour split | 1.279 (2) | 1.277 (11) | 1.552 (2) | 1.897 (9) |
| normalisation | 0.581 (2) | 0.536 (2) | 0.744 (2) | 0.742 (2) |
| fate | 0.010 (1) | 0.010 (1) | 0.017 (1) | 0.017 (1) |
| births (scan) | 0.015 (3) | 0.015 (3) | 0.021 (3) | 0.020 (3) |

The Layer sweep row is taken from the A + Layer and B + Layer runs.

- **A's cost is its root contention and its path stacks.** Its balance walk on the oak is the longest single dispatch (1.8 ms): 1.47M axes each adding at up to 25 ancestors. Its shedding walk on Telperion, with paths of up to 63, is 1.5 ms.
- **B is the faster schedule on both trees,** by 1.5 ms on the oak and 0.5 ms on Telperion. Its longest dispatch is 0.3 to 0.5 ms. The split is B's slowest phase: each axis runs its attachments in sequence (p99 29 to 54, at most 234).
- **The normalisation's PA sums put up to 233k atomics on 15 groups' words** (0.5 to 0.7 ms). A workgroup reduction before the atomics would remove most of that; it was not tried.
- **The two sweeps give the same lattice** (the same bit-equal share and largest difference against the CPU). At these lattices the Wg form, at 2 dispatches, is faster than the Layer form, at 25 to 29: 0.07 against 0.12 ms on the oak and 0.13 against 0.16 on Telperion.

**Block cap (Telperion, B + Wg).** The sums stayed bit-exact at every cap:

| Cap | Blocks | Levels | Dispatches | GPU first-to-last ms |
|--:|--:|--:|--:|--:|
| 256 | 17,993 | 13 | 66 | 6.18 |
| 1,024 | 5,961 | 9 | 50 | 5.76 |
| 4,096 | 1,486 | 7 | 42 | 5.71 |

### Compile, preparation, upload, scratch

- **Compile:** six modules and their 23 pipelines.
  - Cold, with the driver's disk cache off (`__GL_SHADER_DISK_CACHE=0`): 354 ms on the oak's run and 326 ms on Telperion's. Shedding's module is the largest at 103 to 108 ms.
  - Warm, with the driver's cache on, on a second run: 24 ms.
- **Host preparation** (single-threaded Rust here):
  - Children by index and by (node, index): 32 to 43 ms.
  - B's blocks: 136 to 164 ms.
  - A device grower must keep both incrementally, at birth. Their cost on the device is not measured here (see Design calls).
- **Upload:** 40 to 49 ms for 315 to 330 MB. That holds every input, the CPU's reference arrays and all scratch, so it is not what a resident grower would move.
- **Scratch:**
  - A: 82 MB, 56 bytes per axis (two 64-bit integer sums per axis for each of two totals, local decisions, incoming).
  - B: 12 MB of blocks plus 5.9 MB of rest, 12 bytes per axis.
  - Sweep Wg: 1.1 to 2.4 MB. Sweep Layer: 2.1 to 4.4 MB.

## The empty dependency chain (80 × 8 = 640 dispatches, one submission)

Each dispatch reads its workgroup count from the dispatch before as an indirect argument, and writes the next one's. All 640 steps ran, so wgpu kept the dependency.

| Form | Encoding ms | Fenced wall ms | GPU first-to-last ms | Per dispatch (GPU / wall) |
|---|--:|--:|--:|--:|
| Indirect chain, one pass | 0.03 | 4.87 | 2.86 | 4.5 / 7.6 µs |
| Indirect chain, a pass each (timestamped) | 0.19 | 16.52 | 8.83 | 13.8 / 25.8 µs |
| Direct no-ops, one pass | 0.03 | 1.11 | 0.48 | 0.75 / 1.7 µs |

## The stop condition

COUNCIL.md stops the full port if dependency overhead alone exhausts the build budget, if correctness fails, or if a dispatch causes device loss.

**Dependency overhead** is the dispatches a build makes times their cost per dispatch. The cost per dispatch is the chain's: one pass at 7.6 µs of wall, or a pass each at 25.8 µs. The dispatches a build makes are the busy cycle's count times the cycles. That over-counts, because earlier cycles have fewer block levels and layers.

| Build (F4 warm budget) | Schedule + sweep | Dispatches | At 7.6 µs | At 25.8 µs | Share of budget, worst |
|---|---|--:|--:|--:|--:|
| Telperion, 82 cycles (1.2 s) | A + Wg | 1,804 | 14 ms | 47 ms | 3.9% |
| | B + Layer | 6,314 | 48 ms | 163 ms | 13.6% |
| Beech, 40 cycles (0.8 s; the oak's counts as proxy) | A + Wg | 880 | 7 ms | 23 ms | 2.8% |
| | B + Layer | 3,240 | 25 ms | 84 ms | 10.5% |

- **Overhead:** not exhausted. The worst case is 14% of the budget.
- **Correctness:** no failure on this cycle. Every decision and every integer agreed; the reals agreed within their last places, as the table shows.
- **Device loss:** none in any run. The longest dispatch was 1.8 ms.

**The full port is not stopped by this test.**

**For scale, the execution these phases would take.** Summed over the build, Telperion holds 16.5 busy cycles' worth of axes and the oak 18.8. If each phase's time grows with the axes held, the phases replayed here would take about 85 to 115 ms of device time per build.

That excludes the two phases this spike did not replay, which are the CPU cycle's largest on Telperion:

- **Units:** growing each apex's growth unit, its nodes and its buds. On the CPU this is 3.01 s of 11.95 s.
- **Incremental layout:** the sketch, 2.03 s on the CPU.

Fate here is the unit's survival decision only, and births are the scan that places the new axes.

## Design calls (NEEDS_HUMAN)

These are for the host. The spike measured them and did not decide them.

1. **The schedule.**
   - B is faster here and reproduces the CPU's f32 sums and walks to the bit, at any block cap.
   - A is exact only against an integer reference. Choosing A means the CPU reference takes balance and allocation totals as integer sums too. COUNCIL.md has "carbon and allocation sums are not quantised".
   - B needs connected blocks kept as the tree grows. Their upkeep on the device, and the children-by-attachment lists both schedules read, were built on the host here and not measured on the device.
2. **The standard of agreement for reals.**
   - With the CPU's operations in the CPU's order, the device still differs in last places. The cause is the driver's fused multiply-add, which it decides by the code's shape, and WGSL's 2.5-ulp division.
   - Bit-equality would need a contraction barrier the driver cannot see through. The one tried, passing the product through its bits, cured the read. Division would need correct rounding.
   - The alternative is to state R4's CPU–GPU agreement as a tolerance.
3. **Survival's viability.** The CPU decides it in f64 and the device from the table's f32. Decisions agreed on 363,398 apexes here, but nothing guarantees they will. The contract would take viability as the f32 the table holds on both sides.
4. **The normalisation's sums.** They are integer on the device and an f32 sum in bud order on the CPU, so sizes differ by up to 1.2e-4 relative. This joins call 1: which sum is the reference.
