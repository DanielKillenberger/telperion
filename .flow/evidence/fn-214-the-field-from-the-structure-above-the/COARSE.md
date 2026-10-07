# fn-214 task 1: the coarse field probe

The owner accepted, on 2026-10-07, a probe of a field built from the structure above fn-209's cut. This file holds what it built, what it measured and what it leaves for the host.

## What was built

- **The option:** `growField(species, seed, { cut })` in TypeScript. The cut travels through the slim ABI's `grow(seed, limb_order, cut)` and `pipeline::Request.cut` into `GrowInput.cut`.
  - A JavaScript caller that passes two arguments gives the Wasm 0, so 0.1.4's wrapper still works.
  - At cut 0 (the default) the field is today's. Its answers match the branch's base byte for byte for the palm, the oak and the spruce (`raw/field-path/same.mjs`, n = 24).
- **The engine:** `telperion_space::grow_coarse` (`fine/coarse.rs`) grows the structure above the cut with `grow_fine`'s own driver (`expansion::settle_with`). It grows none of the fine roots. In their place, an executor reads each root's committed boundary row (`closed_form::boundary`, fn-209 option 4) at its base scale for every phase:
  - **living:** to the tree's age, with presence equal to its chance of being kept;
  - **radius:** E[r | kept] weighed by the kept chance to the power 1/e, so its section is E[S; kept];
  - **load:** mass and moment along its heading;
  - **bend:** nothing.
- **The field:** the pipeline's tree carries each live root as a `Stand`:
  - its plan descriptor is a capsule from its bud along its heading, to twice its expected wood's centre, spread by that centre's distance (`Descriptor.spread`, zero everywhere else);
  - its leaves are its expected kept phytomers times the plan's own leaves per leaf-bearing node on the tree above the cut;
  - its wood is a segment from its base radius tapering to nothing.
- **Refused by name:**
  - a cut on a species whose structure above the cut reads its fine roots as grown (`keeps`, or buds that read light: the oak);
  - a cut with wood, curve or leaves in the request.

### Representation, with numbers (spruce 40, cut 1,024, one core)

The spec offered two ways to stand in for a fine root: a volume from its envelope and kept leaf area, or its realized totals where they are cheaper. Realized totals are never cheaper: they need the root grown, which is the cost the cut removes. The boundary rows give the same quantities as expectations, read once per (PA, age).

| | seed 1234 | seed 5678 |
|---|--:|--:|
| Roots below the cut | 21,933 | 21,210 |
| Realized: shed / kept phytomers | 1,867 / 1,211,420 | 1,873 / 1,158,254 |
| Coarse: stands / expected phytomers | 21,933 / 1,279,727 (+5.6 %) | 21,210 / 1,231,580 (+6.3 %) |
| Above the cut, phytomers (coarse = grown) | 12,571 | 12,106 |
| Trunk base radius, grown / coarse | 0.184 / 0.192 m | 0.181 / 0.189 m |
| Stand base radius p10 / p50 / p90 | 0.8 / 2.3 / 3.2 mm | 1.0 / 2.4 / 3.0 mm |
| Stand wood centre p10 / p50 / p90 | 0.02 / 0.13 / 0.18 m | 0.02 / 0.13 / 0.18 m |

`cargo run --release -p telperion-space --example coarse`. The stands lie far below the site's cells (0.25 to 0.5 m) and its wood cutoff (2.5 to 5 cm), so their shape inside a cell does not show. Their positions and their leaves do show.

Three ways to read a root's living were tried before this one:
- the mean last-living time from the extinction curve;
- the curve's median;
- the curve's last living cycle.

Each shed 16,088 of 21,933 roots, against the realized 1,867. The curve stops at a PA's lifespan (PA 10's at 4 cycles) and does not follow the continuations that keep a spruce shoot alive. So each stand stands to the tree's age, weighed by its row's kept chance.

## Measurements (Wasm under Node, one thread, the site's request)

The site's request (FIELD-PATH.md): the family's default age (40), its limb order, the per-species dials, and n = 48 / 36 / 30 / 24. The full field is the branch at cut 0. Raw output is in `raw/field-path/` (gitignored): `sweep.jsonl` (three repeats, load 22 to 41) and `quiet*.jsonl`.

**The final pass, n = 48, five repeats at load 7.6 to 8.9** (`final.jsonl`, the module at `62c35fd7`; medians). The end-to-end time is grow, query and voxelize:

