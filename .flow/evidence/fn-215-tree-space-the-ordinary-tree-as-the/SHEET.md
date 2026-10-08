# fn-215: the ordinary tree's sheet, walks and cost

Raw stills are in `raw/` (gitignored). Commands, from the worktree root:

- Rows: `cargo run --profile ci -p telperion-core --example centroid` writes `presets/ordinary.values`' `/space` rows; a rerun leaves the file byte-identical.
- Sheet: `target/ci/examples/space_<name> <out> 40 80 --seeds 1,7,42 --shots bare,whole,crown`, for `ordinary`, `beech`, `oak` and `spruce`.
- Walks: `target/ci/examples/space_walk <out> ordinary <corner> <t> 1 40`, t = 0, 0.25, 0.5, 0.75, 1.

## How each row was averaged (R1)

Every rule below is the space's walk between two families (`blend::families`, fn-206), folded so each corner carries a weight of 1/3. None was invented here.

| Row | Rule | Source |
|---|---|---|
| Settings of an age | Each setting on its own scale: by value, by log above a floor, or by the share it keeps | `species/scale.rs`, host decision 9 |
| Stop outcomes | Mixed as shares of carrying on, relaying and dying | host decision 12 |
| An age only some corners live in | Their canonical neighbour stands in at lifespan 0, so the centroid lives in that age for a short span. Ages 0, 3, 5 and 13 come out at 0.44 to 0.59 cycles; every age 0 to 14 is lived | `Species::canonical`, scale `lifespan` Log floor 1 |
| A zone role one corner leaves out | A zone of no nodes | `blend.rs` `empty` |
| Node counts, `lateral/<j>`, `dormant/<j>` | By value, entry by entry; counts come out fractional (for example 2.33) | scale table |
| `/space/ages/<k>/trunk` | The union of the corners' stem ages: 0 to 5 | `core::blend::space` |
| Light | By value: extinction 1/6, sky 0.5 | `core::blend::space` |

`/age = 40` follows fn-198 host decision 5, the age the harness opens an engine preset at, and is the age every corner states. Every row outside `/space` is the default family's, which is the ordinary tree's dressing and material.

## Sheet (R2): viewed, host to judge

| Age | Seed | Ordinary | Beech | Oak | Spruce |
|---|---|---|---|---|---|
| 40 | 1 | 4,577 ms, 451,641 nodes, 12.5 m | 69 ms, 102,846 | 620 ms, 167,162 | 925 ms, 1,147,168 |
| 40 | 7 | 6,253 ms, 603,650 nodes, 13.0 m | 97 ms, 181,014 | 943 ms, 296,157 | 939 ms, 1,107,132 |
| 40 | 42 | 8,366 ms, 671,488 nodes, 12.5 m | 146 ms, 249,031 | 688 ms, 221,483 | 911 ms, 1,163,226 |
| 80 | 1, 7, 42 | **refused**: engine phytomer budget (20,000,000), after 86 to 288 s of growth | 0.7 to 1.1 s | 7 to 14 s | 3.5 s |

Grow time is `executor::grow`, measured once on the RTX 3080 machine, and does not include dressing. Per node, the ordinary tree grows about 10 to 12 µs a node, 3 to 4 times the oak's time and 12 to 21 times the beech's and the spruce's.

At 40 the ordinary tree stands 12.5 to 13 m tall, with an excurrent leader and many fine, short laterals. Its crown is a narrow cone 8.6 to 9.9 m wide, and its leaves are sparse: 45,000 to 67,000 per tree. The default family dresses it, so its bark is pinkish and its leaves the default bright green. The sheet has no age-80 tree.

## Walks (R3)

Seed 1, age 40, bare, on the beech's dressing, in the fixed 30 m frame. From the ordinary tree to the beech, the oak and the spruce, at quarter steps. Height moves by degree: 12.5 to 10.1 m toward the beech, 12.5 to 13.4 to 12.4 m toward the oak, and 12.5 to 11.7 m toward the spruce. The quarter-step strips show no visible pop at the frame's scale. The steps are coarse, and no numeric walk test starts at the ordinary tree: `walks.rs` walks only the corners.

## The predictor check for the host's size decision (2026-10-08)

The host decided to scale the centroid's lateral probabilities until `expected_kept` at 80 equals the corners' geometric mean. That decision assumes the ordinary tree reads no light. It does: `Species::reads_light` is true for any age whose `shade_hazard`, `shade_size`, `balance_hazard` or `leaf_girth` is not zero. The oak states all four at every lived age, at 1, 1.5, 2 and 1, so the centroid carries a third of each (balance hazard 0.049 on its log scale). For such a species, `expected_kept` returns none.

| Species | Expected kept at 40 | Expected kept at 80 | Time at 80 |
|---|---|---|---|
| European beech | 147,362 | 2,187,240 | 37 ms |
| Oregon white oak | none (reads light) | none | |
| Norway spruce | 1,149,893 | 4,653,767 | 62 ms |
| Ordinary (centroid) | none (reads light) | none | |
| Ordinary with those four rows set to 0 (probe only, not shipped) | 1,456,352 | 21,391,939 | |

The predictor can therefore reach neither the target, which needs the oak, nor the tree that would be scaled. Nothing was scaled, and R1 is not amended.

The friction fix, refusing up front through `foreseen`, would not have caught this tree either:

- `foreseen` (`pipeline/space.rs`) runs only in Wasm, where `KEPT` is set, and checks expected kept phytomers against 4,000,000.
- The engine's 20,000,000 budget counts grown phytomers, shed ones included.
- `settled`, the path `executor::grow` and the stills take, does not call `foreseen` at all.
- The ordinary tree has no expected count.
