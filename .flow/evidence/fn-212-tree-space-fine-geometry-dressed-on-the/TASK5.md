# fn-212 task 5: the curve's sweep and clusters on the device (worker, 2026-10-07)

CONTRACT.md's probe, second part. The fine roots' F3 curve is now built on the device: its runs, points, transported frames and clusters (bounds, radius extrema, error ladders). It reads the nodes task 4 drew. The compact rings need no pass of their own: since task 2 they are the curve's points.

## Answer in brief

- **Every fine run on the device, in D5's order.**
  - Beech and spruce at 40 and 80, seeds 1 and 7 (`swept_at_40_and_80`, `raw/task5/swept-5.txt`).
  - The device's runs equal, one for one, the fine runs of `executor::grow(&family)`'s curve. The run counts are equal on all eight trees.
  - At most one device run per tree has no reference run. That is the node near a threshold which task 4 found, D1''s residual.
  - Every table is widest first.
- **Within the screen:**

  | Measure | Result |
  |---|---|
  | Centres, worst | 49 µm (spruce at 80) |
  | Centres, RMS | 5.3 µm or less |
  | Distance along | 23 µm or less |
  | Frames | 0.008 rad or less |
  | Cluster bounds | 28 µm or less |
  | Ladders | 25 µm or less |
  | Cluster counts | equal |

- **Three residuals, each measured and named below:**
  - frame seeds that flip: 4 to 40 runs per tree;
  - runs with a point over the screen: 12 to 13 on the spruce at 80, mostly the socket sample;
  - one beech codominance tie of float rounding's making.
- **Time:** 61 to 85 ms for the spruce at 80 on the device, including allocation and two totals read back. The same trees' CPU curve and rings took 2.6 s in baseline B. Timings are from the probe's wall clock, not the 3x harness (task 7).
- **One conversion change, D4 completed.**
  - Every fine root's node drawn from wood above the cut is now a seam.
  - Before, only the first axis's first node was. A lateral that hangs from the bearer's node, because the root's first phytomers merged into that node, could carry the bearer's run on. That is a cross-seam coupling the device cannot decide.
  - **No curve moved at 80:** runs and points are identical to TASK3.md's after D4 (beech 80 s1: 122,177 runs and 1,647,368 points; spruce 80 s1: 673,758 and 5,311,240; spruce 80 s7: 675,507 and 5,285,829).
  - **Task 4's counts are now exact:** the spruce's device and reference fine nodes are equal.

## How it works (`sweep.wgsl`, `sweep.rs`)

1. **`gather`** lays every drawn record out by id: its node, and each root's bearer from its stub.
   - Drawn records now carry their codominance, an id, their distance along the wood from the root's bearer, and their parent's id. Axis records carry the root's bearer's distance from the tree's root, the axis's depth in its root, and its lineage.
   - Distances are kept per root, so differences stay exact in f32.
2. **`lead` then `ties`** pick each node's leader by `paths.rs`'s rule: the widest child by start radius that is not codominant, else the widest.
   - Ties go to the conversion's first node, by depth, then lineage, then id, as `lineage_order` numbers them.
   - This matters: the spruce's codominant twins tie to the bit. Without it, 91 of 112k runs parted at 30.
3. **`starts`** begins a run at every seam and at every node that is not its parent's leader (D4).
4. **`walk`** finds each run's widest sample.
5. **Sorting:** a bitonic sort orders the runs widest first.
6. **`block`, `blocks` and `place`** scan the runs' points and clusters.
7. **`emit`** samples each run as `samples.rs`'s branch run does (socket, fork easing, swelling, flare), transports its frames as `frames.rs` does, and writes its clusters and ladders as `curve.rs` does.

- **Read back:** two totals (the run count, then the points and clusters). Nothing per node or per run.
- **Device memory per drawn record:** 108 B (node, links), plus the records' own 68 B.

## The residuals

1. **Frame seeds that flip** (4 to 40 runs a tree; 0 on the beech at 40). The CPU seeds a run's frame with `Vec3::perpendicular` of its first tangent: it crosses with the axis of the tangent's smallest component.
   - Where two components are nearly equal, the device's tangent (a few micrometres off over a millimetre's segment) chooses another axis, and the whole run's frame turns by about 90° about the tangent.
   - Centres and radii are unchanged. Only the bark's and lobes' angle on that run moves.
   - The test counts these by recomputing each executor's choice, and leaves them out of the frame bound.
   - A rule that does not jump (seeding from the parent's frame at the socket, for one) would change every run's bytes on both executors. That is the host's call.
2. **Runs that leave the screen at a point.** 12 to 13 of the spruce at 80's 640k runs have a point over 50 µm off.
   - On seed 1, all 13 are the first sample, 52 to 122 µm off. That sample sits on the bearer, pushed in along the first segment's direction. Every other point of those runs is in the screen.
   - On seed 7, the worst is 1.0 mm. Where along that run it lies was not located.
   - The probe's device tree settles on the device's own loads, and the reference on the CPU's. Their structure above the cut differs by tens of micrometres at the top, and a direction over a short first segment carries that further.
3. **One beech codominance tie** (beech 80 s7). Here a point's radius is off by 6 %.
   - A head the conversion scales to exactly `FORK_FROM` of its carrier reads a codominance of about 1e-16 in f64 on the CPU (`Some`), and 0 on the device. The parent's fork easing then differs on one run.
   - D1''s host decision kept these ties as deterministic, which holds within one executor but not across two.
   - The fix that makes both agree is to give a head scaled to `FORK_FROM` no codominance on both executors. It is the host's call; nothing was changed.

## Gate

- **CPU half:** 988 passed.
- **Render half,** under the GPU lock: 159 passed.
  - The first render run ran out of device memory: the two fast probes ran beside the realized suite. They now share one settled tree (`fine_roots_are_drawn_folded_and_swept_on_the_device`), and the rerun was green. FRICTION.md has the entry.
- `RAYON_NUM_THREADS`, `RUST_TEST_THREADS` and `CARGO_BUILD_JOBS` were 8.

## Carried to task 7

- The leaf box's reach terms come from the combined envelope in production (host).
- The fine runs' points, clusters and table are relative to the fine block. The resident handoff offsets them after the CPU side's (D5).
