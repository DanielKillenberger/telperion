# The pipeline boundary

Every tree is built by `telperion_core::pipeline`. Its stages are private to it, so a second chain outside the pipeline does not compile (fn-158; `docs/principles.md`, How they are held). What the pipeline exposes is what may be called:

- **`pipeline::build(&Family, Request)`**, the one build, with its request and outputs. `mesh::build` is the same build assembled into one mesh.
- **`pipeline::executor`**, the interface a second executor builds through. Its callers are the sanctioned exception, the GPU executor (`telperion_render::generation`), and the renderer's tests.

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

The skeleton stage is the tree-space engine (`pipeline/space.rs`, private). It grows the family's engine species (`Family.space`, the preset's indexed `/space` rows, `presets/space.rs`) for `Family.age` whole years at the family's seed, under the site light the preset states, then converts the engine's structure into the pipeline's tree. Before conversion every structure is put into one canonical axis order, parents first and siblings by lineage (host, 2026-10-07), so the tree never depends on how it was assembled. Every constant of the conversion names its owner where it is defined. A family with no engine species is refused by name (`no engine species`), never grown another way; on this branch that is the silver birch, Laurelin, Telperion and the ordinary tree. Every stage past it reads the envelope of the grown tree (`grown_envelope`: its height, its reach, where its crown starts and is widest, and the least shoulder whose outline holds the crown; the outline is smooth, and no family row stands in, host decisions 7 and 11a). The scaffold's grower compiles only into the crate's tests (fn-293), where fn-288 migrates and deletes it.

## The executor interface

The interface hands a second executor prepared artifacts and the CPU steps that make them. It never hands it a stage to chain.

```text
executor::grow(&Family) -> Grown             the skeleton stage, and the inputs
executor::expand(Tree, &Family) -> Expansion  a solved tree the caller supplies, prepared
                                             as `grow` would prepare its own
executor::element(ElementParams) -> Element  the leaf element, built from its rows
Grown::expansion(self) -> Expansion          the shed leaf bases (fn-204) and the fronds'
                                             axes (fn-283) hung on the stems,
                                             the element (validated), the twig placement,
                                             the shell depth checked, the leaf box
Expansion
  tree, element, twig, reference, leaves     the prepared artifacts and the rows the GPU packs
  supports_stations, round_section, seats    what the executor may take on
  compact, compact_with_contacts             the wood's compact rings, with contacts
  compact_stations(&shared), stations        the leaf stations, on shared rings or their own
  prepared_with_contacts, shared_stations    the canonical float32 wood, and stations on it
  prepared_wood, wood                        the canonical wood; the CPU wood
  curve                                      the wood as curves, which it is surfaced from (fn-208)
  mesh                                       the CPU reference for the whole tree
  wood_reach, wood_box(keep)                 a station's reach terms; the leaf box of some nodes
Folds, envelope, bins, beyond, crown_point   the whole-tree folds of some nodes, combined exactly
```

An executor that holds the whole tree on its device (fn-256 task 3) hands the pipeline only its summaries (`executor::resident`, `Summaries`): the whole-tree folds (`Folds`), the crown front, the leaf box's corners, the root's radius and the stem radius, and the pipeline returns the rows, element, twig placement, box and bearing bounds its drawing stages read. No node of the tree crosses.

Each step is synchronous CPU work. The device's build is async end to end (fn-256 task 3): natively the caller blocks on it, and a browser awaits it, since WebGPU completes a readback only on the page's event loop. The GPU executor keeps its own schedule: it calls the steps between its GPU submissions exactly where it called the stage functions, and every timing it reports covers the same work. `Expansion::mesh` is the pipeline's own CPU build of the prepared tree, the reference the device's build is compared with; the device never falls back to it. The renderer's production build reads none of the expansion's CPU station and wood steps (`compact*`, `*stations`, `prepared_*`, `wood`, `curve`, `mesh`, and `foliage/prepared.rs` behind them) since the CPU delivery was removed (fn-293 task 5); its tests read them across the crate boundary, as the device's reference.

