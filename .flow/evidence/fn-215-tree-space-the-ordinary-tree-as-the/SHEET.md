# fn-215: the ordinary tree's sheet, walks and cost

Raw stills are in `raw/` (gitignored): the sized tree's in `raw/sheet2/` and `raw/walks2/`; the unsized centroid's and the corners' in `raw/sheet/` and `raw/walks/`. Commands, from the worktree root:

- Rows: `cargo run --profile ci -p telperion-core --example centroid` writes `presets/ordinary.values`' `/space` rows. A rerun leaves the file byte-identical.
- Sheet: `target/ci/examples/space_<name> <out> <age> --seeds 1,7,42 --shots bare,whole,crown`.
- Walks: `target/ci/examples/space_walk <out> ordinary <corner> <t> 1 <age>`.

## Decision: centroid of shape, mean of size (host, 2026-10-08)

The first rows, a pure equal-weight centroid, grew a plausible tree at 40. At 80, every seed outgrew the engine's 20,000,000-phytomer budget, after 86 to 288 s of growth (FRICTION.md). Averaging growth rates from differently built species compounds: the spruce's dense lateral production combined with the broadleaves' lifespans. The host decided:

1. Keep the equal-weight centroid of every row. Then multiply every age's lateral probabilities by one factor, so the tree's light-neutral expected kept phytomers at 80 equal the geometric mean of the corners'.
2. "Light-neutral" means `expected_kept` with the four light rows (`shade_hazard`, `shade_size`, `balance_hazard`, `leaf_girth`) at 0. The oak reads light, so the centroid does too, and a species that reads light has no expected count. Light only removes growth: fn-211's OAK-LIGHT.md has the lit oak keeping 58 to 84 % of the unlit oak's phytomers. So the light-neutral count bounds the real tree's size from above, and the shipped tree keeps the centroid's light rows.
3. The tool computes the factor, by 64 bisection steps, and a rerun reproduces the preset to the bit.
4. The dressing (bark and leaf colour) stays at the default family's, a later question for the owner.

| Light-neutral expected kept at 80 | Phytomers |
|---|---|
| European beech | 2,187,240 |
| Oregon white oak | 4,430,867 |
| Norway spruce | 4,653,767 |
| Target, their geometric mean | 3,559,562 |
| The unsized centroid | 21,391,939 |
| Lateral factor | 0.7419269039293579 |

## How each row was averaged (R1)

Every rule below is the space's walk between two families (`blend::families`, fn-206), folded so each corner weighs 1/3. None was invented here.

| Row | Rule | Source |
|---|---|---|
| Settings of an age | Each setting on its own scale: by value, by log above a floor, or by the share it keeps | `species/scale.rs`, host decision 9 |
| Stop outcomes | Mixed as shares of carrying on, relaying and dying | host decision 12 |
| An age only some corners live in | Their canonical neighbour stands in at lifespan 0, so the centroid lives in that age for a short span. Ages 0, 3, 5 and 13 come out at 0.44 to 0.59 cycles; every age 0 to 14 is lived | `Species::canonical`, scale `lifespan` Log floor 1 |
| A zone role one corner leaves out | A zone of no nodes | `blend.rs` `empty` |
| Node counts, `lateral/<j>`, `dormant/<j>` | By value, entry by entry; counts come out fractional (for example 1.67). The laterals are then multiplied by the factor above | scale table |
| `/space/ages/<k>/trunk` | The union of the corners' stem ages: 0 to 5 | `core::blend::space` |
| Light | By value: extinction 1/6, sky 0.5 | `core::blend::space` |

`/age = 40` follows fn-198 host decision 5, the age the harness opens an engine preset at, and is the age every corner states. Every row outside `/space` is the default family's.

## Size and cost of the sized tree (R1, R2)

Grown counts were bracketed by growing with a lower budget through `telperion_space::grow`, at the site's light. Times are measured once on the RTX 3080 machine.

