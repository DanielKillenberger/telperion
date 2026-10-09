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

## Parts 1 and 2 built, part 3 measured (worker, 2026-10-09)

### Part 1: leaves by wood age

- **The row.** `PaState::leaf_years`, neutral at never. It is refused below 0 or as NaN, mixes on the `Kept` scale (halfway from never to 1 is 3), and is written `/space/ages/<k>/leafYears` (`never` is admitted, as on `shedding`).
- **The record.** The conversion writes `Tree::leafing` on both executors. For each node it records the boundary in the node's grown wood where its merged phytomers' retention changes, and the retention before and from that boundary, each a 16-bit share. On the CPU this is `space.rs` `convert`; on the device it is `dress.wgsl`, which reads each phytomer's cycle from the laid record (`PL` 27) and each PA's first shedding age and share from the geometry table (`GG` 42).
  - The record is empty where every age bears for ever.
  - A node that merges three retentions gives its oldest piece the middle one, by the stated rule in `tree/leafing.rs`.
  - Device records grow from 30 to 32 words (`CV`), and sweep nodes from 31 to 33 (`NW`).
- **Where retention is applied.** The plan's descriptors, the count, the CPU placement, the prepared station segments, and the device's `measure` and `emit` all keep a station by its retention, compared in whole micrometres along the run.
  - A fractional retention draws under the registry's new `RETAIN` key, by the station's ordinal along the run.
  - A dropped station still takes its leaf's draws, so every kept leaf is bit for bit the one the tree bore for ever. The test `a_leaf_stays_only_where_its_wood_keeps_it` checks this.
  - A segment some of whose stations shed carries its boundary, its retentions and its stream in the three spare lanes of its frame. Placement (`pack.wgsl`) walks at most 256 stations to find each leaf's ordinal.
  - Clusters bear for as long as their shoot lives.
  - Retention without a twig layer is refused.
- **The beech.** `leafYears` 1 on every age. The short shoots (PA 14) bear a cluster of 3: [LET] gives three metamers per short shoot and one leaf per node, so the 3 is inferred, not quoted. The cluster's spread is 0.6 rad (estimated). Laurelin and Telperion state the same rows. The ordinary tree keeps its own rows; the centroid test that would move them is already `#[ignore]`.

### Part 2: spacing

- **The value.** `/skeleton/twigs/twig/internodeLength` goes from 0.006 to 0.025 on the beech, Laurelin and Telperion. That is one leaf per node of the shoot's 2.5 cm internode (estimated, fn-193 SOURCES.md).
- **How it was chosen.** On close stills of the beech at 80 at 6, 15, 25 and 35 mm, against the two beech-shoot photographs (`TWIGS-spacing.jpg`). At 25 mm the leaves alternate along a visible thin shoot, as they do in the photographs. At 35 mm the sprays thin out.
- **The comment.** The row's stale comment ("a leaf every 5 cm") now states the value.

### Proof

- **Red/green tests:**
  - space crate (`tests/refusals.rs`, `tests/chain.rs`): refusals and the Kept-scale mix;
  - preset rows (`tests/space_presets.rs`);
  - the conversion's record (`pipeline/space/leafing_tests.rs`);
  - placement (`suite/retention.rs`): the kept leaves are a subset of all leaves, bit for bit; a share of a year keeps that share within four standard errors; the plan, placement and segment counts agree;
  - the device (`resident_tests.rs`, `the_real_path_keeps_the_stations_the_cpu_path_keeps`).
- **CPU against device.** On the beech at 30 (seed 1), at 1 year and at 1.5 years, the device places every leaf the CPU places: 7,161 and 8,520 leaves. Every leaf is compared, none is left over, and every one lies within the screen. The neutral parity numbers did not change.
- **Neutral identity.** It was checked against the base (`c6cd30d3`, built in its own worktree and target), on every shipped preset at seeds 1 and 7, with the presets still neutral.
  - The CPU artifacts (`artifact_digest`) match to the bit once the new empty `leafing: []` is stripped.
  - In `generation_digest`, these match on every preset: the pipeline's leaves, the mesh, the GPU-CPU path's instances, and the resident path's count, bounds and curve.
  - Two things differ, and why:
    - `pipeline.tree`: by the `leafing` field alone.
    - The memory metrics: by the wider records.
  - The resident path's instance digest differs from run to run at the base itself (two base runs gave `077d…` and `1d5f…`), so its leaves are compared by the parity test instead.
  - The catalogue digests were re-pinned. With `leaf_years: inf, ` stripped and the three value files at base, each digest matched its old pin.
