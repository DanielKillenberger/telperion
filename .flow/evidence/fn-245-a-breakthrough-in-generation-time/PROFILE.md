# The site path, stage by stage (F4 release at ad67cc47)

A measurement for the host. No source in any checkout was changed. The instrumented build is a copy in this folder.

## The three biggest costs

1. **The settle's layout, `place`, run twice** (`crates/telperion-space/src/geometry/place.rs:31`, called from `fine/lay.rs:60` and `fine/lay.rs:76`, with `thicken_given` beside it). It costs 92 to 401 ms in Wasm:
   - spruce: 47 % of the grow (166 of 351 ms);
   - oak: 29 % (401 of 1,405 ms);
   - birch: 18 % (153 of 867 ms).

   It runs 3.1 to 3.5× slower in Wasm than natively, the highest ratio of any stage. It lays every axis above the cut, and most of those axes are fine-root placeholders: 96 % of the spruce's 22,223 axes, 89 % of the birch's and 80 % of the oak's. Two things here are inferred and were not measured apart: the high Wasm ratio, from the layout's Q32 fixed point (`geometry/q.rs`, `fixed.rs:472` `mul` widening to i128), and the share of the time the placeholders take. For the oak, the per-cycle rough layout (`grow/draw.rs:88`, mostly `draw_round` at `:134`) adds 125 to 175 ms in Wasm, again at 3.1×.
2. **The oak's allotment split** (`grow/stepped/phases.rs:57` → `allocation.rs:72` `split` → `allocation.rs:40` `thrice_over` → `fixed/feedback/wide.rs:102` `I256::quotient`). It costs 256 ms of the oak 1234's 1,405 ms in Wasm (18 %) and 191 of 1,020 ms for seed 5678.
   - The quotient is a 256-iteration bitwise long division on u128 halves, called 296,244 times over the 40 cycles (native counter).
   - Natively, 97 % of `split` is the walk that calls it (166 of 173 ms). Subtotals, the kids' sort and the rest take 6 ms.
3. **The birch's closed form** (`fine/coarse.rs:82` `Rows::laterals` → `closed_form/boundary.rs:76` `read` → `closed_form/kept.rs:234` `Kept::of`). It costs 634 ms of the birch's 787 to 867 ms in Wasm (73 to 80 %), for 205 (PA, age) rows.
   - It is the same for both seeds: a row is a value of the species and age alone (`boundary.rs` doc comment).
   - Its Wasm ratio is 1.1×, so it is algorithmic, not Wasm. It is recomputed on every `grow` call.
   - For the oak and the spruce the same stage costs 50 ms.

Next in line, oak only: the light lattice, 122 to 167 ms in Wasm (`grow/stepped/feedback.rs:76`, `light/field.rs:35` `Field::on`, 100 to 132 ms). Of the 820,000 leaf deposits over 40 cycles, 761,110 (93 %) are the stand-ins' seven-point stencils (`grow/stepped/stands.rs:187`). The coarse grower deposits 58,767.

## End to end at the site's request (n = 36, cut 1,024, age 40), Wasm, medians of 3

| Tree | 0.1.4 grow / query / voxelize ms | F4 grow / query / voxelize ms | F4 end to end | 0.1.4 end to end | F4 native grow, 1 core |
|---|--:|--:|--:|--:|--:|
| birch 1234 | 246 / 84 / 12 | 874 / 124 / 10 | **1,012** | 346 | 642 |
| birch 5678 | 267 / 74 / 11 | 794 / 62 / 8 | **868** | 357 | 607 |
| oak 1234 | 166 / 94 / 13 | 1,367 / 96 / 10 | **1,478** | 278 | 698 |
| oak 5678 | 191 / 110 / 13 | 1,021 / 73 / 10 | **1,109** | 319 | 508 |
| spruce 1234 | 149 / 84 / 3 | 345 / 34 / 4 | **388** | 240 | 146 |
| spruce 5678 | 151 / 86 / 3 | 326 / 34 / 3 | **367** | 244 | 151 |

End to end is grow, the JavaScript grid (4 to 5 ms), query and voxelize.
- **The query is not the problem:** F4 queries as fast as 0.1.4 or faster. The whole regression is in `grow`.
- **Wasm overhead is not the whole of it either:** even natively on one core, F4's grow is 1.0 to 4.2× 0.1.4's Wasm grow. The spruce is the closest, 146 to 151 ms against 149 to 151.

## Stages, Wasm / native on one core, ms (Wasm ÷ native)

