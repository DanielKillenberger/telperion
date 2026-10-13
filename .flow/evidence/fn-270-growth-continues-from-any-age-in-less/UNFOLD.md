# fn-270 task 4: leaves and shoots unfold within the year (worker, 2026-10-13)

R5 and R5a, CPU then device, and R4's oak at 80. Raw output is in `raw/t4/` (ignored).

## The rule (`telperion_space::unfold`, `present.wgsl`)

- **Time.** A year runs 365 days from bud burst; a draw at fraction `f` is `365 f` days in.
- **Curves.** A logistic course over a span, 0 at its start and 1 at its end exactly (4% at a fifth, 96% at four fifths). Pallardy 2008 (*Physiology of Woody Plants*, ch. 3): "seasonal and lifetime growth of shoots ... generally conform to such a [sigmoid] pattern".
- **Shoots.** This year's phytomers of an axis (those it did not hold a year before; a woken bud's whole shoot) extend from its base by the course over its PA's `elongation` days, measured by the wood each grows (scale times its internode's spread), each internode filling in turn.
- **Leaves.** A node's leaf expands over its PA's `expansion` days from the moment the shoot reaches it.
- **Old organs.** Every phytomer of both years moves by `f` from last year's drawing (tip, frame, girth, scale), keyed by axis identity and place. The leaves the wood's age keeps move by `f` from last year's share.
- **Gone subtrees.** Drawn as last year, carried with the wood they stood on, girth and scale faded by `f`.
- The whole year (`f = 1`) is the tree as grown. The palm's rosette fronds are drawn whole: a rosette has no per-frond age (open).

## Curves and sources (each value checked against the quoted text)

| Preset | Elongation (d) | Expansion (d) | Source |
|---|--:|--:|---|
| european-beech, Laurelin, Telperion | 14 | 14 | Kozlowski and Pallardy 1997: fixed growth "complete[s] shoot expansion in 2 to 6 weeks" (low end); Urban et al. 2014, iForest 8: 438: bud break DOY 117, full foliation DOY 131 (2012); long-term 104 to 132 |
| oregon-white-oak | 14 | 35 | Q. alba stands in (none for Q. garryana): Dougherty et al. 1979, Plant Physiol. 64: 930, leaves ">90% some 4 to 5 weeks following bud burst", shoots "underway or completed" 12 days after growth began |
| silver-birch | 110 | 24 | Campos et al. 2023, ISPRS Archives XLVIII-1/W2: 1715, height growth DOY 125 to late August; Robson and Aphalo 2012, a leaf measured over 24 days |
| norway-spruce | 35 | 35 | Rossi et al. 2009, Ann. For. Sci. 66: 206: "35 days of shoot and needle lengthening" |
| date-palm | 365 | 0 | Zaid and de Wet 2002, FAO 156 ch. I: one terminal bud, "an annual formation of 10 to 26 new leaves" |
| ordinary | 19.08 | 25.89 | The corners' point (`blend::centroid`, as `examples/centroid.rs`) |

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

The state keeps last year's laid node table (Telperion about 370 MB) and settles it on every `grow_to` that commits a year. Telperion's draw within its year first peaked at 4,478 MB, past the 4 GiB default (4,295 MB); this year's settle is now freed before the presented tree is drawn, and it peaks at 4,160 MB, 135 MB under (Laurelin 4,160 before, 3,859 after). Before this task the grown-on peaks were Telperion 3,072 and Laurelin 2,836 (MEMORY.md): last year's table and its settle on `grow_to` cost about 840 MB on Telperion.

Warm build times, A/B against the task's base on the same machine (ms, base / task): ordinary 464 / 477, oak 237 / 234, spruce 449 / 453, beech 194 / 202, birch 218 / 221, palm 73 / 73, Telperion 1,283 / 1,253, Laurelin 1,158 / 1,157; device peaks identical.

## The oak at 80 (R4)

A build starts at the hint's margin and halves it only where the device budget refuses the tables (`Sizes::margins`). The oak at 80 now builds under the 4 GiB default: 1,486,513 instances, warm 1,038 ms, ledger peak 4,209 MB (the refused first attempt's reservations; under the cap by construction). No shipped preset at its age is refused at the first margin, so none takes the new path.