- **Cost.**
  - Device preparation time (`prepare_timing`, 3 runs each) is unchanged within noise:
    - beech 80: 621-628 ms → 624-647 ms;
    - Laurelin: 676-684 ms → 667-755 ms;
    - Telperion: 864-875 ms → 858-1122 ms;
    - spruce 40 (neutral): 477-739 ms → 461-481 ms.
  - The first head run after the shader change compiled cold: 22.7 s, then 287 ms.
  - Native frame time at 1920×1080 after part 1 (`headless --timing`, p50 total):
    - Telperion: 121 ms → 101 ms (9.05M → 1.42M leaves);
    - Laurelin: 89 ms → 75 ms;
    - beech 40: 4.4 ms → 4.3 ms.
  - Part 2 removes more leaves still.
- **Leaf counts at seed 1 (before → part 1 → part 2):**
  - beech 40: 66,155 → 39,007 → 25,113;
  - beech 80: 2,494,405 → 586,237 → 361,315;
  - Laurelin 80: 6,126,178 → 1,027,891 → 468,203;
  - Telperion 92: 9,052,762 → 1,424,271 → 664,957.

### The stills (viewed)

The sheets are `TWIGS-beech40.jpg`, `TWIGS-spacing.jpg` and `TWIGS-after.jpg`; the full set is under `raw/before`, `raw/after-part1`, `raw/after-part2` and `raw/spacing`.

- **Beech 40.** The crown is thinner, and the twigs show between the leaves. At mid, sprays carry leaves spaced along visible shoots. At close, a single spray carries a fan of leaves.
- **Beech 80 and 92.** At mid and whole, the fine wood is now visible as a bare twig layer inside the crown, with leaves at the shoot ends and spurs. The caterpillar and fern-frond sprays are gone. The crown reads as more open than before; whether it now reads too bare is the owner's call.

### Part 3 measured (no build)

The Two Trees' twigs still read as branches.

- **What the stills show.**
  - Laurelin close: the leaves stand 2.5 cm apart along a shoot as thick as a finger.
  - Laurelin mid, Telperion mid: bare, dark, branch-thick fine wood dominates. Laurelin's leaves are few beside her racemes; on Telperion the flowers carry most of the colour.
- **The numbers** (`raw/twig_layer-after.log`), Laurelin against the beech at 80:
  - Current-year shoots are still 2.5 times as thick and as long: median diameter 6.4-6.6 mm against 2.6 mm, median length 60 mm against 24 mm.
  - The median seat under a leaf is 4.75 mm against 1.90 mm.
  - Blade / seat is 19 against 46; petiole / seat is 1.9 against 4.6.
  - Telperion against the beech at 92 is the same: shoots 5.1-6.1 mm against 2.1-2.6 mm, seat 4.56 mm against 1.90 mm.
  - The example's "leaves per metre" now counts cluster leaves against run wood, so that line no longer gives the spacing.
- **What follows.** Parts 1 and 2 fix the natural tree's foliage. They do not change what the uniform scale does to the finest wood: it still draws a current-year shoot of the beech at 2.4-2.5 times its thickness, beside leaves at their natural size. With most leaves now gone from that wood, the thick bare shoots are what the eye reads. The decided route (Astra's per-PA enlargement) is the host's next call.

### Review, gate and npm test (worker, 2026-10-09)

- **impl-review (codex, three draws): SHIP.** It returned two P2 findings.
  - **#1, fixed in `e1690cf5`.** Placement recomputed the station unit from an f32 internode, while the CPU counts by an f64 one. The device's stations and placement now both take the CPU's whole-micrometre unit.
  - **#2, declined and raised to the host.** A node that merges three ages straddling a fractional year keeps two retentions, not three. Fixing it needs a third record word, against the host's two. No shipped preset can produce such a node: the beech's `leafYears` is a whole number, and the conversion test found 2 mixed nodes of 5,224 at 1.5 years.
- **Gate (`cargo test --profile ci --workspace --no-fail-fast`).** 141 test binaries pass. One test fails: `every_pipeline_within_2_5_s_but_grow_within_20_s_cold`, which timed the `grow` pipeline at 26.8 s against its 20 s bound while running in parallel with the suite. Run alone at this head it passes, with grow at 18.3 s. This change touches no part of `grow` beyond adding one `KEY_RETAIN` constant to `lineage.wgsl`. No base gate run was made, so whether the base fails the same way under contention is unknown (FRICTION.md).
- **npm test.** 14 files and 154 tests pass, after `src/browser/presets.generated.ts` was regenerated for the three value files (`TELPERION_WRITE_REFERENCE=1 npm run wasm:build`).