| Stage | birch 1234 | birch 5678 | oak 1234 | oak 5678 | spruce 1234 | spruce 5678 |
|---|--:|--:|--:|--:|--:|--:|
| 1 closed-form rows and tables, rules | 657 / 574 (1.1×) | 655 / 568 (1.2×) | 56 / 33 (1.7×) | 56 / 32 (1.7×) | 64 / 44 (1.5×) | 62 / 43 (1.4×) |
| 2a above the cut: the grower's cycles | 17 / 8 (2.1×) | 12 / 5 (2.4×) | 690 / 361 (1.9×) | 500 / 265 (1.9×) | 37 / 20 (1.8×) | 30 / 22 (1.4×) |
| 2b above the cut: the settle passes (exchange, shed, thicken, place ×2, levers) | 153 / 48 (3.2×) | 92 / 27 (3.5×) | 401 / 145 (2.8×) | 291 / 104 (2.8×) | 166 / 51 (3.2×) | 158 / 51 (3.1×) |
| 3 stand-ins | 19 / 6 (3.0×) | 11 / 4 (3.1×) | 185 / 133 (1.4×) | 125 / 87 (1.4×) | 55 / 19 (2.9×) | 53 / 18 (2.9×) |
| 4 conversion | 3 / 2 | 2 / 1 | 9 / 4 | 7 / 3 | 3 / 1 | 3 / 1 |
| 5 field construction (leaf plan, `Field::planned`) | 13 / 6 | 9 / 2 | 46 / 20 | 34 / 17 | 21 / 9 | 19 / 8 |
| not covered by a span | 5 / 1 | 5 / 1 | 8 / 3 | 9 / 3 | 5 / 2 | 5 / 1 |
| **grow() total** | **867 / 642 (1.3×)** | **787 / 607 (1.3×)** | **1,405 / 698 (2.0×)** | **1,020 / 508 (2.0×)** | **351 / 146 (2.4×)** | **333 / 151 (2.2×)** |
| 6 grid query | 122 / 100 (1.2×) | 60 / 50 (1.2×) | 96 / 74 (1.3×) | 72 / 57 (1.3×) | 34 / 29 (1.2×) | 32 / 28 (1.1×) |
| 7 voxelize (JavaScript only) | 10 | 8 | 10 | 10 | 4 | 3 |

How the stages split:
- **The oak (lit)** grows above the cut in the stepped executor:
  - Stage 2a is balance, allotment, `grow_live`, wake, rough layout, the lattice and the standing pass, with the stand-ins' parts taken out.
  - Stage 3 is those parts: the stand totals, kept, lit and sums; the stands' advance; `stand_over`; the stencils and the stand read. It also holds the `Expected` executor's phases 2 to 4 and the stands' assembly (`coarse()`).
- **The spruce and the birch** take `settle_with`:
  - Stage 2a is `streamed` above the cut.
  - Stage 3 is the `Expected` executor's phases 1 to 4, the seeds and `coarse()`.

### Inside the largest stages, Wasm / native ms

| Sub-stage | oak 1234 | oak 5678 | spruce 1234 | birch 1234 |
|---|--:|--:|--:|--:|
| rows: `Kept::of` (205 / 246 / 123 rows) | 36 / 22 | 37 / 22 | 32 / 21 | **634 / 562** |
| (i) balance (40 cycles) | 31 / 26 | 22 / 17 | – | – |
| (ii)+(iii) allotment | **332 / 179** | **241 / 130** | – | – |
| … of which `split` | 256 / 144 | 191 / 106 | – | – |
| … of which the stand sums | 40 / 16 | 29 / 10 | – | – |
| stands advance (138,137 / 101,317 root-cycles) | 62 / 49 | 46 / 36 | – | – |
| (iv) `grow_live` + wake | 34 / 29 | 24 / 19 | 37 / 20 (streamed) | 17 / 8 |
| (iv) rough layout, `sketch` (43,938 / 31,908 axes drawn) | **175 / 58** | 125 / 43 | – | – |
| (iv) lattice, `shade` | **167 / 100** | 122 / 75 | – | – |
| … of which `Field::on` | 132 / 74 | 100 / 58 | – | – |
| `stand_over` | 29 / 32 | 16 / 18 | – | – |
| the exchange | 63 / 27 | 45 / 20 | 3 / 2 | 1 / 1 |
| thicken and place (unbent) | **163 / 53** | 119 / 39 | 68 / 19 | 59 / 19 |
| levers and place (bent) | **168 / 56** | 121 / 40 | **92 / 29** | **91 / 29** |
| `Expected` phase 3 lay | 15 / 5 | 11 / 4 | 24 / 7 | 8 / 3 |
| stands out, `coarse()` | 16 / 5 | 11 / 4 | 24 / 8 | 9 / 3 |

