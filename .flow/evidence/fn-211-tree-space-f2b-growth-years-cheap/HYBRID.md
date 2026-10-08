# fn-211 task 6: what a CPU-led hybrid would save the oak (worker, 2026-10-07)

The hybrid is Astra's (ASTRA-REVIEW-OAK-GPU.md, point 2). The CPU keeps the oak's whole light loop: the balance, allotment, rough layout of every axis (fine roots included), the re-lays, the deposits, sweep and sampling, and the settle. The GPU does only the fine roots' growth bookkeeping (`grow_live` and waking), from the light, size and kept share the CPU hands it, and hands back the new botanical records. This task measures that split. It builds nothing.

## Answer in brief

- **Measured:** the GPU's share is small. The fine roots' growth and waking cost 11 to 12 % of the stepped oak's build at 80 on all cores, and 25 to 26 % on one core. Everything else stays on the CPU: 56 to 58 % in the light loop and 25 to 28 % in the settle (all cores).
- **Measured:** these records would cross each build.
  - **Up:** 9.5M records (seed 1) and 19.8M (seed 7). That is 1.09 GB and 2.27 GB at the CPU's own struct sizes, or 0.23 GB and 0.47 GB in a lean encoding.
  - **Down:** 1.7M records and 3.5M. That is 45 MB and 97 MB as held, or 20 MB and 43 MB lean.
  - **The late cycles carry most of it:** up to 42 MB up a cycle on seed 1 and 101 MB on seed 7, as held.
- **Estimated:**
  - **The saving on all cores** is between +7 % and −11 % of the build. A free device with free transfers caps it at 8 to 10 %.
  - **On one core** it is 17 to 24 %. That is not the bar the oak is measured against.
- **Verdict for the host: delete the idea.** Even a GPU that costs nothing leaves the oak at 80 on all cores at about 4.2 s (seed 1) and 9.9 s (seed 7). Today's whole-tree `grow` takes 3.63 to 3.68 s and 7.83 s (PHASES.md). Those runs were at another load, so this comparison is an estimate.

## How measured

- **The instrumentation:** `hybrid-instrumentation.patch` puts a wall-clock span around each phase of `Executor::cycle` (`grow/stepped.rs`).
  - The items' growth and waking now run as their own pass over the items, and their sketch as a second pass. Today's executor runs the two together, per item. The tree is the same: `stepcost` reports "same tree: true" at 40 and 80, seeds 1 and 7, on the instrumented build.
  - Inside each item, the existing `Stage::Pass` reports (`Homes`, `Units`, `Merge`) and the wake are timed. They are summed over the items as core-seconds.
  - Each cycle also counts the records in each direction (below).
- **The probe and its tooling:** the probe is `hybrid-probe.rs`. `hybrid-run.sh` runs it, gated on load, and `hybrid-tables.py` makes the tables.
- **Reverted:** the patch was reverse-applied after the runs, and `git diff -- crates/` is empty.
- **The tree:** the lit oak (extinction 0.5, sky 0.5) at 80, seeds 1 and 7, cut 1,024, forward order. The timings are the warm build (the second in the process).
- **The machine:** Ryzen 9 5950X. "One core" is `taskset -c 0`.
- **The desk's load:** other workers held the one-minute load at 20 to 176 for most of the hour.
  - The runs used in the tables are the ones started below or near load 8. Each row gives its load.
  - Runs at loads of 20 to 131 are kept in `raw/task6/{b,c,d}` (gitignored). Their shares of the build agree with the tables to within 2 points. Only their seconds are inflated, by 1.2 to 7 times.
- **The per-phase hash is excluded.** The stepped executor hashes every phase of every cycle for probe (a) (`mark`). That costs 0.87 s (seed 1) and 2.02 s (seed 7) on all cores, and a production executor would not pay it. Every "build" below is the stepped build minus `mark`.

## 1. Where the stepped oak's time goes (measured, seconds)

