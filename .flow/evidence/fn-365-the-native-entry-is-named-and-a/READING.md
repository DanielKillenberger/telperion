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
