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

## After the host's decisions (2026-10-07)

### 1. Frame seed: what was built, and a question for the host

- **The rule as asked.** Parallel transport of the bearer's frame from its heading onto the run's first tangent (`rotate(side, heading, t0)`). It was built on both executors and measured (`raw/task5/seeded-1.txt`).
  - It turns wherever a run leaves its bearer nearly backwards, because the shortest rotation onto the opposite direction has no unique axis.
  - The spruce has many such runs. Of the fine runs, these leave more than 170°, 175° and 179° from the bearer's laid heading:

    | Tree | Fine runs | Over 170° | Over 175° | Over 179° |
    |---|--:|--:|--:|--:|
    | spruce 80 s1 | 638,494 | 5,457 | 2,802 | 590 |
    | spruce 80 s7 | 644,903 | 5,608 | 2,954 | 634 |
    | spruce 40 | about 200k | 1,337 to 1,349 | | |
    | beech | | 0 | 0 | 0 |

  - **Result:** 1,702 to 1,704 runs at the spruce at 80, and 66 to 68 at 40, had frames over 0.01 rad apart, up to 3.1 rad. The beech had none.
- **What is committed: the bearer's laid side, laid across the first tangent.**
  - The run's first normal is the side, with its component along the tangent removed. That is the frame's own first step, so the seed is just `side`.
  - It turns only where a run leaves along the bearer's side. On the eight trees, 0 to 4 runs leave within 10° of it, and none within 1° (`raw/task5/seeded-2.txt`).
  - Measured end to end (`raw/task5/probes-seed.txt`): frames within 0.0091 rad on every tree, with no exclusion; centres, radii and clusters as before.
  - It is a seed from the bearer's frame at the socket, but it is a projection, not the parallel transport the host named. Some direction must turn under any continuous seed (the hairy ball), and this choice puts it where no measured run goes. **The host's choice:** keep the side, or take the rotation and accept its 1,700 runs.
- **Not yet done, pending that choice:** the twist distribution before and after, and the stills (hero and close up, spruce and beech at 80). They show whichever rule is kept.
- **Bytes:** every fine run's frame on both executors moves; centres and radii do not. The node frames the seed reads agree across executors within 8e-6 rad (a temporary check).

### 2. Codominance tie

- A head the conversion scales to `FORK_FROM` now gets no codominance on both executors, decided by the branch taken (`space.rs`, `aside`; `dress.wgsl`). Built in `16328f13`.
- **The beech 80 seed 7 radius outlier was not this.** It is still 6 %. The run's node 1548695 bears a lateral of codominance 8.9e-9: its radius is 0.4 of its parent's plus 2.7e-9 relative. The CPU reads a fork there and eases the next node's girth; the device, in float32, does not.
  - This is a lateral's own codominance near its threshold: a D1' residual, which contract A's exact radii remove. It is not a tie of the code's own making.

### 3. The spruce at 80, seed 7: the point 1.0 mm off

- **Where:** run 62151, point 2 of 23. Every other point of the run is within 4 µm.
- **Why: a `SHORTEST` decision at its threshold.**
  - The parent node P and the CPU's node X are 2.0 mm apart. The phytomer between them, Y, lies 1.0 mm from P.
  - The CPU found Y under 1 mm from P and merged it into P, then made X.
  - The device found Y at 1 mm or more, made it, then merged X into Y (X lies under 1 mm from Y).
  - Both runs have 23 points. Point 2 is Y on the device and X on the CPU, 1.0 mm apart.
- **Classification:** a D1' residual (a decision within float noise of its threshold), not settled-structure noise. Contract A removes it.
- **The other run of seed 7 over 200 µm** (run 179788: 111 to 206 µm, growing towards its tip) was not traced.

### 4. GPU test memory

- `generation::whole_tree()` is a process-wide lock. The fine executor's kernels take it on creation and hold it while they live, because their pool keeps a settled tree's buffers.
- The render half and both 40/80 probes together pass at the default eight threads (`raw/task5/gate-render-lock.txt`, `raw/task5/probes-seed.txt`).

### Gate after these changes

- **CPU half:** 988 passed.
- **Render half:** 159 passed, at eight test threads.

## The side seed kept: what moved (host, 2026-10-07)

- **The seed's turn, by run** (`seed_twist_at_80`, `raw/task5/seed-twist.txt`). This is each fine run's first normal against the old seed (the first tangent's `perpendicular`), in 15° bins from 0° to 180°:
  - **Beech 80 s1:** 9,064, 10,111, 10,550, 10,382, 10,528, 10,004, 9,949, 10,661, 10,452, 10,460, 9,893 and 8,733.
  - **Spruce 80 s1:** 80,285, 57,545, 53,673, 53,141, 52,813, 54,749, 54,218, 52,163, 49,586, 48,656, 48,914 and 67,684.
  - The old seed came from world axes and the new one from the bearer, so the turn is spread evenly. Every fine run's bark and lobe angle moves; its centres, radii and twist rate along the run do not.
- **Stills at 80, seed 1, before (old seed) and after (side seed)**, hero view and a close look at the wood (`--view bare --camera '{"fill": 10, "targetHeight": 0.45, "elevation": 5}'`). All eight are in `raw/task5/stills/` and all were viewed.
  - **Indistinguishable by eye.** Triangle and instance counts are equal.
  - **Pixels apart** (ImageMagick AE):

    | Still | Exact | Under 6 % fuzz |
    |---|--:|--:|
    | beech hero | 45 | 10 |
    | beech close | 1,690 (0.16 %) | 808 |
    | spruce hero | 124 | 51 |
    | spruce close | 38 | 0 |

  - The close-ups show mostly the structure above the cut. The fine runs' wood is thin at those distances, so its bark turn barely registers.
- **The binaries:** `raw/task5/bin/headless-before` is this branch with the seed switched off, and `headless-after` is the branch as committed.
