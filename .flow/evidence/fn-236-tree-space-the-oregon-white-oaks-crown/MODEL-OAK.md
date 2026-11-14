# fn-236 R1: the Oregon white oak's open-grown crown, 2026-10-09

The Oregon white oak (*Quercus garryana* Dougl.), preset `oregon-white-oak`, as an open-grown tree at 40 and 80 years. This file holds what the sources say its crown is, the engine's current values read against them, and which traits values can reach. It was written before any value moved (round 0 is today's preset at `22d29326`). Every number is quoted from its source or computed by code from a quoted number; a value no source gives is marked **estimated**.

## Sources

Fetched texts sit in the spec's ignored `raw/lit/`; the catalogue's extracts are in `catalogue/oregon-white-oak/sources/`.

| Tag | Source | Kind | Read |
|---|---|---|---|
| **OWIC** | Niemiec, Ahrens, Willits & Hibbs 1995, *Hardwoods of the Pacific Northwest*, as the Oregon Wood Innovation Center's page ([owic.oregonstate.edu/node/79](https://owic.oregonstate.edu/node/79)) | Forestry account of *Q. garryana* | Catalogue extract |
| **SILV** | Stein, W.I. 1990. Oregon white oak. *Silvics of North America* vol. 2, USDA Agric. Handb. 654 | Silvics of *Q. garryana* | Catalogue extract (full text) |
| **OSU** | Oregon State University Landscape Plants, *Quercus garryana* | University page | Catalogue extract |
| **PH82** | Paine, D.P. & Hann, D.W. 1982. *Maximum crown-width equations for southwestern Oregon tree species.* OSU Forest Research Lab Res. Pap. 46 ([ir.library.oregonstate.edu](https://ir.library.oregonstate.edu/downloads/b2773w92s)) | Regression on 75 open-grown *Q. garryana*, D 0.6 to 37.3 in | Full text, `raw/lit/paine-hann1982.md` |
| **G08** | Gould, P.J., Marshall, D.D. & Harrington, C.A. 2008. Prediction of growth and mortality of Oregon white oak in the Pacific Northwest. *West. J. Appl. For.* 23(1) ([treesearch 29814](https://research.fs.usda.gov/download/treesearch/29814.pdf)) | Growth model, open-grown data from SW Washington | Full text, `raw/lit/gould2008.md` |
| **G11** | Gould, P.J., Harrington, C.A. & Devine, W.D. 2011. Growth of Oregon white oak (*Quercus garryana*). *Northwest Science* 85(2) | Growth model and dendrometer study, open-grown class included | Full text, `raw/lit/gould2011.md` |
| **O-WHOLE** | `.refs/fn9/quga788B.jpg`, an open-grown *Q. garryana* in a pasture, age not stated | Reference photograph | Read by code (`photo_crown.py`) |
| fn-195 | BC07, PR00, T83, K19, CIR, HEIN (fn-195 `SOURCES.md`; fn-190 `MODEL-OAK.md`) | Architecture of *Q. robur* and oaks in general | As fn-195 read them |

**A note on the architecture's species.** `oak.rs` and fn-195's rules 1 to 10 are written from *Q. robur* and from oaks in general (Rauh's model, Kędra's forks). No source read here gives *Q. garryana* its own architecture; the rounds below keep those rules and move only rates, angles, lifespans and shares.

**Not run:** Jev's literature screen. Every number used is quoted below with its stated age or size and growing condition, and the derived ranges are computed by code from the quotes.

## The open-grown crown in the sources

| # | Trait | Quote | Age and condition stated |
|---|---|---|---|
| T1 | **Broad, rounded, dense crown** | SILV: "Typically, open-grown trees have short boles bearing very large, crooked branches that form dense, rounded crowns (fig. 3)." OWIC: "Open-grown trees develop very broad, rounded crowns (crown width may equal total height) with massive, crooked branches on short, massive boles." OSU: "broad spreading, rounded crown, rugged, heavy ascending, crooked branches". SILV: "Branch development on open-grown trees may be very dense." | Open-grown; mature, age not stated |
| T2 | **Crown width to height about 1** | OWIC: "crown width may equal total height". O-WHOLE by code: width 1.31 of height (935 × 716 px, ground line to top). | OWIC open-grown, no age; the photograph's tree is old, age not stated |
| T3 | **A short bole: the crown starts low** | SILV, OWIC: "short boles", "short, massive boles". O-WHOLE by code: the lowest foliage at 0.12 of the height. | Open-grown |
| T4 | **Few, massive, crooked, ascending limbs** | SILV: "very large, crooked branches"; OSU: "heavy ascending, crooked branches"; OWIC: "massive, crooked branches". | Open-grown, mature |
| T5 | **Shrubby when young** | OSU: "often shrubby when young (less than 25 years)". | Under 25 years |
| T6 | **Mature size** | SILV: "mature Oregon white oak trees are 15 to 27 m (50 to 90 ft) tall and 60 to 100 cm (24 to 40 in) in d.b.h."; record "crown spread of 38.4 m (126 ft)". OWIC: "Mature Oregon white oaks are 50 to 90 ft tall … 24 to 40 in. in DBH". | "Under favorable conditions", mature; not open-grown specifically |
| T7 | **Slow height growth** | OWIC: "Height growth is usually less than 1 ft per year". SILV: a full stand "47 to 70 years old … averaged 14 m (46 ft) in height, 15 cm (6.0 in) in d.b.h."; forest-grown trees "95 to 135 years old … averaged 24 m (80 ft) tall and 48 cm (19 in) in d.b.h." | Stand-grown; OWIC no condition |
| T8 | **Diameter growth of an open-grown oak** | G08: "The maximum predicted diameter growth of an open-grown oak (SI, 100) is 1.27 in./5years"; "Previously reported 5-year growth rates in Washington range from 1.4 to 2.0 in. (Witt 1979, Stein 1990)". G11: growth peaks "when DBH is about 40 cm, stand BA is low (e.g., open-grown trees)". | Open-grown, SW Washington; ages not stated |
| T9 | **Crown width of an open-grown oak by its diameter** | PH82, table 2, Oregon white oak: MCW = 3.0786 + 1.9242 D (ft, D in inches), fitted on 75 open-grown trees, D 0.6 to 37.3 in; "crown diameters were measured … The widest crown diameter and that at a 90-degree angle" (geometric mean). | Open-grown, SW Oregon, by diameter, not age |

**Ranges at 40 and 80 years, computed by code from T7 to T9.** A constant 1.0 to 2.0 in of diameter each 5 years (T8, rounded to span G08's 1.27 and Witt's 1.4 to 2.0; that the rate holds from planting is **estimated**) gives D 8 to 16 in (20 to 41 cm) at 40 and 16 to 32 in (41 to 81 cm) at 80. Through PH82 that is a crown 5.6 to 10.3 m wide at 40 and 10.3 to 19.7 m at 80. At under 0.3 m a year (T7), the height is under 12 m at 40 and 24 m at 80; SILV's stands put 14 m at 47 to 70 and 24 m at 95 to 135 years. **The target ranges** used below, all derived and so marked: at 40, height 8 to 12 m, crown 6 to 11 m wide, DBH 20 to 41 cm; at 80, height 14 to 24 m, crown 10 to 20 m wide, DBH 41 to 81 cm; width over height 1.0 to 1.3 at both (T2).

## The form measures (`oak_measure.rs`, `photo_crown.py`)

Code measures every tree grown from the preset through the pipeline, as `space_oak` grows it. Crown points are the nodes of fine wood (radius under 5 mm), where the leaves sit.

- **height, width, w/h:** the tallest node; the mean of the crown points' x and z extents.
- **base:** the crown points' 2nd height percentile.
- **fill:** in two side views on a grid of height/60, occupied cells over the cells inside each row's outline. A see-through crown lowers it.
- **solid:** the row outlines' area over their convex hull's. A limb past the crown's edge lowers it.
- **spikes:** of 36 plan sectors about the trunk, those whose farthest crown point lies past 1.25 times the median sector's reach.
- **top:** the highest crown point above the 99th height percentile: a tuft above the dome.
- **dbh, mcw:** the stem's diameter at 1.3 m, and PH82's crown width for it.

O-WHOLE by `photo_crown.py`, same grid: **w/h 1.31, fill 0.99, solid 0.92**, lowest foliage at 0.12 of the height. The photograph's tree is older than 80 and seen from one side, so its fill and solid are the targets in kind (a closed crown, a round outline), not to the digit.

## Round 0: today's values read against the sources

Today's preset at `22d29326`, seeds 1 / 7 / 42 (`raw/measure-r0.jsonl`, stills `raw/sheet0/`, sheets `raw/compare/compare0-{40,80}.png`):

| Measure | 40 | 80 | Target (derived) |
|---|---|---|---|
| Height, m | 12.5 / 13.5 / 12.4 | 22.3 / 26.9 / 24.2 | 8 to 12; 14 to 24 |
| Crown width, m | 10.3 / 11.1 / 10.3 | 21.8 / 29.6 / 25.4 | 6 to 11; 10 to 20 |
| Width / height | 0.83 / 0.82 / 0.84 | 0.98 / 1.10 / 1.05 | 1.0 to 1.3 |
| Crown base, m | 2.6 / 4.7 / 3.3 | 5.4 / 5.6 / 8.7 | low (T3) |
| Fill | 0.65 / 0.80 / 0.74 | 0.87 / 0.88 / 0.87 | towards 0.99 |
| Solid | 0.66 / 0.63 / 0.72 | 0.73 / 0.84 / 0.81 | towards 0.92 |
| Spikes | 10 / 2 / 3 | 4 / 3 / 1 | 0 to 1 |
| Top tuft, m | 1.2 / 1.7 / 0.6 | 2.2 / 4.8 / 4.9 | small |
| DBH, cm | 73 / 91 / 80 | 177 / 295 / 238 | 20 to 41; 41 to 81 |
| Nodes | 167,123 / 291,579 / 218,923 | 947,012 / 2,222,419 / 1,481,549 | |
| Grown, ms (`space_oak`, load 3 to 6) | 631 / 881 / 730 | 6,663 / 13,346 / 8,133 | |

**What the stills show** (viewed; `compare0-80.png`, `r0-whole.png`, `r0-bare.png`): at 40 a narrow, conical young tree led by a strong leader, its limbs long and open, foliage in dark tufts at the tips. At 80 a crown wider than tall but ragged: a few long diagonal limbs reach past the foliage (seed 1's low left limb), the old leader stands as a tuft above the crown (seeds 7 and 42), the foliage sits in clumps at the limb ends with sky through the crown, and the trunk is very thick and flared. O-WHOLE is a closed, round dome on a short trunk; the 0.1.4 oak is a dense, round, bushy crown, brighter green.

**Against the sources:**
- Too tall and too narrow at 40 (w/h 0.83); too large at 80 (crown 22 to 30 m against 10 to 20).
- Not closed (fill 0.65 to 0.88 against 0.99) and not round (solid 0.63 to 0.84 against 0.92, up to 10 spikes).
- The stem is two to four times too thick for its age (DBH 0.7 to 0.9 m at 40, 1.8 to 3.0 m at 80, against SILV's mature 0.6 to 1.0 m), so PH82's crown for that stem is 18 to 69 m.

## The values that make these traits (`oak.rs`, the preset's `/space/ages/<pa>` rows)

| Trait | Values today | Reading |
|---|---|---|
| Conical at 40, tuft at 80 | `LEADER` (4) lives 19 years above the 2-year fork, bearing limbs at 0.08 and boughs at 0.1 a top bud, then carries on as a limb; stem `tropism` 0.6 to the vertical | The leader outgrows the fork's limbs for two decades and stays the tree's highest axis |
| Narrow at 40 | Fork limbs (`FORK` 3, top buds at 0.55), `LIMB` (6) `insertion` 0.5, `elevation` 1.05 rad, `tropism` 0.5, `straightening` 0.15 | Limbs rise at 60° and straighten upward |
| Limbs past the edge | `LIMB` lives 500 years at 0.045 m internodes, with `abortion` 0.15 and relays; `dominance` 0; light: `shadeSize` ψ 1.5 and `apicalControl` 0.45 on every age | A lit limb's units grow by light^1.5 against its age's mean, so a limb that reaches out of the crown into full light grows fastest and keeps reaching (inferred from the field docs in `species.rs`; not traced) |
| Open crown, clumped foliage | `LIMB` bears `BRANCH` 0.45 and `SHORT` 0.25 medially, boughs 0.08 a top bud; `BOUGH` bears `BRANCH` 0.3; `BRANCH` bears `TWIG` 0.18 to 0.25; light: `shadeHazard` φ 1, `balanceHazard` 2, `upkeep` 0.35 | Few boughs fill between the limbs, and the light sheds shaded inner shoots |
| Thick stem | `pipe` 0.009 on the stem and limbs at `exponent` 2.6 to 2.8; `retained` 0.5 (half a shed branch's pipe stays); `leafGirth` 1 | The stem keeps half the pipe of every shed branch |
| Crown base | `TRUNK` (2) lives 12 years bearing `SPRIG` (8), shed 3 years after death; fork at 12 to 14 | Base about 2.5 to 9 m |

## Capability assessment

Which traits values can reach, as far as reading the engine's fields tells; each is checked by the rounds, and a trait that a round shows values cannot reach stops for the host.

| Trait | Values that move it | Assessment |
|---|---|---|
| T2, w/h 1.0 to 1.3, and the sizes at 40 and 80 | `LEADER` lifespan and laterals, `LIMB`/`FORK` angles and internodes, `BOUGH` angles | **Values**, expected |
| T3, a low crown on a short bole | `TRUNK` lifespan, `SPRIG` shedding, the fork's years | **Values**, expected |
| T4, few massive crooked limbs | Fork counts, `abortion`/relays, `wander` | **Values**; today's already draws it |
| T1, a closed crown (fill towards 0.99) | Lateral shares of `LIMB`, `BOUGH`, `BRANCH`, `TWIG`; sleeping buds; the shade hazards | **Values, at a node cost**: more shoots cost nodes, and the light sheds shaded ones. Whether the crown closes under the light model before the node budget is unknown until probed |
| T1, a round outline, no limbs past the edge | `shadeSize` ψ and `apicalControl` λ on `LIMB`/`BOUGH`; limb lifespan or a limb-to-bough carry-on (as the birch's round 7 did); `dominance` | **Probably values.** The engine draws no crown envelope: roundness has to come from the limbs' growth law. If equal limbs still leave a ragged outline, the trait is not reachable with values, and that is the host's |
| A tuft above the dome | `LEADER` lifespan and vigour | **Values**, expected |
| DBH in range | `pipe`, `exponent`, `retained`, `leafGirth` | **Values**; outside the spec's named traits, but its stem is what the owner sees as "short trunk". Moved only if the crown rounds leave it out of range, and named when moved |

Light (`/space/light`) stays the site's (extinction 0.5, sky 0.5): fn-226's verdict. Other species and the leaf, bark and material rows are not touched.

**A dependency to name before the rounds:** `the_ordinary_tree_is_the_centroid_of_the_branching_species` (`tests/space_presets.rs`) computes the ordinary preset as the centroid of the beech, the oak and the spruce. Any move of the oak's `/space` rows moves that centroid, so the ordinary tree's value file either moves with the oak (the ordinary tree then changes, against R6's "other species byte-identical") or the test fails. This is the host's call; see the round reports.

## Round 1 (2026-10-09)

Leader 6 years, limb internode 0.035 m, ψ 0.5 on limbs and boughs, boughs ascending at 0.9 rad, no retained pipe, more short shoots with a thinner pipe. (round 1's sheet, archived; decisions in the spec). It reaches the sizes, the stem and the tuft; it does not reach T1's closed, round crown, which stops for the host.

## The leaves (host decision after round 1: the leaf rows join the brief)

| # | Trait | Quote | Source |
|---|---|---|---|
| L1 | **Blade size** | "Leaf blade obovate, elliptic or subrotund, moderately to deeply lobed, 25-120(-140) × 15-85 mm"; "petiole 4-10 mm" | Flora of North America, *Quercus garryana* (`raw/lit/fna-garryana.md`) |
| L2 | **Blade size, a second account** | "Leaves alternate, simple, 5-15 cm long, dark green, round lobed"; OSU-OAK-ID: "Blade width 2-5 inches" | OSU, OSU-OAK-ID (catalogue extracts) |
| L3 | **Buds, and so shoots and leaves, crowd at the tip of the year's shoot** | fn-195 rule 5 (BC07): "Acrotony … is the prevalent development", the distal laterals the most vigorous; fn-195 `SOURCES.md`: "S4 shows laterals bunched at the shoot tip" | *Q. robur* and oaks in general; for *Q. garryana* no source read states leaf crowding, so a tip cluster's leaf count is **estimated** |
| L4 | **A dense crown on open-grown trees** | SILV: "Branch development on open-grown trees may be very dense"; "dense, rounded crowns" | T1 above |

Today's leaf rows (preset, round 1): blade 100 × 75 mm (`/element/length`, `/width`), inside L1 and L2; one leaf every 8 mm of grown shoot (`/skeleton/twigs/twig/internodeLength`, the fn-193-to-196 still rows, **estimated**); leaves on wood under 6 % of the root's radius (`/canopy/shootRadius`, **estimated**); size variation 0.2, forward lean 0.25, no outward or upward turn; no cluster on any age.

## Round 3 (2026-10-09)

On round 1's structure with L3's leaves (140 mm blades, FNA's upper bound, L1): twigs moved to the branch systems' medial nodes at a wider angle, twigs living 5 years at 0.98, the fine orders' light share at 1.0, the fork's limbs at 0.8. All **estimated**. At 80 the three seeds read as broad, closed crowns from four azimuths (width over height 1.09 to 1.12, projected foliage gaps 0.14 to 0.16, against 0.23 to 0.32 for round 1). (round 3's sheet, archived; result in the spec).
