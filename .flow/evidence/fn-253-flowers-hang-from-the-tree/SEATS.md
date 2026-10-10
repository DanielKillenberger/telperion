# fn-253: where the flowers are seated and what holds them

## R1: measured before any code (worker, 2026-10-09)

Measured on branch `fn-253-flowers-hang-from-the-tree` at base `750460d3` (F4 with fn-248 merged).

**The instruments.**
- `crates/telperion-core/examples/bloom_seats.rs`, run as `cargo run --profile ci -p telperion-core --example bloom_seats -- <preset> [age] [seed]`. It builds the tree through `pipeline::build`, regrows the engine structure to recover each node's age (PA), axis and wood age by phytomer key (the conversion keeps neither), and replays `bloom.rs`'s presence draw to say which seats flower. For each drawn floret it measures the distance from the nearest flowering seat's wood surface to the floret's base.
- The device: `preset_florets` in `generation/fine/florets_tests.rs`, an ignored probe that builds a shipped preset on the real path and matches its florets against the CPU whole tree's, under the GPU lock.
- Logs: `raw/bloom_seats-before.log`, `raw/preset_florets-before.log`.

### Where each inflorescence is seated

Both trees flower on one age only, 14 (the beech's short shoots; `/supernatural/flowering/age14`).

| | Laurelin 80, seed 42 | Telperion 82, seed 42 | Laurelin 80, seed 1 | Telperion 82, seed 1 |
|---|---|---|---|---|
| Nodes | 3,331,690 | 3,572,076 | 1,939,618 | 2,075,326 |
| Bloom seats (every drawn node of age 14) | 1,934,353 | 2,072,892 | 1,130,053 | 1,208,178 |
| Short-shoot tips (fn-223 clusters) | 196,853 | 209,747 | 114,488 | 122,312 |
| Seats that flower (share 0.13 / 0.09) | 279,853 | 208,167 | 163,472 | 121,494 |
| of those at their axis's tip | 28,624 (10 %) | 21,306 (10 %) | 16,669 (10 %) | 12,273 (10 %) |
| Short shoots bearing more than one | 79,278 of 141,129 | 55,503 of 125,682 | 46,371 of 82,410 | 32,565 of 73,184 |
| Grown wood between seat and tip, mm p10 / median / p90 | 0 / 19.8 / 66 | 0 / 19.8 / 66 | 0 / 18.0 / 66 | 0 / 18.1 / 66 |
| Seat wood diameter, mm p10 / median / p90 | 1.2 / 3.8 / 6.6 | 1.2 / 3.8 / 6.6 | 1.1 / 3.3 / 6.6 | 1.1 / 3.3 / 6.6 |
| Florets drawn | 1,677,467 | 1,039,409 | 979,974 | 606,710 |

Seats by wood age (years since the phytomer under them grew), Laurelin seed 42: 0: 85,287; 1: 83,602; 2: 58,476; 3: 35,865; 4: 16,623. Telperion's shares are the same to the percent. A short shoot lives four years and adds a growth unit each year, so its wood is 0 to 4 years old and up to about 66 mm long; nine in ten flowering seats stand along it, not at its tip, and more than half the bearing short shoots carry two or more inflorescences at different points along it.

### What connects a floret to the wood

Nothing that is drawn.
- **The rows.** Laurelin's rachis is 0.72 m with a 24 mm pedicel, six florets, hanging (droop 1). Telperion's is 0.39 m with a 45 mm pedicel, five florets, standing (droop -1).
- **The gap,** from the seat's wood surface to the floret's base, mm p10 / median / p90: Laurelin 50 / 167 / 391 (seed 42) and 50 / 170 / 394 (seed 1); Telperion 49 / 118 / 245 and 48 / 116 / 245. Every floret has its seat within one reach (rachis plus pedicel).
- **Checked against the code.** `bloom.rs` `inflorescence` lays the axis (`Axis::laid`, eight segments from the node along the bud's side, turned toward hanging or standing by the droop), places floret `i` at `t = (i + 0.5) / florets` along it, and pushes one transform per floret at `at + stalk * pedicel`. The organ element (`bloom.rs` `element`) is the petals and the stamens alone. The axis and stalks are used for placement and never emitted, as fn-217's review said (ASTRA-REVIEW-DESIGN.md §5).

### The seating rule, checked against the code, on both executors

- **CPU.** `pipeline/space.rs` `converted` pushes a `Bloom` for every node it draws on an axis whose age's flowering share is above zero (`share > 0.0 && d.relay.is_none()`), not only the tip. The presence draw (`bloom.rs` `presence`, keyed by the node's lineage under `BLOOM`) then keeps the share of them that flowers.
- **Device.** `dress.wgsl` `place` writes the age's flowering share into every own-drawn record (word 28, the sweep node's word 29), and `blooms.wgsl` `grown_bloom` reads it on every fine node. The tip of a short shoot is marked (`TIP`) only for its cluster; blooms do not read the mark.
- **They agree.** The probe matched every floret: Laurelin at 30 (seed 1) 16,630 on both sides, Telperion at 30 9,548, and Laurelin at 80 (seed 42) 1,677,419 device florets plus 48 in the CPU block against the whole tree's 1,677,467. Every floret is exact or within fn-229's screen, except 6, 5 and 127 per side that the probe's "turned" class misses because its screen is the catkin fixture's 3 mm stalk, not these trees' 24 to 45 mm (largest position difference among the matched: 1.4 mm).

### What it means

The owner's "placed randomly and attached to the air" is both causes on record, measured:
1. **Randomly:** nine in ten inflorescences stand somewhere along a short shoot's four years of wood, often two or three to a shoot, not at its tip.
2. **In the air:** every floret hangs 5 to 39 cm from its wood (median 17 cm on Laurelin, 12 cm on Telperion) with nothing drawn between them.

