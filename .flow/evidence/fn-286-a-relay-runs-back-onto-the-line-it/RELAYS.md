# fn-286: a relay runs back onto the line it takes over

Base: F4 at 28f495793 with fn-236's sourced sprig segment (ce22a336d). Probes: `relay_turns.rs` (every relay's drawn turn, seeds 1-10, `turns.sh`), fn-236's `seed7/oak_lines.rs` (the oak's limb lines, seeds 1-50, `seed7/lines.sh`), `relay_views.rs` (stills, `views.sh`). Raw lines in `raw/` (ignored); sheets in `sheets/`, viewed.

## The mechanism

`takeover`, a per-PA share: a relay's phytomers are pulled onto its parent's running direction at the parent's tip, by `takeover` times the relay's blend at its base, fading to none over the relay's own reach, as its straightening fades (`geometry::taken`, `Layer::step`). The pulls towards the vertical share what it leaves. 0 is today's tree to the bit: the oak's fifty lines at 0 equal the run before the row existed byte for byte, and `tests/takeover.rs` holds a tree without relays unchanged whatever the row says. The device holds it in four pencil columns (`P_TAKE`, `P_LINE`) and the PA form's twentieth word.

## Values, by measure (relay_turns, seeds 1-10)

A relay's *joint* is its turn off the parent's heading at its node to its first drawn phytomer; *chord*, to the chord 0.5 m on.

