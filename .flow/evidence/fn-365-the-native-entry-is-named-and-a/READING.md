# fn-365 R1: the entry's surface as an outside crate reaches it

Read 14 Nov 2026 at `990b10556`, before any change. Code read: `crates/telperion-core/src/{lib.rs,pipeline.rs,pipeline/contract.rs,pipeline/space/growth.rs,pipeline/input.rs,params.rs,presets.rs,mesh.rs}`.

## `pipeline::growth`

Reached as `telperion_core::pipeline::growth` (`pub use space::growth`; `pipeline::space` itself is private).

- `Params` (opaque, no public fields) and `Params::of(&Family) -> Result<Params>`.
- `GrowthState` (opaque), with `committed() -> u32` and `horizon() -> u32`.
- `begin(&Arc<Params>, seed: u32, horizon: f64) -> Result<GrowthState>`.
- `grow_to(&mut GrowthState, age: f64) -> Result<()>`.
- `settle(&GrowthState, at: f64) -> Result<Tree>`.
- `colouring(&GrowthState, at: f64) -> Result<f64>`.
- `mesh(&GrowthState, at: f64, &Family) -> Result<mesh::TreeMesh>`, under `geometry`.

Every type in these signatures is public at a path an outside crate names: `Family` (crate root), `tree::Tree`, `mesh::TreeMesh`, `Error`, `Result`. Nothing private has to be opened; R2's test calls all of it but `colouring` from `crates/telperion-core/tests/`.

## `pipeline::build`

- `build(&Family, Request) -> Result<Built>`.
- `Request` (all fields public; `Default`, `Request::mesh()`), `Schedule`, `FieldRequest`, `clock()`.
- `Built { skeleton: Skeleton { tree }, outputs: Outputs }`; `Outputs` with `element`, `plan`, `wood`, `curve`, `leaves: Leaves`, `organ`, `field`, `structure: Structure`, `stages: Stages`, every field public.
- The artifact types are public through `pipeline::contract`, re-exported at the crate root: `foliage::{Element, Instances, plan::Plan}`, `surface::{SurfaceMesh, SurfaceChunk, Curve}`, `field::{Grid, Occupancy}`.

## Seed and age

`growth` takes the seed as `begin`'s argument and the age as `grow_to`'s and `settle`'s; `Params::of` keeps neither of the family's rows. `growth::mesh` dresses the tree through the same `Inputs::of(family, ...)` the build uses, so its leaf stages read the family's `skeleton.seed` row (`pipeline/input.rs`, `PlanInput::seed`, `LeafInput::seed`), not `begin`'s seed. `pipeline::build` reads `family.age` and `family.skeleton.seed` (`GrowInput::of`).

## `cargo tree -p telperion-core --no-default-features --features json`

Run 14 Nov 02:32 through the queue (it waited from 02:26:54 for memory room; `FRICTION.md`). Normal dependencies:

- `serde` 1.0.229 (with `serde_derive`, a proc-macro: `proc-macro2`, `quote`, `syn`, `unicode-ident`), `serde_json` 1.0.151 (`itoa`, `memchr`, `serde_core`, `zmij`): both from `json`.
- `slotmap` 1.1.1 (its `serde` under `json`; build-dependency `version_check`).
- `telperion-space`, which brings `libm` 0.2.16 and `rayon` 1.12.0 (`either`, `rayon-core`, `crossbeam-deque`, `crossbeam-epoch`, `crossbeam-utils`).
- Build-dependency: `telperion-space` again.

No browser, GPU, windowing or harness crate. The survey's list (`telperion-space`, `slotmap`, `serde`) leaves out `telperion-space`'s own `libm` and `rayon`. Dev-dependencies (`bincode`, `rayon`, `serde_json`, `trybuild` with `toml`, `glob`, `termcolor`) are the tests' and not linked by a caller.

## Names and ids on the surface

The calls themselves take none: `growth`'s and `build`'s inputs are `Family`, `Params`, a seed, ages and a `Request`. `Family` has no name or identity row (`family.rs`; `presets::space::Space` holds the engine species as values).

Beside the entry, inside `telperion-core`:

- `params`, the module whose `parse` is the parameter door, also exports `preset(id: u32) -> Result<Family>` (a lookup by the binding's numeric id) and re-exports `by_identity(&str)`, `CATALOGUE` and `IN_WORK` from `presets`. `params::parse` refuses a string in place of the parameter object by name.
- `params::decode`, which `parse` runs, starts every parsed family from `preset(0)`, the ordinary family looked up by its numeric id: a row the text leaves out takes the ordinary family's value. The id never reaches the caller.
- `presets` is public: `Preset` (with `from_id(&str)` and `profile_id()`), `by_identity(&str)`, `CATALOGUE`.

Per R4 these are returned to the host and not removed here.

## R6: where each seed is read after `begin` (14 Nov 2026, at `de1215694`)

`begin`'s seed goes into `telperion_space::Request::seed` and is held only inside the engine's `Paused`. `Paused` has no accessor for it, and `GrowthState` keeps no seed of its own. So the state does not hold its seed in a form the core can read. Everything the engine grows and settles comes from that seed: the structure, its keys, `Tree::clusters`, `Tree::blooms` and `Tree::unfolding`.

The family's `skeleton.seed` row reaches a growth only through `growth::mesh`, which derives `Inputs::of(family, ...)` as the build does. That gives it two fields:

- `LeafInput::seed` → `foliage::place_on` (`pipeline/drawn.rs`). This keys every leaf stream:
  - each run's leaves (`placement.rs`, `tree::leaf_stream(seed, key)`);
  - a short shoot's cluster (`cluster.rs`);
  - a frond's rosette (`rosette.rs`, `stream`);
  - a station's leaflets (`station.rs`, `leaflet_stream`);
  - the clumping thin (`clumping.rs`, `draw`).
- `PlanInput::seed` → `plan::plan` (`pipeline/stage.rs`). This runs only when a field is requested. `growth::mesh` requests `Request::mesh()`, with no field, so it never reads this one.

Not read after `begin`:

- **The wood.** Rings, sweep and curve key their frames from the tree (`surface/rings.rs`, `frames::seed`, the node's seed side), not from a seed.
- **Flowers.** Florets are keyed by the bearing node's lineage and their index (`foliage/bloom.rs`). They are packed against the leaves' box, though, so they move with the leaves.
- **`settle`.** `Params::of` keeps no seed.
- **`colouring`.** It reads the species and the year's fraction only, and draws nothing.
- **`GrowthBias`.** Its noise is keyed by the seed, but it is built only by the scaffold's specimen, which compiles into tests alone.
