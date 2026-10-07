# fn-198 task 1: the engine inside the pipeline, a survey and a proposal (worker, 2026-10-06)

This file is the overnight scope's step 1. It changes no product code. The survey records what the code does today. The proposal sets out options for the host to decide before step 2 is built.

Every code claim was read on this branch (`fn-198-tree-space-f-the-engine-in-production`, at `c11e5d85`: master with F1 fn-209 and F2b fn-211 merged). Each claim cites file:line. A **measured** claim was run on this branch. Anything not checked is marked **unknown**.

## In brief

- **A preset reaches stage 2 as a `Family`, and stage 2 is today's grower.** Both entries funnel into one private function, `pipeline::skeleton` (`crates/telperion-core/src/pipeline.rs:169-176`). It is fed by `GrowInput` (`pipeline/input.rs:33-47`): the skeleton, radii and canopy rows.
- **The engine has no place in a preset today:**
  - Its species are Rust constructors (`crates/telperion-space/src/{beech,oak,spruce,palm}.rs`).
  - The dressing rows the examples overlay are JSON literals in example code (`examples/space_{beech,oak,spruce}.rs:14`).
  - The trunk ages are constants in example code (`examples/space/tree.rs:93-99`).
  - The oak's light is a command-line flag (`examples/space/tuning.rs:37-41`).
- **A preset holds scalars only, and finite ones.** The catalogue's 250 wire rows are scalars at fixed paths. The grammar and the bounds both refuse a non-finite value (`presets/grammar.rs:56-57`, `catalogue.rs:136`).
- **An engine species is about 2,000 to 2,100 scalars across 15 ages.** Of those, 75 to 1,398 sit in the ages a species lives in. Each species sets `shedding = ∞` on 6 to 15 ages (**measured**, `wasm-probe/species-count.rs`).
- **All four of GPU-PATH.md's missing pieces are confirmed in code** (section 2.5).
  - Two more were found that GPU-PATH.md does not name:
    - The European beech is not in the shipped catalogue at all (`presets.rs:42-43`), so the harness cannot open it.
    - The leaf cull and the wood's twist read the authored envelope, which the engine does not grow into (section 2.8).
- **`telperion-space` compiles for wasm32 as it is, and `grow` runs there (measured):**
  - It runs on one thread there.
  - The fine path traps: `Expected::new` and `grow_fine` call `std::time::Instant::now()`, which traps on `wasm32-unknown-unknown`.
- **The 4 GB ceiling (measured in Node).** Each figure is the peak of the Wasm memory, through growth, conversion, expansion, rings, stations and curve: the CPU work `setTreeGpu` would do.

  | Species | Age 40 | Age 80 |
  |---|--:|--:|
  | palm | – | 4 MB |
  | beech | 68 MB | 1.48 GB |
  | oak | 196 MB | 1.98 GB |
  | spruce | 1.46 GB | **traps at 3.9 GB** in the stations step |

- **The engine's code is about 466 KB of Wasm (measured).** It is 521 KB standalone on the workspace's release profile, against a 55 KB std floor. The shipped modules have 190 KB (main), 215 KB (render) and 37 KB (field) of headroom under the owner's ceilings (`scripts/artifact-budgets.json`). Reaching the engine from every module crosses every ceiling. Crossing a ceiling is an owner decision.

## 1. How checked

- **Code:** every file:line below was read on this branch.
- **The Wasm probe:** `wasm-probe/` (the source; it runs from scratch with its own `target/`).
  - `lib.rs` is a `cdylib` built `--release --target wasm32-unknown-unknown`. It depends on `telperion-space` and `telperion-core` (with `json`) by path, and includes `examples/space/tree.rs` for the conversion.
  - Per species and age, it grows the tree, converts it and expands it with the preset's rows overlaid by the example's `ROWS`. It then runs the GPU path's CPU steps in `prepare_async`'s branch order: compact rings with or without contacts, stations, then the curve; the palm, which takes the CPU fallback, runs `Expansion::mesh` instead.
  - It reads `memory.size` after each step. Wasm memory never shrinks, so every figure is a high-water mark.
  - `run.mjs` runs one tree per Node process: `node run.mjs <0 palm|1 beech|2 oak|3 spruce> <age> 1`, or `9` for `grow_fine`.
  - Inputs:
    - seed 1;
    - budget 20M, as `still.rs:21`;
    - the oak under light 0.5, 0.5, as BASELINE.md;
    - the others under neutral light.
  - The raw lines are in `raw/wasm-matrix.log` (gitignored).
  - **Load:** the 1-minute load was 14 to 15 on 32 threads, so the times below are indicative only. Memory does not depend on load.
- **Engine size:** `wasm-probe/engine-size.rs` exports one function that grows any of the four species. It was built with the workspace's release profile (`lto = true`, `codegen-units = 1`, `Cargo.toml` at the root), and compared against a module of std, the allocator and formatting alone.
- **Species sizes:** `wasm-probe/species-count.rs` flattens each species' public fields per age and counts them (output in `raw/species-count.log`).

## 2. Survey

### 2.1 How a preset reaches stage 2 today

**Native**
1. `pipeline::build(&Family, Request)` (`crates/telperion-core/src/pipeline.rs:188-199`) calls `skeleton(GrowInput::of(family))`.
2. It then clothes the tree and runs `outputs` on it.