| preset | takeover | relays | joint ≥ 40° | chord ≥ 40° | joint p90 (median of seeds) |
|---|---|---|---|---|---|
| oak (7, 9; limb 6 kept at 0) | 0 → 0.75 | 11,956 | 3,173 → 2 | 167 → 0 | 57.2 → 20.0 |
| beech (2, 4, 6, 7) | 0 → 0.5 | 1,598 | 121 → 0 | 29 → 0 | 34.0 → 20.2 |
| Telperion (beech's) | 0 → 0.5 | 19,725 | 1,630 → 8 | 405 → 1 | 37.0 → 26.7 |
| Laurelin (beech's) | 0 → 0.5 | 18,213 | 1,515 → 7 | 381 → 1 | 37.1 → 26.5 |
| birch (5, 6, 7) | 0 → 0.5 | 2,649 | 4 → 0 | 50 → 7 | 34.3 → 17.6 |
| ordinary (0-4, 6, 7, 9, 10) | 0 → 0.75 | 997 | 242 → 1 | 79 → 0 | 52.7 → 19.9 |

- The beech's relays erect by its straightening (1.0 on the trunk and leader, 0.6 on its modules); at takeover 1 the two fight and joints of 40° or more rise again (beech 48, Telperion 5,883), so the beech family takes 0.5.
- The oak's limbs (PA 6) keep their relays' kinks, the tortuous course `oak.rs` asks for.

## The oak at 40, seeds 1-50 (oak_lines; drawn joins)

| per tree | F4 (median / max, seed 7) | sprig 0.031 | + takeover |
|---|---|---|---|
| young-stem lines reaching past crown95 | 13 / 20, 17 | 1 / 4, 4 | 1 / 4, 4 |
| farthest tip / crown95 | 1.40 / 1.63, 1.60 | 1.05 / 1.33, 1.13 | 1.04 / 1.32, 1.13 |
| relay turns ≥ 40° on those lines | 1 / 4, 3 | 1 / 4, 3 | 0 / 0, 0 |
| relay turns ≥ 40° on every line | 3 / 10, 5 | 3 / 11, 5 | 0 / 0, 0 |
| kinks > 45° (0.5 m chords), every line | 0 / 2, 2 | 0 / 2, 2 | 0 / 1, 0 |
| crown95 (m) | 3.35 | 2.96 | 2.96 |

Five kinks over 45° are left, one each at seeds 4, 11, 24, 34 and 42. Three are limbs' (PA 6) relays in close succession (51-62° over 0.5 m chords), the course `oak.rs` keeps. Two are on young-stem lines: two relays 0.13 m apart adding to 46° (seed 42), and a relay of 14° beside the line's continuation into its shoot age at 33° (seed 24). None is a single hook.

## Cost (CPU executor, the probe's build, ms and nodes, seeds 1 / 7 / 42)

| preset | today ms | new ms | today nodes | new nodes |
|---|---|---|---|---|
| oak | 718 / 1,057 / 859 | 751 / 1,064 / 811 | 280,024 / 371,210 / 284,328 | 275,902 / 384,593 / 285,930 |
| beech | 172 / 252 / 379 | 173 / 230 / 341 | 114,649 / 205,341 / 249,111 | 114,700 / 205,400 / 251,816 |
| birch | 499 / 940 / 602 | 504 / 958 / 632 | 242,346 / 355,757 / 301,033 | 242,453 / 355,649 / 300,787 |
| Telperion | 6,343 / 11,023 / 11,595 | 6,374 / 10,951 / 11,499 | 1,985,984 / 3,355,228 / 3,363,723 | 1,985,624 / 3,358,968 / 3,359,470 |
| Laurelin | 5,815 / 9,763 / 10,614 | 5,790 / 9,844 / 10,762 | 1,859,233 / 3,122,312 / 3,139,949 | 1,859,573 / 3,127,439 / 3,143,210 |
| ordinary | 263 / 375 / 435 | 244 / 322 / 357 | 86,353 / 130,350 / 152,170 | 86,361 / 130,355 / 152,178 |

These are the CPU build in one process, not the harness's warm device time; within run-to-run noise the row costs nothing.

## Stills (sheets/, today left, new right, seeds 1, 7, 42), viewed

- **Oak:** the long low wands and the left hook at seed 7 are gone. The crown is narrower than today's (crown95 3.35 → 2.96 m): the wands made much of today's width. The owner's call whether the narrower oak reads better.
- **Telperion, Laurelin:** limbs run straighter and reach further; the crowns read a little wider and more open (Laurelin seed 1 shows a gap in the crown's middle). Not clearly better or worse: the owner's call.
- **Beech, birch, ordinary:** no difference visible at the whole-tree view.

## The device's refusal of the oak at 9 (host decision 1, 2026-10-15)

- **Symptom:** with fn-236's sourced sapling segment (0.031 m), every device build of the oak that reached cycle 9 was refused: "the build passed its sizes at cycle 9: Given (0x1)". At 0.07 m it built. The `takeover` row was not involved: with every take-over at 0 the refusal stood.
- **Which size (`CYCLE_TRACE=1`):** `Div(Add(At(32), At(44)), C(64))` given 9 workgroups, needed 10: the glue's `BORN` dispatch over the axes a cycle's units (census word `MADE + T_UNITS`) and wake (`MADE + T_WAKE`) made (`plan_units.rs`, `plan_made`).
- **Which formula:** `given::Span::new` bounds each census word for the cycle two on from the census it read. For these two words it used the generic projection, from the word's own last two values: its count, plus the larger of three times its last step and its count times its rate squared less one, plus 64 and an eighth (`given.rs`).
- **Why the shorter segment breaks it:** the oak's new axes a cycle (seed 1) ran 224, 270, 372, then 591 at cycles 6-9. Read at cycle 7 (270 after 224, rate 1.2), the projection for cycle 9 reached 505, plus 64 for the wake: 569, 9 workgroups of 64 (rounded up). 591 needed 10. At 0.07 m the young oak's new axes grew steadily (286, 567, 921, 1,500), a rate its own projection follows. At 0.031 m the young crown makes axes in a lull and then a burst, which a projection of the flow's own rate cannot see. The axis table's own span (its count, from `AXES0` to `AXES`) did hold it.
- **Fix (`given.rs`, general):** every word a cycle's units and wake make, its axes, phytomers, units and sleeping buds (`plan_made`'s order), is now bounded by that table's end less its start, held as the table is. fn-276 already held the phytomers this way after the beech's burst at 26. It is derived from the table's own bound, with no projection of its own, no species branch and no added margin. Test: `a_burst_of_made_axes_is_within_its_table` (the oak's numbers). It is red on the old formula: the bound reached 9 workgroups for 10 needed.
- **Memory:** `Span` sizes dispatches' workgroups only; no table, buffer or arena reads it (`build_grow.rs:272` is its one use), so no preset's memory moves. The cost is invocations given past the true count; see the measurements below.

## The within-year test at f = 1e-6 (host decision 2)

- **Symptom:** `unfold::tests::a_tree_drawn_within_its_year_joins_both_years_with_no_pop` (the oak at 24, seed 7) fails its 1e-3 tolerance at f = 1e-6 with take-over on, and passes with it off.
- **What moves:** one organ, a leaf of an axis *gone* in year 24 (drawn by `Presenting::fade`). Its leaf share at f = 1e-6 is 0.998216 against 1.0 a year before. Nothing reads an unfrozen reach; the take-over only changed which tree grew.
- **Why:** a gone axis's leaf is `(r(1 - f) - FINEST) / (r - FINEST)` (`unfold.rs`, `fade`), its wood's radius `r` thinning to the thinnest drawn. That phytomer's radius is 0.00020011 m, 1.1e-7 m above `FINEST` (2e-4 m). The leaf's slope in f is `r / (r - FINEST)`, about 1,800, so at f = 1e-6 it has already fallen 0.0018. Measured 1 - 0.998216 = 0.001784; the formula gives 0.00178. Any tree with a gone twig just above `FINEST` pops this way; with take-over off, the oak at 24, seed 7, happened to have none.
- **Not decided:** the fade law is the fn-270 presentation's, and how a near-`FINEST` leaf should fade is a design choice (for example, the leaf fading at least as slowly as the fraction). Making take-over "read what straightening reads" would not change it. Left for the host.