| Age | Seed | Kept phytomers | Grown phytomers | Engine grow | Pipeline grow (still runner) | Nodes | Height, width |
|---|---|---|---|---|---|---|---|
| 40 | 1 | 246,416 | | 1.4 s | 1.2 s | 134,975 | 12.5 m, 8.3 x 7.8 m |
| 40 | 7 | 315,864 | | 1.8 s | 1.5 s | 188,518 | 13.0 m, 8.8 x 10.1 m |
| 40 | 42 | 365,277 | | 1.9 s | 1.6 s | 217,379 | 12.5 m, 10.0 x 8.7 m |
| 80 | 1 | 1,839,605 | 7.5M to 10M | 21.7 s | 22.2 s | 972,367 | 23.2 m, 14.5 x 15.4 m |
| 80 | 7 | 2,974,674 | 10M to 15M | 32.3 s | 33.6 s | 1,580,355 | 23.2 m, 14.9 x 15.7 m |
| 80 | 42 | 3,311,617 | 10M to 15M | 36.9 s | 42.9 s | 1,796,941 | 23.0 m, 15.9 x 14.7 m |

- Every seed at 80 builds, and keeps fewer phytomers than the 3.56M target.
- Grown phytomers are at most 15M, under the 20M budget by at least a quarter of it.
- At 80 the tree grows in 22 to 43 s. The corners take 0.7 to 1.1 s (beech), 7 to 14 s (oak) and 3.5 s (spruce) at the same age, so the ordinary tree is the slowest preset at 80.
- The TypeScript field test ("grows every shipped preset, byte for byte") passes in 41.4 s under its unchanged 60 s limit; it took 77 s on the unsized rows.

## Sheet (R2): viewed, host to judge

The composites are `raw/sheet2/ordinary-{bare,whole,crown}.png` (rows age 40 and 80; columns seeds 1, 7, 42). The corners' sheets are `raw/sheet/sheet-40-*.png` and `raw/sheet/sheet-80-*.png`.

- **At 40:** an excurrent leader with fine, open, level laterals, sparser than the unsized centroid.
- **At 80:** a narrow columnar crown 23 m tall and 15 m wide, with a band of dense fine wood and leaves low on the stem (about 2 to 5 m).
- **Dressing:** the default family's, so the bark is pinkish and the leaves bright green.

## Walks (R3)

Seed 1, bare, on the beech's dressing, in the fixed 30 m frame; t = 0, 0.25, 0.5, 0.75, 1.

**At 40** (`raw/walks2/strips-40.png`), all fifteen frames build. Height moves by degree:

- toward the beech: 12.5 to 10.1 m
- toward the oak: 12.5 to 13.3 m, then 12.4 m
- toward the spruce: 12.5 to 11.7 m

There is no visible pop at the frame's scale. No numeric walk test starts at the ordinary tree: `walks.rs` walks only the corners.

**At 80** (`raw/walks2/strips-80.png`), not every frame builds:

| Toward | 0.25 | 0.5 | 0.75 | 1 |
|---|---|---|---|---|
| Beech | 22.1 m | not finished in 110 s | not finished in 110 s | 17.8 m |
| Oak | 22.9 m | refused (budget) | refused (budget) | 20.2 m |
| Spruce | 22.9 m | 22.8 m | refused (budget) | 21.7 m |

Deferred to fn-230, "Points between species stay viable trees" (host, 2026-10-08; spec R3 amended). fn-230 owns the age-80 walks and re-derives the ordinary tree through one space-wide blend rule. This predates fn-215. Between the corners themselves at t = 0.5 and 80, the beech-to-oak walk builds (20.4 m), but the oak-to-spruce and beech-to-spruce walks are refused by the budget. A walk mixes the ends' lateral production with their lifespans as the unsized centroid did, so points inside the space can outgrow the budget where neither end does. Left to the host.

## Open for the owner

- **Build time at 80:** the ordinary tree grows in 22 to 43 s at 80 (seeds 1, 7, 42), the slowest preset at that age. The corners take 0.7 to 1.1 s (beech), 7 to 14 s (oak) and 3.5 s (spruce). At 40 it grows in 1.2 to 1.6 s.
- **Dressing:** the bark and leaf colour stay at the default family's.

## The early refusal (host: record, do not extend)

`foreseen` (`pipeline/space.rs`):

- runs only in Wasm, where `KEPT` is set;
- checks expected kept phytomers against 4,000,000, while the engine's 20,000,000 budget counts grown phytomers, shed ones included;
- is not called by `settled`, the path `executor::grow` and the stills take;
- has no expected count for a species that reads light, so it could not have refused the unsized centroid either.