**The GPU executor**
1. `Generator::prepare` (`crates/telperion-render/src/generation.rs:211`, native only) blocks on `prepare_async` (`generation/preparation.rs:4`).
2. `prepare_async`'s first step is `executor::grow(family)` (`preparation.rs:14`), which calls the same `super::skeleton(GrowInput::of(family))` (`pipeline/executor.rs:34-38`).
3. Everything after `preparation.rs:17` works on the `Expansion` (`x`).

**The stage itself**
- `skeleton` (`pipeline.rs:169-176`) is `branching::crowned(skeleton, radii, crowned)`: today's grower.
- It is `pub(crate)`. The `branching` module is private (`pipeline.rs:213`).

**What stage 2 reads.** `GrowInput` (`pipeline/input.rs:33-47`) lends three things:
- `&SkeletonParams`;
- `RadiusParams`;
- `&CanopyParams`, only for its frond crown (`rosette_fronds > 0`, `pipeline.rs:170`).

Nothing else reaches stage 2.

**What the later stages read.** `Inputs::of` (`input.rs:82-116`):
- The plan, the surface and the leaves read the element, the envelope, the canopy and the surface.
- They read the seed as `skeleton.seed: u32` (`input.rs:56`, `77`).
- The twig placement comes from `skeleton.twigs` (`input.rs:85`, `120-125`).

**The second door, `executor::expand(Tree, &Family)`** (`executor.rs:43-46`), takes a solved tree.
- `docs/pipeline.md:48` sanctions it for "the renderer's tests of the GPU kernels on hand-built trees" only.
- Its only callers outside tests are the tree-space examples: `examples/space/still.rs:220` and `examples/space_walk.rs:81`.

### 2.2 The preset's format and its generation

**The value files**
- A preset is `crates/telperion-core/presets/<id>.values`.
- Each line is a wire path, `=`, and one value: `true`, `false` or a decimal (`src/presets/grammar.rs:1-5`).
- A non-finite number is refused (`grammar.rs:56-57`).

**Compiled into tables**
- `build.rs` compiles each file into a `const` table of rows (`build.rs:27-47`).
- Every row is checked against the catalogue as the crate compiles (`src/presets/table.rs:31-54`, `65-86`). The check refuses a path the catalogue lacks, a value of the wrong kind, and a value off its bounds.
- `Bounds::admits` requires a finite value (`catalogue.rs:136`).
- `table::family` lays the rows over `Family::default()` (`table.rs:99-105`).

**The catalogue**
- It is declared with `rows!` across 14 groups (`grep "catalogue::rows!"`: `family.rs:13`, `branching.rs:33/118`, `twigs.rs:7/86` and others).
- It holds **250 wire rows** (`wire: 0` to `249`). Every row is a scalar inside a fixed struct; none is indexed.
- Each row carries:
  - its path, unit, bounds and meaning;
  - its check site;
  - a growth note;
  - its blend;
  - its dial.

**The wire**
- `params::metadata(&Family)` writes every row at its path as JSON (`params.rs:18-28`).
- `params::parse` reads either a whole wire object or a preset identity string (`params.rs:29-41`). An identity resolves through `by_identity` (`presets.rs:47-59`).
- `params::overlay` lays a partial object over a family's wire and parses it back, refusing unknown keys (`params.rs:91-114`). The examples dress the engine with it.

**The shipped catalogue**
- `CATALOGUE` lists 7 identities (`presets.rs:24-37`).
- **The European beech is not one of them:**
  - `IN_WORK` holds it "until fn-62 accepts it" (`presets.rs:38-43`).
  - `by_identity` refuses anything outside `CATALOGUE` (`presets.rs:48-50`).
  - `Preset::from_id("european-beech")` still works natively (`presets.rs:80`), which is how the examples reach it.

**`presets.generated.ts`**
- `scripts/build-wasm.mjs` builds `telperion-wasm` for wasm32 (`build-wasm.mjs:4`) and calls its `catalogue()` export.
- It reads the catalogue JSON from the module's memory (`build-wasm.mjs:8-9`) and writes `src/browser/presets.generated.ts` as `CATALOGUE: {abiId, id, name, note, family}[]` (`build-wasm.mjs:14-22`).
- That file is 54,107 bytes today and holds 7 ids, with no `european-beech`.
- The harness opens a preset by `presetById(id)` over that array (`src/browser/core.ts:8-12`).

### 2.3 How the engine's species are defined today

**The types**
- `telperion_space::Species { states: Vec<PaState> }` (`crates/telperion-space/src/species.rs:58`).
- `PaState` has 27 scalar settings, a `Form` of 12 (`species.rs:65`, `189`), and `zones: Vec<Zone>`.
- Each `Zone` (`species.rs:269`) is made of:
  - a `NodeLaw` (`species.rs:292`): an enum, `Uniform { min, max }` or `Poisson { mean }`;
  - `buds`, `delay` and `rate`;
  - `lateral: Vec<f64>` and `dormant: Vec<f64>`, one entry per age on the chain.

**The chain**
- Every species is written on one chain of 15 ages, `chain::AGES` (`chain.rs:12-30`).
- `on_chain` fills each age a species does not live in with a passed-through state, then makes the species canonical (`chain.rs:76-82`, `species.rs:331-351`).

**The species are Rust functions**
- `beech()`, `oak()`, `spruce()` and `palm()` each build that vector in code (`beech.rs:81`, `palm.rs:33`).
- Each value's source sits in the species spec's SOURCES.md, cited in the file's header (`beech.rs:1-7`, `palm.rs:1-8`).

**How big a species is (measured, `raw/species-count.log`)**

