# fn-248 R1: why the Two Trees' leaves sit on branches

Measured 2026-10-09 on branch `fn-248-leaves-hang-on-twigs` (base `c6cd30d3`).

Laurelin is the European beech at 80, drawn at scale 2.5. Telperion is the beech at 92, drawn at scale 2.4. Each preset's header says that with the supernatural group off it is "that beech", to the byte. The off build is therefore the requested natural base: the beech at the same age, at scale 1. Both trees use seed 1.

The leading hypothesis holds, but it is only half the cause. The scale makes every twig 2.4 to 2.5 times as thick and as long, while the leaf and its petiole keep their natural size. That is what turns the twigs into branches on the Two Trees. A second cause is already present at scale 1, in the plain beech. Its leaves stand one every 5.7 mm along all slender wood, and about four in five of them sit on wood older than the current year. The scale then multiplies that sheath of leaves by 2.5.

## How it was measured

- Numbers: `crates/telperion-core/examples/twig_layer.rs`, run as `cargo run --profile ci -p telperion-core --example twig_layer -- <preset> <age> [seed]`. The example does two things:
  - It builds the tree through `pipeline::build` and reads the leaf plan's descriptors. Each descriptor is one bearing segment with its station count and its drawn radii. The example matches each descriptor to its node.
  - It regrows the engine structure at natural size to recover each node's wood age, as structure age minus phytomer cycle. The conversion does not carry age into `Tree`.
- Raw output: `raw/twig_layer.log`.
- Stills: `crates/telperion-render/examples/space_two_trees.rs`, run on the device (whole, mid and close views, group off and on). They are in `raw/stills/`, and every still was viewed.
- Comparison sheet: `TWIGS-sheet.jpg`. It sets the close and mid views beside two real beech shoots from Wikimedia Commons (CC0; `File:20170921Fagus sylvatica2.jpg` and `File:20170921Fagus sylvatica1.jpg`; EXIF stripped; local copies in `raw/refs/`).

## Numbers (seed 1)

| | beech 80 (scale 1) | Laurelin 80 (2.5) | beech 92 (scale 1) | Telperion 92 (2.4) |
|---|---|---|---|---|
| Planned leaves | 2.49 M | 6.13 M (2.46×) | 3.85 M | 9.05 M (2.35×) |
| Wood nodes (same topology) | 1.54 M | 1.54 M | 2.26 M | 2.26 M |
| Current-year shoot diameter, median (orders 3 to 7) | 2.6 mm | 6.4 to 6.6 mm | 2.1 to 2.6 mm | 5.1 to 6.1 mm |
| 1 to 3-year wood diameter, median (orders 5 to 7) | 4.4 to 5.0 mm | 11.0 to 12.6 mm | 4.4 to 5.2 mm | 10.6 to 12.4 mm |
| Current-year shoot length per run, p10 / median / p90 | 7 / 24 / 75 mm | 18 / 60 / 188 mm | 7 / 24 / 75 mm | 17 / 58 / 180 mm |
| Seat diameter under a leaf, p10 / median / p90 | 2.3 / 6.1 / 13.5 mm | 5.7 / 15.2 / 33.9 mm | 2.3 / 6.4 / 14.9 mm | 5.5 / 15.5 / 35.9 mm |
| Leaf blade (88 mm) / median seat diameter | 14 | 6 | 14 | 6 |
| Petiole (8.8 mm) / median seat diameter | 1.4 | 0.6 | 1.4 | 0.6 |
| Leaves on wood of radius 4 mm or less | 63 % | 21 % | 59 % | 20 % |
| Leaf spacing along bearing wood | 5.7 mm (177 per m) | 5.9 mm | 5.7 mm | 5.9 mm |

Leaves by the age of the wood they sit on (scale-invariant; the age shares match with the group on):

| Wood age | beech 80 | beech 92 |
|---|---|---|
| Current year | 21 % | 20 % |
| 1 to 3 years | 50 % | 48 % |
| 4 years and older | 29 % | 32 % |