## Host decision (2026-10-09): part 3, the Two Trees grow large through their own coarse ages

I viewed `TWIGS-after.jpg`. The caterpillar sprays are gone and the twig layer shows. On the Two Trees, the fine wood under the leaves is still as thick as branches (the uniform scale). The beech's crown at 80 now reads more open. That is the owner's verdict to give, and it goes to the owner with these stills.

**Part 3 is Astra's route.** The Two Trees reach their size through their own per-age rows: larger internodes and pipes on the coarse ages (stem, limbs, boughs) and ordinary values on the fine ages (shoots, twigs, short shoots). The one engine then computes attachment and girth. Their uniform `/supernatural/scale` goes to 1. No post-growth deformation.

- **The off-state stays the ordinary old beech.** The coarse ages' enlarged rows sit in the /supernatural group, as overrides at `/supernatural` plus the natural row's path (fn-217's X2 convention). The natural rows stay the beech's.
- **Before building, check whether the group's override mechanism already reaches the engine's per-age `/space` rows.**
  - **If it does,** this is values only.
  - **If it does not,** extending it to `/space` rows is the one capability change. Same convention, applied by degree, and every species is byte-identical with the group off. Report the size of that change before building it.
- **The targets.** The Two Trees' heights and spreads stay at today's values (about 45-50 m), within 10 %. Their current-year shoots and the wood under the leaves come back to the beech's measures (about 2.6 mm and 1.9 mm).
- **Proof:**
  - Close and mid stills of both trees beside the beech and the real shoots, viewed.
  - The `twig_layer` measures.
  - Height and spread.
  - Frame rate at 1080p.
  - Contact error.
  - Astra's agreement on the sheets, as for fn-218 and fn-219.
  - Then the gate, npm test and impl-review.

**The review's declined P2,** three ages merged across a fractional year: accepted as declined. No shipped preset can produce it, and it is recorded in the code's doc comment.

## Before part 3: the override does not reach `/space` rows (worker, 2026-10-09)

**What the code does today.** `Family::applied` (`supernatural.rs`) lays down two kinds of override:

- **Mirrored rows.** `mirrored()` admits only overrides whose natural row is a catalogue row under `/material/` or `/bloom/`.
- **Flowering.** The one per-age engine override is hand-written: `SupernaturalFlowering` (`supernatural/flowering.rs`, 189 lines) declares 15 catalogue rows, `/supernatural/flowering/age0` to `age14`, each with a wire number. `applied()` writes them into `species.states[k].flowering`, and `blend()` walks them by hand.

The `/space` rows are not catalogue rows. They are an indexed grammar of their own (`presets/space.rs`). So no `/supernatural/space/...` override exists, and the group cannot reach `internode` or `form.pipe` today.

**The change it needs, sized on the flowering precedent:**

- **Two more override tables.** `/supernatural/internode/age<k>` and `/supernatural/pipe/age<k>` give 30 catalogue rows across the 15 ages. Each row is `Option<f64>`, blends as `Coupled`, has the dial `Derived`, carries the engine's own bounds, and takes a wire number from 333 on. That is about 380 lines of row declarations in two new files, split to stay under 400 lines each.
- **Wiring, about 40 lines in `supernatural.rs`:**
  - `applied()` writes the set ages into `states[k].internode` and `states[k].form.pipe`;
  - `overrides()` and `validate()` include the new tables;
  - `blend()` walks them as it walks flowering;
  - `catalogue/mount.rs` mounts them.
- **Both executors need nothing new.** The pipeline already grows from the applied species: the CPU through `pipeline/space.rs`, and the device through `GeometryTables::new` on the same species, as the flowering override already does.
- **Pins and generated files:**
  - catalogue identity re-pin (with the group off, every natural species byte-identical);
  - the `suite/sweep.rs` list of rows the walks leave unset;
  - `docs/parameters.md` and `src/browser/parameters.generated.ts`, regenerated;
  - the jev dial table, if it counts rows.
- **Tests.**
  - With the group off, every species is byte-identical.
  - An override replaces only its own age.
  - A walk ramps the override in from the natural row.
  - Out-of-bounds values are refused by name.
  - CPU and device agree on a tree with an override.
- **Then the values.** The coarse ages (2, 4 to 8) get larger internodes and pipes on Laurelin and Telperion, `/supernatural/scale` goes to 1, and the values are tuned to the height, spread and shoot targets on stills.

**Estimate:** about 600 to 700 changed lines including tests, all in the catalogue's existing pattern. Nothing new is added to the WGSL shaders.

**One alternative, not proposed.** A generic `/supernatural/space/ages/<k>/<setting>` group that reuses the `/space` grammar would cover every setting at once. It is not a catalogue row group, though, so the wire, the blend, the dials and the harness would each need a new path. That is larger and departs from fn-217's convention.

Waiting for the host's go-ahead before building.

## Part 3 built: the Two Trees' size from their coarse ages (worker, 2026-10-09)

**The capability (`79304adf`).**
- **The rows.** `/supernatural/internode/age<k>` and `/supernatural/pipe/age<k>`, 30 catalogue rows on the flowering convention, laid over the engine's rows while the group is on.
- **Tests.**
  - With the group off, nothing is applied (`Cow::Borrowed`).
  - An age's override moves only that age.
  - A walk ramps each age's override in from its natural row.
  - Rows off their bounds are refused by name (`supernatural internode`, `supernatural pipe`).
  - The device grows the same tree as the CPU: on the beech at 30 with ages 4, 6 and 7 overridden, the tree is taller than the natural one by more than a fifth, and all 6,939 leaves are compared, each within the screen.
  - With the empty tables stripped from the text and the wire, the family, walk and override catalogue digests were each part 2's to the bit.

**The values.** Tuned with `twig_layer` against today's sizes (`raw/twig_layer-part3.log`).
- The coarse ages are 2 (stem) and 4 to 8 (leader, fork, limbs, boughs, spurs).
- Laurelin: internode ×2.6, pipe ×3.4.
- Telperion: internode ×2.6, pipe ×3.0.
- `/supernatural/scale` goes back to 1.
- The fine ages (9 branch systems, 12 shoots, 14 short shoots) stay the beech's.

| Seed 1 | Height | Spread | Stem radius at 1.3 m | Current-year shoot, median | Wood under a leaf, median |
|---|---|---|---|---|---|
| Laurelin today (scale 2.5) | 44.3 m | 43.1 × 47.6 m | 1.161 m | 60 mm, 6.4 mm thick | 4.75 mm |
| Laurelin, part 3 | 44.9 m (+1 %) | 44.3 × 44.5 m (+3 %, −7 %) | 1.174 m | 24 mm, 2.6-2.9 mm | 1.90 mm |
| Telperion today (scale 2.4) | 46.9 m | 55.4 × 51.9 m | 1.341 m | 58 mm, 5.1-6.1 mm | 4.56 mm |
| Telperion, part 3 | 48.9 m (+4 %) | 50.7 × 51.7 m (−8 %, 0 %) | 1.298 m | 24 mm, 2.6-3.1 mm | 1.90 mm |
| The beech at 80 / 92 | 17.7 / 19.5 m | | 0.494 / 0.594 m | 24 mm, 2.1-2.6 mm | 1.90 mm |

Both targets are met. Height and spread are within 10 % of today's, and the shoots and the wood under the leaves are the beech's.

**Frame time at 1080p** (`headless --timing`, p50 total):
- Telperion: 121 ms at base → 101.5 ms.
- Laurelin: 89 ms at base → 68.9 ms.
- Florets dominate both draws: 8.6M and 10.4M, unchanged.

**The stills, viewed** (`TWIGS-part3.jpg`, `TWIGS-part3-whole.jpg`).
- **Close.** The twigs are now ordinary twigs: thin shoots with leaves on petioles beside the real shoot. The racemes (Laurelin) and the corymbs (Telperion) dominate the view.
- **Whole.** Both crowns now read as open, branchy and nearly leafless, a winter tree hung with flowers. Each tree now carries the natural beech's fine wood and leaves, within 1 % (365,167 against the beech's 361,315 at 80; 526,031 against 520,885 at 92), spread over a crown about 2.5 times as wide. That is about one sixth of the leaf density per area of crown. Under the uniform scale the same branch systems were 2.5 times larger and filled the crown.

**For the host before the remaining proofs.** The contact error, Astra, the gate, npm test and impl-review are held until the values are settled, because each would be spent again on any change. The size targets are met, but the crown no longer reads as a full tree. Neither the scale nor these rows add branch systems, as Astra noted; a giant crown of ordinary twigs needs more of them. Whether to accept the open crown, raise the fine ages' branching, or pursue another route is the host's design call.
