# A slim growth-and-field package for the homepage

## Conversation Evidence

> user: "I have an idea on how to get my homepage killenberger.com shipped more quickly with a tree. We can just use the generated field and make a voxel tree no? one that fits the black white aesthetic."
> user: "But for that we'd need the generated field to be lightweight enough to be included in killenberger.com We shouldn't drag in the full rendering thing."
> user: "HOw do you propose to go about this making an importable package that's efficient and fast"
> user: "no we don't want voxels output in telperion-core."
> assistant: "Dropping clumps is a styling choice for the homepage, so I would keep it in the consumer."
> user: "i agree with that last bit."
> user: "this pipeline restructure's performance must be faster or at worst equal to current performance."

## Goal & Context

<!-- Goal & Context: 60% [paraphrase], 40% [inferred] measurements -->

killenberger.com wants a black-and-white voxel tree, a different species and seed per visit, without shipping the renderer. The page needs three things from Telperion: grow a tree, prepare its plan, and answer occupancy queries. It draws the cubes itself.

Today's generator Wasm is 1,242,383 bytes raw and 344,254 bytes with brotli, measured on master `3fdfb440` on 2026-09-21. It carries the wood surface, leaf placement, materials, the specimen API and a JSON request path, none of which the page uses. This spec ships the same core built with only the stages the page reads, behind a small typed entry point.

## Architecture & Data Models

Each output stage of the core sits behind its own Cargo feature: wood surface, leaves, field, structure export, the specimen API and the JSON request path. The default feature set is today's full build. The slim build enables growth, the plan stage and the field only. [paraphrase]

The slim binding takes a preset id, a seed and nothing else as its request; presets are the value tables the core already compiles in. It exposes the field's bounds and the batch occupancy query. It does not depend on the JSON-enabled core or on any serialization crate, which today's binding pulls in unconditionally. Growth is the direct build's skeleton generation. [inferred]

The slim build stands on the foliage descriptor layer the field spec delivers, which compiles with no wood-surface and no leaf-placement code. A family that layer cannot describe is rejected by the slim build with the core's input error, because the fallback through leaf placement is not linked. [inferred]

## API Contracts

A typed entry point, separate from the package's main entry, offers one call: given a species id, a seed and an optional limb order it resolves to the field's bounds and a batch query that takes cell centres with a half extent and returns, per cell, the wood flag, the thickest wood radius in metres, the foliage flag, the leaf-count estimate and the owning limb id, as fn-100's field answers them. It runs unchanged in a browser worker and in Node, so a site can query at request time or bake grids at build time. [inferred]

Beside it, an example voxelizer (owner, 2026-09-22): a small typed module that takes the entry point's answers over a regular grid and returns cube lists for wood and foliage, with three dials, a wood cutoff in metres of radius, a foliage thinning rule with its keep fraction, and the limb order, carrying the rules the `experiments/voxel-field` prototype settled on fn-100 (coin, density, tuft, gap). It is an example a consumer may import or copy, never the only entry point; the field query stays primary. It draws nothing. [user]

## Edge Cases & Constraints

- An unknown species id rejects with the core's existing input error. The seed is an unsigned 32-bit integer; any other value rejects. [inferred]
- A batch whose length is not a multiple of four, or that holds a non-finite centre or a negative half extent, rejects whole and returns no partial answers. [inferred]
- The same species and seed give byte-identical answers across calls and across a release and rebuild. [inferred]
- Releasing a tree invalidates its query handle, as the full binding does today. [inferred]
- No species or template branch appears in the binding; the species id only selects a value table. [inferred]

## Acceptance Criteria

- **R1:** The slim Wasm, compressed with brotli at quality 11, is smaller than today's 344,254 bytes, and the package's total download for the slim entry point (Wasm plus JavaScript) is recorded beside it. The target is half of today's figure; nothing has been built, so the target is a feasibility gate: if the first build misses it, the worker records the largest retained symbols and stops with `NEEDS_HUMAN` instead of trimming further. [inferred]
- **R2:** The slim build is produced by an isolated feature build. An inspection of its retained code and its dependency tree shows no wood-surface, leaf-placement, material, specimen, JSON or serialization code, and a consumer that imports only the slim entry point bundles without the full module. [paraphrase]
- **R3 (amended by the host, 2026-09-22):** In Node, for the oak, birch and spruce at seeds 1 and 7, from species id and seed to a fully queried 64-cell grid runs at parity with the full binding's field-only build measured on the same code in the same minutes: no tree's warm median of five is more than 10% slower, the cold run is reported separately with module initialization, and the Node version and the machine are recorded. The first draft's 250 ms ceiling was inferred from the prototype's structure-export path (93 to 205 ms), which never ran the plan query; it was never measured before the spec was marked ready and is withdrawn. Measured: warm medians 349, 388, 406, 649, 289 and 273 ms, binding overhead 1 to 17 ms, the rest growth plus the plan query. A faster query (a flags-only answer that keeps the placed path's early exit) is a core follow-up, not part of this spec. [user constraint, host amendment]
- **R4:** The full build is unchanged: with default features the generator Wasm's exports and every existing binding test pass without edits. [inferred]
- **R5:** Smoke tests load the slim entry point in Node and in a browser worker, without relying on a bundler-resolved URL, and run one query each. [inferred]
- **R6:** The `experiments/voxel-field` script, rewritten on the typed entry point and the example voxelizer, reproduces the order-3 sheet the owner accepted under fn-100 R8 byte for byte at the same dials, and the wood-only sheet at 2 cm. [inferred]
- **R7 (owner, 2026-09-22):** The example voxelizer is typed, tested on a fixture grid for each rule and dial, and documented in one page under `docs/` that names it an example beside the primary entry point. [user]

## Boundaries

- The homepage's rendering, styling and clump thinning live in killenberger.com, never here. No renderer code ships in the slim build. [user]
- No voxel or grid type is added to the core. The package's entry point returns query answers; the example voxelizer is a separate module that consumes them and is not required to use the package. [user, amended 2026-09-22]
- Publishing to a package registry is the owner's step and is not part of this spec. [inferred]

## Decision Context

- Depends on "The field reads the plan, not placed leaves". Without it the slim build would still need leaf placement and could not meet R1 or R3. [inferred]
- Rejected: baking a fixed set of trees at build time as the only path. It loses a random seed per visit, which fn-91 names as the homepage's use. The same entry point still allows baking for a first paint. [inferred]
- Sibling from the same conversation: "One build pipeline; leaves expand from stations". This spec does not wait for it.

## Resolved before ready (host, 2026-09-22)

- The slim build's real size is settled by R1's own gate: the first isolated feature build measures it and stops with the retained symbols if it misses half of today's figure. fn-100 delivered the `geometry` feature and the plan-only core the build needs (`cargo build -p telperion-core --no-default-features` is green in CI).
- Resolution is a consumer choice, not a spec value: the owner looked at 64, 96 and 128 cells (fn-100 evidence, RESULTS.md and the comparison page) and the entry point takes any cell size. The example voxelizer's fixture uses 64.
- Depends on fn-100 as a chain parent: this spec branches from fn-100's tip until PR #55 lands.