| | Ages lived | Zones per age | Scalars, all 15 ages | In lived ages | Non-zero in lived ages | `shedding = ∞` | Poisson zones |
|---|---|---|--:|--:|--:|--:|--:|
| palm | 1 (age 2) | 1 | 1,125 | 75 | 15 | 15 ages | 0 |
| beech | 9 | 1 to 4 | 2,097 | 1,251 | 277 | 9 ages | 0 |
| oak | 10 | 1 to 3 | 2,133 | 1,398 | 377 | 6 ages | 0 |
| spruce | 9 | 1 to 3 | 1,989 | 1,251 | 262 | 9 ages | 0 |

- With the zones padded to the four roles (`species.rs:16-20`), one age has 39 + 4 × 36 = 183 scalars, and a species has 2,745.

**The engine's own metadata**
- `species/scale.rs` names every setting once and gives each a mixing scale: Linear, Log above a floor, Kept for a possibly infinite delay, or Outcome (`scale.rs:42-90`).
- `settings` and `zone_settings` list the fields by name (`scale.rs:131`, `222`). The module is `pub(crate)` (`species.rs:9`).
- `blend` walks two species with it (`blend.rs:15`), and refuses a Poisson zone (`blend.rs:55`).
- `Species::validate` refuses every input the engine cannot draw, by name (`species.rs:397-409` and `species/validate.rs`).

**What a grow takes beyond the species.** `Request { age: u32, seed: u64, budget: u32, light: Light }` (`grow.rs:40-46`):
- The light is "a site's settings, not a species'" (`light.rs:30-42`).
- The oak's light-reading values are in the species (`oak.rs:87-100`). The site's extinction 0.5 and sky 0.5 are not: they come from `--light` (`tuning.rs:37-41`; BASELINE.md:20).

### 2.4 What the tree-space examples add between the engine and the pipeline

**The dressing rows**
- Each species is dressed by an existing preset's rows, overlaid by a `ROWS` literal (`space_beech.rs:14`, `space_oak.rs:14`, `space_spruce.rs:14`; the palm passes `"{}"`, `space_palm.rs:19`).
- The overlays set `canopy.shortShootSpacing = 0` and `canopy.limbClumping = 0`, a `shootRadius` (beech, oak), and `skeleton.twigs.twig.internodeLength`.
- The shipped beech sets `shortShootSpacing = 0.03` and `limbClumping = 0.25` (`european-beech.values:109`, `120`). These are the rows that make the shipped beech fail `plan::supports` (GPU-PATH.md, item 5).

**The run**
- `still.rs:194-199` builds the `Request` from the command line: age, seed, `BUDGET = 20_000_000` (`still.rs:21`), and light from the tuning flags.
- The family's 32-bit seed is folded from the engine's 64-bit one (`still.rs:245-249`).
- Then it converts (`still.rs:213`) and expands (`still.rs:220`).

**The conversion** (`examples/space/tree.rs`)
- `convert(&Structure, trunk: &[usize]) -> Tree` (`tree.rs:107-240`) makes one node per phytomer and turns z-up to y-up (`tree.rs:20-22`).
- It reads two things that are not in any species or preset:
  - **The trunk ages per species** (`tree.rs:93-99`): `OAK_TRUNK`, `BEECH_TRUNK`, `SPRUCE_TRUNK` and `PALM_TRUNK`. The examples pick one per species by name (`space_walk.rs:27-34`). A walk takes the union of the two ends (`space_walk.rs:76-80`).
  - **Five constants with no owner named:**
    - `STRUCTURAL` 0.05: where fine wood begins, as a share of the root radius;
    - `FINEST` 2e-4 m: the thinnest wood drawn;
    - `SHORTEST` 1e-3 m: the shortest internode kept;
    - `FORK_FROM` 0.4 and `FORK_AT` 0.7: where a lateral starts and ends being a codominant fork (`tree.rs:14-18`).
- The doc comment at `tree.rs:1-9` gives the reasons for `FINEST` and `SHORTEST` (float32 rings), but no owner or measurement for the values.
- It sets `crossover = nodes.len()` (`tree.rs:234`): the whole tree counts as above the crossover.

### 2.5 GPU-PATH.md's four missing pieces, checked

1. **`prepare` takes no grown tree.**
   - **Confirmed.** `prepare_async(&self, family: &Family, delivery)` (`preparation.rs:4`). Its first step is `executor::grow(family)` (`preparation.rs:14`), and the rest works on `grown.expansion()` (`preparation.rs:17`).
   - No `Generator` method takes a `Tree`, a `Structure` or an `Expansion` (`grep "pub fn\|pub async fn" generation.rs`).
2. **`executor::expand` is a test-only door.**
   - **Confirmed by documentation and callers.** `docs/pipeline.md:48` names its purpose; its only product-adjacent callers are examples (section 2.1).
   - **Not enforced by visibility:** it is `pub` (`executor.rs:43`), so the boundary holds by documentation alone.
3. **`tree::convert` is example code, and `telperion-space` is a dev-dependency.**
   - **Confirmed.** `crates/telperion-render/Cargo.toml:33-36` (`[dev-dependencies]` … `telperion-space`).
   - `telperion-core` does not depend on `telperion-space`, and `telperion-space` depends only on `rayon` (`crates/telperion-space/Cargo.toml:11`). So `core → space` would add no cycle.
4. **`setTreeGpu` and the Wasm binding.**
   - **Confirmed.** `set_tree_gpu(&self, family: &str)` (`crates/telperion-render/src/web/generation.rs:13`) parses family JSON with `params::parse` (`:16`), then calls `prepare_async(&family, Delivery::Resident)` (`:41`) and `submit_prepared` (`:56`).
   - The harness calls it with the whole family (`harness/GrowerDev.tsx:78`, `harness/family.ts:89-109`).