`expand` takes a tree the caller already holds, for the renderer's tests of the GPU kernels on hand-built trees, under the envelope that tree grew to. It builds no tree, and nothing outside the pipeline converts the engine's structure, so the tree-space examples' handed-in path is closed (fn-198): growing one stays inside the pipeline, so `expand` opens no second chain. Expansion opens by hanging the shed leaf bases (fn-204) and each frond's axis (fn-283) on the stems, as one tail, so a handed-in tree is clothed exactly as a grown one; a tree that already carries that tail is not clothed again.

The growth path, the second sanctioned exception, was removed on 2026-10-02 (fn-181, `docs/growth-path.md`); the scaffold's grower, `branching::Specimen`, compiles only into the crate's tests (fn-293).

## The one cycle and its executors (fn-255, fn-256)

Every tree grows through one cycle (fn-255): the rough layout and the light read, the balance and the allotment, every phase every cycle for every species, a zero setting a neutral term and never another way of building. It has two executors, chosen by the caller and never by the species (fn-256 R1):

- The CPU's (`telperion_space::grow`, the cycle in `crates/telperion-space/src/grow/cycle.rs`) is the reference. The pipeline's own build (`pipeline::build`, `mesh::build`) runs it; the renderer reaches it only in its tests, through `executor::grow` and the expansion (`prepare_whole_resident`), since the CPU delivery was removed (fn-293 task 5).
- The device's (`crates/telperion-render/src/generation/cycle/`) grows, settles and lays the whole tree in device buffers and dresses its settled node table into one drawn-record buffer, chain by chain, one wave a dispatch (`chains.wgsl`, `dress.wgsl`): the records by id, then room for the retained leaf bases and each frond's rachis (fn-283: the boots stage draws a crown's fronds once, into the table its stations read, and hangs each frond's axis from it), then the axes by slot (fn-293 task 6). The drawing stages (`generation/resident/`) fold, sweep, station, ring and place by reading that buffer, with no node table of their own; the sweep writes the renderer's packed curve points and clusters itself (`pack.wgsl`), and the rings decode their frames from them, so no pass converts the curve. The pipeline sees only the whole-tree summaries. Every device build takes it (`Generator::prepare`). A family it cannot draw is refused by name, by its capability (leaves that stand on no regular twig stations), and a device or capacity failure is a named error, never a CPU rebuild.

The two agree by lineage, not to the bit: integers exactly, floats within the lineage tolerance (`telperion_space::agree`; the renderer's `resident/agree_tests.rs`). Every decision both take is listed in FINE-CONTRACT.md with its representation, so the executors cannot disagree on topology:

- a draw is 53 bits, compared as an integer with a probability's threshold `ceil(p · 2^53)`;
- tables computed once per species point are supplied to both executors;
- a probability, a log, a lead, a window and a presence are ordinary f32 on both executors, by the same operations, with the contract's own logarithms and powers (`telperion_space::contract`, the device's `cycle/decide.wgsl`);
- time is a count of ticks of 2^-20 cycle, which the CPU holds in an f64, where every count below 2^53 is exact, and which is never accumulated in an f32.

## The field (fn-257 R8)