Laurelin's leaves by drawn radius class (mm, upper bound) and wood age (full table in the log):

| Radius ≤ | beech 80: age 0 / 1-3 / 4+ | Laurelin 80: age 0 / 1-3 / 4+ |
|---|---|---|
| 1 | 81 k / 65 k / 11 k | 27 k / 15 k / 3 k |
| 2 | 350 k / 167 k / 49 k | 119 k / 72 k / 15 k |
| 4 | 95 k / 614 k / 141 k | 622 k / 314 k / 70 k |
| 8 | 0 / 385 k / 466 k | 532 k / 1,211 k / 237 k |
| 16 | 0 / 18 k / 52 k | 0 / 1,188 k / 905 k |
| 32 | 0 / 0 / 0.5 k | 0 / 250 k / 544 k |

For reference, these are typical values for a real beech shoot. They are not measured in this task and come from no cited source, so they are marked unknown until checked against a source: current-year shoots about 2 to 3 mm thick; leaves 5 to 10 cm on a 5 to 15 mm petiole; alternate, roughly 1.5 to 4 cm apart on long shoots; two to four leaves at the tip of a short shoot. Beech is deciduous, so every leaf grows on current-year wood. On older wood, a leaf can only come from a short shoot, a few millimetres long, that bears its leaves at the tip.

## The stills

- **Close, scale 1** (sheet row 1 and row 2, left): the leaf at the centre sits on a thin twig. The sprays behind it read as caterpillars or fronds: long shoots fringed with leaves every few millimetres, with no twig visible between them.
- **Close, scale 2.5 / 2.4** (sheet middle column): the leaves lie flat along a shoot as thick as a finger. A petiole shorter than the shoot is thick leaves the blade's base on the bark. The hanging sprays are long, densely fringed ropes. Laurelin's racemes and Telperion's florets add to the crowding but do not cause it.
- **Mid**: at scale 1 the leaves already form two-ranked fronds along the shoots. At 2.5 the crown reads as fern or hemlock sprays on bare, brown, finger-thick wood, with no fine twig layer between the leaves and the branches.
- **Whole**: the Two Trees' crowns are as dense as the beech's. The fine texture is a uniform needle-like fringe, not twigs.

## The cause, checked against the code

1. **The scale (fn-217 S1) acts on all wood alike and on no leaf** (the hypothesis, confirmed).
   - `pipeline/space.rs` `scaled()` multiplies every node's position, `radius`, `start_radius` and `base_radius`, and every value in `Tree::grown`, by the scale after conversion. The finest order is treated exactly like the trunk.
   - The device does the same: `generation/fine/dress.wgsl` uses `cfg.scale` on `pos` and `thick`, and `the_real_path_matches_the_cpu_path_at_every_scale` pins this.
   - The schema (`supernatural.rs`, the `scale` row) states that "Leaves and flowers keep their own size; the leaves keep their spacing along the wood."
   - So a twig's diameter grows 2.5 times (2.6 to 6.4 mm), its length grows 2.5 times, and the number of leaves on it grows 2.5 times. Meanwhile the blade stays 88 mm and the petiole 8.8 mm.
2. **Leaves stand every 6 mm along all slender wood, whatever its age** (present at scale 1, and amplified by the scale).
   - `foliage/plan.rs` `bearing_of` makes a node leaf-bearing when it is a `Twig`, or when its radius is at most `canopy.shootRadius` (0.018) × the stem radius. That test is relative, so it is unchanged by the scale. It never reads the wood's age.
   - `runs` places one station every `skeleton.twigs.twig.internodeLength` along the grown wood, and the grown wood is scaled. The beech sets that length to `0.006`.
   - The beech value file comments on that row: "with a leaf every 5 cm. Two-ranked at 32 degrees with a leaf every 2 cm, every spray was a flat frond and the crown read as a hemlock (owner and host, round 11)". The value was 0.05 until fn-198 (`d512accf`, 2026-10-06), which moved the beech onto the engine and set it to 0.006 without changing the comment. The comment and the value disagree.
   - `plan.rs`'s `micrometres` note explains the choice: the beech's 25 mm internode is 25/6 of 6 mm, so stations count exactly on both executors.
   - The beech states no fn-223 clusters ("no clusters seated along slender wood"), so no leaves are borne on short shoots.
