# fn-270 task 4: leaves and shoots unfold within the year (worker, 2026-10-13)

R5, R5a, R5b and R5c, CPU then device, and R4's oak at 80. Raw output is in `raw/t4/` (ignored).

## The rule (`telperion_space::unfold`, `present.wgsl`)

- **Time.** A draw at fraction `f` of its last year is `365 f` days past the whole age. Each PA's year (R5c, host 2026-10-14): the whole age stands at full leaf, which is budburst plus the shoot's elongation plus the leaf's expansion; from there autumn (leaves past their lifespan colour, then fall), winter bare, budburst, full leaf at the next whole age.
- **Curves.** A logistic course over a span, 0 at its start and 1 at its end exactly (4% at a fifth, 96% at four fifths). Pallardy 2008 (*Physiology of Woody Plants*, ch. 3): "seasonal and lifetime growth of shoots ... generally conform to such a [sigmoid] pattern".
- **Shoots.** This year's phytomers of an axis (those it did not hold a year before; a woken bud's whole shoot) extend from its base from budburst by the course over its PA's `elongation` days, measured by the wood each grows (scale times its internode's spread), each internode filling in turn.
- **Leaves.** A node's leaf expands over its PA's `expansion` days from the moment the shoot reaches it. A leaf of older wood past its lifespan (`leafLife`, a draw by its lineage) colours toward the species' named autumn colour (`leafDead*`, through the withered bit and one per-tree colouring uniform), stays whole on the tree, and is gone on its own abscission day, a draw by its lineage within the fall window, so the crown thins leaf by leaf (owner, 2026-10-14). Where its wood keeps a leaf it grows again by the expansion course from budburst. Only that expansion ever scales a leaf.
- **Old organs.** Every phytomer of both years holds last year's drawing (tip, frame, girth, scale, sag) until budburst, moves along its PA's elongation course to this year's, and holds there (owner, 2026-10-14: no creep through summer, autumn and winter). An axis's base moves with its bearer's course. Keyed by axis identity and place.
- **Gone subtrees.** Drawn as last year, carried with the wood they stood on; the wood thins by `f` to nothing; its leaves all pass, each whole until its abscission day, which comes no later than its wood reaches the thinnest drawn (2e-4 m).
- **Leaves the wood keeps no longer.** Fall as a passing leaf does; which stations stand follows last year's retention, which holds every station kept now (`Leafing::was`).
- **Short shoots.** A short shoot that stood a year before keeps its cluster at whichever tip it stands; a new one's cluster expands.
- **Relay joins.** A relay born this year joins from its bearer's own wood along its course; a gone one fades back into it by `f`.
- **Palm fronds (R5b, owner option a; host 2026-10-14).** Each frond is keyed by its serial in the spiral, its turn `serial x divergence` and its draws by the stem's first node. The crown makes its living count over a frond's life: `frondLife` 5 years ("a normal life of 3 to 7 years", Zaid and de Wet 2002, FAO 156 ch. I; the midpoint), so 42 / 5 = 8.4 fronds a year. Cross-check: FAO's "annual formation of 10 to 26 new leaves" on 100 to 125 green leaves; this crown, about a third of a real one's count, makes 8.4, a sixth under the low end. A frond made in a year before stands whole all year; this year's emerge evenly over the stem's elongation and expand over 139 days from emergence, small at the centre. Each frond stands a place further down for every frond grown above it, by degree as each unfolds, so the crown below moves out only as the new ones grow; it turns from living to dead over its last living place and fades over the skirt's last. With no `frondLife` the crown is dormant.
- **Seasons that overlap budburst (host, 2026-10-14).** A colouring and fall that would run past the next budburst are compressed by degree to end at it (`Season::of`); nothing is refused. A walk between the palm and a deciduous species is a tree again.
- The whole year (`f = 1`) is the tree as grown.

## Curves and sources (each value checked against the quoted text)