## Wasm overhead

Native single-thread against Wasm, per stage, from the tables above:
- **Near native (1.1 to 1.4×):** the closed form's f64 recursion, the query, and the oak's stand aggregates (f64).
- **About 2× (1.9 to 2.1×):** the stepped grower's cycles (balance, allotment, grow).
- **About 3× (2.8 to 3.5×):** the settle passes and the rough layout. These are the Q32 layout (`place`, `sketch`'s `draw_round`) and the girth (`thicken_given`, 3 to 4×).

Whole grow: 1.3× for the birch (its cost is the f64 closed form), 2.0× for the oak, and 2.2 to 2.4× for the spruce.

If Wasm ran at native speed, the oak would still grow in 508 to 698 ms against 0.1.4's 166 to 191, and the birch in 607 to 642 against 246 to 267. Measured against the regression from 0.1.4's grow, the Wasm overhead accounts for about all of the spruce's (199 of 196 ms), 56 to 62 % of the oak's and 35 to 37 % of the birch's.

## The waste: what is grown against what the field fills (n = 36)

| | birch 1234 | birch 5678 | oak 1234 | oak 5678 | spruce 1234 | spruce 5678 |
|---|--:|--:|--:|--:|--:|--:|
| Phytomers grown above the cut (incl. shed) | 16,668 | 9,867 | 60,771 | 44,336 | 12,571 | 12,106 |
| Phytomers kept above the cut | 16,668 | 9,855 | 60,036 | 43,974 | 12,571 | 12,106 |
| Axes above the cut / of them live stand-ins | 7,947 / 7,057 | 4,643 / 4,071 | 17,058 / 13,579 | 12,220 / 9,772 | 22,223 / 21,299 | 21,494 / 20,586 |
| Fine roots handed over (seats) | 8,051 | 4,725 | 30,366 | 22,028 | 21,933 | 21,210 |
| Stand-ins' expected phytomers | 524,790 | 299,195 | 1,205,967 | 863,782 | 1,279,727 | 1,231,580 |
| Pipeline nodes after conversion | 14,683 | 8,631 | 46,840 | 39,330 | 12,028 | 11,574 |
| Leaf-plan descriptors | 7,779 | 4,449 | 48,060 | 34,912 | 22,942 | 22,070 |
| Cell edge (m) | 0.513 | 0.498 | 0.324 | 0.322 | 0.363 | 0.360 |
| Cells the field fills, of 46,656 | 7,283 | 6,194 | 9,598 | 9,224 | 3,242 | 3,232 |
| Cubes drawn, wood + foliage | 167 + 2,241 | 144 + 1,878 | 1,750 + 3,139 | 1,328 + 3,158 | 156 + 2,469 | 129 + 2,482 |
| Phytomers grown per drawn cube | 6.9 | 4.9 | 12.4 | 9.9 | 4.8 | 4.6 |
| Cells holding any node | 682 | 460 | 1,908 | 1,521 | 1,085 | 1,083 |
| Nodes per node-holding cell, median / p90 | 14 / 43 | 12 / 36 | 10 / 62 | 9 / 45 | 10 / 20 | 10 / 19 |
| Nodes thinner than a cell (diameter < edge) | 100 % | 100 % | 99.6 % | 99.7 % | 99.8 % | 99.8 % |
| Nodes under the site's wood cutoff (no wood cube) | 95.8 % | 94.3 % | 88.6 % | 90.3 % | 97.0 % | 97.4 % |
| Internodes shorter than a cell | 100 % | 100 % | 100 % | 100 % | 100 % | 100 % |
| Branch runs lying wholly inside one cell (their nodes) | 34 of 174 (1,335) | 31 of 112 (1,386) | 235 of 836 (5,126) | 259 of 639 (12,302) | 2 of 146 (9) | 1 of 142 (1) |
| Stand-ins shorter than a cell | 2,727 of 6,716 | 1,927 of 3,858 | 7,536 of 11,654 | 6,744 of 9,300 | 7,456 of 21,293 | 7,351 of 20,586 |

What the table shows:
- **Nearly everything grown is finer than a cube.** Almost every node is thinner than a cell, every internode is shorter than one, and 88 to 97 % of the nodes are under the wood cutoff, so they draw no wood cube.
- **Nodes stack up in few cells.** They sit 9 to 14 to a cell at the median and up to 62 at p90, in only 460 to 1,908 cells. The field fills 3,232 to 9,598 cells, so most filled cells are answered by the leaf plan's sweeps and the stand-ins, not by grown wood.
- **The oak grows the most per cube:** 10 to 12 phytomers per drawn cube. Its stepped run also advances 101,000 to 138,000 stand root-cycles and deposits 556,000 to 761,000 stencil leaves to fill those cubes.
- **The spruce's work is in its placeholders.** Its structure above the cut is small (12,000 phytomers), but it lays 22,000 axes, of which 21,000 are stand-in placeholders (see cost 1).

The "finer than one cube" measures are taken on the pipeline's nodes after the conversion merges phytomers under 1 mm, in the field's frame (`siteprof-native`, untimed).

## Why 0.1.4 is cheap

0.1.4 (`v0.1.4`, `crates/telperion-core/src/branching.rs:291` `generate` → `branching/specimen.rs:296` `Specimen::grow`) builds one mature specimen in a single pass, with no clock of years.
- **The scaffold:** grown once in growth units, every unit's heading the sum of its rule heading, the bias field and a space-colonization pull (`colonization.rs`). The pull comes from 500 attractors (every shipped preset) scattered in the authored envelope, indexed on a grid, with a step of 2.2 % of the height.
- **The twig layer:** appended by the local planner's rules (`branching/local`).
- **The finish:** shoots outside the envelope are shed and the radii solved once by the pipe model (`branching.rs:298` `finish`).

All of it is plain f64. There is no closed-form table to read and no fine-root expectation. Nothing is repeated each year across the tree:
- no light lattice deposited and swept 40 times;
- no allotment split in 256-bit fixed point;
- no balance or shedding by light;
- no rough layout redrawn each cycle;
- no Q32 settle laid twice over tens of thousands of placeholder axes.

Its cost is one walk over the nodes it keeps, bounded by the envelope and the attractor count. The F4 engine buys botanical fidelity (age-stepped growth under light and expected fine roots) with work that is mostly finer than the 0.3 to 0.5 m cubes the site draws. That cost lands in exactly the passes named above. This last point is read from the code, not measured on 0.1.4, which exposes no stage timers.

## Method

- **Builds:**
  - `repo/` is a `git archive` of `ad67cc47` with spans and counters added. The diff is `instrumentation.diff`: `prof.rs`, the wasm32 clock reading `performance.now()` through an import, and spans at the stage boundaries.
  - `base/` is the same archive untouched.
  - Both are built `--release --target wasm32-unknown-unknown -p telperion-field` with strip, as `scripts/build-wasm.mjs` builds them, each in its own `target/`.
  - The instrumented grow is within +3 % of the pristine one (birch 867 against 874 ms, oak 1,405 against 1,367 and 1,020 against 1,021, spruce 352 against 345 and 333 against 326).
- **Wasm:**
  - `run.mjs` follows the site's call pattern through the slim ABI directly: `species_alloc`, `grow(seed, FAMILY_ORDER, 1024)`, the site's `grid` and `voxelize` from fn-214's `voxelize.js`, and the dials oak 0.08 / tuft 0.4, birch 0.08 / tuft 0.32, spruce 0.1 / density 0.8.
  - Node v26.8.1, one Node process per tree, three repeats, medians (`wasm.jsonl`).
  - 0.1.4 is the site's own `node_modules/telperion/dist/telperion-field.wasm` (`v014.jsonl`).
- **Native:**
  - `repo/crates/siteprof-native` calls the same C ABI through the `telperion-field` rlib, with the core built without geometry, on the release profile (fat LTO, as Wasm).
  - `taskset -c 6` and `RAYON_NUM_THREADS=1`. Under the affinity mask `available_parallelism` is 1, so `cores::available()` is 1 and the engine runs its one-thread paths.
  - Three repeats in one process (`native.jsonl`). The waste counts come from a fourth, untimed `pipeline::build`.
- **Load:** 1.8 to 4.0 on 32 cores. No GPU was used.
- **Summaries:** `stages.py` writes `stages.md`, and `summary.py` prints every span.

Everything is in `/tmp/claude-1000/-home-daniel-Projects-telperion/dd1a391e-8087-41d5-8d26-b3ea78d9cb37/scratchpad/siteprof/`.