| Oak 80 | Load | Build (no marks) | Light loop the CPU keeps | Of it: balance / allot / sketch, items / relay / shade | Fine roots' growth and waking (GPU-able) | Settle (CPU) | Coarse growth and handover |
|---|---|--:|--:|---|--:|--:|--:|
| s1, all cores | 5.9 → 5.5 | 4.65 | 2.72 (58 %) | 0.27 / 0.41 / 0.29 / 1.02 / 0.65 | **0.56 (12.0 %)** | 1.17 (25 %) | 0.08 |
| s7, all cores | 5.5 → 8.0 | 10.73 | 6.00 (56 %) | 0.54 / 0.88 / 0.66 / 2.38 / 1.35 | **1.20 (11.2 %)** | 3.00 (28 %) | 0.19 |
| s1, one core | 8.0 → 10.3 | 15.46 | 7.40 (48 %) | 0.98 / 1.39 / 1.67 / 2.28 / 1.00 | **3.86 (24.9 %)** | 3.89 (25 %) | 0.08 |
| s7, one core | 8.0 → 7.0 | 25.44 | 13.07 (51 %) | 1.71 / 2.49 / 2.87 / 4.15 / 1.69 | **6.66 (26.2 %)** | 5.33 (21 %) | 0.16 |

Notes on the table:
- **The light loop's total** includes the coarse grower's growth, its sketch and the handover.
- **The residual:** the rest of each build (0.2 to 0.5 s) is the years' time outside the phases: the probe's own counting and the per-cycle vectors.
- **The one-core builds are slower than PHASES.md's** (9.17 s and 20.07 s with marks). Core 0 was shared with other workers. The shares are the measurement here, not the seconds.

**How the fine roots' growth splits** (core-seconds summed over items, from the `Stage::Pass` reports):
- **On all cores:**
  - `Units`, the growth itself: 68 to 74 %.
  - `Merge`, appending the shares to the item's tree: 21 % (seed 1) and 28 % (seed 7).
  - `Homes`: 4 %; waking: under 1 %.
- **On one core:** `Units` is 90 % and `Merge` is 5 %.
- **The merge stays on the CPU in the hybrid.** The CPU appends the GPU's records to its own copy of each root, because its sketch, balance, allotment and re-lay read them. Its share of the wall time is kept on the CPU below.

## 2. What would cross each cycle (measured counts; bytes as defined)

The counts come from the executor's own state each cycle. They are the same in every run, because the tree is deterministic.

**Down (CPU to GPU):**
- **Each living fine apex that grows:** its light, size and kept share. That is 24 B as f64, or 12 B lean (Q32).
- **Each fine apex shed by the balance:** its id, 4 B.
- **Each new fine root handed over:** its axes, its phytomers grown in its birth cycle, and its apexes.
  - As the CPU holds them: an axis is 408 B (`Axis` 224, `Draws` 136, plus 48 for the line's `units`, `successor`, `root` and `horizon`). A phytomer is 136 B (`Phytomer` 128, plus its node draw).
  - Lean: 40 B and 28 B.

**Up (GPU to CPU):**
- **Each new axis:** 408 B as held, 40 B lean (lineage, PA, birth, origin, vigour, end).
- **Each new phytomer:** 136 B as held. Lean, 28 B: the key, cycle, scale and rank. The CPU already knows the size and light, because it supplied them.
- **Each unit grown:** 32 B as held (`Unit` and its `units` entry), 24 B lean.
- **The next cycle's living list:** 4 B an apex.
- **No sleepers or woken buds** cross for the oak. Its items held none in any cycle.

| Oak 80 | Records down, build (median / max a cycle) | Bytes down, as held / lean | Records up, build (median / max a cycle) | Bytes up, as held / lean | Late cycles (71 to 80), up as held |
|---|--:|--:|--:|--:|--:|
| s1 | 1.66M (12.1k / 63.4k) | 45 MB / 20 MB | 9.54M (70.7k / 368k) | 1.09 GB / 0.23 GB | 38 MB a cycle (median), 42 MB at most |
| s7 | 3.48M (21.6k / 157k) | 97 MB / 43 MB | 19.8M (126k / 882k) | 2.27 GB / 0.47 GB | 89 MB a cycle (median), 101 MB at most |

What the counts say:
- **The new phytomers are most of the upward traffic.** That is 5.5M on seed 1 and 11.3M on seed 7: every phytomer the fine roots grow crosses once.
- **This is the per-node exchange that FINE-CONTRACT decisions 5 and 6 forbid,** as Astra warned. Its total is 1.4 times (lean) to 7 times (as held) probe (b)'s whole exchange, lattice included, which was about 0.17 GB and 0.38 GB a build (derived from TASK5-GPU.md's node counts at 4 B words).

## 3. The saving (estimated)

