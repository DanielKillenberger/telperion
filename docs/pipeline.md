# The pipeline boundary

Every tree is built by `telperion_core::pipeline`. Its stages are private to it, so a second chain outside the pipeline does not compile (fn-158; `docs/principles.md`, How they are held). What the pipeline exposes is what may be called:

- **`pipeline::build(&Family, Request)`**, the one build, with its request and outputs. `mesh::build` is the same build assembled into one mesh.
- **`pipeline::executor`**, the interface a second executor builds through. Its callers are the two sanctioned exceptions, the GPU executor (`telperion_render::generation`) and the growth path (`specimen`), and the renderer's tests.

The stages live in private modules of the pipeline (`crates/telperion-core/src/pipeline/{branching,foliage,surface,...}`). Their data stays public as types, through `pipeline::contract`, at the paths consumers already name (`telperion_core::foliage::Element`, `telperion_core::surface::SurfaceMesh`): the parameter groups a family is made of and the artifacts a build returns. A type is a contract; a function that builds one is a stage and stays inside. The rest of the crate reaches only the family's own checks (`validate_skeleton`, `build_element`, `canopy_rows`, `height`), re-exported to it by name.

## Typed stage inputs

`pipeline::build` reads the family once and hands each stage a narrow input:

| input | stage | rows |
|---|---|---|
| `GrowInput` | the skeleton | skeleton, radii, canopy (a frond crown clears apical twigs; shed leaf bases) |
| `PlanInput` | the element, the leaf plan, the leaf box | element, envelope, canopy, surface, seed, the twig placement, the leaf box |
| `SurfaceInput` | the rings and the wood | the envelope's height, surface |
| `LeafInput` | placement and the cull | envelope, canopy, seed, shell depth |

`GrowInput` lends the family's groups to the one stage that reads them; the others are owned copies with no reference back to the family, so an expansion outlives it. Family-level values are derived there once: the resolved twig rows, the twig placement and the leaf box read from them, the height. A derivation that can fail is kept as its result and answered where its stage runs, so the error a request meets is the one it met before, in the same order. Values that live at a branch or a surface sample (internodes, curtains, sample positions) stay derived inside their stage, where their inputs exist (fn-152 `ASTRA-REVIEW.md`, finding 1).

## The executor interface

The interface hands a second executor prepared artifacts and the CPU steps that make them. It never hands it a stage to chain.

```text
executor::grow(&Family) -> Grown             the skeleton stage, and the inputs
executor::expand(Tree, &Family) -> Expansion  a solved tree the caller supplies, prepared
                                             as `grow` would prepare its own
executor::element(ElementParams) -> Element  the leaf element, built from its rows
Grown::expansion(self) -> Expansion          the element (validated), the twig placement,
                                             the shell depth checked, the leaf box
Expansion
  tree, element, twig, reference, leaves     the prepared artifacts and the rows the GPU packs
  supports_stations, round_section, seats    what the executor may take on
  compact, compact_with_contacts             the wood's compact rings, with contacts
  compact_stations(&shared), stations        the leaf stations, on shared rings or their own
  prepared_with_contacts, shared_stations    the canonical float32 wood, and stations on it
  prepared_wood, wood                        the canonical wood; the CPU wood
  mesh                                       the CPU reference for the whole tree
```

Each step is synchronous CPU work. The GPU executor keeps its own schedule: it calls the steps between its GPU submissions exactly where it called the stage functions, so the position upload still runs while the CPU prepares stations (`preparation.rs`, the overlap after `begin_positions`), and every timing it reports covers the same work. `Expansion::mesh` is the pipeline's own CPU build of the prepared tree, the reference the GPU falls back to.

`expand` takes a tree the caller already holds, for the renderer's tests of the GPU kernels on hand-built trees. It builds no tree: growing one stays inside the pipeline, so `expand` opens no second chain.

The growth path keeps its own timeline, the sanctioned exception, and presents a grown tree through one crate-internal step: `executor::present` sweeps the wood at the specimen's height, builds the element, adds to the recorded leaves the short shoots and rosette the wood on screen bears, culls against the envelope of its age and bounds the union, in the order it did. Its public handles are `specimen::SpecimenStore` and `specimen::SpecimenView`; `branching::Specimen` stays public as the type they hand out, and its constructors (the scaffold's own grower among them) are the crate's.

## Why not a resolver

fn-152's review (`.flow/evidence/fn-152-one-parameter-table-the-generator-flows/ASTRA-REVIEW.md`) rejected one `resolve(&Family)` for every derived value: values live at the family, the grown tree, a branch and a surface sample, and requests skip outputs nobody reads. Typed inputs at the family's lifetime and ordinary functions below it keep each derivation where its inputs exist, and the executor interface is the one door the two exceptions use.

## Tests

Tests of a stage's implementation are private test modules of the crate, `crates/telperion-core/src/suite/`, compiled only into its own test binary. They name the crate as its consumers do (`telperion_core::...`, through `extern crate self`), and under `cfg(test)`, which no other crate sees, each contract module carries its whole stage; no feature opens the stages to another crate. Tests that need only the public build stay in `crates/telperion-core/tests/`. Stage functions no production path calls any more are compiled for the suite alone.

Two compile-fail tests hold the boundary, each beside a positive control that builds through the interface: `crates/telperion-render/tests/boundary.rs` (a surface sweep called from the renderer) and `crates/telperion-wasm/tests/boundary.rs` (the skeleton solve and the specimen's scaffold grower, called from the binding).