**One consequence for R2's values.** Tip seating caps the inflorescences at one per short shoot: 114,488 tips on Laurelin at seed 1 against today's 163,472 flowering seats, so even a share of 1 seats 70 % of today's inflorescences. Holding the florets' coverage (fn-248 held it within 1 %) then takes the share up near 1, and possibly more florets per inflorescence; that is measured on the organ mask after R2 lands.

## R2: seated at the tips (worker, 2026-10-10)

**The rule (`80f3ac29`).** An axis of an age that flowers bears one inflorescence at its tip: its last node drawn from its own wood, the node fn-223 seats a cluster on. The age's share draws which tips flower, by the same presence draw, keyed by the tip's lineage. Nothing new is a row.
- **CPU.** `pipeline/space.rs` `converted` keeps the tip and its bud's side while it draws, and pushes the bloom after the axis's last phytomer, beside its cluster. A tip drawn at a relay's join bears none, as before no relay node did.
- **Device.** `dress.wgsl` `place` writes no share on the nodes along the axis; `draw` writes the age's share on the tip when the tip came from the axis's own wood.
- **Neutral.** Only Laurelin and Telperion flower among the shipped presets, and only with the group on, so every other tree, and both with the group off, draws no bloom before or after.

**Tests.**
- `space::bloom_tests::a_flowering_axis_bears_its_bloom_at_its_own_tip` (beech at 15, seed 7, shoots at 0.4 and short shoots at 0.7): one bloom per flowering axis, at its tip, at its age's share and with its last phytomer's side. Red on the base rule (4,958 blooms against 500 tips), green after.
- The device against the CPU (`florets_tests`, under the GPU lock): the beech and the spruce at 30 flowering on every age match with none unmatched (55,160 and 2,284,952 florets); Laurelin at 30 matches all 1,836. The cluster parity test passes.

