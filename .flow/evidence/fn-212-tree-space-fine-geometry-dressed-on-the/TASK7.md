# fn-212 task 7: the 3x measurement, a composite; the handoff still to build (worker, 2026-10-07)

## Answer in brief

- **The spruce at 80, settled tree to draw-ready resident, five builds a side alternately:**

  | | Seed 1 | Seed 7 |
  |---|--:|--:|
  | Today, median, conversion excluded | 6,707 ms | 5,511 ms |
  | Device, median, conversion excluded | 892 ms | 795 ms |
  | Today with baseline B's conversion (965 ms) | about 7.7 s | about 6.5 s |
  | Device with its conversion share by nodes (9.1 % and 8.1 % of 965 ms) | about 0.98 s | about 0.88 s |
  | Ratio | about 7.8× | about 7.4× |

  Against baseline B's 8.1 s, the target of about 2.7 s is met by this measurement.
- **It is a composite, not the handoff.** The device's fine block is built and placed, and its leaves and curve are copied into buffers of their own.
- **Not built** (the acceptance's "submit path takes device buffers"):
  - the fine curve packed into the renderer's curve form (8-word points, 16-word clusters, offset after the CPU block);
  - the curve's demand;
  - one leaf buffer and one mass grid for the whole crown;
  - a submit that takes them;
  - a production entry that converts only the structure above the cut and the landed roots. The harness prunes the converted tree instead.
- **Final-output parity is not measured** on a submitted tree. Per block, task 5 (curve) and task 6 (stations and leaves) measured it. **The seam close-ups are not taken.**

## Method (`crates/telperion-render/src/generation/fine/timing_tests.rs`, `three_x_composite`)

- **One side a process, alternating, five builds a side a seed** (`SIDE`, `PROBE`; the driver is a loop of `cargo test` under the GPU lock). Load averages were 6 to 10 (`raw/task7/three-x-s1.txt`, `three-x-s7.txt`).
- **Today:** a fresh renderer and generator, `prepare(Resident)`, `submit_prepared` and the drain. The time is the generator's total less its skeleton, plus the submit and the drain: task 2's probe boundary, conversion excluded.
- **Device,** on the same settled tree (settled on the device by the probe's fixture; growth outside the boundary). Each stage is waited on:
  - **CPU:** `executor::expand` of the converted tree without the device's roots (above the cut, plus the landed roots); its curve with contacts, compact stations, GPU ring positions, placement with its mass grid, and its curve uploaded (`CurveGpu::new`).
  - **Device:**
    - the dress (the fixture's measured `dress()`);
    - the folds with their three round trips;
    - the sweep, stations, ring words and ring positions;
    - the placement, with its summary read;
    - the copy of the fine leaves, points and clusters into buffers of their own.
- **Conversion:** outside both. TASK7 adds baseline B's 965 ms to today, and the CPU side's node share of it to the device's. That share is an estimate, not a measurement.

## Profile (medians of the device's side, ms)

| Stage | Seed 1 | Seed 7 |
|---|--:|--:|
| CPU side (above the cut and landed roots: expand, curve, stations, positions, placement, curve upload) | 386 | 338 |
| Dress | 16 | 14 |
| Folds | 23 | 22 |
| Sweep | 72 | 65 |
| Stations | 302 | 294 |
| Rings and ring positions | 35 | 34 |
| Placement | 22 | 19 |
| Copy | 13 | 10 |
| **Total** | **892** | **795** |

- **Stations is the device's largest stage.** Its `bases` pass numbers the runs serially in one invocation (`@workgroup_size(1)`), over about 1M runs. Whether that pass is most of the 300 ms is **unknown** (not timed per pass).
- **The CPU side is a third to a half of the device's total.** The landed roots make up most of its nodes (D3(c), about 7 % of the spruce's).
- **Memory:** two builds of the spruce at 80 in one process run the 10 GB card out of memory. The kernels' pool keeps a settled tree's buffers, so each build runs in its own process.

## For the host

The composite answers the 3× question. Building the handoff is the rest of this task's acceptance, and it needs choices:
1. **The partial conversion's entry in production:** where the structure above the cut and the landed roots are converted without the device's roots (the settle's structure, by D3(c)), and how the attachments reach the device.
2. **The curve's handoff:** a device kernel packing the fine curve into `CurvePoint::pack`'s words (octahedral frame) and `packed_clusters`' (run, flags, offset by the CPU block's points), with the demand folded on the device. Or the curve renderer reads two blocks.
3. **The leaves:** one buffer (the fine block placed at an offset, or copied after the CPU block's), and the mass grid over both.

An estimate of the build, not measured: about a day of work, with parity and seam stills after it.