**Also confirmed.** The GPU executor's fine growth is compiled for tests only: `#[cfg(test)] mod fine;` (`crates/telperion-render/src/generation.rs:9-10`).

### 2.6 What crosses the Wasm boundary

- **The render module** (`telperion-render`, built by `scripts/build-render.mjs:34`, glued by wasm-bindgen):
  - `setTreeGpu` takes one string, a wire family object or a preset identity (`params.rs:30-31`).
  - It returns a JSON string: what was submitted, the backend, the stage times and the buffer bytes (`web/generation.rs:45-64`).
  - The tree, its expansion and its buffers stay inside the module and on the device; none of it crosses.
- **The main module** (`telperion-wasm`) exports:
  - a request buffer (`request_alloc`, `request_ptr`; `crates/telperion-wasm/src/lib.rs:72-85`);
  - the catalogue's JSON (`metadata_ptr`, `metadata_len`, `catalogue`; `:89-105`);
  - `build` (`:130`);
  - typed result slots (`buffer_ptr`, `buffer_len`; `:147`, `:235`);
  - the field queries (`:276-314`).
- **The shipped presets enter TypeScript** through the main module's catalogue JSON, as `presets.generated.ts` (section 2.2).
  - Whatever a preset holds crosses there and then crosses back on every `setTreeGpu`.
  - An engine species of 2,745 scalars in the wire would add an **estimated** 50 to 70 KB per family per call, at about 20 to 25 bytes a key and value. This was not measured.

### 2.7 The engine on wasm32 (measured)

- **It builds as it is.** `cargo build --release --target wasm32-unknown-unknown -p telperion-space` finished cleanly on this branch (17 s, own `target/`).
- **Threads:**
  - `cores::available()` is `available_parallelism()` or 1 (`cores.rs:12-15`). In Node it ran single-threaded.
  - rayon-core 1.13 falls back to the current thread where threads are unsupported (`rayon-core-1.13.0/src/registry.rs:209-227`).
  - `spread` runs every piece on the calling thread when one helper is available (`cores.rs:19-24`).
- **`grow` runs.** All four species grew at 40 and 80.
- **The fine path traps.**
  - `run.mjs 9` (`grow_fine` on the spruce at 10) trapped with `unreachable`, and a bare `Instant::now()` export traps the same way.
  - `Expected::new` calls `std::time::Instant::now()` (`bounded.rs:57`), as do `Bounded::with` (`bounded.rs:121`), `grow_fine` (`fine.rs:146`) and the fine executor's `expand` (`fine/executor.rs:60`).
  - The pipeline already avoids this: `pipeline::clock` returns 0 on wasm32, and a caller passes its own clock (`pipeline.rs:80-96`).
- **`grow_fine` refuses any species that reads light**, so the oak cannot take it (`grow/alone.rs:91-95`; `fine.rs:95`). The spec's architecture paragraph cites fn-211's DESIGN-INTERLEAVE.md for the oak's year-by-year interleave. **No file of that name exists on this branch**, so the state of that design is unknown.

**Memory and time per species and age.** Peak Wasm memory after each step, in MB, cumulative. Times are wall ms at a load of 14 to 15.

| | Age | Phytomers | Nodes | Grown | Converted | Rings | Stations | Curve | Grow ms | Prep ms |
|---|--:|--:|--:|--:|--:|--:|--:|--:|--:|--:|
| palm (CPU fallback mesh) | 80 | 960 | 961 | 1 | 2 | – | – | 4 | 6 | 24 |
| beech | 40 | 113k | 102k | 36 | 55 | 55 | 68 | 68 | 96 | 97 |
| oak (light 0.5, 0.5) | 40 | 191k | 158k | 171 | 196 | 196 | 196 | 196 | 683 | 151 |
| spruce | 40 | 1.18M | 1.15M | 525 | 827 | 827 | 1,456 | 1,456 | 1,868 | 1,576 |
| beech | 80 | 1.60M | 1.53M | 545 | 847 | 847 | 1,477 | 1,477 | 1,736 | 1,967 |
| oak (light 0.5, 0.5) | 80 | 1.10M | 869k | **1,983** | 1,983 | 1,983 | 1,983 | 1,983 | 10,119 | 995 |
| spruce | 80 | 4.78M | 4.64M | 1,856 | 3,064 | 3,064 | **trap** | – | 8,952 | – |

- **The spruce at 80 cannot reach the GPU in wasm32 as built.** It trapped in the stations step (`compact_with_contacts` then `compact_stations`), with the memory at 3,903 MB.
- **The oak at 80 peaks in growth**, as natively (GPU-PATH.md: 2.02 GB). Its light lattice dominates.
- **The figures are a lower bound for the harness.** The probe drops the `Structure` after conversion and holds no renderer state. Whether a browser tab grants the full 4 GB to the render module next to the page is **unknown**: only Node was run.
- **What the probe does not cover:**
  - the GPU executor's own uploads and buffers (device memory, not Wasm memory);
  - any figure at ages other than 40 and 80. A species' default age in the harness is not stated anywhere checked (**unknown**; `Family.age` is 120 for the beech and 100 by default, `european-beech.values:6`, `family.rs:54`).

