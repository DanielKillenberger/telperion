# fn-211 task 10: contract A, the oak's fine roots on the device against the CPU (worker, 2026-10-07)

Astra's integration proof (ASTRA-REVIEW-CONTRACT-A.md, section 4): inside the stepped executor the device runs the fine roots' rough layout, D16, deposits and sampling each cycle, and the CPU runs the rest. The complete phase state is compared word for word against an all-CPU run on a small lit specimen that spans scheduled re-lays.

## Answer in brief

- **Identity holds to the word.** Every grower's whole state after every phase of every cycle, and every cycle's lattice and swept light, equal the all-CPU run's. The specimen is the lit oak at 24, which spans the re-lays at cycles 10 and 20. The comparison covers:
  - cuts of 64, 256 and 1,024;
  - the items in forward, reversed and shuffled order, in chunks of 1, 7 and 64;
  - the device's deposits taken forward, reversed and scrambled, in one dispatch or chunks of 777 and 4,096;
  - a forced retry: each cycle's first attempt goes into a lattice half the box's size, so 21 cycles a run overflow and retry.
- **The oak alone leaves three paths dormant in its fine roots at 24, so a neighbour is proven too.** In the oak, D16 decides 23,939 times and never meets a threshold. Relays carry no pulls, and no fine wood reaches the ground. The neighbour wakes all three: 36 to 84 fine laterals shed inside the roots, relays carry pulls on, and 19 to 26 phytomers come to rest on the ground. It is the oak with tolerance 0.99, straightening 0.4, erection 0.3, and its laterals turned towards −1.4 rad with three times the tropism.
- **The proof catches a wrong device.** Six mutations of the device, each one unit or one halving, each fail the proof:
  - the kept share;
  - the wander's curvature;
  - the curvature's keep;
  - a carried lift;
  - the landing's ease;
  - the rest on the ground.
- **The device is slower than the CPU it replaces.** The lit oak at 80 on cut 1,024, warm:

| Oak lit 80 | Today's `grow` | Stepped, CPU | Stepped, device | Device calls: layout / D16 / deposits / reads | Of them on the GPU |
|---|--:|--:|--:|---|---|
| seed 1 (7,179 roots) | 8.73 s | 6.67 s | **13.52 s** | 3.74 / 2.00 / 0.08 / 0.04 s | 2.51 / 1.28 / 0.007 / 0.001 s |
| seed 7 (17,370 roots) | 17.15 s | 14.59 s | **32.34 s** | 9.53 / 5.24 / 0.45 / 0.14 s | 3.82 / 1.56 / 0.008 / 0.001 s |

  - The load was 7 falling to 3 during these runs. The CPU's whole build in TASK9-CPU.md is 7.6 s on seed 1; here today's `grow` took 8.73 s.
  - At 80 the device's tree equals the CPU's by every phase's digest and by the tree's digest. The whole-state hash is too costly to take at 80 (FRICTION.md).
  - Per build: 392 dispatches, 1.1 GB (seed 1) and 2.2 GB (seed 7) read back, 5.3 M and 10.8 M phytomers laid.

## What runs where

| Each cycle | Executor |
|---|---|
| The items' D16 walk, parents first (`fixed::feedback::shed::walk`) | device: one thread an item |
| The items' rough layout: each axis's units sized, ranked and stepped (`draw`, `Layer::step` without a load) | device: one thread an axis, a dispatch a round |
| The items' leaves deposited into the W lattice | device: 128-bit atomic adds |
| The items' buds' light sampled from the swept lattice | device |
| Each round's framing (`pencil`, `frame`), each axis's pull at its age (`bend`), relay nodes | CPU |
| The coarse grower: its balance, allotment, growth, layout, deposits and reads | CPU |
| The items' totals, demand, sizes, growth and waking | CPU |
| The lattice's box, the one sweep, the re-lay, the settle | CPU |

## How it is built

- **The interface:** `telperion_space::Feedback` (`grow/stepped/feedback.rs`) has four calls: `lay`, `shed`, `deposit` and `sample`.
  - Every value that crosses is an integer of contract A.
  - `grow_stepped_on` takes a device, or none for the executor's own path.
  - `Cpu` is the CPU's own implementation. It lays a job by materialising it and calling `draw`, and the space crate's test holds it to the executor's own path.
