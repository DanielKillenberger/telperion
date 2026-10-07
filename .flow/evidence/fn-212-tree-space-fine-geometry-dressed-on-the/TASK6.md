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