| Species, seed, build | grow ms | query ms | voxelize ms | end to end ms | Wasm memory MB | cubes (wood + foliage) |
|---|--:|--:|--:|--:|--:|--:|
| spruce 1234, 0.1.4 | 192 | 125 | 6 | 333 | 87.0 | 5,818 (250 + 5,568) |
| spruce 1234, full | 3,738 | 711 | 7 | 4,456 | 1,048.9 | 4,883 (421 + 4,462) |
| spruce 1234, **coarse** | 171 | 72 | 8 | **249** | **43.3** | 5,604 (515 + 5,089) |
| spruce 5678, 0.1.4 | 160 | 125 | 7 | 291 | 87.3 | 5,798 (255 + 5,543) |
| spruce 5678, full | 3,686 | 746 | 9 | 4,349 | 1,021.9 | 4,878 (433 + 4,445) |
| spruce 5678, **coarse** | 164 | 71 | 8 | **242** | **43.2** | 5,569 (507 + 5,062) |
| palm 1234, 0.1.4 / full / coarse | 15 / 20 / 18 | 32 / 36 / 36 | 5 / 8 / 9 | 53 / 65 / 62 | 8.7 / 9.0 / 8.9 | 3,719 / 5,364 / 5,364 |
| palm 5678, 0.1.4 / full / coarse | 15 / 20 / 19 | 30 / 37 / 38 | 5 / 9 / 11 | 51 / 64 / 66 | 8.7 / 9.0 / 8.9 | 3,448 / 5,409 / 5,409 |
| oak 1234, 0.1.4 / full / coarse | 415 / 3,504 / refused | 193 / 461 / – | | | 72.6 / 304.9 / – | 10,899 / 6,930 / – |
| oak 5678, 0.1.4 / full / coarse | 529 / 1,369 / refused | 284 / 192 / – | | | 115.6 / 123.8 / – | 10,723 / 3,650 / – |

The oak's rows come from the three-repeat sweep at load 22 to 41 (`sweep.jsonl`), so their times are high for every build.

**Grove grids, spruce 1234, cubes at n = 36 / 30 / 24:**
- 0.1.4: 2,660 / 1,629 / 901
- full: 2,466 / 1,594 / 945
- coarse: 2,674 / 1,689 / 965

The coarse spruce's query falls with n, as 0.1.4's does. The full field's fixed cost of about 0.7 s a query is gone, because the coarse tree has 12,000 nodes, not 1.2 million.

**Native, one core, n = 48** (`raw/native/native.jsonl`, load 15, heap peak by a counting allocator):

| spruce | grow ms | query ms | heap peak MB |
|---|--:|--:|--:|
| full | 3,883 / 4,247 | 761 / 960 | 1,001 / 979 |
| coarse | 227 / 247 | 60 / 59 | 41 / 41 |

**R3, the gate, for the spruce: it holds.** The coarse spruce runs in 242 to 249 ms end to end, against 0.1.4's 291 to 333 ms, in 43 MB against 87 MB. The palm is unchanged within noise. **For the oak it is not reached:** a cut is refused (see "The oak").

## Renders for the host

The site's portrait drawing (its worker's grow and its rasterizer's still frame, copied as call patterns into `raw/field-path/render.mjs`; the site itself was only read). Each image holds three panels, left to right: 0.1.4, the full field, then the coarse field at cut 1,024. All are 200 px, angle 0.6, shown at 2×.

- `raw/field-path/spruce-1234-n48.png`, `spruce-5678-n48.png`: the portrait and specimen card's n = 48.
- `raw/field-path/spruce-1234-n30.png`, `spruce-5678-n30.png`: a grove layer's n = 30.
- `raw/field-path/palm-1234-n48.png`, `palm-5678-n48.png`: the coarse palm is the full palm, cube for cube (it bears no lateral).

The worker's reading: the coarse spruce keeps the full field's silhouette, its tiers and its skirt. It has about 14 % more foliage cubes and 20 % more wood cubes than the full field at n = 48, and fewer than 0.1.4. The verdict on the look is the host's.

## The oak: refused, for the host

The oak's structure above any cut reads its fine roots as grown. Its buds read light, which its fine roots' leaves shade, and its girth keeps their shed pipes (`retained 0.5`, `leaf_girth 1.0`). A cut is therefore refused by name, and R3's gate cannot hold for the oak. Options, none built:

1. **The oak waits on fn-211's stepped executor**, which grows a lit, retaining tree's fine roots apart from the structure above them. A coarse oak then reads expectations in place of that executor's roots, as the spruce does here.
2. **A committed light field for the structure above the cut:** each fine root's expected leaf area spread over its extent, as fn-209 probe P2 (`alone/field.rs`) measured it. The oak's above-the-cut structure would then grow under expected shade. This is a new design (the shade its roots cast, the retained pipes' expectation), and the tree would move.
3. **The site keeps the full oak:** 0.6 to 1.6 s and 124 to 305 MB (FIELD-PATH.md), passing `cut` only for the spruce. The birch waits on fn-213.

## Cost

- **The slim module:** 829,238 bytes on the base (`6381e130`) and 844,244 on this branch (+1.8 %); gzip -9 303,289 against 309,361 (+2.0 %). The branch's module was already over its 632,580-byte ceiling before this task (fn-198 host decision 6).
- **The coarse spruce's stages,** native on one core at load about 15 (`raw/native/`): rows and predictor about 25 ms after the second commit (36 ms before it, plus 8 ms for the predictor); above the cut 33 to 45 ms; seeds 8 to 11 ms; shed and scale 8 ms; thicken and place 12 to 15 ms; levers and place 12 to 16 ms; then the plan and the field.
  - What is left to take: the GPU's chain inputs that `settle_with` builds for every seed (8 to 11 ms, none of it read by a coarse tree).