- **One implementation on the CPU.** Three passes were split so that both executors use the same code:
  - **D16:** `shed_by` is now `laterals`, then `walk`, then `shed_walked`.
  - **The rough layout:** it runs in rounds (`sketch_begin`, `draw_round`, `sketch_round`, `sketch_end`), and the stepped executor lays a round's items on the device between them. The CPU grower takes the same rounds.
  - **The re-lay's leaves:** each grower's leaves are re-gathered from its own axes. The deposit is order-free, so the lattice is the same.
  - **No tree moved.** The stepped tree is still the whole tree's to the bit (`the_stepped_tree_is_the_whole_tree`).
- **The GPU:** `fine/feedback_stepped.{rs,wgsl}` holds D16, the deposits and the reads, and `fine/feedback_lay.wgsl` holds the layout. Both run over the feedback contract's WGSL.
- **The proof:** `fine/feedback_stepped_tests.rs`.
  - `the_lit_fine_roots_on_the_device_are_the_cpus` takes 40 s under the lock. It runs four cases: two on the oak and two on its neighbour.
  - `the_lit_oak_at_80_on_the_device_timed` is ignored by default; it is the timing above, with `SEED`.

## Why the device is slower

Each cause below is named from the measurement. None was optimised here:
- **The kernels themselves** take 3.8 s (seed 1) and 5.4 s (seed 7). On all cores the CPU's whole stepped build is 6.7 s and 14.6 s; its layout of the items is about 6 to 7 % of that (CONTRACT-A.md, Part 1).
  - **Division is bit-serial.** Contract A's quotient is `xquotient`, 256 steps of 128-bit arithmetic on 32-bit words. Every `unit` takes three, so a layout step takes fifteen or more: the units of the elevation's level, carry's pivot, the curvature, the blend and the side, then `over` for each pull.
  - **The layout runs one thread an axis**, serially over its phytomers. A round's dispatch is as long as its longest axis.
  - **D16 runs one thread an item**, serially over its laterals. Its kernel time (1.3 to 1.6 s) is the longest items' log and exponential chains.
- **The traffic** is the rest of the device calls' time:
  - every call builds its buffers afresh and reads its results back;
  - the layout reads 88 B a phytomer back;
  - each generation of new axes is its own round, because the CPU frames between rounds.
  - The CPU side of the rounds (building and applying jobs) adds about 1 s on seed 1.

## Open for the host

1. **Whether the device's cost is worth pursuing, and how.** Three levers are visible, each a decision:
   - **(a) A cheaper quotient with identical bits** (a reciprocal by Newton steps, corrected to the truncated quotient). This keeps contract A's results and changes only the WGSL.
   - **(b) Resident state:** keep the items' walks and phytomers on the device across cycles, so only framing inputs and leaves cross. This is FINE-CONTRACT decisions 5 and 6's direction, and the "full device grower" the task names as a later step.
   - **(c) Framing on the device**, so a cycle's layout is one dispatch, not one a generation.
2. **Framing and each axis's pull at its age stay on the CPU.** They are once an axis or once a cycle, against the steps' once a phytomer. Moving them is (c) above.
3. **The specimen.** The oak's fine roots at 24 leave D16's thresholds, carried lifts and the landing dormant, so the proof leans on a neighbour for them. FRICTION.md proposes a coverage count so a specimen's dormant paths are printed by every proof.

## Files

- `crates/telperion-space/src/grow/stepped/feedback.rs`: the interface, the CPU's own implementation, the device's light, and the whole-state hash.
- `crates/telperion-space/src/grow/stepped/lay.rs`: the layout's jobs and results, and the rounds across items.
- `crates/telperion-space/src/grow/draw.rs`: the rough layout in rounds.
- `crates/telperion-space/src/grow/balance.rs` and `fixed/feedback/shed.rs`: D16's walk.
- `crates/telperion-render/src/generation/fine/feedback_stepped.{rs,wgsl}`, `feedback_lay.wgsl`, `feedback_stepped_tests.rs`: the GPU's side and the proof.
- `raw/task10/` (gitignored): the timing runs, the green proof run, and the last mutation set.

## The gate

Run at `11d3b703`, with 8 build jobs, 8 test threads and 8 Rayon threads:
- The render half ran first, under the GPU lock: `cargo test --profile ci -p telperion-render --no-fail-fast`. 167 passed, none failed.
- Then the rest: `cargo test --profile ci --workspace --no-fail-fast --exclude telperion-render`. 1,013 passed, none failed.
