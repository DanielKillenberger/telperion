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

## Task 2: the oak's coarse field, options for the host (2026-10-08, not built)

F4 at `8f451083` is merged (`dd7336f6`). It brings fn-211's stepped executor as the one grower (`grow` is `grow_roots` at N = 1,024), contract A, and the as-grown exchange. `grow_coarse` still refuses the oak, and that refusal is still right. Under the stepped executor every cycle of the lit oak is a synchronisation point (PHASES.md, "the schedule"). Its structure above the cut reads its fine roots in six places, and a coarse oak has to answer each one without growing the roots:

| Where | What a root hands its bearer in the stepped executor | Per |
|---|---|---|
| (i) balance | its subtree's light `Q`, upkeep and living; down: shed flag, kept share | cycle |
| (ii) demand | its subtree's lit presence; down: its incoming vigour | cycle |
| (iii) normalisers | per PA, Σp and Σp(v/p)^ψ over its growing buds | cycle |
| (iv) light | its leaves of the cycle, into the one lattice | cycle |
| re-lay | section, retained section, mass and moment at its node | every 10 cycles |
| settle | the exchange (leaf partials, then base radius as grown); then phases 2 to 4 | once |

The closed form's rows answer only the last two. They hold expected section, retained (pre-fade) section, mass, moment, count, spread and centre for any PA at any age m, so each holds for a root of age `cycle − birth` at any cycle. They are light-neutral, though, so on their own they give a root no `Q`, no lit presence, no normaliser moment and no response to shade. Phases (i) to (iv) need a model of the root that steps with the cycles.