3. **Nothing drops the thin wood.** The CPU curve keeps every run. Tessellation culls by frustum and screen error only, and draws a ring under 1 px as a ribbon. `render/src/select.rs` is leaf level-of-detail only. The twig layer is grown and drawn. It is hidden under the leaves (cause 2) and, at scale, it is too thick to read as twigs (cause 1).
4. **The shoot colour is lost at scale.** The beech's `material.shootRadius` (4 mm) is natural and is not scaled. It colours wood thinner than 4 mm radius as a shoot. At scale 1, 63 % of the leaves sit on such wood; at 2.5, only 21 % do. The rest sit on bark-coloured wood.

## Not done here (the host's call)

The fix's design, as the task asked: how the scale should apply to the finest orders, whether the station spacing goes back toward 5 cm or becomes age-bound, and whether leaves leave the older wood (for example through clusters on short shoots). The spec's sweep of the other species at 40 and 80 (seeds 1, 7 and 42) was not run either: the task asked to stop after the Two Trees. The example measures any engine preset as it stands.

## Host design (2026-10-09), before Astra's review

I viewed `TWIGS-sheet.jpg`. The engine's shoots carry a solid fringe of leaves, like caterpillars or fern fronds. The real beech shoots carry leaves 2-4 cm apart, alternating on thin twigs, each on a visible petiole. The scale makes the twigs read as branches. Three parts, each by degree:

1. **Leaves follow wood age, not wood radius.** A per-PA engine row, `leaf_years`, sets how many years a phytomer bears its leaf: 1 means current-year wood only, and fractional values blend by degree. At the default (unbounded), today's radius rule decides, and every other species stays byte-identical.
   - **The beech:** `leaf_years` = 1. Its spurs (short shoots) carry the leaves on older wood through fn-223's clusters: `cluster_leaves` 2-3, sourced in the beech's model.
2. **Spacing is the beech's own.** Long-shoot leaf spacing goes back toward the sourced 2-4 cm. The `internodeLength` row's own comment says 5 cm. Round 11's "hemlock" reading came from leaves on every wood age, which part 1 removes. Set the value on stills against the reference shoots.
3. **The scale fades on the finest wood.** A giant tree carries more orders of ordinary twigs, not giant twigs.
   - **The rule:** each node's scale factor falls continuously from `s`, on wood thicker than a stated natural radius (about 2 cm), to 1 on the current-year shoot's natural radius. It is a function of the node's natural radius, applied to its own length and radius relative to its parent's attachment, so the tree stays connected.
   - **Executors:** CPU (`scaled()`) and device (`dress.wgsl`) apply one rule.
   - **At scale 1** the output is byte-identical.

Order of work: build part 3 first and measure, then parts 1 and 2. Stills follow each part.

## Host decision (2026-10-09), after Astra's review (ASTRA-TWIGS.md)

Astra's order is adopted: fix the natural tree's foliage first, then measure whether the scale still needs work. "Question, delete, then optimise."

1. **Leaf retention by wood age.**
   - **The rule:** eligibility is today's radius rule × retention, where retention = clamp(`leaf_years` − wood age, 0, 1). It never switches between ways of building.
   - **The default:** with `leaf_years` unbounded, retention is 1 everywhere, and station positions, keys and draws are byte-identical.
   - **Wood age:** each phytomer's birth cycle is carried through conversion on both executors, as eligible spans or a per-station retention. Merged phytomers keep each span's own age, so a station takes the age of the wood it actually sits on.
   - **Fractional retention:** a station is kept by a lineage-keyed draw against its retention, and it fades in by degree the way making draws do (decision 8).
   - **The beech's old wood:** it carries its leaves on its existing short-shoot PA 14, through `cluster_leaves`. The cluster count is a sourced whole value; a stopped short-shoot tip keeps its cluster while it lives, and that rule is stated.
