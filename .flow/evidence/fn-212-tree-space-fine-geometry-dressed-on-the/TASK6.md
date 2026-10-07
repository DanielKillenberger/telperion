# fn-212 task 6: stations and contact rings on the device, three questions first (worker, 2026-10-07)

Task 6 is started and nothing of it is built. Reading the station path for fine roots on the device turned up three decisions that set what a seated leaf becomes on both executors. They are the host's. Each cites the code it was checked against, at `52310b6f`.

## What the device would replicate

The resident path's stations are `Expansion::compact_stations` → `prepare_compact_stations` (`foliage/prepared.rs:103`). It works in five steps:

1. **Bearing runs** (`plan::bearing_runs`, `plan.rs:270`).
   - A node bears when it is a twig: under `STRUCTURAL` of the root's radius and childless. A node also bears when its pipe radii are within the slender bound (`tree.stem_radius` × `shoot_radius`).
   - A run continues from a bearing parent into a bearing child that is on the parent's branch, where the parent has exactly one bearing child.
2. **Station counts per run** (`plan::runs`): `ceil(length / internode − 1e-9)` internodes, times the stations per internode.
3. **Station frames per run** (`station::station_frame_iter`, `station.rs:304`).
4. **Tiles of up to 256 stations per segment** (`prepared.rs:167-221`), each with its contact rings `[lower, upper, first, last]` read from the compact rings, which are the curve's points.
5. **Placement on the GPU** (`pack.wgsl`).

The device has every input this needs:
- the drawn nodes and their kinds (root radius handed down);
- their branch ids, which `dress.wgsl` would carry as global ids;
- the curve's points, which give the contact rings;
- the slender bound, handed down as one number.

## Question 1: a leaf's randomness reads its station's place in the whole tree

- **The finding.** `pack.wgsl`'s `station(index)` draws a leaf's scatter and its scale from `random(index · k)` (`pack.wgsl:31`, `:66-73`). `index` is the station's place among all of the tree's stations.
- **How the CPU numbers stations.** Runs are taken in the order of their first node's index (`plan::runs`), and the CPU's node order interleaves the fine roots' nodes with the structure above the cut's. Conversion takes axes breadth first by lineage, so one fine root's nodes are not contiguous.
- **Consequence:** the device cannot number a fine root's stations as the CPU does without the CPU's numbering of every run. So a fine leaf's scale and scatter cannot equal the CPU reference's unless the numbering changes.
- **Options:**
  - (a) **D5 for stations.** The structure above the cut's stations come first, then the fine roots', in an order both executors make: the fine runs' first nodes by depth, then lineage, as `dress.wgsl`'s tie-break already orders them. Every engine tree's fine leaves draw new random values: statistically the same, in different bytes.
  - (b) **Key a leaf's random values by its station's identity.** The run's first node's lineage key and the station's place within its run would replace the global place. This holds whatever the order, but it is a change to `pack.wgsl`'s stream for every tree, the scaffold's included.
  - (c) **Hand the device each fine run's station base from the CPU.** This needs the run list on the CPU, which per-run readback rules out. Not proposed.

## Question 2: station frames seed by `perpendicular` too

