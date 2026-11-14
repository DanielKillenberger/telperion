# The native entry

What an engine links to grow trees without a browser, the harness or a GPU (fn-351 P10, fn-365). It is the pipeline's own door (`docs/pipeline.md`), named here; there is no facade crate, no C ABI for native callers and no plugin.

## What an engine links

The crate `telperion-core`. Its dependencies are `telperion-space` (with `libm` and `rayon`) and `slotmap`, and with `json` also `serde` and `serde_json`; none is a browser, GPU or windowing crate (`.flow/evidence/fn-365-the-native-entry-is-named-and-a/READING.md`). Two features matter:

- `json`: `params::parse`, the parameter table read from its wire object. An engine that builds a `Family` in Rust does not need it.
- `geometry` (default): the wood, the leaves, the field and `growth::mesh`. Without it the crate returns the solved tree, the leaf plan and the structure, for an engine that draws its own geometry.

## The calls, in order

1. `params::parse(&serde_json::Value) -> Result<Family>`: the parameters, every row judged. A preset's id in their place is refused by name.
2. Either hold the tree between years:
   - `pipeline::growth::Params::of(&family)`, kept in an `Arc`;
   - `growth::begin(&params, seed, horizon)`, the tree before its first year;
   - `growth::grow_to(&mut state, age)`, any age, past the horizon too;
   - `growth::settle(&state, at) -> Tree` and `growth::mesh(&state, at, &family) -> TreeMesh`, the tree at any `at` within the last year grown; `growth::colouring(&state, at)`, how far its falling leaves have turned.
3. Or build once: `pipeline::build(&family, Request) -> Built`.

The seed and the age are arguments of `growth`: `begin`'s `seed` grows the tree, and `grow_to`'s `age` and `settle`'s `at` say when it is. A growth has one seed (fn-365 R6). The state keeps the seed it was begun with, and `growth::mesh` keys the leaves by it as well as the wood. `growth` reads neither the family's `skeleton.seed` row nor its `age` row. `pipeline::build` reads both rows and takes no seed or age of its own.

## What comes back

`growth::settle` returns the `Tree`: its nodes with position, parent, radii, kind and stem. `growth::mesh` returns the `TreeMesh`: the wood as chunked indexed meshes (`SurfaceMesh`), the wood as a curve, the leaf element with its instances, the flowers, and the bounds of all of it.

`pipeline::build` returns the solved tree and, by `Request`, these outputs:

- `wood`: the chunked wood mesh.
- `curve`: the wood as curves.
- `leaves`: the leaf element, its instances, the counts placed and kept, and their bounds.
- `structure`: six floats and three words a node.
- `field`: the occupancy grid of `n` cells a side.

## What is promised

- **Parameters, never a name.** The entry reads a parameter table and refuses a preset's id. Tests: `native_entry::a_native_entry_takes_parameters_and_a_seed_and_returns_the_trees_data_at_any_age` (`crates/telperion-core/tests/native_entry.rs`), which grows the oak from parameter text; `a_preset_id_is_refused_and_its_parameters_build` (`crates/telperion-wasm/src/generate/tests.rs`).
- **A growth has one seed.** The tree it draws, wood and leaves, depends on `begin`'s seed and not on the family's `skeleton.seed` row. Test: `native_entry::a_growth_draws_by_the_seed_it_was_begun_with_never_the_familys_row`.
- **A year is a state.** The tree at a year reached on the way to a later age is the tree grown to that year alone, to the bit. Tests: `native_entry` (the oak at 20 on the way to 40), `a_tree_grown_a_year_at_a_time_repeats_and_ends_at_the_tree_asked_for` (`pipeline/space/growth/tests.rs`), and `a_year_is_a_state_not_a_horizon` and `every_year_is_the_horizons_prefix` (`telperion-space`, `grow/cycle/paused/tests.rs`).
- **The same tree on native and Wasm.** Every decision the grower takes has a fixed representation (`docs/pipeline.md`, "The one cycle and its grower"). Test: `native/wasm node buffers byte-identical` (`harness/parity.test.ts`), every shipped preset's structure through `pipeline::build`. No test compares a tree drawn through `growth` across the two targets.
- **A tree past the machine is refused by name.** No count stands for memory: a store whose size is known before it fills (the conversion's per-node stores, the wood's buffers, the flowers) is reserved at once, and a reserve the machine refuses is an error naming the store and the bytes it asked for. In Wasm, an allocation the code cannot return is recorded with its size, the live bytes and the stage; the package's loader names it as the refusal and replaces the instance, so the next request builds. Natively the generator holds no cap of its own. Tests: `a_wood_the_machine_cannot_hold_is_refused_with_its_bytes` (`pipeline/surface.rs`), `a_request_the_machine_refuses_is_recorded_with_its_stage` (`telperion-wasm`'s `live.rs`), `scripts/test-dist.mjs`'s refused case (the spruce at seed 42 at 118, refused by name, the next request built), `generation_limit_guard`.

## What is not built

- A plugin for any engine (owner, 1 Nov: "Any imports to game engines are out of scope for this goal").
- A native C ABI. The binding's ABI (`crates/telperion-wasm`) is the browser's, and no native build of it is produced or tested.
- Twigs for a region asked (fn-321 R11): fn-351's P10 does not carry it, and nothing builds wood on demand.