The field is a consumer of the drawn stage's output, as the renderer is: it reads the drawn tree's records and writes them into a grid, and no stage, format or index of its own stands between the records and the grid. A field request names its grid (`FieldRequest`: `n` cells a side over the tree's box, and the limb order); the grid is `field::Frame`, the one definition the CPU, the device and the site's voxelizer (`src/field/voxelize.ts` `grid`) read: `n - 2` cells span the box's longest axis, centred in x and z, one empty cell below. A side outside 3 to 256 is refused by name before any work.

- The CPU reads the CPU's drawn tree (`pipeline/field.rs`): each solved node's tapered sweep from its parent, and each leaf-bearing record of the leaf plan (`Plan::descriptors`, a frond's leaflets as oriented boxes) with its station count and limb system. Every record writes the cells it reaches by the cell rule (`field/splat.rs`); `Outputs::field` is the grid, one `Occupancy` a cell. A family the plan cannot describe is refused by name, never placed for its field. The field needs the drawn tree, so without the `geometry` feature it is refused.
- The device reads the device's (`Generator::grow_field`, `generation/resident/field*`): the drawn records as the dress wrote them, without the curve the field never reads, and the station segments; one pass writes every record into the grid on the device, and only the grid is read back. Nothing is held on the device after it. It is the site's field (`src/field/index.ts`, `docs/field-package.md`); there is no `cut` and no CPU coarse route. Without WebGPU, or on a lost device, it is refused by name.

The CPU's is the device's reference: every shipped preset at 24, 36 and 48 cells a side agrees within the tolerances `resident/field_tests.rs` holds, both asked over the CPU's grid.

## Flowers (fn-216)

Where a tree flowers is the engine's: each age has a `flowering` share (`/space/ages/<k>/flowering`, neutral 0), and the conversion keeps every drawn node of a flowering age as a bloom beside the tree (`Tree::blooms`: the node, its age's share, the side its bud faces), empty where no age flowers. What flowers is the expansion's (`pipeline/foliage/bloom.rs`, the `/bloom` rows): each bloom grows in past its keyed draw by its share; its florets stand along an axis that leaves the bud's side and turns toward hanging or standing by its droop, each on a stalk, the last floret sized by the count's fraction; each floret is one instance of the organ element, a petal turned `petals` times about it, the last petal sized by the count's fraction (host decision 4); and the time of year (`Request::season`, the window's middle by default) scales each floret in and out of its staggered window. Every draw is keyed by the bearing node's identity, its lineage (fn-220), so a floret is a pure function of (bloom, index, seed).

The flowers are a second foliage (`Outputs::organ`, `TreeMesh::organ`), none where nothing flowers, so a tree that flowers nowhere keeps its bytes. On the CPU they are one step (`Expansion::organ`) over the tree the CPU converts. On the device path (fn-229, fn-256), the device draws every floret by the same keys (`bloom.wgsl`, a mirror of `bloom.rs`), packed against the tree's leaf box padded by a floret's reach. The dress carries each node's phytomer key and its age's share, and the stations count each node's florets beside its runs in one scan. A second entry in `place.wgsl` places them in one organ buffer, in the submission that places the device's leaves. The renderer draws them with the leaf shader under a copy of the frame's block carrying the petals' colours (`/material/bloomFront*`, `bloomBack*`), no veins, margin or mottle, and the florets' own box. They cast no shadow yet. The CPU entry returns them as `organ` beside `foliage`, with `organReference` in the diagnostics (`docs/migration-0.2-flowers.md`).

## The wood's curve (fn-208)

The wood is handed over as curves, not triangles (`telperion_core::surface::Curve`, `Expansion::curve`). The surface is drawn from them, by the GPU at the screen's error (fn-208) or by the CPU at a stated view and error. Every tree produces the curve from its solved skeleton by the sweep's own steps (paths, samples, frames), so whatever grew the tree, its wood has one path to the screen.

| Table | Per | Holds |
|---|---|---|
| `points` | ring the sweep stands | centre, radius (girth with the fork's easing and the flare, or a shaped run's extent), distance along the wood from the root, and the parallel-transported frame (`normal × binormal` is the tangent) |
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

fn-152's review (`.flow/evidence/fn-152-one-parameter-table-the-generator-flows/ASTRA-REVIEW.md`) rejected one `resolve(&Family)` for every derived value: values live at the family, the grown tree, a branch and a surface sample, and requests skip outputs nobody reads. Typed inputs at the family's lifetime and ordinary functions below it keep each derivation where its inputs exist, and the executor interface is the one door the exception uses.

## Tests

Tests of a stage's implementation are private test modules of the crate, `crates/telperion-core/src/suite/`, compiled only into its own test binary. They name the crate as its consumers do (`telperion_core::...`, through `extern crate self`), and under `cfg(test)`, which no other crate sees, each contract module carries its whole stage; no feature opens the stages to another crate. Tests that need only the public build stay in `crates/telperion-core/tests/`. Stage functions no production path calls any more are compiled for the suite alone.

Two compile-fail tests hold the boundary, each beside a positive control that builds through the interface: `crates/telperion-render/tests/boundary.rs` (a surface sweep called from the renderer) and `crates/telperion-wasm/tests/boundary.rs` (the skeleton solve and the scaffold's grower, called from the binding).
