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

## The real path (worker, 2026-10-07, after the host's decisions on the handoff)

### What was built (`4f7411c3`, `3aa10064`)

- **Partial conversion, one path** (decision 1).
  - `telperion_space::partial` takes the settle's structure: each root laid on the CPU is assembled where it stands, as `grow_fine` assembles it; each other live root's first axis keeps its stand-in's place with no phytomers. It is put in the canonical order and marked as `grow_marked` marks.
  - The core converts it with the one `convert` (`executor::settle` → `Settling::part`). The conversion records every root's attachment and draws none of a device root's wood.
  - `Part::expansion` takes the whole tree's folds, the device's sifted crown nodes and its leaf box. Its envelope, inputs and reference are the whole tree's. The reach terms the device folds its box by come from the combined height (`Part::wood_reach`), the one envelope field they read.
  - The probe's stand-in (`above`, pruning the converted tree) is deleted.
- **Curve** (decision 2). `handoff.wgsl` packs the fine block's points into `CurvePoint::pack`'s 8 words (octahedral frame) and its clusters into `packed_clusters`' 16 words, after the CPU block's points and clusters, every index offset by them. The same pass folds the fine block's demand (stretches, and 256 compensated partial sums of the sags' roots, added in f64 on the host) and its bounds. `CurveGpu::on_device` takes the one block. `submit_prepared` takes it as it is (`Prepared::wood`).
- **Leaves** (decision 3). `place.wgsl` writes from `config.random.w`. Both placements write into one leaf buffer, the device's block at the CPU's count, with no copy. One mass grid is built over both.
- **Read back:** the totals and summaries the probes already read, plus the handoff's 264 words. Nothing per node or per run.

### The 3× verdict on the real path (`timing_tests.rs`, `three_x`; `raw/task7b/three-x-*.txt`)

Five builds a side, alternately, one a process, under the GPU lock. Today's side is the generator's total less its skeleton, plus the submit and drain; baseline B's conversion (965 ms) is added to it as before. The device's side is the real path from the settled tree to draw-ready, its own partial conversion measured inside it, plus the submit and drain. The settle is growth, outside the boundary on both sides.

| | Seed 1 (load 7 to 9) | Seed 7, retaken (load 6.6 to 23) |
|---|--:|--:|
| Today, median | 5,668 ms | 6,914 ms |
| Today with baseline B's conversion | 6,633 ms | 7,879 ms |
| Device, median, its conversion included | 857 ms | 1,110 ms |
| **Ratio** | **7.7×** | **7.1×** |

- **Verdict: the 3× holds on the real path** at both seeds, against a target of about 2.7 s.
- **Seed 7's first pass ran at a load of 20 to 33** (`three-x-s7.txt`, FRICTION.md) and gave 7.1× as well. The retake still ran above 12 for six of its ten builds; seed 1 is the clean figure.
- **The conversion is now measured, not estimated:** 99 ms (seed 1) and 165 ms (seed 7), against TASK7's estimate of 88 and 78 ms by node share.

**Stage medians, device side (ms):**

| Stage | Seed 1 | Seed 7 |
|---|--:|--:|
| Partial conversion | 99 | 165 |
| Dress | 41 | 58 |
| Folds (four round trips) | 13 | 16 |
| Expansion of the part (clothing, element, box) | 14 | 19 |
| CPU block: curve, stations, ring positions | 258 | 339 |
| Sweep | 64 | 72 |
| Stations | 273 | 282 |
| Rings and ring positions | 34 | 40 |
| Both placements and the mass grid | 47 | 63 |
| Curve handoff | 29 | 39 |
| Submit and drain | 5 | 5 |
| **Total** | **857** | **1,110** |
| (Settle, outside) | 533 | 791 |

**The stations' numbering pass dominates.** `PROFILE=1` timed every compute pass (`profile-s1.txt`, `profile-s7.txt`): `bases` took 248 ms of the stations' 284 ms on seed 1, and 258 ms on seed 7. It is one invocation (`@workgroup_size(1)`) numbering about a million runs serially. A parallel scan there would take the device's total to about 610 ms on seed 1 (estimated by subtraction, not built).

### Final-output parity, by named class (`resident_tests.rs`; `raw/task7b/parity-80.txt`, `fast-2.txt`)

Read back from the buffers the submit takes, against the CPU path's tree (`executor::grow`), its curve and its resident leaves.