**Code size.**
- The engine's `grow`, with all four species reachable, is **521,027 bytes** standalone, built with LTO. A module of std, the allocator and formatting alone is 55,011 bytes, so the engine is about 466 KB.
- Headroom under the ceilings (`scripts/artifact-budgets.json`), each figure the ceiling minus the recorded baseline:

  | Module | Ceiling | Baseline | Headroom |
  |---|--:|--:|--:|
  | `telperion.wasm` | 1,600,000 | 1,410,330 | 189,670 |
  | `telperion-render.wasm` | 2,200,000 | 1,985,291 | 214,709 |
  | `telperion-field.wasm` | 400,000 | 363,305 | 36,695 |

- **What each module would actually gain is unknown until it is built.** Monomorphisation and LTO decide what is shared with the code already in each module. Today's grower, if removed (R0), would give some back.

### 2.8 Further findings the integration meets

1. **The shipped European beech does not ship.** It is `IN_WORK` (`presets.rs:42-43`), so the harness cannot open it (2.2). "The four species open in `npm run dev`" needs either the owner's word to ship it, or a door for in-work presets.
2. **The cull and the wood's twist read the authored envelope.**
   - `Inputs::of` takes the envelope from `skeleton.envelope` (`input.rs:84`) for three uses:
     - the surface's height, which the twist rate is a share of (`input.rs:104-107`; `docs/pipeline.md`, "The wood's curve");
     - the leaves' cull, which reads `envelope.max_radius()` and `envelope.profile()` (`foliage.rs:255-266`);
     - the plan's reach.
   - An engine tree's height and crown are grown, not authored. R1 removes the foliage cull on the engine's path. The twist's height and the plan's envelope are not named anywhere checked.
3. **`Family.age` is read by no build** (`family.rs:16-19`). The engine needs an age (`Request.age`, `grow.rs:41`).
4. **The seed differs between the two sides.** The family's is `u32` (`input.rs:56`); the engine's is `u64`. The examples fold one into the other (`still.rs:245-249`).
5. **The slim field module builds through stage 2 too.**
   - `telperion-field` takes the core without geometry (`crates/telperion-field/Cargo.toml:14`), and its build runs `pipeline::skeleton` through the planned outputs (`pipeline.rs:188-210`).
   - If the engine is stage 2, the field module carries it. Its headroom is 37 KB.
6. **The no-geometry build (fn-186) skips the expansion.** `pub mod executor` is behind `geometry` (`pipeline.rs:231-234`). Fine growth placed "in the expansion" therefore never runs on a no-geometry build. Whether a field built from the structure above the cut is acceptable is **unknown**.

## 3. Proposal

Each option names its trade-offs. The host decides. Where an option's claim was not checked, it says so.

### 3.1 The value format of an engine species inside a shipped preset

**A. Catalogue rows in the preset's own `.values` file, under `/space/...`**

The catalogue would gain indexed rows. One example of the shape: `/space/ages/4/form/pipe = 0.008` and `/space/ages/4/zones/2/lateral/9 = 0.35`.
- **For:**
  - One file per preset and one wire.
  - The harness's dials, `overlay` and the generated TypeScript carry it unchanged.
  - Every row gets bounds, a meaning and a check, as every other row does.
  - The engine's `SCALES` (`scale.rs:42`) gives each row its blend.
- **Against:**
  - The catalogue grows from 250 rows to about 3,000.
  - `rows!` has no indexed rows, so the macro, `locate` (`catalogue/mount.rs:87`), the browser metadata and `docs/parameters.md` all change.
  - The infinite `shedding` needs a finite encoding: the `Kept` share `1/(1+v)`, the scale blend already uses (`scale.rs` "Kept"), with 0 meaning never shed.
  - The `NodeLaw` enum must become values, for example a uniform law plus a Poisson weight. Today `blend` refuses Poisson (`blend.rs:55`) and no species uses it.
  - Each family on the wire carries the full table. That is an estimated 50 to 70 KB per `setTreeGpu`, not measured.

**B. A second value file per preset, `presets/<id>.species`, in the engine's own table form**

It would use the same grammar with paths of the engine's own `settings` names (for example `4/form.pipe = 0.008`). `build.rs` would compile it into a `Species` constant beside the preset's rows. A preset without one has no engine species.
- **For:**
  - The family's 250 rows are untouched.
  - The engine's `Species::validate` (`species.rs:397`) and `SCALES` stay the one place that judges and blends a species.
  - A file states only the lived ages' values, and `canonical()` derives the rest (`species.rs:331`).
  - The file is about 75 to 400 rows per species (section 2.3).
- **Against:**
  - Two value files per preset.
  - The harness's family wire does not carry it, so the harness can neither show nor move it without a second wire.
  - The infinity and enum questions are the same as in option A.

**C. The species stays Rust code; the preset names it by a scalar row such as `/space/species = 2`**
- **For:** the least work.
- **Against:**
  - It is a species branch by name in code (AGENTS.md: "presets are value tables; no species or template branch in generator or renderer").
  - The species could not be authored, walked or shipped as values.
  - This option breaks the overnight scope's step 1 as written. It is listed only to be rejected.

**What goes with any of the three.** These are values today held in example code or in flags:
- the dressing rows (the `ROWS` overlays become the preset's own rows, since today's grower is removed);
- the trunk ages: a per-age stem weight in 0 to 1 would blend, where a set does not;
- the site's light (extinction and sky);
- the age, from `Family.age` or an engine row;
- the budget.

### 3.2 The door by which stage 2 grows through the engine