| Preset | Elongation (d) | Expansion (d) | Source |
|---|--:|--:|---|
| european-beech, Laurelin, Telperion | 14 | 14 | Kozlowski and Pallardy 1997: fixed growth "complete[s] shoot expansion in 2 to 6 weeks" (low end); Urban et al. 2014, iForest 8: 438: bud break DOY 117, full foliation DOY 131 (2012); long-term 104 to 132 |
| oregon-white-oak | 14 | 35 | Q. alba stands in (none for Q. garryana): Dougherty et al. 1979, Plant Physiol. 64: 930, leaves ">90% some 4 to 5 weeks following bud burst", shoots "underway or completed" 12 days after growth began |
| silver-birch | 110 | 24 | Campos et al. 2023, ISPRS Archives XLVIII-1/W2: 1715, height growth DOY 125 to late August; Robson and Aphalo 2012, a leaf measured over 24 days |
| norway-spruce | 35 | 35 | Rossi et al. 2009, Ann. For. Sci. 66: 206: "35 days of shoot and needle lengthening" |
| date-palm | 226 | 139 | Derived, low confidence: frond elongation "24 mm/day" to "36 mm/day" (Zhen et al. 2015) over a 4 m frond (Zaid and de Wet 2002, FAO 156 ch. I) gives 111 to 167 days, 139 the midpoint; the stem takes the 226 days left. 18 fronds a year, the midpoint of "10 to 26 new leaves" (ibid.) |
| ordinary | 19.08 | 25.89 | The corners' point (`blend::centroid`, as `examples/centroid.rs`) |

## The seasonal year's rows (R5c; sources quoted beside each row in the value files)

| Preset | Leaf life (y) | Budburst (DOY) | Senescence (DOY) | Colouring (d) | Fall (d) | Autumn colour (named, not measured) |
|---|--:|--:|--:|--:|--:|---|
| european-beech, Laurelin, Telperion | 0.54 | 115 | 264 | 16 | 45 | copper (RHS) |
| oregon-white-oak | 0.53 | 128 (Q. robur stand-in) | 264 | 16 | 45 (beech by the centroid rule) | brown |
| silver-birch | 0.61 | 101 | 277 | 5 | 45 | yellow |
| norway-spruce | never (leafYears, host 2026-10-14) | 132 | 264 | 16 | 45 (stand-ins, dormant) | - |
| ordinary | 1.30 | 125 | 264 | 16 | 45 | the corners' point |
| date-palm | never | - | - | - | - | its skirt's dead colour |

Lifespans are rounded down to the second place so no leaf of the lifespan's year stands into winter.

## Fail-early premise (CPU)

`unfold::tests::a_tree_drawn_within_its_year_joins_both_years_with_no_pop`, the oak at 24: at `f = 0` every old organ is year 23's to the bit and every new one at scale and leaf 0; at fractions 1e-6 to 1 − 1e-6, halving the step at least nearly halves every organ's largest move (no step); just past the start the tree is year 23's within 1e-3, just before the end year 24's; `settle_at(24)` is `grow_cycle`'s tree; no draw moves the grower. Held. A probe over six presets at 6, 12, 20 and 30 found no axis that loses a phytomer between years (the test asserts it).

Stills, the oak preset at 40, seed 7, the top of the crown, the whole year's camera (`examples/unfold.rs`): `raw/t4/oregon-white-oak-40-7-{0.001,0.03,0.07,1}-crown.png`. At 0.001 the crown is last year's; at 0.03 (11 days) the year's shoots stand out bare-tipped with small leaves; at 0.07 the leaves are near whole; at 1 the year's tree.

## Device

`present_tests::a_tree_drawn_within_its_year_on_the_device_is_the_cpus`: every young preset at 0.02, 0.1 and 0.6, every node matched by lineage, positions within 1.3e-5 m, radii within 5e-8 m, leaf shares within 7e-7. The split, agreement, station, rosette, boot and repeat tests hold (`raw/t4/dev-focused.log`).

## Memory and time (device, decimal MB; `presets_grow_on_on_the_device`)

| Preset | Paused at H−1 | Grown on beside the drawn | Drawn at H beside the drawn | Drawn at H − 0.5 beside the drawn |
|---|--:|--:|--:|--:|
| telperion (82) | 1,953 | 3,544 | 3,912 | 4,160 |
| laurelin (80) | 1,790 | 3,281 | 3,627 | 3,859 |
| oregon-white-oak (40) | 475 | 874 | 711 | 770 |

The state keeps last year's laid node table (Telperion about 370 MB) and settles it on every `grow_to` that commits a year. Telperion's draw within its year first peaked at 4,478 MB, past the 4 GiB default (4,295 MB); this year's settle is now freed before the presented tree is drawn, and it peaks at 4,160 MB, 135 MB under (Laurelin 4,160 before, 3,859 after). Before this task the grown-on peaks were Telperion 3,072 and Laurelin 2,836 (task 3's census): last year's table and its settle on `grow_to` cost about 840 MB on Telperion.

