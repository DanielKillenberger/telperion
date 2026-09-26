# The pipeline boundary

Every tree is built by `telperion_core::pipeline`. Its stages are private to it, so a second chain outside the pipeline does not compile (fn-158; `docs/principles.md`, How they are held). What the pipeline exposes is what may be called:

- **`pipeline::build(&Family, Request)`**, the one build, with its request and outputs.
- **`pipeline::executor`**, the interface a second executor builds through. Its only callers are the two sanctioned exceptions: the GPU executor (`telperion_render::generation`) and the growth path (`specimen`).

The data the stages hand on stays public as types: the tree, the element, the instances, the surfaces, the field. A type is a contract; a function that builds one is a stage and stays inside.

## Typed stage inputs

`pipeline::build` reads the family once and derives one narrow input per stage, each an owned copy of the rows that stage reads, with no reference back to the family:

| input | stage | rows |
|---|---|---|
| `GrowInput` | the skeleton | skeleton, radii, canopy (a frond crown clears apical twigs; shed leaf bases) |
| `PlanInput` | the element, the leaf plan, the leaf box | element, envelope, canopy, surface, radii, seed, the resolved twig rows |
| `SurfaceInput` | the rings and the wood | the envelope's height, surface |
| `LeafInput` | placement and the cull | envelope, canopy, seed, shell depth |

Family-level values are derived there once: the resolved twig rows, the twig placement read from them, the height. A derivation that can fail is kept as its result and answered where its stage runs, so the error a request meets is the one it met before, in the same order. Values that live at a branch or a surface sample (internodes, curtains, sample positions) stay derived inside their stage, where their inputs exist (fn-152 `ASTRA-REVIEW.md`, finding 1).

## The executor interface

The interface hands a second executor prepared artifacts and the CPU steps that make them. It never hands it a stage to chain.

```text
executor::grow(&Family) -> Grown              the skeleton stage, and the inputs
Grown::expansion(self) -> Expansion           the element (validated), the twig placement,
                                              the shell depth checked, the leaf box
Expansion
  tree, element, twig, reference, placement   the prepared artifacts and the rows the GPU packs
  supports_stations, round_section, seats     what the executor may take on
  compact, compact_with_contacts              the wood's compact rings, with contacts
  compact_stations(&shared), stations         the leaf stations, on shared rings or their own
  prepared_with_contacts, shared_stations     the canonical float32 wood, and stations on it
  prepared_wood, wood                         the canonical wood; the CPU wood
  mesh                                        the CPU reference for the whole tree
```

Each step is synchronous CPU work. The GPU executor keeps its own schedule: it calls the steps between its GPU submissions exactly where it called the stage functions, so the position upload still runs while the CPU prepares stations (`preparation.rs`, the overlap after `begin_positions`), and every timing it reports covers the same work. `Expansion::mesh` is the pipeline's own CPU build of the prepared tree, the reference the GPU falls back to.

The growth path keeps its own timeline, the sanctioned exception, and presents a grown tree through one step: `executor::present` sweeps the wood at the specimen's height, builds the element, adds to the recorded leaves the short shoots and rosette the wood on screen bears, culls against the envelope of its age and bounds the union, in the order it did.

## Why not a resolver

fn-152's review (`.flow/evidence/fn-152-one-parameter-table-the-generator-flows/ASTRA-REVIEW.md`) rejected one `resolve(&Family)` for every derived value: values live at the family, the grown tree, a branch and a surface sample, and requests skip outputs nobody reads. Typed inputs at the family's lifetime and ordinary functions below it keep each derivation where its inputs exist, and the executor interface is the one door the two exceptions use.

## Tests

Tests of a stage's implementation are private test modules of the crate (`src/suite/`), compiled only into its own test binary; no feature opens the stages to another crate. A compile-fail test (`crates/telperion-render/tests/boundary.rs`) holds the boundary: a stage called from telperion-wasm or telperion-render does not compile, beside a positive control that builds through the interface.