**The values (`09835ae0`).** `/supernatural/flowering/age14` goes to 1 on both trees: every short shoot's tip flowers.
- **Why 1.** Tip seating caps the inflorescences at one per short shoot. At seed 1 that is 114,488 on Laurelin against the 163,472 fn-248's share seated along the shoots, and 122,312 on Telperion against 121,494. A share of 1 holds the count as near the owner-approved build as the tips allow.
- **The organ-mask coverage cannot choose it.** With the preset shares the florets fell from 979,974 to 99,916 on Laurelin, and the mask's coverage moved from 0.320 to 0.333 (whole) and 0.150 to 0.166 (mid); at a share of 1 it reads 0.335 and 0.172. It does not follow the floret count (fn-248 saw the same), so it is reported, not used (`raw/coverage-r2.log`).
- **The stills (viewed: `raw/laurelin-mid-share.jpg`, `raw/telperion-mid-share.jpg`, beside fn-248's `raw/florets/*-1.jpg` in its own worktree).** At the old shares the flowers almost vanish at the mid view. At 1, Laurelin's racemes read about as dense as fn-248's mid view. Telperion's white clusters read heavier than fn-248's, because they now stand at the crown's surface rather than along the spurs inside it; that is the owner's call (open item 4).
- After: Laurelin 686,928 florets (seed 1), Telperion 611,560.

## R3: the design, stated before code (worker, 2026-10-10): stopped for the host

### What the code fixes

- **A floret's base is its element's origin.** `Element` stands "attachment at origin, axis +Y"; the petals start on a connector from the origin. `bloom.rs` `matrix` and `bloom.wgsl` `floret` put the floret's translation at `at + stalk * pedicel` and its +Y axis along the stalk. Every size the floret takes (growth, presence, season, taper, scatter) scales the matrix about that origin. fn-246's thinning enlargement divides the element position by `sqrt(kept)` in `foliage.wgsl`, also about the origin. So the floret's base already stays put under every transform the invariant lists. The stalk's two ends are therefore known after them: the floret end is the instance's translation; the seat end is `translation - axis * pedicel`, which is `at` on the rachis.
- **One draw is one element.** The organ is a `Foliage` (`lib.rs`): one element, one instance list, one indexed draw per level, the level chosen per instance by `select.wgsl`. Every instance draws the whole element's level.
- **An instance is three words.** It holds the rotation, the position over the box, and the scale as a half float whose sign bit marks a withered leaf (`leaf.wgsl`). No organ withers, so that bit is free on the organ.
- **Thinning ranks each instance by a hash of its own three words** (`leaf_draw_rank`). Nothing ties one instance's draw to another's.

### The representation proposed

**The pedicel is the floret's own; the rachis is one instance of its own per inflorescence. Both kinds share the organ element and its one draw.**

1. **The pedicel, in the floret's instance.** The organ element gains a thin stalk: a prism of unit length along -Y below the origin, its vertices marked. The organ's vertex stage places a stalk vertex at `base + R * (x * w, y * pedicel, z * w)`, where R is the instance's rotation without its scale, and `pedicel` and `w` are the tree's own values in metres. The stalk's length and width are therefore never scaled with the floret or enlarged by thinning. Its floret end is the floret's base after every transform; its seat end is `at` on the rachis. It is ranked and leveled with its floret, so a drawn floret always has its pedicel.
2. **The rachis, one instance per inflorescence.** The instance stands at the seat node with +Y along the bud's side, carrying the organ's free sign bit. Its vertex stage lays the axis per vertex as `Axis::laid` does (eight segments, turned by the droop toward hanging or standing, `rachis / 8` a step), so the florets' seats lie on it.
   - Emitted only where the inflorescence keeps a floret. Dormant where the rachis is 0: none is emitted, and the pedicels stand on the node itself.
   - **Never thinned.** Thinning ranks each instance by its own words, so a thinned rachis would strand kept florets. Unthinned, every drawn floret reaches the wood: floret, its own pedicel, the rachis, the node.
   - **Count:** one per flowering inflorescence (Laurelin 114,488 and Telperion 122,312 at seed 1), against 687k and 612k florets.
3. **Both executors.** The CPU (`bloom.rs`) and the device (`blooms.wgsl` and `place.wgsl`) each append the rachis instances after the florets, from the same records they place florets from. The vertex stage is shared, so the stalks are drawn identically.
4. **The cost.** One element means a rachis instance runs the petals' vertices collapsed, and a floret instance runs the rachis's. Frame time is measured against fn-248's (whole / close at 1080p: Laurelin 174.8 / 77.5, Telperion 187.8 / 80.6 fps).

### Representations considered and set aside

- **Every stalk an instance of its own** (a pedicel per floret, eight straight rachis segments per inflorescence): no curve in the shader, but about 1.6M more instances on Laurelin. The pedicels would be thinned apart from their florets unless they too are exempted.
- **The floret carrying its whole path from the node** (the instance at the node, its index in the words): each floret is connected on its own, and thinning could act per inflorescence. But three words leave no room for the index (up to six bits). Selection, the shadow pass and the own-mass grid would also all read the node as the floret's position.
- **A second element and draw for the stalks:** it leaves the spec's "one draw".

### The host's calls before R3 is built

1. **The representation above**, including the rachis left out of thinning. That departs from fn-246's one thinning rule for leaves and flowers.
2. **The stalk's width and colour: no row exists for either.**
   - **New bloom rows** (`/bloom/stalkWidth` and a stalk colour): the catalogue's whole path, from wire numbers and the supernatural mirror to the docs and generated files.
   - **Derived from rows that exist:** for example, the width from `stamenWidth` and the colour from the shoot's material colour or the bloom's back colour.
3. **Which way a tip's raceme leaves.** It now stands at the shoot's tip but still leaves along the bud's side, as an axillary one did, then turns by its droop. A terminal raceme might instead continue the shoot's heading. The choice changes both executors' placement.
4. **Telperion's density at a share of 1** (R2 above) is the owner's look to judge. Laurelin's reads as before.

Not run yet: R4's stills, test, timing, Astra, the gate, npm test and impl-review. They wait on R3.

## Host decisions on R3 (2026-10-10)

1. **The representation is accepted.**
   - **The pedicel** is part of the floret organ, at a fixed width and length in metres. It never scales or enlarges with the floret. Its ends are the rachis point and the floret's base after every transform the floret gets.
   - **The rachis** is one instance per inflorescence, laid along the curved axis the florets already sit on.
   - **The draw.** Both use the existing flower draw.
   - **Thinning.** The rachis is exempt from thinning, a scoped exception to fn-246's rule that is the stalk invariant's consequence: a thinned rachis would leave kept florets in the air. Its instance count and measured frame cost are recorded. If the cost is more than about 2 % of frame time at the whole, mid or close view, the build stops and reports.
2. **No new rows** (one parameter set; delete before adding).
   - The rachis width is the seat's twig radius at the shoot tip, tapered to the stamen width at its far end.
   - The pedicel width is the stamen width.
   - The colour is the shoot's own wood colour.
3. **A terminal raceme continues the shoot's direction at its tip,** and the existing droop bends it, so Laurelin's racemes still hang. Lateral seating no longer exists after R2, so the departure along the bud's side is deleted.
4. **Telperion's density:** the host puts the stills to the owner; the share stays at 1.

## R3 built; R4 stopped on the rachis's cost (worker, 2026-10-10)

### What is built

- **The element** (`bloom/stalk.rs`). The organ element gains a pedicel tube (two rings) and a rachis tube (nine rings, one at each of the axis's points), each three-sided. Their vertices stand within a micrometre of the origin, so no bound, sphere or deviation of the floret reads them; the surface coordinate names each one (x is -1 for the pedicel, -2 for the rachis; y is its ring times three plus its side).
- **The levels.** The pedicel is in every floret level, the card included (`card.rs`). The rachis is a level of its own in front of the floret's, at a deviation of 1e30, so no floret is ever drawn at it.
- **The rachis instance.** One per inflorescence that keeps a floret and has a rachis above zero, on both executors (`bloom.rs` `inflorescence`; `place.wgsl` `rachis_words` after the device's florets). It stands at the tip along the shoot, with the scale's sign bit set (the organ has no withered flower) and the scale holding the seat's radius.
- **The vertex stage** (`foliage.wgsl`). It lays the pedicel from the floret's base, back a pedicel's length along the floret's own axis to its seat on the rachis. It lays the rachis from the instance by `Axis::laid`'s rule: the shoot's heading turned toward hanging or standing by the droop, in the plane it turns in, tapering from the seat's radius to half the stamen width. A floret's rachis vertices, and a rachis instance's other vertices, fold to a point outside the frame. A stalk is lit in the shoot's colour, with nothing passed through or mirrored.
- **Selection** (`select.wgsl`). A rachis is culled by its length from its seat, never thinned, and placed in the rachis level.
- **Direction.** The axis leaves the tip along the shoot's heading on both executors; `Bloom.side` is deleted.
- **The rows read, none added:** `/bloom/rachis`, `/bloom/droop`, `/bloom/pedicel` and `/bloom/stamenWidth` (the stalks' half width at the far end, and the pedicel's), through `/supernatural/bloom/*` while the group is on; the seat node's drawn radius; `/material/shootRed`, `shootGreen` and `shootBlue` for the colour (the frame block's `shoot`). They reach the renderer through `Renderer::set_family`.

### The invariant, tested

- `stalk_tests::every_drawn_floret_hangs_from_wood_through_its_stalk` decodes each executor's organ words (Laurelin at 30, seed 1, a stagger of 1, so 11,701 of 12,294 florets are drawn below full size). Every floret's pedicel seat stands within the rachis tube or at most 0.32 mm outside it, against two position codes (0.79 mm); every rachis stands within 0.20 mm of a flowering tip. The device and the CPU agree: 12,294 florets and 2,072 rachises on each side. With the rachis at zero, no rachis is drawn and every seat is inside its tip's wood.
- `select::tests::a_rachis_is_culled_but_never_thinned`: at a size where thinning drops every floret, every rachis in the frame is kept, and one behind the eye is culled.
- `card::tests::the_card_level_keeps_the_stalks`; the core tests `a_rachis_stands_at_each_flowering_tip`, `a_raceme_continues_its_shoot_from_the_tip` and `the_stalks_stand_at_the_origin_in_every_level`.
- The device floret parity tests pass, with the rachises among the instances matched (beech at 30: 57,130).

### Two defects found by looking

1. **The card hid the rachis.** fn-246's card level carried the card's quad alone, and selection puts a rachis in the coarsest level, so no rachis was drawn. The words-level test cannot see the renderer's levels. A full-resolution crop showed pedicels ending in the air. Fixed: the card keeps the pedicel, and the rachis has its own level.
2. **The stills example's coverage measured the picture.** `COVERAGE` drew the organ mask in the whole view, where the mask does nothing. It now draws it in the clay room. The real whole-view coverage is 0.061 (Laurelin) and 0.061 (Telperion), against the 0.32 the broken measure reported. fn-248's "coverage held within 1 %" and R2's coverage figures above were taken on the broken measure (FRICTION.md).

### The rachis's cost: past the host's bound, stopped

`frames.rs`, 1080p, best of 3, GPU total p50 in ms. "No rachis" is the same tree with `/supernatural/bloom/rachis` at 0 (`frames <tree> no-rachis`): the florets sit on their tips, and no rachis instance is drawn. Raw: `raw/frames-r3d.log`.

| | whole | mid | close |
|---|---|---|---|
| Laurelin, with rachis | 6.86 (141.6 fps) | 14.15 (69.6) | 13.21 (74.4) |
| Laurelin, no rachis | 5.89 (164.7) | 13.26 (74.3) | 12.68 (77.6) |
| **Rachis cost** | **+0.97 (16.5 %)** | **+0.88 (6.7 %)** | **+0.52 (4.1 %)** |
| Telperion, with rachis | 8.15 (119.9) | 15.85 (62.3) | 13.94 (70.3) |
| Telperion, no rachis | 7.28 (133.9) | 15.59 (63.3) | 13.84 (71.2) |
| **Rachis cost** | **+0.87 (11.9 %)** | **+0.27 (1.7 %)** | **+0.10 (0.7 %)** |

- **Instances.** 114,488 rachises on Laurelin and 122,312 on Telperion (seed 1), each 48 triangles over 27 vertices, never thinned. At the whole view most are thinner than a pixel and still rasterised.
- **Against fn-248** (whole / close): Laurelin 174.8 / 77.5 → 141.6 / 74.4; Telperion 187.8 / 80.6 → 119.9 / 70.3. Telperion's drop is mostly R2's share of 1 (at a share of 0.1, with the stalks before the rachis level: 195 / 98 / 90 fps whole / mid / close, `raw/frames-r3-share01.log`). Every view stays above fn-246's 60 fps; Telperion's mid view sits at 62.
- **The pedicel at the card level** also costs: with no rachis, Laurelin's whole view is 5.89 ms against 5.49 ms before the card carried it (+7 %).

**The host's call.** Candidates the measurement suggests, none built:
- a rachis level ladder by projected size, still never thinned: a two-sided ribbon of two to four segments under a few pixels across, in place of 48 triangles;
- thinning the rachis and its florets together, by a rank all of an inflorescence shares;
- accepting the cost.

### Astra, round 1 (on stills taken before the card fix; `raw/astra/stalks-r1.md`)

DISAGREE.
- Mid arrangement: PASS ("repeated hanging racemes … sprays at shoot ends").
- Close attachment: FAIL ("cannot consistently trace individual florets through a visible pedicel and rachis"). Those stills were taken while the card hid every rachis.
- Telperion's white "now reads too heavy compared with before".
- The trees: PASS.

The raceme view (`raw/r3d/*-raceme.png`) now shows each rachis and its pedicels. Astra has not seen it; a second round waits on the cost decision.

### The stills (viewed)

`STALKS-laurelin.jpg` and `STALKS-telperion.jpg`, at the whole, mid, close and raceme views (1920×1080 sources in `raw/r3d/`).
- **Laurelin.** Her racemes hang as golden chains from the shoot tips; at the raceme view each horn hangs on its pedicel from a thin dark rachis that leaves the twig.
- **Telperion.** His candles stand upright from the tips on a thin rachis, five florets on each; at the mid and whole views the white is heavy.

## Host decision on the cost, and what it measured (2026-10-10)

**The decision.**
- Each inflorescence is thinned as one unit under fn-246's single rule: the rachis and its florets are kept or dropped together, by the whole inflorescence's projected size, and a kept unit is enlarged as fn-246 enlarges any kept organ. This replaces the "rachis never thinned" exception.
- At the card level the pedicel is dropped.
- The rachis keeps levels by its projected width.
- Target: within about 2 % of "no rachis" at every view; otherwise report and stop.
- Telperion's share stays 1; the rachis colour stays the shoot's.

**Built.**
- **One run per unit.** An inflorescence's florets then its rachis stand in one run on both executors. The CPU already places them so. The device now counts one more instance per bloom and places the rachis last in its record's run; the separate rachis block is deleted.
- **Finding the unit** (`shaders/unit.wgsl`). Selection and the organ's draw find a floret's rachis as the first marked instance after it (at most 65).
- **Selection** (`select.wgsl`). The unit's size is the rachis plus the pedicel plus the floret's radius, from the rachis's seat. Its rank is the rachis's, and every member takes the unit's kept share. A rachis takes its level by its width on screen: a ribbon facing the eye over two segments under 1 px, over four under 3 px, and the tube above.
- **The draw** (`foliage.wgsl`, `stalk.wgsl`). A kept unit is enlarged about its seat as a whole, so stalks and florets stay joined. Leaves keep their exact arithmetic.
- **The card** (`card.rs`). It is its quad alone again, behind the rachis's three levels.
- **Tests.**
  - `select::tests::an_inflorescence_is_kept_or_dropped_whole`: 1,000 units of three florets and a rachis, at a size that keeps a third of them; no unit splits, and between 200 and 800 are kept.
  - `stalk_tests` now checks each floret against its own run's rachis, as the shaders find it. Same numbers as before: 12,294 florets and 2,072 rachises on both executors, at most 0.32 mm off the tube.
  - `card::tests::the_card_follows_the_rachis_levels_and_drops_the_pedicel`; the core's `the_stalks_stand_at_the_origin_in_their_levels`.

**Measured** (`frames.rs`, 1080p, best of 3, GPU total p50 in ms; `raw/frames-r3e.log`):

| | whole | mid | close |
|---|---|---|---|
| Laurelin, with rachis | 5.571 (173.4 fps) | 13.487 (72.9) | 12.830 (76.7) |
| Laurelin, no rachis | 5.413 (178.0) | 13.380 (73.7) | 12.785 (77.1) |
| **Δ** | **+0.158 (+2.9 %)** | +0.107 (+0.8 %) | +0.045 (+0.4 %) |
| Telperion, with rachis | 6.971 (139.7) | 14.841 (66.4) | 13.387 (73.4) |
| Telperion, no rachis | 6.696 (145.6) | 15.361 (64.3) | 13.719 (71.9) |
| **Δ** | **+0.275 (+4.1 %)** | −0.520 (−3.4 %) | −0.332 (−2.4 %) |
| fn-248 (whole / close) | Laurelin 174.8 / 77.5 | | Telperion 187.8 / 80.6 |

- **Against "no rachis".** The mid and close views are within the target. The whole views are not: +2.9 % and +4.1 %.
- **How exact the comparison is.** The "no rachis" tree is not the same picture: its florets sit on their tips and thin one by one. That is why Telperion's mid and close views come out faster with the rachis (unit thinning drops more florets there). The comparison carries about ±3 % of its own.
- **Against fn-248.** Laurelin is back to fn-248's rate (173.4 / 76.7 against 174.8 / 77.5). Telperion's whole view is at 139.7 against 187.8, which is the share of 1 (measured above: 195 fps at a share of 0.1).

**An experiment, reverted: no rachis under half a pixel wide.** Read as "it draws only where it covers a pixel", selection would give a rachis under 0.5 px wide nothing to draw. The whole views then measure 5.425 ms on Laurelin (+0.2 % over no rachis) and 6.779 ms on Telperion (+1.2 %), inside the target (`raw/frames-r3e-subpixel.log`). But a floret of a kept unit can then be drawn while its rachis is not. At that distance the floret is a few pixels and its rachis under half of one, yet that breaks the stalk invariant as R3 states it. That is the host's call, so it is not built.

Stopped here, per the decision. Astra round 2, the gate, npm test, impl-review and `done` wait on it.

## Host decision: the cost accepted (2026-10-10)

The host accepts the rachis's cost at the whole view: +2.9 % on Laurelin and +4.1 % on Telperion against "no rachis" (table above; `raw/frames-r3e.log`).
- **The bound.** About 2 % was the host's bound, not the owner's, and the comparison carries about ±3 % of its own.
- **Against fn-248.** Laurelin is back level with fn-248: 173.4 / 76.7 fps whole / close, against 174.8 / 77.5.
- **The invariant** stays exactly as written. No sub-pixel rachis cutoff is built.
- **Telperion's share of 1** stays pending with the owner.

| GPU ms (fps) | whole | mid | close |
|---|---|---|---|
| Laurelin | 5.571 (173.4), +2.9 % | 13.487 (72.9), +0.8 % | 12.830 (76.7), +0.4 % |
| Telperion | 6.971 (139.7), +4.1 % | 14.841 (66.4), −3.4 % | 13.387 (73.4), −2.4 % |

## R4: the sheets and Astra round 2 (worker, 2026-10-10)

**The owner's sheets, on the unit-thinned build (viewed).** `STALKS-laurelin.jpg` and `STALKS-telperion.jpg`, at the whole, mid, close and raceme views (seed 1, 1920×1080 sources in `raw/r3e/`).
- **Laurelin.** Her racemes hang from the shoot tips as golden chains; at the raceme view each horn hangs on its pedicel from a thin rachis that leaves the twig.
- **Telperion.** His candles stand on their rachis at the tips. His white is heavy at the whole and mid views, which is pending with the owner.

**Astra round 2** (`raw/astra/stalks-r2.md`; prompt `stalks-r2-prompt.txt`; sheets `laurelin-r2.jpg` and `telperion-r2.jpg`: mid view, raceme view and its 1:1 crop). **AGREE.**
- Florets traced through pedicel and rachis to the twig: PASS (Telperion's route partly behind leaves).
- Racemes and candles at the shoot ends: PASS.
- Plausible stalks, not extra branches: PASS ("Laurelin's longest exposed sections look slightly ruler-straight").
- Convincing trees: PASS.
- Optional polish: a little more continuous curvature in Laurelin's long rachises; a Telperion view with one whole twig–rachis–pedicel junction clear of leaves.

## impl-review round 1 (codex, three reviewers): NEEDS_WORK, fixed (worker, 2026-10-10)

1. **P1: the coarse rachis ribbons left pedicels off the drawn rachis.** All three reviewers found it. The ribbons over two and four segments skip axis points, while every pedicel is seated on the eight-segment axis: 36 to 113 mm apart on a Laurelin-sized rachis. The connectivity test checked the axis, not the level drawn.
   - **Fix.** The rachis has four levels: ribbons over two, four and eight segments, then the tube. Selection gives a rachis under three pixels wide the coarsest ribbon whose chords stand under half a pixel off its axis (`unit.wgsl` `rachis_error`), the rule every level is chosen by.
   - **Test.** `select::tests::a_thin_rachis_takes_the_coarsest_ribbon_true_to_its_axis`.
   - **Cost.** An eight-segment ribbon for every thin rachis cost +8.4 % and +6.7 % at the whole view (`raw/frames-r3f.log`). The error rule is used instead.
2. **P2: forced levels crossed the two kinds' ladders.** Forcing the finest level hid the rachises, and forcing level 0 hid the florets. A forced level is now held within each kind's ladder. Test: `select::tests::a_forced_level_stays_within_each_kind_s_ladder`.
3. **P2: culling ignored the unit's enlargement.** An organ unit is now culled whole, by its sphere once enlarged.

**Frame time after the fixes** (GPU p50 ms, best of 3; `raw/frames-r3g.log`):

| | whole | mid | close |
|---|---|---|---|
| Laurelin | 5.522 (171.0 fps) | 13.456 (72.6) | 13.094 (74.8) |
| Laurelin, no rachis | 5.242 (184.4) | 13.457 (73.2) | 12.970 (76.0) |
| Telperion | 7.023 (136.2) | 15.897 (61.6) | 14.099 (69.8) |
| Telperion, no rachis | 6.676 (145.7) | 15.653 (63.1) | 13.872 (70.9) |

- **Against the accepted build.** The rachis build is where the host accepted it: Laurelin's whole view is 5.522 ms against 5.571, Telperion's 7.023 against 6.971.
- **Against "no rachis".** The comparison reads +5.3 % and +5.2 % at the whole view. Its reference moved between runs (Laurelin 5.413 → 5.242 ms), within the ±3 % noted above.

## The gate, npm test and review (worker, 2026-10-10)

- **impl-review (codex).** Round 1, three reviewers: NEEDS_WORK (above, fixed in `d4388608`). Round 2, one reviewer: **SHIP**, all prior findings fixed.
- **npm test:** 14 files and 155 tests, green.
- **Gate, package by package** (`cargo test --profile ci --no-fail-fast`).
  - Green: `telperion-core`, `-space`, `-field`, `-wasm` and `-jev`.
  - `telperion-render`: green but for two tests, and the cold-compile test passed.
- **Fixed on the way.**
  - `material_shaders` now builds the organ's shader as `foliage.rs` does.
  - `bloom_form`: the stamen width is no longer dormant at no stamens, by the host's decision that the pedicel takes it.
  - The catalogue identity re-pin (the share): at the old shares all four digests were the last pins.
  - The petal walk counts one petal beside the stalks.
  - The generation-limits inventory classifies the stalk tube's loop.

**Two render tests red, both caused by R2's tip seating, before any stalk code.** Checked by building the R2 commit `80f3ac29` in its own worktree and target:

| Test | Bound | At R2 (`80f3ac29`) | Now |
|---|---|---|---|
| `thinning::far_florets_drawn_as_cards_cover_what_the_florets_cover` (fn-246's card) | far within 30 % of near | 0.00669 against 0.00486 (+38 %) | 0.00661 against 0.00476 (+39 %) |
| `blossom_parity::the_flowering_tree_draws_alike_on_both_executors` (fn-234) | coverage within 1 % | CPU 0.01825, device 0.01844 (1.05 %) | CPU 0.01913, device 0.01963 (2.6 %) |

- **The fixtures.** Both are beeches flowering on their outer ages with no rachis (rachis 0), so no stalk instance and no unit thinning act on them. Tip seating left them with about a tenth of their florets, at the tips.
- **The card test.** fn-246's card, accepted on the owner's look at +25 % on along-shoot florets, now overstates coverage by 38 to 39 % on florets at the tips.
- **The parity test.** The executors drew 1.05 % apart at R2, and 2.6 % with the stalks. The pedicels drawn on both (0.4 mm wide in this fixture) widened it; the cause of the difference was not found.
- **The host's call.** Both bounds were set on the old seating:
  - re-fixture the tests for tip-seated flowers;
  - set the bounds again on the new seating;
  - or treat the card's larger error as a finding.

`flowctl done` waits on that call.

## The two reds after the host's decisions (worker, 2026-10-10)

### Parity: the cause found and fixed (`513c6175`)

The parity test's own cherry fixture, measured CPU against device:
- **The pedicels drawn invisible** (stamen width 1e-5): still 2.7 % apart, so drawing the pedicels is not the cause.
- **The pedicel's length at zero:** 0.9 % apart, so it follows where the florets stand.
- **The florets themselves:** 60,437 of 60,438 match exactly or within fn-125's screen, one unmatched on each side.
- **The CPU florets packed against the device's box:** 0.36 % apart. That is the cause.

**The cause.** The CPU mesh path packed florets against their own box; the device packs them against the leaves' box padded by a floret's reach. Thinning ranks an organ by a hash of its packed words, so the two paths kept different random subsets. With tip seating's tenth of the florets, the sampling noise rose past the 1 % bound.

**The fix.** One packing box on both executors, `foliage::bloom_box` (the leaves' box padded by the reach), used by `mesh::build`, `Expansion::organ` and the device's `Organ::of`. The test passes: CPU 0.01969 against device 0.01963 (0.3 %). Core is green on it.

### The per-inflorescence card: built, and it misses the bound (branch `fn-253-unit-card-trial`, `9139e9b3`)

**As decided.**
- One card per inflorescence once its florets fall under 4 px across. The anchor (the rachis instance, now standing even where the rachis has no length) draws it, and the florets draw nothing.
- **For a raceme:** a camera-facing quad on the chord from its first floret's seat to its last, a floret's radius past each end, a floret's diameter wide (the floret's bounding sphere).
- **For a rachis-less cluster:** a disc of radius pedicel plus the floret's radius.
- No pedicel at the card level, no empirical factor and no new row.

**Measured.**

| | far, at 1× | at 4× | far over 4× | bound |
|---|---|---|---|---|
| fn-246's fixture (`thinning`, rachis-less clusters: the disc) | 0.02759 | 0.01196 | +131 % | 30 % |
| same, per-floret card (now) | 0.00661 | 0.00476 | +39 % | 30 % |
| Laurelin whole, 960×720 against 3840×2880 (the quad) | 0.1310 | 0.0597 | +119 % | |
| Telperion whole | 0.1273 | 0.0682 | +87 % | |

- **The mid views** agree within 4 to 7 % at both sizes.
- **The 4× renders** still draw many florets under 4 px as cards, so they bound the true coverage from above.
- **Why it overstates.** A full quad or disc paints the whole span its florets stand in, where the florets cover a part of it. Laurelin's horns are far narrower than their bounding sphere, and Telperion's florets do not fill their candle.
- **The stills** (`raw/card-whole-sheet.jpg`, viewed; top row with the card, bottom row without). Laurelin's crown turns a pinkish grey (the card takes the petal's front colour at mid-petal and none of the gold the horns show). Telperion's white turns to large blobs.
- **Frame time with the card** (GPU p50, best of 3; `raw/frames-card.log`):

| | whole | mid | close |
|---|---|---|---|
| Laurelin | 4.93 ms (was 5.52) | 14.66 (13.46) | 13.14 (13.09) |
| Telperion | 8.39 ms (was 7.02) | 16.00 (15.90) | 13.98 (14.10) |

Per the decision, stopped with the numbers. The trial is kept on its own branch; the task branch stays at the parity fix, with fn-246's per-floret card (+39 %).

## Host decision: fn-246's floret card stays, its bound restated (2026-10-10)

**The decision.**
- The unit card is rejected on its measurement: +87 to +131 % against the 4× render. Branch `fn-253-unit-card-trial` stays unmerged as evidence.
- fn-246's per-floret card stays. The owner accepted it by eye against the 4× reference ("almost no difference from reference").
- Its error moved from +25 % to +39 % only because tip seating clusters the florets, so they overlap.

**`tests/thinning.rs`'s bound** goes from 30 % to 45 %:
- the card measures +38 % on the R2 commit and +39 % now, on tip-seated florets;
- the bound is that plus about 5 points, as fn-246 set it;
- the owner's eye against the 4× reference is the real gate. The comment states each of these.

**The pairs for the owner** (viewed): `STALKS-laurelin-whole-reference.jpg` and `STALKS-telperion-whole-reference.jpg`.
- **What each holds:** the whole view at 1920×1080 as the harness draws it, beside the same frame drawn at 4× and box-reduced in linear light (`examples/crown_truth`, raw in `raw/truth/`).
- **What they show:** Laurelin's gold reads denser and brighter in the live frame than in the reference; Telperion's white reads close, the live frame a little whiter.

## The render gate on the restated bound (worker, 2026-10-10)

`npm test` is green (14 files, 155 tests), and every render test passes but one: the card test reads **0.00699 against 0.00477, +46.5 %**, against the restated 45 % (`raw/gate-render-3.log`).

| Build | far | 4× | over |
|---|---|---|---|
| R2 (`80f3ac29`) | 0.00669 | 0.00486 | +38 % |
| Before the packing-box fix | 0.00661 | 0.00476 | +39 % |
| After it (`513c6175`) | 0.00699 | 0.00477 | +46.5 % |

**The reason.** The packing-box fix changed nothing but the box the CPU path packs florets against. That changes each floret's words, and so the random subset thinning keeps (the same mechanism as the parity drift). With tip seating's fewer florets, the card test's figure moves by about 7 points on that alone. The bound sits inside the measurement's own noise.

Stopped for the host's call on the bound. The codex re-review and `flowctl done` wait on it.

## The card test measured above its noise (worker, 2026-10-10, under the host's rule)

The host's rule: enlarge the fixture until the figure's spread over three packing variants is under 3 points, then set the bound at the measured value plus 10, with the test under 60 s.

**Packing variants.** The same tree's florets repacked against the shipped box and against it padded by 1 and 2 m. Only the box moves, and with it which florets thinning keeps. Far coverage over the 4× render (`raw/card-spread.log`):

| Fixture | shipped box | +1 m | +2 m | spread |
|---|---|---|---|---|
| beech at 20 (the old fixture) | +46.5 % | +43.6 % | +31.2 % | 15.3 points |
| **beech at 40** | **+30.1 %** | +27.6 % | +28.4 % | **2.5 points** |
| beech at 60 | +12.9 % | +12.7 % | +13.9 % | 1.2 points |

- **Over three seeds** (the trees themselves differ), the spread stays wider: at 20, +46.5 / +46.4 / +30.4 %; at 40, +30.1 / +24.8 / +21.9 %; at 60, +12.9 / +21.7 / +13.5 %.
- **The fixture** goes to the beech at 40, the smallest under 3 points.
- **The bound** is +30.1 % plus 10 points: **40 %**. The test reads 0.01207 against 0.00928 (+30.1 %) and passes. Both tests in `tests/thinning.rs` run in 0.96 s.
- **The gate.** Since `gate-render-3.log` only this test file changed; every other render test passed there.

## impl-review and the final gate (worker, 2026-10-11)

**impl-review (codex).**
- **The fresh round** (the receipt was rotated as stale) was NEEDS_WORK, with two findings:
  - the unit's culling bound left out the floret's offset and scale;
  - the attachment test did not go through selection with thinning active.
- **The fix round** (`4b63df0d`) brought a third finding: the `chosen()` accessor had moved `counted()`'s native-only attribute, breaking the Wasm build.
- **All three fixed in `1d373383`:**
  - the bound now doubles the offset and the radius;
  - the attachment test measures after the unit's enlargement;
  - the attribute is restored, and `cargo check --target wasm32-unknown-unknown` passes.
- **One part declined, with its reason:** pedicel checks in the thinned view. A thinned unit's florets are all cards, which carry no pedicel by the host's decision; the test asserts that instead.
- **The round cap** was reached and reset per the owner's standing rule (memory `reset-review-round-caps-without-asking-2026-10-09`).
- **The re-review: SHIP.**

**The new R4 test.** `every_drawn_floret_hangs_from_its_drawn_rachis` runs on both executors through selection.
- **The hero view:** 2,072 inflorescences kept; 1,484 florets drawn at a floret level, each pedicel at most 0.45 px off its rachis's chosen level once enlarged; none drawn without its rachis.
- **Six times as far** in a 96-line frame: thinning keeps 317 and drops 1,755 inflorescences whole on the device (325 and 1,747 on the CPU), and every drawn floret there is a card.

**The final gate.**
- `telperion-render`: every test target green, the cold-compile test among them (`raw/gate-render-4.log`).
- `telperion-core`: green on the packing-box fix.
- `-space`, `-field`, `-wasm` and `-jev`: green earlier, their code unchanged since.
- `npm test`: 14 files and 155 tests, green.

## The owner's verdicts (2026-10-11, viewed in the harness)

- Stalks: "yes the flowers lok like they are attached".
- Laurelin: "laurelin looks pretty good".
- Telperion: "i think the density is a little much can barely make out individual flowers. Also leaves seem to be missing? but it's pretty good."

Follow-ups (host): measure whether Telperion's leaves were dropped or only hidden, and render his flowering share at 0.3 and 0.5 beside 1 for the host to pick.

## Follow-up 1: Telperion's leaves are hidden, not dropped (worker, 2026-10-11)

**The instrument.** `examples/leaf_share.rs` builds a preset and draws three views: whole, mid and close (the same cameras as `space_two_trees`). For each view it counts the share of the frame the crown changes over the bare wood, the share the florets cover under the organ mask, and the crown's pixels no floret covers: the leaves that show. It ran on the base (`750460d3`, in its own worktree and target) and on the head. Raw: `raw/leaf_share.log`.

**Telperion at 82, seed 42:**

| | leaves | tip clusters | whole: florets / leaves showing | mid | close |
|---|---|---|---|---|---|
| base | 980,082 | 209,747 | 0.085 / 0.176 | 0.156 / 0.457 | 0.262 / 0.642 |
| head (share 1) | 980,082 | 209,747 | 0.165 / 0.107 | 0.372 / 0.274 | 0.426 / 0.437 |
| either, no flowers | 980,082 | 209,747 | 0 / 0.252 | 0 / 0.587 | 0 / 0.814 |

- **No leaf was dropped.** The leaf and cluster counts are the base's, and with the flowers off the two builds draw the same frames. Every tip that bears an inflorescence keeps its fn-223 cluster: the conversion seats both at the axis's last node, independently. On the device, the gate's leaf parity tests showed it placing the leaves the CPU places.
- **They are hidden.** At a share of 1 the florets cover about twice what they did, and they stand at the crown's surface. The leaves that show fall by 39 % at the whole view and 40 % at the mid view. (The close camera aims at a floret, so it differs between the builds.)

## Follow-up 2: Telperion's flowering share at 0.3, 0.5 and 1 (seed 42), for the host

`STALKS-telperion-share.jpg` (viewed): one row per share, mid and close views at 1920×1080 (sources in `raw/share/<share>/`). Frame rates are `frames.rs`'s best of 3 at 1080p and seed 42 (`raw/share/frames-<share>.log`):

| share | florets and anchors | whole | mid | close |
|---|---|---|---|---|
| 0.3 | 420,016 | 115.1 fps | 49.2 | 54.5 |
| 0.5 | 700,699 | 99.9 | 41.6 | 49.5 |
| 1 | 1,258,482 | 81.9 | 33.9 | 43.0 |

- **The look.** At 0.3 the candles stand apart, with leaves between them, and single flowers read at the close view. At 0.5 the clusters begin to merge at the mid view. At 1 the mid view is mostly white.
- **Seed 42 is not seed 1.** The preset's own seed grows the larger tree, so every rate here is below the seed-1 figures above.
- **The host picks the share.** Telperion's value file is unchanged until then.

## Telperion's share set to 0.3 (host, 2026-10-11)

- **The decision.** The host picked 0.3 from `STALKS-telperion-share.jpg`: the candles stand apart with leaves between them, which answers the owner's "can barely make out individual flowers". The owner gets the final look in the harness.
- **What changed.** Only `telperion.values` moved: `/supernatural/flowering/age14` from 1 to 0.3, its comment stating why. Laurelin stays at 1.
- **Pins and generated files.** The catalogue's family and walk digests are re-pinned for that one row, and `src/browser/presets.generated.ts` is regenerated.
- **Tests.** `telperion-core` (the catalogue identity among them), the render conformance test and `npm test` (14 files, 155 tests) are green.

**The leaves, recorded for the owner's "leaves seem to be missing?".** They were hidden, not dropped. Telperion at 82, seed 42 keeps 980,082 leaves and 209,747 tip clusters on the base and on the head alike. With his flowers off the two builds draw the same frames. Every tip that bears an inflorescence keeps its leaf cluster. At a share of 1 the florets covered about twice the frame they did before fn-253, standing at the crown's surface, and the leaves showing fell by 39 % (whole view) and 40 % (mid view); see "Follow-up 1" above.

**For fn-259.** Telperion's mid view at seed 42 runs under 60 fps even at 0.3: 49.2 fps (whole 115.1, close 54.5; `raw/share/frames-0.3.log`, 1080p, best of 3).