**i. `pipeline::skeleton` becomes the engine (recommended shape to judge)**
- `GrowInput` would carry the preset's species and request values in place of `SkeletonParams` and `RadiusParams`. The engine would then grow, and the conversion (section 3.3) run, all inside the private stage.
- `executor::grow` and `pipeline::build` keep their signatures (`executor.rs:34`, `pipeline.rs:188`), so `prepare_async`, `setTreeGpu` and the main and field bindings change no entry.
- A preset with no species fails in `GrowInput`'s derivation with a named error. That follows the input's rule of "a derivation that can fail is kept as its result" (`input.rs:1-6`).
- **For:** one door. No second chain. The boundary tests stay as they are.
- **Against:** the whole tree is grown on the CPU before any expansion. F1's fine roots move under `pipeline::executor` later, as `docs/pipeline.md` "Fine growth" already plans.

**ii. Stage 2 grows above the cut; the expansion grows the fine roots (F1's architecture now)**
- `Grown` would carry the structure above the cut plus the `FineRoot` records. `Grown::expansion` would run `grow_fine`'s executor, on the CPU reference or the GPU.
- **For:** this is the spec's revised architecture paragraph.
- **Against:**
  - `grow_fine` refuses the oak (`grow/alone.rs:91-95`), so the oak could not build this way.
  - The fine path traps on wasm32 (2.7).
  - The GPU fine executor is `cfg(test)` and ends in device buffers no reader consumes yet (`generation.rs:9-10`; `docs/pipeline.md`, "Fine growth"). The spec's step 2 says "fn-209's executors as they land".
- A staged route is open: build option i now, with `GrowInput` shaped so that option ii replaces only the body.

**iii. A new executor entry that takes a grown `Structure`**
- **Against:** it is the second chain principle 1 forbids, as `expand` already nearly is (2.5 item 2).
- **Recommended for deletion:** close `expand` with the handed-in path (spec, step 2). The renderer's kernel tests on hand-built trees could keep a `#[cfg(test)]`-only door. Whether they can is **unknown**: `expand`'s test callers were not counted.

### 3.3 Where conversion lives

**a. A private stage module in `telperion-core` (`pipeline/space.rs` or similar), with `telperion-space` a normal dependency of the core**
- **For:** conversion is a stage, and stages are private to the pipeline (`docs/pipeline.md`), so it sits where the boundary already holds.
- **Against:**
  - Every consumer of the core links the engine: the main, render and field Wasm modules, plus `telperion-jev` (section 2.7, code size).
  - It could be gated behind a feature only if the field module may keep today's grower. "One pipeline" rules that out (spec, "One pipeline").

**b. In `telperion-space`, producing the core's `Tree`**
- **Against:** the engine would depend on the pipeline's types, and its oracle tests would compile against the core. It also puts a stage outside the pipeline's private modules, the thing `docs/pipeline.md` forbids.

**c. A new crate between the two**
- **Against:** a third crate for one function. It would also be outside the pipeline's privacy.

**For any of the three:** the five constants of `tree.rs:14-18` need an owner and a reason (`docs/principles.md`, "Every requirement has a name"). Otherwise each needs a measurement or a deletion.

### 3.4 The harness's `setTreeGpu` and the Wasm binding

- **Under 3.2 i with 3.1 A**, `setTreeGpu` needs no change. It takes the family JSON, and the engine runs inside `prepare_async`'s `executor::grow`.
  - Under 3.1 B, it needs a second argument or a preset identity. `params::parse` already accepts an identity (`params.rs:30-31`), but the harness always sends the whole family (`family.ts:89-109`).
- **Wasm32 as it is:**
  - the engine builds;
  - `grow` runs single-threaded;
  - `Instant::now()` in `bounded.rs`, `fine.rs` and `fine/executor.rs` traps. The fix is a clock passed in, as `pipeline::Request::clock` does (`pipeline.rs:80-96`). That fix is needed before any fine path runs in the browser.
- **The 4 GB ceiling** (measured, section 2.7):
  - The palm, the beech at 40 and 80, the oak at 40 and 80, and the spruce at 40 all fit, at 4 MB to 1.98 GB.
  - The spruce at 80 does not.
- **Options for the spruce at 80** (the host's call):
  1. **Refuse by name in wasm32** above a measured size, never a fallback. The harness's default age for the spruce would sit below that size.
  2. **Cut the preparation's memory first.** The stations step took 629 MB on the spruce at 40 and was the step that trapped at 80; dropping the converted `Tree`'s CPU copy after upload is one lever. Neither lever was measured.
  3. **Wait for F1's fine growth on the device.** The CPU then never holds the fine roots. Not measured; the GPU fine executor is not wired (2.5).
- **The size ceilings:** reaching the engine from all three modules crosses every ceiling, by the estimate in 2.7. That is the owner's decision, per `docs/principles.md`, "The size budget".

### 3.5 Decisions for the host before step 2

1. **The value format:** A or B (C rejected). With it:
   - the encoding of `shedding = ∞`;
   - whether `NodeLaw` becomes values;
   - the trunk ages as stem weights;
   - where light, age and budget live.
2. **The stage 2 door:** i now, with ii's shape reserved; or ii directly. If ii, the oak's light refusal and the wasm32 clock must be answered first.
3. **Conversion's home:** a, b or c. With it, the owners of the five conversion constants.
4. **The European beech:** ship it (fn-62's acceptance, the owner's word) or open in-work presets in the dev harness.
5. **The default age per species in the harness**, and the spruce at 80 in the browser (3.4).
6. **The size ceilings:** the owner's decision once the modules are built with the engine.
7. **The authored envelope** on the engine's path: the twist's height and the plan's envelope (2.8, item 2).

## Host decisions (2026-10-06, overnight; owner: "be bold")

1. **Value format: (A)**, indexed rows in each preset's own `.values` file; presets stay value tables, one per preset. Unbounded shedding delays are the grammar's named token `never` (a typed value, not a magic number); the trunk ages, the oak's growing light (extinction and sky) and the default age are preset values; `Family.age` is read by the build.
2. **Stage 2: door (i) now.** The private `pipeline::skeleton` becomes the engine (today's whole-tree CPU grower with F2b's levers); no entry signature changes; door (ii), fine roots in the expansion, follows when fn-209's executor and the oak's light path land. Not (iii).
3. **Conversion** is a private module of core's pipeline; telperion-core depends on telperion-space; each of the five conversion constants gets a named owner and a doc line where it is defined.
4. **The beech ships** on this branch (passed by the owner; `IN_WORK` lifted).
5. **Harness:** default age 40 for every engine species, the age slider to 80. In the browser, a species and age whose CPU preparation cannot fit the 4 GB Wasm ceiling is refused with a named error, never a trap (the spruce at 80 measured 3.9 GB); fn-209's fine path lifts the ceiling.
6. **Size ceilings:** raised on this branch to the measured sizes and flagged for the owner (an owner decision; nothing merges).
7. **The envelope:** the pipeline derives it from the grown structure for every tree; the presets' hand-set envelope rows are removed.
8. **The browser clock:** `Instant::now()` leaves every code path the Wasm build runs (timing stays in examples and probes).
9. **Presets without an engine species** (silver birch, laurelin, telperion, ordinary) are refused by name at stage 2; tests that build them become expected-refusal tests on this branch, listed for the owner.

## Host decisions after task 2 (2026-10-07)

10. **Native against Wasm parity:** rounding-level for engine trees tonight (topology to the bit, positions within 1e-12 m, as `harness/parity.test.ts` holds). Moving the engine's transcendental call sites to the pinned library (fn-11's reason) is right and is done after fn-209 and fn-211 merge into this branch, to avoid three-way conflicts; trees change once then.
11. **Envelope shape rows** (shoulder, irregularity, lobe scale) are measured from the grown tree like height, crown base, spread and fullness, in that same follow-up; no family default stands in for a measured value.
12. **Trunk ages** become a 0-1 weight per age, not a boolean, so blending presets changes the tree by degree (docs/principles.md); same follow-up.
13. **The 25 ignored old-grower tests** stand until R0 removes the grower; no frozen fixture copies.
14. `executor::expand` stays for the renderer's GPU tests on hand-built trees; no engine structure can reach it.

## Task 4 (worker, 2026-10-07): what is built, and the GPU executor in stage 2, for the host

**Built.**
1. **F1 merged** (`8a1eafc1`, F1's tip `2975e8d5`). One conflict: F1 deleted `fine/executor.rs`, which this branch had moved to the engine's clock; F1's new stage clocks (`fine.rs`, `fine/realized.rs`, `fine/expansion.rs`, `fine/cases.rs`) were moved to `crate::clock` too (host decision 8). The engine crate differs from F1's tip only in those clocks. **Identical by lineage:** F1's own `examples/digests.rs` (32 trees: four species, 40 and 80, seeds 1 and 7, neutral and lit) prints F1's final digests and kept counts to the digit (`raw/task4/digests.txt` against F1's `raw/gpu-fast/digests-final.txt`). Stage 2 calls the same `telperion_space::grow`, so every engine preset's tree is F1's. Core and engine tests green after the merge (567 passed, 0 failed).
2. **Host decision 15** (`f4c2ad12`): `telperion_space::expected_kept(species, age)`, the predictor's table at the seed, equal to the closed form's seed row to the bit (test), and stage 2 refuses a tree expected over 4,000,000 kept phytomers in Wasm before growth. A species whose buds read light gets none (see BROWSER.md, "Task 4": the light-neutral count would refuse the lit oak at 80, which builds).

**Not built: stage 2 through `settle` with the GPU executor.** Two things stand in the way that the task's text does not decide, and one of them reverses a recorded decision.

1. **Nothing downstream reads fine roots on the device.** Stage 3 and 4 (descriptors, wood, leaves) are CPU code over the converted `Tree`. The GPU executor ends in device buffers (host decision 5 of FINE-CONTRACT.md: F4 with fn-171 defines device-side readers; option (c), reading fine geometry back, was rejected). "The same tree either way, by lineage" in the pipeline therefore needs the roots read back into a `Structure`: no code does that today (`fine/readback.rs` reads topology and laid indices for the tests, not tips and radii). Size: the fine roots hold 1.37M of the beech's 1.60M kept phytomers at 80 and 3.08M of the spruce's 4.78M (`harness/fine_share.rs`).
2. **The browser cannot block.** `settle` and `Roots` are synchronous, with 14 to 40 device round trips a tree; the GPU executor waits with `pollster::block_on` and `device.poll(wait)`, builds its tables and the reach rows on `std::thread::spawn`, and unwraps its reads. None of that runs in a browser's WebGPU. The whole GPU fine module is `#[cfg(test)]`; its region buffers take about 1.4 GB, whose fit under a browser's buffer limits is unknown.

**Options.**
- **(a) Read back, natively and in the browser.** The GPU executor's roots read back per phytomer (tip, radius, scale) and assembled as `grow_fine` assembles; `settle` and `Roots` made async (the CPU executor's futures ready at once), the threads replaced by in-turn work on wasm32, the reads made errors. Reverses decision 5's (c) until fn-171's readers land. Cost unmeasured; the readback is roughly 16 bytes a phytomer (50 MB for the spruce at 80) plus the assembly.
- **(b) Read back natively only.** As (a) for `Generator::prepare` on a native device; the browser keeps the CPU executor until the driver is async. Smaller; the browser gains nothing.
- **(c) Door (ii) proper: the device-side readers first.** `Grown` carries the structure above the cut and the roots' records; wood and leaves read fine roots from device buffers (fn-171's scope). No readback, the architecture the spec names; the largest piece of work.
- **(d) Hold.** Stage 2 stays on the CPU `grow` (the partition on all cores natively, one core in Wasm) until fn-211's stepped executor and fn-171 land.

**Also for the host.**
- **The merge is slower on one core** (9 to 41 percent, the oak's unchanged path included; TIMING.md, "Task 4"), which is the browser's case: the harness's beech and oak at 80 build 7 and 25 percent slower than in task 3. Natively on all cores the beech and spruce grow twice as fast. Not profiled.
- **The refusal's two counts.** The expectation is seed-free and runs up to 7 percent under the spruce's kept count, so at seed 42 the spruce at 71 and 72 still grows and is refused after growth (kept over 4,000,000), while 73 and older are refused before it. Options: keep both; raise the after-growth bound towards the measured trap edge (4.27M built, 4.48M trapped); or refuse on the expectation alone.
- **The canonical sort** in stage 2 re-orders `grow`'s lineage order into (depth, identity) for 0.14 s at spruce 80; FINE-CONTRACT.md's decision C says fn-198 adopts F1's order, which would delete it and move the conversion's bytes.

## Task 5 (worker, 2026-10-07): one core as fast as before, and one order

**The cause** (TIMING.md, "Task 5"). F1's host decision 8 made every presence a Q32 integer, so each draw's lead became `logit(threshold) - logit_draw(d)` through the contract's fixed-point `log2` (32 dependent 128-bit squarings: 65 ns natively, 145 ns in Wasm, where a 64-by-64-bit product is a call to `__multi3`), and each window an `exp` (up to 32 Q62 products: 97 and 141 ns). At spruce 40 on one core the engine's `log2` calls rose from 24,606 to 5,604,666 a tree and `exp2_neg`'s from 7,268 to 838,521: 441 ms natively and 1,048 ms in Wasm by the primitives' measured costs, against +422 ms and +1,167 ms measured, nearly all in the growth units' pass (276 to 618 ms). Three smaller costs came with F1: the lineage order (decision C, 33 to 85 ms at spruce 40), Q32 conversions in assignment and shedding (+20 to 30 ms at 80), and, on the partition's path alone, 49.3M allocations a spruce at 80 against the whole grower's 5.6M (each fine root's `Rules` copied with its tables, and its per-cycle lists made anew), which Wasm's allocator makes 700 ms. One thread grows on one core before and after (2 threads in the process: the caller and rayon's one).