Warm build times, A/B against the task's base on the same machine (ms, base / task): ordinary 464 / 477, oak 237 / 234, spruce 449 / 453, beech 194 / 202, birch 218 / 221, palm 73 / 73, Telperion 1,283 / 1,253, Laurelin 1,158 / 1,157; device peaks identical.

## The oak at 80 (R4)

A build starts at the hint's margin and halves it only where the device budget refuses the tables (`Sizes::margins`). The oak at 80 now builds under the 4 GiB default: 1,486,513 instances, warm 1,038 ms, ledger peak 4,209 MB (the refused first attempt's reservations; under the cap by construction). No shipped preset at its age is refused at the first margin, so none takes the new path.

## R5b, R5c and the owner's two rulings of 2026-10-14 (worker)

- **Leaves fall whole.** `tree::Unfolded::leaf`: a passing leaf, or one its wood keeps no longer, stands at full size and colours until its abscission day, a draw on its run's stream salted by `FALL` against the share of the fall gone; then it is gone. Device `pack.wgsl`'s `leaf_share` draws the same (`unfold_tests`: every one of 64 stations the CPU's share at half the fall, with retention kept and not; `leafing_tests`: shares are 0 or 1, a quarter of 400 gone at a quarter of the fall).
- **No creep.** `unfold::Presenting::moving` and `present.wgsl`'s `moving`: old organs at last year's drawing to budburst, along elongation, then held. `unfold::tests::old_organs_move_only_while_the_shoots_grow` (oak at 12): no organ of both years moves between 0 and budburst or between the end of the latest elongation and 1 − 1e-6; the no-pop ladder holds. Device agreement at 0.02, 0.35, 0.6, 0.88 (mid-elongation) and 0.95, all eight young presets: positions within 1.4e-5 m, radii 5e-8, leaf shares 4e-6, seasons equal.
- **Palm fronds on the device.** `stations.wgsl`'s `rosette` draws `rosette::fronds` from `Crown::rows` (`Growth::year`) and a fixed-point divergence; `every_rosette_is_placed_as_the_cpu_places_it` holds at the whole age and at 0.3 and 0.8 (apart 3.1e-5 of the height, withering identical). A frond at no size now keeps the family's leaflet count on the CPU, as the plan and the device count it.
- **Catalogue.** All four digests re-pinned; family, walk and override verified by stripping `rosetteYear` and reading the last pins' value files.
- **Stills** (oak at 40 seed 7, palm at 12 seed 1, crown camera): `raw/t4/r5bc/oregon-white-oak-40-7-{0.001,0.3,0.6,0.9}-crown.png`, `raw/t4/r5bc/date-palm-12-1-{0.001,0.3,0.6,1}-crown.png`. The oak at 0.3 is in full-size brown leaves a week into its fall, at 0.6 bare, at 0.9 leafing out; the palm at 0.3 has this year's fronds part emerged and small above last year's crown.
- **Cost a frame (CPU draw, `growth::mesh`, oak at 40):** the whole year 768 ms; within it 786 ms at 0.3, 927 ms at 0.88, 989 ms at 0.95 (more wood and leaves stand as this year's shoots join last year's gone ones).

## The host's rulings after the gate (worker, 2026-10-14)

- **Budget.** A build whose least tables the budget cannot hold is refused by name before any submit; one whose least tables fit starts and may be refused by name while growing, the tree drawn before it untouched (`full_card_tests::a_build_past_the_budget_is_refused_by_name_and_leaves_the_drawn_tree`, both cases).
- **Palm stills, frond turnover** (`raw/t4/palm-turnover/date-palm-12-1-{0.001,0.3,0.6,1}-crown.png`): the upper crown keeps last year's fronds whole through the year; the new ones grow small at the centre. CPU and device agree at 1, 0.3 and 0.8 (3.1e-5 of the height apart; same fronds withered). The year turns with no frond changing size (`suite::rosette::last_years_fronds_stay_whole_and_the_year_turns_with_no_pop`).
- **Dress cold make.** The dress works a node's leaves once, not at each record it places; its pipeline makes cold under 2.5 s again (was 2.86 to 3.1 s).
