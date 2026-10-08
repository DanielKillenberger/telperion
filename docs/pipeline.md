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

The skeleton stage is the tree-space engine (`pipeline/space.rs`, private). It grows the family's engine species (`Family.space`, the preset's indexed `/space` rows, `presets/space.rs`) for `Family.age` whole years at the family's seed, under the site light the preset states, then converts the engine's structure into the pipeline's tree. Before conversion every structure is put into one canonical axis order, parents first and siblings by lineage (host, 2026-10-07), so the tree never depends on how it was assembled. Every constant of the conversion names its owner where it is defined. A family with no engine species is refused by name (`no engine species`), never grown another way; on this branch that is the silver birch, Laurelin, Telperion and the ordinary tree. Every stage past it reads the envelope of the grown tree (`grown_envelope`: its height, its reach, where its crown starts and is widest, and the least shoulder whose outline holds the crown; the outline is smooth, and no family row stands in, host decisions 7 and 11a). The scaffold's grower is reached by the suite alone until R0 removes it.

## The executor interface

The interface hands a second executor prepared artifacts and the CPU steps that make them. It never hands it a stage to chain.

```text
executor::grow(&Family) -> Grown             the skeleton stage, and the inputs
executor::expand(Tree, &Family) -> Expansion  a solved tree the caller supplies, prepared
                                             as `grow` would prepare its own
executor::element(ElementParams) -> Element  the leaf element, built from its rows
Grown::expansion(self) -> Expansion          the shed leaf bases hung on the stems (fn-204),
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

An executor that holds part of a tree's nodes (the GPU's fine roots, fn-212 task 4) folds its part and the pipeline combines it with its own: the height, the crown's lowest node and the widest (`Folds::with`), the shoulder's bins by their most, and the leaf box by its corners, each exactly the whole tree's over any split (`parts_of_a_tree_combine_to_the_whole`). The shoulder is read off the pipeline's own nodes and the crown nodes the executor hands on, those no node in a bin beyond may pass: a bounded summary, since the front is the nodes no other passes whatever the bins. `Tree::attachments` gives each fine root's first axis's lineage, the node its bearer's phytomer is drawn as, and its seam: what the executor draws the root from. A bearer reads its fine roots through their attachments alone, on every path (fn-212 task 7, host option 3): each root counts as one child for the bearer's kind, whatever it draws at its seam, and forks its bearer as its base radius from phase 2 reads against the bearer's (`Attachment::fork`), so nothing above the cut reads wood another executor draws. On the real path the pipeline converts only the structure above the cut and the roots laid on the CPU (`executor::settle`, `Settling::part`, `Part::expansion`), and the executor draws the rest.

Each step is synchronous CPU work, apart from `settle`, which awaits each of the executor's phases (`telperion_space::Step`; fn-212 task 10): natively the caller blocks on it, and a browser awaits it, since WebGPU completes a readback only on the page's event loop. The GPU executor keeps its own schedule: it calls the steps between its GPU submissions exactly where it called the stage functions, so the position upload still runs while the CPU prepares stations (`preparation.rs`, the overlap after `begin_positions`), and every timing it reports covers the same work. `Expansion::mesh` is the pipeline's own CPU build of the prepared tree, the reference the GPU falls back to.

`expand` takes a tree the caller already holds, for the renderer's tests of the GPU kernels on hand-built trees, under the envelope that tree grew to. It builds no tree, and nothing outside the pipeline converts the engine's structure, so the tree-space examples' handed-in path is closed (fn-198): growing one stays inside the pipeline, so `expand` opens no second chain. Expansion opens by hanging the shed leaf bases on the stems (fn-204), so a handed-in tree is clothed exactly as a grown one; a tree that already carries its bases is clothed again.

The growth path, the second sanctioned exception, was removed on 2026-10-02 (fn-181, `docs/growth-path.md`); the scaffold's grower, `branching::Specimen`, is private to the crate.

## Fine growth (fn-209 F1)

The tree-space engine's finest orders are grown in the expansion, not on the CPU before it (`.flow/specs/fn-209-*.md`; DESIGN-FINE.md and ASTRA-REVIEW-FINE.md in its evidence). The CPU grows the structure above a cut by the engine's own grower. A bud whose expected kept phytomers (the closed form's `count`, light-neutral, read before growth) are at most the cut is a fine root: it is made where it stands and handed on, and its stand-in keeps its place among its bearer's children.

Production uses realized mechanics (fn-211's DESIGN-INTERLEAVE.md, "After Astra's review", host decision 3). The expansion grows each fine root whole from its bud alone, by the engine's own grower, and the root and the structure above the cut settle together in four phases. In each phase a root reports one record of totals up and its bearer hands it what it reads down, never anything per node:

1. The root is marked, assigned and shed by its realized subtree; up goes its subtree's living, which its bearer's fade reads.
2. Down comes its bearer's scale and its node's drawn scale; the root is scaled and thickened; up goes its base radius.
3. Down comes its bearer's laid frame at its node; the root is laid and its levers gathered; up goes its load at its base (mass and first moment).
4. Down comes its bearer's frame as laid under sag; the root is laid again under its levers, from its own first lay's frames.

Every pass is the whole tree's own and every sum a root enters is taken in the whole tree's order, so the tree is today's whole-tree engine's to the bit at every cut, in every item order and chunking (`crates/telperion-space/src/fine/tests/realized.rs`; beech and spruce at 40 and 80 in the `realized` example). A species that reads light, or whose girth reads the tree as grown (retained pipes, leaf girth), is refused here and follows fn-211's stepped executor.

The CPU reference executor of this step is `telperion_space::grow_fine` (`crates/telperion-space/src/fine.rs`, `fine/realized.rs`; growth in `grow/realized.rs`). It lives in the engine's crate until F4 (fn-198) brings the engine into this pipeline; then it moves under `pipeline::executor` beside the expansion's other steps. The structure above the cut and its four phases are one driver, `telperion_space::settle` (`fine/expansion.rs`), whichever executor grows the roots (the `Roots` trait): the CPU reference or the GPU's. A root whose conservative reach bound can reach the ground's landing zone from its bearer's node is laid on the CPU reference executor whichever executor grew it (host decision 3, `fine/landing.rs`). Option 4's committed boundaries are gone, its probe `grow_committed` with them. Every decision a fine root takes is listed in FINE-CONTRACT.md with its representation, so the two executors cannot disagree on topology:

- a draw is 53 bits, compared as an integer with a probability's threshold `ceil(p · 2^53)`;
- tables computed once per species point are supplied to both executors;
- a per-bud threshold is computed in one specified fixed point, time in ticks of 2^-20 cycle (host decision 1, `telperion_space::fixed`).

The GPU executor (`crates/telperion-render/src/generation/fine/`) runs realized mechanics as `settle`'s `Roots` (`device.rs`). It grows each wave of chains on the contract to the bit, every axis kept (task 2), and takes every presence a fade reads as a Q32 integer, bit for bit the CPU's: growth records each one's inputs and the chain's end takes them through one call of the contract's leads and windows (`assign.wgsl`; host decision 8). Phase 1's fades carry each subtree's living up from the deepest wave, each chain reading its borne buds' chains at the range the emit scan gave it, and shed from the first wave down (`fades.wgsl`). Phases 2 to 4 scale, thicken, lay, gather levers and bend in f32 between the CPU's settles (`settle.wgsl`). Per run, per wave and per phase only totals or one record per root cross (host decision 6). It ends at the botanical tree and its laid geometry in device buffers, positions each root's own from its bearer's node (host decision 5). Its readers dress, sweep, station and place those roots on the device, and `Generator::prepare` takes this path for every resident tree whose fine roots come apart (fn-212 task 10, `resident.rs`). A tree that does not come apart is grown whole on the CPU executor, and `executor::apart` names the gap: a family with no engine species, a species whose buds read light or whose girth reads the tree as grown (fn-211), or leaves that stand on no regular twig stations (the palm's fronds). Its topology and presences equal the CPU executor's to the bit, and its geometry is held to fn-125's screen against `grow_fine` (`realized_tests.rs`, `realized_timing.rs`).

A species any bud of which reads light is refused until F2b (fn-211) carries its light aggregates in the record.

The engine marks the axes it grew in fine roots (`telperion_space::grow_marked`), and the conversion keeps as a seam (`Tree::seams`) every fine root's node drawn from wood above the cut: its first axis's first node, and a lateral that its first phytomers, merged into the bearer's node, hang there (fn-212 task 5). A sweep run never carries on into a seam: a fine root's wood starts a run of its own and meets its bearer at a socket (fn-212 D4). The runs keep two orders, the structure above the cut's and then the fine roots', each widest first, and no sort reaches across the seam (D5).

## The wood's curve (fn-208)

The wood is handed over as curves, not triangles (`telperion_core::surface::Curve`, `Expansion::curve`). The surface is drawn from them, by the GPU at the screen's error (fn-208) or by the CPU at a stated view and error. Every tree produces the curve from its solved skeleton by the sweep's own steps (paths, samples, frames), so whatever grew the tree, its wood has one path to the screen.

| Table | Per | Holds |
|---|---|---|
| `points` | ring the sweep stands | centre, radius (girth with the fork's easing and the flare, or a shaped run's extent), distance along the wood from the root, and the parallel-transported frame (`normal × binormal` is the tangent) |
| `runs` | run of the sweep: the structure above the cut, then the fine roots, widest first on each side (fn-212 D5) | first point, count, whether it stands on the root, the cell a shaped run is drawn as (an index into `sections`), its largest radius |
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

The curve is the renderer's only wood (host decision 20). `TreeMesh` carries it beside the reference build's `SurfaceMesh`, `Renderer::submit` uploads it, and nothing else of the wood reaches the device; the CPU mesh stays for tests and for consumers without a GPU. Each frame the renderer surfaces it twice, for the camera and for the sun's map, in one compute pass that the timer's fourth pair stands around (`surfacing_p50_ms`):

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