**Fixed, every tree unchanged** (lineage digests of 32 trees and `measures hash 80` of 16, natively; Wasm digests of 7 against the unchanged Wasm build; the pipeline's 16 clothed hashes against the pre-task commit `58e08866`, `raw/task5/space-hash-*.txt`): a bud's lead taken only for a bud that is made; the windows' wood shares tabled once a tree; a log ratio's two logs interleaved; threshold log-odds kept per core; Q32 floors by the cast; Wasm's wide products from 32-bit halves; `log2` on a doubled mantissa; a presence's quotient in 64 bits where it fits; a draw's and a node's presence settled from grid bounds of their logs where the bounds settle it; the lineage order on compact keys and one gather; every fine root's grower sharing the tree's tables; a cycle's homes in one walk and its lists kept between cycles.

**Decision 17** (`8f3ce65d`): stage 2 converts the engine's tree as `grow` returns it; its (depth, identity) re-sort is gone. The converted bytes changed once: the beech's, spruce's and oak's clothed hashes move, node counts unchanged, the palm's do not (`raw/task5/space-hash-before17.txt`, `-after17.txt`).

**For the host.**
1. **Engine files to carry to F1** (`fn-209-tree-space-f1-fine-orders-grown-in-the`): `crates/telperion-space/src/{dormant.rs, fixed.rs, fixed/decide.rs, fixed/presence.rs, fixed/tests.rs, grow.rs, grow/advance.rs, grow/alone.rs, grow/parallel.rs, grow/shoot.rs, grow/stop.rs, grow/unit.rs, grow/wake.rs, lineage.rs, presence.rs}` and two under `src/fine/`, which F1's worker may touch: `fine/canonical.rs` (`lineage_order`) and `fine/tests/draws.rs` (`bud` returns the PA alone). `fine/tables.rs:51` still names `decide::node` in a doc line; the function is gone (`node_window` and the bounded node presences replace it). The GPU mirror (`contract.wgsl`) is untouched: `under`, `over`, `past_q`, `above_q`, `presence` and `log2` keep their values; the fast paths are new CPU functions whose results equal them (tests).
2. **The duplicate-identity refusal is gone with the re-sort.** Stage 2 refused "engine: two axes share a lineage"; `lineage_order` does not check. Options: (a) leave it, the engine's identities being unique by construction; (b) a debug assertion in `lineage_order`; (c) the check in `lineage_order`, at the cost of an identity sort.
3. **Two choices worth a look.** The threshold log-odds memo is a per-core `thread_local` table of 256 entries (a pure function's cache, empty in a fresh thread); Wasm's products by halves are chosen by `cfg!(target_arch = "wasm32")`, two routes to one integer, tested against each other natively.
