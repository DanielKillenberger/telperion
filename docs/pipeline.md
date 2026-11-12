# The pipeline boundary

Every tree is built by `telperion_core::pipeline`. Its stages are private to it, so a second chain outside the pipeline does not compile (fn-158; `docs/principles.md`, How they are held). What the pipeline exposes is what may be called:

- **`pipeline::build(&Family, Request)`**, the one build, with its request and outputs. `mesh::build` is the same build assembled into one mesh.
- **`pipeline::growth`**, the same growth held between years: `begin` a family's tree for a first age, `grow_to` any age, past it too (a year is a state, fn-355), and `settle`, `colouring` and `mesh` draw it at any age within its last year. The browser's grow-in and play-the-year and the native film tools grow through it.
- **`pipeline::executor`**, the grown tree and its expansion as tools and tests reach them. Its callers are the renderer's examples and tests and the core's own tests.

The stages live in private modules of the pipeline (`crates/telperion-core/src/pipeline/{branching,foliage,surface,...}`). Their data stays public as types, through `pipeline::contract`, at the paths consumers already name (`telperion_core::foliage::Element`, `telperion_core::surface::SurfaceMesh`): the parameter groups a family is made of and the artifacts a build returns. A type is a contract; a function that builds one is a stage and stays inside. The rest of the crate reaches only the family's own checks (`validate_skeleton`, `build_element`, `canopy_rows`, `height`), re-exported to it by name.

## Typed stage inputs

`pipeline::build` reads the family once and hands each stage a narrow input:

| input | stage | rows |
|---|---|---|
| `GrowInput` | the skeleton | the engine species, its stem ages and site light (`/space`), the age, the seed |
| `PlanInput` | the element, the leaf plan, the leaf box | element, the grown tree's envelope, canopy, surface, seed, the twig placement, the leaf box |
| `SurfaceInput` | the rings and the wood | the envelope's height, surface |
| `LeafInput` | placement | envelope, canopy, seed, shell depth (checked, unread since fn-212 R2) |

`GrowInput` lends the family's groups to the one stage that reads them; the others are owned copies with no reference back to the family, so an expansion outlives it. Family-level values are derived there once: the resolved twig rows, the twig placement and the leaf box read from them, the height. A derivation that can fail is kept as its result and answered where its stage runs, so the error a request meets is the one it met before, in the same order. Values that live at a branch or a surface sample (internodes, curtains, sample positions) stay derived inside their stage, where their inputs exist (fn-152 `ASTRA-REVIEW.md`, finding 1).

## Stage 2: the tree-space engine (fn-198)

The skeleton stage is the tree-space engine (`pipeline/space.rs`, private). It grows the family's engine species (`Family.space`, the preset's indexed `/space` rows, `presets/space.rs`) for `Family.age` whole years at the family's seed, under the site light the preset states, then converts the engine's structure into the pipeline's tree. Before conversion every structure is put into one canonical axis order, parents first and siblings by lineage (host, 2026-10-07), so the tree never depends on how it was assembled. Every constant of the conversion names its owner where it is defined. A family with no engine species is refused by name (`no engine species`), never grown another way. Ground contact (fn-301) lives in the lay walk and the frond curve, and in the pose where instance matrices finish. Every stage past it reads the envelope of the grown tree (`grown_envelope`: its height, its reach, where its crown starts and is widest, and the least shoulder whose outline holds the crown; the outline is smooth, and no family row stands in, host decisions 7 and 11a). The scaffold's grower compiles only into the crate's tests (fn-293), where fn-288 migrates and deletes it.

## The executor interface

The interface hands tools and tests the grown tree, its prepared artifacts and the CPU steps that make them. It never hands them a stage to chain.