**The GPU's side, per build:**
- **Kernels:** 15 to 57 ns a grown phytomer.
  - 15 ns is fn-209 task 8's spruce 80: its grow, assign and asks kernels took 65 ms for 4.35M phytomers.
  - 57 ns is the beech 80 seed 1's whole fine stage: 88 ms for 1.54M phytomers (REALIZED.md).
  - Both priced chains that grow many cycles in one dispatch. The hybrid grows one unit an apex a cycle, so the real cost is more likely near the top of this range.
- **Round trips:** two a cycle, down then up. That is 0.21 to 0.54 ms each, from probe (b)'s median trip and its mean trip at 80 (TASK5-GPU.md). Over 80 cycles: 34 to 86 ms.
- **Transfer:**
  - The low end is the lean bytes at 20 GB/s, near PCIe 4.0 x16 in practice. That rate is assumed, not measured.
  - The high end is the as-held bytes at about 2 GB/s. That is probe (b)'s effective rate, its bytes over its whole time, so it includes synchronisation. It is derived, not measured.
- **The CPU's merge of the records:** kept, at its measured share of the fine roots' growth.

| Oak 80 | GPU-able growth (measured) | CPU merge kept (est.) | GPU kernels + trips + transfer (est.) | Saving (est.) | Ceiling: free GPU, free transfer (est.) | Hybrid build (est.) |
|---|--:|--:|--:|--:|--:|--:|
| s1, all cores | 0.56 s | 0.12 s | 0.13 to 0.97 s | **+0.31 to −0.53 s (+7 % to −11 %)** | 0.44 s (9.5 %) | 4.34 to 5.18 s |
| s7, all cores | 1.20 s | 0.33 s | 0.23 to 1.91 s | **+0.64 to −1.05 s (+6 % to −10 %)** | 0.87 s (8.1 %) | 10.09 to 11.77 s |
| s1, one core | 3.86 s | 0.19 s | 0.13 to 0.97 s | +2.70 to +3.54 s (+17 % to +23 %) | 3.67 s (23.7 %) | 11.92 to 12.76 s |
| s7, one core | 6.66 s | 0.33 s | 0.23 to 1.91 s | +4.42 to +6.10 s (+17 % to +24 %) | 6.33 s (24.9 %) | 19.34 to 21.02 s |

What the estimate says:
- **Nothing overlaps.** Each phase feeds the next: growth reads this cycle's light, and the sketch reads this cycle's growth. The GPU's time therefore adds to the CPU's rather than hiding behind it.
- **On all cores, the CPU's share caps the saving.** The light loop and settle the CPU keeps are 83 to 84 % of the build, and the hybrid cannot touch them.
- **The best case, with a free device and free transfers, still loses to today's tree.**
  - Seed 1: about 4.2 s (4.65 − 0.44), against today's whole-tree `grow` at 3.63 to 3.68 s.
  - Seed 7: about 9.9 s, against 7.83 s.
  - Today's figures are PHASES.md's, at load 10 to 12. A same-run comparison here was spoiled by load 23 to 53 (`stepcost`: stepped 103 s against today 6.2 s on seed 1), so the gap is an estimate. Its sign is not in doubt: the stepped CPU executor itself is 1.1 to 1.4 times today's (PHASES.md).
- **Where the oak's time would come from instead** is the light loop's own serial passes: the re-lay, sketch, allot, balance and shade (PROFILE.md section 4, layers 1 and 2). That is CPU work, and none of it needs a device.

## Verdict

**Delete the idea:** the GPU could take at most 8 to 10 % of the oak's 80-year build on all cores. Moving that share costs 0.23 to 2.4 GB of per-node traffic a build, and the estimated saving ranges from +7 % to −11 %.

## Files

- `HYBRID.md`: this file.
- `hybrid-instrumentation.patch`: the timers and counts, reverse-applied after the runs.
- `hybrid-probe.rs`: the probe, run as `crates/telperion-space/examples/hybrid.rs`.
- `hybrid-run.sh` and `hybrid-tables.py`: the runner and the tables.
- `raw/task6/` (gitignored): `e/` (the low-load runs above), `d/oak80-s7-one.txt` (the one-core seed 7 row), `b/`, `c/` and `old-binary-highload/` (high-load repeats), and `counts/`.
- `FRICTION.md`: the desk's load and the load gate.