| | beech 30 s1 | spruce 30 s1 | spruce 80 s1 | spruce 80 s7 |
|---|--:|--:|--:|--:|
| Part's nodes, all matched by key and place | 4,795 | 54,753 | 423,983 | 374,507 |
| Their positions, most apart | 0 | 2.2 µm | 38 µm | 44 µm |
| **Kinds apart (seam class A)** | 1 | 92 | 322 | 323 |
| **Bearers of a codominant device root (seam class B)** | 89 | 862 | 2,990 | 2,996 |
| Runs (CPU block, device block) | 298, 2,167 | 6,475, 111,976 | 35,264, 638,494 | 30,604, 644,903 |
| Runs within the screen, point for point | 2,465 | 118,451 | 673,758 | 675,506 |
| Runs parted / unmatched / missing | 0 / 0 / 0 | 0 / 0 / 0 | 0 / 0 / 0 | 1 / 0 / 0 |
| Runs off in radius > 1e-3 at a class-B bearer | 12 | 0 | 1 | 1 |
| Centres, most apart | 2.3 µm | 2.3 µm | 38 µm | 44 µm |
| Radius (relative), along, frames | 3e-5, 2.5 µm, 0.5 mrad | 1e-3, 0.8 µm, 0.5 mrad | 3e-4, 3.7 µm, 2.7 mrad | 6e-4, 3.4 µm, 2.5 mrad |
| Demand: clusters, stretches apart; root sag | 0, 0; +2.8 % | 0, 0; +1e-5 | 0, 0; +5e-5 | 0, 0; +5e-5 |
| Leaves (submitted, CPU path) | 11,440, 11,437 | 2,349,014, 2,349,003 | 16,871,102, 16,871,100 | 16,806,937, 16,806,937 |
| Leaves compared, over the bound | 11,437, 0 | 2,349,003, 0 | 16,870,770, 0 | 16,806,937, 0 |
| Leaves on a run the CPU path lacks (likely class A) | 3 | 11 | 0 | 0 |
| Rounding class (count apart; over in direction) | 0 | 0 | 332; 42 | 0 |
| Face turned, grazing | 0, 0 | 2, 0 | 122, 0 | 128, 1 |

- **Every float class is the tasks' own:** the rounding case on spruce 80 s1 (task 6), the face turned about a near-vertical axis, the one grazing seat on spruce 80 s7, and its one parted run (TASK5's `SHORTEST` flip, run 62151, which task 8 removes). The device now settles the whole tree, so TASK5's socket-sample residual is gone (0 on every tree).
- **The demand's root sag** is conservative: the f32 tangents of straight stretches give each sag a little noise, and its root adds it up (+2.8 % on the beech's straight twigs, +5e-5 on the spruce). Clusters and stretches are exact.
- **Two classes are new and structural, not float.** They are cross-seam couplings the partial conversion exposes (for the host, below).

### The seams at a close camera (`seam_stills.rs`; `raw/task7b/stills2/`, viewed)

- The spruce at 80, seed 1: two device-drawn roots' seams on outer limbs of 3 to 15 mm at 0.4 to 0.7 of the height (nodes 129106 and 139878), 0.3 m off, bare and whole. The same cameras drew the CPU path's tree.
- **The device's and the CPU's stills are indistinguishable by eye.** The fine wood leaves its limbs at sockets with no gap, kink or step.
- **Pixels apart** (ImageMagick AE, of 786,432): bare 403 and 487 exact (0.06 %), 129 and 117 under 6 % fuzz; whole 0.7 % and 1.1 % exact, 0.1 % and 0.2 % under fuzz (leaves sitting differently by the face-turned and rounding classes).
- A first pair at the trunk (`stills/`) was too dark to judge a seam and was replaced.

## For the host: two dependencies cross the seam the wrong way

The partial conversion sees no device root's wood, so two things a bearer above the cut reads from its children differ from the whole tree's. Host decision 3 on task 6 made dependencies cross the seam one way only for bearing runs; these two are not covered by it.

**A. A node's kind** (`space.rs`, `convert`: `children[i] == 0` makes thin wood a twig). A bearer whose only children are device roots' first nodes is a branch in the whole tree and a twig in the part. Measured: 1 (beech 30), 92 (spruce 30), 322 and 323 (spruce 80) nodes, every one of them this case. 3 and 11 leaves at 30 sit on runs the CPU path does not have (none at 80). A twig bears leaves, so class A is the likely cause; not traced.

**B. The fork easing at a bearer** (`samples.rs`, `paths.forks`). The sweep eases a bearer's girth where a codominant child leaves it. A device root's first node can be that child. Measured: 89 to 2,996 such bearers. 12 runs on the beech at 30 are off in radius by up to 16 %; 1 run on each spruce at 80.

**Options, for each:**
1. **The seam is one-way here too:** a seam never counts among its bearer's children, for its kind (A) or its forks (B), on both executors, as decision 3 does for bearing runs. The CPU path's trees change at these bearers (A: a thin bearer whose only children are seams becomes a twig and bears leaves; B: no fork easing at a seam). No readback. Stills and counts before and after would be shown.
2. **One record per root up** (host decision 6): after the dress, the device hands each root whether its seam draws a node (A) and that node's codominance (B). The CPU part's kinds and its curve read them. Trees stay as they are. It costs one more round trip of a word per root (about 74,000 on the spruce at 80) and a pass over the bearers before the CPU block's curve, and it orders the CPU block after the dress (it already is).
3. **Decide them from data the CPU holds:** count every attachment as a child (A), and take a root's codominance from its base radius, handed up in phase 2 (B), on both executors. The trees change only where a root draws nothing at its seam or merges its first phytomers. No readback.