2. **Spacing.** This is a preset change, not a new mechanism. Correct the stale "5 cm" comment. Choose the value on stills against the reference shoots, after part 1 lands. Candidates are 2-4 cm, sourced or marked estimated.
3. **The scale.** Measure after parts 1 and 2. If the Two Trees' twigs still read as branches, the route is Astra's cleaner candidate: the Two Trees enlarge through their own per-PA geometry, with coarse PAs' internodes and pipes larger and fine PAs at ordinary size, and the one engine computes the rest. That revises fn-217's uniform scale for them and goes through the host first. No post-growth deformation is built.

Proof for parts 1 and 2:
- red/green tests;
- byte identity at neutral values (`space_hash` and stills for every other species);
- CPU and device agreement on the stations;
- the beech, Laurelin and Telperion stills before and after, viewed beside the reference shoots;
- the cost;
- then the gate, npm test and impl-review.

Keep the task open for the owner.

## Before part 1: what the code says (worker, 2026-10-09)

I read the files that part 1 touches before building anything:

- the conversion, `pipeline/space.rs`;
- the leaf plan, stations and placement: `foliage/plan.rs`, `station.rs`, `placement.rs`;
- on the device: `dress.wgsl`, `stations.wgsl` and the draws in `pack.wgsl`;
- the beech's PA 14 rows and their sources (fn-193 `SOURCES.md`).

None of it contradicts the decision. Six points do decide how part 1 gets built:

1. **Retention must reach the counts, not only the placement.** The plan's descriptors, `leaves_on`, the instance reservation and the device's `measure` all count stations "before any cull", and the field is built from those counts. If retention only removed leaves at placement, the field would count about five times the beech's real leaves. Retention therefore goes into every count as well, and the counts become the kept stations.
2. **A dropped station keeps its draws.** Both executors draw a leaf's values from its run's stream by ordinal: the CPU's `Rng` is a Weyl counter, and the device uses `random(key, ordinal)`. If a dropped station still consumes its draws, every leaf that stays keeps its own draws and keys. Fading in then changes only the leaves it drops, and at the unbounded default nothing moves.
3. **Mixed ages inside one node are rare but real.** In the beech at 80 (seed 1), 15,485 of 1,540,442 drawn nodes merge several phytomers, and 2,834 of them (0.18%) merge phytomers of different cycles. Each of those needs its age spans recorded: the length in whole micrometres where its age changes, plus its youngest cycle. That is two more words per node on both executors. On the device this means the 30-word conversion record (`CV`) and the 31-word sweep node (`NW`).
   - **Rounding risk:** each span boundary is a partial sum that the CPU takes in f64 and the device in f32. Where the fn-222 spread factor applies, that sum is not an exact multiple, so on a station lying within about 1 µm of a boundary the two executors could assign different ages. The parity test will show whether this ever happens.
4. **A fraction is drawn against a whole-number threshold.** A retention strictly between 0 and 1 falls on one age only: floor(`leaf_years`). The CPU will hand each PA that age and its retention as a Q32 threshold (decision 8), so the f32 device and the f64 CPU compare one integer. The `leaf_years` row mixes on the `Kept` scale, which already carries an unbounded value.
5. **Short shoots already keep their tip cluster for life.** The conversion seats a cluster at a short shoot's tip whatever its age. The beech's PA 14 grows once (`continuation` 0, 3 to 5 nodes 6 mm apart) and lives 3 years, so it bears its cluster in each of those years. That is the stated rule. Setting a cluster on PA 14 also stops it bearing leaves along its 6 mm nodes in its first year.
6. **The count of 3 comes from a metamer count.** [LET] gives a short shoot "usually 3 or 4" metamers and uses 3. The beech bears one leaf per node, two-ranked ([M98]), so its leaves per short shoot are 3. That is an inference from a sourced metamer count, and it will be recorded as such. The cluster's spread angle has no source and will be marked estimated.

The scope is large: an engine row, the conversion on both executors, the plan, station and placement counts, the device's stations, and parity tests.
