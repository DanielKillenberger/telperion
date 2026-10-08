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