**The worker's reading:** option 1 matches decision 3's rule and costs nothing at run time. Option 2 keeps every tree as it is. The choice is the host's. Task 7 stays open on it: the acceptance's final-output parity holds by named class apart from A and B. The gate has not been run, since the answer may change trees.

## Option 3 built (host, 2026-10-07; `496e4c46`)

- **The base radius is available before conversion on both paths.** Phase 2's `Roots::girth` hands each live root's base radius to the settle on either executor. The settle used it locally and dropped it, so it now keeps it as `Settled::radii`. `grow_fine` (the CPU path, through `grow_marked`) and `partial` (the real path) carry it by the root's first axis's lineage. Nothing is substituted: it is the value phase 2 sends up.
- **One conversion, both paths:**
  - each attachment records `fork`: codominance of the root's base radius against its bearer's drawn node's radius;
  - a node's children for its kind are the ones that are not seams, plus one for each attachment;
  - `paths` takes a seam's fork from its attachment, never from its node.

**Bearers that moved on the CPU path** (`seams_read_by_their_attachments`; `raw/task7b/option3-bearers-2.txt`):

| Tree | Roots (draw nothing at the seam) | Kinds moved | Forks moved | of them at a root that draws a seam |
|---|--:|--:|--:|--:|
| beech 30 s1 | 143 (26) | 4 | 3 | 0 |
| beech 40 s1 | 463 (51) | 10 | 3 | 0 |
| beech 40 s7 | 819 (100) | 10 | 0 | 0 |
| beech 80 s1 | 4,973 (382) | 84 | 4 | 2 |
| beech 80 s7 | 8,061 (654) | 122 | 14 | 0 |
| spruce 30 s1 | 9,337 (118) | 1 | 0 | 0 |
| spruce 40 s1 | 18,869 (202) | 2 | 3 | 0 |
| spruce 40 s7 | 18,190 (174) | 1 | 0 | 0 |
| spruce 80 s1 | 72,870 (910) | 5 | 5 | 0 |
| spruce 80 s7 | 72,021 (617) | 2 | 3 | 0 |

- **As the host expected,** apart from 2 forks on the beech at 80 s1: those roots draw a seam, and their base radius and their seam node's radius read on either side of a codominance threshold (a merged first phytomer, or the pipe's secondary share; not traced further).
- **The beech-30 runs that moved** (`option3_moved_runs_on_the_beech`; `raw/task7b/option3/`, viewed):
  - On the CPU path, no run moves at 30. Its curve is the same before and after option 3: the 3 moved forks stand where no run eases them.
  - The runs that moved are the real path's: the 12 of class B. The part's curve with no fine root forking its bearers, against option 3: 12 runs, radius up to 19 %.
  - The most moved one (run 20, at the trunk's fork at 6.19 m) was drawn bare before and after: 133 pixels apart, indistinguishable by eye. The real path and the CPU path at the same camera are 2 pixels apart.
- **The extra leaves at 30 are class A** (`option3_extra_leaves_at_30`). The real path's part with its kinds read as before (a thin node is a twig where it draws no child) stations 862 leaves in its CPU block, against 859 under option 3 (beech), and 185,047 against 185,036 (spruce). That is exactly 3 and 11 more, and every one is on a segment ending at a node whose kind option 3 moved: 25 against 22 there (beech), and 725 against 714 (spruce).

**Parity after option 3** (`fast-3.txt`, `parity-80-option3.txt`):

| | beech 30 | spruce 30 | spruce 80 s1 | spruce 80 s7 |
|---|--:|--:|--:|--:|
| Kinds apart | 0 | 0 | 0 | 0 |
| Runs off in radius at a fork | 0 | 0 | 0 | 0 |
| Leaves on runs the CPU path lacks | 0 | 0 | 0 | 0 |
| Leaves (submitted, CPU path) | 11,437, 11,437 | 2,349,003, 2,349,003 | 16,871,102, 16,871,100 | 16,806,937, 16,806,937 |

- What remains is the tasks' named float classes:
  - the rounding case on spruce 80 s1, whose 2 extra leaves come from 2 keys counting differently, as in task 6;
  - the face turned about a near-vertical axis (2, 122 and 128 leaves);
  - the grazing seat and the `SHORTEST`-flip run on spruce 80 s7.
- Every run is matched, and none is over the screen beyond these. The fast test (`the_real_path_matches_the_cpu_path`) now holds every run matched and is in the gate.

### Gate (after option 3)

- **Render half,** under the GPU lock, first: 162 passed, 0 failed (`raw/task7b/gate-render.txt`).
- **CPU half,** the workspace without the render crate: 988 passed, 0 failed (`raw/task7b/gate-cpu.txt`).
- `RAYON_NUM_THREADS`, `RUST_TEST_THREADS` and `CARGO_BUILD_JOBS` were 8.

### The acceptance

- **The 3× verdict with its profile:** 7.7× (seed 1) and 7.1× (seed 7), stage medians above.
- **Final-output parity:** by named class, after option 3.
- **Seam close-ups:** viewed.
- **Two-part gate:** green.
