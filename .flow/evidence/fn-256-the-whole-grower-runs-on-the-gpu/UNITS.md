# fn-256 task 2: the units and the wake on the device, measured (worker, 2026-10-11)

DESIGN.md sections 2.1, 2.2 and 7 for the units and the wake. The layout is not built here. This file records what ran and what it measured. It does not extrapolate to a build or judge the budget.

## How it ran

- **Device:** NVIDIA GeForce RTX 3080, Vulkan, native wgpu 30, through `crate::Gpu` with WebGPU's default limits (8 storage buffers a stage).
- **Capture:** `telperion_space::grow_capture_units_at` (`grow/cycle/capture_units.rs`) grows the preset through the one cycle. For the captured cycle it then runs `open`, `retire`, `balance_one` and `allot_one`, snapshots every input of `grow_live` and `wake`, runs both, and snapshots the state they leave. `grow_capture_units` captures the tree's age.
- **Replay:** `crates/telperion-render/src/generation/cycle/`, test-only. The order of dispatches is:
  1. Units count, one invocation per living apex.
  2. Two exclusive scans (`fine/scan.wgsl`).
  3. Units emit.
  4. Compaction of `asleep[cycle]`: flag, scan, gather.
  5. Wake count, one invocation per sleeping bud.
  6. Two scans.
  7. Wake emit.

  That is 21 dispatches, each in its own stamped compute pass. No index comes from an atomic. The scans' totals reach the next kernels by buffer copies into a tally, with no host read.
- **Compile:** one entry serves both phases and both modes. The heavy steps have one call site each, in a small loop of operations. The first young run took 348 s, nearly all of it the driver's compile. After the restructure the same run takes 38 s.
- **Timing:** median of 9 runs. Each run restores the captured state by copies, which are not stamped.
- **Tests:**
  - `young_cycles_agree_with_the_cpu` covers european-beech 6 at 6, Telperion 8 at 8, european-beech 12 at 8, Telperion 14 at 10, ordinary 16 at 12 and norway-spruce 24 at 16. The last two make and wake sleeping buds.
  - `busy_cycles_agree_and_are_timed` (`#[ignore = "a measurement"]`) covers Telperion 82 and european-beech 40, both at seed 42.
  - Both tests assert exact agreement of every word.
- **Logs:** `raw/units-young-contract.txt`, `raw/units-busy-final.txt`, and the experiment runs `raw/units-*-exact*.txt`.

## Agreement

Every integer and every f32 is compared exactly: each axis's 25 columns by index, each axis's new units and phytomers in order, the sleeping buds by the cycle they wake in, the new living apexes and the woken buds in order, and the phytomers grown.

**On `fine/contract.wgsl` as it stands:**
- **What agrees exactly:** every count, every index and every order, in every capture. That covers axes, units, phytomers, sleeping buds, the live list, woken buds and successors, and so the topology.
- **What differs:** f32 values in their last places, and two integers floored from an f32.

| Capture | Made: axes / phytomers / units / sleeping buds / woken | Fields that differ (differing / compared) |
|---|---|---|
| european-beech 6 at 6 | 57 / 349 / 80 / 0 / 0 | phytomer node presence 8 / 349 |
| Telperion 8 at 8 | 71 / 667 / 163 / 0 / 0 | node 10 / 667 |
| european-beech 12 at 8 | 71 / 667 / 163 / 0 / 0 | node 10 / 667 |
| Telperion 14 at 10 | 110 / 1,024 / 248 / 0 / 0 | node 13 / 1,024; `b0` 2 / 998 words |
| ordinary 16 at 12 | 427 / 2,283 / 785 / 4 / 3 | node 41 / 2,283; `b0` 14 / 3,650 words |
| norway-spruce 24 at 16 | 8,628 / 27,154 / 12,677 / 145 / 114 | node; `b0` 82 / 79,116 words; unit survival 63 / 12,677; sleeping bud's own presence 1 / 145; **sleeping bud's waking tick 7 / 145 (±1 tick)** |
| Telperion 82 (busy) | 69,067 / 851,381 / 211,848 / 0 / 0 | node 9,739 / 851,381; unit survival 908 / 211,848; `b0` 1,339 / 3,065,956 words; blend 6 words; **`below` 1 / 1,532,978 (2818 against 2817)** |
| european-beech 40 (busy) | 8,250 / 88,937 / 22,001 / 0 / 0 | node 1,022 / 88,937; unit survival 90 / 22,001; `b0` 150 / 133,846 words; blend 4 words |

