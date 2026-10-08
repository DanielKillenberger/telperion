# fn-211 task 11: the stepped executor against today's `grow`, and the device's quotient (worker, 2026-10-07)

Host decisions after task 10 (TASK10-GPU.md). Part 1: the stepped executor becomes the build's only executor if it is identical and never slower on twenty trees. Part 2: a faster fixed-point division in WGSL, with identical bits, and the device run re-measured.

## Answer in brief

- **Part 1: identical everywhere, but not never slower, so the build was not moved.** All 20 trees have the same digest. The stepped executor is faster only on the lit oak (0.75 to 0.78 times). It is level on the unlit oak (0.87 to 0.99), and slower on the beech (1.11 to 1.50) and the spruce (1.53 to 1.90 at low load). Per the host's condition this part stops here. The design question is at the end.
- **Where the lit oak's gain comes from: cores, not less work.** On one thread the stepped executor is 1.09 times slower than `grow` on the lit oak (17.10 s against 15.63 s). On 8 threads it is 0.78 times (6.14 s against 7.84 s). The whole-tree grower runs balance, allotment, the rough layout and the waking serially over the whole tree. The stepped executor runs them once per item, with the items spread over the cores.
- **Part 2: the quotient is Knuth's algorithm D in 16-bit digits, with identical bits.** The proof is 4,194,512 random and boundary cases on the GPU, 0 differ. A model of the steps shows that every path is reached, including 503,785 add-backs. The probe's 17 selectors agree. A mutation (the add-back's correction removed) fails both tests.
- **The kernels are faster, but the device run is still about twice the CPU's.** Layout kernels dropped from 2.74 to 1.87 s (seed 1) and from 3.86 to 3.01 s (seed 7); D16 kernels from 1.39 to 0.70 s and from 1.54 to 1.10 s. The whole device run went from 12.95 to 13.35 s on seed 1 (no change beyond noise) and from 34.34 to 30.74 s on seed 7. The stepped CPU run takes 6.86 s and 14.57 s.

## Part 1: stepped against `grow`

`examples/stepvsgrow.rs`. Each tree gets one cold build of each executor, then three warm builds of each in turn; the table gives the best warm build. The cut is 1,024, with forward order and chunks of 1. Ryzen 9 5950X, 8 Rayon threads. The desk's one-minute load ran from 11 to 24 during the run (`raw/task11/stepvsgrow.txt`).

| Tree | `grow` routes to | grow, s | stepped, s | stepped / grow | roots | same digest |
|---|---|--:|--:|--:|--:|---|
| beech 40 s1 | `grow_fine` | 0.042 | 0.049 | 1.17 | 553 | yes |
| beech 40 s7 | `grow_fine` | 0.066 | 0.073 | 1.11 | 975 | yes |
| beech 80 s1 | `grow_fine` | 0.544 | 0.816 | 1.50 | 7,865 | yes |
| beech 80 s7 | `grow_fine` | 1.128 | 1.525 | 1.35 | 12,829 | yes |
| spruce 40 s1 | `grow_fine` | 0.854 | 1.306 | 1.53 | 21,307 | yes |
| spruce 40 s7 | `grow_fine` | 0.795 | 1.274 | 1.60 | 20,444 | yes |
| spruce 80 s1 | `grow_fine` | 3.513 | 18.169 (6.949 at load 7) | 5.17 (1.90) | 92,703 | yes |
| spruce 80 s7 | `grow_fine` | 4.669 | 8.160 | 1.75 | 91,262 | yes |
| oak 40 s1 | `grow_whole` | 0.332 | 0.289 | 0.87 | 1,886 | yes |
| oak 40 s7 | `grow_whole` | 0.526 | 0.481 | 0.91 | 3,045 | yes |
| oak 80 s1 | `grow_whole` | 3.267 | 3.243 | 0.99 | 7,453 | yes |
| oak 80 s7 | `grow_whole` | 7.194 | 7.020 | 0.98 | 18,093 | yes |
| lit oak 40 s1 | `grow_whole` | 0.758 | 0.583 | 0.77 | 1,821 | yes |
| lit oak 40 s7 | `grow_whole` | 1.145 | 0.853 | 0.75 | 2,937 | yes |
| lit oak 80 s1 | `grow_whole` | 8.685 | 6.701 | 0.77 | 7,179 | yes |
| lit oak 80 s7 | `grow_whole` | 17.692 | 13.563 | 0.77 | 17,370 | yes |
| palm 40, 80, s1, s7 | `grow_fine` | 0.001 | 0.000 to 0.001 | 0.84 to 0.90 | 0 | yes |

- **Two builds, by species.** `grow` sends a species that reads light or keeps its grown tree (the oak: `leaf_girth`, `retained`) to `grow_whole`. It sends the others (beech, spruce and palm) through F1's realized partition, `grow_fine`.
- **Against `grow_whole`, the stepped executor is never slower.** Against `grow_fine` it is slower on every tree that has roots.
- **The palm has no fine roots at the cut**, so both builds are the same work in under 1 ms.

### Where the time goes (`raw/task11/diag.txt`, load 3 to 7)

| 80 s1 | Threads | grow, s | stepped, s | stepped's cycles / assembly and settle, s |
|---|--:|--:|--:|---|
| spruce | 8 | 3.66 | 6.95 | 3.06 / 3.34 |
| spruce | 1 | 14.37 | 16.32 | 5.16 / 10.73 |
| lit oak | 8 | 7.84 | 6.14 | 4.91 / 1.16 |
| lit oak | 1 | 15.63 | 17.10 | 13.77 / 3.28 |

- **The lit oak.** The whole-tree grower spreads only each cycle's growth over the cores (`parallel.rs`). Balance, the allotment's inputs and split, the rough layout, the waking and the leaf gathering run on one thread over the whole tree, every cycle. The stepped executor runs each of these per item, with the items spread over the cores (`Executor::each`, 64 items a piece). Only the coarse structure and the merges stay serial. On one thread there is nothing to spread, and the stepped executor pays for its per-item bookkeeping (1.09 times).
- **The unlit oak** has no rough layout, balance or allotment: these read light, which this oak does not get. So there is nothing to spread, and the stepped executor ties.
- **The beech and spruce.** Their `grow` is already partitioned: `grow_fine` grows and settles each root alone, the roots spread over the cores. The stepped executor assembles the whole tree at the end and settles it on one thread with today's settle. That is 3.3 s of the spruce's 6.9 s on 8 threads, and 10.7 s on one. It also visits all of the spruce's 92,703 items every cycle, whether they live or not.
- **Spruce 80 s1's 18.2 s** in the main table came while the load rose to 24. The low-load rerun gives 6.95 s (1.90 times). Either way it is slower.

### Design question for the host: what the build becomes

Today's build has two executors: `grow_whole` for a species that reads light or keeps its grown tree, and `grow_fine` for the rest. The stepped executor is identical on all 20 trees. It is faster than `grow_whole` wherever light is read, and slower than `grow_fine`. Options:

- **(a) Stepped replaces `grow_whole` in `grow`; `grow_fine` stays for the rest.** This takes the measured gain now (0.75 to 0.99 times on the oak), but the build keeps two executors. `grow_whole` stays as the tests' reference, and `grow_staged`'s per-stage reporting would need its own route.
- **(b) Stepped becomes the only executor after its settle is partitioned.** The assembly and serial settle are replaced by `settle`'s four-phase per-root driver (`fine/expansion.rs`, the `Roots` trait), which `grow_fine` already uses. Dead items are dropped from the per-cycle visits. Then `grow_fine`'s path is deleted. Whether the realized phases accept a light-reading root that F2b's stepped history carries is unchecked; PHASES.md and the "fine growth" section of `docs/pipeline.md` say a light-reading species is refused there today.
- **(c) Leave the build as it is** until F4 (fn-198) brings the engine into the pipeline, and decide there.

## Part 2: the device's quotient

### The change

Contract A's quotient (`xquotient`, a 256-bit numerator over a 128-bit divisor, truncated toward zero) was a restoring bit loop: 256 steps of 128-bit shift, compare and subtract, whatever the operands. It is now Knuth's algorithm D (Hacker's Delight's `divmnu`) in 16-bit digits. Every partial product and digit estimate fits a `u32`, and the steps run over the operands' significant digits only:
- a one-digit divisor uses short division;
- a numerator shorter than the divisor is a zero quotient;
- otherwise the operands are normalised, each digit is estimated from the top two of the remainder and corrected by the divisor's second digit, and one add-back is made where the digit is still one too large.

The truncated quotient is unique, so any exact algorithm gives the CPU's bits; the probe checks that this one is exact. The CPU's `I256::quotient` is unchanged.

### The proof

`fine/feedback_quotient.rs`, `the_gpus_quotient_is_the_cpus_over_four_million_cases`, about 12 s under the lock with the probe:
- **The cases.** 4,194,304 random cases. Numerators have 1 to 16 digits and divisors 1 to 8, each drawn from edge digits (0, 1, 0x7fff, 0x8000, 0xffff) and random ones, with random signs. A sixth are a divisor times a quotient less one. 208 boundary cases follow, built from:
  - divisors: zero, ±1, 2, `i128::MAX`, `i128::MIN` and `i128::MIN + 1`, 2^64 and 2^64 − 1, and one- and two-digit edges;
  - numerators: `I256` min and max, all ones, zero, ±1, the divisor, and the divisor less one;
  - each divisor's sign taken both ways.
- **The result.** The GPU's words and statuses equal `I256::quotient`'s in every case.
- **Paths reached.** A model of the WGSL's steps, checked against the CPU on every case, counts the paths the cases take:
  - by zero: 74,791;
  - numerator shorter than the divisor: 764,334;
  - short division: 519,837;
  - long division: 2,835,550, of which 1,301,254 were already normalised;
  - digit estimates corrected 0, 1 and 2 times: 12,724,762, 3,557,913 and 232,959;
  - add-backs: 503,785.
- **The probe.** It gains a `quotient` selector (16,592 cases). The other 16 selectors, which include `ratio`, `unit` and `unit_w` that divide through it, still agree.
- **A mutation.** With the add-back's decrement removed, the quotient test fails, and so does the probe on `ratio`, `unit`, `unit_w` and `quotient` (`raw/task11/probe-m.txt`).

### The device run, re-measured

`the_lit_oak_at_80_on_the_device_timed`, the base binary (`11d3b703`, bit loop) and the new one (Knuth), back to back, each under the GPU lock. Load was 3 to 7 (`raw/task11/device.txt`).

| Lit oak 80 | Quotient | Today's `grow` | Stepped, CPU | Stepped, device | Device calls: layout / D16 / deposits / reads, s | Of them on the GPU, s |
|---|---|--:|--:|--:|---|---|
| seed 1 (7,179 roots) | bit loop | 8.13 s | 6.50 s | 12.95 s | 3.32 / 1.91 / 0.07 / 0.03 | 2.739 / 1.389 / 0.009 / 0.001 |
| seed 1 | Knuth | 8.51 s | 6.86 s | 13.35 s | 3.68 / 1.77 / 0.13 / 0.07 | **1.873 / 0.703** / 0.012 / 0.001 |
| seed 7 (17,370 roots) | bit loop | 17.55 s | 14.88 s | 34.34 s | 10.03 / 5.41 / 0.48 / 0.15 | 3.858 / 1.537 / 0.009 / 0.001 |
| seed 7 | Knuth | 26.60 s (load spike) | 14.57 s | 30.74 s | 8.48 / 4.42 / 0.47 / 0.17 | **3.008 / 1.104** / 0.017 / 0.002 |

- **Kernels.** The layout's kernels took 32 % less time on seed 1 and 22 % less on seed 7; D16's took 49 % and 28 % less.
- **The device is still about twice the CPU.** Each call's wall time is mostly outside the kernel. On seed 1 the layout's call takes 3.68 s against 1.87 s of kernel. The rest is buffers built afresh each call, 1.1 to 2.2 GB read back, and one dispatch per generation of axes (TASK10-GPU.md, "Why the device is slower"). That is what levers (b) resident state and (c) device framing address, the next task's decision.
- **What is left in the kernels** is the serial walk itself. The layout is one thread an axis, serially over its phytomers. D16 is one thread an item, over its log and exponential chains.

## The gate

The render half ran first, under the GPU lock: `cargo test --profile ci -p telperion-render --no-fail-fast`. 168 passed, 0 failed, 11 ignored. The load rose from 6 to 98 during it, from other workers.

Then the rest: `cargo test --profile ci --workspace --no-fail-fast --exclude telperion-render`. 1,013 passed, 0 failed, 18 ignored, in 10 min 11 s at a load of 75 to 98 falling to 13. Both were run at `62432ff6` with 8 build jobs, 8 test threads and 8 Rayon threads (`raw/task11/gate-*.txt`).

## Files

- `crates/telperion-space/examples/stepvsgrow.rs`: Part 1's measurement.
- `crates/telperion-render/src/generation/fine/feedback_wide.wgsl`: `xquotient`, `short_quotient` and `long_quotient`.
- `crates/telperion-render/src/generation/fine/feedback_quotient.rs`: the cases, the model and the 4.2 M-case test.
- `feedback_probe.wgsl`, `feedback_tests.rs`, `feedback_cases.rs`: the probe's quotient selector.
- `raw/task11/` (gitignored): `stepvsgrow.txt`, `diag.txt` and `device.txt`.
