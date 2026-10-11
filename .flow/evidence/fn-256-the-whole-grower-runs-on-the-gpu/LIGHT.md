# fn-256 task 2: the light on the device, measured (worker, 2026-10-11)

DESIGN.md sections 1.6, 2 (step 8) and 8 for the light (`Grower::read_light`). This file records what ran and what it measured. It does not extrapolate to a build or judge the budget.

## How it ran

- **Device:** NVIDIA GeForce RTX 3080, Vulkan, native wgpu, through `crate::Gpu`, as UNITS.md.
- **Capture:** `telperion_space::grow_capture_light_at` (`grow/cycle/capture_light.rs`) grows the one cycle to the captured cycle, runs `step` and `sketch` on it, then takes what `read_light` takes and makes:
  - inputs: the leaves (x, y, z, area) and the living apexes' tips in f32, the lit directions' `x/z`, `y/z`, `CELL/z` and share as f64 words, and the extinction;
  - the CPU's outputs, from `Field::quantised_with` and `Field::at`: the box, quanta, densities, lattice and reads.
- **Replay:** `generation/cycle/light_device.rs`, `light.wgsl`, `light_sweep.wgsl`, test-only, registered with the naga check as "cycle light". Seven dispatches, each its own stamped pass:
  1. box (atomic min and max, order-free);
  2. grid (one invocation: size, node limit, capacity);
  3. deposit (integer quanta, `atomicAdd`, overflow detected by the wrap);
  4. densities;
  5. sweep (one workgroup of 256 a direction, the layers looped with barriers, as COUNCIL.md decided);
  6. combine (directions in order, clamped, to f32 once);
  7. read.
- **The software f64** (`soft64.wgsl`): binary64 on u32 pairs in integers only. It does add, subtract, multiply, divide (restoring, 63 bits), floor, f32 to f64, f64 to f32, i32 to f64, u32 to f64 and f64 to i32. Every operation rounds to nearest, ties to even, subnormals included, and overflow goes to infinity. Inputs are finite only, because the sweep reaches no infinity or NaN.
- **The programs** (`light_program.rs`, `light_vm.wgsl`): the sweep's node (`swept` and `bilinear`, then libm 0.2.16's `exp` with its `scalbn`) and `corners`' weights are short instruction lists. They run on a private file of 64 f64 registers in one loop. Each software operation therefore has one call site an entry, and the integer steps between them are hooks: floors, neighbour loads, `exp`'s cases and `k`, stores.
- **The weights in f32** are computed in the same software. Each f32 operation is taken exactly in f64 and rounded to f32 once. Products of two f32s are exact in f64. The differences `s - n` and `1 - f` are exact in f64 too, except where the f32 result is 1.0 either way. So the deposit's weights are the CPU's by construction, with no hardware float.
- **The read** sums `w × light` over the eight corners in hardware f32, in the CPU's order, unguarded (as instructed).
- **Not stamped:** the box's reset and the quanta's clear, by copy and `clear_buffer` before each run.
- **Dispatch sizes:** the densities and combine dispatch over the buffers' capacity, which the test sets to the CPU's node count. Each early-outs past the device's own box. A build would need an indirect dispatch or a capacity bound (open, below).
- **Logs:** `raw/light-soft64-1.txt`, `raw/light-young-final.txt`, `raw/light-young-cold.txt`, `raw/light-busy-2.txt` (warmed, the timings below), `raw/light-busy-1.txt` (unwarmed, superseded), and the fusion experiment `raw/light-young-reads-unfused-experiment.txt`.

## The software f64, on the device

Each operation was run on at least 2^20 inputs, against Rust's f64, libm's `exp` and Rust's `as`. Every word agrees.

| Operation | Inputs | Differ | Paths reached (results) |
|---|--:|--:|---|
| add | 1,048,576 | 0 | 74,868 subnormal, 67,450 zero, 1,180 infinite |
| sub | 1,048,576 | 0 | 37,989 subnormal, 53,845 zero, 7,420 infinite |
| mul | 1,048,576 | 0 | 37,627 subnormal, 404,202 zero, 153,457 infinite |
| div (nonzero divisor) | 1,048,576 | 0 | 63,399 subnormal, 213,329 zero, 78,909 infinite |
| f64 to f32 | 1,048,576 | 0 | f32 subnormals, overflow, exact ties (normal and subnormal) |
| f32 to f64 | 1,044,460 | 0 | every finite f32 bit pattern class |
| floor | 1,048,576 | 0 | |
| f64 to i32 | 1,048,576 | 0 | saturation |
| i32 to f64 | 1,048,576 | 0 | |
| the light's `exp` program | 1,048,576 | 0 | [-750, 0], tiny, [-710, 710], ±4 ulps about each case threshold |

The inputs mix any exponent, near one, subnormal, zeros of both signs, short significands (rounding ties), near the largest f64, and pairs a few ulps apart of either sign (cancellation). The tests are `soft64_tests.rs` (`the_software_f64_is_ieee`, `the_software_conversions_and_floor_are_rusts`, `the_light_programs_exp_is_libms`).

## Agreement

Every word is compared: the refusals, the box, every node's quanta, density and light, and every tip's read.