- **The finding.** `station_frame_iter` seeds each bearing run's normal with `tangent.perpendicular()` (`station.rs:327`). This is the seed host decision 1 found defective for the sweep: near-equal components flip the axis it crosses with, by float32 noise, and every station of the run turns about 90°.
- **Proposal:** carry decision 1 to station frames. A fine bearing run's first normal would be its first node's (the attachment's) laid side, projected off the first tangent. This moves every fine leaf's turn on both executors.
- **Unknown:** how many station runs sit near the seed's jump. It would be measured as for the sweep.

## Question 3: a bearing run on the CPU counts its bearer's bearing seam children

- **The finding.** `bearing_runs` continues a run from a parent into its one bearing child (`children[parent].0 == 1`, `plan.rs:291-295`). A bearer above the cut that has a bearing seam child (a fine root's first node) counts it. So whether the CPU's own run continues past that node depends on a fine root's bearing, which only the device knows.
  - A seam never continues a run itself: it is a lateral's first node and starts a branch of its own.
- **Options:**
  - (a) **A seam never counts among its bearer's bearing children** (D4 for bearing runs). This changes the CPU's runs where a bearer has one bearing child above the cut and a bearing seam.
  - (b) **The device hands each root's bearing-seam flag up as a record per root** (Astra change 3's "a record per root"), and the CPU counts it.
- **Unknown:** how many bearers this touches. That needs `tree.stem_radius`, which is `pub(crate)` in core; it was not measured.

## Not affected by these questions

- **D2:** counts come from today's radius and length until contract A. Bearing (radius against the slender bound) and the station count (length against the internode) are threshold decisions fed by f32 against f64, the class the host named on 2026-10-07. Task 7 counts them.
- **The leaf box's reach terms** come from the combined envelope in production; that goes to task 7.

## After the host's decisions: what was built and measured (worker, 2026-10-07)

### The three decisions, on both executors (`357515c2`)

- **1. Leaves draw by station identity.**
  - Each bearing run's leaves draw from their own stream: `tree::leaf_stream(seed, key)`, where the key is `Tree::keys[first node]` (`node_key` of the axis's lineage and the phytomer's place along the axis). The CPU placement creates one `Rng` per run, and `pack.wgsl` reads the key from the segment (`range.w`) with an ordinal of the station's place in its run.
  - **Unchanged in law** (`stations_measured_at_80`): the Kolmogorov-Smirnov distance from uniform of every leaf's scale draw and first scatter draw, global stream before and per-run streams after.

    | Tree | Leaves | Scale draw, before | Scale draw, after | Scatter draw, before | Scatter draw, after |
    |---|--:|--:|--:|--:|--:|
    | beech 80 s1 | 2,474,018 | 5.9e-4 | 7.2e-4 | 8.2e-4 | 5.4e-4 |
    | spruce 80 s1 | 16,911,023 | 1.6e-4 | 2.4e-4 | 1.3e-4 | 1.7e-4 |

    The 95 % critical values are 8.6e-4 (beech) and 3.3e-4 (spruce). Scale and scatter are affine and spherical maps of these draws, so their laws are unchanged.
  - **The scaffold birch's pins moved with its stream:** its bounds by 1.3 mm, its placement hash, and its twelve seed digests (`digests.json`).
- **2. The side seed for station frames.**
  - A fine bearing run's first station normal is its attachment's laid side, projected off the first tangent (`station_frame_iter`).
  - **Runs near the seed's jump** (first tangent within 10°, 5° and 1° of the side):
    - beech 80 s1: 0, 0 and 0 of 227,844;
    - spruce 80 s1: 24, 1 and 0 of 1,069,861.
  - The old seed had 5,727 (beech) and 26,308 (spruce) runs within 1e-2 of its axis tie.
- **3. A seam never counts among its bearer's bearing children** (`plan::bearing_children`).
  - **Bearers changed:** 2,251 on the beech at 80 s1, of which 1,340 now carry their run on; 36,020 on the spruce, of which 4,684 now carry on.
  - Placed leaves moved from 2,474,647 to 2,474,018 (beech) and from 16,912,892 to 16,911,023 (spruce).
- **Stills at 80, seed 1, before and after these three** (`raw/task6/stills/`): all close-ups and the hero afters were viewed; the hero befores are TASK5.md's afters.
  - The tree's character is the same. Individual leaves sit and scale differently, as their new draws give.
  - Pixels apart, under 6 % fuzz: beech hero 0.95 %, close 3.4 %; spruce hero 0.54 %, close 3.7 %.
- **The slender and twig bounds** reach the device through `Expansion::bearing_bounds` (`[STRUCTURAL × root, slender]`). This is an input the device needs, as `wood_reach` is.

### Stations on the device (`2981ccb7`, `stations.wgsl`)

The shader works in these steps:
- **Bearing runs:** `kids`, `bear` and `starts`. Kinds come from children counted on the device and from the bounds handed down; seams are left out of a bearer's bearing children.
- **Station counts:** `measure`, then `bases` numbers the runs in any order, since leaves draw by key.
- **Segments:** `emit` writes them in `data.rs`'s layout, with frames from the attachment's side, tiles of 256, the run's first node's key, and contact rings as vertex places in the fine block's compact rings (each node's ring recorded by `sweep.wgsl`'s `emit`).

Probe (`stationed_at_40_and_80`, `raw/task6/stationed-3.txt`). Segments are matched by key and first station along the run:

| Tree | Device runs / segments / stations | Reference segments / stations (device keys) | Runs counted differently (stations) | Segments split differently | Geometry where split alike |
|---|---|---|--:|--:|---|
| beech 30 s1 | 2,457 / 7,242 / 10,729 | 7,239 / 10,567 | 156 (382) | 5 | endpoints 1.9 µm, frames 6e-4 rad |
| spruce 30 s1 | 174,450 / 509,254 / 2,169,101 | 509,252 / 2,169,108 | 18 (809) | 241 | 2.3 µm, 1.3e-3 rad |
| beech 40 s1 | 12,066 / 42,963 / 64,102 | 42,878 / 63,249 | 847 (5,405) | 729 | see below |
| beech 80 s1 | 226,569 / 1,103,972 / 2,441,607 | 1,130,519 / 2,409,915 | 31,624 (350,092) | 50,562 | see below |
| spruce 80 s1 | 1,013,164 / 4,199,621 / 15,672,858 | 4,199,428 / 15,672,152 | 217 (8,073) | 2,831 | see below |
| spruce 80 s7 | 1,023,708 / 4,224,170 / 15,792,921 | 4,224,110 / 15,792,601 | 217 (9,553) | 5,255 | see below |

- **At 30 the geometry agrees.** Where segments split alike, the beech and spruce are within 2 µm and 1.3e-3 rad, and the contacts name the same rings of their run.
- **From 40 on, matching breaks down.** The beech 40 to 80 and the spruce 80 show endpoints up to 6 mm, radii up to 3.9 relative and frames up to 0.1 rad. Segments with equal counts get matched to the wrong segment where a split moved by a node, so these figures are not a geometric error.
- **Contacts:** 27 at the spruce 40 s1 and 4 at the spruce 80 name other rings, on runs whose curve run parted (TASK5.md's residuals).

### For the host: whole-internode ties in the station count and split

- **The finding.**
  - `plan::runs` counts `ceil(length / internode − 1e-9)` internodes, and `stations_before` splits a run at `mid × internode < along`.
  - The beech's fine internodes are the twig layer's internode, 6 mm at full scale. So a run of one straight full-scale phytomer is 6.000000 mm, a whole number of internodes in f64 to within 1e-16, and the CPU's 1e-9 keeps it at one.
  - On the device the length is a float32 distance between float32 positions in the tree's frame: 6.0000x mm. That flips the ceiling to two.
  - The split points (`along` at interior nodes) tie the same way.
- **The extent:**
  - beech at 80: 14 % of its fine bearing runs (31,624 of 226,569) count differently, and the device places 1.3 % more stations;
  - spruce: 217 runs.
- **This is the named class** (threshold decisions fed by f32 against f64), but here the ties are exact and of the code's own making, as the beech's `FORK_FROM` codominance was. Contract A's fixed-point layout would make both sides' lengths exact, and therefore equal, if it carries lengths to the stations.
- **Options:**
  - (a) Count and split by the grown length: the phytomers' internode × scale, summed. That is record data, as D2 asks, and exact on both. It changes the CPU where a run bends: chord against arc, up to a few per cent of a curved run.
  - (b) A tolerance both executors share, wider than their disagreement, such as `ceil(length / internode × (1 − 1e-5))`. It moves the CPU only where a run lies within 1e-5 of a whole number of internodes, which is exactly these ties.
  - (c) Wait for contract A and keep the device as it is.
- **Still to build,** whichever the choice: placement of the fine segments' leaves on the device, and the seated leaves against the reference by key and station (the acceptance's "within fn-125's bound"). Task 6 is not done.

### Gate (task 6 so far)

- **CPU half:** 987 passed, then 1 red. The scaffold birch's catalogue record (`catalogue/silver-birch/pins.json`) still held the old leaf stream's bounds and placement. It was re-pinned with `identity.rs`'s values, and the identity tests now pass.
- **Render half,** under the GPU lock at eight test threads: 160 passed.

## Grown length: counts and splits by record data (worker, 2026-10-07, host decision (a))

### What was built

- **The record.** `Tree::grown` holds, per node, the internodes of the phytomers it draws, each its PA's length at its scale (`space.rs`, `convert`). Phytomers merged into a node are carried to the next node drawn. A tree no engine grew reads its chord (`Tree::grown_length`).
- **One rule, both executors.**
  - `plan::walk` measures a run: the chord for its frames, and the grown wood for its stations.
  - Under a twig layer, each node's grown wood is rounded to whole micrometres (`plan::micrometres`), and the sums are exact.
  - Counts are `ceil(Σ / internode)`, and splits are `mid × internode < along`, all in micrometres.
  - `stations.wgsl` does the same in `u32`, from a new word on each drawn record (`dress.wgsl` reads each PA's internode from the geometry table).
- **Why micrometres.** In plain float sums, f32 and f64 disagree at real ties:
  - the spruce's fine PA is 3.25 twig internodes, so four nodes sum to exactly 13 (at 30: 22 keys and 296 split segments);
  - a dyadic grid (1/4096 internode) breaks the beech's 25 mm PA, which is 25/6 of the twig layer's 6 mm, and moved 2,755 beech runs at 80;
  - the internodes are decimal lengths, so whole micrometres keep every such ratio exact.
- **Positions.** The prepared segment's `along` and `span` (what `pack.wgsl` reads) are now micrometres × 1e-6 of grown wood. Frames still read the chord.

### CPU: stations per tree, chord (before) against grown (after) (`grown_stations_at_80`, `raw/task6b/grown-stations-3.txt`)

| Tree | Bearing runs | Stations before | Stations after | Change | Runs counted differently (more / fewer) | Equal count, split elsewhere |
|---|--:|--:|--:|--:|--:|--:|
| beech 80 s1 | 229,177 | 2,474,018 | 2,474,226 | +0.008 % | 222 (160 / 62) | 39,155 |
| beech 80 s7 | 390,776 | 4,295,198 | 4,294,864 | −0.008 % | 413 (259 / 154) | 57,781 |
| spruce 80 s1 | 1,070,192 | 16,911,023 | 16,871,064 | −0.236 % | 19,616 (797 / 18,819) | 8,879 |
| spruce 80 s7 | 1,073,177 | 16,846,937 | 16,806,937 | −0.237 % | 19,584 (812 / 18,772) | 10,321 |

- **The spruce loses runs mostly** because its runs' chords are longer than their grown wood: the chord from a bearer's node takes in what the conversion merged into that node.
- **The beech's many "split elsewhere"** are the whole-internode ties themselves. A station exactly at a node now always opens the next segment, where f64 noise used to place it on either side. Its place on the wood is the same.
- **The scaffold birch** is measured by its chord in micrometres. Its placement hash and twelve seed digests move (`identity.rs`, `catalogue/silver-birch/pins.json`, `digests.json`); its bounds do not.

### Stills at 80, seed 1 (`raw/task6b/stills/`, viewed as before|after pairs)

- **Before** is `raw/task6/bin/headless-after`, HEAD's product. **After** is `raw/task6b/bin/headless-after`.
- **Hero and close** (`--camera '{"fill": 10, "targetHeight": 0.45, "elevation": 5}'`, leaves shown), beech and spruce, all four pairs viewed: indistinguishable by eye.
- **Instances:** beech 2,474,226 and spruce 16,871,064, the CPU counts above.

| Still | Pixels apart, exact | Under 6 % fuzz |
|---|--:|--:|
| beech hero | 0.42 % | 0.12 % |
| beech close | 0.55 % | 0.17 % |
| spruce hero | 0.26 % | 0.06 % |
| spruce close | 1.05 % | 0.44 % |

### Device against the reference (`stationed_at_40_and_80`, `raw/task6b/stationed-80.txt`; fast test at 30 in `stationed-30.txt`)

The test now matches a key's every run. 32-bit keys collide, about runs²/2³³ pairs, so two runs can share a key; the four "tied" runs at 30 were such pairs. Two runs that share a key draw the same leaf stream (host decision 1).

| Tree | Device runs / segments / stations | Reference segments / stations | Keys counted differently (device stations) | Split elsewhere | Residual keys by class: rounding / parted |
|---|---|---|--:|--:|---|
| beech 30 s1 | 2,457 / 7,243 / 10,578 | 7,243 / 10,578 | 0 | 0 | 0 / 0 |
| spruce 30 s1 | 174,450 / 509,317 / 2,163,967 | 509,317 / 2,163,967 | 0 | 0 | 0 / 0 |
| beech 40 s1, s7 | 12,066 / 43,702 / 63,256; 21,256 / 115,117 / 191,948 | equal | 0 | 0 | 0 / 0 |
| spruce 40 s1, s7 | 325,437 / 1,052,373 / 4,305,432; 315,135 / 1,031,333 / 4,233,230 | equal | 0 | 0 | 0 / 0 |
| beech 80 s1 | 226,569 / 1,157,539 / 2,409,975 | 1,157,568 / 2,410,077 | 1 (8) | 0 | see below |
| beech 80 s7 | 385,593 / 1,956,230 / 4,170,671 | equal | 0 | 0 | 0 / 0 |
| spruce 80 s1 | 1,013,164 / 4,200,343 / 15,635,597 | 4,200,369 / 15,635,810 | 20 (501) | 222 | see below |
| spruce 80 s7 | 1,023,708 / 4,224,869 / 15,755,194 | 4,224,972 / 15,755,520 | 11 (133) | 0 | see below |

Before this change the same probe had 31,624 beech runs at 80 s1 counted differently (TASK6.md, above).

**The residuals, by class** (`raw/task6b/stationed-80-diag2.txt` to `diag5.txt`):

1. **Rounding, the f32-against-f64 class.** A node's grown wood lies within float32's error of a half micrometre, and the executors round it apart.
   - Spruce 80 s1: all 222 split segments (111 runs) and 2 of the 20 keys. They are one recurring value, 14,666.499 µm, which is 0.001 µm under the half. Float32's step there is 0.0009 µm.
2. **Parted runs, not located.** A run continues on one executor and stops on the other, where the stopped side starts a new run.
   - Counts: beech 80 s1, 1 key; spruce 80 s1, 18 keys; spruce 80 s7, 11 keys.
   - The device's endpoints sit on the reference's within micrometres.
   - **What it is not:** the nodes involved lie 15 to 86 % from both bearing bounds (slender and `STRUCTURAL`), so this is not those bounds fed by f32.
   - **Unknown:** which bearing-child count differs. Locating it needs the device's per-node station words read back, which was not done.

## Seated leaves on the device against the reference (worker, 2026-10-07)

### What was built

- **`rings.wgsl`** writes the fine block's ring words from the device's curve, in `compact::read`'s layout: centre and radius, normal and along, binormal and phase, and vertex and cap offsets in the fine block.
- **The positions pass's own `emit`** lays them as vertices (`Generator::ring_vertices`, test-only until the handoff). Those vertices are what a seated leaf's contact reads.
- **`Generator::place_leaves`** is the placement dispatch, taken out of `compute_buffer_async`. The resident path and the fine block now share one placement pass, which takes its segments as a device buffer. `data::config` takes the counts, not the CPU's prepared stations. Production bytes are unchanged (render half green).
- **The probe** (`placed_tests.rs`).
  - The reference is the pipeline's own resident leaves: `compact_stations` placed on its emitted rings.
  - It is matched to the device run by run: key, first station, then the nearest first endpoint. Two runs that share a 32-bit key are told apart this way.
  - Leaves are compared station by station against fn-125's leaf bound in `generation/tests.rs`: position within two code steps of the leaf box, a column's direction within 0.005, scale within 1/1024.
  - The fast test (`fine_roots_are_seated_on_the_device`, beech and spruce at 30) holds that every leaf is compared and every leaf is in the bound or in a named class. It is in the gate.

### Measured (`seated_at_40_and_80`, `raw/task6b/seated-80-2.txt`; spruce 80 s1 with frames, `seated-spruce80-frames.txt`; 30 in `seated-30.txt`)

| Tree | Device leaves / reference leaves of its keys | Compared | Left: device-only run / count differs / reference-only run | Over: position / direction / scale | Classes: code step / face turned | Position max (bound) | ms |
|---|---|--:|---|---|---|---|--:|
| beech 30 s1 | 10,578 / 10,578 | 10,578 | 0 / 0 / – | 0 / 0 / 0 | 1 / 0 | 126 µm (375) | 4 |
| spruce 30 s1 | 2,163,967 / 2,163,967 | 2,163,967 | 0 / 0 / – | 0 / 0 / 0 | 15 / 13 | 176 µm (412) | 75 |
| beech 40 s1 | 63,256 / 63,256 | 63,256 | 0 / 0 / 0 | 0 / 0 / 0 | 1 / 4 | 156 µm (432) | 31 |
| beech 40 s7 | 191,948 / 191,948 | 191,948 | 0 / 0 / 0 | 0 / 0 / 0 | 0 / 4 | 200 µm (488) | 38 |
| beech 80 s1 | 2,409,975 / 2,410,077 | 2,409,975 | 0 / 0 / 102 | 0 / 0 / 0 | 30 / 78 | 408 µm (991) | 581 |
| beech 80 s7 | 4,170,671 / 4,170,671 | 4,170,671 | 0 / 0 / 0 | 0 / 0 / 0 | 45 / 164 | 498 µm (1,173) | 494 |
| spruce 40 s1 | 4,305,432 / 4,305,432 | 4,305,432 | 0 / 0 / 0 | 0 / **37** / 0 | 48 / 178 | 241 µm (557) | 450 |
| spruce 40 s7 | 4,233,230 / 4,233,230 | 4,233,230 | 0 / 0 / 0 | 0 / 0 / 0 | 43 / 157 | 238 µm (546) | 418 |
| spruce 80 s1 | 15,635,597 / 15,635,810 | 15,635,229 | 36 / 332 / 251 | 0 / **434** / 0 | 4,886 / 15,719 | 539 µm (1,093) | 832 |
| spruce 80 s7 | 15,755,194 / 15,755,520 | 15,755,194 | 0 / 0 / 326 | **1** / **300** / 0 | 4,544 / 14,502 | 1.55 mm (1,049) | 675 |

- **Scale agrees to 1.2e-7 everywhere,** so every leaf draws the reference's random values; the key-and-ordinal streams match. The placement flags (transform, contact, box) are clear on every tree.
- **"ms"** is the probe's wall clock from the sweep to the placed leaves read back, including allocation. It is not task 7's harness.

### The residual classes

1. **Code step** (counted, not over). Orientation words one code apart in each kept component. Frames a few microradians apart land either side of the 10-bit grid: the f32-against-f64 class at the encoding's thresholds.
2. **Face turned about the axis** (counted, not over). The axis column is within 0.005, and the face and side are turned about it.
   - `pack.wgsl` takes a leaf's face from `up - axis·axis.y` before scatter. Where that axis stands near the vertical, an axis difference of 1e-3 turns the face many times as far.
   - Scatter's draws are the reference's to the bit, so this is the rule's conditioning, fed by the frames' difference.
3. **Over in direction, carried from the station frames.** 433 of the 434 at spruce 80 s1 sit on segments whose normal is 1 mrad or more from the reference's, up to 0.025 rad.
   - The station probe saw the same frames: 0.015 to 0.016 rad on the spruce at 40 s1 and 80.
   - **Cause:** the frames are transported along chords of 1 mm and more, between nodes the device settles on its own loads, tens of micrometres from the CPU's (TASK5.md, residual 2). 50 µm across a 1 mm segment is up to 0.05 rad.
   - **For the host:** fn-125's orientation bound (0.005) sits below what the position screen (50 µm) allows on the shortest segments. Either bound would have to give, or the probe compares the device against a reference settled on the device's own loads.
4. **The one position over** (spruce 80 s7, 1.55 mm against 1.05 mm). Likely TASK5's `SHORTEST` flip, the point 1.0 mm off on the same tree. Not traced.
5. **Leaves left** are the station probe's classes: the parted runs (the device's shorter run and the reference's run with no device twin) and the rounding value on spruce 80 s1.

### Gate

- **Render half,** under the GPU lock at eight test threads: 161 passed, 0 failed (`raw/task6b/gate-render.txt`).
- **CPU half,** run in full again: 988 passed, 0 failed (`raw/task6b/gate-cpu.txt`).

### The acceptance, as it stands

- **Counts are equal at 30 and 40** on both trees, and on the beech at 80 s7.
- **At 80** they differ by the rounding class (spruce s1) and by the parted runs (1, 18 and 11 keys), whose cause is not located.
- **Leaves are within fn-125's bound,** apart from the named classes, on every beech and on the spruce at 30 and 40 s7. On the spruce at 40 s1 and at 80, 1 to 434 leaves are over in direction, carried from station frame differences that the position screen allows.
- **So `flowctl done` was not run.** The host's call: whether the frame-carried leaves and the parted runs are classes the acceptance names, or work for this task.