```text
executor::grow(&Family) -> Grown             the skeleton stage, and the inputs
executor::growth(&Family) -> Growth          the engine's species and request; `year`
executor::expand(Tree, &Family) -> Expansion  a solved tree the caller supplies, prepared
                                             as `grow` would prepare its own
executor::element(ElementParams) -> Element  the leaf element, built from its rows
Grown::expansion(self) -> Expansion          the shed leaf bases (fn-204) and the fronds'
                                             axes (fn-283) hung on the stems,
                                             the element (validated), the twig placement,
                                             the shell depth checked, the leaf box
Expansion
  tree, element, reference                   the prepared artifacts
  wood_box(keep)                             the leaf box of some nodes
  stations                                   the leaf stations, with contact rings of their own
  curve                                      the wood as curves, which it is surfaced from (fn-208)
  wood                                       the wood the CPU sweeps
  mesh                                       the pipeline's own build of the whole tree
```

Each step is synchronous CPU work. `Expansion::mesh` is the pipeline's own build of the prepared tree, the one `mesh::build` returns.

`expand` takes a tree the caller already holds, for tests on hand-built trees, under the envelope that tree grew to. It builds no tree, and nothing outside the pipeline converts the engine's structure, so the tree-space examples' handed-in path is closed (fn-198): growing one stays inside the pipeline, so `expand` opens no second chain. Expansion opens by hanging the shed leaf bases (fn-204) and each frond's axis (fn-283) on the stems, as one tail, so a handed-in tree is clothed exactly as a grown one; a tree that already carries that tail is not clothed again.

The scaffold's grower, `branching::Specimen`, compiles only into the crate's tests (fn-293); `docs/growth-path.md` keeps the lessons of the separate growth path.

## The one cycle and its grower (fn-255)

Every tree grows through one cycle (fn-255): the rough layout and the light read, the balance and the allotment, every phase every cycle for every species, a zero setting a neutral term and never another way of building. It runs on one grower, on the CPU (`telperion_space::grow`, the cycle in `crates/telperion-space/src/grow/cycle.rs`). The pipeline's own build (`pipeline::build`, `mesh::build`) runs it, and `pipeline::growth` holds it between years. The device draws: `Renderer::submit` uploads the built tree, and no tree is grown on it.

A year is a state (fn-355). Nothing the grower reads depends on the age asked for, so the tree at year y on the way to any later age is the tree asked for at y, to the bit, and a growth goes on past its first request to the tree asked for that far. Every draw's window looks `LOOK` cycles on (40, the host's estimate, beside `RATE`, `SPAN` and `FLOOR`) and divides the wood a bud of its age grows over the look, its own stages and the ones it goes on into, by the whole tree expected at the draw's cycle plus the look; the expected-length table runs as far as the year grown plus the look and the longest sleep and is extended as a growth goes on (an entry is the same however far it runs). A sleeping bud is recorded with the cycle it wakes in when its drawn waking falls before it ages past its last stage, and a sleeping bud's expected wood is summed over those years; one that can never wake is not recorded and holds nothing open. Every year is laid out and lit like any other, the last one too, and the expected-kept ceiling is judged at the year grown (`pipeline::growth::grow_to`).