`b0` and blend are f64 columns compared as two words each. Each is the f64 of an f32 presence, or `1 - real(p)`.

**The cause, traced by recomputing each step on the CPU from the device's own inputs:**
- The draw, the bound and the window agree. The lead and the presence differ. Two sources were found, each confirmed by an uncommitted experiment.
- **WGSL division is not correctly rounded on this driver:**
  - `contract.wgsl:64` against `contract.rs:81`: `log2c`'s `(m - 1) / (m + 1)`.
  - `contract.wgsl:213` against `contract.rs:251`: `presence`'s `lead / window`.
  - `contract.wgsl:235` against `contract.rs:276`: `logit53`'s ratio.
  - `contract.wgsl:330` against `contract/decide.rs:87`: `lived_part`'s `u / l`. This is the `below` tick.
  - `contract.wgsl:257` against `contract.rs:218`: `neg_ln1m`'s `1 / k`.
- **The driver fuses a multiply into the add that follows it:**
  - `log2c`'s series and its final `e + ln · log2 e`: `contract.wgsl:66-67` against `contract.rs:83-84`.
  - `lnc`'s product before `past` subtracts the bound: `contract.wgsl:82` against `contract.rs:107`.
  - `neg_ln1m`'s series: `contract.wgsl:257` against `contract.rs:218`.
  - fn-254 measured these functions within a tolerance (`fine/tests.rs`, `Tolerance`). DESIGN.md section 8 keeps f32 "with no guards on fused multiply-adds".
- **The experiment** (`raw/contract-exact-experiment.patch`, not committed) changes the shared contract in two ways. It replaces those divisions with an integer, correctly rounded division. It keeps those products apart by passing them through their bits XORed with a uniform zero (`kept_apart`), the spike's barrier, which needs the cycle module's uniform. With it, **all six young cycles and both busy cycles agree exactly in every word**.

`young_cycles_agree_with_the_cpu` is committed `#[ignore]`d with the reason named. It is red on the contract as it stands, and its assertion is unchanged.

## Timing (median of 9, ms of GPU time in the stamped passes)

| Phase | Telperion 82, contract as is | Telperion 82, exact experiment | european-beech 40, contract as is | european-beech 40, exact experiment |
|---|--:|--:|--:|--:|
| units count | 7.807 | 11.313 | 0.813 | 1.167 |
| units scan (2 × 3 dispatches) | 2.226 | 2.100 | 0.634 | 0.634 |
| units emit | 8.399 | 11.280 | 0.906 | 1.216 |
| compaction (flag, scan, gather) | 0.008 | 0.008 | 0.008 | 0.008 |
| wake count | 0.002 | 0.001 | 0.002 | 0.001 |
| wake scan | 0.012 | 0.012 | 0.011 | 0.012 |
| wake emit | 0.001 | 0.001 | 0.001 | 0.001 |
| first stamp to last | 18.592 | 24.845 | 2.496 | 3.172 |

**Per run:**

| | Telperion 82 | european-beech 40 |
|---|--:|--:|
| Dispatches | 21 | 21 |
| Resident: state, tables and inputs | 529.1 MB | 39.3 MB |
| Scratch: counts and scan sums | 7.49 MB | 0.78 MB |
| Living apexes | 233,233 | 24,185 |
| Phytomers emitted | 851,381 | 88,937 |
| Axes emitted | 69,067 | 8,250 |

- **Copies for the runs:** the captured state each run restarts from is a second copy of the resident state. It is test scaffolding and is excluded above.
- **The wake was not exercised:** neither busy cycle wakes a sleeping bud, since Telperion and the beech have none. So the wake rows time empty dispatches. The wake's agreement rests on the young ordinary and spruce cycles.
- **The capture's CPU time:** 8.4 s for Telperion 82 and 0.3 s for the beech 40.

## Open for the host

1. **The f32 contract on the device.** There are three options:
   - Correctly rounded division and contraction barriers in `fine/contract.wgsl`. That gives exact agreement and costs about 6.3 ms on Telperion's busy cycle. The barrier needs a uniform zero the fine shaders do not have today.
   - DESIGN.md section 8's tolerance for floats. Under it, two integers floored from f32 differ here: the waking tick and `below`.
   - Something else.

   This is the host's call. Until it is made, the young test stays ignored.
2. **The scans are a quarter of Telperion's units time** (2.2 ms of 18.6). `fine/scan.wgsl` sums each block of 256 records in one thread and the spine in one invocation. This file does not judge whether a faster scan is wanted.