| Capture | Leaves / tips / lattice (nodes holding area) | Box, quanta, densities, lattice | Reads differing |
|---|---|---|--:|
| oregon-white-oak 16 at 12 (k 0.5) | 8,194 / 2,805 / 1,296 (397) | exact | 890 of 2,805 |
| oregon-white-oak 8 at 5 | 513 / 266 / 100 (64) | exact | 85 of 266 |
| ordinary 16 at 12 (k 1/6) | 2,283 / 882 / 540 (208) | exact | 341 of 882 |
| Telperion 14 at 10 (unlit) | 1,024 / 316 / 392 (0) | exact (all ones) | 0 |
| oregon-white-oak 80 seed 1 at 79 (busy) | 380,249 / 132,098 / 29,376 (9,354) | exact | 37,328 of 132,098 |
| Telperion 82 seed 42 at 81 (busy, unlit) | 823,805 / 233,233 / 61,600 (0) | exact (all ones) | 0 |

- **No refusals:** none were raised in any capture.
- **The reads differ by the driver's fusion.** The differences are at most 1.8e-7, a last place.
  - **The site:** `light_sweep.wgsl:140`, `sum = sum + w * l`. The driver fuses the product into the add.
  - **The proof:** an uncommitted run with the product passed through its bits XORed with a uniform zero agreed in every read of all four young captures (`raw/light-young-reads-unfused-experiment.txt`).
  - **What is committed:** no guard. `young_reads_agree_with_the_cpu` keeps its exact assertion and is ignored with that reason.
  - **Unlit sites are unaffected:** every weight times 1.0 sums exactly.
- **`young_lattices_agree_with_the_cpu` is not ignored and passes.** It holds the box, quanta, densities and lattice of the four young captures exact.

## Timing (busy cycle, median of 9, after a second of untimed runs)

ms of GPU time in each stamped pass. The spike's f32 sweep and combine took 0.07 (oak) to 0.13 (Telperion) ms (SPIKE.md).

| Phase | Oak 80 at 79 | Telperion 82 at 81 (unlit) | Telperion, swept at extinction 0.5 |
|---|--:|--:|--:|
| box | 0.058 | 0.114 | 0.114 |
| grid | 0.003 | 0.003 | 0.003 |
| deposit | 0.596 | 1.181 | 1.175 |
| densities | 0.003 | 0.004 | 0.004 |
| sweep (software f64) | **3.809** | 0.005 (skipped) | **4.047** |
| combine | 0.009 | 0.004 | 0.012 |
| read | 0.203 | 0.346 | 0.348 |
| first stamp to last | 4.721 | 1.690 | 5.743 |

| | Oak 80 at 79 | Telperion 82 at 81 |
|---|--:|--:|
| Dispatches | 7 | 7 |
| Lattice | 36 × 34 × 24 = 29,376 | 50 × 44 × 28 = 61,600 |
| Leaves, tips | 380,249, 132,098 | 823,805, 233,233 |
| Longest dispatch | sweep, 3.8 ms | deposit 1.2 ms (unlit); sweep 4.0 ms (swept) |
| Resident (inputs, box, quanta, densities, lattice, reads) | 9.08 MB | 18.59 MB |
| Scratch (each direction's light and depths, f64) | 6.35 MB | 13.31 MB |
| The capture's CPU time | 5.5 s | 7.1 s |

- **The sweep, f32 against software f64:** 3.8 ms on the oak against the spike's 0.07 ms.
- **Telperion's leaves carry no area in this capture.** No lattice node holds any quanta, on the CPU as on the device. Its run swept at extinction 0.5 therefore sweeps zero densities: every `exp` takes its tiny path, and the division returns early on a zero dividend. Its 4.0 ms over 61,600 nodes is a lower bound for a lit Telperion; the oak's 3.8 ms over 29,376 nodes is a full sweep.
- **The deposit is bound by the corner weights' software, not the atomics:** Telperion's deposits no quanta and still takes 1.2 ms for 823,805 leaves.
- **The first run was unwarmed.** It measured the oak's sweep at 20 ms and its deposit at 17 ms, with the device still at low clocks (`raw/light-busy-1.txt`). The test now runs a second of untimed passes first.

## Compile

- **Measured:** around `Kernels::new` through the device's completion.
- **With the driver's disk cache off** (`__GL_SHADER_DISK_CACHE=0`): 0.47 s for the seven entries.
- **Warm:** 0.01 s.
- **Why it is short:** each software operation has one call site in its entry's program loop.

## Open for the host

1. **The read's f32 sum** is fused at `light_sweep.wgsl:140`. It is part of the f32 contract question UNITS.md and LAYOUT.md raise.
2. **The software f64 sweep costs 3.8 ms on the oak's busy cycle** against the f32 sweep's 0.07 ms. DESIGN.md section 8's alternative, the CPU's sweep in f32 under one reduction order, is the host's call.
3. **The densities' and combine's dispatch size** comes from the host's capacity. A build needs the device's node count by an indirect dispatch or a readback. DESIGN.md does not say which.
4. **Telperion's captured leaves have zero area.** Whether that is the preset's intent was not checked.