The wood is laid as the tree grows (fn-354). Each phytomer carries its whole section and the sapwood it holds, from the year it is laid at its age's primary radius (`form.primary`) at its scale. After each year's growth and its rough layout, which is laid in the last year too wherever an age lays wood, one pass from the tips down (`girth::lay`, on the grower's threads) gives every held phytomer the leaf above it that year and lays what its sapwood is short of: the sapwood a unit of leaf needs (`form.sapwood`, cm² per m²), of which one ring in `form.sapwood_rings` turns to heartwood each year. The leaf a phytomer bears in a year comes from one function (`girth::leaf`, fn-354 R5), which the light's deposit reads too: its PA's `leaf_area` at its scale and size while its age is under the PA's `leaf_years`, the share left in the year it passes it, while its wood is open, on the way to a living apex or a sleeping bud; closed wood bears none. The deposit walks every open axis that bears leaf, the year's phytomers first in the order laying them gave; where no age keeps a leaf past its year only those deposit. While the supernatural group sets its canopy size, every age's `leaf_area` is that size's square over the natural `/canopy/size`'s, so a larger drawn leaf is a larger leaf to the light and the wood (`Family::applied`). Wood is never lost: a crown removed or a lateral shed lays nothing more, and a retired subtree keeps nothing. The pass does not share the balance's walk: the balance runs at the start of a year over the open axes, before that year's growth, while the wood reads the year's layout after it, and a tree settled at any age reads the wood of its last year. The settle computes no girth: a phytomer's radius is `sqrt(section × share / π)`, `share` the fade its axis is drawn at, and a relay joins at the radius of the wood it stands in, on both sides, since its leaf enters its bearer at the tip. Sag, the ground and every measure read that radius; only the sweep eases a drop in section along its run, wherever the run goes, across a lateral that takes over a bearer that ends too, to fall no faster than 1 in 1, `max(own, below − distance)`, so a drop is drawn as a shoulder as long as it is deep; a lateral that starts its own run is drawn at its own base (`surface/samples.rs`; the slope is an estimate). The pass skips a closed subtree, which bears no leaf and lays nothing; wood that reopened would first retire its sapwood for the years skipped, `S (1 − 1/N)^k`, and none reopens today.

A lateral's kind follows its light (fn-354 R7). Each age holds a shade table, `shade_short`, one exponent per age as its zones' `lateral` tables hold one probability per age. A bud drawn as an age whose table sums to e > 0, formed at its bearer's light L (the light the shade hazard reads, 0 to 1 of open sky), keeps its kind with probability L^e and otherwise becomes age j with probability `shade_short[j]` / e (`lineage::turn`). Both draws are keyed under the bud's place, none is taken where e is 0, and each adds its lead to the bud's presence, so the kind changes by degree. An entry on the age's own index is refused. The tables then hold what a species does in full light.

Every decision the grower takes has a fixed representation, listed in FINE-CONTRACT.md (`.flow/evidence/fn-209-tree-space-f1-trees-resolved-on-demand/`), so a native build and a Wasm build grow the same tree:

- a draw is 53 bits, compared as an integer with a probability's threshold `ceil(p · 2^53)`;
- tables computed once per species point are supplied to the grower;
- a probability, a log, a lead, a window and a presence are ordinary f32, with the contract's own logarithms and powers (`telperion_space::contract`, `Transcendental`);
- time is a count of ticks of 2^-20 cycle, which the grower holds in an f64, where every count below 2^53 is exact, and which is never accumulated in an f32.

## The field (fn-257 R8)

The field is a consumer of the drawn stage's output, as the renderer is: it reads the drawn tree's records and writes them into a grid, and no stage, format or index of its own stands between the records and the grid. A field request names its grid (`FieldRequest`: `n` cells a side over the tree's box, and the limb order); the grid is `field::Frame`, the one definition the pipeline and the site's voxelizer (`src/field/voxelize.ts` `grid`) read: `n - 2` cells span the box's longest axis, centred in x and z, one empty cell below. A side outside 3 to 256 is refused by name before any work.

The field reads the drawn tree (`pipeline/field.rs`): each solved node's tapered sweep from its parent, and each leaf-bearing record of the leaf plan (`Plan::descriptors`, a frond's leaflets as oriented boxes) with its station count and limb system. Every record writes the cells it reaches by the cell rule (`field/splat.rs`); `Outputs::field` is the grid, one `Occupancy` a cell. A family the plan cannot describe is refused by name, never placed for its field. The field needs the drawn tree, so without the `geometry` feature it is refused. It is the site's field (`src/field/index.ts`, `docs/field-package.md`), built on the CPU by `TreeEngine.build(family, { field })` in `telperion.wasm`, with or without WebGPU; there is no `cut` and no coarse route.

## Flowers (fn-216)

Where a tree flowers is the engine's: each age has a `flowering` share (`/space/ages/<k>/flowering`, neutral 0), and the conversion keeps every drawn node of a flowering age as a bloom beside the tree (`Tree::blooms`: the node, its age's share, the side its bud faces), empty where no age flowers. What flowers is the expansion's (`pipeline/foliage/bloom.rs`, the `/bloom` rows): each bloom grows in past its keyed draw by its share; its florets stand along an axis that leaves the bud's side and turns toward hanging or standing by its droop, each on a stalk, the last floret sized by the count's fraction; each floret is one instance of the organ element, a petal turned `petals` times about it, the last petal sized by the count's fraction (host decision 4); and the time of year (`Request::season`, the window's middle by default) scales each floret in and out of its staggered window. Every draw is keyed by the bearing node's identity, its lineage (fn-220), so a floret is a pure function of (bloom, index, seed).

The flowers are a second foliage (`Outputs::organ`, `TreeMesh::organ`), none where nothing flowers, so a tree that flowers nowhere keeps its bytes. They are one step of the drawn stage over the tree it converts (`pipeline/drawn.rs`), and `Renderer::submit` uploads them beside the leaves. The renderer draws them with the leaf shader under a copy of the frame's block carrying the petals' colours (`/material/bloomFront*`, `bloomBack*`), no veins, margin or mottle, and the florets' own box. They cast no shadow yet. The CPU entry returns them as `organ` beside `foliage`, with `organReference` in the diagnostics (`docs/migration-0.2-flowers.md`).

## The wood's curve (fn-208)

The wood is handed over as curves, not triangles (`telperion_core::surface::Curve`, `Expansion::curve`). The surface is drawn from them, by the GPU at the screen's error (fn-208) or by the CPU at a stated view and error. Every tree produces the curve from its solved skeleton by the sweep's own steps (paths, samples, frames), so its wood has one path to the screen.

| Table | Per | Holds |
|---|---|---|
| `points` | ring the sweep stands | centre, radius (girth with the fork's easing, the run's easing and the flare, or a shaped run's extent), distance along the wood from the root, and the parallel-transported frame (`normal × binormal` is the tangent) |
| `runs` | run of the sweep, widest first | first point, count, whether it stands on the root, the cell a shaped run is drawn as (an index into `sections`), its largest radius |
| `clusters` | up to `CLUSTER` (32) points of one run, neighbours sharing an end | a sphere round every ring (lobes included), largest and smallest radius, and for each of `LEVELS` ring levels (level `k` keeps every `2^k`-th point) the most a left-out ring stands off its kept neighbours, monotone |
| `sections` | shaped run | the cell (`tree::Section`), as the tree keeps it: the palm's leaf bases |

Its tree-wide rows are the section's `lobes`, `lobeDepth` and `twistRate`, and the tree's height. A ring point `p` at angle `θ` stands at `centre + (normal cos θ + binormal sin θ) · radius · (1 + lobeDepth cos(lobes (θ + phase)))`, where `phase = 2π · twistRate · along / height`. A shaped run's ring is the cell's own (`Section::vertex`). The bark's coordinates are `(along, θ)`.

On the GPU a point is 32 bytes (`CurvePoint::pack`, `POINT_WORDS`; host decision 8):
- float32 centre, radius and distance along;
- the frame's two vectors, octahedral at 24 bits a component.

A ring drawn from a packed point stands within 1e-6 of its run's radius and two float32 units of its position from the exact one. A test measures this for every preset (`curve/tests.rs`).

`Curve::tessellate(&Viewer, error, budget)` is the CPU reference of the surface at the screen's error (fn-208, host decisions 2, 3 and 5). It surfaces the curve for one view within `error` pixels:
- **Ring level:** each cluster keeps the coarsest ring level whose error stands under half the budget at its nearest depth.
- **Pieces:** a curved stretch between kept rings is cut along its Hermite curve until its chords sag under that half (`Lθ/(8m²)`).
- **Sides:** each ring takes the fewest sides, `3·2^k`, whose polygon stands within the other half (`ceil(π / acos(1 − e/ρ))`).
- **Normals:** analytic, from the tube's radial direction, the lobes' turn and the radius's slope along the curve.
- **Bark coordinates:** `(along, angle)`, as the sweep's.
- **Budget:** a budget of triangles coarsens the error by 2, 4 or 8 and reports the scale; past that it refuses by name.

It is camera-dependent by definition, and reads only the curve.

The sweep's rings drawn from the curve alone are the sweep's own, to the bit, for every values preset, the palm's shaped cells included: that is what shows the curve carries the whole wood.

### The renderer's one wood path

The curve is the renderer's only wood (host decision 20). `TreeMesh` carries it beside the reference build's `SurfaceMesh`, `Renderer::submit` uploads it, and nothing else of the wood reaches the device; the CPU mesh stays for tests and for consumers without a GPU.

The CPU mesh is written in chunks of at most 65,536 vertices (`SurfaceChunk`, fn-291), so every index fits 16 bits and no buffer needs one long run of memory. Runs fill a chunk in table order until the next does not fit; a run longer than a chunk splits at ring boundaries into pieces that repeat three rings, and its last piece repeats the first ring and the bottom cap too, so the chunks laid end to end draw the run's triangles in its order. A chunk keeps three f32 a vertex of positions, its unit normal folded onto the octahedron in two snorm16 (`surface::encode`, within 1e-4 radians), its bark coords in two unorm16 (the distance along over `SurfaceMesh::along`, the angle over a turn, each within half a code) and `u16` indices local to it. The CPU entry returns the chunks with positions, normals and indices; the coords stay in the core (`docs/migration-0.2-wood-chunks.md`). Each frame the renderer surfaces it twice, for the camera and for the sun's map, in one compute pass that the timer's fourth pair stands around (`surfacing_p50_ms`):

1. `measure` counts each cluster at the four error scales, and `choose` takes the finest that fits the view's budget.
2. `count` and a three-pass scan give every cluster its offsets.
3. `rings` walks each cluster into ring records: a ring's point, sides and form, and where its vertices and indices go.
4. `emit` writes every ring's vertices and indices, one thread a ring, by an indirect dispatch.
5. `draws` writes the two indirect draws, tubes and ribbons.

A view's budget is its pixels times a bound a pixel on rings, vertices, tube indices and ribbon indices (host decision 19, `curve/target.rs`). A view past its budget is drawn at a coarser scale and reports it. Points, records, vertices and indices are each bound in two slices of at most WebGPU's default 128 MB, so the same passes bind in a browser (host decision 21).

## Why not a resolver

fn-152's review (`.flow/evidence/fn-152-one-parameter-table-the-generator-flows/ASTRA-REVIEW.md`) rejected one `resolve(&Family)` for every derived value: values live at the family, the grown tree, a branch and a surface sample, and requests skip outputs nobody reads. Typed inputs at the family's lifetime and ordinary functions below it keep each derivation where its inputs exist, and the executor interface is the one door tools and tests use.

## Tests

Tests of a stage's implementation are private test modules of the crate, `crates/telperion-core/src/suite/`, compiled only into its own test binary. They name the crate as its consumers do (`telperion_core::...`, through `extern crate self`), and under `cfg(test)`, which no other crate sees, each contract module carries its whole stage; no feature opens the stages to another crate. Tests that need only the public build stay in `crates/telperion-core/tests/`. Stage functions no production path calls any more are compiled for the suite alone.

Two compile-fail tests hold the boundary, each beside a positive control that builds through the interface: `crates/telperion-render/tests/boundary.rs` (a surface sweep called from the renderer) and `crates/telperion-wasm/tests/boundary.rs` (the skeleton solve and the scaffold's grower, called from the binding).