That model exists. fn-211.4's coupling pass (`couple.rs` `Aggregate`, `grow/couple.rs`, `grow/couple/stand.rs`; TASK4-LIGHT.md, step 2) holds the oak's structure above the cut and represents every fine root as an aggregate advanced each cycle:
- under the light it reads, its kept share and each PA's unit size;
- in the balance, the allocation (its `(v/p)^ψ` entering the normalisers as its root's ratio) and the lattice (its last unit's expected leaves spread over a seven-point stencil of the row's extent);
- in the re-lays, standing with its row's section, retained share and load.

What made it an experiment there was its second half: growing each fine subtree from the records afterwards. Neither split held there (`Recorded` blows scales up; `PerLight` cuts the fine PAs' growth by 25 to 52 %). A coarse field never runs that half. The pass's own cost was 0.04 s for the oak at 40, seed 1, on all cores, against 0.33 s for the whole tree (TASK4-LIGHT.md cost table, at the load of the time).

### The settle and the exchange from the closed form

This question is the same for every option below. **The exchange's totals can come from the rows**, with nothing new in the closed form:
- **Leaf partials** `[Σpipe, Σpipe·light^χ]`: the total is the root's E[S; kept] at its base scale (the pipe model's base section, as phase 2 already reads it). The weighted sum is that total times a light the coarse tree holds for the root, raised to χ. Under options 1 and 2 that light is the aggregate's last read; under option 3 it is the tree's mean, which leaves the root neutral in the mean.
- **The base radius as grown** (the retained pipe its bearer keeps): the row's `grown1` and `grown2` (E[S; kept] before its own fade) through `committed_radius`'s form. This is the share task 1's `Expected` executor leaves out today.
- **Phases 2 to 4** stay as task 1 built them.

With the exchange answered, `Expected` implements `grown_partials` and `grown_radii` (a small change, not yet written). The refusal in `grow_coarse` then narrows to "a species that reads light", and option 1 or 2 lifts that.

### Options for phases (i) to (iv)

1. **Aggregates as the stepped executor's items (recommended).** An item is either a grower (today) or an `Aggregate` whose crossings are its expectations:
   - `Q`, upkeep and living, from its cohorts, wood and read light;
   - lit presence, from its living cohorts and read light;
   - its normaliser partials, from its cohorts and ratio;
   - its stencil leaves into the lattice;
   - its row's section and load in the re-lay's assembly.
   
   The coarse oak is then the stepped executor run with aggregate items, followed by the settle above on `Expected`, which turns each live aggregate into a `Stand` as the spruce's are. This keeps one grower and one wiring of phases (i) to (iv) (principle 1). The `Aggregate` transitions and `couple/stand.rs`'s rows are reused, not copied. The work is an item kind in `phases.rs`, `feedback.rs`'s shade and the relay's assembly, plus the exchange above.
   - **Moves:** the oak's structure above the cut, by however much the aggregates' expectations differ from the grown roots. This is not measured for the structure above alone. TASK4-LIGHT.md measured only the coupling buds' light against the laid tree: p50 0.053, p95 0.174 at 80, against today's own rough-vs-laid 0.026 and 0.095.
   - **Known weak point:** a root reads its light at its centroid, inside its own deposit (TASK4-LIGHT.md question 2). It reads 12 to 17 % low at p50, so its expected growth runs low and its balance sheds it sooner. Reading at the stencil's outer points without its own deposit is the cheap repair TASK4 names. I would build without it, measure, and add it only if the cubes or stills show a thin lower crown.
2. **The coupling pass as it stands** (`grow_coupled`'s first half, through `run_beside`'s hooks in the grower), with the same settle. This is the least new code, and it is the configuration TASK4 measured. But it is a second wiring of the four phases beside the stepped executor, entered by the field package's default call (task 3), so production would hold two growers for the structure above the cut. I would take it only as a quick measurement before option 1, never as what ships.
3. **Open loop: the rows alone, light-neutral.** The structure above the cut grows lit by its own leaves only. Its fine roots cast no shade, take no part in balance or demand, and enter the normalisers not at all. Then the settle above. It is cheapest and needs no model of a root, but the structure above grows in near-full light with no competition from 97 to 99.7 % of the tree's buds (PHASES.md), so its limbs come out too vigorous and nothing below the cut is ever shed by shade. I expect it to fail the look and list it as the floor.
4. **No coarse oak.** The oak stays full (task 1's option 3), and the gate's oak row is reported as a gap. On F4 the full oak is the stepped build. Natively, on one core at 40, the grower before it took 0.48 to 0.80 s (PROFILE.md), and the stepped executor costs 1.2 to 1.3 times that (PHASES.md). In Wasm, seed 1 took 1.35 s (FIELD-PATH.md, measured before F4). Neither figure was re-measured on this merge.

### Recommendation

Take **option 1**, with the exchange from the rows.
- **Order of work:** the exchange on `Expected` first, then aggregate items.
- **Measurement:** the oak 1234 and 5678 at the site's request, as task 1 did for the spruce: Wasm time and memory against 0.1.4 and the full field, the cubes, and the three-panel stills.
- **Before the gate:** one comparison of the structure above the cut, by lineage against the stepped oak's (kept phytomers by order, trunk base, height, crown extent).

Its cost should land well under 0.1.4's grow: the structure above is about 2 % of the phytomers, aggregates step a handful of cohorts each, and the fixed cost is the lattice's sweep (PROFILE.md: 28 ms of the oak's 475 ms natively on one core at 40). That is an estimate, not a measurement.

## Host decision on task 2 (2026-10-08)

The light model stays today's lattice (fn-226's verdict: outline light rejected). So the coarse oak takes **option 1**: the fine roots stand in as aggregates, stepped inside the one grower each cycle (fn-211.4's `Aggregate`, the per-root expectation model fn-226 also chose), with the as-grown exchange taken from the closed form's expectations.

The known weak point is that a root reads its light inside its own leaves, reading 12–17 % low (TASK4-LIGHT.md). Measure it first. Add TASK4's repair only if the coarse oak's structure above the cut, its field cubes or its site timing miss.

Option 2 (a second grower) is out. Options 3 and 4 are kept only as the floor and the report.

## Task 2: the coarse oak, built under option 1 and measured (2026-10-08)

F4 at `e292c435` is merged (`9cb352ae`). The coarse oak is built as the host decided, without TASK4's light repair. The look does not hold, and seed 1234 misses 0.1.4's time. The task stops here for the host.

### What was built

- **Stands in the stepped executor** (`grow/stepped/stands.rs`, `standing.rs`). A species whose buds read light grows its structure above the cut in the stepped executor. Each fine root the cut hands over at birth keeps its placeholder there and stands as an `Aggregate`, with no grower of its own:
  - (i) up, its light, upkeep and living; down, the coarse grower's shed decision and kept share at the placeholder;
  - (ii) its lit presence at the placeholder, and its vigour per presence from what the placeholder is given;
  - (iii) its per-PA normaliser partials, merged with the coarse grower's;
  - (iv) its last unit's leaves on the seven-point stencil of its row's extent, in the one lattice, and the light read at its centroid.
- **The settle** (`fine/coarse.rs`, `fine/coarse/expected.rs`). `settle_grown` runs on the `Expected` executor, which now answers the exchange:
  - leaf partials: its pipes at its base scale as grown, by the light it last read raised to χ;
  - base radius as grown: its row's `grown1`/`grown2` through `committed_radius`'s form, by its leaf factor under the tree's mean.
- **Refusals.** A species that keeps its tree as grown but whose buds read no light is still refused by name. The spruce and palm path is unchanged.
- **One departure from the decision text.** The exchange's pipe weight is the aggregate's own pipe sum (a new `Aggregate::pipes`: Σ nodes × PA pipe × size), not E[S; kept]. The leaf mean is Σpipe / Σpipe·light^χ, where each pipe is a phytomer's own pipe radius. A section (radius^e) is a different unit: at a 3 mm root it is about 10^-6 against a pipe sum near 0.1, and it would drop the roots out of the mean. (Inferred from `girth/disused.rs::leaf_partials`; the two were not run against each other.)
- **Speed.** `Aggregate::advance` now reads each PA's survival under shade once per cycle, not once per cohort. The stands' unit sizes are computed only for the PAs their apexes live in. Both give the same values; the stands' advance fell from 53 to 28 ms natively on one core.

### The structure above the cut, against the stepped oak (age 40, cut 1,024, by lineage)

`raw/native/compare.jsonl`, `src/bin/compare.rs` (the family's own species and light):

| | seed 1234 stepped / coarse | seed 5678 stepped / coarse |
|---|--:|--:|
| Axes above the cut, shared by lineage | 193 / 193 | 94 / 94 |
| Kept phytomers above, orders 0 / 1 / 2 / 3 | 278 / 4,127 / 2,624 / 159 against 278 / 4,897 / 2,624 / 159 | 284 / 2,637 / 373 against 284 / 2,993 / 373 |
| Height of the structure above (m) | 12.71 / 12.72 | 12.96 / 13.01 |
| Crown extent of the structure above (m) | 6.43 / 6.42 | 4.63 / 4.64 |
| **Trunk base radius (m)** | **0.559 / 0.309** | **0.330 / 0.182** |
| Live roots | 1,460 / 1,596 | 352 / 415 |
| Phytomers below the cut, realized / expected | 404,688 / 587,025 | 118,358 / 134,292 |
| **Extent of the whole tree / the stands' reach (m)** | **7.85 / 6.26** | **7.19 / 4.65** |

The structure above grows as the stepped oak's does: same axes, height and extent. The coarse tree's order-1 surplus (770 and 356 phytomers) sits on axes outside the shared ones; its cause was not looked into. The miss is in the roots:

- Matched by lineage, the stands' total section (r^2.5) is 0.12 of the realized roots'. At cut 4,096 and 16,384 it is 0.12 and 0.11, and the trunk 0.336 and 0.332 m.
- The thickest realized roots were predicted at 357 to 661 kept phytomers. They kept 952 to 2,750 and stand at 0.16 to 0.22 m base radius; their stands are 0.02 to 0.03 m. The closed form's rows carry neither retained pipes nor light (`closed_form` reads neither), and the `grown` moments give the same trunk (0.311 m against 0.309).
- The fine roots reach 1.6 to 2.5 m further out than the stands' expected wood.

So under light the cut's light-neutral count puts some of the oak's limbs below the cut, and their rows stand for a small, thin subtree. (Measured: the sizes and reach above. Inferred: that these are the lower spreading limbs.)

### The site's request in Wasm (n = 48, three repeats, load 4.8 to 6.7)

`raw/field-path/oak.jsonl`, `oak2.jsonl` (medians; end to end is grow, query and voxelize; the module is `field-oak.wasm`, 1,265,238 bytes stripped):

| Oak, seed, build | grow ms | query ms | voxelize ms | end to end ms | Wasm memory MB | cubes n = 48 (wood + foliage) |
|---|--:|--:|--:|--:|--:|--:|
| 1234, 0.1.4 | 175 to 201 | 149 to 151 | 25 to 26 | 350 to 377 | 72.6 | 10,899 (634 + 10,265) |
| 1234, full | 5,801 | 260 | 14 | 6,076 | 437.8 | 6,428 (3,776 + 2,652) |
| 1234, **coarse** | 353 | 51 | 17 | **421** | **10.8** | 6,988 (1,981 + 5,007) |
| 5678, 0.1.4 | 201 to 232 | 166 to 169 | 24 to 25 | 392 to 425 | 115.6 | 10,723 (649 + 10,074) |
| 5678, full | 2,064 | 106 | 9 | 2,179 | 157.7 | 3,730 (2,028 + 1,702) |
| 5678, **coarse** | 220 | 33 | 14 | **267** | **10.2** | 5,216 (842 + 4,374) |

**R3 for the oak: not met.**
- Memory holds.
- Seed 5678 holds in time.
- Seed 1234 runs 421 ms against 0.1.4's 350 to 377 ms.

Natively on one core (load about 6), seed 1234's coarse build is rows 62 to 65 ms, the stepped run with its stands 109 to 147 ms, and the settle 22 to 27 ms. Per cycle, the coarse grower's allotment, the lattice and the stands' advance each take 28 to 34 ms in all. The stands' own sums take 3 ms of the allotment.

### Renders for the host

`raw/field-path/oak-1234-n48.png`, `oak-5678-n48.png`, `oak-1234-n30.png`, `oak-5678-n30.png`: 0.1.4, the full field and the coarse field at cut 1,024, left to right, drawn as task 1's stills are. The worker viewed all four. Its reading, not a verdict:
- The coarse oak is a narrow, upright crown, densely leaved.
- The full field's spreading lower limbs and its wide, wood-heavy crown are gone.
- The trunk shows about half as wide.
- The coarse oak holds about half the full field's wood cubes and nearly twice its foliage cubes.

### For the host

- Option 1 grows the structure above the cut faithfully and fits the memory.
- The roots' expectations are the closed form's light-neutral, retention-free rows, so they miss the size, girth and reach of the roots that light lets grow past their prediction. TASK4's light repair moves where a root reads its light, not what its row stands for. Whether it would close any of this gap is not measured.
- The time gap on seed 1234 is 44 to 71 ms. Nothing measured here closes it.
- Task 3 depends on this task and was not started.

## Task 2: the cut sweep (2026-10-09)

The host's hypothesis: the cut follows the light-neutral prediction, so the oak's lower spreading limbs fall below a cut of 1,024 and stand in as small subtrees, and a lower cut grows them exactly. This section measures the lit oak's coarse field as built at `433c4c86` (option 1, no light repair) at cuts 1,024, 512, 256, 128 and 64. No modelling changed.

- **Wasm:** `raw/field-path/cut-sweep.sh` → `cut-sweep.jsonl`. The site's request is age 40, dials 0.08 / tuft 0.4, and n = 48, 36, 30 and 24. Each run is three repeats, one Node process per tree, at load 4.0 to 5.8. The module is `field-sweep.wasm`, rebuilt from HEAD. It has the same size as `field-oak.wasm` and differs in one byte region. The cube counts at cut 1,024 match the earlier run.
- **Native:** `raw/native/compare` → `raw/native/cut-sweep.jsonl` and `.err`, on 4 threads.
- **Timing method:** the same as the earlier sections, Wasm under Node with no browser. End to end is grow, query and voxelize, given as the median (min..max).

### Time, memory and cubes (Wasm)

0.1.4 in this sweep: seed 1234 ran 365 (344..479) ms at n = 48 and 302 (282..412) at n = 36, in 72.6 MB. Seed 5678 ran 399 (388..440) and 332 (323..370), in 115.6 MB. The site's reference is 350 to 425 ms and 73 to 116 MB.

| Oak, seed, cut | grow ms | end to end n = 48 | end to end n = 36 | memory MB | cubes n = 48 (wood + foliage) | cubes n = 36 |
|---|--:|--:|--:|--:|--:|--:|
| 1234, full (earlier run) | 5,801 | 6,076 | | 437.8 | 3,776 + 2,652 | 1,904 + 1,343 |
| 1234, 1024 | 346 | **410** (384..434) | 377 (351..398) | 10.8 | 1,981 + 5,007 | 967 + 2,219 |
| 1234, 512 | 1,246 | 1,358 (1,334..1,472) | 1,312 | 54.4 | 2,295 + 4,870 | 1,082 + 2,210 |
| 1234, 256 | 1,896 | 2,028 (2,020..2,272) | 1,986 | 96.1 | 2,440 + 2,758 | 1,122 + 1,460 |
| 1234, 128 | 2,099 | 2,251 (2,030..2,398) | 2,207 | 96.1 | 2,462 + 2,780 | 1,149 + 1,466 |
| 1234, 64 | 2,102 | 2,242 (2,059..2,516) | 2,205 | 96.1 | 2,474 + 2,787 | 1,153 + 1,466 |
| 5678, full (earlier run) | 2,064 | 2,179 | | 157.7 | 2,028 + 1,702 | 1,038 + 925 |
| 5678, 1024 | 205 | **250** (249..298) | 226 (224..264) | 10.2 | 842 + 4,374 | 378 + 1,955 |
| 5678, 512 | 387 | 441 (428..537) | 415 (403..509) | 13.9 | 819 + 3,882 | 384 + 1,758 |
| 5678, 256 | 673 | 740 (696..798) | 709 (673..768) | 27.3 | 1,387 + 2,018 | 582 + 1,122 |
| 5678, 128 | 708 | 772 (692..872) | 743 (667..832) | 31.4 | 1,378 + 2,057 | 588 + 1,136 |
| 5678, 64 | 718 | 780 (692..839) | 756 (668..813) | 31.4 | 1,382 + 2,084 | 596 + 1,152 |

### Against the stepped full oak (native, age 40)

The stepped tree's own values do not move with the cut. Its trunk base is 0.559 m for seed 1234 and 0.330 m for seed 5678. Its whole extent is 7.85 m and 7.19 m.

| Seed, cut | trunk base, coarse (m) | stands / realized roots, section r^2.5 by lineage | stands' reach / fine roots' extent (m) | structure above: extent stepped / coarse (m) | thickest realized root below the cut: r m, kept / predicted | native coarse ms |
|---|--:|--:|--:|--:|--:|--:|
| 1234, 1024 | 0.309 | 0.12 | 6.26 / 7.85 | 6.43 / 6.42 | 0.217, 1,695 / 555 | 160 |
| 1234, 512 | 0.304 | 0.12 | 6.32 / 7.85 | 7.25 / 6.60 | 0.174, 2,750 / 401 | 511 |
| 1234, 256 | 0.329 | 0.19 | 7.66 / 7.85 | 7.74 / 7.96 | 0.087, 254 / 178 | 704 |
| 1234, 128 | 0.331 | 0.19 | 7.74 / 7.71 | 7.85 / 8.09 | 0.063, 97 / 60 | 696 |
| 1234, 64 | 0.333 | 0.18 | 7.75 / 7.71 | 7.85 / 8.12 | 0.063, 97 / 60 | 715 |
| 5678, 1024 | 0.182 | 0.19 | 4.65 / 7.19 | 4.63 / 4.64 | 0.146, 5,165 / 476 | 123 |
| 5678, 512 | 0.180 | 0.15 | 4.71 / 7.19 | 4.63 / 4.69 | 0.146, 5,165 / 476 | 177 |
| 5678, 256 | 0.203 | 0.52 | 6.72 / 7.09 | 7.19 / 6.85 | 0.049, 320 / 178 | 255 |
| 5678, 128 | 0.204 | 0.52 | 6.83 / 7.07 | 7.19 / 7.00 | 0.036, 149 / 101 | 280 |
| 5678, 64 | 0.206 | 0.53 | 6.84 / 7.07 | 7.19 / 7.02 | 0.023, 61 / 48 | 279 |

- At every cut, the axes above the cut match by lineage, with 2 to 13 axes unshared. The height of the structure above holds at 12.70 to 12.72 m (seed 1234) and 12.96 to 13.02 m (seed 5678).
- The coarse tree's surplus in order 1 stays at 623 to 829 phytomers for seed 1234 and 244 to 356 for seed 5678. From cut 256 down, the deeper orders also carry a surplus for seed 1234, at 0 to 1,000 per order.
- Below the cut, the expected phytomers stay above the realized ones at every cut: 502,577 against 313,756 for seed 1234 at cut 64, and 107,869 against 93,609 for seed 5678.

### Stills

`raw/field-path/oak-{1234,5678}-n{48,36}-cuts.png` hold seven panels, left to right: 0.1.4, full, then coarse at 1,024, 512, 256, 128 and 64. They are drawn as task 1's stills were. The worker viewed all four. What follows is its reading, not a verdict:
- At 1,024 and 512, the coarse oak is the narrow, upright, densely leaved crown task 2 reported.
- From 256 down, the long lower limbs of the full field appear: seed 1234's limb running out to the right, and the low spread of seed 5678. The silhouette and the wood pattern come close to the full field's, and the foliage cubes fall to about the full field's count.
- 128 and 64 look the same as 256.
- At every cut, the trunk shows narrower than the full field's.
- Every coarse and full panel is far from 0.1.4's round, solid crown. That is the F4 oak itself, not the cut.

### The silver birch

The silver birch is refused at every cut and seed, including cut 0. The error is `Field core: invalid input: no engine species`, returned in 2 ms (`raw/field-path/birch-sweep.jsonl`). The birch has no `/space` row on this branch, as FIELD-PATH.md found before. 0.1.4's birch runs: seed 1234 grows in 264 ms in 59.7 MB, and seed 5678 in 269 ms in 50.6 MB.

### Reading, for the host

- **The reach and the limbs:** the hypothesis holds. At cut 256 or below, the roots realized past their prediction grow above the cut, and the reach closes to within 0.2 to 0.5 m.
  - The stills gain the lower limbs.
  - The section ratio rises from 0.12 to 0.19 for seed 1234 and from 0.19 to 0.52 for seed 5678.
  - Nothing changes further from 256 to 64.
- **The trunk:** it does not close. At cut 64 it is 0.333 m against 0.559 (seed 1234) and 0.206 against 0.330 (seed 5678), still about 60 % of the stepped radius. Its cause was not looked into. The thickest root left below the cut is 0.02 to 0.06 m, so the remaining gap is not in limbs below the cut (inferred).
- **The time:** it does not hold below 1,024.
  - Seed 1234 takes 1.3 s at 512 and 2.0 to 2.3 s from 256 down, against 0.1.4's 350 to 425 ms. Seed 5678 takes 0.44 s at 512 and 0.74 to 0.78 s from 256 down.
  - n = 36 saves only 25 to 45 ms, because the grow dominates.
- **The memory:** it holds at every cut. The peak is 96 MB at most, at or under 0.1.4's 73 to 116 MB.
- No cut tested gives both the limbs and 0.1.4's time.

## Task 2: the sweep on the release's oak and birch (2026-10-09)

F4 at `aab5f591` (oak round 3, fn-236; the engine silver birch, fn-213 round 14) is merged as `8777bd72`.
- **Conflicts:** three files, `input.rs`, `space.rs` and `tree.rs`, resolved by keeping both sides:
  - `GrowInput` carries the cut and the supernatural group. It is `Clone`, no longer `Copy`.
  - The coarse tree is scaled by the group's scale, its stands included.
  - `Tree` keeps `stands` beside `clusters`.
  - The cluster descriptor takes `spread: 0`.
- **Digests:** `catalogue_identity` and `parameter_reference` pass unchanged, so no digest moved.
- **Generated files:** `npm run wasm:build` reports the family metadata current and changes no generated file.

### How this was measured

The method is the same as the sections above: Wasm under Node with no browser, the family's default age of 40, and the site's dials (oak 0.08 / tuft 0.4, birch 0.08 / tuft 0.32).

- **Repeats:** three, one Node process per tree, at load 1.9 to 3.2.
- **Builds:** each tree ran as 0.1.4, as the full field (cut 0) and as the coarse field at each cut.
- **End to end:** grow, query and voxelize, given as the median (min..max).
- **Files:** `raw/field-path/release-sweep.sh` → `release-sweep.jsonl` (the module is `field-release.wasm`, built at `8777bd72`, 1,341,072 bytes stripped).
- **Structure against the stepped tree:** `raw/native/compare` → `raw/native/release.jsonl` and `.err`, natively on 4 threads. Each row compares against the stepped (full) tree at the same cut. The stepped tree's realized fine roots shift slightly at cut 64, because the plan's cut partitions them.
- **Stills:** `raw/field-path/release/{oak,birch}-<seed>-n{48,36}.png`. Each holds seven panels, left to right: 0.1.4, full, then coarse at 1,024, 512, 256, 128 and 64.

### Time, memory and cubes (Wasm)

| Tree, seed | build | end to end n = 48 ms | n = 36 ms | memory MB | field height m | cubes n = 48 (wood + foliage) | cubes n = 36 |
|---|---|--:|--:|--:|--:|--:|--:|
| oak 1234 | 0.1.4 | 333 (332..336) | 270 | 72.6 | 24.51 | 634 + 10,265 | 372 + 4,743 |
| oak 1234 | full | 9,814 (9,680..10,075) | 9,648 | 782.3 | 11.08 | 4,116 + 4,526 | 2,099 + 2,097 |
| oak 1234 | cut 1,024 | 1,544 (1,463..1,548) | 1,467 | 80.3 | 11.01 | 3,595 + 7,126 | 1,750 + 3,148 |
| oak 1234 | cut 512 | 2,404 (2,340..2,493) | 2,327 | 120.1 | 11.03 | 3,193 + 3,968 | 1,566 + 1,879 |
| oak 1234 | cut 256 | 2,501 (2,407..2,513) | 2,420 | 131.9 | 11.04 | 3,293 + 4,022 | 1,610 + 1,908 |
| oak 1234 | cut 128 | 2,580 (2,521..2,669) | 2,497 | 144.1 | 11.04 | 3,416 + 4,104 | 1,683 + 1,936 |
| oak 1234 | cut 64 | 5,735 (5,665..5,763) | 5,589 | 377.8 | 11.06 | 3,753 + 4,482 | 1,859 + 2,099 |
| oak 5678 | 0.1.4 | 378 (366..396) | 316 | 115.6 | 24.09 | 649 + 10,074 | 341 + 4,618 |
| oak 5678 | full | 7,713 (7,462..8,197) | 7,561 | 648.4 | 11 | 3,930 + 5,390 | 2,051 + 2,538 |
| oak 5678 | cut 1,024 | 1,138 (1,090..1,158) | 1,077 | 49.6 | 10.97 | 2,775 + 7,156 | 1,327 + 3,153 |
| oak 5678 | cut 512 | 1,777 (1,727..1,805) | 1,703 | 91.8 | 11.03 | 3,526 + 5,354 | 1,743 + 2,527 |
| oak 5678 | cut 256 | 1,870 (1,863..1,936) | 1,786 | 91.8 | 11.03 | 3,541 + 5,369 | 1,749 + 2,535 |
| oak 5678 | cut 128 | 1,984 (1,883..2,055) | 1,898 | 91.8 | 11.03 | 3,435 + 5,084 | 1,693 + 2,422 |
| oak 5678 | cut 64 | 4,090 (3,966..4,241) | 3,965 | 197.6 | 11.03 | 3,763 + 5,379 | 1,930 + 2,527 |
| oak 6137 | 0.1.4 | 386 (369..393) | 319 | 107.8 | 24.47 | 662 + 10,628 | 360 + 4,856 |
| oak 6137 | full | 7,642 (7,556..7,927) | 7,491 | 624.3 | 11.05 | 4,441 + 5,634 | 2,286 + 2,601 |
| oak 6137 | cut 1,024 | 1,057 (1,020..1,063) | 996 | 48.3 | 11 | 2,917 + 7,247 | 1,398 + 3,184 |
| oak 6137 | cut 512 | 1,720 (1,649..1,752) | 1,648 | 92 | 11.2 | 3,591 + 5,044 | 1,758 + 2,396 |
| oak 6137 | cut 256 | 1,796 (1,765..1,815) | 1,719 | 92 | 11.2 | 3,650 + 5,097 | 1,783 + 2,422 |
| oak 6137 | cut 128 | 1,898 (1,795..1,917) | 1,813 | 92 | 11.2 | 3,747 + 5,184 | 1,808 + 2,464 |
| oak 6137 | cut 64 | 3,832 (3,678..3,846) | 3,686 | 186.3 | 11.05 | 4,069 + 5,581 | 2,116 + 2,564 |
| oak 7432 | 0.1.4 | 356 (356..358) | 291 | 100.6 | 23.99 | 667 + 10,504 | 361 + 4,832 |
| oak 7432 | full | 8,641 (8,483..8,791) | 8,465 | 720.5 | 11.11 | 4,518 + 5,541 | 2,327 + 2,541 |
| oak 7432 | cut 1,024 | 1,292 (1,272..1,305) | 1,215 | 54.8 | 11.12 | 3,406 + 7,799 | 1,621 + 3,418 |
| oak 7432 | cut 512 | 2,046 (2,007..2,122) | 1,960 | 101 | 11.18 | 3,848 + 5,268 | 1,888 + 2,503 |
| oak 7432 | cut 256 | 2,207 (2,102..2,225) | 2,113 | 121.7 | 11.18 | 3,863 + 5,349 | 1,913 + 2,531 |
| oak 7432 | cut 128 | 2,346 (2,277..2,348) | 2,245 | 132.9 | 11.18 | 3,985 + 5,426 | 1,948 + 2,572 |
| oak 7432 | cut 64 | 4,920 (4,823..4,987) | 4,763 | 286.8 | 11.08 | 4,059 + 5,565 | 2,126 + 2,559 |
| oak 7737 | 0.1.4 | 376 (366..422) | 308 | 115.4 | 24.56 | 663 + 10,245 | 362 + 4,686 |
| oak 7737 | full | 7,522 (7,355..7,736) | 7,358 | 623.7 | 11.09 | 3,587 + 4,440 | 1,784 + 2,022 |
| oak 7737 | cut 1,024 | 1,060 (1,041..1,062) | 994 | 50.3 | 11.14 | 2,884 + 6,166 | 1,376 + 2,727 |
| oak 7737 | cut 512 | 1,740 (1,657..1,756) | 1,662 | 98.6 | 11.15 | 3,221 + 4,452 | 1,562 + 2,099 |
| oak 7737 | cut 256 | 1,809 (1,773..1,819) | 1,728 | 98.6 | 11.15 | 3,226 + 4,470 | 1,562 + 2,105 |
| oak 7737 | cut 128 | 1,925 (1,832..1,931) | 1,841 | 98.6 | 11.15 | 3,239 + 4,435 | 1,582 + 2,076 |
| oak 7737 | cut 64 | 3,978 (3,860..4,091) | 3,854 | 215.3 | 11.16 | 3,509 + 4,639 | 1,740 + 2,154 |
| oak 7919 | 0.1.4 | 385 (378..388) | 318 | 107.4 | 24.52 | 596 + 10,386 | 321 + 4,761 |
| oak 7919 | full | 9,299 (9,274..9,314) | 9,103 | 747.3 | 11.38 | 5,356 + 5,116 | 2,596 + 2,306 |
| oak 7919 | cut 1,024 | 1,548 (1,489..1,710) | 1,466 | 82.6 | 11.34 | 3,659 + 6,875 | 1,769 + 3,037 |
| oak 7919 | cut 512 | 2,402 (2,299..2,500) | 2,300 | 112.3 | 11.35 | 4,501 + 4,812 | 2,224 + 2,199 |
| oak 7919 | cut 256 | 2,529 (2,504..2,703) | 2,417 | 112.3 | 11.35 | 4,547 + 4,876 | 2,252 + 2,226 |
| oak 7919 | cut 128 | 2,693 (2,680..2,722) | 2,584 | 136.3 | 11.35 | 4,693 + 4,972 | 2,328 + 2,264 |
| oak 7919 | cut 64 | 5,561 (5,373..6,067) | 5,379 | 291.4 | 11.38 | 4,969 + 5,212 | 2,406 + 2,382 |
| birch 1234 | 0.1.4 | 410 (409..412) | 348 | 59.7 | 16.91 | 712 + 7,333 | 289 + 3,312 |
| birch 1234 | full | 2,516 (2,478..2,523) | 2,493 | 315.4 | 17.44 | 334 + 2,111 | 155 + 1,061 |
| birch 1234 | cut 1,024 | 1,067 (1,052..1,120) | 945 | 18.8 | 19.72 | 234 + 7,519 | 120 + 3,204 |
| birch 1234 | cut 512 | 1,146 (1,101..1,160) | 999 | 28 | 19.71 | 232 + 8,623 | 120 + 3,667 |
| birch 1234 | cut 256 | 1,174 (1,152..1,177) | 1,026 | 28 | 21.47 | 196 + 8,029 | 104 + 3,424 |
| birch 1234 | cut 128 | 1,938 (1,905..1,945) | 1,768 | 109.9 | 23.55 | 168 + 7,792 | 85 + 3,332 |
| birch 1234 | cut 64 | 2,391 (2,340..2,415) | 2,232 | 149.4 | 26.06 | 130 + 7,421 | 65 + 3,188 |
| birch 5678 | 0.1.4 | 400 (392..400) | 345 | 50.6 | 16.99 | 714 + 6,812 | 317 + 3,126 |
| birch 5678 | full | 1,500 (1,445..1,522) | 1,476 | 158.4 | 16.92 | 258 + 1,539 | 145 + 779 |
| birch 5678 | cut 1,024 | 875 (862..904) | 810 | 14.5 | 19.39 | 203 + 7,189 | 107 + 3,072 |
| birch 5678 | cut 512 | 933 (927..938) | 851 | 15.3 | 19.38 | 197 + 8,048 | 103 + 3,430 |
| birch 5678 | cut 256 | 911 (904..934) | 832 | 17.1 | 20.77 | 174 + 7,654 | 82 + 3,275 |
| birch 5678 | cut 128 | 1,364 (1,309..1,376) | 1,268 | 55.9 | 23.02 | 143 + 7,417 | 69 + 3,165 |
| birch 5678 | cut 64 | 1,603 (1,598..1,623) | 1,507 | 77 | 26.01 | 113 + 7,730 | 59 + 3,300 |

The field height is the field's bounds, which take in the stands' expected wood.

### Against the stepped tree (native, age 40)

| Tree, seed, cut | trunk base stepped / coarse m | stands / realized section r^2.5 | stands' reach / fine roots' extent m | structure above, extent stepped / coarse m | thickest realized root below the cut: r m (kept / predicted) |
|---|--:|--:|--:|--:|--:|
| oak 1234, 1,024 | 0.428 / 0.363 | 0.13 | 5.79 / 6.63 | 6.88 / 6.26 | 0.129 (4,591 / 965) |
| oak 1234, 512 | 0.428 / 0.374 | 0.16 | 6.42 / 6.63 | 6.88 / 6.93 | 0.086 (224 / 241) |
| oak 1234, 256 | 0.428 / 0.377 | 0.17 | 6.43 / 6.63 | 6.88 / 6.93 | 0.086 (224 / 241) |
| oak 1234, 128 | 0.428 / 0.381 | 0.17 | 6.43 / 6.63 | 6.88 / 6.93 | 0.052 (169 / 117) |
| oak 1234, 64 | 0.428 / 0.410 | 0.18 | 6.44 / 6.54 | 6.88 / 6.82 | 0.038 (38 / 40) |
| oak 5678, 1,024 | 0.344 / 0.290 | 0.17 | 5.43 / 6.65 | 4.96 / 5.41 | 0.167 (11,297 / 917) |
| oak 5678, 512 | 0.344 / 0.306 | 0.26 | 6.11 / 6.65 | 6.14 / 6.21 | 0.042 (542 / 390) |
| oak 5678, 256 | 0.344 / 0.307 | 0.26 | 6.12 / 6.65 | 6.14 / 6.23 | 0.041 (199 / 241) |
| oak 5678, 128 | 0.344 / 0.310 | 0.27 | 6.55 / 6.43 | 6.65 / 6.78 | 0.036 (96 / 96) |
| oak 5678, 64 | 0.344 / 0.333 | 0.28 | 6.48 / 6.43 | 6.65 / 6.70 | 0.021 (39 / 60) |
| oak 7919, 1,024 | 0.447 / 0.387 | 0.12 | 4.87 / 5.93 | 5.39 / 5.06 | 0.139 (3,109 / 944) |
| oak 7919, 512 | 0.447 / 0.403 | 0.14 | 5.62 / 5.88 | 5.93 / 5.72 | 0.064 (577 / 390) |
| oak 7919, 256 | 0.447 / 0.404 | 0.14 | 5.64 / 5.88 | 5.93 / 5.74 | 0.059 (153 / 89) |
| oak 7919, 128 | 0.447 / 0.407 | 0.14 | 5.66 / 5.88 | 5.93 / 5.76 | 0.059 (153 / 89) |
| oak 7919, 64 | 0.447 / 0.438 | 0.16 | 5.63 / 5.88 | 5.93 / 5.73 | 0.035 (60 / 40) |
| oak 7737, 1,024 | 0.376 / 0.341 | 0.19 | 5.24 / 8.20 | 5.25 / 5.22 | 0.158 (3,226 / 917) |
| oak 7737, 512 | 0.376 / 0.351 | 0.24 | 7.87 / 8.07 | 8.20 / 8.06 | 0.055 (221 / 241) |
| oak 7737, 256 | 0.376 / 0.351 | 0.25 | 7.89 / 8.07 | 8.20 / 8.08 | 0.055 (221 / 241) |
| oak 7737, 128 | 0.376 / 0.354 | 0.25 | 7.92 / 8.07 | 8.20 / 8.12 | 0.044 (122 / 64) |
| oak 7737, 64 | 0.376 / 0.368 | 0.25 | 7.89 / 8.07 | 8.20 / 8.08 | 0.032 (52 / 60) |
| oak 6137, 1,024 | 0.341 / 0.277 | 0.17 | 4.87 / 6.12 | 5.55 / 4.96 | 0.139 (5,057 / 917) |
| oak 6137, 512 | 0.341 / 0.295 | 0.25 | 6.14 / 6.12 | 6.09 / 6.12 | 0.045 (215 / 390) |
| oak 6137, 256 | 0.341 / 0.297 | 0.25 | 6.14 / 6.12 | 6.09 / 6.12 | 0.039 (135 / 131) |
| oak 6137, 128 | 0.341 / 0.300 | 0.26 | 6.14 / 6.12 | 6.09 / 6.12 | 0.034 (111 / 96) |
| oak 6137, 64 | 0.341 / 0.323 | 0.27 | 6.08 / 6.12 | 6.09 / 6.07 | 0.034 (71 / 60) |
| oak 7432, 1,024 | 0.384 / 0.331 | 0.14 | 5.34 / 6.21 | 6.32 / 5.45 | 0.108 (2,373 / 674) |
| oak 7432, 512 | 0.384 / 0.338 | 0.18 | 6.24 / 6.21 | 6.32 / 6.39 | 0.046 (427 / 426) |
| oak 7432, 256 | 0.384 / 0.338 | 0.18 | 6.25 / 6.21 | 6.32 / 6.41 | 0.042 (162 / 117) |
| oak 7432, 128 | 0.384 / 0.341 | 0.19 | 6.28 / 6.21 | 6.32 / 6.44 | 0.042 (162 / 117) |
| oak 7432, 64 | 0.384 / 0.367 | 0.19 | 6.27 / 6.21 | 6.32 / 6.42 | 0.027 (65 / 60) |
| birch 1234, 1,024 | 0.158 / 0.156 | 0.70 | 8.04 / 7.58 | 7.40 / 7.46 | 0.020 (1,379 / 871) |
| birch 1234, 512 | 0.158 / 0.156 | 0.68 | 8.04 / 7.58 | 7.40 / 7.46 | 0.019 (947 / 369) |
| birch 1234, 256 | 0.158 / 0.156 | 0.66 | 8.04 / 7.58 | 7.40 / 7.46 | 0.014 (645 / 131) |
| birch 1234, 128 | 0.158 / 0.157 | 0.60 | 7.58 / 7.58 | 7.40 / 7.42 | 0.014 (524 / 110) |
| birch 1234, 64 | 0.158 / 0.159 | 0.64 | 7.52 / 7.58 | 7.40 / 7.39 | 0.013 (383 / 15) |
| birch 5678, 1,024 | 0.129 / 0.129 | 0.70 | 6.32 / 6.35 | 6.61 / 6.63 | 0.022 (1,245 / 755) |
| birch 5678, 512 | 0.129 / 0.129 | 0.68 | 6.31 / 6.35 | 6.61 / 6.62 | 0.018 (902 / 479) |
| birch 5678, 256 | 0.129 / 0.128 | 0.67 | 6.31 / 6.35 | 6.61 / 6.62 | 0.018 (704 / 115) |
| birch 5678, 128 | 0.129 / 0.129 | 0.62 | 6.50 / 6.35 | 6.61 / 6.60 | 0.018 (704 / 115) |
| birch 5678, 64 | 0.129 / 0.130 | 0.64 | 7.55 / 6.35 | 6.61 / 6.59 | 0.013 (414 / 6) |

### Stills (viewed: all sixteen; the worker's reading, not a verdict)

**The oak, all six seeds:**
- Round 3's full oak is a broad, lobed crown with heavy limbs and a visible trunk. It is far from 0.1.4's round, solid crown at every seed.
- At 1,024, the coarse oak is a more compact, upright, denser crown. It misses one or two of the full oak's long lower limbs: seed 1234's limb to the right, 7919's low limb to the right, 7432's open lower limbs.
- From 512 down, those limbs appear, and the panels come close to the full field's silhouette and wood pattern.
- 256 and 128 look like 512. At 64, the trunk and wood fill in a little more.
- The panels read the same at n = 36 as at n = 48.

**The birch, both seeds:**
- The full birch is a narrow, tall column with its trunk showing.
- The coarse birch is a different shape at every cut: a broad, round, dense mass of foliage cubes with almost no wood showing. It grows lumpier and taller as the cut falls.
- At 1,024, its foliage at n = 48 is 7,189 to 7,519 cubes, against the full birch's 1,539 to 2,111. Its field height is 19.4 to 26.1 m, against 16.9 to 17.4 m, while the structure above the cut measures 17.2 m (seed 1234) and 16.7 m (seed 5678) natively, as in the stepped tree. The extra height and bulk is in the stands' extent: the native table shows the trunk and the section within reach. Their cause was not looked into.

### Reading, for the host

**The oak (round 3):**
- **Time:** no cut is near 0.1.4.
  - 0.1.4 runs in 333 to 386 ms at n = 48, and the full field in 7.5 to 9.8 s.
  - The coarse field runs in 1.06 to 1.55 s at 1,024, 1.72 to 2.40 s at 512, 1.80 to 2.69 s at 256 and 128, and 3.8 to 5.7 s at 64.
  - n = 36 saves 60 to 160 ms, because the grow dominates.
- **Memory:** within 0.1.4's 73 to 116 MB, or under it, at 1,024 (48 to 83 MB).
  - At 512 to 128, it reaches 92 to 144 MB.
  - At 64, it is 186 to 378 MB.
- **Trunk:**
  - At 1,024, the coarse trunk is 0.81 to 0.91 of the stepped one.
  - At 512 to 128, it is 0.87 to 0.94.
  - At 64, it is 0.95 to 0.98.
- **Section ratio:** it rises from 0.12–0.19 at 1,024 to 0.16–0.28 at 64.
- **Reach:** at 1,024, the stands fall short by 0.8 to 3.0 m. From 512 down, they come within 0.3 m on five seeds. Seed 5678 needs 128.
- **The thickest root left below the cut:** 0.11 to 0.17 m at 1,024, kept 2,373 to 11,297 phytomers against a prediction of 674 to 965. From 512 down it is 0.09 m or less.
- **In short:** under round 3 the limbs come back at 512, not 256, and the trunk nearly closes. On this machine, the coarse oak at any cut is 3 to 15 times 0.1.4's time, while it is 5 to 8 times faster than the full field at 1,024 and 512.

**The birch (fn-213 round 14) is no longer refused.**
- **Time:**
  - 0.1.4 runs in 400 to 410 ms, and the full field in 1.5 to 2.5 s in 158 to 315 MB.
  - The coarse field runs in 0.88 to 1.07 s at 1,024, 0.91 to 1.17 s at 512 and 256, and 1.4 to 2.4 s at 128 and 64.
- **Memory:** 15 to 28 MB from 1,024 to 256, and 56 to 149 MB at 128 and 64.
- **Native structure:** it matches. The trunk is within 0.002 m, the section ratio is 0.60 to 0.70, and the reach is within 0.5 m, except 5678 at 64 (1.2 m over).
- **The field:** it does not match. The stills and the cube counts (above) show a round, overfull crown and a field 2.5 to 9 m too tall.

## Task 2: the coarse birch's field, its cause (2026-10-09)

The coarse birch's field is wrong: at cut 1,024 it has 4 to 5 times the full birch's foliage cubes and stands 2.5 to 9 m too tall. The cause is the shape of each stand's foliage, a solid capsule whose radius is its expected wood's centre distance. Its leaf count is not the cause, and neither are the clusters. The stands also carry no droop, and some reach below the ground. A fix needs a decision on how a stand's foliage is shaped, so it was not built (below).

**Measured, natively.** Age 40, at `94e4cc03`, cut 1,024.
- **Sources:** an ignored scratch test in the core (`raw/native/diag_coarse_tests.rs.txt`; it was not committed, and the file is back to HEAD), and `raw/native/src/bin/shape.rs` (output in `raw/native/shape.txt`).
- **Cell counts:** foliage cells at n = 48 on one grid over both fields' union. A cell is wood instead when its wood radius is at least 0.08 of a cell, the site's rule. Before the site's thinning.

| | birch 1234 | birch 5678 | oak 1234 |
|---|--:|--:|--:|
| Foliage cells, full field | 4,928 | 3,525 | 11,315 |
| Foliage cells, coarse field | 23,488 | 22,465 | 12,521 |
| from the structure above the cut alone | 962 | 570 | 5,143 |
| from the stands alone | 23,487 | 22,464 | 12,108 |
| coarse, with each stand's spread at 0 (its segment and the blade's reach only) | 6,697 | 5,097 | 6,833 |
| coarse, with each stand's spread halved | 13,828 | 12,423 | 9,585 |
| coarse, with each stand's tip at its centre, not twice it | 13,405 | 13,033 | 10,079 |
| Coarse foliage cells below the ground (y < 0) | 896 | 924 | 51 |

**Leaves, from the plan: not the cause.**
- **Leaf totals:** the birch's plan holds 162,175 leaves on the full tree and 145,797 on the coarse tree at 1,024, 144,724 of them on its 6,717 stands. At 256 the coarse tree holds 153,300.
- **Phytomers:** the matched stands expect 399,382 kept phytomers against 508,564 realized (seed 1234).
- **Clusters:** the structure above the cut holds no short shoot (0 clusters, against the full tree's 36,015), so the stand's leaves per phytomer (0.28) come from the long shoots above. Even so, the coarse total is 10 % under the full one. Clusters do not explain the bulk.

**The extent, by matched lineage (birch 1234; 5678 alike).**

| | realized fine root | its stand |
|---|---|---|
| Phytomer tip farthest from its base (realized) / capsule reach 3 × centre (stand), p50 / p90 | 0.33 / 2.11 m | 0.50 / 2.64 m |
| Centroid distance from base, p50 / p90 | 0.20 / 1.25 m | 0.17 / 0.88 m (centre) |
| Centroid vertical offset, p10 / p50 | −1.12 / 0.00 m | −0.09 / 0.01 m |
| Lowest point (z) | 0.00 m (the ground) | −1.43 m |

- **The fill:** a stand's reach is close to its realized root's, at 1.3 to 1.5 times. But the capsule's whole volume answers foliage, while the root's leaves are a sparse set of hanging shoots inside that volume.
- **Why the birch and not the oak:** the birch's roots are long and sparse. Their centre is 0.88 m at p90, against the oak's 0.13 m, and the capsule's volume grows as the centre cubed. So the birch's stands fill 4.8 times its full field's foliage cells, and the oak's fill 1.1 times.
- **The droop:** a realized birch root's centroid hangs 1.1 m below its base at p10. A stand lies along its bud's heading, which points against the realized centroid's direction for at least a tenth of the stands (cosine −0.41 at p10).
- **The ground:** a stand's capsule is not held above the ground, and 4 % of the coarse birch's foliage cells lie below it. That accounts for the field's extra height, 2.4 m at 1,024 and 4.2 m at 256 (bounds min y). The structure's own top is the same, 17.18 m.

**Why no fix was built.** No row the closed form commits carries what a stand would need to agree with its grown root:
- how sparsely its leaves fill its reach;
- its droop (the closed form is a count, a section, a load and a distance, with no direction);
- the ground.

Each repair is a choice of representation, and none is a matter of agreement:
- a thinner capsule;
- foliage answered only where a stand's expected leaves per cell pass a density;
- a new expected displacement row for the droop;
- a clamp at the ground.

Setting the spread to 0 alone gives 1.36 and 1.45 times the full birch's cells, and 0.60 times the full oak's. It is listed as a measurement, not proposed. The host decides.

### Host decision on the stand-ins' foliage (2026-10-09)

A stand-in will answer foliage by its expected occupancy, not by its volume. The row already holds the expected leaf count, and the capsule holds the reach, so the expected number of leaves in one cell of the capsule is λ = expected leaves × cell volume / capsule volume. The full field marks a cell as foliage when a leaf stands in it, so the cell is foliage with probability 1 − e^(−λ), the Poisson chance that at least one leaf lands there.

- **The draw.** It is keyed by the stand-in's lineage and the cell index through the lineage key registry: a new key, mirrored wherever the field is answered. It is deterministic, and it changes by degree, with no switch between ways of building.
  - **What happens as λ grows:** a dense oak stand-in fills its small capsule, as it does today.
  - **What happens as λ shrinks:** a sparse birch stand-in fills only its expected share of a large capsule.
- **The ground.** No stand-in answers foliage below the ground. This clamps every capsule at the ground; it is not a special case.
- **Droop.** This needs a new expected-displacement row and is left out. Measure the birch first. If the stills still read wrong after occupancy, droop becomes its own spec.
- **Proof.**
  - A red/green test: a stand-in's expected occupied cells equal 1 − e^(−λ) of its cells, within sampling error.
  - The birch's and oak's foliage cell counts against their full fields at cuts 1024 and 256.
  - The field stills, all viewed.
  - The time.
  - Full trees byte-identical.
  - The full workspace gate.

Still no cut is chosen and no default is changed.

## Task 2: the stand-ins' occupancy, built and measured (2026-10-09)

The host's decision is built.

- **The red test came first:** `0658faef`, `field/stand_tests.rs`. It failed with 1.000 of 1,344 cells answering at λ 0.1, against an expected 0.095.
- **Occupancy** (`92fd9439`):
  - **The rule:** a stand-in's sweep answers a cell with the chance 1 − e^(−λ), where λ = leaves × cell volume / capsule volume (`field/stand.rs`).
  - **The draw:** keyed by `tree::stand_key(lineage)`. That is the fine root's lineage under the registry's new `STAND` key (`u64::MAX − 16`, under the lineage), mirrored in `lineage.wgsl` and checked by `the_wgsl_keys_are_the_cpus`. The key is then mixed with the cell's integer coordinates.
  - **The ground:** no stand-in answers a cell whose centre is below the ground.
- **The bounds** (`139baa05`): a stand-in's foliage bounds stop at the ground as well. Without this, the field's bounds, which set the site's grid, still reached 2.4 to 4.2 m below the ground, and the birch's field stood 19.7 m tall at 1,024. The test asserts it.
- **Shape of the change:**
  - `Stand` moves to `tree/stand.rs` with its key.
  - A descriptor's `spread` becomes `stand: Option<Standing { spread, key }>`.
- **One addition beyond the decision text.** A cell a stand-in fills reports λ / (1 − e^(−λ)) expected leaves, the expectation given that at least one landed. It no longer reports the segment's share. This keeps the per-cell leaf estimates summing, in expectation, to the stand-ins' leaves above the ground, and no cell reports leaves without foliage. The `density` thinning rule reads this. The `tuft` rule the site uses for the oak and the birch does not.
- **Full trees are unchanged.** The full fields of the oak (1234, 5678), the birch (1234, 5678), the spruce (1234) and the palm (1234) answer cell for cell alike at n = 48, before (`field-release.wasm`) and after (`field-fill.wasm`), bounds included (`raw/field-path/same2.mjs`, `same2.txt`). `catalogue_identity` passes unchanged.

### Foliage cubes and time (Wasm under Node, the site's request, age 40)

These are after the site's thinning, which keeps a fixed share of the foliage cells, so the ratios are the cells' ratios. "Before" is the release sweep (`release-sweep.jsonl`, load 1.9 to 3.2). "After" is `raw/field-path/fill-sweep.jsonl`: three repeats at load 7.7 to 18.2, so its times run about 5 % high against "before". The grid follows the field's bounds, so a field whose bounds moved is gridded more finely.

| Tree, seed | cut | foliage cubes n = 48, full | before | after | after / full | n = 36 full / before / after | wood n = 48 full / after | end to end n = 48 ms, after (before) | n = 36 ms | field height m, full / before / after | memory MB |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| oak 1234 | 1,024 | 4,526 | 7,126 | 7,024 | 1.55 | 2,097 / 3,148 / 3,139 | 4,116 / 3,595 | 1,613 (1,544) | 1,521 | 11.08 / 11.01 / 11.01 | 80.3 |
| oak 1234 | 256 | 4,526 | 4,022 | 4,019 | 0.89 | 2,097 / 1,908 / 1,906 | 4,116 / 3,293 | 2,659 (2,501) | 2,572 | 11.08 / 11.04 / 11.04 | 131.9 |
| oak 5678 | 1,024 | 5,390 | 7,156 | 7,053 | 1.31 | 2,538 / 3,153 / 3,158 | 3,930 / 2,797 | 1,227 (1,138) | 1,157 | 11 / 10.97 / 10.94 | 49.6 |
| oak 5678 | 256 | 5,390 | 5,369 | 5,342 | 0.99 | 2,538 / 2,535 / 2,520 | 3,930 / 3,533 | 2,185 (1,870) | 2,086 | 11 / 11.03 / 10.95 | 91.8 |
| oak 6137 | 1,024 | 5,634 | 7,247 | 7,318 | 1.30 | 2,601 / 3,184 / 3,234 | 4,441 / 2,976 | 1,063 (1,057) | 1,003 | 11.05 / 11 / 10.87 | 48.3 |
| oak 6137 | 256 | 5,634 | 5,097 | 5,279 | 0.94 | 2,601 / 2,422 / 2,480 | 4,441 / 3,800 | 1,849 (1,796) | 1,770 | 11.05 / 11.2 / 11.01 | 92 |
| oak 7432 | 1,024 | 5,541 | 7,799 | 7,776 | 1.40 | 2,541 / 3,418 / 3,458 | 4,518 / 3,465 | 1,342 (1,292) | 1,265 | 11.11 / 11.12 / 11.05 | 54.8 |
| oak 7432 | 256 | 5,541 | 5,349 | 5,312 | 0.96 | 2,541 / 2,531 / 2,506 | 4,518 / 3,907 | 2,280 (2,207) | 2,185 | 11.11 / 11.18 / 11.05 | 121.7 |
| oak 7737 | 1,024 | 4,440 | 6,166 | 6,123 | 1.38 | 2,022 / 2,727 / 2,726 | 3,587 / 2,884 | 1,143 (1,060) | 1,083 | 11.09 / 11.14 / 11.14 | 50.3 |
| oak 7737 | 256 | 4,440 | 4,470 | 4,468 | 1.01 | 2,022 / 2,105 / 2,105 | 3,587 / 3,226 | 1,866 (1,809) | 1,787 | 11.09 / 11.15 / 11.15 | 98.6 |
| oak 7919 | 1,024 | 5,116 | 6,875 | 6,722 | 1.31 | 2,306 / 3,037 / 3,010 | 5,356 / 3,659 | 1,666 (1,548) | 1,584 | 11.38 / 11.34 / 11.34 | 82.6 |
| oak 7919 | 256 | 5,116 | 4,876 | 4,865 | 0.95 | 2,306 / 2,226 / 2,221 | 5,356 / 4,547 | 2,657 (2,529) | 2,532 | 11.38 / 11.35 / 11.35 | 112.3 |
| birch 1234 | 1,024 | 2,111 | 7,519 | 4,241 | 2.01 | 1,061 / 3,204 / 2,241 | 334 / 323 | 1,133 (1,067) | 998 | 17.44 / 19.72 / 17.43 | 18.8 |
| birch 1234 | 256 | 2,111 | 8,029 | 3,595 | 1.70 | 1,061 / 3,424 / 1,839 | 334 / 267 | 1,176 (1,174) | 1,041 | 17.44 / 21.47 / 19.28 | 28 |
| birch 5678 | 1,024 | 1,539 | 7,189 | 3,214 | 2.09 | 779 / 3,072 / 1,878 | 258 / 262 | 926 (875) | 858 | 16.92 / 19.39 / 16.93 | 14.5 |
| birch 5678 | 256 | 1,539 | 7,654 | 3,001 | 1.95 | 779 / 3,275 / 1,702 | 258 / 226 | 968 (911) | 893 | 16.92 / 20.77 / 18.22 | 17.1 |

### Stills

`raw/field-path/fill/{oak,birch}-<seed>-n{48,36}.png` hold four panels, left to right: 0.1.4, full, coarse at 1,024 and coarse at 256. The worker viewed all sixteen. What follows is its reading, not a verdict.
- **The birch:** it is now a tall, upright crown of the full birch's height with its trunk below. It is no longer a round lump.
  - It is wider and fuller than the full birch's narrow, open column. The full birch shows its trunk through the crown, and the coarse birch shows almost no wood inside it.
  - At 256, the trunk is mostly hidden by low foliage.
  - The full birch's few hanging low shoots are absent: there is no droop.
- **The oak:** it reads as it did before this change at both cuts. Its stand-ins are dense (λ high), so they still fill their small capsules.

### Reading, for the host

- **The birch's bulk:**
  - The foliage cubes fall from 3.6 to 5.0 times the full birch's to 1.7 to 2.1 times.
  - At 1,024, the field height matches the full tree: 17.43 against 17.44 m, and 16.93 against 16.92 m.
- **What is left of the birch's excess:**
  - The look: wider, fuller and no droop.
  - At 256 the field still stands 1.3 to 1.8 m too tall (19.28 and 18.22 m). That comes from the stand-ins' wood, the segment from base to tip at the base radius, which still reaches below the ground. The decision clamps the foliage capsules only, so the wood was left alone.
  - Whether droop would close the rest is not measured. The decision makes droop its own spec if the stills still read wrong.
- **The oak:** foliage moves by −2 % to +4 % against before, its time by noise, and its look not at all.
- **The time:** the coarse oak runs in 1.06 to 1.67 s at 1,024 and 1.85 to 2.66 s at 256 (n = 48), and the coarse birch in 0.93 to 1.18 s. Against "before", every row is within the load difference. Occupancy is a few multiplies and one hash per stand-in a cell meets.

### Gates

- **The workspace gate,** run package by package because the whole does not fit one 600 s call (FRICTION.md):
  - `telperion-core`: 452 passed, 44 ignored.
  - `telperion-space`: 221 passed.
  - `telperion-field`: 7 passed.
  - `telperion-wasm`: 6 passed.
  - `telperion-jev`: 389 passed.
  - `telperion-render`, under the GPU lock: its integration tests 95 passed, and its library 119 of 120.
- **The one render failure:** `every_pipeline_within_2_5_s_but_grow_within_20_s_and_the_first_tree_within_5_s_cold` made its cold `grow` pipeline in 20,132 ms against a bound of 20,000 ms while the library's tests ran four at a time. Run alone, it passed in 21.7 s.
- **Two inherited failures, fixed:**
  - `telperion-field`'s `a_cut_grows_a_coarse_field_or_is_refused_by_name` and `src/field/field.test.ts` still expected the oak's cut to be refused. 433c4c86 lifted that refusal (task 2's stepped stand-ins). Both now assert that the oak answers a coarse field with foliage (`79329794`, `904761c0`).
  - The limits guard needed the two new ground sites declared in `docs/generation-limits-inventory.json`.
- **`npm test`:** 144 of 144 passed.
